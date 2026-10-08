"""Original-first CLK03 execution, with actual outcomes and metadata guards only.

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
BUILD_PATH = RAW / "native-build-03/native-driver-build.json"
ADMISSION_UTILITY = RAW / "runtime-input-admission-prep-01/admission.py"
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
    return json.loads(Path(value).read_text(encoding="utf-8"))


def write(destination, value):
    with Path(destination).open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(value, stream, ensure_ascii=False, indent=2, allow_nan=False)
        stream.write("\n")


def unchanged_sources(rows):
    require(rows and len({row["historical_path"] for row in rows}) == len(rows), "Unique actual source archives required")
    for row in rows:
        require(path(row["historical_path"]).read_bytes() == verify(row["archive"]).read_bytes(), "Actual built/reviewed source changed")


def utilities(build_path, review_path, launch_path, admission_path, admission_review_path):
    require(build_path == BUILD_PATH, "Explicit CLK03 actual build receipt required")
    build, review = read(build_path), read(review_path)
    require(build["schema"] == "clk03-native-driver-build.v1" and build["checks_passed"] is True
            and build["energyplus_commit"] == PIN and build["scientific_source_patches"] is False
            and build["scientific_execution_performed"] is False and build["Cargo_or_Git_executed"] is False,
            "Actual successful genuine CLK03-only build required")
    require(review["schema"] == "clk03-independent-native-build-review.v1"
            and review["status"] == "pass-build-and-derivative-before-original-execution", "Independent actual build/PE review required")
    identity(review["reviewed_build"], ref(build_path))
    require(instant(build["finished_utc"]) <= instant(review["review_completed_utc"]), "Actual build must precede independent review")
    for key in ("original_core_and_API_bytes_unchanged", "same_compiler_owned_state", "original_directory_and_target_definitions_inherited",
                "assertions_enabled", "existing_target_compile_commands_unchanged"):
        require(build[key] is True, "Actual native build guard failed: " + key)
    require(build["fp_contract"] == "off" and build["original_removed"] is False and build["full_linked_original_copied"] is False
            and build["registry_admission_performed"] is False and build["gates_updated"] is False, "Build retention/scope differs")
    for key, reviewed in (("independent_contract_review", "reviewed_contract_review"),
                          ("independent_helper_static_review", "reviewed_helper_static_review")):
        identity(review[reviewed], build[key]); verify(build[key])
    # Authenticate every importable new/inherited source before executing imports.
    unchanged_sources(build["executed_source_archives"])
    wanted = TOOLS / "clk03_reference_native.py"
    require(sum(path(row["historical_path"]) == wanted for row in build["executed_source_archives"]) == 1, "Built builder archive required")
    static = read(verify(build["independent_helper_static_review"]))
    require(static["schema"] == "clk03-independent-helper-static-review.v1"
            and static["status"] == "pass-source-contract-bound-before-build"
            and static["scientific_execution_performed"] is False, "Independent exact helper/builder static review required")
    unchanged_sources(static["reviewed_sources"])
    require(any(path(row["historical_path"]) == wanted for row in static["reviewed_sources"]), "Reviewed imported builder required")
    launch = read(launch_path)
    require(launch["schema"] == "clk03-independent-original-launcher-static-review.v1"
            and launch["status"] == "pass-source-and-input-contract-before-original-execution"
            and launch["scientific_execution_performed"] is False, "Independent original-launcher static review required")
    require(path(launch["reviewed_launcher"]["historical_path"]) == Path(__file__).resolve()
            and verify(launch["reviewed_launcher"]["archive"]).read_bytes() == Path(__file__).read_bytes(), "Reviewed launcher bytes differ")
    # Current runtime input review is separate from the actual historical compile review.
    admission = read(admission_path); admission_review = read(admission_review_path)
    require(admission_review["schema"] == "clk03-independent-runtime-input-admission-review.v1"
            and admission_review["status"] == "pass-input-only-runtime-admission-before-original-execution"
            and admission_review["scientific_execution_performed"] is False, "Independent actual post-build input admission review required")
    identity(admission_review["reviewed_admission"], ref(admission_path))
    identity(admission_review["reviewed_source_review"], admission["reviewed_source_review"])
    identity(admission_review["reviewed_contract_review"], admission["reviewed_contract_review"])
    identity(launch["reviewed_contract_review"], admission["reviewed_contract_review"])
    identity(launch["reviewed_admission_source_review"], admission["reviewed_source_review"])
    source_review = read(verify(admission["reviewed_source_review"]))
    require(source_review["schema"] == "clk03-independent-runtime-admission-source-review.v1"
            and source_review["status"] == "pass-source-and-input-only-delta-before-runtime-admission"
            and source_review["scientific_execution_performed"] is False, "Distinct runtime admission utility/launcher source review required")
    identity(source_review["reviewed_contract_review"], admission["reviewed_contract_review"])
    unchanged_sources(source_review["reviewed_sources"])
    require(any(path(row["historical_path"]) == ADMISSION_UTILITY for row in source_review["reviewed_sources"])
            and any(path(row["historical_path"]) == Path(__file__).resolve() for row in source_review["reviewed_sources"]),
            "Exact utility and launcher must be source-reviewed before metadata import")
    identity(admission["executed_metadata_utility"], ref(ADMISSION_UTILITY))
    require(instant(build["finished_utc"]) <= instant(source_review["review_completed_utc"]) <= instant(admission["admitted_utc"])
            <= instant(admission_review["review_completed_utc"]), "Actual post-build input admission chronology differs")
    actual_admission_command = read(verify(admission_review["reviewed_admission_execution"]))
    require(actual_admission_command["schema"] == "recorded-porting-command.v1"
            and type(actual_admission_command["exit_code"]) is int and actual_admission_command["exit_code"] == 0
            and actual_admission_command["launch_error"] is None, "Actual metadata admission command required")
    admission_stdout = read(verify(actual_admission_command["stdout"])); verify(actual_admission_command["stderr"])
    identity(admission_stdout["admission"], ref(admission_path))
    admission_argv = actual_admission_command["command"]
    require(admission_argv[-7:] == [ADMISSION_UTILITY.relative_to(ROOT).as_posix(), "--contract-review",
            admission["reviewed_contract_review"]["path"], "--source-review", admission["reviewed_source_review"]["path"],
            "--output-dir", admission_path.parent.relative_to(ROOT).as_posix()]
            and Path(actual_admission_command["cwd"]).resolve() == ROOT, "Actual admission invocation input differs")
    require(admission_stdout["scientific_execution_performed"] is False
            and instant(actual_admission_command["finished_utc"]) <= instant(admission_review["review_completed_utc"]),
            "Admission actual stdout/chronology differs")
    admission_spec = importlib.util.spec_from_file_location("clk03_authenticated_input_admission", ADMISSION_UTILITY)
    require(admission_spec is not None and admission_spec.loader is not None, "Authenticated admission metadata module unavailable")
    admission_module = importlib.util.module_from_spec(admission_spec); admission_spec.loader.exec_module(admission_module)
    admission = admission_module.validate_admission(ref(admission_path)); packet = admission["runtime_packet"]
    spec = importlib.util.spec_from_file_location("clk03_original_metadata_only", wanted)
    require(spec is not None and spec.loader is not None, "Authenticated metadata module unavailable")
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    checked_packet = module.check_contracts()
    require(all(admission_module.exact(checked_packet[key], packet[key]) for key in ("contracts", "helper_request", "counts")),
            "Admitted current runtime packet/request differs")
    # Authenticate old packet solely from actual original02 archives, never its overwritten canonical aliases.
    historical = {row["historical_path"]: row["archive"] for row in admission["historical_packet_archives"]}
    old_contract = read(verify(build["independent_contract_review"]))
    require(old_contract["schema"] == "clk03-independent-contract-review.v1" and old_contract["status"] == "pass-before-scientific-execution",
            "Historical actual compile contract review differs")
    for key, binding in {**build["contracts"], "helper_request": build["helper_request"]}.items():
        row = old_contract["reviewed_contracts"][key]
        require(path(row["historical_path"]) == path(binding["path"]) and row["archive"]["sha256"] == binding["sha256"]
                and verify(row["archive"]).read_bytes() == verify(historical[binding["path"]]).read_bytes(), "Actual historical compile packet archive differs")
    identity(static["reviewed_contract_review"], build["independent_contract_review"])
    derived = read(verify(build["validated_derivative"]))
    require(module.old.validate_derivative(derived, build["linked_original"]) == build["helper_binary"], "Wrong accepted derivative")
    for key in ("configure", "compile_link", "derivation_command"):
        command = read(verify(build[key]))
        require(type(command["exit_code"]) is int and command["exit_code"] == 0, "Actual build command failed")
        verify(command["stdout"]); verify(command["stderr"])
    reviews = {"actual_build_review": ref(review_path), "original_launcher_static_review": ref(launch_path),
               "historical_contract_review": build["independent_contract_review"], "historical_helper_static_review": build["independent_helper_static_review"],
               "runtime_contract_review": admission["reviewed_contract_review"], "runtime_admission_source_review": admission["reviewed_source_review"],
               "runtime_input_admission_review": ref(admission_review_path)}
    return module, build, reviews, packet, admission_module, ref(admission_path), admission_review["reviewed_admission_execution"]


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


def con_inputs(packet):
    cases = read(verify(packet["contracts"]["cases"])); scope = read(verify(cases["fixed_CON_scope"]))
    require(scope["schema_version"] == "con01-scope.v2" and scope["energyplus"]["commit"] == PIN
            and len(scope["cases"]) == 15, "Immutable CON fifteen-case scope required")
    bindings = [binding for row in scope["cases"] for binding in (row["input"], row["weather"], row["metadata"])]
    require(len(bindings) == 45, "All forty-five CON case input/weather/metadata bindings required")
    for binding in bindings:
        verify(binding)
    return cases["fixed_CON_scope"], bindings


def failed_build_preservation(review_binding):
    review = read(verify(review_binding))
    relocation_binding = review["reviewed_failed_original_relocation"]
    relocation = read(verify(relocation_binding))
    original = review["retained_failed_original02_independently_hashed"]
    derivative = review["retained_failed_derivative02_independently_hashed"]
    verify(original); verify(derivative)
    require(relocation["schema"] == "clk03-failed-original-relocation.v1" and relocation["relocation_completed"] is True
            and type(relocation["actual_script_exit_code"]) is int and relocation["actual_script_exit_code"] == 0
            and relocation["deletion_performed"] is False and relocation["full_debug_copy_created"] is False,
            "Actual failed original relocation proof differs")
    require(path(relocation["destination_path"]) == path(original["path"])
            and relocation["original_link"]["sha256"] == original["sha256"]
            and relocation["original_link"]["size_bytes"] == original["size_bytes"], "Retained failed debug original differs")
    for phase in ("before", "after"):
        identity(relocation[phase]["retained_failed_CLK03_derivative"], derivative)
    return {"actual_relocation_receipt": relocation_binding, "retained_failed_original02": original,
            "retained_failed_derivative02": derivative}


def guards(module, build, runtime, reviews, packet, admission_module, admission_binding):
    admission = admission_module.validate_admission(admission_binding)
    checked = module.check_contracts()
    require(admission_module.exact(admission["runtime_packet"], packet)
            and all(admission_module.exact(checked[key], packet[key]) for key in ("contracts", "helper_request", "counts")), "Admitted frozen runtime packet changed")
    registry_path, registry, _ = module.registry(build["retained_artifact_registry"]["path"])
    identity(ref(registry_path), build["retained_artifact_registry"])
    core = read(verify(build["core_build"]))
    require(core["schema"] == "native-core-build.v1" and core["checks_passed"] is True
            and core["energyplus_commit"] == PIN and core["scientific_source_patches"] is False, "Original core receipt differs")
    for binding in core["artifacts"].values():
        require(any(path(binding["path"]) == path(row["path"]) and binding["sha256"] == row["sha256"]
                    for row in registry["protected_core_API_and_container"]), "Core/API not protected by exact registry")
    for key in ("binary", "toolchain_publisher_receipt", "object_cache_publisher_receipt"):
        verify(core["compiler"][key])
    source_guard = module.util.original_source_guard(core)
    unchanged_sources(build["executed_source_archives"])
    for binding in reviews.values():
        verify(binding)
    for binding in build["derivative_utility"].values():
        verify(binding)
    require((module.BUILD / "compile_commands.json").read_bytes() == verify(build["compile_commands"]).read_bytes()
            and (module.BUILD / "CMakeCache.txt").read_bytes() == verify(build["cache"]).read_bytes(), "Actual cache/all-target compile metadata changed")
    linked = build["linked_original"]; linked_path = path(linked["historical_path"])
    require(linked_path.parent == module.BUILD / "Products" and linked_path.name == "clk03_reference_helper.exe"
            and linked_path.is_file() and not linked_path.is_symlink() and sha(linked_path) == linked["sha256"]
            and linked_path.stat().st_size == linked["size_bytes"], "New debug Product identity differs")
    verify(build["helper_binary"]); verify(runtime["publisher_archive"])
    for row in runtime["runtime_libraries"]:
        verify(row["installed_binary"])
    scope, bindings = con_inputs(packet)
    failed_preservation = failed_build_preservation(reviews["actual_build_review"])
    return {"observed_utc": utc(), "contracts": packet["contracts"], "helper_request": packet["helper_request"],
            "retained_artifact_registry": ref(registry_path), "protected_legacy_and_library_count": 14,
            "retired_CLK02_original_absence_verified": True, "retained_CLK02_derivative": registry["new_targets"][0]["retained_derivative"],
            "native_core_build": build["core_build"], "original_source_guard": source_guard, "built_sources_unchanged": True,
            "new_CLK03_linked_original": linked, "execution_derivative": build["helper_binary"],
            "runtime_publisher_bytes_unchanged": True, "CON_scope": scope, "CON_case_input_bindings_checked": len(bindings),
            "all_target_compile_commands": build["compile_commands"], "cache": build["cache"], "independent_reviews": reviews,
            "runtime_input_admission": admission_binding, "historical_compiled_contracts": build["contracts"],
            "historical_compiled_request": build["helper_request"], "historical_packet_checked_by_actual_archives": True,
            "failed_build02_preservation": failed_preservation}


def archive_inputs(directory, build, reviews, packet, admission_module, admission_binding, admission_execution):
    destination = directory / "source-and-inputs"; destination.mkdir()
    admission = admission_module.validate_admission(admission_binding)
    cases = read(verify(packet["contracts"]["cases"])); scope, con_bindings = con_inputs(packet)
    files = [verify(binding) for binding in [ref(BUILD_PATH), *packet["contracts"].values(), packet["helper_request"], scope,
             *cases["input_artifacts"], *con_bindings, *reviews.values(), build["core_build"], build["prior_native_build"],
             build["validated_derivative"], build["retained_artifact_registry"], *build["derivative_utility"].values(),
             build["configure"], build["compile_link"], build["derivation_command"], build["compile_commands"], build["cache"],
             build["actual_compile_commands"], build["previous_target_compile_commands"], build["frozen_original_build_commands"],
             admission_binding, admission_execution, admission["prior_original_execution"], admission["input_amendment"],
             admission["packet_freeze"], admission["packet_freeze_execution"],
             read(verify(reviews["actual_build_review"]))["reviewed_failed_original_relocation"],
             *(row["archive"] for row in admission["historical_input_archives"])]]
    source_files = [path(row["historical_path"]) for row in build["executed_source_archives"]] + [Path(__file__).resolve(), ADMISSION_UTILITY]
    groups = {}
    for key, items in (("input_archives", files), ("executed_source_archives", source_files)):
        rows = []
        for index, original in enumerate(dict.fromkeys(items)):
            target = destination / f"{key}-{index:03d}-{original.name}"; shutil.copyfile(original, target)
            require(sha(original) == sha(target), "Source/input changed while archiving")
            rows.append({"historical_path": original.relative_to(ROOT).as_posix(), "archive": ref(target)})
        groups[key] = rows
    return groups


def source_outcomes(result, request, build, packet):
    require(result["schema"] == "clk03-helper-results.v1" and result["source_commit"] == PIN and result["complete"] is True
            and result["expected_answers_supplied"] is False and result["gates_updated"] is False
            and result["pure_body_fallback_used"] is False and result["processed_weather_physics_retained_unpaired"] is True
            and result["native_raw_record_index_claimed"] is False and result["original_error_text_peer_parity_claimed"] is False,
            "Actual helper schema/scope differs")
    identity(result["actual_binary"], build["helper_binary"]); identity(result["actual_request"], packet["helper_request"])
    for key, binding in packet["contracts"].items():
        identity(result["contracts"][key], binding)
    require(result["requested_counts"] == packet["counts"], "Actual helper requested coverage differs")
    statuses, calls = Counter(), Counter(); observed = []
    for lane in ("handoff", "weather"):
        actual, requested = result[lane + "_sequences"], request[lane + "_sequences"]
        require(len(actual) == len(requested), "Actual helper sequence coverage differs")
        for sequence, original in zip(actual, requested, strict=True):
            require(sequence["id"] == original["id"] and len(sequence["operations"]) == len(original["operations"]), "Actual operation coverage differs")
            for operation, supplied in zip(sequence["operations"], original["operations"], strict=True):
                require(operation["id"] == supplied["id"] and operation["kind"] == supplied["kind"]
                        and operation["requested_operation"] == supplied, "Native operation input identity differs")
                outcome, invoked = operation["call_outcome"], operation["actual_source_invoked"]
                require(type(invoked) is bool, "Actual invocation availability required")
                status = outcome["status"]
                require(status in ("source_returned", "source_fatal", "not-invoked-after-prior-source-outcome"), "Unknown source status")
                require(invoked == (status != "not-invoked-after-prior-source-outcome"), "Invocation/status availability differs")
                require(outcome["source_fatal"] is (True if status == "source_fatal" else False if invoked else None), "Actual fatal availability differs")
                statuses[status] += 1
                if invoked:
                    calls[operation["kind"]] += 1
                observed.append({"lane": lane, "sequence_id": sequence["id"], "operation_id": operation["id"], "kind": operation["kind"],
                                 "actual_source_invoked": invoked, "call_outcome": outcome})
    require(result["actual_operation_counts"] == dict(calls) and result["actual_operation_invocations"] == sum(calls.values())
            and result["actual_operations_skipped"] == statuses["not-invoked-after-prior-source-outcome"], "Actual source count transcription differs")
    return {"requested_counts": result["requested_counts"], "actual_operation_counts": dict(calls), "actual_source_status_counts": dict(statuses),
            "observed_operations": observed, "request_root_coverage_complete": True, "source_outputs_numerically_compared": False,
            "source_error_text_parity_claimed": False, "original_data_review_required_before_Rust_numerical_execution": True}


def inventory(directory):
    return [ref(item) for item in sorted(directory.rglob("*")) if item.is_file()]


def run(args):
    directory = path(args.output_dir)
    require(directory.is_relative_to(RAW.resolve()) and directory != RAW.resolve() and not directory.exists(), "Fresh contained output required")
    directory.mkdir(parents=True)
    record = {"schema": "clk03-original-helper-execution.v1", "status": "preflight-in-progress", "started_utc": utc(),
              "actual_launcher_argv": sys.orig_argv, "actual_launcher_cwd": str(Path.cwd().resolve()), "energyplus_commit": PIN,
              "Rust_executed": False, "Rust_compared": False, "gates_updated": False, "builds_or_Git_executed": False,
              "source_expected_answers_supplied": False, "deletion_performed": False, "preservation_observations": {},
              "processed_weather_physics_retained_unpaired": True, "original_data_review_required_before_Rust": True}
    module = build = runtime = groups = reviews = packet = admission_module = admission_binding = None; return_code = 1
    try:
        build_path, review_path = path(args.build_receipt), path(args.build_review)
        module, build, reviews, packet, admission_module, admission_binding, admission_execution = utilities(
            build_path, review_path, path(args.launcher_review), path(args.runtime_admission), path(args.runtime_admission_review))
        request = read(verify(packet["helper_request"])); runtime = runtime_identity(module, read(verify(build["core_build"])))
        groups = archive_inputs(directory, build, reviews, packet, admission_module, admission_binding, admission_execution); record.update(groups)
        own = next(row["archive"] for row in groups["executed_source_archives"] if path(row["historical_path"]) == Path(__file__).resolve())
        record.update(executed_launcher={"historical_path": Path(__file__).resolve().relative_to(ROOT).as_posix(), "archive": own},
                      native_driver_build=ref(build_path), independent_reviews=reviews, actual_request=packet["helper_request"],
                      contracts=packet["contracts"], compiler_runtime=runtime, runtime_packet=packet, runtime_input_admission=admission_binding,
                      runtime_input_admission_execution=admission_execution, historical_compiled_contracts=build["contracts"],
                      historical_compiled_request=build["helper_request"], new_build_claimed=False)
        record["preservation_observations"]["before_execution"] = guards(module, build, runtime, reviews, packet, admission_module, admission_binding)
        native_dir = directory / "native-output"; require(not native_dir.exists(), "Native output must be fresh before child")
        command = [str(verify(build["helper_binary"])), str(ROOT), str(verify(packet["helper_request"])), str(native_dir)]
        started = utc()
        for binding in reviews.values():
            require(instant(read(verify(binding))["review_completed_utc"]) <= instant(started), "Every independent review must precede actual child")
        require(instant(admission_module.validate_admission(admission_binding)["admitted_utc"]) <= instant(started),
                "Actual runtime admission must precede genuine child")
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
                     "stderr": ref(directory / "helper-stderr.log"), "binary": build["helper_binary"], "request": packet["helper_request"],
                     "native_driver_build": ref(build_path), "runtime_input_admission": admission_binding,
                     "runtime_packet": packet, "spawn_error": spawn_error}
        write(directory / "helper-command.json", execution); record["execution"] = ref(directory / "helper-command.json")
        record["actual_helper_exit_code"] = code
        require(spawn_error is None, "Actual helper could not launch; receipts/logs retained")
        result_path = native_dir / "results.json"
        record["results"] = ref(result_path) if result_path.is_file() else None
        record["actual_source_outcomes"] = source_outcomes(read(result_path), request, build, packet) if result_path.is_file() else None
        record["preservation_observations"]["after_execution"] = guards(module, build, runtime, reviews, packet, admission_module, admission_binding)
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
        if module is not None and build is not None and runtime is not None and admission_binding is not None:
            try:
                if "after_execution" not in record["preservation_observations"]:
                    record["preservation_observations"]["failure_final"] = guards(module, build, runtime, reviews, packet, admission_module, admission_binding)
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
    parser.add_argument("--runtime-admission", required=True); parser.add_argument("--runtime-admission-review", required=True)
    parser.add_argument("--output-dir", required=True)
    return run(parser.parse_args())


if __name__ == "__main__":
    raise SystemExit(main())
