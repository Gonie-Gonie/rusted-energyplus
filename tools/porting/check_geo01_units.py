#!/usr/bin/env python3
"""Read preserved ordinary-CLI compile projections against genuine GEO-01 runs.

No executable invocation, coordinate equation, expected-output input, physics
comparison, tolerance change or automatic gate update is performed here.
"""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
import math
from pathlib import Path
import shutil
import struct
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def integer(value, label):
    require(type(value) is int, "Expected an integer metadata field: " + label)
    return value


def read(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def sha(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as f:
        for block in iter(lambda: f.read(1048576), b""):
            h.update(block)
    return h.hexdigest()


def ref(path):
    p = Path(path).resolve()
    return {"path": p.relative_to(ROOT).as_posix(), "sha256": sha(p)}


def path_of(value):
    p = Path(value)
    return p.resolve() if p.is_absolute() else (ROOT / p).resolve()


def binding(value):
    require(type(value) is dict and type(value.get("path")) is str and type(value.get("sha256")) is str, "Malformed hash reference")
    p = path_of(value["path"])
    require(sha(p) == value["sha256"], "Changed hash reference " + value["path"])
    return p


def same_binding(actual, expected):
    return (type(actual) is dict and type(expected) is dict
            and type(actual.get("path")) is str and type(actual.get("sha256")) is str
            and path_of(actual["path"]) == path_of(expected["path"])
            and actual["sha256"] == expected["sha256"])


def bits(value):
    require(type(value) in (int, float), "Expected a numerical observation")
    return struct.pack(">d", float(value)).hex()


def exact_equal(a, b):
    if type(a) is not type(b):
        return False
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(exact_equal(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(exact_equal(x, y) for x, y in zip(a, b))
    return a == b


def name(value):
    require(type(value) is str, "Expected an actual name")
    return value.upper()


def canonical_path(value):
    require(type(value) is str and value.startswith("/"), "Bad raw numeric path")
    chunks = value.split("/")
    require(len(chunks) >= 4, "Bad raw numeric path depth")
    chunks[1] = chunks[1].casefold()
    chunks[2] = chunks[2].upper()
    return "/".join(chunks)


def raw_numbers(parsed):
    """Validate emitted bits using the observed JSON numbers, excluding IDF order metadata."""
    objects = parsed["objects"]
    actual = {}

    def visit(value, path):
        if type(value) in (int, float):
            if not any(piece.startswith("idf_") for piece in path.split("/")):
                actual[canonical_path(path)] = bits(value)
        elif isinstance(value, list):
            for i, child in enumerate(value):
                visit(child, path + "/" + str(i))
        elif isinstance(value, dict):
            for key, child in value.items():
                visit(child, path + "/" + key)

    visit(objects, "")
    emitted = {}
    for row in parsed["numeric_bits"]:
        if any(piece.startswith("idf_") for piece in row["path"].split("/")):
            continue
        key = canonical_path(row["path"])
        require(key not in emitted, "Duplicate numeric observation path")
        require(type(row["bits"]) is str and len(row["bits"]) == 16, "Bad numerical observation bits")
        emitted[key] = row["bits"]
    require(exact_equal(actual, emitted), "Raw input bit observations disagree with actual parsed values")
    return emitted


def named_rows(rows):
    require(type(rows) is list, "Actual row array required")
    result = {}
    ids = set()
    for row in rows:
        key = name(row["name"])
        require(key not in result and type(row["id"]) is int and row["id"] not in ids, "Duplicate actual name/engine ID")
        result[key] = row
        ids.add(row["id"])
    return result


def fnv1a(data):
    value = 0xcbf29ce484222325
    for byte in data:
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    return f"{value:016x}"


def verify_inputs(output, projection, frozen):
    p = output / "input/input-hashes.json"
    receipt = read(p)
    require(receipt["algorithm"] == "fnv-1a-64" and projection["inputs"]["hash_algorithm"] == receipt["algorithm"], "Unexpected normal pipeline input hash algorithm")
    refs = {"receipt": ref(p)}
    for source_key, projection_key in [("source", "original_input"), ("staged_original", "staged_original_input"), ("converted_epjson", "converted_epjson")]:
        row = receipt[source_key]
        require(exact_equal(row, projection["inputs"][projection_key]), "Compile projection hash receipt differs")
        file = path_of(row["path"])
        content = file.read_bytes()
        require(type(row["bytes"]) is int and row["bytes"] == len(content) and row["hash"] == fnv1a(content), "Normal pipeline input receipt does not match actual file")
        refs[source_key] = ref(file)
    require(refs["source"] == frozen["input"], "Rust source IDF differs from frozen original input")
    require(refs["staged_original"]["sha256"] == frozen["input"]["sha256"], "Staged IDF bytes differ")
    require(projection["inputs"]["source_assisted_lexical_conversion"] is True and projection["inputs"]["conversion_is_physical_observation"] is False, "Lexical conversion/proof boundary differs")
    return refs


class Comparison:
    def __init__(self):
        self.checks = 0
        self.mismatches = Counter()
        self.examples = []
        self.coordinate_count = 0
        self.max_absolute_error = 0.0
        self.max_relative_error = 0.0
        self.sum_squared_error = 0.0

    def fail(self, kind, location, actual, original):
        self.mismatches[kind] += 1
        if len(self.examples) < 100:
            self.examples.append({"kind": kind, "location": location, "rust": actual, "original": original})

    def exact(self, actual, original, location):
        self.checks += 1
        if not exact_equal(actual, original):
            self.fail("exact_metadata", location, actual, original)

    def coordinate(self, actual, original, actual_bits, original_bits, profile, location):
        self.checks += 1
        self.coordinate_count += 1
        require(bits(actual) == actual_bits and bits(original) == original_bits, "Coordinate payload/bits disagree " + location)
        if not math.isfinite(actual) or not math.isfinite(original):
            self.fail("finite_class", location, actual_bits, original_bits)
            return
        diff = abs(actual - original)
        self.max_absolute_error = max(self.max_absolute_error, diff)
        self.sum_squared_error += diff * diff
        if original != 0:
            self.max_relative_error = max(self.max_relative_error, diff / abs(original))
        if diff > profile["absolute_tolerance"] + profile["relative_tolerance"] * abs(original):
            self.fail("coordinate_tolerance", location, actual, original)
        if actual == 0 and original == 0 and actual_bits != original_bits:
            self.fail("signed_zero", location, actual_bits, original_bits)


def locate(rust_root, case_id):
    candidates = [rust_root / case_id / "compiled-geometry.json", rust_root / case_id / "dry-run/compiled-geometry.json",
                  rust_root / case_id / "full/compiled-geometry.json"]
    found = [p for p in candidates if p.is_file()]
    require(len(found) == 1, f"Need exactly one preserved compiled-geometry artifact for {case_id}; got {len(found)}")
    return found[0]


def compare_case(case, original_row, rust_path, cmp, profile):
    case_before = sum(cmp.mismatches.values())
    coordinate_kinds = ("finite_class", "coordinate_tolerance", "signed_zero")
    coordinate_before = sum(cmp.mismatches[k] for k in coordinate_kinds)
    receipt_path = binding(original_row["receipt"])
    receipt = read(receipt_path)
    original_path = binding(original_row["results"])
    require(receipt["results"] == original_row["results"] and receipt["input"] == case["input"] and receipt["weather"] == case["weather"], "Original matrix input/output binding differs")
    for key in ["binary", "execution", "native_core_build", "native_driver_build", "executed_launcher", "input", "weather", "idd"]:
        binding(receipt[key])
    core = read(binding(receipt["native_core_build"]))
    driver = read(binding(receipt["native_driver_build"]))
    require(core["checks_passed"] is True and driver["checks_passed"] is True and core["energyplus_commit"] == driver["energyplus_commit"] == PIN, "Original state/compiler peer not verified")
    authoritative = [ref(CONTRACTS / f"GEO-01-{key}.json") for key in ["source", "cases", "tolerances"]]
    require(exact_equal(receipt["contracts"], authoritative), "Original execution did not use the authoritative three frozen contracts")
    require(same_binding(driver["binary"], receipt["binary"]) and same_binding(driver["core_build"], receipt["native_core_build"]), "Original driver/receipt binary or core crossbinding differs")
    require(same_binding(driver["source_contract"], authoritative[0]), "Original driver source contract differs")
    for value in authoritative:
        binding(value)
    execution = read(binding(receipt["execution"]))
    require(integer(execution["exit_code"], "original wrapper exit") == 0, "Original wrapper execution failed")
    command = execution["command"]
    require(type(command) is list and len(command) == 8 and path_of(command[0]) == path_of(receipt["binary"]["path"])
            and path_of(command[1]) == ROOT
            and path_of(command[2]) == path_of(receipt["native_core_build"]["path"]) and command[3] == case["id"]
            and path_of(command[4]) == path_of(case["input"]["path"]) and path_of(command[5]) == path_of(case["weather"]["path"])
            and path_of(command[6]) == path_of(receipt["idd"]["path"])
            and path_of(command[7]) == original_path.parent, "Original executed command input/binary/core/output binding differs")
    native = read(original_path)
    rust = read(rust_path)
    require(native["schema"] == "geo01-original-geometry.v1" and integer(native["energyplus_exit_code"], "native exit") == 0 and native["callback_error"] == "", "Original schema/execution invalid")
    require(integer(native["geometry_probe_calls_added"], "native probe count") == 0 and native["state_reset_requested"] is False and native["simulation_inputs_modified"] is False, "Original observer supplied state/input")
    require(native["case_id"] == case["id"] and receipt["case_id"] == case["id"], "Original case identity differs")
    for key in ["input", "weather", "idd", "native_core_build"]:
        require(same_binding(native[key], receipt[key]), "Original payload/receipt crossbinding differs: " + key)
        binding(native[key])
    require(same_binding(native["library"], core["artifacts"]["api_library"]), "Original observed DLL differs from verified genuine core")
    binding(native["library"])
    native_command = native["command"]
    require(type(native_command) is list and len(native_command) == 8
            and native_command[0] == "energyplus" and native_command[1] == "-i" and native_command[3] == "-w" and native_command[5] == "-d"
            and path_of(native_command[2]) == path_of(receipt["idd"]["path"])
            and path_of(native_command[4]) == path_of(case["weather"]["path"])
            and path_of(native_command[6]) == original_path.parent
            and path_of(native_command[7]) == path_of(case["input"]["path"]), "Original DLL's actual simulation argv binding differs")
    require(rust["schema"] == "compiled-geometry.v1" and rust["physics_executed"] is False and rust["preparation_only"] is True and rust["observer_supplies_inputs"] is False, "Rust compile-only projection boundary differs")
    inputs = verify_inputs(rust_path.parent, rust, case)
    n = native["first_initialized"]
    require(native["first_final_vertex_identity_exact"] is True, "Original retained vertices changed")
    require(integer(native["constructor"]["native_only_state"]["max_vertices_per_surface"], "native constructor max") == 4, "Native constructor maximum differs")
    for phase in ["first_initialized", "first_physical", "final_weather", "final"]:
        require(integer(native[phase]["surface_count"], "native surface count") == 6 and integer(native[phase]["zone_count"], "native zone count") == 1, "Native topology count differs")
        state = native[phase]["native_only_state"]
        require(integer(state["max_vertices_per_surface"], "native max") == 4 and integer(state["total_coincident_vertices"], "native coincident count") == 0 and integer(state["total_degenerate_surfaces"], "native degenerate count") == 0, "Excluded native cleanup/growth observed")
        require(state["aspect_transform"] is False and state["no_transform"] is True and state["zone_cos_array_allocated"] is False and state["zone_sin_array_allocated"] is False, "Excluded aspect/false lifecycle observation")
    original_numbers = raw_numbers(native["parsed_input"])
    rust_numbers = raw_numbers(rust["parsed_input"])
    cmp.exact(rust_numbers, original_numbers, case["id"] + ".actual_parsed_input_bits")
    # Input signs must match before signed-output comparison has scientific meaning.
    require(exact_equal(rust_numbers, original_numbers), "Original/Rust actual parsed input numeric identity failed: " + case["id"])
    settings = rust["settings"]
    cmp.exact(settings["building_effective_north_axis_bits"], n["settings"]["building_effective_north_axis_bits"], case["id"] + ".effective_building_angle")
    require(bits(settings["building_effective_north_axis_deg"]) == settings["building_effective_north_axis_bits"], "Rust angle payload/bits disagree")
    cmp.exact(settings["appendix_g_rotation_deg"], float(n["settings"]["appendix_g_rotation_deg"]), case["id"] + ".fixed_appendix_g_angle")
    ggr = settings["global_geometry_rules"]
    corner = {"UpperLeftCorner": 1, "LowerLeftCorner": 2, "LowerRightCorner": 3, "UpperRightCorner": 4}
    require(ggr["starting_vertex_position"] in corner and ggr["vertex_entry_direction"] in ["CounterClockwise", "Clockwise"] and ggr["coordinate_system"] in ["World", "Relative"], "Unknown actual typed GGR")
    cmp.exact(corner[ggr["starting_vertex_position"]], integer(n["settings"]["corner"], "native corner"), case["id"] + ".corner")
    cmp.exact(ggr["vertex_entry_direction"] == "CounterClockwise", n["settings"]["counterclockwise"], case["id"] + ".winding")
    cmp.exact(ggr["coordinate_system"] == "World", n["settings"]["world_coordinate_system"], case["id"] + ".coordinate_system")
    nz = named_rows(n["zones"])
    rz = named_rows(rust["zones"])
    cmp.exact(sorted(rz), sorted(nz), case["id"] + ".zone_names")
    require(set(rz) == set(nz), "Zone identity mismatch prevents geometry pairing")
    for key in nz:
        cmp.exact(rz[key]["relative_north_bits"], nz[key]["relative_north_bits"], case["id"] + ".zone_north")
        cmp.exact(rz[key]["origin_bits"], nz[key]["origin_bits"], case["id"] + ".zone_origin")
        require(bits(rz[key]["relative_north_deg"]) == rz[key]["relative_north_bits"] and [bits(x) for x in rz[key]["origin_m"]] == rz[key]["origin_bits"], "Rust zone numerical payload/bits disagree")
    ns = named_rows(n["surfaces"])
    rs = named_rows(rust["surfaces"])
    cmp.exact(sorted(rs), sorted(ns), case["id"] + ".surface_names")
    require(set(rs) == set(ns) and len(rs) == 6, "Surface identity mismatch prevents coordinate pairing")
    zone_names_by_rust_id = {v["id"]: k for k, v in rz.items()}
    zone_names_by_native_id = {v["id"]: k for k, v in nz.items()}
    class_ids = {"Wall": 1, "Floor": 2, "Roof": 3}  # locked original DataSurfaces::SurfaceClass, not a geometry equation
    for key, source in ns.items():
        actual = rs[key]
        cmp.exact(name(actual["zone_name"]), name(source["zone_name"]), case["id"] + "." + key + ".zone_name")
        cmp.exact(zone_names_by_rust_id[integer(actual["zone_id"], "Rust surface zone ID")], zone_names_by_native_id[integer(source["zone_id"], "native surface zone ID")], case["id"] + "." + key + ".own_id_binding")
        cmp.exact(class_ids.get(actual["class"]), integer(source["class"], "native surface class"), case["id"] + "." + key + ".surface_class")
        cmp.exact(integer(actual["sides"], "Rust sides"), integer(source["sides"], "native sides"), case["id"] + "." + key + ".cardinality")
        require(type(actual["sides"]) is int and actual["sides"] == source["sides"] == 4, "Unsupported cardinality")
        require(len(actual["world_vertices_m"]) == len(actual["world_vertex_bits"]) == 4, "Coordinate array length differs")
        for i in range(4):
            require(len(actual["world_vertices_m"][i]) == len(actual["world_vertex_bits"][i]) == 3, "Coordinate dimension differs")
            for axis in range(3):
                cmp.coordinate(actual["world_vertices_m"][i][axis], source["world_vertices_m"][i][axis], actual["world_vertex_bits"][i][axis], source["world_vertex_bits"][i][axis], profile, f"{case['id']}.{key}.vertex[{i}][{axis}]")
    return {"case_id": case["id"], "kind": case["kind"], "input": case["input"], "weather": case["weather"],
        "original_receipt": original_row["receipt"], "original_results": original_row["results"],
        "rust_compiled_geometry": ref(rust_path), "rust_input_files": inputs, "physics_executed": False,
        "parsed_input_bits_equal": True, "own_engine_ids_paired_by_name": True,
        "coordinate_count": 72, "mismatch_count": sum(cmp.mismatches.values()) - case_before,
        "coordinate_mismatch_count": sum(cmp.mismatches[k] for k in coordinate_kinds) - coordinate_before,
        "native_counter_before_after": [4, 4], "native_counter_Rust_lifecycle_peer": False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rust-root", type=Path, required=True)
    parser.add_argument("--original-matrix", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--case", action="append", default=[])
    args = parser.parse_args()
    require(not args.output_dir.exists(), "Fresh analysis output directory required")
    cases = read(CONTRACTS / "GEO-01-cases.json")
    source = read(CONTRACTS / "GEO-01-source.json")
    tol = read(CONTRACTS / "GEO-01-tolerances.json")
    require(source["energyplus_commit"] == PIN and cases["case_count"] == 37 and cases["production_case_count"] == 15, "Wrong frozen source/case domain")
    profile = tol["profiles"]["world_vertices_m"]
    require(profile["absolute_tolerance"] == 1e-10 and profile["relative_tolerance"] == 1e-13 and tol["frozen_before_comparison"] is True, "Fixed coordinate tolerance policy changed")
    selected = cases["cases"]
    if args.case:
        require(len(set(args.case)) == len(args.case), "Duplicate --case")
        selected = [x for x in selected if x["id"] in args.case]
        require(len(selected) == len(args.case), "Unknown --case")
    matrix = read(args.original_matrix)
    originals = {x["case_id"]: x for x in matrix["cases"]}
    require(len(originals) == len(matrix["cases"]) and all(c["id"] in originals for c in selected), "Original matrix lacks requested case identities")
    args.output_dir.mkdir(parents=True)
    archived = args.output_dir / "check_geo01_units.py"
    shutil.copyfile(Path(__file__), archived)
    cmp = Comparison()
    summaries = [compare_case(c, originals[c["id"]], locate(args.rust_root, c["id"]), cmp, profile) for c in selected]
    mismatches = sum(cmp.mismatches.values())
    report = {"schema": "geo01-unit-comparison.v1", "card": "GEO-01", "status": "pass" if mismatches == 0 else "fail",
        "command": [sys.executable, *sys.argv], "executed_checker": ref(archived), "original_matrix": ref(args.original_matrix),
        "contracts": {x: ref(CONTRACTS / f"GEO-01-{x}.json") for x in ["source", "cases", "tolerances"]},
        "case_count": len(summaries), "full_37_case_matrix": len(summaries) == 37, "cases": summaries,
        "coordinate_count": cmp.coordinate_count, "checks": cmp.checks, "mismatch_count": mismatches,
        "mismatch_kinds": dict(cmp.mismatches), "examples": cmp.examples,
        "metrics": {"world_vertices_m": {"maximum_absolute_error": cmp.max_absolute_error, "maximum_relative_error_nonzero_reference": cmp.max_relative_error,
            "rmse": math.sqrt(cmp.sum_squared_error / cmp.coordinate_count) if cmp.coordinate_count else None,
            "count": cmp.coordinate_count, "signed_zero_mismatches": cmp.mismatches["signed_zero"]}},
        "actual_parsed_input_bits_checked": True, "source_flags_and_named_bindings_checked": True,
        "physics_executed": False, "production_connection_compared": False, "gates_updated": False,
        "limitations": ["original full-model initialization reference versus ordinary Rust compile-only output", "diagnostic inputs do not expand CON", "native MaxVerticesPerSurface lifecycle has no Rust peer", "native transient cache state unpaired; stored vertices paired", "GEO02/GEO03 algorithms and actual physical connection checks excluded"]}
    path = args.output_dir / "comparison-report.json"
    path.write_text(json.dumps(report, ensure_ascii=False, indent=2, allow_nan=False) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"status": report["status"], "case_count": len(summaries), "coordinate_count": cmp.coordinate_count, "mismatch_count": mismatches, "metrics": report["metrics"], "report": ref(path)}))
    return 0 if mismatches == 0 else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, OSError, KeyError) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(2)
