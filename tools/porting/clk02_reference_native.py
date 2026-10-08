"""Build only the new genuine CLK-02 helper and retain a validated derivative.

No original scientific request, Rust, comparer, Git, registry admission or removal
is executed. Independent contract/utility reviews precede the build; a separate
independent build review is required before Root authorizes original execution.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import sys
import traceback

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
HERE = ROOT / "tools/porting"
spec = importlib.util.spec_from_file_location("clk02_unchanged_native_utilities", HERE / "geo03_reference_native.py")
util = importlib.util.module_from_spec(spec)
spec.loader.exec_module(util)
require, read, sha, ref, verify = util.require, util.read, util.sha, util.ref, util.verify
SOURCE, BUILD, CORE, PIN = util.SOURCE, util.BUILD, util.CORE, util.PIN
RAW = ROOT / ".runtime/porting/CLK-02"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
PRIOR = ROOT / ".runtime/porting/ZON-01/native-build-01/native-driver-build.json"
REGISTRY_SHA = "f2455dc3cc74f5840fab60b58aba79b01b250bb15dcf399932915ddd6a43abc6"
PRIOR_SHA = "4fd88058f53aae38b49dc774550cb4b875ec3516275dc323c97fb24f8039bdd9"
DERIVER_SHA = "3b39cc4562943692e43e11e4d2d7e1bc70f84a58e09c0fea28c0b85b8e905dcf"
PE_READER_SHA = "afef047aec5a0bb5be90b6da155688b9bbaa672dd4dd90615569396d306cba56"
TARGET = "clk02_reference_helper"
OWN = ["clk02_reference_helper.cpp", "clk02_reference_fields.hh", "clk02_reference.cmake", "clk02_reference_native.py"]


def utc():
    return datetime.now(timezone.utc).isoformat()


def path(value):
    result = Path(value)
    result = (ROOT / result).resolve() if not result.is_absolute() else result.resolve()
    require(result.is_relative_to(ROOT) and result != ROOT, "Path outside repository")
    return result


def write(destination, value):
    with destination.open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(value, stream, indent=2, allow_nan=False)
        stream.write("\n")


def timestamp(value):
    result = datetime.fromisoformat(value)
    require(result.utcoffset() is not None and result.utcoffset().total_seconds() == 0, "Aware UTC timestamp required")
    return result


def contracts():
    return {key: ref(CONTRACTS / f"CLK-02-{key}.json") for key in ("source", "cases", "tolerances")}


def check_contracts():
    source, cases, tolerances = [read(CONTRACTS / f"CLK-02-{key}.json") for key in ("source", "cases", "tolerances")]
    require(source["schema"] == "clk02-source-contract.v1" and source["energyplus_commit"] == PIN
            and cases["schema"] == "clk02-cases-contract.v1"
            and tolerances["frozen_before_numerical_execution"] is True, "Source/cases/frozen tolerance policy differs")
    for item in (source, cases, tolerances):
        require(item["status"] == "frozen-before-numerical-execution"
                and item["frozen_before_numerical_execution"] is True, "Final frozen contract required")
    for row in source["source_files"]:
        require(sha(SOURCE / row["path"]) == row["sha256"], "Pinned original file differs")
    for row in source["selected_ranges"]:
        lines = (SOURCE / row["file"]).read_bytes().splitlines(keepends=True)
        require(hashlib.sha256(b"".join(lines[row["start_line"] - 1:row["end_line"]])).hexdigest() == row["range_sha256"],
                "Pinned original source range differs")
    request = read(verify(cases["helper_request"]))
    require(request["schema"] == "clk02-helper-cases.v1" and request["expected_values_supplied"] is False
            and request["expected_exits_supplied"] is False, "Input-only helper request required")
    counts = cases["counts"]
    require(len(request["record_sequences"]) == counts["record_sequences"] == 40
            and sum(len(row["operations"]) for row in request["record_sequences"]) == counts["diagnostic_raw_calls"] == 46
            and len(request["header_cases"]) == 22 and counts["total_header_roots"] == 23
            and request["fixed_epw"]["record_count"] == counts["fixed_epw_raw_calls"] == 8760
            and counts["total_raw_calls"] == 8806 and request["fixed_epw"]["physics_executed"] is False,
            "Frozen parser-only counts differ")
    verify(cases["fixed_CON_scope"])
    verify(request["fixed_epw"]["file"])
    for row in request["header_cases"]:
        for key in ("stream", "file"):
            if key in row:
                verify(row[key])
    return {"contracts": contracts(), "helper_request": cases["helper_request"], "counts": counts}


def contract_review(value):
    review_path = path(value)
    record = read(review_path)
    require(record["schema"] == "clk02-independent-contract-review.v1"
            and record["status"] == "pass-before-scientific-execution", "Independent frozen-contract PASS required")
    current = {**contracts(), "helper_request": read(CONTRACTS / "CLK-02-cases.json")["helper_request"]}
    for key, binding in current.items():
        require(verify(record["reviewed_contracts"][key]["archive"]).read_bytes() == verify(binding).read_bytes(),
                "Independently reviewed contract bytes differ")
    timestamp(record["review_completed_utc"])
    return review_path, record


def registry(value):
    registry_path = path(value)
    require(sha(registry_path) == REGISTRY_SHA, "Explicit frozen legacy registry required")
    record = read(registry_path)
    require(record["schema"] == "native-retained-artifact-registry.v1" and record["status"] == "frozen-legacy-baseline-only"
            and record["energyplus_commit"] == PIN and record["legacy_product_count"] == 11
            and len(record["legacy_products"]) == 11 and record["new_targets"] == []
            and record["removal_authorized"] is False, "Legacy-only registry differs")
    require(len({row["target"] for row in record["legacy_products"]}) == 11, "Duplicate legacy target")
    for row in record["legacy_products"]:
        live = verify(row["binary"])
        require(live.parent == BUILD / "Products" and live.name == row["target"] + ".exe"
                and live.stat().st_size == row["size_bytes"], "Legacy Product identity differs")
    for row in record["protected_core_API_and_container"]:
        verify(row)
    require(len(record["protected_core_API_and_container"]) == 3, "Protected core/API/container inventory differs")
    return registry_path, record


def prior_identity():
    require(sha(PRIOR) == PRIOR_SHA, "Historical ZON build receipt differs")
    record = read(PRIOR)
    require(record["schema"] == "zon01-native-driver-build.v1" and record["checks_passed"] is True
            and record["energyplus_commit"] == PIN and record["scientific_source_patches"] is False
            and verify(record["core_build"]) == CORE, "Genuine prior build/core required")
    for row in record["executed_source_archives"]:
        require(path(row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Prior source/target CMake changed")
    rows = read(verify(record["compile_commands"]))
    require(len(rows) == 654, "Historical all-target row count differs")
    fragment = record["bulk_fragment"]
    selected = fragment["source_range"]
    require(selected["path"] == "src/EnergyPlus/HeatBalanceSurfaceManager.cc" and selected["start"] == 2231
            and selected["end"] == 2239 and fragment["artifact"]["sha256"] == selected["range_sha256"],
            "Historical genuine bulk fragment binding differs")
    lines = (SOURCE / selected["path"]).read_bytes().splitlines(keepends=True)
    require(verify(fragment["artifact"]).read_bytes() == b"".join(lines[2230:2239]), "Retained bulk source fragment differs")
    return record, rows


def utility_identity(args):
    review_path, validation_path = path(args.derivation_review), path(args.derivation_validation)
    actual_path = path(args.derivation_actual_review)
    review, validation = read(review_path), read(validation_path)
    actual = read(actual_path)
    require(review["schema"] == "native-debug-derivative-independent-amendment-review.v1"
            and review["status"] == "pass-bounded-amendment-before-fresh-transformation", "Independent utility amendment PASS required")
    expected = {"PE_reader": PE_READER_SHA, "derivation_driver": DERIVER_SHA}
    for key, wanted in expected.items():
        row = review["reviewed_sources"][key]
        require(row["archive"]["sha256"] == wanted and sha(path(row["historical_path"])) == wanted,
                "Reviewed derivative utility source differs")
        verify(row["archive"])
    validate_derivative(validation)
    require(actual["schema"] == "native-debug-derivative-independent-actual-review.v1"
            and actual["status"] == "pass-structural-derivative-before-execution"
            and actual["reviewed_derivation"] == ref(validation_path)
            and actual["reviewed_static_amendment"] == ref(review_path)
            and actual["reviewed_registry"] == ref(path(args.registry)), "Actual utility structural-review binding differs")
    require(actual["actual_original_whole_file_SHA_verified"] is True
            and actual["actual_derivative_whole_file_SHA_verified"] is True
            and actual["reviewed_maps"] == validation["maps"] and actual["scientific_execution_certified"] is False,
            "Independent actual utility proof differs")
    require(timestamp(validation["finished_utc"]) <= timestamp(actual["review_completed_utc"]),
            "Actual structural review must follow the completed legacy trial")
    runtime_path, runtime_review_path = path(args.derivation_runtime_validation), path(args.derivation_runtime_review)
    runtime, runtime_review = read(runtime_path), read(runtime_review_path)
    require(runtime["schema"] == "native-debug-derivative-bounded-execution-check.v1"
            and runtime["status"] == "pass-bounded-same-input-helper-results"
            and runtime["derivation"] == ref(validation_path)
            and runtime["independent_actual_PE_review"] == ref(actual_path)
            and runtime["registry"] == ref(path(args.registry)), "Bounded utility runtime binding differs")
    for key in ("stdout_byte_equal_to_original", "structured_results_exactly_equal", "input_and_binary_unchanged",
                "legacy_core_API_container_unchanged", "source_unchanged"):
        require(runtime[key] is True, "Bounded utility preservation check failed: " + key)
    require(runtime["binary_before"] == runtime["binary_after"] == validation["helper_binary"]
            and runtime["sequence_count"] == 8 and runtime["new_card_scientific_certification"] is False
            and runtime["old_card_proofs_or_gates_changed"] is False and runtime["deletion_performed"] is False,
            "Bounded utility scope differs")
    command = read(verify(runtime["command"]))
    require(type(command["exit_code"]) is int and command["exit_code"] == 0
            and command["command"] == [str(verify(runtime["binary_before"])), str(verify(runtime["request"]))],
            "Actual bounded utility command differs")
    verify(command["stderr"])
    original = read(verify(runtime["preserved_original_reference"]))
    require(original["request"] == runtime["request"], "Bounded utility original input differs")
    original_command = read(verify(original["execution"]))
    require(verify(command["stdout"]).read_bytes() == verify(original_command["stdout"]).read_bytes(),
            "Bounded utility raw output bytes differ from the original")
    request = read(verify(runtime["request"]))
    require(len(request["sequences"]) == 8 and sum(len(row["operations"]) for row in request["sequences"]) == 34,
            "Bounded utility input counts differ")
    require(runtime_review["schema"] == "native-debug-derivative-independent-runtime-review.v1"
            and runtime_review["status"] == "pass-bounded-same-input-preserved-data", "Independent utility runtime PASS required")
    for key, binding in {"reviewed_bounded_execution": ref(runtime_path), "reviewed_actual_command": runtime["command"],
                         "reviewed_derivation": ref(validation_path), "reviewed_independent_PE_review": ref(actual_path),
                         "reviewed_original_reference": runtime["preserved_original_reference"], "reviewed_request": runtime["request"],
                         "reviewed_binary": validation["helper_binary"], "reviewed_registry": runtime["registry"]}.items():
        require(runtime_review[key] == binding, "Independent utility runtime reference differs: " + key)
    require(timestamp(actual["review_completed_utc"]) <= timestamp(runtime["started_utc"])
            and timestamp(runtime["finished_utc"]) <= timestamp(runtime_review["review_completed_utc"]),
            "Utility PE review / bounded execution / data review chronology differs")
    return {"independent_review": ref(review_path), "prior_successful_validation": ref(validation_path),
            "independent_actual_review": ref(actual_path), "prior_bounded_runtime_validation": ref(runtime_path),
            "independent_runtime_review": ref(runtime_review_path)}


def validate_derivative(record, linked=None):
    require(record["schema"] == "validated-debug-derivative.v1"
            and record["status"] == "pass-exact-retained-runtime-sections-and-directories"
            and record["original_bytes_unchanged"] is True and record["source_bytes_match_before_and_after"] is True
            and record["accepted_derivative_bytes_unchanged"] is True and record["original_removed"] is False
            and record["full_original_copied"] is False and record["scientific_execution_certified"] is False,
            "Successful bounded derivative proof required")
    for key, expected in (("executed_driver", DERIVER_SHA), ("executed_PE_reader", PE_READER_SHA)):
        require(record[key]["sha256"] == record[key]["matching_archive"]["sha256"] == expected,
                "Unreviewed executed utility source")
        verify(record[key]["matching_archive"])
    require(record["original_before"] == record["original_after"]
            and record["accepted_derivative"] == record["observed_derivative"], "Derivative preservation binding differs")
    for binding in record["maps"].values():
        verify(binding)
    before = read(verify(record["maps"]["original_before"]))
    after = read(verify(record["maps"]["original_after"]))
    derived = read(verify(record["maps"]["derived"]))
    require(before == after and before["sha256"] == record["original_before"]["sha256"]
            and before["size_bytes"] == record["original_before"]["size_bytes"]
            and derived["sha256"] == record["accepted_derivative"]["sha256"]
            and derived["size_bytes"] == record["accepted_derivative"]["size_bytes"], "PE map/payload identity differs")
    comparison = read(verify(record["maps"]["comparison"]))
    require(comparison == record["comparison"] and comparison["status"] == record["status"]
            and comparison["code_or_data_masked"] is False and comparison["scientific_execution_certified"] is False,
            "Accepted retained-byte comparison differs")
    require(record["strip_command"]["exit_code"] == 0 and type(record["strip_command"]["exit_code"]) is int,
            "Actual strip did not succeed")
    require(read(verify(record["strip_command_receipt"])) == record["strip_command"], "Actual strip command receipt differs")
    strip = record["strip_command"]["command"]
    require(len(strip) == 6 and strip[1:4] == ["--strip-debug", "--preserve-dates", "-o"]
            and path(strip[4]) == path(record["helper_binary"]["path"])
            and path(strip[5]) == path(record["original_before"]["historical_path"]), "Actual derivation argv differs")
    for key in ("stdout", "stderr"):
        verify(record["strip_command"][key])
    helper = verify(record["helper_binary"])
    accepted = record["accepted_derivative"]
    require(ref(helper) == record["helper_binary"] and record["helper_binary"]["path"] == accepted["path"]
            and sha(helper) == accepted["sha256"] and helper.stat().st_size == accepted["size_bytes"], "Accepted derivative bytes differ")
    if linked is not None:
        require(record["original_before"] == {**linked, "retention_class": "prospective-debug-original"},
                "Derivative input is not this actual linked helper")
    return record["helper_binary"]


def archive_sources(directory, old, utility):
    destination = directory / "source"
    destination.mkdir()
    files = {path(row["historical_path"]) for row in old["executed_source_archives"]}
    files.update(HERE / name for name in OWN + ["derive_native_helper.py", "native_pe.py", "freeze_native_registry.py"])
    rows = []
    for original in sorted(files):
        archived = destination / original.name
        require(not archived.exists(), "Source archive basename collision")
        shutil.copyfile(original, archived)
        require(sha(original) == sha(archived), "Source changed while archiving")
        rows.append({"historical_path": original.relative_to(ROOT).as_posix(), "archive": ref(archived)})
    for name, binding in utility.items():
        shutil.copyfile(verify(binding), destination / (name + ".json"))
    return rows


def preservation(previous, frozen, core, expected_source, sources, registry_path):
    _, legacy = registry(registry_path)
    verify(core["compiler"]["binary"])
    verify(core["compiler"]["toolchain_publisher_receipt"])
    verify(core["compiler"]["object_cache_publisher_receipt"])
    current = read(BUILD / "compile_commands.json")
    require([row for row in current if Path(row["file"]).name != TARGET + ".cpp"] == previous,
            "Any of all654 historical compile rows changed")
    require(len(frozen) == 643 and all(row in current for row in frozen), "Original643 compile rows changed")
    guard = util.original_source_guard(core)
    require(guard == expected_source, "Original source-preservation snapshot differs")
    for row in sources:
        require(path(row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Archived build source changed")
    return {"observed_utc": utc(), "legacy_products_checked": len(legacy["legacy_products"]),
            "historical_compile_rows_checked": len(previous), "frozen_core_rows_checked": len(frozen),
            "core_API_container_and_legacy_bytes_unchanged": True, "source_guard": guard,
            "archived_build_sources_unchanged": True}


def build_driver(args):
    directory = path(args.output_dir)
    require(directory.is_relative_to(RAW.resolve()) and directory != RAW.resolve() and not directory.exists(),
            "Fresh contained CLK-02 output directory required")
    directory.mkdir(parents=True)
    receipt = {"schema": "clk02-native-driver-build.v1", "started_utc": utc(), "actual_argv": sys.orig_argv,
               "actual_cwd": str(Path.cwd().resolve()), "energyplus_commit": PIN, "checks_passed": False,
               "preservation_observations": {}, "scientific_execution_performed": False, "scientific_source_patches": False,
               "Cargo_or_Git_executed": False, "gates_updated": False, "original_removed": False,
               "full_linked_original_copied": False, "registry_admission_performed": False,
               "independent_build_review_required_before_original_execution": True,
               "derivative_utility_limits": ["Existing ZON-01 successful inputs only; no CLK-02 scientific request executed here.",
                    "Forced C++ throw/error-unwind execution is not certified by the utility foundation."]}
    baseline = frozen = core = source_before = sources = registry_path = None
    try:
        receipt.update(check_contracts())
        reviewed_path, reviewed = contract_review(args.contract_review)
        registry_path, _ = registry(args.registry)
        old, baseline = prior_identity()
        utility = utility_identity(args)
        for key in ("independent_review", "independent_actual_review", "independent_runtime_review"):
            require(timestamp(read(verify(utility[key]))["review_completed_utc"]) <= timestamp(receipt["started_utc"]),
                    "Utility reviews must precede this new CLK-02 build")
        core = util.verified_core()
        frozen = read(util.FROZEN_CORE_ROWS)
        source_before = util.original_source_guard(core)
        sources = archive_sources(directory, old, utility)
        receipt.update(core_build=ref(CORE), prior_native_build=ref(PRIOR), independent_contract_review=ref(reviewed_path),
                       retained_artifact_registry=ref(registry_path), derivative_utility=utility,
                       executed_source_archives=sources, bulk_fragment=old["bulk_fragment"],
                       frozen_original_build_commands=ref(util.FROZEN_CORE_ROWS), frozen_original_build_command_count=643,
                       previous_card_compile_row_count=654)
        receipt["preservation_observations"]["before_configure"] = preservation(baseline, frozen, core, source_before, sources, registry_path)
        util.write(directory / "previous-target-compile-commands.json", baseline)
        receipt["previous_target_compile_commands"] = ref(directory / "previous-target-compile-commands.json")
        prior_configure = read(verify(old["configure"]))
        require(prior_configure["exit_code"] == 0, "Historical configure failed")
        require(sum(token.startswith("-DCMAKE_PROJECT_INCLUDE=") for token in prior_configure["command"]) == 1,
                "Historical project registration differs")
        command = ["-DCMAKE_PROJECT_INCLUDE=" + str(HERE / "clk02_reference.cmake")
                   if token.startswith("-DCMAKE_PROJECT_INCLUDE=") else token for token in prior_configure["command"]]
        command.append("-DCLK02_REPOSITORY_ROOT=" + ROOT.as_posix())
        require(timestamp(reviewed["review_completed_utc"]) <= timestamp(utc()), "Contract review is in the future")
        configured = util.process(command, directory, "configure")
        receipt["configure"] = ref(directory / "configure-command.json")
        require(configured["exit_code"] == 0, "Configure failed; actual receipt/logs preserved")
        require(timestamp(reviewed["review_completed_utc"]) <= timestamp(configured["started_utc"]),
                "Independent contract review must precede actual configure")
        receipt["preservation_observations"]["after_configure"] = preservation(baseline, frozen, core, source_before, sources, registry_path)
        rows = read(BUILD / "compile_commands.json")
        own = [row for row in rows if Path(row["file"]).name == TARGET + ".cpp"]
        require(len(rows) == 655 and len(own) == 1 and path(own[0]["file"]) == HERE / (TARGET + ".cpp"), "Only one new helper compile row allowed")
        for flag in ("-DEP_psych_errors", "-UNDEBUG", "-Werror", "-O0", "-ffp-contract=off", "-Wa,-mbig-obj", "-std=c++20"):
            require(flag in own[0]["command"].split(), "Missing inherited original compile flag " + flag)
        for flag in ("_GLIBCXX_DEBUG", "-ffast-math", "EP_psych_stats"):
            require(flag not in own[0]["command"], "Incorrect native ABI/mode " + flag)
        require(own[0]["command"].startswith(str(verify(core["compiler"]["binary"]))), "New helper uses a different compiler")
        util.write(directory / "actual-driver-compile-commands.json", own)
        receipt["actual_compile_commands"] = ref(directory / "actual-driver-compile-commands.json")
        compiled = util.process([command[0], "--build", str(BUILD), "--target", TARGET, "--parallel", "1", "--verbose"], directory, "build")
        receipt["compile_link"] = ref(directory / "build-command.json")
        require(compiled["exit_code"] == 0, "Targeted compile/link failed; actual receipt/logs preserved")
        receipt["preservation_observations"]["after_compile_link"] = preservation(baseline, frozen, core, source_before, sources, registry_path)
        live = BUILD / "Products" / (TARGET + ".exe")
        linked = {"historical_path": live.relative_to(ROOT).as_posix(), "sha256": sha(live), "size_bytes": live.stat().st_size}
        receipt["linked_original"] = linked
        write(directory / "linked-original-identity.json", linked)
        for name in ("CMakeCache.txt", "compile_commands.json"):
            shutil.copyfile(BUILD / name, directory / name)
        receipt.update(cache=ref(directory / "CMakeCache.txt"), compile_commands=ref(directory / "compile_commands.json"))
        derived_dir = directory / "derivation"
        derive_command = [sys.executable, "-X", "utf8", "-B", str(HERE / "derive_native_helper.py"),
                          "--original", str(live), "--original-sha256", linked["sha256"], "--strip-tool", str(path(args.strip_tool)),
                          "--tool-audit", str(path(args.tool_audit)), "--help-receipt", str(path(args.help_receipt)), "--output-dir", str(derived_dir)]
        derived = util.process(derive_command, directory, "derive")
        receipt["derivation_command"] = ref(directory / "derive-command.json")
        if (derived_dir / "receipt.json").is_file():
            receipt["validated_derivative"] = ref(derived_dir / "receipt.json")
        require(derived["exit_code"] == 0, "Derivation failed; maps/actual receipt/logs preserved")
        receipt["helper_binary"] = validate_derivative(read(derived_dir / "receipt.json"), linked)
        receipt["preservation_observations"]["after_derivation"] = preservation(baseline, frozen, core, source_before, sources, registry_path)
        require(sha(live) == linked["sha256"] and live.stat().st_size == linked["size_bytes"], "New linked original changed")
        receipt.update(checks_passed=True, existing_target_compile_commands_unchanged=True,
                       original_core_and_API_bytes_unchanged=True, same_compiler_owned_state=True,
                       original_directory_and_target_definitions_inherited=True, assertions_enabled=True, fp_contract="off")
    except Exception as error:
        receipt["failure"] = {"type": type(error).__name__, "message": str(error)}
        (directory / "failure.log").write_text(traceback.format_exc(), encoding="utf-8", newline="\n")
        receipt["failure_log"] = ref(directory / "failure.log")
    finally:
        if sources is not None:
            try:
                receipt["preservation_observations"]["final"] = preservation(baseline, frozen, core, source_before, sources, registry_path)
                for binding in [*receipt["contracts"].values(), receipt["helper_request"]]:
                    verify(binding)
                if "linked_original" in receipt:
                    linked = receipt["linked_original"]
                    live = path(linked["historical_path"])
                    receipt["linked_original_final_observation"] = {"historical_path": linked["historical_path"],
                        "sha256": sha(live), "size_bytes": live.stat().st_size}
                    require(receipt["linked_original_final_observation"] == linked, "New linked original changed during preparation")
                if "helper_binary" in receipt:
                    verify(receipt["helper_binary"])
            except Exception as error:
                receipt["checks_passed"] = False
                receipt["final_preservation_failure"] = {"type": type(error).__name__, "message": str(error)}
        receipt["finished_utc"] = utc()
        write(directory / "native-driver-build.json", receipt)
    return {"driver_build": ref(directory / "native-driver-build.json"), "checks_passed": receipt["checks_passed"],
            "helper_binary": receipt.get("helper_binary")}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    modes = parser.add_mutually_exclusive_group(required=True)
    modes.add_argument("--check", action="store_true")
    modes.add_argument("--build-driver", action="store_true")
    for name in ("output-dir", "contract-review", "registry", "derivation-review", "derivation-validation", "derivation-actual-review",
                 "derivation-runtime-validation", "derivation-runtime-review", "strip-tool", "tool-audit", "help-receipt"):
        parser.add_argument("--" + name)
    args = parser.parse_args()
    if args.check:
        print(json.dumps(check_contracts(), indent=2))
        return 0
    for name in ("output_dir", "contract_review", "registry", "derivation_review", "derivation_validation", "derivation_actual_review",
                 "derivation_runtime_validation", "derivation_runtime_review", "strip_tool", "tool_audit", "help_receipt"):
        require(getattr(args, name), "Build requires --" + name.replace("_", "-"))
    result = build_driver(args)
    print(json.dumps(result))
    return 0 if result["checks_passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
