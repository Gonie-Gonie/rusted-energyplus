#!/usr/bin/env python3
"""Record canonical ZON-01 input-only unit or ordinary production commands.

The archived committed Rust binary receives only its own request, or IDF/EPW.
Original proof metadata is required before launch; numerical comparison is a
separate reader. A partial failed matrix is retained with the actual exit code.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess
import sys

sys.dont_write_bytecode = True
from zon01_launch_provenance import (
    ROOT, RAW, PROVENANCE, PROVENANCE_SHA, Bindings, archive_sources, exact,
    original_metadata, packet, path_of, read, ref, require, same_binding,
    timestamp, verify_rust_build, write,
)

EXAMPLE = "crates/ep_runtime/examples/zon01_tuples.rs"
COMMON_SOURCES = {
    "crates/ep_runtime/src/heat_balance/state.rs",
    "crates/ep_runtime/src/heat_balance/zone_air_initialization.rs",
    "crates/ep_runtime/src/heat_balance/air_manager.rs",
    "crates/ep_runtime/src/heat_balance/initialization/state_shell.rs",
}
CLI_SOURCES = {
    "crates/ep_cli/src/main.rs",
    "crates/ep_runtime/src/heat_balance/zone_air_initialization_trace.rs",
    "crates/ep_runtime/src/runtime.rs",
    "crates/ep_runtime/src/ideal_loads/coupled_runtime.rs",
    "crates/ep_run/src/zon01_trace.rs",
    "crates/ep_run/src/pipeline.rs",
}
ORIGINAL = {
    "helper": {"path": ".runtime/porting/ZON-01/original-helper-first-01/helper-reference.json",
               "sha256": "e677a24b59ddb47bbde2eecd76cdce4e99a3ffebd8ca63576c4e60a938c20d59"},
    "matrix": {"path": ".runtime/porting/ZON-01/original-native-first-01/matrix.json",
               "sha256": "8672ce9c0a674efe1c8d46b715b2d0024f9d7004cec3d51cb8edabbfc76c0b42"},
    "review": {"path": ".runtime/porting/ZON-01/independent-original-data-review-01/review.json",
               "sha256": "a396421d1358f5150482c2823f773b147eddf6f7ce488d3f698fe7aeb03f3361"},
}


def recorded_command(command, directory, build, compilation, completed, bindings):
    recorder = ROOT / "tools/porting/record_command.py"
    recorder_ref = ref(recorder)
    bindings.check(recorder_ref)
    bindings.unchanged()
    result = subprocess.run([sys.executable, "-X", "utf8", "-B", str(recorder),
                             "--output-dir", str(directory), "--", *command], cwd=ROOT, check=False)
    receipt_ref = ref(directory / "receipt.json")
    actual = read(bindings.check(receipt_ref))
    require(actual["schema"] == "recorded-porting-command.v1"
            and exact(actual["command"], command) and path_of(actual["cwd"]) == ROOT
            and actual["launch_error"] is None and type(actual["exit_code"]) is int
            and actual["exit_code"] == result.returncode,
            "Actual command/exit/launch differs; receipt retained")
    require(completed <= timestamp(actual["started_utc"]) <= timestamp(actual["finished_utc"])
            and timestamp(compilation["finished_utc"]) <= timestamp(actual["started_utc"]),
            "Original review/build chronology differs")
    require(actual["source_bytes_match_before_and_after"] is True
            and same_binding(actual["source_snapshot"], build["source_snapshot"])
            and actual["repository_before"]["head"] == actual["repository_after"]["head"]
            and actual["recorder_updates_gates"] is False
            and actual["recorder_supplies_reference_answers"] is False
            and actual["executed_launcher"]["archive"]["sha256"] == recorder_ref["sha256"],
            "Actual input/source observation boundary differs")
    bindings.check(actual["executed_launcher"]["archive"])
    bindings.unchanged()
    return receipt_ref, actual


def run_units(directory, build_ref, build, available, compilation, completed, refs, request, sources, bindings):
    command = [str(bindings.check(build["binary"])), str(bindings.check(request))]
    command_ref, actual = recorded_command(command, directory / "command", build, compilation, completed, bindings)
    execution = {
        "schema": "zon01-Rust-helper-execution.v1", "implementation_stage": "canonical_owned_state",
        "build": build_ref, "binary": build["binary"], "request": request, "contracts": refs,
        "command_receipt": command_ref, "results": actual["stdout"],
        "executed_launcher": sources[0]["archive"], "executed_source_archives": sources,
        "reviewed_example_source": available[EXAMPLE],
        "original_helper_execution": ORIGINAL["helper"], "native_original_matrix": ORIGINAL["matrix"],
        "original_first_review": ORIGINAL["review"],
        "execution_repository_head": actual["repository_before"]["head"],
        "implementation_commit": build["implementation_commit"], "crates_tree": build["crates_tree"],
        "committed_source_certification": build["committed_source_certification"],
        "original_outputs_supplied_to_Rust": False, "ordinary_native_pristine_member_pairing_claimed": False,
        "physics_executed": False, "scientific_comparison_performed": False, "gates_updated": False,
    }
    write(directory / "execution.json", execution)
    require(actual["exit_code"] == 0, "Actual canonical helper failed; raw receipt retained")
    result = read(bindings.check(actual["stdout"]))
    require(result["schema"] == "zon01-helper-results.v1"
            and result["implementation_stage"] == "canonical_owned_state"
            and result["reference_outputs_supplied_to_Rust"] is False
            and result["expected_values_supplied"] is False
            and result["physics_executed"] is False and result["gates_updated"] is False,
            "Actual canonical helper result boundary differs")
    print(json.dumps({"execution": ref(directory / "execution.json"), "actual_exit": actual["exit_code"]}))


def run_production(directory, build_ref, build, compilation, completed, refs, cases, sources, bindings):
    selected = cases["Rust_physical_case_ids"]
    levels = cases["Rust_physical_trace_levels"]
    require(exact(selected, ["A-24H", "A-72H", "B-BOTH-24H"])
            and exact(levels, ["full", "summary"]) and cases["Rust_physical_command_count"] == 6,
            "Frozen production plan differs")
    frozen = {row["id"]: row for row in cases["native_cases"]}
    matrix = {
        "schema": "zon01-production-matrix.v1", "implementation_commit": build["implementation_commit"],
        "crates_tree": build["crates_tree"], "build": build_ref, "binary": build["binary"], "contracts": refs,
        "executed_launcher": sources[0]["archive"], "executed_source_archives": sources,
        "original_helper_execution": ORIGINAL["helper"], "native_original_matrix": ORIGINAL["matrix"],
        "original_first_review": ORIGINAL["review"], "requested_case_ids": selected,
        "cases": [], "actual_command_count": 0, "complete": False, "requested_subset_complete": False,
        "original_outputs_supplied_to_Rust": False, "gates_updated": False,
    }
    matrix_path = directory / "matrix.json"
    # Update only the active factual matrix; each actual recorder receipt is immutable.
    def update():
        matrix_path.write_text(json.dumps(matrix, indent=2, allow_nan=False) + "\n", encoding="utf8", newline="\n")
    update()
    for case_id in selected:
        case = frozen[case_id]
        input_path, weather = bindings.check(case["input"]), bindings.check(case["weather"])
        bindings.check(case["metadata"])
        row = {key: case[key] for key in ("scope", "duration", "input", "weather", "metadata")}
        row["case_id"] = case_id
        matrix["cases"].append(row)
        case_dir = directory / case_id
        case_dir.mkdir()
        for level in levels:
            output = case_dir / level
            command = [str(bindings.check(build["binary"])), "run", str(input_path), "--weather", str(weather),
                       "--output-dir", str(output), "--mode", "compatibility", "--partial", "deny",
                       "--trace-level", level, "--porting-scope", case["scope"]]
            actual_ref, actual = recorded_command(command, case_dir / (level + "-command"),
                build, compilation, completed, bindings)
            artifacts = [ref(path) for path in sorted(output.rglob("*")) if path.is_file()]
            summary_path = output / "run-summary.json"
            summary_ref = ref(summary_path) if summary_path.is_file() else None
            execution = {
                "schema": "zon01-Rust-cli-execution.v1", "build": build_ref, "binary": build["binary"],
                "command_receipt": actual_ref, "input": case["input"], "weather": case["weather"],
                "metadata": case["metadata"], "run_summary": summary_ref, "artifacts": artifacts,
                "output_directory": output.relative_to(ROOT).as_posix(), "trace_level": level,
                "configured_porting_scope": case["scope"], "executed_launcher": sources[0]["archive"],
                "executed_recorder": actual["executed_launcher"], "implementation_commit": build["implementation_commit"],
                "execution_repository_head": actual["repository_before"]["head"],
                "original_outputs_supplied_to_Rust": False, "dry_run": False, "gates_updated": False,
            }
            execution_path = case_dir / (level + "-execution.json")
            write(execution_path, execution)
            row[level.title()] = {"execution": ref(execution_path), "command_receipt": actual_ref,
                "run_summary": summary_ref, "artifacts": artifacts, "output_directory": execution["output_directory"]}
            matrix["actual_command_count"] += 1
            update()
            require(summary_ref is not None, "Actual run summary absent; raw evidence retained")
            summary = read(bindings.check(summary_ref))
            require(actual["exit_code"] == summary["exit_code"] == 0 and summary["status"] == "success"
                    and summary["config"]["dry_run"] is False and summary["config"]["oracle_baseline"] is False
                    and summary["config"]["compare_oracle"] is False and summary["oracle"] is None
                    and summary["config"]["trace_level"] == level,
                    "Actual ordinary input-only outcome differs; raw evidence retained")
            print(f"{case_id} {level} actual_exit={actual['exit_code']}", flush=True)
    matrix["requested_subset_complete"] = True
    matrix["complete"] = matrix["actual_command_count"] == 6
    update()
    print(json.dumps({"matrix": ref(matrix_path), "actual_commands": matrix["actual_command_count"], "complete": matrix["complete"]}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", required=True)
    parser.add_argument("--kind", choices=["units", "production"], required=True)
    parser.add_argument("--output-dir", required=True)
    args = parser.parse_args()
    require(Path.cwd().resolve() == ROOT, "Launch from repository cwd")
    directory = path_of(args.output_dir)
    require(directory.is_relative_to(RAW) and directory != RAW and not directory.exists(), "Fresh bounded output required")
    bindings = Bindings()
    require(ref(PROVENANCE)["sha256"] == PROVENANCE_SHA, "Pinned generic reader changed")
    bindings.check(ref(PROVENANCE))
    refs, cases, request = packet(bindings)
    completed = original_metadata(ORIGINAL["helper"], ORIGINAL["matrix"], ORIGINAL["review"], refs, cases, bindings)
    build_ref = ref(path_of(args.build))
    unit = args.kind == "units"
    build, available, compilation = verify_rust_build(build_ref, bindings,
        kind="example" if unit else "cli", example="zon01_tuples" if unit else None, committed=True,
        required_sources=COMMON_SOURCES | ({EXAMPLE} if unit else CLI_SOURCES))
    bindings.unchanged()
    directory.mkdir(parents=True)
    sources = archive_sources(directory, [Path(__file__).resolve(),
        Path(__file__).with_name("zon01_launch_provenance.py"), PROVENANCE, ROOT / "tools/porting/record_command.py"])
    if unit:
        run_units(directory, build_ref, build, available, compilation, completed, refs, request, sources, bindings)
    else:
        run_production(directory, build_ref, build, compilation, completed, refs, cases, sources, bindings)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError, TypeError) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(2)
