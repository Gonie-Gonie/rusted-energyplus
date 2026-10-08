#!/usr/bin/env python3
"""ZON-01 factual ordinary run provenance; no scientific RHS or engines.

recorded_run preserves the existing reviewed ordinary command reader, with
only its ZON-01 execution schema changed. Source and unit proofs are separate.
"""
from __future__ import annotations

from geo03_provenance import (
    ROOT, Bindings, archive_map, command_paths, exact, execution_interval,
    identical_content, integer, path_of, read, ref, require, same_binding,
    timestamp, verify_rust_build,
)
from zon01_launch_provenance import original_metadata, packet, PROVENANCE_SHA

REQUIRED_SOURCES = {
    "crates/ep_cli/src/main.rs",
    "crates/ep_runtime/src/heat_balance/state.rs",
    "crates/ep_runtime/src/heat_balance/zone_air_initialization.rs",
    "crates/ep_runtime/src/heat_balance/zone_air_initialization_trace.rs",
    "crates/ep_runtime/src/heat_balance/air_manager.rs",
    "crates/ep_runtime/src/heat_balance/initialization/state_shell.rs",
    "crates/ep_runtime/src/heat_balance/timestep.rs",
    "crates/ep_runtime/src/runtime.rs",
    "crates/ep_runtime/src/ideal_loads/coupled_runtime.rs",
    "crates/ep_run/src/zon01_trace.rs",
    "crates/ep_run/src/pipeline.rs",
}


def producer(matrix, bindings):
    require(matrix["schema"] == "zon01-production-matrix.v1" and matrix["complete"] is True
            and matrix["requested_subset_complete"] is True and matrix["original_outputs_supplied_to_Rust"] is False
            and matrix["gates_updated"] is False and integer(matrix["actual_command_count"], "CLI commands") == 6,
            "Incomplete or answer-fed production matrix")
    refs, cases, request = packet(bindings)
    require(exact(matrix["contracts"], refs) and exact(matrix["requested_case_ids"], cases["Rust_physical_case_ids"])
            and len(matrix["cases"]) == 3 and [row["case_id"] for row in matrix["cases"]] == cases["Rust_physical_case_ids"],
            "Actual frozen physical case/input identity differs")
    frozen = {row["id"]: row for row in cases["native_cases"]}
    for row in matrix["cases"]:
        declared = frozen[row["case_id"]]
        require(all(exact(row[key],declared[key]) for key in ("scope","duration","input","weather","metadata")),
                "Actual case differs from original frozen input")
    completed = original_metadata(matrix["original_helper_execution"], matrix["native_original_matrix"],
        matrix["original_first_review"], refs, cases, bindings)
    build, available, compilation = verify_rust_build(matrix["build"], bindings, kind="cli", committed=True,
        required_sources=REQUIRED_SOURCES)
    require(same_binding(build["binary"], matrix["binary"])
            and matrix["implementation_commit"] == build["implementation_commit"]
            and matrix["crates_tree"] == build["crates_tree"], "Actual committed CLI build differs")
    mappings = archive_map(matrix["executed_source_archives"], bindings)
    require(set(mappings) == {"tools/porting/run_zon01_rust.py", "tools/porting/zon01_launch_provenance.py",
                             "tools/porting/geo03_provenance.py", "tools/porting/record_command.py"}
            and same_binding(mappings["tools/porting/run_zon01_rust.py"], matrix["executed_launcher"])
            and mappings["tools/porting/geo03_provenance.py"]["sha256"] == PROVENANCE_SHA,
            "Actual launcher/dependency archive identity differs")
    return refs, cases, read(bindings.check(request)), completed, build, available, compilation


def recorded_run(case, item, matrix, build, available, compilation, completed, bindings, directories, negative=False):
    output = path_of(item["output_directory"])
    require(output.is_dir() and output not in directories, "Missing/reused physical run directory")
    directories.add(output)
    wrapper = read(bindings.check(item["execution"]))
    require(wrapper["schema"] == "zon01-Rust-cli-execution.v1" and wrapper["dry_run"] is False
            and wrapper["original_outputs_supplied_to_Rust"] is False and wrapper["gates_updated"] is False
            and path_of(wrapper["output_directory"]) == output, "Wrong ordinary invocation wrapper")
    level = wrapper["trace_level"]
    configured = "A" if negative else case["scope"]
    require(level in {"full", "summary"} and (not negative or level == "full")
            and wrapper["configured_porting_scope"] == configured, "Wrong physical configured scope/trace level")
    for key, expected in (("build", matrix["build"]), ("binary", matrix["binary"]), ("input", case["input"]),
                          ("weather", case["weather"]), ("metadata", case["metadata"]),
                          ("executed_launcher", matrix["executed_launcher"]), ("command_receipt", item["command_receipt"]),
                          ("run_summary", item["run_summary"])):
        require(same_binding(wrapper[key], expected), "Actual wrapper crossbinding differs: " + key)
        bindings.check(wrapper[key])
    require(wrapper["implementation_commit"] == wrapper["execution_repository_head"] == build["implementation_commit"],
            "Actual committed run revision differs")
    execution = read(bindings.check(wrapper["command_receipt"]))
    require(execution["schema"] == "recorded-porting-command.v1" and execution["launch_error"] is None
            and execution["source_bytes_match_before_and_after"] is True and execution["recorder_updates_gates"] is False
            and execution["recorder_supplies_reference_answers"] is False and path_of(execution["cwd"]) == ROOT,
            "Actual recorder boundary differs")
    start, _ = execution_interval(execution, bindings, exit_code=6 if negative else 0)
    require(completed <= start and timestamp(compilation["finished_utc"]) <= start, "Physical CLI precedes original/build review")
    require(execution["repository_before"]["head"] == execution["repository_after"]["head"] == build["implementation_commit"],
            "Repository changed or differs from the actual CLI build")
    command_paths(execution["command"], [matrix["binary"]["path"], "run", case["input"]["path"], "--weather", case["weather"]["path"],
        "--output-dir", str(output), "--mode", "compatibility", "--partial", "deny", "--trace-level", level,
        "--porting-scope", configured], {0, 2, 4, 6})
    require(exact(wrapper["executed_recorder"], execution["executed_launcher"])
            and execution["executed_launcher"]["historical_path"] == "tools/porting/record_command.py",
            "Actual executed recorder identity differs")
    bindings.check(execution["executed_launcher"]["archive"])
    expected_recorder = {x["historical_path"]: x["archive"] for x in matrix["executed_source_archives"]}["tools/porting/record_command.py"]
    require(identical_content(execution["executed_launcher"]["archive"], expected_recorder, bindings), "Executed recorder bytes differ")
    snapshot = read(bindings.check(execution["source_snapshot"]))
    require(snapshot["schema"] == "rust-command-source-snapshot.v1" and snapshot["bytes_normalized"] is False,
            "Actual source snapshot claim differs")
    executed = archive_map(snapshot["files"], bindings)
    require(set(executed) == set(available) and all(identical_content(executed[k], available[k], bindings) for k in available),
            "Physical invocation available-source inventory differs from actual build")
    require(exact(wrapper["artifacts"], item["artifacts"]), "Wrapper/output inventories differ")
    artifacts = {}
    for binding in item["artifacts"]:
        path = bindings.check(binding)
        require(path.is_relative_to(output) and path not in artifacts, "Duplicate/foreign artifact")
        artifacts[path] = binding
    require(set(artifacts) == {p.resolve() for p in output.rglob("*") if p.is_file()}, "Retained physical output inventory differs")
    summary_path = output / "run-summary.json"
    require(summary_path in artifacts and same_binding(item["run_summary"], artifacts[summary_path]), "Actual run summary absent")
    summary = read(summary_path)
    require(type(execution["exit_code"]) is int and type(summary["exit_code"]) is int
            and summary["exit_code"] == execution["exit_code"] == (6 if negative else 0), "Actual CLI outcome differs")
    config = summary["config"]
    require(config["dry_run"] is False and config["mode"] == "compatibility" and config["partial_policy"] == "deny"
            and config["trace_level"] == level and config["oracle_baseline"] is False and config["compare_oracle"] is False
            and config["hours"] is None and config["output_format"] == "rust-native"
            and summary["oracle"] is None and summary["comparison"] is None
            and summary["oracle_status"] == summary["compare_status"] == "not-requested", "Nonphysical or answer-fed configuration")
    if negative:
        require(summary["status"] == "runtime" and summary["message"] == "Rust runtime failed" and summary["rust_runtime"] is None,
                "Negative did not reach actual early runtime initialization rejection")
    else:
        require(summary["status"] == "success" and summary["message"] == "arbitrary run completed"
                and integer(summary["rust_runtime"]["samples"], "actual hourly samples") == {"24H": 24, "72H": 72}[case["duration"]]
                and summary["source_order_gate"]["matches"] is True and summary["selected_algorithm_lane"]["diagnostic_probe_used"] is False
                and summary["support"]["conformance_claim"] is False, "Successful supported normal route absent")
        if configured == "B":
            require(summary["rust_runtime"]["fixture_demand_injection_used"] is False
                    and integer(summary["rust_runtime"]["purchased_air_coupling_call_count"], "B actual system coupling") == 96,
                    "B is fixture-driven or missing real coupling")
    return output, summary, wrapper, artifacts
