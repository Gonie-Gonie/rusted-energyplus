#!/usr/bin/env python3
"""Read-only provenance for GEO-02 physical runs; never launch an engine."""
from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
import re
import sys
from geo02_unit_provenance import bits

sys.dont_write_bytecode = True
import check_geo02_units as units
from geo02_unit_provenance import (
    PIN, ROOT, exact, from_bits, path_of, read, ref, require, same_binding,
    scalar, sha, timestamp, value_class,
)

CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
FROZEN = {
    "source": "68af6b2b164832f110d1fa308688463790ab899d9fae77565661cf3f56bbeb09",
    "cases": "452d910b6afb81a1dcbf23b43d75d50e74c59da5d7e372ff45424511dc6cf435",
    "tolerances": "3876a34a85f84dc459d1fec406106c0166ef0ff1dd6942beb0d3a7fbcb19a0e1",
}
PHYSICAL_CASES = {"A-24H", "A-72H", "B-BOTH-24H"}
NEGATIVE_CASES = {"GEO02-INVALID-COLLINEAR", "GEO02-INVALID-COINCIDENT"}
SOURCE_OWNERS = {
    "crates/ep_runtime/src/geometry.rs",
    "crates/ep_runtime/src/heat_balance/initialization.rs",
    "crates/ep_runtime/src/heat_balance/state.rs",
    "crates/ep_runtime/src/heat_balance/convection.rs",
    "crates/ep_runtime/src/heat_balance/solar.rs",
    "crates/ep_run/src/pipeline.rs",
    "crates/ep_run/src/geometry_trace.rs",
    "crates/ep_run/src/geo02_trace.rs",
    "crates/ep_runtime/src/geometry/production_trace.rs",
    "crates/ep_runtime/src/geometry/source_geometry.rs",
    "crates/ep_runtime/src/geometry/centroid_precision.rs",
    "crates/ep_runtime/src/heat_balance/convection/geometry_operands.rs",
    "crates/ep_runtime/src/heat_balance/inside_convection.rs",
    "crates/ep_runtime/src/heat_balance/surface_balance.rs",
}


def integer(value, label, minimum=0):
    require(type(value) is int and value >= minimum, "Invalid integer: " + label)
    return value


def named(rows):
    require(type(rows) is list, "Named rows are not an array")
    result, ids = {}, set()
    for row in rows:
        require(type(row) is dict and type(row.get("name")) is str and row["name"].strip(),
                "Missing named identity")
        key = row["name"].upper()
        require(key not in result, "Duplicate case-normalized identity")
        engine_id = integer(row["id"], "own engine ID", 1)
        require(engine_id not in ids, "Duplicate own engine ID")
        ids.add(engine_id)
        result[key] = row
    return result


class Bindings:
    """Hash immutable bytes once, rejecting changes to any checked file."""
    def __init__(self):
        self.checked = {}
        self.source_lines = {}

    @staticmethod
    def stamp(path):
        value = path.stat()
        return value.st_size, value.st_mtime_ns, value.st_ctime_ns

    def check(self, binding):
        require(type(binding) is dict and type(binding.get("path")) is str
                and type(binding.get("sha256")) is str
                and re.fullmatch(r"[0-9a-f]{64}", binding["sha256"]), "Malformed artifact binding")
        path = path_of(binding["path"])
        require(path.is_file(), "Missing bound file: " + binding["path"])
        stamp = self.stamp(path)
        if path not in self.checked:
            digest = sha(path)
            require(stamp == self.stamp(path), "File changed while hashing")
            self.checked[path] = (stamp, digest)
        previous, digest = self.checked[path]
        require(previous == stamp and digest == binding["sha256"], "Changed artifact: " + binding["path"])
        if "bytes" in binding:
            require(type(binding["bytes"]) is int and binding["bytes"] == stamp[0], "Declared artifact size differs")
        return path

    def recursive(self, value):
        if type(value) is dict:
            if "path" in value and "sha256" in value:
                self.check(value)
            for item in value.values():
                self.recursive(item)
        elif type(value) is list:
            for item in value:
                self.recursive(item)

    def unchanged(self):
        for path, (stamp, _) in self.checked.items():
            require(path.is_file() and self.stamp(path) == stamp, "File changed during proof inspection")

    def report(self):
        return [{"path": path.relative_to(ROOT).as_posix(), "sha256": data[1]}
                for path, data in sorted(self.checked.items())]


def contains_binding(value, expected):
    if type(value) is dict:
        if "path" in value and "sha256" in value and same_binding(value, expected):
            return True
        return any(contains_binding(item, expected) for item in value.values())
    return type(value) is list and any(contains_binding(item, expected) for item in value)


def frozen_contracts(matrix, bindings):
    refs = {key: ref(CONTRACTS / f"GEO-02-{key}.json") for key in FROZEN}
    for key, digest in FROZEN.items():
        require(refs[key]["sha256"] == digest, "Frozen contract changed: " + key)
    units.verify_contract_bindings(matrix["contracts"], refs, bindings)
    source, cases, tolerances = [read(bindings.check(refs[key])) for key in FROZEN]
    require(source["energyplus_commit"] == PIN and tolerances["frozen_before_numerical_comparison"] is True,
            "Wrong original pin or unfrozen numerical policy")
    require(set(cases["Rust_physical_cases"]) == PHYSICAL_CASES, "Frozen physical subset changed")
    return refs, source, cases, tolerances


def verify_families(binding, cases, bindings):
    require(same_binding(binding, cases["static_geometry_family_projection"]), "Frozen family proof differs")
    family = read(bindings.check(binding))
    require(family["schema"] == "geo02-input-family-preparation.v1" and family["status"] == "pass"
            and integer(family["case_count"], "family case count") == 7
            and family["missing_fields_inferred"] is False, "Family proof is not the accepted input-only result")
    bindings.recursive(family)
    raw = read(bindings.check(family["raw_result"]))
    require(raw["schema"] == "geo02-recorded-input-families.v1" and raw["status"] == "pass"
            and exact(raw["family_projection_hashes"], family["families"]), "Family raw result differs")
    require(len(raw["cases"]) == 7 and {x["scope"] for x in raw["cases"]} == {"A", "B"},
            "Wrong family representatives")
    bindings.recursive(raw["cases"])
    return family, raw


def verify_original(matrix, refs, cases, bindings):
    helper_receipt = read(bindings.check(matrix["original_helper_execution"]))
    helper, helper_execution = units.verify_native(helper_receipt, refs, cases["helper_request"], bindings)
    require(helper["valid_kernel_input_identity_passed"] is True, "Original helper input identity failed")
    original = read(bindings.check(matrix["native_original_matrix"]))
    require(original["schema"] == "geo02-original-matrix.v1" and original["complete"] is True
            and integer(original["case_count"], "original case count") == 13
            and len(original["cases"]) == 13, "Incomplete original physical input matrix")
    expected = {row["id"]: row for row in cases["native_cases"] + cases["native_invalid_cases"]}
    require(len(expected) == 13 and {row["case_id"] for row in original["cases"]} == set(expected),
            "Original case set differs from frozen input contract")
    core = read(bindings.check(helper_receipt["native_core_build"]))
    driver = read(bindings.check(helper_receipt["native_driver_build"]))
    outputs, ended = {}, [timestamp(helper_execution["finished_utc"])]
    for item in original["cases"]:
        receipt = read(bindings.check(item["receipt"]))
        source = read(bindings.check(item["results"]))
        case = expected[item["case_id"]]
        units.verify_contract_bindings(receipt["contracts"], refs, bindings)
        built_sources = {row["historical_path"]: row["archive"] for row in driver["executed_source_archives"]}
        executed_sources = {row["historical_path"]: row["archive"] for row in receipt["executed_source_archives"]}
        require(len(executed_sources) == len(receipt["executed_source_archives"])
                and set(executed_sources) == set(built_sources), "Original executed source inventory differs")
        for owner, binding in executed_sources.items():
            require(binding["sha256"] == built_sources[owner]["sha256"], "Original executed source bytes differ")
            bindings.check(binding)
        require(receipt["schema"] == "geo02-original-execution.v1"
                and receipt["case_id"] == source["case_id"] == case["id"]
                and receipt["rust_compared"] is False and receipt["gates_updated"] is False
                and receipt["physical_input_scope_expanded"] is False, "Original execution claim boundary differs")
        for key in ("input", "weather"):
            require(same_binding(receipt[key], case[key]) and same_binding(source[key], case[key]),
                    "Original input/weather differs")
            bindings.check(case[key])
        for key in ("native_core_build", "native_driver_build"):
            require(same_binding(receipt[key], helper_receipt[key]), "Original core/driver differs")
        require(same_binding(receipt["binary"], driver["binary"])
                and same_binding(source["library"], core["artifacts"]["api_library"])
                and same_binding(source["native_core_build"], helper_receipt["native_core_build"])
                and same_binding(source["idd"], receipt["idd"])
                and same_binding(receipt["results"], item["results"]), "Original payload/receipt crossbinding differs")
        bindings.check(receipt["idd"])
        bindings.check(receipt["geometry_error_log"])
        execution = read(bindings.check(receipt["execution"]))
        bindings.check(execution["stdout"])
        bindings.check(execution["stderr"])
        command = execution["command"]
        expected_paths = {0: receipt["binary"]["path"], 1: str(ROOT), 2: receipt["native_core_build"]["path"],
                          4: case["input"]["path"], 5: case["weather"]["path"], 6: receipt["idd"]["path"],
                          7: str(path_of(item["results"]["path"]).parent)}
        require(type(command) is list and len(command) == 8 and command[3] == case["id"]
                and all(path_of(command[index]) == path_of(value) for index, value in expected_paths.items())
                and path_of(execution["cwd"]) == ROOT and integer(execution["exit_code"], "original wrapper exit") == 0,
                "Actual original command identity differs")
        require(source["command"] == ["energyplus", "-i", command[6], "-w", command[5], "-d", command[7], command[4]],
                "Original EnergyPlus input argv differs")
        require(source["schema"] == "geo02-original-geometry.v1" and source["callback_error"] == ""
                and integer(source["geometry_probe_calls_added"], "original probes") == 0
                and source["state_reset_requested"] is False and source["simulation_inputs_modified"] is False,
                "Original passive observation boundary differs")
        finish = timestamp(execution["finished_utc"])
        require(timestamp(execution["started_utc"]) <= finish, "Original UTC chronology differs")
        ended.append(finish)
        valid = case["id"] not in NEGATIVE_CASES
        if valid:
            require(integer(source["energyplus_exit_code"], "original EnergyPlus exit") == 0
                    and source["first_final_owned_geometry_identity_exact"] is True
                    and integer(source["physical_zone_callback_count"], "original physical callbacks")
                        == {"24H": 96, "72H": 288}[case["duration"]], "Original valid physical lifetime incomplete")
            for phase in ("first_initialized", "first_physical", "final_weather", "final"):
                snapshot = source[phase]
                require(type(snapshot) is dict and integer(snapshot["surface_count"], "original surfaces") == 6
                        and integer(snapshot["zone_count"], "original zones") == 1, "Original phase missing")
                flags = snapshot["native_only_state"]
                require(integer(flags["total_coincident_vertices"], "coincident count") == 0
                        and integer(flags["total_degenerate_surfaces"], "degenerate count") == 0
                        and flags["aspect_transform"] is False and flags["no_transform"] is True,
                        "Original geometry exclusion became active")
                units.floating_environment(snapshot["floating_environment"])
                for surface in snapshot["surfaces"]:
                    require(integer(surface["sides"], "original sides") == 4
                            and surface["native_consumer"]["is_degenerate"] is False
                            and surface["native_consumer"]["vertices_processed"] is True,
                            "Original selected shape is different")
        else:
            require(integer(source["energyplus_exit_code"], "original negative exit", 1) > 0
                    and integer(source["physical_zone_callback_count"], "original negative callbacks") == 0,
                    "Original negative input entered physics")
        outputs[case["id"]] = source
    original_completion = max(ended)
    reviews = matrix["original_first_reviews"]
    require(set(reviews) == {"helper", "native_matrix"}, "Original independent review set differs")
    reviewed_methods = {
        "helper": ("geo02-independent-original-helper-provenance-review.v1",
                   "pass-original-provenance-input-state-and-floating-environment-only",
                   "original_helper_receipt", matrix["original_helper_execution"]),
        "native_matrix": ("geo02-independent-original-native-review.v1",
                          "pass-original-provenance-input-and-lifetime-only",
                          "original_matrix", matrix["native_original_matrix"]),
    }
    for key, (schema, status, method, target) in reviewed_methods.items():
        review = read(bindings.check(reviews[key]))
        require(review["schema"] == schema and review["status"] == status
                and same_binding(review[method], target),
                "Original independent review does not bind its actual method")
        if key == "native_matrix":
            units.verify_contract_bindings(review["contracts"], refs, bindings)
        bindings.recursive(review)
        completed = timestamp(review["completed_utc"])
        method_completed = (timestamp(helper_execution["finished_utc"])
                            if key == "helper" else original_completion)
        require(method_completed <= completed, "Original independent review predates its actual execution")
        ended.append(completed)
    return outputs, max(ended)


def verify_build(matrix, bindings):
    build = read(bindings.check(matrix["build"]))
    require(build["schema"] == "geo02-Rust-build.v1" and build["kind"] == "cli"
            and build["source_worktree_clean"] is True
            and same_binding(build["binary"], matrix["binary"]), "Production needs an immutable committed CLI build")
    commit = build["implementation_commit"]
    require(type(commit) is str and re.fullmatch(r"[0-9a-f]{40}", commit)
            and commit == build["repository_head"] == matrix["implementation_commit"], "Compiled commit differs")
    for key in ("binary", "build_execution", "source_snapshot", "cargo_lock", "toolchain", "executed_launcher"):
        bindings.check(build[key])
    execution = read(bindings.check(build["build_execution"]))
    require(execution["command"] == ["cargo", "build", "-p", "ep_cli", "--bin", "eplus-rs", "-j", "2"]
            and integer(execution["exit_code"], "actual build exit") == 0
            and execution["source_bytes_match_before_and_after"] is True
            and path_of(execution["cwd"]) == ROOT
            and execution["repository_before"]["head"] == execution["repository_after"]["head"] == commit
            and same_binding(execution["source_snapshot"], build["source_snapshot"]), "Actual CLI build identity differs")
    bindings.recursive(execution)
    snapshot = read(bindings.check(build["source_snapshot"]))
    files = snapshot["files"]
    sources = {row["historical_path"]: row["archive"] for row in files}
    require(len(sources) == len(files) and SOURCE_OWNERS <= set(sources), "Compiled owner snapshot is incomplete")
    bindings.recursive(files)
    rust = {key: value for key, value in sources.items() if key.endswith(".rs")}
    actual = {row["historical_path"]: row for row in build["crates_sources"]}
    require(len(actual) == len(build["crates_sources"]) and set(actual) == set(rust), "Compiled Rust source inventory differs")
    for key, value in rust.items():
        require(same_binding(actual[key], value), "Compiled Rust source bytes differ")
    for key, owner in (("cargo_lock", "Cargo.lock"), ("toolchain", "rust-toolchain.toml")):
        require(same_binding(build[key], sources[owner]), "Actual Cargo/toolchain source differs")
    precision = build["compiled_source_vs_committed_blobs"]
    differences = precision["line_ending_only_differences"]
    require(type(differences) is list and precision["all_other_content_exact"] is True
            and integer(precision["source_count"], "compiled source count") == len(rust)
            and integer(precision["exact_byte_matches"], "committed exact count") + len(differences) == len(rust),
            "Committed byte audit is incomplete")
    seen = set()
    for row in differences:
        key = row["historical_path"]
        require(key in rust and key not in seen and same_binding(row["compiled_archive"], rust[key]),
                "Compiled line-ending exception differs")
        seen.add(key)
        content = bindings.check(row["compiled_archive"]).read_bytes()
        normalized = content.replace(b"\r\n", b"\n")
        require(content != normalized and hashlib.sha256(normalized).hexdigest() == row["git_blob_sha256"]
                and hashlib.sha1(b"blob " + str(len(normalized)).encode() + b"\0" + normalized).hexdigest()
                    == row["git_blob"], "Declared Git blob differs from normalized compiled bytes")
    return build


def recorded_run(case, row, matrix, build, original_finished, bindings, directories, negative=False):
    output = path_of(row["output_directory"])
    require(output.is_dir() and output not in directories, "Missing/reused actual run output")
    directories.add(output)
    level = row["trace_level"]
    require(level in {"full", "summary"} and (not negative or level == "full"), "Wrong recorded trace level")
    execution = read(bindings.check(row["execution"]))
    require(execution["dry_run"] is False and execution["trace_level"] == level
            and execution["original_outputs_supplied_to_Rust"] is False
            and path_of(execution["cwd"]) == ROOT, "Command was not an actual passive normal run")
    require(execution["repository_head"] == build["implementation_commit"], "Physical run revision differs")
    for key, expected_binding in (("binary", matrix["binary"]), ("build", matrix["build"]),
            ("input", case["input"]), ("weather", case["weather"]),
            ("executed_launcher", matrix["executed_launcher"])):
        require(same_binding(execution[key], expected_binding), "Physical run crossbinding differs: " + key)
        bindings.check(execution[key])
    require(original_finished <= timestamp(execution["started_utc"]) <= timestamp(execution["finished_utc"]),
            "Original-first/physical execution chronology differs")
    for key in ("stdout", "stderr"):
        bindings.check(execution[key])
    configured_scope = row.get("configured_porting_scope", "A" if negative else case["scope"])
    require(configured_scope == ("A" if negative else case["scope"]), "Configured bounded route differs")
    require(execution["configured_porting_scope"] == configured_scope, "Recorded configured route differs")
    scope_args = ["--porting-scope", configured_scope]
    expected = [matrix["binary"]["path"], "run", case["input"]["path"], "--weather", case["weather"]["path"],
                "--output-dir", str(output), "--mode", "compatibility", "--partial", "deny", "--trace-level", level] + scope_args
    command = execution["command"]
    require(type(command) is list and len(command) == len(expected) and all(type(x) is str for x in command),
            "Unexpected normal physical CLI argv")
    require(all(path_of(command[i]) == path_of(value) if i in {0, 2, 4, 6} else command[i] == value
                for i, value in enumerate(expected)), "Command differs from unchanged-input normal physical route")
    artifacts = {}
    for binding in row["artifacts"]:
        path = bindings.check(binding)
        require(path.is_relative_to(output) and path not in artifacts, "Duplicate/foreign actual artifact")
        artifacts[path] = binding
    require(set(artifacts) == {path.resolve() for path in output.rglob("*") if path.is_file()},
            "Actual output file inventory differs from recorded hashes")
    summary_path = output / "run-summary.json"
    require(summary_path in artifacts and same_binding(row["run_summary"], artifacts[summary_path]), "Run summary binding absent")
    summary = read(summary_path)
    code = integer(execution["exit_code"], "physical CLI exit")
    require(type(summary["exit_code"]) is int and summary["exit_code"] == code, "Run summary exit differs from process")
    config = summary["config"]
    require(config["dry_run"] is False and config["mode"] == "compatibility"
            and config["partial_policy"] == "deny" and config["trace_level"] == level
            and config["oracle_baseline"] is False and config["compare_oracle"] is False
            and config["hours"] is None and config["output_format"] == "rust-native"
            and summary["oracle"] is None and summary["comparison"] is None
            and summary["oracle_status"] == summary["compare_status"] == "not-requested",
            "Dry-run/answer-fed/partial/limited-duration route cannot prove physical connection")
    for key in ("input", "weather"):
        bindings.check(case[key])
    if negative:
        require(code == 6 and summary["status"] == "runtime"
                and summary["message"] == "Rust runtime failed" and summary["rust_runtime"] is None,
                "Negative input did not reach the runtime initialization failure route")
        metadata = read(bindings.check(case["metadata"]))
        patches = metadata["patches"]
        require(len(patches) == 1 and patches[0]["object_type"] == "BuildingSurface:Detailed",
                "Negative geometry patch is not the frozen single surface")
        surface_name = patches[0]["name"].upper()
        diagnostics = read(output / "diagnostics.json")["diagnostics"]
        errors = [row for row in diagnostics if row["severity"] == "error"]
        expected_message = (f"surface {surface_name} geometry is invalid: "
                            "surface has zero area or a zero Newell normal")
        require(len(errors) == 1 and errors[0]["blocking"] is True
                and errors[0]["code"] == "RuntimeConvergenceFailure"
                and errors[0]["stage"] == "runtime" and errors[0]["message"] == expected_message,
                "Negative failure is unrelated to the canonical geometry initializer")
    else:
        require(code == 0 and summary["status"] == "success" and summary["message"] == "arbitrary run completed",
                "Physical run was unsuccessful")
        runtime = summary["rust_runtime"]
        expected_samples = {"24H": 24, "72H": 72}[case["duration"]]
        require(type(runtime) is dict and integer(runtime["samples"], "actual runtime samples") == expected_samples,
                "Actual runtime duration incomplete")
        require(summary["source_order_gate"]["matches"] is True
                and summary["selected_algorithm_lane"]["diagnostic_probe_used"] is False
                and summary["support"]["conformance_claim"] is False, "Different or promoted scientific lane")
        if case["scope"] == "B":
            require(runtime["fixture_demand_injection_used"] is False
                    and integer(runtime["purchased_air_coupling_call_count"], "B actual calls") == expected_samples * 4,
                    "Fixture demand or missing genuine B coupling")
    return output, summary, execution, artifacts


def input_geometry(case, bindings):
    source = case["geometry_input_source"]
    value = read(bindings.check(source["artifact"]))
    require(type(source["json_pointer"]) is str and source["json_pointer"].startswith("/"),
            "Frozen geometry input pointer is malformed")
    for token in source["json_pointer"].split("/")[1:]:
        token = token.replace("~1", "/").replace("~0", "~")
        value = value[int(token)] if type(value) is list else value[token]
    require(type(value) is dict, "Frozen input projection is not an object")
    return value


def fnv1a(content):
    value = 0xcbf29ce484222325
    for byte in content:
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    return f"{value:016x}"


def compiled_projection(output, case, original, family_raw, bindings):
    """Verify real staged input and own typed coordinates without transforming them."""
    path = output / "compiled-geometry.json"
    projection = read(path)
    bindings.check(ref(path))
    require(projection["schema"] == "compiled-geometry.v1"
            and projection["phase"] == "typed_compile" and projection["physics_executed"] is False
            and projection["preparation_only"] is True and projection["observer_supplies_inputs"] is False,
            "Compiled coordinate observation is not preparation-only")
    hashes = read(output / "input/input-hashes.json")
    bindings.check(ref(output / "input/input-hashes.json"))
    require(hashes["algorithm"] == projection["inputs"]["hash_algorithm"] == "fnv-1a-64"
            and projection["inputs"]["source_assisted_lexical_conversion"] is True
            and projection["inputs"]["conversion_is_physical_observation"] is False,
            "Lexical staging must not claim physical original outputs")
    for key, target in (("source", "original_input"), ("staged_original", "staged_original_input"),
                        ("converted_epjson", "converted_epjson")):
        item = hashes[key]
        require(exact(item, projection["inputs"][target]), "Typed projection input receipt differs")
        source_path = path_of(item["path"])
        content = source_path.read_bytes()
        require(integer(item["bytes"], "staged byte count") == len(content) and item["hash"] == fnv1a(content),
                "Actual lexical input bytes differ")
        binding = ref(source_path)
        bindings.check(binding)
        if key == "source":
            require(same_binding(binding, case["input"]), "Actual typed original input differs")
        elif key == "staged_original":
            require(binding["sha256"] == case["input"]["sha256"] and source_path.is_relative_to(output),
                    "Staged IDF was rewritten or belongs to another run")
        else:
            require(source_path.is_relative_to(output), "Converted lexical input belongs to another run")
    frozen = input_geometry(case, bindings)
    surfaces = named(projection["surfaces"])
    zones = named(projection["zones"])
    source_surfaces = named(original["first_initialized"]["surfaces"]) if original is not None else None
    require(len(surfaces) == 6 and len(zones) == 1 and (source_surfaces is None or set(surfaces) == set(source_surfaces))
            and set(surfaces) == {row["name"].upper() for row in frozen["surfaces"]},
            "Actual typed/native/input named topology differs")
    zone_ids = {integer(row["id"], "typed zone ID", 1): key for key, row in zones.items()}
    for row in frozen["surfaces"]:
        key = row["name"].upper()
        actual = surfaces[key]
        require(integer(actual["zone_id"], "typed surface zone ID", 1) in zone_ids
                and zone_ids[actual["zone_id"]] == row["zone_name"].upper()
                and actual["zone_name"].upper() == row["zone_name"].upper()
                and actual["class"] == row["class"] and integer(actual["sides"], "typed sides") == 4,
                "Actual stored typed surface binding/class differs")
        require(exact(actual["world_vertex_bits"], row["input_vertex_bits"])
                and (source_surfaces is None or exact(source_surfaces[key]["vertex_bits"], row["input_vertex_bits"])),
                "Own stored kernel vertex inputs differ from frozen raw World points")
        for values, tokens in zip(actual["world_vertices_m"], actual["world_vertex_bits"], strict=True):
            require(type(values) is list and len(values) == len(tokens) == 3, "Stored coordinate cardinality differs")
            for value, token in zip(values, tokens, strict=True):
                scalar(value, token)
    for row in frozen["zones"]:
        actual = zones[row["name"].upper()]
        require(exact(actual["origin_bits"], [bits(x) for x in row["origin_m"]])
                and actual["relative_north_deg"] == row["relative_north_deg"], "Actual zone coordinate inputs differ")
    settings = projection["settings"]
    require(settings["appendix_g_rotation_deg"] == 0
            and settings["building_raw_north_axis_deg"] == frozen["building_raw_north_axis_deg"]
            and settings["global_geometry_rules"] == frozen["rules"], "Actual geometry settings differ")
    if family_raw is not None:
        family_projection = {key:projection[key] for key in ("parsed_input", "settings", "zones", "surfaces")}
        encoded = json.dumps(family_projection,sort_keys=True,separators=(",",":"),allow_nan=False).encode()
        require(hashlib.sha256(encoded).hexdigest() == family_raw["family_projection_hashes"][case["scope"]],
                "Fresh typed input projection differs from the reviewed geometry family")
    return projection
