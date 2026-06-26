from __future__ import annotations

import argparse
import json
import math
import statistics
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

REQUIRED_MEASUREMENTS = [
    {
        "id": "parse_time",
        "label": "Parse time",
        "field": None,
        "phases": ["raw_model", "oracle_input_stage"],
        "description": "Rust epJSON RawModel parse when arbitrary-run timing is available; oracle input staging otherwise.",
    },
    {
        "id": "compile_time",
        "label": "Compile time",
        "field": None,
        "phases": ["typed_compile"],
        "description": "RawModel-to-TypedModel compile and reference resolution.",
    },
    {
        "id": "graph_build_time",
        "label": "Graph build time",
        "field": None,
        "phases": ["graph_build"],
        "description": "SimulationModel and ModelGraph construction.",
    },
    {
        "id": "execution_plan_build_time",
        "label": "Execution plan build time",
        "field": None,
        "phases": ["execution_plan"],
        "description": "ExecutionPlan and output registry precomputation.",
    },
    {
        "id": "runtime_time",
        "label": "Runtime time",
        "field": None,
        "phases": ["rust_runtime"],
        "description": "Rust runtime execution excluding setup, output export, and report generation.",
    },
    {
        "id": "output_export_time",
        "label": "Output export time",
        "field": None,
        "phases": ["rust_output_export", "rust_artifact_write"],
        "description": "Rust result/report artifact serialization.",
    },
    {
        "id": "report_generation_time",
        "label": "Report generation time",
        "field": None,
        "phases": ["report_generation", "rust_compare_report"],
        "description": "Markdown/JSON comparison or run report generation after runtime outputs exist.",
    },
    {
        "id": "energyplus_cli_time",
        "label": "EnergyPlus CLI time",
        "field": "energyplus_oracle_wall_seconds",
        "phases": ["energyplus_oracle"],
        "description": "EnergyPlus oracle process wall time.",
    },
    {
        "id": "rust_cli_time",
        "label": "Rust CLI time",
        "field": "ep_cli_total_wall_seconds",
        "phases": ["ep_cli_total"],
        "description": "Rust CLI/evidence command wall time measured inside ep_cli when available.",
    },
    {
        "id": "rust_runtime_only_time",
        "label": "Rust runtime-only time",
        "field": None,
        "phases": ["rust_runtime"],
        "description": "Rust runtime loop wall time without setup/export/report overhead.",
    },
    {
        "id": "trace_overhead_time",
        "label": "Trace overhead",
        "field": None,
        "phases": ["trace_overhead"],
        "description": "Detailed/debug trace metadata and snapshot artifact overhead.",
    },
]

PERFORMANCE_PLOTS = {
    "stage_timing_stacked_bar": "plots/stage_timing_stacked_bar.png",
    "trace_overhead": "plots/trace_overhead.png",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build a machine-readable performance summary.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.1.0")
    return parser.parse_args()


def evidence_root(repo_root: Path, version: str) -> Path:
    return repo_root / ".runtime" / "release-evidence" / f"v{version}"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def percentile(values: list[float], pct: float) -> float | None:
    if not values:
        return None
    if len(values) == 1:
        return values[0]
    ordered = sorted(values)
    rank = (len(ordered) - 1) * pct
    low = math.floor(rank)
    high = math.ceil(rank)
    if low == high:
        return ordered[low]
    weight = rank - low
    return ordered[low] * (1.0 - weight) + ordered[high] * weight


def stat(values: list[Any]) -> dict[str, Any]:
    samples = [float(value) for value in values if value is not None]
    if not samples:
        return {"count": 0, "min": None, "median": None, "mean": None, "p90": None, "max": None}
    return {
        "count": len(samples),
        "min": min(samples),
        "median": statistics.median(samples),
        "mean": statistics.fmean(samples),
        "p90": percentile(samples, 0.9),
        "max": max(samples),
    }


def phase_seconds(sample: dict[str, Any], phase_names: list[str]) -> float | None:
    for phase in sample.get("phases", []):
        if phase.get("name") in phase_names and phase.get("wall_seconds") is not None:
            return float(phase["wall_seconds"])
    return None


def measurement_value(sample: dict[str, Any], measurement: dict[str, Any]) -> float | None:
    field = measurement.get("field")
    if field and sample.get(field) is not None:
        return float(sample[field])
    return phase_seconds(sample, list(measurement.get("phases", [])))


def sample_from_timing(
    timing: dict[str, Any],
    run_index: int = 0,
    gate_elapsed: Any = None,
    energyplus_elapsed: Any = None,
) -> dict[str, Any]:
    phases = timing.get("phases") if isinstance(timing.get("phases"), list) else []
    phase_sample = {"phases": phases}
    energyplus_oracle = timing.get("energyplus_oracle_wall_seconds")
    if energyplus_oracle is None:
        energyplus_oracle = phase_seconds(phase_sample, ["energyplus_oracle"])
    rust_compare_report = timing.get("rust_compare_report_wall_seconds")
    if rust_compare_report is None:
        rust_compare_report = phase_seconds(phase_sample, ["rust_compare_report"])
    ep_cli_total = timing.get("ep_cli_total_wall_seconds")
    if ep_cli_total is None:
        ep_cli_total = timing.get("total_wall_seconds")
    if ep_cli_total is None:
        ep_cli_total = phase_seconds(phase_sample, ["ep_cli_total"])
    gate_overhead = timing.get("release_gate_overhead_seconds")
    if gate_overhead is None and gate_elapsed is not None and ep_cli_total is not None:
        gate_overhead = max(float(gate_elapsed) - float(ep_cli_total), 0.0)
    return {
        "run": run_index,
        "release_gate_wall_seconds": gate_elapsed,
        "ep_cli_total_wall_seconds": None if ep_cli_total is None else float(ep_cli_total),
        "energyplus_oracle_wall_seconds": None
        if energyplus_oracle is None
        else float(energyplus_oracle),
        "rust_compare_report_wall_seconds": None
        if rust_compare_report is None
        else float(rust_compare_report),
        "release_gate_overhead_seconds": gate_overhead,
        "energyplus_reported_elapsed_seconds": energyplus_elapsed,
        "phases": phases,
    }


def timing_samples_from_case(case: dict[str, Any]) -> list[dict[str, Any]]:
    samples = case.get("timing_samples") or []
    if samples:
        return samples
    timing = case.get("timing")
    if not isinstance(timing, dict):
        return []
    return [
        sample_from_timing(
            timing,
            gate_elapsed=case.get("gate_elapsed_seconds"),
            energyplus_elapsed=case.get("energyplus_elapsed_seconds"),
        )
    ]


def required_measurement_statistics(samples: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        measurement["id"]: {
            "label": measurement["label"],
            "description": measurement["description"],
            "sources": {
                "field": measurement.get("field"),
                "phases": measurement.get("phases", []),
            },
            "statistics": stat([measurement_value(sample, measurement) for sample in samples]),
        }
        for measurement in REQUIRED_MEASUREMENTS
    }


def cold_repeated_summary(samples: list[dict[str, Any]]) -> dict[str, Any]:
    cold = samples[:1]
    repeated = samples[1:]
    return {
        "policy": "first captured sample is reported as cold_run; all later samples are repeated_runs",
        "cold_run": {
            "release_gate": stat([sample.get("release_gate_wall_seconds") for sample in cold]),
            "rust_cli": stat([sample.get("ep_cli_total_wall_seconds") for sample in cold]),
            "rust_runtime_only": stat(
                [
                    measurement_value(
                        sample,
                        next(item for item in REQUIRED_MEASUREMENTS if item["id"] == "rust_runtime_only_time"),
                    )
                    for sample in cold
                ]
            ),
        },
        "repeated_runs": {
            "release_gate": stat([sample.get("release_gate_wall_seconds") for sample in repeated]),
            "rust_cli": stat([sample.get("ep_cli_total_wall_seconds") for sample in repeated]),
            "rust_runtime_only": stat(
                [
                    measurement_value(
                        sample,
                        next(item for item in REQUIRED_MEASUREMENTS if item["id"] == "rust_runtime_only_time"),
                    )
                    for sample in repeated
                ]
            ),
        },
    }


def case_performance(case: dict[str, Any]) -> dict[str, Any]:
    samples = timing_samples_from_case(case)
    required = required_measurement_statistics(samples)
    return {
        "case_id": case.get("case_id"),
        "milestone": case.get("milestone"),
        "measurement": "wall-clock seconds",
        "sample_count": len(samples),
        "energyplus_cli_oracle": stat([sample.get("energyplus_oracle_wall_seconds") for sample in samples]),
        "rust_compare_report": stat([sample.get("rust_compare_report_wall_seconds") for sample in samples]),
        "ep_cli_total": stat([sample.get("ep_cli_total_wall_seconds") for sample in samples]),
        "release_gate": stat([sample.get("release_gate_wall_seconds") for sample in samples]),
        "release_gate_overhead": stat([sample.get("release_gate_overhead_seconds") for sample in samples]),
        "phase_breakdown": case.get("timing_statistics", {}).get("phases", {}),
        "required_measurements": required,
        "rust_runtime_only": required["rust_runtime_only_time"]["statistics"],
        "trace_overhead": required["trace_overhead_time"]["statistics"],
        "cold_repeated_runs": cold_repeated_summary(samples),
    }


def arbitrary_run_performance(repo_root: Path, evidence: dict[str, Any]) -> list[dict[str, Any]]:
    runs = []
    for index, record in enumerate(evidence.get("arbitrary_runs", [])):
        if not record.get("available"):
            continue
        summary_path = record.get("summary_path")
        if not summary_path:
            continue
        path = repo_root / str(summary_path)
        if not path.is_file():
            continue
        summary = load_json(path)
        timing = summary.get("timing")
        if not isinstance(timing, dict):
            continue
        samples = [sample_from_timing(timing, run_index=index)]
        required = required_measurement_statistics(samples)
        runs.append(
            {
                "label": record.get("label"),
                "summary_path": str(summary_path),
                "runtime_class": record.get("runtime_class"),
                "run_result_state": record.get("run_result_state"),
                "sample_count": len(samples),
                "required_measurements": required,
                "rust_runtime_only": required["rust_runtime_only_time"]["statistics"],
                "trace_overhead": required["trace_overhead_time"]["statistics"],
                "cold_repeated_runs": cold_repeated_summary(samples),
                "phase_breakdown": {
                    phase.get("name"): stat([phase.get("wall_seconds")])
                    for phase in timing.get("phases", [])
                    if phase.get("name")
                },
            }
        )
    return runs


def build_summary(repo_root: Path, version: str) -> dict[str, Any]:
    source_path = evidence_root(repo_root, version) / "numeric-conformance-evidence.json"
    evidence = load_json(source_path)
    cases = [case_performance(case) for case in evidence.get("cases", [])]
    arbitrary_runs = arbitrary_run_performance(repo_root, evidence)
    all_samples = [
        sample
        for case in evidence.get("cases", [])
        for sample in timing_samples_from_case(case)
    ]
    for run in arbitrary_runs:
        summary_path = repo_root / run["summary_path"]
        summary = load_json(summary_path)
        all_samples.append(sample_from_timing(summary.get("timing", {})))
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    dynamic_summary = None
    if dynamic.get("available"):
        samples = dynamic.get("timing_samples", [])
        dynamic_summary = {
            "case_id": dynamic.get("case_id"),
            "measurement": "wall-clock seconds",
            "sample_count": len(samples),
            "release_gate": stat([sample.get("release_gate_wall_seconds") for sample in samples]),
            "energyplus_reported_elapsed": stat(
                [sample.get("energyplus_reported_elapsed_seconds") for sample in samples]
            ),
            "rust_report_residual": stat([sample.get("rust_report_residual_seconds") for sample in samples]),
        }
    return {
        "schema_version": 1,
        "version": version,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "source_json": source_path.as_posix(),
        "measurement_definitions": {
            "parse_time": "raw_model phase for arbitrary runs, or oracle input staging for conformance evidence runs.",
            "compile_time": "typed_compile phase when arbitrary-run timing is available.",
            "graph_build_time": "graph_build phase for SimulationModel/ModelGraph construction.",
            "execution_plan_build_time": "execution_plan phase for source-order plan and registry precomputation.",
            "runtime_time": "rust_runtime phase.",
            "output_export_time": "rust_output_export phase for arbitrary runs, or rust_artifact_write for conformance evidence.",
            "report_generation_time": "report_generation phase for arbitrary runs, or rust_compare_report for conformance evidence.",
            "energyplus_cli_oracle": "EnergyPlus oracle run that produces oracle files.",
            "rust_compare_report": "Rust model/oracle loading, evaluation, comparison, and artifact writing after oracle files exist.",
            "ep_cli_total": "Staging, oracle execution, conversion, Rust comparison, and manifest writes inside ep_cli.",
            "rust_runtime_only": "runtime loop wall time separated from setup, export, and report generation when arbitrary-run timing is available.",
            "trace_overhead": "trace_overhead phase for detailed/debug trace metadata and snapshot artifacts.",
            "release_gate": "PowerShell/cargo/assertion wrapper wall-clock around the gate.",
            "release_gate_overhead": "release_gate minus ep_cli_total when both are available.",
        },
        "required_measurements": required_measurement_statistics(all_samples),
        "repeat_policy": {
            "promoted_gate_repeats": evidence.get("timing_repeats"),
            "dynamic_diagnostic_repeats": evidence.get("dynamic_timing_repeats"),
            "warmup_discard": "not applied in this summary; every captured sample is reported",
            "cold_repeated_classification": "first captured sample is cold_run; samples after the first are repeated_runs",
            "recommended_future_policy": "N=10 with first 2 discarded before median/p90 publication",
        },
        "cold_repeated_runs": cold_repeated_summary(all_samples),
        "cases": cases,
        "arbitrary_runs": arbitrary_runs,
        "dynamic_diagnostic": dynamic_summary,
        "artifacts": {
            "json": f".runtime/release-evidence/v{version}/performance-summary.json",
            "plots": {
                name: f".runtime/release-evidence/v{version}/{relative_path}"
                for name, relative_path in PERFORMANCE_PLOTS.items()
            },
        },
    }


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    summary = build_summary(repo_root, args.version)
    output_path = evidence_root(repo_root, args.version) / "performance-summary.json"
    output_path.write_text(json.dumps(summary, indent=2), encoding="utf-8")
    print("Performance summary")
    print(f"  cases: {len(summary['cases'])}")
    print(f"  json: {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
