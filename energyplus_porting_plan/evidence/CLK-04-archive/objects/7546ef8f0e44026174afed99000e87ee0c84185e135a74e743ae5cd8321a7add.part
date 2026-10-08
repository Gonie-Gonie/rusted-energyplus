#!/usr/bin/env python3
"""ZON-01 draft native preparation and original-only execution, gated by reviews.

No Rust, scientific comparer, source math patch or gate update is performed.
The unchanged GEO-03 launcher supplies only hashing/process/core-identity helpers.
Root owns actual CMake/build/engine invocations; writing this tool is preparation.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import sys

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
spec = importlib.util.spec_from_file_location("zon01_unchanged_native_utilities", ROOT / "tools/porting/geo03_reference_native.py")
util = importlib.util.module_from_spec(spec)
spec.loader.exec_module(util)
require, read, write, sha, ref, verify = util.require, util.read, util.write, util.sha, util.ref, util.verify
SOURCE, BUILD, CORE, PIN = util.SOURCE, util.BUILD, util.CORE, util.PIN
RAW = ROOT / ".runtime/porting/ZON-01"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
OLD_BUILD = ROOT / ".runtime/porting/GEO-03/native-build-02/native-driver-build.json"
OWN_NAMES = ["zon01_reference.cpp", "zon01_reference_helper.cpp", "zon01_reference_fields.hh",
             "zon01_reference.cmake", "zon01_reference_native.py"]


def fresh(path):
    path = Path(path).resolve()
    require(path.is_relative_to(RAW.resolve()) and path != RAW.resolve() and not path.exists(),
            "A fresh contained ZON-01 directory is required")
    path.mkdir(parents=True)
    return path


def contracts():
    return {key: ref(CONTRACTS / f"ZON-01-{key}.json") for key in ("source", "cases", "tolerances")}


def check_contracts():
    source = read(CONTRACTS / "ZON-01-source.json")
    cases = read(CONTRACTS / "ZON-01-cases.json")
    tolerance = read(CONTRACTS / "ZON-01-tolerances.json")
    require(source["energyplus_commit"] == PIN and tolerance["frozen_before_numerical_comparison"] is True,
            "Original pin/frozen tolerance policy differs")
    for row in source["source_files"]:
        require(sha(SOURCE / row["path"]) == row["sha256"], "Pinned original file changed")
    for row in source["selected_ranges"]:
        lines = (SOURCE / row["path"]).read_bytes().splitlines(keepends=True)
        require(hashlib.sha256(b"".join(lines[row["start"] - 1:row["end"]])).hexdigest() == row["range_sha256"],
                "Pinned original range changed")
    request = read(verify(cases["helper_request"]))
    require(request["schema"] == "zon01-helper-cases.v1" and request["expected_values_supplied"] is False and
            len(request["sequences"]) == cases["helper_sequence_count"] == 8, "Input-only sequence contract differs")
    require(len(cases["native_cases"]) == cases["native_case_count"] == 7, "Native fixed-CON count differs")
    for row in cases["native_cases"]:
        for key in ("input", "weather", "metadata"):
            verify(row[key])
        require(row["scope"] in ("A", "B") and row["duration"] in ("24H", "72H"), "Unreviewed physical input scope")
    return {"contracts": contracts(), "helper_request": cases["helper_request"], "helper_sequence_count": 8,
            "native_case_count": 7, "comparison_run": False, "gates_updated": False}


def reviewed_contracts(path):
    review = read(path)
    require(review["schema"] == "zon01-independent-contract-review.v1" and
            review["status"] == "pass-before-scientific-execution", "Independent pre-run contract PASS required")
    current = {**contracts(), "helper_request": read(CONTRACTS / "ZON-01-cases.json")["helper_request"]}
    for key, item in current.items():
        archive = verify(review["reviewed_contracts"][key]["archive"])
        require(archive.read_bytes() == verify(item).read_bytes(), "Reviewed contract content differs")
    return ref(path)


def archive_sources(directory):
    destination = directory / "source"
    destination.mkdir()
    files = [HERE / name for name in OWN_NAMES] + [ROOT / "tools/porting" / name for name in util.NAMES]
    rows = []
    for original in files:
        archived = destination / original.name
        require(not archived.exists(), "Source archive basename collision")
        shutil.copyfile(original, archived)
        rows.append({"historical_path": original.relative_to(ROOT).as_posix(), "archive": ref(archived)})
    return rows


def prior_native_identity():
    old = read(OLD_BUILD)
    require(old["checks_passed"] is True and old["energyplus_commit"] == PIN, "Prior actual GEO03 build missing")
    for row in old["executed_source_archives"]:
        require((ROOT / row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Prior-card source changed")
    old_commands = read(verify(old["compile_commands"]))
    require(len(old_commands) == 652, "Prior-card compile inventory differs")
    for name in ("binary", "helper_binary"):
        verify(old[name])
    return old, old_commands


def build_driver(directory, contract_review):
    check_contracts()
    review_binding = reviewed_contracts(contract_review)
    core = util.verified_core()
    old, old_commands = prior_native_identity()
    source_before = util.original_source_guard(core)
    directory = fresh(directory)
    sources = archive_sources(directory)
    row = next(row for row in read(CONTRACTS / "ZON-01-source.json")["selected_ranges"]
               if row["path"] == "src/EnergyPlus/HeatBalanceSurfaceManager.cc" and row["start"] == 2231 and row["end"] == 2239)
    lines = (SOURCE / row["path"]).read_bytes().splitlines(keepends=True)
    fragment = directory / "source/zon01_bulk2231-2239.inc"
    fragment.write_bytes(b"".join(lines[2230:2239]))
    require(sha(fragment) == row["range_sha256"], "Exact zone-only bulk fragment changed")
    baseline = read(verify(old["configure"]))
    require(baseline["exit_code"] == 0, "Prior original configure was unsuccessful")
    command = ["-DCMAKE_PROJECT_INCLUDE=" + str(HERE / "zon01_reference.cmake")
               if token.startswith("-DCMAKE_PROJECT_INCLUDE=") else token for token in baseline["command"]]
    command += ["-DZON01_REPOSITORY_ROOT=" + ROOT.as_posix(), "-DZON01_BULK_FRAGMENT=" + fragment.as_posix()]
    current = read(BUILD / "compile_commands.json")
    own = {"zon01_reference.cpp", "zon01_reference_helper.cpp"}
    previous = [item for item in current if Path(item["file"]).name not in own]
    own_previous = [item for item in current if Path(item["file"]).name in own]
    require(previous == old_commands, "Any prior original/card compile row changed")
    frozen = read(util.FROZEN_CORE_ROWS)
    require(len(frozen) == 643 and all(item in previous for item in frozen), "Original core compile rows differ")
    write(directory / "previous-target-compile-commands.json", previous)
    products = [ref(path) for path in sorted((BUILD / "Products").glob("*.exe"))
                if path.name.startswith(("clk01_", "psy02_", "geo01_", "geo02_", "geo03_"))]
    require(len(products) == 9, "Prior-card executable inventory differs")
    container = {**ref(BUILD / "Products/libobjexx.a"), "bytes": (BUILD / "Products/libobjexx.a").stat().st_size}
    configured = util.process(command, directory, "configure")
    require(configured["exit_code"] == 0, "Original native configure failed; raw receipt/logs preserved")
    rows = read(BUILD / "compile_commands.json")
    require([item for item in rows if Path(item["file"]).name not in own] == previous, "Prior target compile row changed")
    driver_rows = [next(item for item in rows if Path(item["file"]).name == name) for name in sorted(own)]
    for item in driver_rows:
        for flag in ("-DEP_psych_errors", "-UNDEBUG", "-Werror", "-O0", "-ffp-contract=off", "-Wa,-mbig-obj"):
            require(flag in item["command"], "Required original flag missing: " + flag)
        for flag in ("-D_GLIBCXX_DEBUG", "-ffast-math", "-DEP_psych_stats"):
            require(flag not in item["command"], "Incorrect original ABI/mode flag: " + flag)
    write(directory / "actual-driver-compile-commands.json", driver_rows)
    compiled = util.process([command[0], "--build", str(BUILD), "--target", "zon01_reference", "zon01_reference_helper",
                             "--parallel", "1", "--verbose"], directory, "build")
    require(compiled["exit_code"] == 0, "Compile/link failed; preserve failure before retry")
    after = read(BUILD / "compile_commands.json")
    require([item for item in after if Path(item["file"]).name not in own] == previous, "Prior compile rows changed")
    for binding in products:
        verify(binding)
    for key in ("core_library", "api_library"):
        verify(core["artifacts"][key])
    verify(container)
    source_after = util.original_source_guard(core)
    require(source_after == source_before, "An original source changed during native build")
    binaries = {}
    for name in ("zon01_reference", "zon01_reference_helper"):
        live = BUILD / "Products" / (name + ".exe")
        archived = directory / live.name
        shutil.copyfile(live, archived)
        binaries[name] = {"binary": ref(archived), "linked_build_binary":
                          {"historical_path": live.relative_to(ROOT).as_posix(), "sha256": sha(live)},
                          "matching_archive": ref(archived)}
    for name in ("CMakeCache.txt", "compile_commands.json"):
        shutil.copyfile(BUILD / name, directory / name)
    receipt = {"schema": "zon01-native-driver-build.v1", "checks_passed": True, "energyplus_commit": PIN,
        "core_build": ref(CORE), "prior_native_build": ref(OLD_BUILD), "contracts": contracts(),
        "source_contract": contracts()["source"], "independent_contract_review": review_binding,
        "binary": binaries["zon01_reference"]["binary"], "helper_binary": binaries["zon01_reference_helper"]["binary"],
        "binaries": binaries, "executed_source_archives": sources,
        "bulk_fragment": {"artifact": ref(fragment), "source_range": row, "whole_surface_initializer_claimed": False},
        "configure": ref(directory / "configure-command.json"), "compile_link": ref(directory / "build-command.json"),
        "actual_compile_commands": ref(directory / "actual-driver-compile-commands.json"),
        "cache": ref(directory / "CMakeCache.txt"), "compile_commands": ref(directory / "compile_commands.json"),
        "frozen_original_build_commands": ref(util.FROZEN_CORE_ROWS), "frozen_original_build_command_count": 643,
        "previous_target_compile_commands": ref(directory / "previous-target-compile-commands.json"),
        "previous_card_compile_row_count": len(previous), "own_driver_prior_attempt_rows": own_previous,
        "previous_card_products": products, "linked_container_library": container, "original_source_guard": source_after,
        "original_core_and_API_bytes_unchanged": True, "existing_target_compile_commands_unchanged": True,
        "same_compiler_owned_state": True, "original_directory_and_target_definitions_inherited": True,
        "assertions_enabled": True, "fp_contract": "off", "scientific_source_patches": False, "gates_updated": False}
    write(directory / "native-driver-build.json", receipt)
    return {"driver_build": ref(directory / "native-driver-build.json"), "binary": receipt["binary"], "helper_binary": receipt["helper_binary"]}


def verified_driver(path, review_path):
    driver = read(path)
    require(driver["checks_passed"] is True and driver["energyplus_commit"] == PIN and
            driver["contracts"] == contracts() and driver["source_contract"] == contracts()["source"], "Frozen native driver differs")
    require(driver["core_build"] == ref(CORE), "Driver original core identity differs")
    util.verified_core()
    for key in ("binary", "helper_binary"):
        verify(driver[key])
    for row in driver["executed_source_archives"]:
        require((ROOT / row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Built/current driver source differs")
    review = read(review_path)
    require(review["schema"] == "zon01-independent-native-build-final-review.v1" and
            review["status"] == "pass-build-identity-before-original-numerical-execution" and
            review["reviewed_build"] == ref(path), "Independent actual native build PASS required")
    return driver


def run_helpers(directory, path, review):
    check_contracts()
    driver = verified_driver(path, review)
    frozen = read(CONTRACTS / "ZON-01-cases.json")
    request = frozen["helper_request"]
    directory = fresh(directory)
    sources = archive_sources(directory)
    execution = util.process([str(verify(driver["helper_binary"])), str(verify(request))], directory, "helper")
    output = directory / "helper-stdout.log"
    result = read(output) if output.stat().st_size else None
    if result is not None:
        write(directory / "helper-results.json", result)
    receipt = {"schema": "zon01-original-helper-execution.v1", "request": request,
        "results": ref(directory / "helper-results.json") if result is not None else None,
        "execution": ref(directory / "helper-command.json"), "binary": driver["helper_binary"],
        "native_core_build": ref(CORE), "native_driver_build": ref(path), "independent_native_build_review": ref(review),
        "contracts": contracts(), "executed_source_archives": sources,
        "reference_method": "genuine-concrete-zone-member-and-whole-manager-with-exact-incoming-zone-bulk-fragment",
        "original_parser_admission_claimed": False, "physics_executed": False, "Rust_compared": False, "gates_updated": False}
    write(directory / "helper-reference.json", receipt)
    require(execution["exit_code"] == 0 and result is not None, "Original helper failed; stop before Rust comparison")
    require(result["schema"] == "zon01-helper-results.v1" and len(result["sequences"]) == frozen["helper_sequence_count"],
            "Actual helper output count/schema differs")
    return {"receipt": ref(directory / "helper-reference.json"), "results": receipt["results"]}


def run_original(directory, path, review, requested_ids):
    check_contracts()
    driver = verified_driver(path, review)
    frozen = read(CONTRACTS / "ZON-01-cases.json")
    rows = frozen["native_cases"]
    if requested_ids:
        require(len(set(requested_ids)) == len(requested_ids), "Duplicate selected native ID")
        rows = [row for row in rows if row["id"] in requested_ids]
        require(len(rows) == len(requested_ids), "Unknown selected native ID")
    directory = fresh(directory)
    sources = archive_sources(directory)
    idd = BUILD / "Products/Energy+.idd"
    output_rows = []
    for row in rows:
        output = directory / row["id"]
        output.mkdir()
        command = [str(verify(driver["binary"])), str(ROOT), str(CORE), row["id"], str(verify(row["input"])),
                   str(verify(row["weather"])), str(idd), str(output / "native")]
        execution = util.process(command, output, "original")
        result_path = output / "native/zone-air-observation.json"
        result = read(result_path) if result_path.exists() else None
        receipt = {"schema": "zon01-original-execution.v1", "case_id": row["id"], "execution": ref(output / "original-command.json"),
            "binary": driver["binary"], "native_core_build": ref(CORE), "native_driver_build": ref(path),
            "independent_native_build_review": ref(review), "contracts": contracts(), "executed_source_archives": sources,
            "input": row["input"], "weather": row["weather"], "idd": ref(idd), "results": ref(result_path) if result is not None else None,
            "source_error_log": ref(output / "native/eplusout.err") if (output / "native/eplusout.err").exists() else None,
            "reference_method": "genuine-full-original-input-parser-with-passive-source-stage-callbacks",
            "pristine_member_output_pairing_claimed": False, "warmup_solver_parity_claimed": False,
            "physical_input_scope_expanded": False, "Rust_compared": False, "gates_updated": False}
        write(output / "receipt.json", receipt)
        output_rows.append({"case_id": row["id"], "receipt": ref(output / "receipt.json"), "results": receipt["results"]})
        write(directory / "matrix.json", {"schema": "zon01-original-matrix.v1", "cases": output_rows, "case_count": len(output_rows),
            "complete": len(output_rows) == frozen["native_case_count"], "requested_subset_complete": len(output_rows) == len(rows),
            "Rust_compared": False, "gates_updated": False})
        require(execution["exit_code"] == 0 and result is not None and result["callback_error"] == "" and
                type(result["energyplus_exit_code"]) is int and result["energyplus_exit_code"] == 0, "Original wrapper/run failed; retain evidence")
        require(type(result["recorded_callback_count"]) is int and result["recorded_callback_count"] > 0 and
                type(result["omitted_callback_count"]) is int and result["omitted_callback_count"] == 0, "Passive trace omission/type differs")
        for phase in ("after_init_heat_balance", "before_predictor", "after_predictor_before_hvac_managers"):
            require(phase in result["first_by_phase"] and phase in result["first_physical_by_phase"], "Missing actual native phase")
        expected_physical = {"24H": 96, "72H": 288}[row["duration"]]
        require(result["callback_counts"]["after_init_heat_balance"]["physical_weather"] == expected_physical,
                "Actual ordinary zone invocation count differs")
        verify(result["ordered_observations"])
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
        require(args.output_dir is not None, "Fresh output directory required")
        if args.build_driver:
            require(args.contract_review is not None, "Independent contract review required")
            result = build_driver(args.output_dir, args.contract_review)
        else:
            require(args.driver_build is not None and args.native_review is not None, "Independent build and actual driver receipts required")
            result = run_helpers(args.output_dir, args.driver_build, args.native_review) if args.run_helpers else run_original(args.output_dir, args.driver_build, args.native_review, args.case)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
