"""Build one CLK-03 genuine-core driver; retain an authenticated derivative.

Explicit retired-v2 registry, frozen input-only contracts and independent source
reviews precede native actions. This tool does not execute scientific requests,
Rust, comparers, Git, registry admissions, removals or gate updates. A separate
independent actual-build review is required before original helper execution.
"""
from __future__ import annotations

import argparse
import copy
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
RAW = ROOT / ".runtime/porting/CLK-03"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
PRIOR = ROOT / ".runtime/porting/CLK-02/native-build-01/native-driver-build.json"
PRIOR_SHA = "b9f727f19b6835a17457322a64446d1e904aabc92add04ef6871a62ec4af617b"
REGISTRY_SHA = "9fbc167c88860561601caa8ac6eec53caad6505fdf1b24e15ae9269def279564"
ADMISSION_SHA = "7229632378458fa5ce8ba97c985554e7f6e898835f89f8267f8bebb6ada2f30c"
REMOVAL_SHA = "e8aa4b71e5b6679905bc52a798422a4779564ea527db4bbc09a04201aa57cc53"
PLAN_SHA = "f1d89988fc954b79fc9b5e7f3d16b8ab664e510b2a7237b065248a1826c7c51b"
TARGET = "clk03_reference_helper"
OWN = [TARGET + ".cpp", "clk03_reference_fields.hh", "clk03_reference.cmake", "clk03_reference_native.py"]
REUSED = ["clk02_reference_fields.hh"]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def path(value):
    item = Path(value)
    result = (ROOT / item).resolve() if not item.is_absolute() else item.resolve()
    require(result.is_relative_to(ROOT) and result != ROOT, "Path outside repository")
    return result


def sha(value):
    with Path(value).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def read(value):
    return json.loads(Path(value).read_text(encoding="utf-8"))


def verify(binding):
    item = path(binding["path"])
    require(item.is_file() and not item.is_symlink() and sha(item) == binding["sha256"], "Bound bytes differ: " + str(item))
    for key in ("bytes", "size_bytes"):
        if key in binding:
            require(type(binding[key]) is int and item.stat().st_size == binding[key], "Bound size differs")
    return item


def ref(value):
    item = path(value)
    return {"path": item.relative_to(ROOT).as_posix(), "sha256": sha(item)}


def write(destination, value):
    with destination.open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(value, stream, indent=2, allow_nan=False)
        stream.write("\n")


def utc():
    return datetime.now(timezone.utc).isoformat()


def timestamp(value):
    result = datetime.fromisoformat(value)
    require(result.utcoffset() is not None and result.utcoffset().total_seconds() == 0, "Aware UTC timestamp required")
    return result


def historical_sources(record):
    rows = record["executed_source_archives"]
    require(len(rows) == 23 and len({row["historical_path"] for row in rows}) == len(rows), "Prior complete source inventory differs")
    for row in rows:
        require(path(row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Prior source/CMake changed")
    return rows


# Authenticate all importable inherited utility bytes before executing imports.
require(sha(PRIOR) == PRIOR_SHA, "Immutable actual CLK-02 build receipt differs")
historical_sources(read(PRIOR))
spec = importlib.util.spec_from_file_location("clk03_pinned_clk02_utilities", HERE / "clk02_reference_native.py")
require(spec is not None and spec.loader is not None, "Pinned utility module unavailable")
old = importlib.util.module_from_spec(spec)
spec.loader.exec_module(old)
util = old.util
SOURCE, BUILD, CORE, PIN = util.SOURCE, util.BUILD, util.CORE, util.PIN


def contracts():
    return {key: ref(CONTRACTS / f"CLK-03-{key}.json") for key in ("source", "cases", "tolerances")}


def check_contracts():
    source, cases, tolerances = [read(CONTRACTS / f"CLK-03-{key}.json") for key in ("source", "cases", "tolerances")]
    for key, item in (("source", source), ("cases", cases), ("tolerances", tolerances)):
        require(item["schema"] == f"clk03-{key}-contract.v1" and item["status"] == "frozen-before-numerical-execution"
                and item["frozen_before_numerical_execution"] is True and item["card"] == "CLK-03"
                and item["energyplus_commit"] == PIN, "Final frozen pinned contract required: " + key)
    for binding in source["source_audits"]:
        verify(binding)
    for row in source["source_files"]:
        item = (SOURCE / row["path"]).resolve()
        require(item.is_relative_to(SOURCE) and sha(item) == row["sha256"], "Pinned original file differs")
    for row in source["selected_ranges"]:
        item = (SOURCE / row["file"]).resolve()
        require(item.is_relative_to(SOURCE), "Original range outside pinned source")
        lines = item.read_bytes().splitlines(keepends=True)
        start, end = row["start_line"], row["end_line"]
        require(type(start) is int and type(end) is int and 1 <= start <= end <= len(lines), "Invalid original range")
        require(hashlib.sha256(b"".join(lines[start - 1:end])).hexdigest() == row["range_sha256"], "Pinned original range differs")
    request_path = verify(cases["helper_request"])
    require(request_path == ROOT / "energyplus_porting_plan/cases/CLK-03/helper-request.json", "Explicit CLK-03 request required")
    request = read(request_path)
    require(request["schema"] == "clk03-helper-cases.v1" and request["expected_values_supplied"] is False
            and request["expected_exits_supplied"] is False, "Input-only genuine-call request required")
    actual = {}
    for lane in ("handoff", "weather"):
        sequences = request[lane + "_sequences"]
        require(isinstance(sequences, list), "Sequence array required")
        actual[lane + "_sequences"] = len(sequences)
        actual[lane + "_operations"] = sum(len(row["operations"]) for row in sequences)
    actual["total_sequences"] = actual["handoff_sequences"] + actual["weather_sequences"]
    actual["total_operations"] = actual["handoff_operations"] + actual["weather_operations"]
    counts = cases["counts"]
    require(counts and {"handoff_sequences", "weather_sequences"}.issubset(counts), "Declared lane counts required")
    for key, number in counts.items():
        require(key in actual and type(number) is int and number >= 0 and number == actual[key], "Declared input count differs: " + key)
    for binding in cases["input_artifacts"]:
        verify(binding)
    require(len({binding["path"] for binding in cases["input_artifacts"]}) == len(cases["input_artifacts"]), "Unique input artifacts required")
    require(cases["input_byte_maps"] in cases["input_artifacts"], "Literal input byte-map binding required")
    verify(cases["input_byte_maps"])
    for sequence in request["weather_sequences"]:
        for key in ("input", "weather"):
            require(sequence[key] in cases["input_artifacts"], "Request input absent from frozen artifact inventory")
            verify(sequence[key])
    require(request["fixed_scope"] == cases["fixed_CON_scope"], "Request CON scope identity differs")
    verify(cases["fixed_CON_scope"])
    dependencies = [row for row in read(PRIOR)["executed_source_archives"] if Path(row["historical_path"]).name in REUSED]
    require(len(dependencies) == len(REUSED), "Historical observer dependency archive required")
    for row in dependencies:
        require(path(row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Historical observer dependency changed")
    return {"contracts": contracts(), "helper_request": cases["helper_request"], "counts": counts,
            "actual_input_counts": actual, "reused_observer_dependency_archives": dependencies}


def independent_reviews(args, packet):
    contract_path, helper_path = path(args.contract_review), path(args.helper_static_review)
    contract, helper = read(contract_path), read(helper_path)
    require(contract["schema"] == "clk03-independent-contract-review.v1"
            and contract["status"] == "pass-before-scientific-execution", "Independent frozen-contract PASS required")
    for key, binding in {**packet["contracts"], "helper_request": packet["helper_request"]}.items():
        require(verify(contract["reviewed_contracts"][key]["archive"]).read_bytes() == verify(binding).read_bytes(), "Reviewed packet bytes differ")
    require(helper["schema"] == "clk03-independent-helper-static-review.v1"
            and helper["status"] == "pass-source-contract-bound-before-build"
            and helper["scientific_execution_performed"] is False, "Independent helper/registration static review required")
    wanted = {"tools/porting/" + name for name in OWN + REUSED}
    require({row["historical_path"] for row in helper["reviewed_sources"]} == wanted
            and len(helper["reviewed_sources"]) == len(wanted), "Four new sources and reused observer dependency must be independently reviewed")
    for row in helper["reviewed_sources"]:
        require(path(row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Reviewed helper/build source differs")
    require(helper["reviewed_contract_review"] == ref(contract_path), "Helper review frozen-contract identity differs")
    for record in (contract, helper):
        timestamp(record["review_completed_utc"])
    return {"independent_contract_review": ref(contract_path), "independent_helper_static_review": ref(helper_path)}


def registry(value):
    registry_path = path(value)
    require(registry_path == ROOT / ".runtime/porting/CLK-02/retirement-command-01/registry-retired.json"
            and sha(registry_path) == REGISTRY_SHA, "Explicit actual retired-v2 registry required")
    record = read(registry_path)
    require(record["schema"] == "native-retained-artifact-registry.v2"
            and record["admitted_by_Root_after_actual_card_closure"] is True and record["admitted_by_this_plan"] is False,
            "Actual post-closure Root admission required")
    admitted_path = verify(record["predecessor"])
    require(sha(admitted_path) == ADMISSION_SHA, "Actual pre-removal admission differs")
    admitted = read(admitted_path)
    require(admitted["schema"] == record["schema"] and admitted["actual_card_closure"] == record["actual_card_closure"]
            and admitted["admission_plan"] == record["admission_plan"] and admitted["admission_utc"] == record["admission_utc"],
            "Post-closure admission chain differs")
    legacy_path = verify(admitted["predecessor"])
    _, legacy = old.registry(legacy_path)
    for key in ("legacy_products", "legacy_product_count", "protected_core_API_and_container"):
        require(record[key] == admitted[key] == legacy[key], "Fourteen protected baseline identities differ")
    require(sha(verify(record["admission_plan"])) == PLAN_SHA, "Actual eligibility plan differs")
    plan, closure = read(verify(record["admission_plan"])), read(verify(record["actual_card_closure"]))
    require(closure["schema"] == "clk02-metadata-card-closure.v1" and closure["only_CLK02_gates_updated"] is True,
            "Recorded actual card closure required")
    require(len(record["new_targets"]) == len(admitted["new_targets"]) == 1, "Only the admitted CLK-02 derivative allowed")
    entry = record["new_targets"][0]
    require(entry["schema"] == "native-new-target-retention-entry.v1" and entry["card"] == "CLK-02"
            and entry["target"] == "clk02_reference_helper" and entry["retention_class"] == "new-validated-derivative"
            and entry["status"] == "retained-derivative-original-retired" and entry["actual_removal_observed"] is True,
            "Retired CLK-02 entry required")
    require(entry["original_link"] == read(PRIOR)["linked_original"] and entry["evidence"] == plan["evidence"], "Original build/evidence chain differs")
    for binding in entry["evidence"].values():
        verify(binding)
    derivative = read(verify(entry["evidence"]["validated_derivative"]))
    old.validate_derivative(derivative, entry["original_link"])
    require(entry["retained_derivative"] == derivative["accepted_derivative"], "Retained CLK-02 derivative identity differs")
    retained = verify(entry["retained_derivative"])
    require(retained.stat().st_size == entry["retained_derivative"]["size_bytes"], "Retained derivative size differs")
    removal_path = verify(entry["removal_receipt"])
    require(sha(removal_path) == REMOVAL_SHA, "Actual precise removal receipt differs")
    removal = read(removal_path)
    require(removal["schema"] == "native-new-target-removal-receipt.v1" and removal["target"] == entry["target"]
            and type(removal["actual_command_exit_code"]) is int and removal["actual_command_exit_code"] == 0
            and removal["actual_original_absence"] is True and removal["new_target_admission"] == record["predecessor"]
            and removal["actual_card_closure"] == record["actual_card_closure"]
            and removal["historical_original"] == entry["original_link"], "Actual removal/closure/admission identity differs")
    expected = [{"binary": row["binary"], "size_bytes": row["size_bytes"]} for row in legacy["legacy_products"]]
    expected += [{"binary": {key: row[key] for key in ("path", "sha256")}, "size_bytes": row["bytes"]}
                 for row in legacy["protected_core_API_and_container"]]
    require(removal["protected_before"] == removal["protected_after"] == expected and len(expected) == 14
            and removal["prior_metadata_bytes_unchanged"] is True and removal["no_full_debug_copy_created"] is True
            and removal["engines_or_scientific_comparers_executed"] is False
            and removal["realized_removed_bytes"] == entry["original_link"]["size_bytes"], "Actual removal preservation differs")
    for key in ("executed_writer", "executed_script", "stdout", "stderr"):
        verify(removal[key])
    require(removal["actual_native_command"] == ["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", str(verify(removal["executed_script"]))],
            "Actual single-file native action differs")
    require(timestamp(record["admission_utc"]) <= timestamp(removal["action_started_utc"])
            <= timestamp(removal["action_completed_utc"]) <= timestamp(removal["completed_utc"]), "Actual retirement chronology differs")
    original = ROOT / entry["original_link"]["historical_path"]
    require(original.resolve().parent == (BUILD / "Products").resolve() and original.name == entry["target"] + ".exe"
            and not original.exists() and not original.is_symlink(), "Retired original must remain absent")
    require(entry["unstripped_original"]["full_debug_bytes_available"] is False
            and entry["unstripped_original"]["full_debug_archive_created"] is False, "Removed debug bytes are not an archive")
    return registry_path, record, legacy_path


def prior_identity():
    require(sha(PRIOR) == PRIOR_SHA, "Historical actual CLK-02 receipt differs")
    record = read(PRIOR)
    require(record["schema"] == "clk02-native-driver-build.v1" and record["checks_passed"] is True
            and record["energyplus_commit"] == PIN and record["scientific_source_patches"] is False
            and verify(record["core_build"]) == CORE, "Genuine prior build/core required")
    historical_sources(record)
    rows = read(verify(record["compile_commands"]))
    require(len(rows) == 655, "All 655 historical compile rows required")
    # The frozen prior ZON provenance still contains the sole historical fragment.
    zon, _ = old.prior_identity()
    require(record["bulk_fragment"] == zon["bulk_fragment"], "Historical original bulk fragment differs")
    return record, rows


def archive_sources(directory, previous, utility, reviews, registry_path):
    destination = directory / "source"
    destination.mkdir()
    files = {path(row["historical_path"]) for row in historical_sources(previous)} | {HERE / name for name in OWN}
    rows = []
    for item in sorted(files):
        archived = destination / item.name
        require(not archived.exists(), "Source archive basename collision")
        shutil.copyfile(item, archived)
        require(sha(item) == sha(archived), "Source changed while archiving")
        rows.append({"historical_path": item.relative_to(ROOT).as_posix(), "archive": ref(archived)})
    for name, binding in {**utility, **reviews, "retained_registry": ref(registry_path)}.items():
        shutil.copyfile(verify(binding), destination / (name + ".json"))
    return rows


def preservation(previous, frozen, core, expected_source, sources, registry_path):
    _, record, _ = registry(registry_path)
    for key in ("binary", "toolchain_publisher_receipt", "object_cache_publisher_receipt"):
        verify(core["compiler"][key])
    current = read(BUILD / "compile_commands.json")
    require([row for row in current if Path(row["file"]).name != TARGET + ".cpp"] == previous, "Historical 655 compile rows changed")
    require(len(frozen) == 643 and all(row in current for row in frozen), "Frozen 643 core compile rows changed")
    guard = util.original_source_guard(core)
    require(guard == expected_source, "Original source-preservation snapshot differs")
    for row in sources:
        require(path(row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Archived build source changed")
    return {"observed_utc": utc(), "protected_legacy_and_library_count": 14, "retained_new_derivative_count": len(record["new_targets"]),
            "retired_CLK02_original_absence_verified": True, "historical_compile_rows_checked": len(previous),
            "frozen_core_rows_checked": len(frozen), "source_guard": guard, "archived_build_sources_unchanged": True}


def build_driver(args):
    directory = path(args.output_dir)
    require(directory.is_relative_to(RAW.resolve()) and directory != RAW.resolve() and not directory.exists(), "Fresh contained CLK-03 output required")
    directory.mkdir(parents=True)
    receipt = {"schema": "clk03-native-driver-build.v1", "started_utc": utc(), "actual_argv": sys.orig_argv,
               "actual_cwd": str(Path.cwd().resolve()), "energyplus_commit": PIN, "checks_passed": False, "preservation_observations": {},
               "scientific_execution_performed": False, "scientific_source_patches": False, "Cargo_or_Git_executed": False,
               "gates_updated": False, "original_removed": False, "full_linked_original_copied": False, "registry_admission_performed": False,
               "independent_build_review_required_before_original_execution": True,
               "derivative_utility_limits": ["The retained utility runtime foundation executes only historical ZON-01 inputs.",
                    "No CLK-03 scientific request or forced throw/error-unwind path is certified here."]}
    baseline = frozen = core = source_before = sources = registry_path = None
    try:
        receipt.update(check_contracts())
        reviews = independent_reviews(args, receipt)
        registry_path, retired, legacy_path = registry(args.registry)
        previous, baseline = prior_identity()
        foundation_args = copy.copy(args)
        foundation_args.registry = str(legacy_path)
        utility = old.utility_identity(foundation_args)
        for binding in [*reviews.values(), *(utility[key] for key in ("independent_review", "independent_actual_review", "independent_runtime_review"))]:
            require(timestamp(read(verify(binding))["review_completed_utc"]) <= timestamp(receipt["started_utc"]), "Independent review must precede this build")
        core, frozen = util.verified_core(), read(util.FROZEN_CORE_ROWS)
        source_before = util.original_source_guard(core)
        require(not (BUILD / "Products" / (TARGET + ".exe")).exists(), "Fresh new helper target required")
        sources = archive_sources(directory, previous, utility, reviews, registry_path)
        receipt.update(core_build=ref(CORE), prior_native_build=ref(PRIOR), **reviews, retained_artifact_registry=ref(registry_path),
                       retired_original_removal=retired["new_targets"][0]["removal_receipt"], derivative_utility=utility,
                       utility_historical_registry=ref(legacy_path), executed_source_archives=sources, bulk_fragment=previous["bulk_fragment"],
                       frozen_original_build_commands=ref(util.FROZEN_CORE_ROWS), frozen_original_build_command_count=643,
                       previous_card_compile_row_count=len(baseline), prior_archived_source_count=len(previous["executed_source_archives"]))
        observe = lambda: preservation(baseline, frozen, core, source_before, sources, registry_path)
        receipt["preservation_observations"]["before_configure"] = observe()
        write(directory / "previous-target-compile-commands.json", baseline)
        receipt["previous_target_compile_commands"] = ref(directory / "previous-target-compile-commands.json")
        configured_before = read(verify(previous["configure"]))
        require(type(configured_before["exit_code"]) is int and configured_before["exit_code"] == 0, "Prior actual configure failed")
        require(sum(token.startswith("-DCMAKE_PROJECT_INCLUDE=") for token in configured_before["command"]) == 1, "Prior project registration differs")
        command = ["-DCMAKE_PROJECT_INCLUDE=" + str(HERE / "clk03_reference.cmake")
                   if token.startswith("-DCMAKE_PROJECT_INCLUDE=") else token for token in configured_before["command"]]
        command.append("-DCLK03_REPOSITORY_ROOT=" + ROOT.as_posix())
        configured = util.process(command, directory, "configure")
        receipt["configure"] = ref(directory / "configure-command.json")
        require(configured["exit_code"] == 0, "Configure failed; actual receipt/logs preserved")
        for binding in reviews.values():
            require(timestamp(read(verify(binding))["review_completed_utc"]) <= timestamp(configured["started_utc"]), "Review did not precede actual configure")
        receipt["preservation_observations"]["after_configure"] = observe()
        rows = read(BUILD / "compile_commands.json")
        own = [row for row in rows if Path(row["file"]).name == TARGET + ".cpp"]
        require(len(rows) == len(baseline) + 1 and len(own) == 1 and path(own[0]["file"]) == HERE / (TARGET + ".cpp"), "Only one new compile row allowed")
        for flag in ("-DEP_psych_errors", "-UNDEBUG", "-Werror", "-O0", "-ffp-contract=off", "-Wa,-mbig-obj", "-std=c++20"):
            require(flag in own[0]["command"].split(), "Missing inherited original flag " + flag)
        for flag in ("_GLIBCXX_DEBUG", "-ffast-math", "EP_psych_stats"):
            require(flag not in own[0]["command"], "Incorrect native ABI/mode " + flag)
        require(own[0]["command"].startswith(str(verify(core["compiler"]["binary"]))), "Different compiler used")
        write(directory / "actual-driver-compile-commands.json", own)
        receipt["actual_compile_commands"] = ref(directory / "actual-driver-compile-commands.json")
        compiled = util.process([command[0], "--build", str(BUILD), "--target", TARGET, "--parallel", "1", "--verbose"], directory, "build")
        receipt["compile_link"] = ref(directory / "build-command.json")
        require(compiled["exit_code"] == 0, "Targeted compile/link failed; actual receipt/logs preserved")
        receipt["preservation_observations"]["after_compile_link"] = observe()
        live = BUILD / "Products" / (TARGET + ".exe")
        linked = {"historical_path": live.relative_to(ROOT).as_posix(), "sha256": sha(live), "size_bytes": live.stat().st_size}
        receipt["linked_original"] = linked
        write(directory / "linked-original-identity.json", linked)
        for name in ("CMakeCache.txt", "compile_commands.json"):
            shutil.copyfile(BUILD / name, directory / name)
        receipt.update(cache=ref(directory / "CMakeCache.txt"), compile_commands=ref(directory / "compile_commands.json"))
        derived_dir = directory / "derivation"
        derive_command = [sys.executable, "-X", "utf8", "-B", str(HERE / "derive_native_helper.py"), "--original", str(live),
                          "--original-sha256", linked["sha256"], "--strip-tool", str(path(args.strip_tool)), "--tool-audit", str(path(args.tool_audit)),
                          "--help-receipt", str(path(args.help_receipt)), "--output-dir", str(derived_dir)]
        derived = util.process(derive_command, directory, "derive")
        receipt["derivation_command"] = ref(directory / "derive-command.json")
        if (derived_dir / "receipt.json").is_file():
            receipt["validated_derivative"] = ref(derived_dir / "receipt.json")
        require(derived["exit_code"] == 0, "Derivation failed; actual receipt/maps/logs preserved")
        receipt["helper_binary"] = old.validate_derivative(read(derived_dir / "receipt.json"), linked)
        receipt["preservation_observations"]["after_derivation"] = observe()
        require(sha(live) == linked["sha256"] and live.stat().st_size == linked["size_bytes"], "New linked original changed")
        receipt.update(checks_passed=True, existing_target_compile_commands_unchanged=True, original_core_and_API_bytes_unchanged=True,
                       same_compiler_owned_state=True, original_directory_and_target_definitions_inherited=True, assertions_enabled=True, fp_contract="off")
    except Exception as error:
        receipt["failure"] = {"type": type(error).__name__, "message": str(error)}
        (directory / "failure.log").write_text(traceback.format_exc(), encoding="utf-8", newline="\n")
        receipt["failure_log"] = ref(directory / "failure.log")
    finally:
        if sources is not None:
            try:
                receipt["preservation_observations"]["final"] = preservation(baseline, frozen, core, source_before, sources, registry_path)
                for binding in [*receipt["contracts"].values(), receipt["helper_request"], *reviews.values(), *utility.values()]:
                    verify(binding)
                if "linked_original" in receipt:
                    linked, live = receipt["linked_original"], path(receipt["linked_original"]["historical_path"])
                    receipt["linked_original_final_observation"] = {"historical_path": linked["historical_path"], "sha256": sha(live), "size_bytes": live.stat().st_size}
                    require(receipt["linked_original_final_observation"] == linked, "New original changed during preparation")
                if "helper_binary" in receipt:
                    verify(receipt["helper_binary"])
            except Exception as error:
                receipt["checks_passed"] = False
                receipt["final_preservation_failure"] = {"type": type(error).__name__, "message": str(error)}
        receipt["finished_utc"] = utc()
        write(directory / "native-driver-build.json", receipt)
    return {"driver_build": ref(directory / "native-driver-build.json"), "checks_passed": receipt["checks_passed"], "helper_binary": receipt.get("helper_binary")}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    modes = parser.add_mutually_exclusive_group(required=True)
    modes.add_argument("--check", action="store_true")
    modes.add_argument("--build-driver", action="store_true")
    names = ("output-dir", "contract-review", "helper-static-review", "registry", "derivation-review", "derivation-validation",
             "derivation-actual-review", "derivation-runtime-validation", "derivation-runtime-review", "strip-tool", "tool-audit", "help-receipt")
    for name in names:
        parser.add_argument("--" + name)
    args = parser.parse_args()
    for name in names:
        if args.build_driver or name in ("contract-review", "helper-static-review", "registry"):
            require(getattr(args, name.replace("-", "_")), "Requires --" + name)
    if args.check:
        packet = check_contracts()
        reviews = independent_reviews(args, packet)
        registry_path, _, _ = registry(args.registry)
        print(json.dumps({**packet, **reviews, "retained_artifact_registry": ref(registry_path), "native_actions_performed": False}, indent=2))
        return 0
    result = build_driver(args)
    print(json.dumps(result))
    return 0 if result["checks_passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
