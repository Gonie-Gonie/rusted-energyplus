"""Supplemental original-first matched CLK03 execution, with actual outcomes and metadata guards only.

The only child is the authenticated retained genuine-core helper derivative.
No builds, Rust, numerical comparer, fixture answers, Git, gates or removal.
"""
from __future__ import annotations
import argparse
from collections import Counter
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
import traceback
import zipfile

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[4]
RAW = ROOT / ".runtime/porting/CLK-03"
TOOLS = ROOT / "tools/porting"
BUILD_PATH = RAW / "matched-native-build-01/native-driver-build.json"
BUILDER = RAW / "production-native-prep-01/build_native.py"
CONTRACT = RAW / "production-matched-inputs-01/native-contract.json"
TARGET = "clk03_production_reference_helper"
HISTORICAL = {
 "old_14_sequence_original": {"path": ".runtime/porting/CLK-03/original-helper-first-03/helper-reference.json", "sha256": "0513bc94d02703d299cb0a63bc66fa11a0b405e0d0873d3e61a0916a4e123cdc"},
 "failed_production_comparison": {"path": ".runtime/porting/CLK-03/production-comparison-02/production-comparison.json", "sha256": "085076f535c285794e893e064b210f7536dcbbf7b02d5de06ae60ede5ab253a2"},
 "failed_production_execution": {"path": ".runtime/porting/CLK-03/production-comparison-command-02/receipt.json", "sha256": "0fb86b4517921fb0bf9c9b236c26044d5c32ed13250c54823123d32d13ab5e5a"},
 "independent_failed_production_review": {"path": ".runtime/porting/CLK-03/independent-production-failure-review-01/review.json", "sha256": "77645ff38c2aab3a7b3c9a4b673b7abdfaa9e63ea70148f749c587fce6a7d4d6"},
}
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
RUNTIME_BIN = ROOT / ".runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin"
RUNTIME_DLLS = ("libgcc_s_seh-1.dll", "libstdc++-6.dll", "libwinpthread-1.dll")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def utc():
    return datetime.now(timezone.utc).isoformat()


def instant(value):
    stamp = datetime.fromisoformat(value)
    require(stamp.utcoffset() is not None and stamp.utcoffset().total_seconds() == 0, "Aware UTC timestamp required")
    return stamp


def path(value):
    item = Path(value)
    item = (ROOT / item).resolve() if not item.is_absolute() else item.resolve()
    require(item.is_relative_to(ROOT) and item != ROOT, "Path outside repository")
    return item


def sha(value):
    with Path(value).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def ref(value):
    item = path(value)
    return {"path": item.relative_to(ROOT).as_posix(), "sha256": sha(item)}


def verify(binding):
    item = path(binding["path"])
    require(item.is_file() and not item.is_symlink() and sha(item) == binding["sha256"], "Bound bytes differ: " + str(item))
    for key in ("bytes", "size_bytes"):
        if key in binding:
            require(type(binding[key]) is int and item.stat().st_size == binding[key], "Bound size differs")
    return item


def identity(left, right):
    require(path(left["path"]) == path(right["path"]) and left["sha256"] == right["sha256"], "Actual path/hash identity differs")


def read(value):
    def pairs(rows):
        result = {}
        for key, item in rows:
            require(key not in result, "Duplicate JSON key")
            result[key] = item
        return result
    return json.loads(Path(value).read_text(encoding="utf-8"), object_pairs_hook=pairs,
                      parse_constant=lambda text: (_ for _ in ()).throw(ValueError("Nonstandard JSON " + text)))


def write(destination, value):
    with Path(destination).open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(value, stream, ensure_ascii=False, indent=2, allow_nan=False)
        stream.write("\n")


def unchanged_sources(rows):
    require(rows and len({row["historical_path"] for row in rows}) == len(rows), "Unique actual source archives required")
    for row in rows:
        require(path(row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Actual built/reviewed source changed")


def utilities(build_path, review_path, launch_path):
    require(build_path == BUILD_PATH, "Explicit actual supplemental build receipt required")
    build, review = read(build_path), read(review_path)
    require(build["schema"] == "clk03-matched-production-native-build.v1" and build["checks_passed"] is True
            and build["source_commit"] == PIN and build["target"] == TARGET, "Actual unique matched whole-core build required")
    for key in ["scientific_source_patches", "scientific_execution_performed", "Cargo_or_Git_executed", "original_removed",
                "full_linked_original_copied", "gates_updated", "existing_unit_packet_or_actual_receipts_rewritten"]:
        require(build[key] is False, "Actual build boundary differs: " + key)
    for key in ["original_core_and_API_bytes_unchanged", "same_compiler_owned_state", "original_directory_and_target_definitions_inherited",
                "assertions_enabled", "existing_target_compile_commands_unchanged"]:
        require(build[key] is True, "Actual build guard failed: " + key)
    require(build["fp_contract"] == "off" and build["previous_compile_row_count"] == 656
            and build["frozen_core_compile_row_count"] == 643, "Original ABI/compile registration differs")
    require(review["schema"] == "clk03-independent-matched-production-native-build-review.v1"
            and review["status"] == "pass-build-and-derivative-before-original-execution"
            and review["scientific_execution_performed"] is False, "Independent actual build/PE review required")
    identity(review["reviewed_build"], ref(build_path))
    for key, owner in [("reviewed_contract", "matched_contract"), ("reviewed_request", "matched_request"),
                       ("reviewed_projection", "input_projection"), ("reviewed_binary", "helper_binary")]:
        identity(review[key], build[owner]); verify(review[key])
    linked = build["linked_original"]
    identity(review["actual_linked_original"], {"path": linked["historical_path"], "sha256": linked["sha256"]})
    verify(review["actual_linked_original"]); verify(review["actual_independent_PE_validation"])
    require(instant(build["finished_utc"]) <= instant(review["review_completed_utc"]), "Actual build must precede actual review")
    for key, reviewed in [("independent_contract_review", "reviewed_contract_review"),
                          ("independent_helper_static_review", "reviewed_helper_static_review")]:
        identity(review[reviewed], build[key]); verify(build[key])
    unchanged_sources(build["executed_source_archives"])
    require(len(build["executed_source_archives"]) == 30 and
            sum(path(row["historical_path"]) == BUILDER for row in build["executed_source_archives"]) == 1,
            "All27 inherited and3 new source archives, including imported builder, required")
    static = read(verify(build["independent_helper_static_review"]))
    require(static["schema"] == "clk03-independent-matched-production-helper-static-review.v1"
            and static["status"] == "pass-source-and-contract-before-native-build"
            and static["scientific_execution_performed"] is False, "Independent source/contract review required")
    unchanged_sources(static["reviewed_sources"])
    require(any(path(row["historical_path"]) == BUILDER for row in static["reviewed_sources"]), "Imported utility must be reviewed")
    launch = read(launch_path)
    require(launch["schema"] == "clk03-independent-matched-production-original-launcher-static-review.v1"
            and launch["status"] == "pass-source-and-input-contract-before-original-execution"
            and launch["scientific_execution_performed"] is False, "Independent separate launcher static review required")
    require(path(launch["reviewed_launcher"]["historical_path"]) == Path(__file__).resolve()
            and verify(launch["reviewed_launcher"]["archive"]).read_bytes() == Path(__file__).read_bytes(), "Reviewed launcher differs")
    require(path(launch["reviewed_metadata_utility"]["historical_path"]) == BUILDER
            and verify(launch["reviewed_metadata_utility"]["archive"]).read_bytes() == BUILDER.read_bytes(), "Reviewed metadata utility differs")
    identity(launch["reviewed_contract_review"], build["independent_contract_review"])
    spec = importlib.util.spec_from_file_location("clk03_matched_original_metadata_only", BUILDER)
    require(spec is not None and spec.loader is not None, "Authenticated metadata module unavailable")
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    bound = module.packet(CONTRACT)
    require(all(build[key] == binding for key, binding in bound.items()), "Actual supplemental build packet differs")
    from types import SimpleNamespace
    checked = module.reviews(SimpleNamespace(contract_review=build["independent_contract_review"]["path"],
        helper_static_review=build["independent_helper_static_review"]["path"]), bound)
    require(all(checked[key] == build[key] for key in checked), "Actual built review identity differs")
    module.prior_identity()
    derived = read(verify(build["validated_derivative"]))
    require(module.old.old.validate_derivative(derived, build["linked_original"]) == build["helper_binary"], "Wrong accepted derivative")
    for key in ["configure", "compile_link", "derivation_command"]:
        command = read(verify(build[key]))
        require(type(command["exit_code"]) is int and command["exit_code"] == 0, "Actual build command failed")
        verify(command["stdout"]); verify(command["stderr"])
    return module, build, {"actual_build_review": ref(review_path), "original_launcher_static_review": ref(launch_path), **checked}


def runtime_identity(module, core):
    publisher = read(verify(core["compiler"]["toolchain_publisher_receipt"]))
    require(publisher["checksums_match"] is True and publisher["installation"] == RUNTIME_BIN.parents[1].relative_to(ROOT).as_posix(),
            "Genuine GNU13.2 runtime publisher differs")
    archive = path(publisher["archive"])
    require(sha(archive) == publisher["publisher_checksums"]["sha256"] == publisher["actual_checksums"]["sha256"], "Publisher ZIP differs")
    verify(core["compiler"]["binary"])
    require(path(core["compiler"]["binary"]["path"]).parent == RUNTIME_BIN, "Wrong GNU PATH owner")
    result = []
    with zipfile.ZipFile(archive) as package:
        for name in RUNTIME_DLLS:
            member = "mingw64/bin/" + name
            expected = hashlib.sha256(package.read(member)).hexdigest(); installed = RUNTIME_BIN / name
            require(sha(installed) == expected, "Runtime DLL differs from publisher ZIP: " + name)
            result.append({"archive_member": member, "member_sha256": expected, "installed_binary": ref(installed)})
    return {"publisher_receipt": core["compiler"]["toolchain_publisher_receipt"], "publisher_archive": ref(archive),
            "runtime_path_prefix": str(RUNTIME_BIN), "runtime_libraries": result}


def con_inputs(build):
    contract = read(verify(build["matched_contract"])); scope = read(verify(build["fixed_scope"]))
    require(scope["schema_version"] == "con01-scope.v2" and scope["energyplus"]["commit"] == PIN
            and len(scope["cases"]) == 15, "Immutable fifteen-CON scope required")
    bindings = [binding for row in scope["cases"] for binding in [row["input"], row["weather"], row["metadata"]]]
    require(len(bindings) == 45 and bindings == contract["literal_CON_input_references"], "Exact all45 CON bindings required")
    for binding in bindings: verify(binding)
    for binding in read(verify(build["contracts"]["cases"]))["input_artifacts"]: verify(binding)
    for binding in read(verify(build["input_projection"]))["input_artifacts"]: verify(binding)
    return build["fixed_scope"], bindings


def guards(module, build, runtime, reviews):
    bound = module.packet(CONTRACT)
    require(all(build[key] == binding for key, binding in bound.items()), "Supplemental input packet changed")
    registry_path, registry, _ = module.old.registry(build["retained_registry"]["path"])
    identity(ref(registry_path), build["retained_registry"])
    _, retained = module.prior_identity()
    core = read(verify(build["core_build"]))
    require(core["schema"] == "native-core-build.v1" and core["checks_passed"] is True
            and core["energyplus_commit"] == PIN and core["scientific_source_patches"] is False, "Original core receipt differs")
    for binding in core["artifacts"].values():
        require(any(path(binding["path"]) == path(row["path"]) and binding["sha256"] == row["sha256"]
                    for row in registry["protected_core_API_and_container"]), "Core/API not protected by exact registry")
    for key in ["binary", "toolchain_publisher_receipt", "object_cache_publisher_receipt"]: verify(core["compiler"][key])
    source_guard = module.util.original_source_guard(core)
    unchanged_sources(build["executed_source_archives"])
    for binding in reviews.values(): verify(binding)
    for binding in build["derivative_utility"].values(): verify(binding)
    require((module.BUILD / "compile_commands.json").read_bytes() == verify(build["compile_commands"]).read_bytes()
            and (module.BUILD / "CMakeCache.txt").read_bytes() == verify(build["cache"]).read_bytes(), "All-target cache metadata changed")
    linked = build["linked_original"]; linked_path = path(linked["historical_path"])
    require(linked_path.parent == module.BUILD / "Products" and linked_path.name == TARGET + ".exe"
            and linked_path.is_file() and not linked_path.is_symlink() and sha(linked_path) == linked["sha256"]
            and linked_path.stat().st_size == linked["size_bytes"], "New unique debug Product identity differs")
    verify(build["helper_binary"]); verify(runtime["publisher_archive"])
    for row in runtime["runtime_libraries"]: verify(row["installed_binary"])
    scope, inputs = con_inputs(build)
    for binding in HISTORICAL.values(): verify(binding)
    return {"observed_utc": utc(), **bound, "retained_registry": ref(registry_path), "protected_legacy_and_library_count": 14,
            "retired_CLK02_original_absence_verified": True, "retained_CLK02_derivative": registry["new_targets"][0]["retained_derivative"],
            "retained_successful_CLK03_original_and_derivative_verified": True,
            "retained_failed_original02": retained["retained_failed_original02_independently_hashed"],
            "retained_failed_derivative02": retained["retained_failed_derivative02_independently_hashed"],
            "failed_original_relocation": retained["reviewed_failed_original_relocation"],
            "native_core_build": build["core_build"], "original_source_guard": source_guard, "built_sources_unchanged": True,
            "new_matched_CLK03_linked_original": linked, "execution_derivative": build["helper_binary"],
            "runtime_publisher_bytes_unchanged": True, "CON_scope": scope, "CON_case_input_bindings_checked": len(inputs),
            "all_target_compile_commands": build["compile_commands"], "cache": build["cache"], "independent_reviews": reviews,
            "preserved_historical_evidence": HISTORICAL}


def archive_inputs(directory, build, reviews, plan_ref, plan):
    destination = directory / "source-and-inputs"; destination.mkdir()
    scope, inputs = con_inputs(build)
    contract = read(verify(build["matched_contract"])); oldcases = read(verify(build["contracts"]["cases"]))
    files = [verify(binding) for binding in [ref(BUILD_PATH), *build["contracts"].values(), build["matched_request"],
             build["matched_contract"], build["input_projection"], scope, contract["input_byte_maps"],
             *oldcases["input_artifacts"], *inputs, *reviews.values(), *HISTORICAL.values(), build["core_build"],
             build["prior_native_build"], build["prior_actual_build_review"], build["validated_derivative"], build["retained_registry"],
             *build["derivative_utility"].values(), build["configure"], build["compile_link"], build["derivation_command"],
             build["compile_commands"], build["cache"], build["actual_compile_commands"], build["previous_compile_commands"],
             build["frozen_original_build_commands"], contract["independent_input_source_review"], plan_ref, plan["independent_sources_review"]]]
    source_files = [path(row["historical_path"]) for row in build["executed_source_archives"] + plan["reader_source_archives"]] + [Path(__file__).resolve()]
    groups = {}
    for key, items in [("input_archives", files), ("executed_source_archives", source_files)]:
        rows = []
        for index, original in enumerate(dict.fromkeys(items)):
            target = destination / f"{key}-{index:03d}-{original.name}"; shutil.copyfile(original, target)
            require(sha(original) == sha(target), "Source/input changed while archiving")
            rows.append({"historical_path": original.relative_to(ROOT).as_posix(), "archive": ref(target)})
        groups[key] = rows
    return groups


def source_outcomes(result, request, build):
    require(result["schema"] == "clk03-production-helper-results.v1" and result["source_commit"] == PIN
            and result["complete"] is True and result["processed_weather_physics_retained_unpaired"] is True,
            "Actual matched helper schema/scope differs")
    for key in ["expected_answers_supplied", "expected_values_supplied", "expected_exits_supplied", "Rust_outputs_supplied_as_inputs",
                "gates_updated", "pure_body_fallback_used", "native_raw_record_index_claimed", "scientific_comparison_executed"]:
        require(result[key] is False, "Matched result boundary differs: " + key)
    identity(result["actual_binary"], build["helper_binary"]); identity(result["actual_request"], build["matched_request"])
    identity(result["actual_contract"], build["matched_contract"]); identity(result["scope"], build["fixed_scope"])
    for key, binding in build["contracts"].items(): identity(result["contracts"][key], binding)
    require(result["requested_counts"] == build["requested_counts"] == request["requested_counts"], "Declared matched coverage differs")
    statuses, calls, observed = Counter(), Counter(), []
    require(type(result["cases"]) is list and len(result["cases"]) == len(request["cases"]) == 3, "Actual matched case cardinality differs")
    for sequence, original in zip(result["cases"], request["cases"], strict=True):
        require(sequence["id"] == original["id"] and len(sequence["operations"]) == len(original["operations"]), "Actual operation coverage differs")
        for operation, supplied in zip(sequence["operations"], original["operations"], strict=True):
            require(operation["id"] == supplied["id"] and operation["kind"] == supplied["kind"]
                    and operation["requested_operation"] == supplied, "Native operation input identity differs")
            outcome, invoked = operation["call_outcome"], operation["actual_source_invoked"]
            require(type(invoked) is bool, "Typed actual invocation availability required")
            status = outcome["status"]
            require(status in ["source_returned", "source_fatal", "not-invoked-after-prior-source-outcome"], "Unknown source status")
            require(invoked is (status != "not-invoked-after-prior-source-outcome"), "Invocation/status availability differs")
            require(outcome["source_fatal"] is (True if status == "source_fatal" else False if invoked else None), "Actual fatal availability differs")
            statuses[status] += 1
            if invoked: calls[operation["kind"]] += 1
            observed.append({"sequence_id": sequence["id"], "operation_id": operation["id"], "kind": operation["kind"],
                             "actual_source_invoked": invoked, "call_outcome": outcome})
    for key, expected in [("actual_operation_counts", dict(calls)), ("actual_source_function_counts", dict(calls)),
                          ("actual_source_status_counts", dict(statuses)), ("actual_operation_invocations", sum(calls.values())),
                          ("actual_source_invocations", sum(calls.values())), ("actual_operations_skipped", statuses["not-invoked-after-prior-source-outcome"]),
                          ("actual_source_operations_skipped", statuses["not-invoked-after-prior-source-outcome"])]:
        require(type(result[key]) is type(expected) and result[key] == expected
                and (type(expected) is not dict or all(type(x) is int for x in result[key].values())),
                "Actual count transcription differs: " + key)
    require(len(observed) == build["requested_counts"]["total_operations"], "Requested root operation cardinality differs")
    return {"requested_counts": result["requested_counts"], "actual_operation_counts": dict(calls), "actual_source_status_counts": dict(statuses),
            "actual_operation_invocations": sum(calls.values()), "actual_operations_skipped": statuses["not-invoked-after-prior-source-outcome"],
            "observed_operations": observed, "request_root_coverage_complete": True, "source_outputs_numerically_compared": False,
            "source_error_text_parity_claimed": False, "independent_data_review_required_before_production_execution": True}


def production_plan(plan_path, build):
    declared = read(plan_path)
    require(declared["schema"] == "clk03-matched-production-plan.v1" and declared["actual_command_count"] == 6,
            "Separately frozen six-run production policy required before supplemental original")
    for key in ["expected_outputs_supplied", "physical_execution_performed", "gates_updated", "native_reference_reads_by_plan_writer",
                "hour1_production_matrix_added", "warmup_options_changed"]:
        require(declared[key] is False, "Input-only plan boundary differs: " + key)
    require(declared["contracts"] == build["contracts"], "Plan R3 input contracts differ")
    for key, binding in [("matched_native_contract", build["matched_contract"]), ("matched_input_projection", build["input_projection"]),
                         ("fixed_scope", build["fixed_scope"])]: identity(declared[key], binding); verify(binding)
    contract = read(verify(build["matched_contract"]))
    identity(declared["input_byte_maps"], contract["input_byte_maps"]); verify(declared["input_byte_maps"])
    review = read(verify(declared["independent_sources_review"]))
    require(review["schema"] == "clk03-independent-matched-production-sources-review.v1"
            and review["status"] == "pass-source-and-matched-observation-selection-before-original-and-production"
            and review["scientific_execution_performed"] is False and review["reviewed_contracts"] == build["contracts"],
            "Independent production observation source review required")
    identity(review["reviewed_matched_contract"], build["matched_contract"])
    unchanged_sources(declared["reader_source_archives"]); unchanged_sources(review["reviewed_sources"])
    projected = {x["historical_path"]: x["archive"]["sha256"] for x in declared["reader_source_archives"]}
    reviewed = {x["historical_path"]: x["archive"]["sha256"] for x in review["reviewed_sources"]}
    require(len(projected) == len(reviewed) == 10 and projected == reviewed, "Exact reviewed ten-source policy projection required")
    expected = read(verify(build["fixed_scope"])); indexed = {row["id"]: row for row in expected["cases"]}
    require([x["case_id"] for x in declared["cases"]] == contract["production_case_ids"], "Plan three-case order differs")
    for row in declared["cases"]:
        require(row["trace_levels"] == ["Full", "Summary"], "Exact six Full/Summary commands required")
        original = indexed[row["case_id"]]
        require(all(row[key] == original[key] for key in ["scope", "duration", "input", "weather", "metadata"]), "Literal CON plan inputs differ")
        for key in ["input", "weather", "metadata"]: verify(row[key])
    require(instant(review["review_completed_utc"]) <= instant(declared["created_utc"]), "Plan freeze must follow independent review")
    return ref(plan_path), declared


def inventory(directory):
    return [ref(item) for item in sorted(directory.rglob("*")) if item.is_file()]


def run(args):
    directory = path(args.output_dir)
    require(directory.is_relative_to(RAW.resolve()) and directory != RAW.resolve() and not directory.exists(), "Fresh contained output required")
    directory.mkdir(parents=True)
    record = {"schema": "clk03-matched-production-original-reference.v1", "status": "preflight-in-progress", "started_utc": utc(),
              "actual_launcher_argv": sys.orig_argv, "actual_launcher_cwd": str(Path.cwd().resolve()), "energyplus_commit": PIN,
              "Rust_executed": False, "Rust_compared": False, "gates_updated": False, "builds_or_Git_executed": False,
              "source_expected_answers_supplied": False, "deletion_performed": False, "preservation_observations": {},
              "processed_weather_physics_retained_unpaired": True, "original_data_review_required_before_Rust": True,
              "old_14_sequence_unit_reference_rewritten": False, "prior_actual_failure_relabelled_PASS": False,
              "scientific_certification_passed": False, "original_data_review_required_before_production": True}
    module = build = runtime = groups = reviews = plan_ref = None; return_code = 1
    try:
        build_path, review_path = path(args.build_receipt), path(args.build_review)
        module, build, reviews = utilities(build_path, review_path, path(args.launcher_review))
        for binding in reviews.values():
            require(instant(read(verify(binding))["review_completed_utc"]) <= instant(record["started_utc"]),
                    "Independent reviews must precede launcher execution")
        request = read(verify(build["matched_request"])); runtime = runtime_identity(module, read(verify(build["core_build"])))
        plan_ref, plan = production_plan(path(args.production_plan), build)
        groups = archive_inputs(directory, build, reviews, plan_ref, plan); record.update(groups)
        own = next(row["archive"] for row in groups["executed_source_archives"] if path(row["historical_path"]) == Path(__file__).resolve())
        record.update(executed_launcher={"historical_path": Path(__file__).resolve().relative_to(ROOT).as_posix(), "archive": own},
                      native_driver_build=ref(build_path), independent_reviews=reviews, actual_request=build["matched_request"], actual_contract=build["matched_contract"],
                      input_projection=build["input_projection"], fixed_scope=build["fixed_scope"], executed_helper=build["helper_binary"],
                      independent_actual_build_review=reviews["actual_build_review"],
                      independent_launcher_source_review=reviews["original_launcher_static_review"],
                      preserved_historical_evidence=HISTORICAL,
                      contracts=build["contracts"], compiler_runtime=runtime)
        record["production_plan"] = plan_ref
        record["production_plan_source_review"] = plan["independent_sources_review"]
        record["preservation_observations"]["before_execution"] = guards(module, build, runtime, reviews)
        native_dir = directory / "native-output"; require(not native_dir.exists(), "Native output must be fresh before child")
        command = [str(verify(build["helper_binary"])), str(ROOT), str(verify(build["matched_request"])), str(verify(build["matched_contract"])), str(native_dir)]
        started = utc()
        require(instant(plan["created_utc"]) <= instant(record["started_utc"]) <= instant(started),
                "Production policy must freeze before launcher and native child")
        for binding in reviews.values():
            require(instant(read(verify(binding))["review_completed_utc"]) <= instant(started), "Every independent review must precede actual child")
        begin = time.perf_counter(); env = module.util.environment()
        require(env["PATH"].split(os.pathsep)[0] == str(RUNTIME_BIN), "Wrong authenticated GNU runtime PATH prefix")
        spawn_error = None; code = None
        with (directory / "helper-stdout.log").open("xb") as stdout, (directory / "helper-stderr.log").open("xb") as stderr:
            try:
                code = subprocess.run(command, cwd=ROOT, env=env, stdout=stdout, stderr=stderr, check=False,
                                      creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0)).returncode
            except Exception as error:
                spawn_error = {"type": type(error).__name__, "message": str(error)}
        execution = {"schema": "original-native-command.v1", "command": command, "cwd": str(ROOT), "exit_code": code,
                     "started_utc": started, "finished_utc": utc(), "elapsed_seconds": round(time.perf_counter() - begin, 6),
                     "runtime_path_prefix": str(RUNTIME_BIN), "stdout": ref(directory / "helper-stdout.log"),
                     "stderr": ref(directory / "helper-stderr.log"), "binary": build["helper_binary"], "request": build["matched_request"], "contract": build["matched_contract"],
                     "production_plan": record["production_plan"],
                     "native_driver_build": ref(build_path), "spawn_error": spawn_error}
        write(directory / "helper-command.json", execution); record["execution"] = ref(directory / "helper-command.json")
        record["actual_helper_exit_code"] = code
        require(spawn_error is None, "Actual helper could not launch; receipts/logs retained")
        result_path = native_dir / "results.json"
        record["results"] = ref(result_path) if result_path.is_file() else None
        record["actual_source_outcomes"] = source_outcomes(read(result_path), request, build) if result_path.is_file() else None
        verify(plan_ref); production_plan(path(args.production_plan), build)
        record["preservation_observations"]["after_execution"] = guards(module, build, runtime, reviews)
        for rows in groups.values():
            unchanged_sources(rows)
        require(record["preservation_observations"]["before_execution"]["original_source_guard"] ==
                record["preservation_observations"]["after_execution"]["original_source_guard"], "Original source inventory changed")
        record["preservation_checks_passed"] = True
        record["status"] = "wrapper-complete-actual-source-outcomes-awaiting-independent-data-review" if code == 0 and record["results"] else "actual-wrapper-failure-preserved"
        return_code = code if code is not None and code != 0 else (0 if record["results"] else 1)
    except Exception as error:
        record["status"] = "failed-preserved-stop-before-Rust-numerical-execution"
        record["failure"] = {"type": type(error).__name__, "message": str(error)}
        (directory / "failure.log").write_text(traceback.format_exc(), encoding="utf-8", newline="\n")
        record["failure_log"] = ref(directory / "failure.log")
    finally:
        if module is not None and build is not None and runtime is not None:
            try:
                if "after_execution" not in record["preservation_observations"]:
                    record["preservation_observations"]["failure_final"] = guards(module, build, runtime, reviews)
                    if plan_ref is not None: verify(plan_ref); production_plan(path(args.production_plan), build)
                else:
                    record["final_guard_policy"] = "actual-after-execution-observation-reused-no-third-full-inventory-hash"
                if groups is not None:
                    for rows in groups.values():
                        unchanged_sources(rows)
            except Exception as error:
                record["final_preservation_failure"] = {"type": type(error).__name__, "message": str(error)}
                record["preservation_checks_passed"] = False
                record["status"] = "failed-preserved-stop-before-Rust-numerical-execution"; return_code = 1
        native_dir = directory / "native-output"
        if native_dir.exists():
            record["native_artifact_inventory"] = inventory(native_dir)
        record["finished_utc"] = utc(); record["actual_launcher_exit_code"] = return_code
        write(directory / "helper-reference.json", record)
    print(json.dumps({"reference": ref(directory / "helper-reference.json"), "status": record["status"],
                      "actual_helper_exit_code": record.get("actual_helper_exit_code"), "actual_launcher_exit_code": return_code}))
    return return_code


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-receipt", required=True); parser.add_argument("--build-review", required=True)
    parser.add_argument("--launcher-review", required=True)
    parser.add_argument("--production-plan", required=True)
    parser.add_argument("--output-dir", required=True)
    return run(parser.parse_args())


if __name__ == "__main__":
    raise SystemExit(main())
