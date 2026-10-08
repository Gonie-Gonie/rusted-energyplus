#!/usr/bin/env python3
"""Receipt-preserving GEO-03 native preparation and original-only execution.

Contracts and independent review must precede native actions. This tool never
starts Rust, compares scientific results, changes source math, or updates gates.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / ".reference/energyplus-src/26.1.0"
RAW = ROOT / ".runtime/porting/GEO-03"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
BUILD = ROOT / ".runtime/ep261-gcc13-o0"
CORE = ROOT / ".runtime/porting/reference-energyplus-26.1.0/native-core-build.json"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
OLD_CONFIGURE = ROOT / ".runtime/porting/GEO-02/native-build-04/configure-command.json"
FROZEN_CORE_ROWS = ROOT / ".runtime/porting/reference-energyplus-26.1.0/provenance/final-core/compile_commands.json"
NAMES = ["geo03_reference.cpp", "geo03_reference_helper.cpp", "geo03_reference_fields.hh",
         "geo03_reference.cmake", "geo03_reference_native.py", "geo02_reference_fields.hh",
         "geo02_reference.cmake", "geo01_reference.cmake", "psy02_reference.cmake",
         "psy02_reachability.cmake", "clk01_reference.cmake"]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def write(path, data):
    Path(path).write_text(json.dumps(data, indent=2, ensure_ascii=True) + "\n", encoding="utf-8", newline="\n")


def sha(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def ref(path):
    path = Path(path).resolve()
    require(path.is_relative_to(ROOT), "Reference outside repository")
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": sha(path)}


def verify(binding):
    path = (ROOT / binding["path"]).resolve()
    require(path.is_relative_to(ROOT) and path.is_file(), "Missing/outside bound file: " + str(path))
    require(sha(path) == binding["sha256"], "Bound bytes changed: " + str(path))
    if "bytes" in binding:
        require(path.stat().st_size == binding["bytes"], "Bound size changed")
    return path


def contracts():
    return {key: ref(CONTRACTS / f"GEO-03-{key}.json") for key in ("source", "cases", "tolerances")}


def check_contracts():
    source = read(CONTRACTS / "GEO-03-source.json")
    cases = read(CONTRACTS / "GEO-03-cases.json")
    tolerances = read(CONTRACTS / "GEO-03-tolerances.json")
    require(source["energyplus_commit"] == PIN and tolerances["frozen_before_numerical_comparison"] is True,
            "Source pin/tolerance preparation differs")
    for row in source["source_files"]:
        require(sha(SOURCE / row["path"]) == row["sha256"], "Pinned original file changed")
    for row in source["selected_ranges"]:
        lines = (SOURCE / row["path"]).read_bytes().splitlines(keepends=True)
        body = b"".join(lines[row["start"] - 1:row["end"]])
        require(hashlib.sha256(body).hexdigest() == row["range_sha256"], "Pinned original range changed")
    request = read(verify(cases["helper_request"]))
    require(request["schema"] == "geo03-helper-cases.v1" and len(request["cases"]) == cases["helper_case_count"],
            "Helper count/schema differs")
    require(sum(row["kind"] == "closed_box" for row in request["cases"]) == cases["paired_closed_boxes"],
            "Paired helper count differs")
    require(len(cases["native_cases"]) == cases["native_case_count"], "Native case count differs")
    for row in cases["native_cases"]:
        for key in ("input", "weather", "metadata"):
            verify(row[key])
    return {"contracts": contracts(), "helper_request": cases["helper_request"],
            "helper_case_count": cases["helper_case_count"], "native_case_count": cases["native_case_count"],
            "comparison_run": False, "gates_updated": False}


def verify_contract_review(path):
    review = read(path)
    require(review["schema"] == "geo03-independent-contract-review.v1" and
            review["status"] == "pass-before-scientific-execution", "Independent frozen-contract review required")
    current = {**contracts(), "helper_request": read(CONTRACTS / "GEO-03-cases.json")["helper_request"]}
    for key, value in current.items():
        archive = review["reviewed_contracts"][key]["archive"]
        verify(archive)
        require(archive["sha256"] == value["sha256"], "Independent reviewed contract differs")
    return ref(path)


def environment():
    env = os.environ.copy()
    env.update(PYTHONUTF8="1", PYTHONIOENCODING="utf-8")
    env["PATH"] = str(ROOT / ".runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin") + os.pathsep + env.get("PATH", "")
    return env


def process(command, directory, name):
    started = datetime.now(timezone.utc).isoformat()
    begin = time.perf_counter()
    with (directory / f"{name}-stdout.log").open("wb") as out, (directory / f"{name}-stderr.log").open("wb") as err:
        completed = subprocess.run(command, cwd=ROOT, env=environment(), stdout=out, stderr=err, check=False)
    receipt = {"command": command, "cwd": str(ROOT), "exit_code": completed.returncode,
               "started_utc": started, "finished_utc": datetime.now(timezone.utc).isoformat(),
               "elapsed_seconds": round(time.perf_counter() - begin, 3),
               "environment_overrides": {"PYTHONUTF8": "1", "PYTHONIOENCODING": "utf-8"},
               "runtime_path_prefix": str(ROOT / ".runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin"),
               "stdout": ref(directory / f"{name}-stdout.log"), "stderr": ref(directory / f"{name}-stderr.log")}
    write(directory / f"{name}-command.json", receipt)
    return receipt


def fresh(path):
    path = Path(path).resolve()
    require(path.is_relative_to(RAW.resolve()) and path != RAW.resolve() and not path.exists(),
            "Use a fresh directory inside .runtime/porting/GEO-03")
    path.mkdir(parents=True)
    return path


def archive_sources(directory):
    destination = directory / "source"
    destination.mkdir()
    rows = []
    for name in NAMES:
        original = ROOT / "tools/porting" / name
        archived = destination / name
        shutil.copyfile(original, archived)
        rows.append({"historical_path": original.relative_to(ROOT).as_posix(), "archive": ref(archived)})
    return rows


def original_source_guard(core):
    preservation = core["source_preservation"]
    proof = read(verify(preservation))
    snapshot = proof["archive_snapshot"]
    rows = read(verify(snapshot))["files"]
    present = 0
    for row in rows:
        path = SOURCE / row["path"]
        require(path.is_file() and path.stat().st_size == row["bytes"] and sha(path) == row["sha256"],
                "An original archive source changed/disappeared: " + row["path"])
        present += 1
    # These historical Git metadata absences are outside the archive table.
    extras = proof["missing_original_archive_non_scientific_git_metadata"]
    require(not set(extras).intersection(row["path"] for row in rows), "Historical absence unexpectedly belongs to archive rows")
    require(all(not (SOURCE / name).exists() for name in extras), "Historical extra absence changed")
    return {"original_preservation": preservation, "archive_snapshot": snapshot,
            "archive_rows_checked": len(rows), "unchanged_present_rows": present,
            "known_historical_git_metadata_absences_outside_archive_table": extras, "source_changes": []}


def verified_core():
    core = read(CORE)
    require(core["checks_passed"] is True and core["energyplus_commit"] == PIN and
            core["scientific_source_patches"] is False, "Verified unchanged genuine core required")
    for key in ("core_library", "api_library"):
        verify(core["artifacts"][key])
    return core


def build_driver(directory, review_path):
    check_contracts()
    contract_review = verify_contract_review(review_path)
    core = verified_core()
    source_before = original_source_guard(core)
    directory = fresh(directory)
    sources = archive_sources(directory)
    fragment_info = read(CONTRACTS / "GEO-03-source.json")["references"]["height_fragment"]
    require(fragment_info["start"] == 455 and fragment_info["end"] == 546, "Selected height-fragment boundary differs")
    lines = (SOURCE / fragment_info["path"]).read_bytes().splitlines(keepends=True)
    fragment = directory / "source/geo03_height455-546.inc"
    fragment.write_bytes(b"".join(lines[454:546]))
    require(sha(fragment) == fragment_info["range_sha256"], "Verbatim height fragment differs")
    baseline = read(OLD_CONFIGURE)
    require(baseline["exit_code"] == 0, "Historical configure was not successful")
    command = ["-DCMAKE_PROJECT_INCLUDE=" + str(ROOT / "tools/porting/geo03_reference.cmake")
               if token.startswith("-DCMAKE_PROJECT_INCLUDE=") else token for token in baseline["command"]]
    command.append("-DGEO03_HEIGHT_FRAGMENT=" + fragment.as_posix())
    trig_path = next(token.split("=", 1)[1] for token in command if token.startswith("-DGEO02_SETUP_FRAGMENT="))
    trig = ref(trig_path)
    trig_range = next(row for row in read(CONTRACTS / "GEO-03-source.json")["selected_ranges"]
                      if row["path"] == "src/EnergyPlus/SurfaceGeometry.cc" and row["start"] == 273 and row["end"] == 289)
    require(trig["sha256"] == trig_range["range_sha256"], "Existing original trig fragment changed")
    before = read(BUILD / "compile_commands.json")
    own = {"geo03_reference.cpp", "geo03_reference_helper.cpp"}
    previous = [row for row in before if Path(row["file"]).name not in own]
    own_previous = [row for row in before if Path(row["file"]).name in own]
    frozen = read(FROZEN_CORE_ROWS)
    require(len(frozen) == 643 and all(row in previous for row in frozen), "Original frozen compile rows differ")
    require(len(previous) == 650, "Prior-card/native compile baseline differs; review before build")
    write(directory / "previous-target-compile-commands.json", previous)
    products = [ref(path) for path in sorted((BUILD / "Products").glob("*.exe"))
                if path.name.startswith(("clk01_", "psy02_", "geo01_", "geo02_"))]
    require(len(products) == 7, "Prior-card executable inventory differs")
    objexx = BUILD / "Products/libobjexx.a"
    container = {**ref(objexx), "bytes": objexx.stat().st_size}
    configured = process(command, directory, "configure")
    require(configured["exit_code"] == 0, "Native configure failed; raw command/logs preserved")
    rows = read(BUILD / "compile_commands.json")
    require(all(row in rows for row in previous), "Original/prior target compile row changed")
    driver_rows = [next(row for row in rows if Path(row["file"]).name == name) for name in sorted(own)]
    for row in driver_rows:
        for flag in ("-DEP_psych_errors", "-UNDEBUG", "-Werror", "-O0", "-ffp-contract=off", "-Wa,-mbig-obj"):
            require(flag in row["command"], "Missing original native flag " + flag)
        for flag in ("-D_GLIBCXX_DEBUG", "-ffast-math", "-DEP_psych_stats"):
            require(flag not in row["command"], "Incorrect native ABI/mode " + flag)
    write(directory / "actual-driver-compile-commands.json", driver_rows)
    built = process([command[0], "--build", str(BUILD), "--target", "geo03_reference", "geo03_reference_helper",
                     "--parallel", "1", "--verbose"], directory, "build")
    require(built["exit_code"] == 0, "Native compile/link failed; raw command/logs preserved")
    after = read(BUILD / "compile_commands.json")
    require(all(row in after for row in previous) and all(row in after for row in frozen), "Prior compilation changed")
    for binding in products:
        verify(binding)
    for key in ("core_library", "api_library"):
        verify(core["artifacts"][key])
    verify(container)
    source_after = original_source_guard(core)
    require(source_before == source_after, "Original sources changed during native build")
    binaries = {}
    for name in ("geo03_reference", "geo03_reference_helper"):
        live = BUILD / "Products" / (name + ".exe")
        archived = directory / live.name
        shutil.copyfile(live, archived)
        binaries[name] = {"binary": ref(archived),
                          "linked_build_binary": {"historical_path": live.relative_to(ROOT).as_posix(), "sha256": sha(live)},
                          "matching_archive": ref(archived)}
    for name in ("CMakeCache.txt", "compile_commands.json"):
        shutil.copyfile(BUILD / name, directory / name)
    receipt = {"schema": "geo03-native-driver-build.v1", "checks_passed": True, "energyplus_commit": PIN,
               "core_build": ref(CORE), "source_contract": contracts()["source"], "contracts": contracts(),
               "independent_contract_review": contract_review, "binary": binaries["geo03_reference"]["binary"],
               "helper_binary": binaries["geo03_reference_helper"]["binary"], "binaries": binaries,
               "executed_source_archives": sources,
               "height_fragment": {"artifact": ref(fragment), "source_range": fragment_info, "whole_SetupZoneGeometry_claimed": False},
               "trig_fragment": {"artifact": trig, "source_range": trig_range, "reused_without_changing_old_target_path": True},
               "configure": ref(directory / "configure-command.json"), "compile_link": ref(directory / "build-command.json"),
               "actual_compile_commands": ref(directory / "actual-driver-compile-commands.json"),
               "cache": ref(directory / "CMakeCache.txt"), "compile_commands": ref(directory / "compile_commands.json"),
               "frozen_original_build_commands": ref(FROZEN_CORE_ROWS), "frozen_original_build_command_count": len(frozen),
               "previous_target_compile_commands": ref(directory / "previous-target-compile-commands.json"),
               "previous_card_compile_row_count": len(previous), "own_driver_prior_attempt_rows": own_previous,
               "own_driver_prior_attempt_policy": "only GEO03-owned fragment-path rows may change on preserved retry; all650 prior target rows stay exact",
               "previous_card_products": products, "linked_container_library": container,
               "original_source_guard": source_after, "original_core_and_API_bytes_unchanged": True,
               "existing_target_compile_commands_unchanged": True, "same_compiler_owned_state": True,
               "original_directory_and_target_definitions_inherited": True, "assertions_enabled": True,
               "fp_contract": "off", "scientific_source_patches": False, "gates_updated": False}
    write(directory / "native-driver-build.json", receipt)
    return {"driver_build": ref(directory / "native-driver-build.json"), "binary": receipt["binary"], "helper_binary": receipt["helper_binary"]}


def verified_driver(path, native_review_path):
    driver = read(path)
    require(driver["checks_passed"] is True and driver["energyplus_commit"] == PIN, "Unverified driver")
    require(driver["source_contract"] == contracts()["source"] and driver["contracts"] == contracts(), "Driver frozen contract differs")
    verify(driver["core_build"])
    require(driver["core_build"] == ref(CORE), "Driver original core differs")
    verified_core()
    for key in ("binary", "helper_binary"):
        verify(driver[key])
    for item in driver["executed_source_archives"]:
        archive = verify(item["archive"])
        require((ROOT / item["historical_path"]).read_bytes() == archive.read_bytes(), "Built source/current source differs")
    review = read(native_review_path)
    require(review["schema"] == "geo03-independent-native-build-final-review.v1" and
            review["status"] == "pass-build-identity-before-original-numerical-execution", "Independent native build review required")
    require(review["reviewed_build"] == ref(path), "Reviewed native build differs")
    return driver


def run_helpers(directory, driver_path, native_review_path):
    check_contracts()
    driver = verified_driver(driver_path, native_review_path)
    frozen = read(CONTRACTS / "GEO-03-cases.json")
    request = frozen["helper_request"]
    directory = fresh(directory)
    sources = archive_sources(directory)
    execution = process([str(ROOT / driver["helper_binary"]["path"]), str(ROOT / request["path"])], directory, "helper")
    output = directory / "helper-stdout.log"
    result = read(output) if output.stat().st_size else None
    if result is not None:
        write(directory / "helper-results.json", result)
    receipt = {"schema": "geo03-original-helper-execution.v1", "request": request,
               "results": ref(directory / "helper-results.json") if result else None,
               "execution": ref(directory / "helper-command.json"), "binary": driver["helper_binary"],
               "native_core_build": ref(CORE), "native_driver_build": ref(driver_path),
               "independent_native_build_review": ref(native_review_path), "contracts": contracts(),
               "executed_source_archives": sources, "reference_method": "genuine-prepared-original-state-whole-volume-helper-and-exact-height-fragment",
               "original_parser_admission_claimed": False, "physics_executed": False, "Rust_compared": False, "gates_updated": False}
    write(directory / "helper-reference.json", receipt)
    require(execution["exit_code"] == 0 and result is not None, "Original helper failed; preserved proof; stop before Rust pairing")
    require(result["schema"] == "geo03-helper-results.v1" and len(result["cases"]) == frozen["helper_case_count"], "Helper output schema/count differs")
    paired = [row for row in result["cases"] if row["kind"] == "closed_box"]
    require(len(paired) == frozen["paired_closed_boxes"] and all(row["kernel_input_identity_checked"] is True for row in paired),
            "Original input identity failed; stop before Rust pairing")
    return {"receipt": ref(directory / "helper-reference.json"), "results": receipt["results"], "paired_case_count": len(paired)}


def run_original(directory, driver_path, native_review_path, requested_ids):
    check_contracts()
    driver = verified_driver(driver_path, native_review_path)
    frozen = read(CONTRACTS / "GEO-03-cases.json")
    rows = frozen["native_cases"]
    if requested_ids:
        require(len(set(requested_ids)) == len(requested_ids), "Duplicate selected native case")
        rows = [row for row in rows if row["id"] in requested_ids]
        require(len(rows) == len(requested_ids), "Unknown selected native case")
    directory = fresh(directory)
    sources = archive_sources(directory)
    idd = BUILD / "Products/Energy+.idd"
    output_rows = []
    for row in rows:
        output = directory / row["id"]
        output.mkdir()
        command = [str(ROOT / driver["binary"]["path"]), str(ROOT), str(CORE), row["id"],
                   str(ROOT / row["input"]["path"]), str(ROOT / row["weather"]["path"]), str(idd), str(output / "native")]
        execution = process(command, output, "original")
        result_path = output / "native/volume-observation.json"
        data = read(result_path) if result_path.exists() else None
        receipt = {"schema": "geo03-original-execution.v1", "case_id": row["id"], "kind": row["kind"],
                   "execution": ref(output / "original-command.json"), "binary": driver["binary"],
                   "native_core_build": ref(CORE), "native_driver_build": ref(driver_path),
                   "independent_native_build_review": ref(native_review_path), "executed_source_archives": sources,
                   "contracts": contracts(), "input": row["input"], "weather": row["weather"], "idd": ref(idd),
                   "results": ref(result_path) if data else None,
                   "source_error_log": ref(output / "native/eplusout.err") if (output / "native/eplusout.err").exists() else None,
                   "reference_method": "genuine-full-original-input-parser-and-retained-zone-space-surface-state",
                   "native_outcome_policy": row.get("native_outcome_policy", "admitted-valid-success-and-retention"),
                   "physical_input_scope_expanded": False, "Rust_compared": False, "gates_updated": False}
        write(output / "receipt.json", receipt)
        output_rows.append({"case_id": row["id"], "kind": row["kind"], "receipt": ref(output / "receipt.json"), "results": receipt["results"]})
        write(directory / "matrix.json", {"schema": "geo03-original-matrix.v1", "cases": output_rows,
              "case_count": len(output_rows), "complete": len(output_rows) == frozen["native_case_count"],
              "requested_subset_complete": len(output_rows) == len(rows), "Rust_compared": False, "gates_updated": False})
        require(execution["exit_code"] == 0 and data is not None and data["callback_error"] == "", "Original wrapper/observer failed; proof retained")
        require(type(data["energyplus_exit_code"]) is int and type(data["physical_zone_callback_count"]) is int and
                data["physical_zone_callback_count"] >= 0, "Original exit/count metadata type differs")
        if row["kind"] == "ordinary-topology-diagnostic-not-CON":
            require(row["scope"] is None and row["native_outcome_policy"] == "observe-only-no-predetermined-exit",
                    "Unreviewed topology diagnostic policy")
            # Preserve actual EP rejection or fallback-with-physics. This row
            # supplies no expected exit, callback availability or error parity.
            continue
        require(data["energyplus_exit_code"] == 0 and data["first_final_zone_space_surface_identity_exact"] is True, "Original execution/state retention failed")
        require(data["physical_zone_callback_count"] == {"24H": 96, "72H": 288}[row["duration"]], "Original physical clock count differs")
        for phase in ("first_initialized", "first_physical", "final_weather", "final"):
            value = data[phase]
            require(len(value["surfaces"]) == 6 and len(value["zones"]) == 1 and len(value["spaces"]) == 1, "Frozen original topology differs")
            context = value["native_only_state"]["geometry_context"]
            require(context["total_coincident_vertices"] == context["total_degenerate_surfaces"] == 0 and
                    context["aspect_transform"] is False and context["no_transform"] is True, "Excluded geometry correction active")
            require(all(surface["sides"] == 4 for surface in value["surfaces"]), "Original surface cardinality changed")
        error = (output / "native/eplusout.err").read_text(encoding="utf-8")
        patterns = [r"coincident vert", r"degenerate surface", r"(?:roof/ceiling|floor) is (?:upside down|not oriented correctly)",
                    r"automatic fix is attempted", r"geometrytransform", r"non-planar surface"]
        require(not any(re.search(pattern, error, re.I) for pattern in patterns), "Excluded geometry-specific correction warning observed")
    return {"matrix": ref(directory / "matrix.json"), "case_count": len(output_rows)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--build-driver", action="store_true")
    mode.add_argument("--run-helpers", action="store_true")
    mode.add_argument("--run-original", action="store_true")
    parser.add_argument("--output-dir", type=Path)
    parser.add_argument("--contract-review", type=Path)
    parser.add_argument("--driver-build", type=Path)
    parser.add_argument("--native-review", type=Path)
    parser.add_argument("--case", action="append", default=[])
    args = parser.parse_args()
    if args.check:
        result = check_contracts()
    else:
        require(args.output_dir is not None, "Fresh --output-dir required")
        if args.build_driver:
            require(args.contract_review is not None, "--contract-review required before build")
            result = build_driver(args.output_dir, args.contract_review)
        else:
            require(args.driver_build is not None and args.native_review is not None, "--driver-build and --native-review required before execution")
            result = run_helpers(args.output_dir, args.driver_build, args.native_review) if args.run_helpers else run_original(args.output_dir, args.driver_build, args.native_review, args.case)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
