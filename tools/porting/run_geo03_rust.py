#!/usr/bin/env python3
"""Record input-only GEO-03 runs through an archived ordinary Rust CLI.

Original proof metadata is verified before launch. Original numerical outputs
are never read or supplied to Rust. Numerical comparison and gates are separate.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import shutil
import subprocess
import sys

sys.dont_write_bytecode = True
from geo03_provenance import (
    ROOT, Bindings, exact, frozen_contracts, path_of, read, ref, require,
    same_binding, timestamp, verify_contract_bindings, verify_rust_build,
)

RAW = ROOT / ".runtime/porting/GEO-03"
POSITIVE = ("A-24H", "A-72H", "B-BOTH-24H")
NEGATIVE = "GEO03-A-TOPOLOGY-DUPLICATE-WALL"
PROVENANCE_SHA = "5ca27645245ee9e7206a31c8906904394a5216625723cbfad12a5d5f49da9bf5"
REQUIRED_SOURCES = {
    "crates/ep_cli/src/main.rs",
    "crates/ep_runtime/src/geometry.rs",
    "crates/ep_runtime/src/geometry/zone_volume.rs",
    "crates/ep_runtime/src/geometry/zone_volume/topology.rs",
    "crates/ep_runtime/src/geometry/zone_volume_trace.rs",
    "crates/ep_runtime/src/heat_balance/initialization.rs",
    "crates/ep_runtime/src/heat_balance/air_manager.rs",
    "crates/ep_runtime/src/psychrometrics.rs",
    "crates/ep_runtime/src/psychrometrics/production_trace.rs",
    "crates/ep_raw_model/src/lib.rs",
    "crates/ep_raw_model/src/idf_order.rs",
    "crates/ep_compiler/src/compiler.rs",
    "crates/ep_run/src/geo03_trace.rs",
    "crates/ep_run/src/pipeline.rs",
    "crates/ep_run/src/geometry_trace.rs",
    "crates/ep_run/src/psychrometrics_trace.rs",
}
ORIGINAL = {
    "helper": {"path": ".runtime/porting/GEO-03/original-helper-first-01/helper-reference.json",
               "sha256": "339cb2bc39f0f4b95bcfebb1965d6b15b17f8fee3965e12d1480ff9601a6c1a5"},
    "matrix": {"path": ".runtime/porting/GEO-03/original-native-first-01/matrix.json",
               "sha256": "4f6edb1834b4b5452310452d8d9253658734fac18c51d96007b0a1f465995ad4"},
    "review": {"path": ".runtime/porting/GEO-03/independent-original-first-review-01/review.json",
               "sha256": "bc6b46a46c1e6d34aab4e8b8f701e614728c1a53839beda43aa9bc2df5723f11"},
}


def write(path, value):
    Path(path).write_bytes((json.dumps(value, indent=2, allow_nan=False) + "\n").encode("utf-8"))


def original_metadata(refs, cases, bindings):
    """Read receipts/reviews only; hash result bytes without parsing values."""
    helper, matrix, review = [read(bindings.check(ORIGINAL[k])) for k in ("helper", "matrix", "review")]
    require(helper["schema"] == "geo03-original-helper-execution.v1"
            and helper["Rust_compared"] is False and helper["gates_updated"] is False,
            "Original helper boundary differs")
    verify_contract_bindings(helper["contracts"], refs, bindings)
    require(same_binding(helper["request"], cases["helper_request"]), "Original input request differs")
    bindings.check(helper["results"])
    require(review["schema"] == "geo03-independent-original-data-review.v1"
            and review["status"] == "pass-source-provenance-before-Rust-numerical-execution"
            and same_binding(review["reviewed_helper_reference"], ORIGINAL["helper"])
            and same_binding(review["reviewed_native_matrix"], ORIGINAL["matrix"])
            and same_binding(review["helper_results"], helper["results"])
            and same_binding(review["reviewed_build_identity"], helper["independent_native_build_review"])
            and review["Rust_compared"] is False and review["scientific_reference_math_recomputed"] is False
            and review["engines_executed_by_review"] is False and review["gates_updated"] is False,
            "Independent original-first proof differs")
    verify_contract_bindings(review["reviewed_contracts"], refs, bindings)
    require(matrix["schema"] == "geo03-original-matrix.v1" and matrix["complete"] is True
            and matrix["requested_subset_complete"] is True and type(matrix["case_count"]) is int
            and matrix["case_count"] == len(matrix["cases"]) == 12
            and {r["case_id"] for r in matrix["cases"]} == {r["id"] for r in cases["native_cases"]}
            and matrix["Rust_compared"] is False and matrix["gates_updated"] is False,
            "Original native matrix incomplete")
    require(timestamp(review["original_completed_utc"]) <= timestamp(review["review_completed_utc"]),
            "Original review predates completed source evidence")
    bindings.check(review["reviewed_build_identity"])
    return timestamp(review["review_completed_utc"])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", required=True, help="Immutable committed porting-Rust-build.v1 CLI receipt")
    parser.add_argument("--output-dir", required=True, help="Fresh child of .runtime/porting/GEO-03")
    parser.add_argument("--case", action="append", default=[], help="Optional frozen case subset; full completion is separate")
    args = parser.parse_args()
    require(Path.cwd().resolve() == ROOT, "Run from the repository root")
    directory = path_of(args.output_dir)
    require(directory.is_relative_to(RAW) and directory != RAW and not directory.exists(), "Fresh bounded output required")
    bindings = Bindings()
    provenance_ref = ref(Path(__file__).with_name("geo03_provenance.py"))
    require(provenance_ref["sha256"] == PROVENANCE_SHA, "Reviewed provenance reader changed")
    bindings.check(provenance_ref)
    refs, _, cases, _ = frozen_contracts(None, bindings)
    completed = original_metadata(refs, cases, bindings)
    build_ref = ref(path_of(args.build))
    build, _, compilation = verify_rust_build(build_ref, bindings, kind="cli", committed=True,
        required_sources=REQUIRED_SOURCES)
    binary = bindings.check(build["binary"])
    selected = args.case or [*POSITIVE, NEGATIVE]
    require(len(selected) == len(set(selected)) and set(selected) <= {*POSITIVE, NEGATIVE}, "Unknown/duplicate requested case")
    frozen = {row["id"]: row for row in cases["native_cases"]}
    recorder = ROOT / "tools/porting/record_command.py"
    recorder_ref = ref(recorder)
    bindings.check(recorder_ref)
    bindings.unchanged()
    directory.mkdir(parents=True)
    sources = []
    source_dir = directory / "source"
    source_dir.mkdir()
    for current in (Path(__file__).resolve(), bindings.check(provenance_ref), recorder):
        archive = source_dir / current.name
        shutil.copyfile(current, archive)
        require(archive.read_bytes() == current.read_bytes(), "Launcher/dependency archive differs")
        sources.append({"historical_path": current.relative_to(ROOT).as_posix(), "archive": ref(archive)})
    executed_launcher = sources[0]["archive"]
    matrix = {
        "schema": "geo03-production-matrix.v1", "implementation_commit": build["implementation_commit"],
        "crates_tree": build["crates_tree"], "build": build_ref, "binary": build["binary"],
        "contracts": refs, "executed_launcher": executed_launcher, "executed_source_archives": sources,
        "original_helper_execution": ORIGINAL["helper"], "native_original_matrix": ORIGINAL["matrix"],
        "original_first_review": ORIGINAL["review"], "requested_case_ids": selected,
        "cases": [], "negative_cases": [], "actual_command_count": 0,
        "complete": False, "requested_subset_complete": False,
        "original_outputs_supplied_to_Rust": False, "gates_updated": False,
    }
    write(directory / "matrix.json", matrix)
    for case_id in selected:
        case = frozen[case_id]
        negative = case_id == NEGATIVE
        configured_scope = case["configured_Rust_scope"] if negative else case["scope"]
        require(configured_scope in {"A", "B"} and (case["scope"] is None if negative else case["scope"] in {"A", "B"}),
                "Actual scoped/diagnostic input boundary differs")
        input_path, weather = bindings.check(case["input"]), bindings.check(case["weather"])
        bindings.check(case["metadata"])
        row = {k: case[k] for k in ("kind", "scope", "duration", "input", "weather", "metadata")}
        row.update(case_id=case_id, configured_porting_scope=configured_scope)
        matrix["negative_cases" if negative else "cases"].append(row)
        case_dir = directory / case_id
        case_dir.mkdir()
        for level in (["full"] if negative else ["full", "summary"]):
            output, command_dir = case_dir / level, case_dir / (level + "-command")
            command = [str(binary), "run", str(input_path), "--weather", str(weather), "--output-dir", str(output),
                       "--mode", "compatibility", "--partial", "deny", "--trace-level", level,
                       "--porting-scope", configured_scope]
            result = subprocess.run([sys.executable, "-X", "utf8", "-B", str(recorder),
                                     "--output-dir", str(command_dir), "--", *command], cwd=ROOT, check=False)
            command_path = command_dir / "receipt.json"
            require(command_path.is_file(), "Actual recorder receipt missing; no synthetic exit created")
            actual_ref = ref(command_path)
            actual = read(command_path)
            artifacts = [ref(p) for p in sorted(output.rglob("*")) if p.is_file()]
            summary_path = output / "run-summary.json"
            summary_ref = ref(summary_path) if summary_path.is_file() else None
            execution = {
                "schema": "geo03-Rust-cli-execution.v1", "command_receipt": actual_ref,
                "build": build_ref, "binary": build["binary"], "input": case["input"], "weather": case["weather"],
                "metadata": case["metadata"], "run_summary": summary_ref,
                "output_directory": output.relative_to(ROOT).as_posix(), "artifacts": artifacts,
                "executed_launcher": executed_launcher, "executed_recorder": actual["executed_launcher"],
                "trace_level": level, "configured_porting_scope": configured_scope,
                "implementation_commit": build["implementation_commit"],
                "execution_repository_head": actual["repository_before"]["head"],
                "original_outputs_supplied_to_Rust": False, "dry_run": False, "gates_updated": False,
            }
            execution_path = case_dir / (level + "-execution.json")
            write(execution_path, execution)
            row[level.title()] = {"output_directory": execution["output_directory"], "execution": ref(execution_path),
                                  "command_receipt": actual_ref, "run_summary": summary_ref, "artifacts": artifacts}
            matrix["actual_command_count"] += 1
            write(directory / "matrix.json", matrix)
            require(actual["schema"] == "recorded-porting-command.v1" and exact(actual["command"], command)
                    and path_of(actual["cwd"]) == ROOT and actual["launch_error"] is None
                    and type(actual["exit_code"]) is int and actual["exit_code"] == result.returncode,
                    "Actual command/exit/launch identity differs; raw proof preserved")
            require(completed <= timestamp(actual["started_utc"]) <= timestamp(actual["finished_utc"])
                    and timestamp(compilation["finished_utc"]) <= timestamp(actual["started_utc"]), "Original/build chronology differs")
            require(actual["source_bytes_match_before_and_after"] is True
                    and same_binding(actual["source_snapshot"], build["source_snapshot"])
                    and actual["repository_before"]["head"] == actual["repository_after"]["head"]
                    and actual["recorder_updates_gates"] is False and actual["recorder_supplies_reference_answers"] is False,
                    "Rust invocation source/revision boundary differs")
            require(actual["executed_launcher"]["archive"]["sha256"] == recorder_ref["sha256"], "Actual recorder source differs")
            bindings.check(actual["executed_launcher"]["archive"])
            bindings.unchanged()
            require(summary_ref is not None, "Actual run-summary missing; raw proof preserved")
            summary = read(summary_path)
            expected_exit = 6 if negative else 0
            require(type(summary["exit_code"]) is int and summary["exit_code"] == actual["exit_code"] == expected_exit
                    and summary["status"] == ("runtime" if negative else "success"), "Unexpected actual Rust outcome; raw proof preserved")
            require(summary["config"]["dry_run"] is False and summary["config"]["oracle_baseline"] is False
                    and summary["config"]["compare_oracle"] is False and summary["oracle"] is None
                    and summary["config"]["trace_level"] == level, "Normal input-only/no-oracle configuration differs")
            print(f"{case_id} {level} actual_exit={actual['exit_code']}", flush=True)
    matrix["requested_subset_complete"] = True
    matrix["complete"] = set(selected) == {*POSITIVE, NEGATIVE} and matrix["actual_command_count"] == 7
    write(directory / "matrix.json", matrix)
    print(json.dumps({"matrix": ref(directory / "matrix.json"), "actual_commands": matrix["actual_command_count"],
                      "complete": matrix["complete"], "gates_updated": False}), flush=True)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError, TypeError) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(2)
