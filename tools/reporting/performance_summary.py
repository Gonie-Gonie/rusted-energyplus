from __future__ import annotations

import argparse
import json
import math
import statistics
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build a machine-readable performance summary.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.32.0")
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


def case_performance(case: dict[str, Any]) -> dict[str, Any]:
    samples = case.get("timing_samples", [])
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
    }


def build_summary(repo_root: Path, version: str) -> dict[str, Any]:
    source_path = evidence_root(repo_root, version) / "numeric-conformance-evidence.json"
    evidence = load_json(source_path)
    cases = [case_performance(case) for case in evidence.get("cases", [])]
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
            "energyplus_cli_oracle": "EnergyPlus oracle run that produces oracle files.",
            "rust_compare_report": "Rust model/oracle loading, evaluation, comparison, and artifact writing after oracle files exist.",
            "ep_cli_total": "Staging, oracle execution, conversion, Rust comparison, and manifest writes inside ep_cli.",
            "release_gate": "PowerShell/cargo/assertion wrapper wall-clock around the gate.",
            "release_gate_overhead": "release_gate minus ep_cli_total when both are available.",
        },
        "repeat_policy": {
            "promoted_gate_repeats": evidence.get("timing_repeats"),
            "dynamic_diagnostic_repeats": evidence.get("dynamic_timing_repeats"),
            "warmup_discard": "not applied in this summary; every captured sample is reported",
            "recommended_future_policy": "N=10 with first 2 discarded before median/p90 publication",
        },
        "cases": cases,
        "dynamic_diagnostic": dynamic_summary,
        "artifacts": {
            "json": f".runtime/release-evidence/v{version}/performance-summary.json",
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
