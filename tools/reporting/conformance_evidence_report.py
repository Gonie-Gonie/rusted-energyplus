from __future__ import annotations

import argparse
import json
import re
import statistics
import subprocess
import time
import tomllib
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from oodocs import (
    Box,
    Chapter,
    Document,
    DocumentSettings,
    Figure,
    PageBreak,
    PageMargins,
    Paragraph,
    Table,
    TableOfContents,
    Theme,
    code,
)
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter


ORACLE_VERSION = "26.1.0"
CLAIM_BOUNDARY = (
    "This document compares the official 1ZoneUncontrolled dynamic heat-balance target and the "
    "ZoneHVAC:IdealLoadsAirSystem no-outdoor-air sensible target. Existing promoted gates remain "
    "regression locks for declared variables only; broad 1Zone dynamic and broad IdealLoads/HVAC "
    "compatibility are not claimed until the target output families in this report pass their own "
    "blocking gates."
)

CASE_LABELS = {
    "heat_balance_nomass_001": "HB no-mass",
    "surface_temperature_nomass_001": "Surface no-mass",
    "schedule_constant_001": "Schedule const",
    "weather_fields_001": "Weather fields",
    "internal_gains_001": "Internal gains",
    "official_1zone_uncontrolled_dynamic_diagnostic_001": "Official 1Zone dynamic",
    "ideal_loads_no_oa_sensible_conformance_001": "IdealLoads no-OA",
}

KEY_LABELS = {
    "ZONE ONE": "Zone One",
    "ALWAYSON": "AlwaysOn",
    "Environment": "Env",
}

VARIABLE_LABELS = {
    "Zone Mean Air Temperature": "Zone MAT",
    "Surface Inside Face Temperature": "Surface IFT",
    "Surface Outside Face Temperature": "Surface OFT",
    "Surface Inside Face Conduction Heat Transfer Rate": "Surface IF cond",
    "Surface Inside Face Conduction Heat Transfer Rate per Area": "Surface IF cond/area",
    "Surface Outside Face Conduction Heat Transfer Rate": "Surface OF cond",
    "Surface Outside Face Conduction Heat Transfer Rate per Area": "Surface OF cond/area",
    "Surface Heat Storage Rate": "Surface storage",
    "Zone Opaque Surface Inside Faces Conduction Rate": "Zone opaque cond",
    "Schedule Value": "Schedule value",
    "Site Outdoor Air Drybulb Temperature": "Outdoor drybulb",
    "Zone Total Internal Convective Heating Rate": "Internal convective",
    "Zone Thermostat Heating Setpoint Temperature": "Heat SP",
    "Zone Thermostat Cooling Setpoint Temperature": "Cool SP",
    "Zone Ideal Loads Zone Total Heating Rate": "IL total heat",
    "Zone Ideal Loads Zone Total Cooling Rate": "IL total cool",
    "Zone Ideal Loads Zone Sensible Heating Rate": "IL sens heat",
    "Zone Ideal Loads Zone Sensible Cooling Rate": "IL sens cool",
    "Zone Ideal Loads Supply Air Total Heating Rate": "IL supply heat",
    "Zone Ideal Loads Supply Air Total Cooling Rate": "IL supply cool",
    "System Node Temperature": "Node temp",
    "System Node Mass Flow Rate": "Node flow",
}

CLASS_LABELS = {
    "zone-state": "zone",
    "surface-state": "surface",
    "schedule": "sched",
    "weather": "weather",
    "internal-gain": "gain",
    "zone-state": "zone",
    "hvac-state": "hvac",
    "node-state": "node",
}


@dataclass(frozen=True)
class CaseSpec:
    milestone: str
    command: str
    summary_path: str
    oracle_end_path: str
    oracle_err_path: str


@dataclass(frozen=True)
class DynamicDiagnosticSpec:
    command: str
    digest_path: str
    oracle_end_path: str
    oracle_err_path: str
    case_manifest_path: str


CASE_SPECS = (
    CaseSpec(
        milestone="v0.8",
        command="compare-heat-balance-conformance",
        summary_path=r".runtime\heat-balance-conformance\26.1.0\heat_balance_nomass_001\compare\compare-summary.json",
        oracle_end_path=r".runtime\heat-balance-conformance\26.1.0\heat_balance_nomass_001\oracle\eplusout.end",
        oracle_err_path=r".runtime\heat-balance-conformance\26.1.0\heat_balance_nomass_001\oracle\eplusout.err",
    ),
    CaseSpec(
        milestone="v0.9",
        command="compare-surface-temperature-conformance",
        summary_path=r".runtime\surface-temperature-conformance\26.1.0\surface_temperature_nomass_001\compare\compare-summary.json",
        oracle_end_path=r".runtime\surface-temperature-conformance\26.1.0\surface_temperature_nomass_001\oracle\eplusout.end",
        oracle_err_path=r".runtime\surface-temperature-conformance\26.1.0\surface_temperature_nomass_001\oracle\eplusout.err",
    ),
    CaseSpec(
        milestone="v0.22",
        command="compare-schedule-conformance",
        summary_path=r".runtime\time-weather-schedule-conformance\26.1.0\schedule_constant_001\compare\compare-summary.json",
        oracle_end_path=r".runtime\time-weather-schedule-conformance\26.1.0\schedule_constant_001\oracle\eplusout.end",
        oracle_err_path=r".runtime\time-weather-schedule-conformance\26.1.0\schedule_constant_001\oracle\eplusout.err",
    ),
    CaseSpec(
        milestone="v0.22",
        command="compare-weather-conformance",
        summary_path=r".runtime\time-weather-schedule-conformance\26.1.0\weather_fields_001\compare\compare-summary.json",
        oracle_end_path=r".runtime\time-weather-schedule-conformance\26.1.0\weather_fields_001\oracle\eplusout.end",
        oracle_err_path=r".runtime\time-weather-schedule-conformance\26.1.0\weather_fields_001\oracle\eplusout.err",
    ),
    CaseSpec(
        milestone="v0.26",
        command="compare-internal-convective-gain-conformance",
        summary_path=r".runtime\internal-gains-conformance\26.1.0\internal_gains_001\compare\compare-summary.json",
        oracle_end_path=r".runtime\internal-gains-conformance\26.1.0\internal_gains_001\oracle\eplusout.end",
        oracle_err_path=r".runtime\internal-gains-conformance\26.1.0\internal_gains_001\oracle\eplusout.err",
    ),
    CaseSpec(
        milestone="IdealLoads",
        command="compare-ideal-loads-no-oa-sensible-conformance",
        summary_path=(
            r".runtime\ideal-loads-no-oa-sensible\26.1.0"
            r"\ideal_loads_no_oa_sensible_conformance_001\compare\compare-summary.json"
        ),
        oracle_end_path=(
            r".runtime\ideal-loads-no-oa-sensible\26.1.0"
            r"\ideal_loads_no_oa_sensible_conformance_001\oracle\eplusout.end"
        ),
        oracle_err_path=(
            r".runtime\ideal-loads-no-oa-sensible\26.1.0"
            r"\ideal_loads_no_oa_sensible_conformance_001\oracle\eplusout.err"
        ),
    ),
)

DYNAMIC_DIAGNOSTIC_SPEC = DynamicDiagnosticSpec(
    command=(
        "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-"
        "current-lw-converged-inside-ctf-out-hist-scriptf-flat-iter20-probe"
    ),
    digest_path=(
        r".runtime\official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-"
        r"balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-"
        r"warmup-min20-surface-iter20\26.1.0\official_1zone_uncontrolled_dynamic_diagnostic_001"
        r"\compare\compare-digest.json"
    ),
    oracle_end_path=(
        r".runtime\official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-"
        r"balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-"
        r"warmup-min20-surface-iter20\26.1.0\official_1zone_uncontrolled_dynamic_diagnostic_001"
        r"\oracle\eplusout.end"
    ),
    oracle_err_path=(
        r".runtime\official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-"
        r"balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-"
        r"warmup-min20-surface-iter20\26.1.0\official_1zone_uncontrolled_dynamic_diagnostic_001"
        r"\oracle\eplusout.err"
    ),
    case_manifest_path=r"data\conformance_cases\official_1zone_uncontrolled_dynamic_diagnostic_001\case.toml",
)

PORTING_FOCUS_MILESTONES = {"0.8", "0.9", "0.22", "0.26", "0.33"}

ONE_ZONE_FOCUS_SERIES = (
    ("ZONE ONE", "Zone Mean Air Temperature", "zone air"),
    ("ZONE ONE", "Zone Air Heat Balance Internal Convective Heat Gain Rate", "zone source"),
    ("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate", "zone exchange"),
    ("ZONE ONE", "Zone Air Heat Balance Air Energy Storage Rate", "zone storage"),
    ("ZN001:FLR001", "Surface Heat Storage Rate", "mass floor"),
    ("ZN001:FLR001", "Surface Inside Face Conduction Heat Transfer Rate", "mass floor"),
    ("ZN001:FLR001", "Surface Inside Face Convection Heat Gain Rate", "mass floor"),
    ("ZN001:FLR001", "Surface Inside Face Temperature", "mass floor"),
    ("ZN001:ROOF001", "Surface Outside Face Convection Heat Gain Rate", "roof exterior"),
    ("ZN001:ROOF001", "Surface Outside Face Net Thermal Radiation Heat Gain Rate", "roof exterior"),
    ("ZN001:ROOF001", "Surface Outside Face Solar Radiation Heat Gain Rate", "roof solar"),
    ("ZONE ONE", "Zone Opaque Surface Inside Faces Conduction Rate", "zone aggregate"),
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build release numerical conformance evidence.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.32.0")
    parser.add_argument("--skip-gate-run", action="store_true")
    parser.add_argument(
        "--timing-repeats",
        type=int,
        default=3,
        help="Number of repeated promoted-gate timing runs when gates are refreshed.",
    )
    parser.add_argument(
        "--run-dynamic-diagnostic",
        action="store_true",
        help="Refresh the active official 1Zone dynamic diagnostic lane before building the report.",
    )
    parser.add_argument(
        "--dynamic-timing-repeats",
        type=int,
        default=1,
        help="Number of repeated official 1Zone diagnostic timing runs when that lane is refreshed.",
    )
    return parser.parse_args()


def number_label(value: float | int | None, digits: int = 6, suffix: str = "") -> str:
    if value is None:
        return "n/a"
    return f"{float(value):.{digits}f}{suffix}"


def elapsed_label(value: float | int | None) -> str:
    if value is None:
        return "not rerun"
    return number_label(value, 3, "s")


def percent_label(numerator: float | None, denominator: float | None, digits: int = 3) -> str:
    if numerator is None or denominator in (None, 0):
        return "n/a"
    return f"{(float(numerator) / float(denominator)) * 100.0:.{digits}f}%"


def compact_number_label(value: float | int | None) -> str:
    if value is None:
        return "n/a"
    number = float(value)
    if number == 0.0:
        return "0"
    if abs(number) < 0.001:
        mantissa, exponent = f"{number:.3e}".split("e")
        mantissa = mantissa.rstrip("0").rstrip(".")
        return f"{mantissa}e{int(exponent)}"
    if abs(number) < 10.0:
        return f"{number:.3f}".rstrip("0").rstrip(".")
    return f"{number:.1f}"


def case_label(case_id: str) -> str:
    return CASE_LABELS.get(case_id, case_id)


def key_label(key: str | None) -> str:
    if key is None:
        return ""
    return KEY_LABELS.get(key, str(key))


def variable_label(variable: str | None) -> str:
    if variable is None:
        return ""
    return VARIABLE_LABELS.get(variable, str(variable))


def class_label(output_class: str | None) -> str:
    if output_class is None:
        return ""
    return CLASS_LABELS.get(output_class, str(output_class))


def status_label(status: str | None) -> str:
    if status in ("pass", "expected", "extracted"):
        return "ok"
    return "" if status is None else str(status)


def repo_path(repo_root: Path, relative: str) -> Path:
    return repo_root / Path(relative.replace("\\", "/"))


def relative_repo_path(repo_root: Path, path: Path) -> str:
    try:
        return path.relative_to(repo_root).as_posix()
    except ValueError:
        return path.as_posix()


def resolve_dynamic_digest_path(repo_root: Path, spec: DynamicDiagnosticSpec) -> Path:
    requested = repo_path(repo_root, spec.digest_path)
    if requested.is_file():
        return requested
    candidates = sorted(
        (
            path
            for path in (repo_root / ".runtime").glob("**/official_1zone_uncontrolled_dynamic_*/compare/compare-digest.json")
            if path.is_file()
        ),
        key=lambda path: path.stat().st_mtime,
        reverse=True,
    )
    return candidates[0] if candidates else requested


def read_toml(path: Path) -> dict[str, Any]:
    if not path.is_file():
        return {}
    with path.open("rb") as handle:
        return tomllib.load(handle)


def run_dev_command(repo_root: Path, command: str) -> float:
    start = time.perf_counter()
    subprocess.run(["cmd", "/c", str(repo_root / "scripts" / "dev.cmd"), command], cwd=repo_root, check=True)
    return time.perf_counter() - start


def elapsed_seconds(path: Path) -> float | None:
    if not path.is_file():
        return None
    text = path.read_text(encoding="utf-8", errors="replace")
    match = re.search(r"Elapsed Time=(?P<hours>\d+)hr\s+(?P<minutes>\d+)min\s+(?P<seconds>[0-9.]+)sec", text)
    if not match:
        return None
    return (
        float(match.group("hours")) * 3600.0
        + float(match.group("minutes")) * 60.0
        + float(match.group("seconds"))
    )


def phase_seconds(timing: dict[str, Any], phase_name: str) -> float | None:
    for phase in timing.get("phases") or []:
        if phase.get("name") == phase_name and phase.get("wall_seconds") is not None:
            return float(phase["wall_seconds"])
    return None


def timing_report(
    summary: dict[str, Any],
    gate_elapsed: float | None,
    energyplus_elapsed: float | None,
) -> dict[str, Any]:
    timing = summary.get("timing")
    if not isinstance(timing, dict):
        timing = {}
    phases = timing.get("phases") if isinstance(timing.get("phases"), list) else []
    energyplus_oracle_wall = timing.get("energyplus_oracle_wall_seconds")
    if energyplus_oracle_wall is None:
        energyplus_oracle_wall = phase_seconds(timing, "energyplus_oracle")
    if energyplus_oracle_wall is None:
        energyplus_oracle_wall = energyplus_elapsed
    rust_compare_report_wall = timing.get("rust_compare_report_wall_seconds")
    if rust_compare_report_wall is None:
        rust_compare_report_wall = phase_seconds(timing, "rust_compare_report")
    ep_cli_total_wall = timing.get("ep_cli_total_wall_seconds")
    if ep_cli_total_wall is None:
        ep_cli_total_wall = phase_seconds(timing, "ep_cli_total")
    gate_overhead = None
    if gate_elapsed is not None and ep_cli_total_wall is not None:
        gate_overhead = max(float(gate_elapsed) - float(ep_cli_total_wall), 0.0)
    return {
        "schema_version": int(timing.get("schema_version", 1)),
        "measurement": timing.get("measurement", "wall-clock seconds"),
        "primary_comparison_scope": timing.get(
            "primary_comparison_scope",
            "EnergyPlus oracle output production wall-clock versus Rust compare/evidence production wall-clock",
        ),
        "energyplus_oracle_wall_seconds": None
        if energyplus_oracle_wall is None
        else float(energyplus_oracle_wall),
        "rust_compare_report_wall_seconds": None
        if rust_compare_report_wall is None
        else float(rust_compare_report_wall),
        "ep_cli_total_wall_seconds": None if ep_cli_total_wall is None else float(ep_cli_total_wall),
        "release_gate_wall_seconds": gate_elapsed,
        "release_gate_overhead_seconds": gate_overhead,
        "energyplus_reported_elapsed_seconds": energyplus_elapsed,
        "phases": phases,
    }


def timing_sample(run_index: int, timing: dict[str, Any]) -> dict[str, Any]:
    return {
        "run": run_index,
        "release_gate_wall_seconds": timing.get("release_gate_wall_seconds"),
        "ep_cli_total_wall_seconds": timing.get("ep_cli_total_wall_seconds"),
        "energyplus_oracle_wall_seconds": timing.get("energyplus_oracle_wall_seconds"),
        "rust_compare_report_wall_seconds": timing.get("rust_compare_report_wall_seconds"),
        "release_gate_overhead_seconds": timing.get("release_gate_overhead_seconds"),
        "energyplus_reported_elapsed_seconds": timing.get("energyplus_reported_elapsed_seconds"),
        "phases": timing.get("phases", []),
    }


def numeric_values(values: list[Any]) -> list[float]:
    return [float(value) for value in values if value is not None]


def timing_stat(values: list[Any]) -> dict[str, Any]:
    samples = numeric_values(values)
    if not samples:
        return {"count": 0, "min": None, "median": None, "mean": None, "max": None, "stdev": None}
    return {
        "count": len(samples),
        "min": min(samples),
        "median": statistics.median(samples),
        "mean": statistics.fmean(samples),
        "max": max(samples),
        "stdev": statistics.stdev(samples) if len(samples) > 1 else 0.0,
    }


def summarize_timing_samples(samples: list[dict[str, Any]]) -> dict[str, Any]:
    fields = [
        "release_gate_wall_seconds",
        "ep_cli_total_wall_seconds",
        "energyplus_oracle_wall_seconds",
        "rust_compare_report_wall_seconds",
        "rust_report_residual_seconds",
        "release_gate_overhead_seconds",
        "energyplus_reported_elapsed_seconds",
    ]
    summary = {field: timing_stat([sample.get(field) for sample in samples]) for field in fields}
    phase_samples: dict[str, list[float]] = {}
    for sample in samples:
        for phase in sample.get("phases", []):
            name = phase.get("name")
            wall = phase.get("wall_seconds")
            if name is not None and wall is not None:
                phase_samples.setdefault(str(name), []).append(float(wall))
    summary["phases"] = {name: timing_stat(values) for name, values in sorted(phase_samples.items())}
    return summary


def error_summary(path: Path) -> dict[str, int | None]:
    if not path.is_file():
        return {"warnings": None, "severes": None}
    text = path.read_text(encoding="utf-8", errors="replace")
    match = re.search(r"Completed Successfully--\s*(?P<warnings>\d+) Warning;\s*(?P<severes>\d+) Severe", text)
    if not match:
        return {"warnings": None, "severes": None}
    return {"warnings": int(match.group("warnings")), "severes": int(match.group("severes"))}


def tolerance_for_class(summary: dict[str, Any], output_class: str) -> float | None:
    for tolerance in summary.get("tolerance_policy", []):
        if tolerance.get("variable_class") == output_class:
            return float(tolerance["max_abs_c"])
    return None


def normalized_series_rows(summary: dict[str, Any], level_filter: str | None = None) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for series in summary.get("series", []):
        if "output" in series:
            output = series["output"]
            level = series.get("level", "conformance")
            if level_filter is not None and level != level_filter:
                continue
            tolerance = tolerance_for_class(summary, output.get("class", ""))
            first_delta = series.get("first_delta_sample") or {}
            max_delta = series.get("max_delta_sample") or {}
            rows.append(
                {
                    "key": output.get("key"),
                    "variable": output.get("variable"),
                    "domain": output.get("domain"),
                    "class": output.get("class"),
                    "frequency": output.get("frequency"),
                    "source": output.get("source"),
                    "level": level,
                    "units": output.get("units"),
                    "samples": int(series.get("samples", 0)),
                    "status": series.get("status"),
                    "max_abs_delta_c": float(series.get("max_abs_delta_c", 0.0)),
                    "mean_abs_delta_c": float(series.get("mean_abs_delta_c", 0.0)),
                    "rmse_delta_c": float(series.get("rmse_delta_c", 0.0)),
                    "max_rel_delta": float(series.get("max_rel_delta", 0.0)),
                    "tolerance_max_abs_c": float(tolerance if tolerance is not None else 0.0),
                    "tolerance_max_rmse_c": float(tolerance if tolerance is not None else 0.0),
                    "first_delta_index": first_delta.get("index"),
                    "max_delta_index": max_delta.get("index"),
                }
            )
            continue

        level = series.get("level", "conformance")
        if level_filter is not None and level != level_filter:
            continue
        rows.append(
            {
                "key": series.get("key"),
                "variable": series.get("variable"),
                "domain": series.get("domain"),
                "class": series.get("class"),
                "frequency": series.get("frequency"),
                "source": series.get("source"),
                "level": level,
                "units": series.get("units"),
                "samples": int(series.get("compared_samples", series.get("observed_samples", 0))),
                "status": series.get("status"),
                "max_abs_delta_c": float(series.get("max_abs_delta", 0.0)),
                "mean_abs_delta_c": float(series.get("mean_abs_delta", 0.0)),
                "rmse_delta_c": float(series.get("rmse_delta", 0.0)),
                "max_rel_delta": float(series.get("max_rel_delta", 0.0)),
                "tolerance_max_abs_c": float(series.get("max_abs_tolerance", 0.0)),
                "tolerance_max_rmse_c": float(series.get("max_rmse_tolerance", 0.0)),
                "first_delta_index": (series.get("first_divergence") or {}).get("index"),
                "max_delta_index": None,
            }
        )

    for meter in summary.get("meter_series", []):
        level = meter.get("level", "diagnostic")
        if level_filter is not None and level != level_filter:
            continue
        rows.append(
            {
                "key": meter.get("name"),
                "variable": meter.get("name"),
                "domain": meter.get("domain", "meter"),
                "class": "meter",
                "frequency": meter.get("frequency"),
                "source": meter.get("source"),
                "level": level,
                "units": meter.get("units"),
                "samples": int(meter.get("compared_samples", meter.get("observed_samples", 0))),
                "status": meter.get("status"),
                "max_abs_delta_c": float(meter.get("max_abs_delta", 0.0)),
                "mean_abs_delta_c": float(meter.get("mean_abs_delta", 0.0)),
                "rmse_delta_c": float(meter.get("rmse_delta", 0.0)),
                "max_rel_delta": float(meter.get("max_rel_delta", 0.0)),
                "tolerance_max_abs_c": float(meter.get("max_abs_tolerance", 0.0)),
                "tolerance_max_rmse_c": float(meter.get("max_rmse_tolerance", 0.0)),
                "first_delta_index": (meter.get("first_divergence") or {}).get("index"),
                "max_delta_index": None,
            }
        )
    return rows


def promoted_series(summary: dict[str, Any]) -> list[dict[str, Any]]:
    return normalized_series_rows(summary, "conformance")


def load_case_report(
    repo_root: Path,
    spec: CaseSpec,
    skip_gate_run: bool,
    timing_repeats: int,
) -> dict[str, Any]:
    summary_path = repo_path(repo_root, spec.summary_path)
    if skip_gate_run and not summary_path.is_file():
        raise FileNotFoundError(f"Missing conformance summary: {summary_path}")

    timing_samples: list[dict[str, Any]] = []
    gate_elapsed = None
    summary: dict[str, Any] = {}
    energyplus_elapsed = None
    if summary_path.is_file():
        summary = json.loads(summary_path.read_text(encoding="utf-8"))
        energyplus_elapsed = elapsed_seconds(repo_path(repo_root, spec.oracle_end_path))
    if not skip_gate_run:
        for run_index in range(1, max(timing_repeats, 1) + 1):
            gate_elapsed = run_dev_command(repo_root, spec.command)
            summary = json.loads(summary_path.read_text(encoding="utf-8"))
            energyplus_elapsed = elapsed_seconds(repo_path(repo_root, spec.oracle_end_path))
            timing_samples.append(timing_sample(run_index, timing_report(summary, gate_elapsed, energyplus_elapsed)))
    if not summary:
        raise FileNotFoundError(f"Missing conformance summary: {summary_path}")

    if summary.get("comparison_class") != "conformance" or summary.get("conformance_claim") is not True:
        raise ValueError(f"Summary is not a promoted conformance claim: {summary_path}")
    if summary.get("status") != "pass":
        raise ValueError(f"Conformance summary did not pass: {summary.get('case_id')}")

    series_reports = promoted_series(summary)
    all_series_reports = normalized_series_rows(summary)
    if not series_reports:
        raise ValueError(f"Conformance summary has no promoted conformance series: {summary_path}")

    err = error_summary(repo_path(repo_root, spec.oracle_err_path))
    timing = timing_report(summary, gate_elapsed, energyplus_elapsed)
    max_abs_delta = max((series["max_abs_delta_c"] for series in series_reports), default=0.0)
    rmse_delta = max((series["rmse_delta_c"] for series in series_reports), default=0.0)
    return {
        "milestone": spec.milestone,
        "case_id": summary.get("case_id"),
        "oracle_version": summary.get("oracle_version"),
        "comparison_class": summary.get("comparison_class"),
        "conformance_claim": bool(summary.get("conformance_claim")),
        "status": summary.get("status"),
        "runtime_class": summary.get("runtime_class") or "time-weather-schedule",
        "tolerance_policy_label": summary.get("tolerance_policy_label"),
        "samples": int(summary.get("samples", summary.get("time_axis_samples", 0))),
        "heat_balance_timesteps": int(summary.get("heat_balance_timesteps", 0)),
        "zone_count": int(summary.get("zone_count", 0)),
        "surface_count": int(summary.get("surface_count", 0)),
        "series_count": len(series_reports),
        "reported_series_count": int(summary.get("series_count", len(series_reports))),
        "max_abs_delta_c": float(summary.get("max_abs_delta_c", max_abs_delta)),
        "rmse_delta_c": float(summary.get("rmse_delta_c", rmse_delta)),
        "max_rel_delta": float(
            summary.get("max_rel_delta", max((series["max_rel_delta"] for series in series_reports), default=0.0))
        ),
        "gate_elapsed_seconds": gate_elapsed,
        "energyplus_elapsed_seconds": energyplus_elapsed,
        "energyplus_oracle_wall_seconds": timing["energyplus_oracle_wall_seconds"],
        "rust_compare_report_wall_seconds": timing["rust_compare_report_wall_seconds"],
        "release_gate_overhead_seconds": timing["release_gate_overhead_seconds"],
        "timing": timing,
        "energyplus_warnings": err["warnings"],
        "energyplus_severes": err["severes"],
        "gate_script": (summary.get("gate") or {}).get("script"),
        "source_summary_json": spec.summary_path.replace("\\", "/"),
        "source_report_md": (summary.get("report_contract") or {}).get("path"),
        "series": series_reports,
        "all_series": all_series_reports,
        "diagnostic_series": [series for series in all_series_reports if series.get("level") != "conformance"],
        "timing_samples": timing_samples,
        "timing_statistics": summarize_timing_samples(timing_samples),
        "raw_summary": {
            key: summary.get(key)
            for key in [
                "claim_boundary",
                "selected_purchased_air_branch",
                "declared_ideal_loads_branch",
                "inactive_branches",
                "ideal_loads_feature_flags",
                "zone_demand_fixture_mode",
                "zone_demand_mismatch_classification",
                "zone_equipment_dispatch_path",
                "zone_equipment_dispatch_validation",
                "energy_source",
                "fuel_energy_rate_source",
                "meter_source",
            ]
            if key in summary
        },
    }


def diagnostic_series(summary: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for series in summary.get("series", []):
        output = series.get("output")
        if not isinstance(output, dict):
            continue
        rows.append(
            {
                "key": output.get("key"),
                "variable": output.get("variable"),
                "class": output.get("class"),
                "frequency": output.get("frequency"),
                "source": output.get("source"),
                "level": "diagnostic",
                "samples": int(series.get("samples", 0)),
                "status": series.get("status"),
                "max_abs_delta_c": float(series.get("max_abs_delta_c", 0.0)),
                "mean_abs_delta_c": float(series.get("mean_abs_delta_c", 0.0)),
                "rmse_delta_c": float(series.get("rmse_delta_c", 0.0)),
                "max_rel_delta": float(series.get("max_rel_delta", 0.0)),
                "first_delta_index": (series.get("first_delta_sample") or {}).get("index"),
                "max_delta_index": (series.get("max_delta_sample") or {}).get("index"),
            }
        )
    return rows


def find_series(
    series_rows: list[dict[str, Any]],
    key: str,
    variable: str,
) -> dict[str, Any] | None:
    for row in series_rows:
        if row.get("key") == key and row.get("variable") == variable:
            return row
    return None


def series_metric_label(series: dict[str, Any] | None) -> str:
    if series is None:
        return "missing"
    return (
        f"RMSE {compact_number_label(series.get('rmse_delta_c'))}; "
        f"max {compact_number_label(series.get('max_abs_delta_c'))}"
    )


def dynamic_focus_metric_label(dynamic: dict[str, Any], key: str, variable: str) -> str:
    if not dynamic.get("available"):
        return "missing diagnostic"
    return series_metric_label(find_series(dynamic.get("series", []), key, variable))


def dynamic_rmse_tiers(series_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    tiers = [
        ("exact", "RMSE <= 1e-9", 0.0, 1.0e-9),
        ("very low", "1e-9 < RMSE <= 0.1", 1.0e-9, 0.1),
        ("low", "0.1 < RMSE <= 1", 0.1, 1.0),
        ("medium", "1 < RMSE <= 10", 1.0, 10.0),
        ("high", "RMSE > 10", 10.0, None),
    ]
    rows: list[dict[str, Any]] = []
    for label, boundary, lower, upper in tiers:
        matched = [
            row
            for row in series_rows
            if float(row.get("rmse_delta_c", 0.0)) > lower
            and (upper is None or float(row.get("rmse_delta_c", 0.0)) <= upper)
        ]
        if label == "exact":
            matched = [row for row in series_rows if float(row.get("rmse_delta_c", 0.0)) <= upper]
        rows.append(
            {
                "tier": label,
                "boundary": boundary,
                "series_count": len(matched),
                "share": len(matched) / len(series_rows) if series_rows else None,
            }
        )
    return rows


def load_porting_rows(repo_root: Path) -> list[dict[str, Any]]:
    milestone_data = read_toml(repo_root / "specs" / "milestones.toml")
    milestone_rows: list[dict[str, Any]] = []
    for milestone in milestone_data.get("milestone", []):
        version = str(milestone.get("version", ""))
        if version not in PORTING_FOCUS_MILESTONES:
            continue
        milestone_rows.append(
            {
                "version": f"v{version}",
                "title": milestone.get("title", ""),
                "status": milestone.get("status", ""),
                "claim_level": milestone.get("claim_level", ""),
                "cases": milestone.get("required_cases", []),
                "variables": milestone.get("required_variables", []),
                "not_claimed": milestone.get("not_claimed", []),
            }
        )
    return milestone_rows


def build_dynamic_focus_rows(series_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for key, variable, group in ONE_ZONE_FOCUS_SERIES:
        series = find_series(series_rows, key, variable)
        if series is None:
            rows.append(
                {
                    "group": group,
                    "key": key,
                    "variable": variable,
                    "status": "missing",
                    "samples": 0,
                    "max_abs_delta_c": None,
                    "mean_abs_delta_c": None,
                    "rmse_delta_c": None,
                    "max_delta_index": None,
                }
            )
            continue
        rows.append(
            {
                "group": group,
                "key": key,
                "variable": variable,
                "status": series["status"],
                "samples": series["samples"],
                "max_abs_delta_c": series["max_abs_delta_c"],
                "mean_abs_delta_c": series["mean_abs_delta_c"],
                "rmse_delta_c": series["rmse_delta_c"],
                "max_delta_index": series["max_delta_index"],
            }
        )
    return rows


def build_dynamic_source_split(summary: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for sample in summary.get("inside_solve_max_sample_deltas", [])[:4]:
        rows.append(
            {
                "key": sample.get("key"),
                "sample_index": sample.get("sample_index"),
                "implied_solve_numerator_delta_w": sample.get("implied_solve_numerator_delta_w"),
                "tracked_solve_source_delta_w": sample.get("tracked_solve_source_delta_w"),
                "tracked_solve_source_coverage_ratio": sample.get("tracked_solve_source_coverage_ratio"),
                "reference_air_source_delta_w": sample.get("reference_air_source_delta_w"),
                "reference_air_coefficient_source_signed_delta_w": sample.get(
                    "reference_air_coefficient_source_signed_delta_w"
                ),
                "reference_air_temperature_source_signed_delta_w": sample.get(
                    "reference_air_temperature_source_signed_delta_w"
                ),
                "inside_conduction_signed_delta_w": sample.get("inside_conduction_signed_delta_w"),
                "inside_current_signed_delta_w": sample.get("inside_current_signed_delta_w"),
                "inside_current_outside_term_signed_delta_w": sample.get(
                    "inside_current_outside_term_signed_delta_w"
                ),
                "inside_current_inside_term_signed_delta_w": sample.get(
                    "inside_current_inside_term_signed_delta_w"
                ),
                "inside_current_cancellation_delta_w": sample.get(
                    "inside_current_cancellation_delta_w"
                ),
                "inside_history_signed_delta_w": sample.get("inside_history_signed_delta_w"),
                "inside_history_delta_w": sample.get("inside_history_delta_w"),
                "inside_net_longwave_delta_w": sample.get("inside_net_longwave_delta_w"),
                "solve_source_residual_delta_w": sample.get("solve_source_residual_delta_w"),
            }
        )
    return rows


def load_dynamic_diagnostic(
    repo_root: Path,
    spec: DynamicDiagnosticSpec,
    run_dynamic_diagnostic: bool,
    dynamic_timing_repeats: int,
) -> dict[str, Any]:
    timing_samples: list[dict[str, Any]] = []
    gate_elapsed = None
    if run_dynamic_diagnostic:
        for run_index in range(1, max(dynamic_timing_repeats, 1) + 1):
            gate_elapsed = run_dev_command(repo_root, spec.command)
            digest_path_for_sample = resolve_dynamic_digest_path(repo_root, spec)
            oracle_end_for_sample = digest_path_for_sample.parent.parent / "oracle" / "eplusout.end"
            energyplus_elapsed_for_sample = elapsed_seconds(oracle_end_for_sample)
            rust_residual = None
            if gate_elapsed is not None and energyplus_elapsed_for_sample is not None:
                rust_residual = max(float(gate_elapsed) - float(energyplus_elapsed_for_sample), 0.0)
            timing_samples.append(
                {
                    "run": run_index,
                    "release_gate_wall_seconds": gate_elapsed,
                    "energyplus_reported_elapsed_seconds": energyplus_elapsed_for_sample,
                    "rust_report_residual_seconds": rust_residual,
                }
            )

    digest_path = resolve_dynamic_digest_path(repo_root, spec)
    if not digest_path.is_file():
        digest_label = spec.digest_path.replace("\\", "/")
        return {
            "available": False,
            "reason": f"missing digest: {digest_label}",
            "command": spec.command,
            "timing_samples": timing_samples,
            "timing_statistics": summarize_timing_samples(timing_samples),
        }

    summary = json.loads(digest_path.read_text(encoding="utf-8"))
    manifest = read_toml(repo_path(repo_root, spec.case_manifest_path))
    series_rows = diagnostic_series(summary)
    top_bottlenecks = sorted(series_rows, key=lambda row: row["rmse_delta_c"], reverse=True)[:12]
    oracle_end_path = digest_path.parent.parent / "oracle" / "eplusout.end"
    oracle_err_path = digest_path.parent.parent / "oracle" / "eplusout.err"
    err = error_summary(oracle_err_path)
    warmup = summary.get("heat_balance_warmup") or {}
    total_timesteps = int(summary.get("heat_balance_timesteps", 0))
    warmup_timesteps = int(warmup.get("timestep_count", 0))
    run_period_timesteps = int(summary.get("heat_balance_run_period_timesteps", 0))
    energyplus_elapsed = elapsed_seconds(oracle_end_path)
    return {
        "available": True,
        "case_id": summary.get("case_id"),
        "title": manifest.get("title", "Official 1ZoneUncontrolled dynamic heat-balance diagnostic"),
        "source_kind": (manifest.get("manifest_v2") or {}).get("source_kind"),
        "source_file": (manifest.get("manifest_v2") or {}).get("source_file"),
        "idf": (manifest.get("input") or {}).get("idf"),
        "weather": (manifest.get("input") or {}).get("weather"),
        "comparison_class": summary.get("comparison_class"),
        "conformance_claim": bool(summary.get("conformance_claim")),
        "status": summary.get("status"),
        "samples": int(summary.get("samples", 0)),
        "outputs": len(summary.get("outputs", [])),
        "series_count": int(summary.get("series_count", len(series_rows))),
        "zone_count": int(summary.get("zone_count", 0)),
        "surface_count": int(summary.get("surface_count", 0)),
        "zone_air_algorithm": summary.get("zone_air_algorithm"),
        "surface_iteration_count": int(summary.get("surface_iteration_count", 1)),
        "ctf_seed_policy": (summary.get("ctf_seed") or {}).get("policy"),
        "ctf_initial_history_policy": summary.get("ctf_initial_history_policy"),
        "zone_conduction_report_source": summary.get("zone_conduction_report_source"),
        "max_abs_delta_c": float(summary.get("max_abs_delta_c", 0.0)),
        "rmse_delta_c": float(summary.get("rmse_delta_c", 0.0)),
        "max_rel_delta": float(summary.get("max_rel_delta", 0.0)),
        "heat_balance_timesteps": total_timesteps,
        "heat_balance_run_period_timesteps": run_period_timesteps,
        "heat_balance_warmup": warmup,
        "warmup_timestep_share": (warmup_timesteps / total_timesteps) if total_timesteps else None,
        "run_period_timestep_share": (run_period_timesteps / total_timesteps) if total_timesteps else None,
        "gate_elapsed_seconds": gate_elapsed,
        "energyplus_elapsed_seconds": energyplus_elapsed,
        "energyplus_warnings": err["warnings"],
        "energyplus_severes": err["severes"],
        "gate_script": spec.command,
        "source_digest_json": relative_repo_path(repo_root, digest_path),
        "source_report_md": (summary.get("report_contract") or {}).get("path"),
        "series": series_rows,
        "focus_series": build_dynamic_focus_rows(series_rows),
        "top_bottlenecks": top_bottlenecks,
        "rmse_tiers": dynamic_rmse_tiers(series_rows),
        "inside_solve_source_split": build_dynamic_source_split(summary),
        "timing_samples": timing_samples,
        "timing_statistics": summarize_timing_samples(timing_samples),
    }


def build_evidence(
    repo_root: Path,
    version: str,
    skip_gate_run: bool,
    run_dynamic_diagnostic: bool,
    timing_repeats: int,
    dynamic_timing_repeats: int,
) -> dict[str, Any]:
    cases = [load_case_report(repo_root, spec, skip_gate_run, timing_repeats) for spec in CASE_SPECS]
    all_series = [series for case in cases for series in case["series"]]
    failed_cases = [case for case in cases if case["status"] != "pass"]
    timing_cases = [case["timing"] for case in cases]
    dynamic_diagnostic = load_dynamic_diagnostic(
        repo_root,
        DYNAMIC_DIAGNOSTIC_SPEC,
        run_dynamic_diagnostic,
        dynamic_timing_repeats,
    )
    return {
        "schema_version": 1,
        "version": version,
        "oracle_version": ORACLE_VERSION,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "claim_boundary": CLAIM_BOUNDARY,
        "timing_repeats": 0 if skip_gate_run else max(timing_repeats, 1),
        "dynamic_timing_repeats": 0 if not run_dynamic_diagnostic else max(dynamic_timing_repeats, 1),
        "aggregate": {
            "status": "fail" if failed_cases else "pass",
            "case_count": len(cases),
            "series_count": len(all_series),
            "max_abs_delta_c": max((case["max_abs_delta_c"] for case in cases), default=0.0),
            "rmse_delta_c": max((case["rmse_delta_c"] for case in cases), default=0.0),
        },
        "timing": {
            "primary_comparison_scope": (
                "EnergyPlus oracle output production wall-clock versus Rust compare/evidence "
                "production wall-clock for the same conformance case. Release gate wall-clock, "
                "cargo/script/assert overhead, IDF conversion, and EnergyPlus self-reported "
                "elapsed time are reported separately."
            ),
            "energyplus_oracle_wall_seconds": sum(
                timing["energyplus_oracle_wall_seconds"] or 0.0 for timing in timing_cases
            ),
            "rust_compare_report_wall_seconds": sum(
                timing["rust_compare_report_wall_seconds"] or 0.0 for timing in timing_cases
            ),
            "ep_cli_total_wall_seconds": sum(timing["ep_cli_total_wall_seconds"] or 0.0 for timing in timing_cases),
            "release_gate_wall_seconds": sum(timing["release_gate_wall_seconds"] or 0.0 for timing in timing_cases),
            "release_gate_overhead_seconds": sum(
                timing["release_gate_overhead_seconds"] or 0.0 for timing in timing_cases
            ),
            "energyplus_reported_elapsed_seconds": sum(
                timing["energyplus_reported_elapsed_seconds"] or 0.0 for timing in timing_cases
            ),
        },
        "cases": cases,
        "porting_milestones": load_porting_rows(repo_root),
        "active_dynamic_diagnostic": dynamic_diagnostic,
        "artifacts": {
            "html": f".runtime/release-evidence/v{version}/numeric-conformance-evidence.html",
            "pdf": f".runtime/release-evidence/v{version}/numeric-conformance-evidence.pdf",
            "json": f".runtime/release-evidence/v{version}/numeric-conformance-evidence.json",
        },
    }


def axis_label(value: float, _position: int) -> str:
    if value == 0:
        return "0"
    if abs(value) < 0.001:
        return f"{value:.6f}".rstrip("0").rstrip(".")
    return f"{value:.3f}".rstrip("0").rstrip(".")


def chart_value_label(value: float) -> str:
    if value == 0.0:
        return "0"
    if abs(value) < 0.001:
        return f"{value:.6f}".rstrip("0").rstrip(".")
    if abs(value) < 10.0:
        return f"{value:.3f}".rstrip("0").rstrip(".")
    return f"{value:.1f}"


def style_axis(ax: Any) -> None:
    ax.grid(axis="x", color="#e3e7ed", linewidth=0.8)
    ax.set_axisbelow(True)
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["left"].set_color("#9aa7b5")
    ax.spines["bottom"].set_color("#9aa7b5")
    ax.tick_params(axis="x", colors="#5b6775", labelsize=8)
    ax.tick_params(axis="y", colors="#17212b", labelsize=9, length=0)
    ax.xaxis.set_major_formatter(FuncFormatter(axis_label))


def build_dual_bar_figure(
    title: str,
    rows: list[dict[str, Any]],
    primary_label: str,
    secondary_label: str,
    x_label: str,
    primary_color: str,
    secondary_color: str,
) -> Any:
    labels = [str(row["id"]) for row in rows]
    primary = [float(row["primary"]) for row in rows]
    secondary = [float(row["secondary"]) for row in rows]
    max_value = max(primary + secondary, default=0.0)
    if max_value <= 0.0:
        max_value = 1.0
    marker_size = max_value * 0.003
    secondary_visible = [value if value > 0.0 else marker_size for value in secondary]

    height = min(7.0, max(2.2, 1.15 + len(rows) * 0.28))
    fig, ax = plt.subplots(figsize=(7.2, height), dpi=180)
    fig.patch.set_facecolor("white")
    ax.set_facecolor("white")

    y_values = list(range(len(rows)))
    ax.barh(
        [y - 0.16 for y in y_values],
        primary,
        height=0.24,
        color=primary_color,
        edgecolor="none",
        label=primary_label,
    )
    ax.barh(
        [y + 0.16 for y in y_values],
        secondary_visible,
        height=0.16,
        color=secondary_color,
        edgecolor="none",
        label=secondary_label,
    )
    label_offset = max_value * 0.012
    for y, value in zip((y - 0.16 for y in y_values), primary):
        ax.text(
            value + label_offset,
            y,
            chart_value_label(value),
            va="center",
            ha="left",
            fontsize=6.6,
            color="#4b5563",
        )
    for y, value, visible in zip((y + 0.16 for y in y_values), secondary, secondary_visible):
        ax.text(
            visible + label_offset,
            y,
            chart_value_label(value),
            va="center",
            ha="left",
            fontsize=6.6,
            color=secondary_color,
        )
    ax.set_yticks(y_values, labels)
    ax.invert_yaxis()
    ax.set_xlim(0, max_value * 1.18)
    ax.set_xlabel(x_label, fontsize=9, color="#5b6775")
    ax.set_title(title, loc="left", fontsize=13, fontweight="bold", color="#17212b", pad=10)
    style_axis(ax)
    ax.legend(
        loc="upper center",
        bbox_to_anchor=(0.5, -0.14),
        ncol=2,
        fontsize=8,
        frameon=False,
    )
    fig.tight_layout(pad=1.0)
    return fig


def build_single_bar_figure(
    title: str,
    rows: list[dict[str, Any]],
    value_key: str,
    x_label: str,
    color: str,
) -> Any:
    labels = [str(row["id"]) for row in rows]
    values = [float(row[value_key] or 0.0) for row in rows]
    max_value = max(values, default=0.0)
    if max_value <= 0.0:
        max_value = 1.0

    height = max(2.2, 1.1 + len(rows) * 0.38)
    fig, ax = plt.subplots(figsize=(7.2, height), dpi=180)
    fig.patch.set_facecolor("white")
    ax.set_facecolor("white")
    y_values = list(range(len(rows)))
    ax.barh(y_values, values, height=0.24, color=color, edgecolor="none")
    label_offset = max_value * 0.012
    for y, value in zip(y_values, values):
        ax.text(
            value + label_offset,
            y,
            chart_value_label(value),
            va="center",
            ha="left",
            fontsize=6.6,
            color="#4b5563",
        )
    ax.set_yticks(y_values, labels)
    ax.invert_yaxis()
    ax.set_xlim(0, max_value * 1.18)
    ax.set_xlabel(x_label, fontsize=9, color="#5b6775")
    ax.set_title(title, loc="left", fontsize=13, fontweight="bold", color="#17212b", pad=10)
    style_axis(ax)
    fig.tight_layout(pad=1.0)
    return fig


def create_charts(evidence: dict[str, Any]) -> dict[str, Any]:
    accuracy_rows: list[dict[str, Any]] = []
    series_index = 1
    for case in evidence["cases"]:
        for series in case["series"]:
            accuracy_rows.append(
                {
                    "id": f"S{series_index:02d}",
                    "primary": series["tolerance_max_abs_c"],
                    "secondary": series["max_abs_delta_c"],
                }
            )
            series_index += 1

    timing_rows = [
        {
            "id": f"C{index + 1:02d}",
            "primary": case["rust_compare_report_wall_seconds"] or 0.0,
            "secondary": case["energyplus_oracle_wall_seconds"] or 0.0,
        }
        for index, case in enumerate(evidence["cases"])
    ]

    accuracy = build_dual_bar_figure(
        "Accuracy Against Declared Tolerance",
        accuracy_rows,
        "Declared tolerance",
        "Observed max abs delta",
        "Numeric delta",
        "#c9d8e8",
        "#1f7a5a",
    )
    timing = build_dual_bar_figure(
        "Same-Scope Case Timing",
        timing_rows,
        "Rust compare/report wall-clock",
        "EnergyPlus oracle wall-clock",
        "Seconds",
        "#3c6e9f",
        "#c77d1a",
    )
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    dynamic_rows: list[dict[str, Any]] = []
    if dynamic.get("available"):
        for index, row in enumerate(dynamic["top_bottlenecks"][:10], start=1):
            dynamic_rows.append(
                {
                    "id": f"D{index:02d}",
                    "rmse_delta_c": row["rmse_delta_c"],
                }
            )
    dynamic_bottlenecks = build_single_bar_figure(
        "1Zone Dynamic Diagnostic Bottlenecks",
        dynamic_rows or [{"id": "D00", "rmse_delta_c": 0.0}],
        "rmse_delta_c",
        "RMSE delta",
        "#7c4d9e",
    )
    return {"accuracy": accuracy, "timing": timing, "dynamic_bottlenecks": dynamic_bottlenecks}


def table(
    headers: list[str],
    rows: list[list[Any]],
    caption: str,
    column_widths: list[float] | None = None,
) -> Table:
    string_rows = [["" if value is None else str(value) for value in row] for row in rows]
    return Table(
        headers,
        string_rows,
        caption=caption,
        column_widths=column_widths,
        unit="in",
        header_background_color="#eef3f7",
        border_color="#d7dde5",
        alternate_row_background_color="#f8fafc",
        cell_padding=3.2,
        border_width=0.4,
        repeat_header_rows=True,
        split=True,
    )


def build_case_matrix(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    for index, case in enumerate(evidence["cases"], start=1):
        rows.append(
            [
                f"C{index:02d}",
                case["milestone"],
                case_label(case["case_id"]),
                status_label(case["status"]),
                case["series_count"],
                case["samples"],
                case["heat_balance_timesteps"],
                compact_number_label(case["max_abs_delta_c"]),
                compact_number_label(case["rmse_delta_c"]),
                "n/a"
                if case["rust_compare_report_wall_seconds"] is None
                else number_label(case["rust_compare_report_wall_seconds"], 3, "s"),
                "n/a"
                if case["energyplus_oracle_wall_seconds"] is None
                else number_label(case["energyplus_oracle_wall_seconds"], 3, "s"),
            ]
        )
    return table(
        [
            "ID",
            "MS",
            "Case",
            "OK",
            "Series",
            "Samples",
            "HB ts",
            "Max abs",
            "RMSE",
            "Rust wall",
            "E+ wall",
        ],
        rows,
        "Promoted numerical conformance case matrix.",
        [0.42, 0.5, 1.45, 0.45, 0.55, 0.6, 0.55, 0.75, 0.75, 0.62, 0.62],
    )


def build_accuracy_values(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    series_index = 1
    for case in evidence["cases"]:
        for series in case["series"]:
            rows.append(
                [
                    f"S{series_index:02d}",
                    case["milestone"],
                    case_label(case["case_id"]),
                    key_label(series["key"]),
                    variable_label(series["variable"]),
                    compact_number_label(series["max_abs_delta_c"]),
                    compact_number_label(series["tolerance_max_abs_c"]),
                    percent_label(series["max_abs_delta_c"], series["tolerance_max_abs_c"]),
                ]
            )
            series_index += 1
    return table(
        ["ID", "MS", "Case", "Key", "Output", "Max", "Tol", "Use"],
        rows,
        "Accuracy values backing the chart.",
        [0.42, 0.5, 1.45, 0.85, 1.55, 0.8, 0.8, 0.6],
    )


def build_timing_values(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    for index, case in enumerate(evidence["cases"], start=1):
        timing = case["timing"]
        rows.append(
            [
                f"C{index:02d}",
                case["milestone"],
                case_label(case["case_id"]),
                "n/a"
                if timing["rust_compare_report_wall_seconds"] is None
                else number_label(timing["rust_compare_report_wall_seconds"], 3, "s"),
                "n/a"
                if timing["energyplus_oracle_wall_seconds"] is None
                else number_label(timing["energyplus_oracle_wall_seconds"], 3, "s"),
                percent_label(
                    timing["rust_compare_report_wall_seconds"],
                    timing["energyplus_oracle_wall_seconds"],
                    1,
                ),
                elapsed_label(case["gate_elapsed_seconds"]),
                "n/a"
                if timing["release_gate_overhead_seconds"] is None
                else number_label(timing["release_gate_overhead_seconds"], 3, "s"),
                "n/a"
                if case["energyplus_elapsed_seconds"] is None
                else number_label(case["energyplus_elapsed_seconds"], 3, "s"),
                case["energyplus_warnings"],
                case["energyplus_severes"],
            ]
        )
    return table(
        [
            "ID",
            "Milestone",
            "Case",
            "Rust wall",
            "E+ wall",
            "Rust/E+",
            "Gate wall",
            "Gate overhead",
            "E+ self",
            "E+ warnings",
            "E+ severes",
        ],
        rows,
        "Same-scope case timing plus release-gate overhead and EnergyPlus self-reported elapsed time.",
        [0.42, 0.58, 1.32, 0.72, 0.72, 0.64, 0.72, 0.82, 0.65, 0.55, 0.55],
    )


def build_phase_timing_values(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    for index, case in enumerate(evidence["cases"], start=1):
        total = case["timing"].get("ep_cli_total_wall_seconds")
        for phase in case["timing"].get("phases", []):
            wall = phase.get("wall_seconds")
            rows.append(
                [
                    f"C{index:02d}",
                    case_label(case["case_id"]),
                    phase.get("name", ""),
                    phase.get("engine", ""),
                    "n/a" if wall is None else number_label(wall, 3, "s"),
                    percent_label(wall, total, 1),
                ]
            )
    return table(
        ["ID", "Case", "Phase", "Engine", "Wall", "Share"],
        rows,
        "Detailed phase timing recorded in each compare-summary.json timing object.",
        [0.42, 1.12, 1.72, 1.45, 0.72, 0.55],
    )


def short_claim_label(claim_level: str) -> str:
    labels = {
        "limited-conformance": "limited conf",
        "declared-variables-only": "declared vars",
        "diagnostic-only": "diagnostic",
    }
    return labels.get(claim_level, claim_level)


def short_status_label(status: str) -> str:
    labels = {
        "historical": "done",
        "complete": "done",
        "in_progress": "active",
    }
    return labels.get(status, status)


def list_label(values: list[str], max_items: int = 3) -> str:
    if not values:
        return "none"
    shown = values[:max_items]
    suffix = "" if len(values) <= max_items else f" +{len(values) - max_items}"
    return ", ".join(shown) + suffix


def case_list_label(values: list[str]) -> str:
    return list_label([case_label(value) for value in values], max_items=3)


def variable_list_label(values: list[str]) -> str:
    return list_label([variable_label(value) for value in values], max_items=3)


def heat_balance_algorithm_label(value: str | None) -> str:
    if not value:
        return "n/a"
    if "scriptf-flat-probe" in value:
        return "third-order ScriptF-flat, frozen hconv/ref-air, current LW, 20 passes"
    if "live-hconv-probe" in value:
        return "third-order ScriptF-flat live hconv probe"
    return value


def build_porting_table(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    for milestone in evidence.get("porting_milestones", []):
        rows.append(
            [
                milestone["version"],
                milestone["title"],
                short_status_label(milestone["status"]),
                short_claim_label(milestone["claim_level"]),
                case_list_label(milestone["cases"]),
                variable_list_label(milestone["variables"]),
            ]
        )
    return table(
        ["MS", "Algorithm / scope", "Status", "Claim level", "Evidence case", "Proof variables"],
        rows,
        "Porting status by milestone and evidence boundary.",
        [0.42, 1.75, 0.55, 0.85, 1.35, 2.1],
    )


def build_algorithm_porting_table(evidence: dict[str, Any]) -> Table:
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    warmup = dynamic.get("heat_balance_warmup") or {}
    rows = [
        [
            "Time/weather/schedule",
            "promoted",
            "v0.22 gates compare schedule and outdoor dry-bulb series exactly for declared variables.",
        ],
        [
            "No-mass zone/surface balance",
            "promoted subset",
            "v0.8/v0.9/v0.25 gates cover no-mass MAT, face temperature, and conduction rows only.",
        ],
        [
            "Internal convective gains",
            "promoted subset",
            "v0.26 gate covers declared convective gain magnitude; radiant, latent, and response coupling remain outside claim.",
        ],
        [
            "1Zone zone-air update",
            "diagnostic",
            dynamic_focus_metric_label(dynamic, "ZONE ONE", "Zone Mean Air Temperature"),
        ],
        [
            "Surface convection coupling",
            "diagnostic",
            dynamic_focus_metric_label(dynamic, "ZONE ONE", "Zone Air Heat Balance Surface Convection Rate"),
        ],
        [
            "Mass-floor CTF storage",
            "diagnostic bottleneck",
            dynamic_focus_metric_label(dynamic, "ZN001:FLR001", "Surface Heat Storage Rate"),
        ],
        [
            "Exterior solar/radiation",
            "diagnostic",
            dynamic_focus_metric_label(dynamic, "ZN001:ROOF001", "Surface Outside Face Solar Radiation Heat Gain Rate"),
        ],
        [
            "Run-period warmup",
            "diagnostic matched count",
            (
                f"Rust warmup days {warmup.get('day_count', 'n/a')} vs "
                f"EnergyPlus run-period days {warmup.get('oracle_run_period_day_count', 'n/a')}; "
                f"converged={str(warmup.get('converged', 'n/a')).lower()}."
            ),
        ],
    ]
    return table(
        ["Algorithm area", "Porting level", "Current evidence"],
        rows,
        "Algorithm porting level used by the active 1Zone dynamic evidence path.",
        [1.65, 1.05, 4.55],
    )


def build_dynamic_error_distribution_table(dynamic: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    if dynamic.get("available"):
        for row in dynamic.get("rmse_tiers", []):
            rows.append(
                [
                    row["tier"],
                    row["boundary"],
                    row["series_count"],
                    percent_label(row["series_count"], dynamic.get("series_count"), 1),
                ]
            )
    return table(
        ["Tier", "Boundary", "Series", "Share"],
        rows,
        "Diagnostic RMSE distribution across all active 1Zone hourly output series. This is not a pass/fail tolerance.",
        [0.85, 2.05, 0.65, 0.65],
    )


def build_dynamic_setup_table(dynamic: dict[str, Any]) -> Table:
    if not dynamic.get("available"):
        return table(
            ["Field", "Value"],
            [["Diagnostic artifact", dynamic.get("reason", "missing")]],
            "Active official 1Zone dynamic diagnostic setup.",
            [2.1, 4.9],
        )
    warmup = dynamic.get("heat_balance_warmup") or {}
    rows = [
        ["Case", dynamic["case_id"]],
        ["Source", dynamic.get("source_file") or dynamic.get("idf")],
        ["Weather", dynamic.get("weather")],
        ["Status", dynamic["status"]],
        ["Conformance claim", str(dynamic["conformance_claim"]).lower()],
        ["Outputs / series", f"{dynamic['outputs']} / {dynamic['series_count']}"],
        ["Samples", dynamic["samples"]],
        ["Zones / surfaces", f"{dynamic['zone_count']} / {dynamic['surface_count']}"],
        ["Algorithm", heat_balance_algorithm_label(dynamic.get("zone_air_algorithm"))],
        ["Surface passes", dynamic["surface_iteration_count"]],
        ["CTF seed / initial history", f"{dynamic['ctf_seed_policy']} / {dynamic['ctf_initial_history_policy']}"],
        ["Warmup days / timesteps", f"{warmup.get('day_count', 'n/a')} / {warmup.get('timestep_count', 'n/a')}"],
    ]
    return table(
        ["Field", "Value"],
        rows,
        "Active official 1ZoneUncontrolled dynamic diagnostic setup.",
        [1.85, 5.3],
    )


def build_dynamic_focus_table(dynamic: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    if dynamic.get("available"):
        for row in dynamic["focus_series"]:
            rows.append(
                [
                    row["group"],
                    key_label(row["key"]),
                    variable_label(row["variable"]),
                    status_label(row["status"]),
                    row["samples"],
                    compact_number_label(row["max_abs_delta_c"]),
                    compact_number_label(row["mean_abs_delta_c"]),
                    compact_number_label(row["rmse_delta_c"]),
                    row["max_delta_index"],
                ]
            )
    return table(
        ["Group", "Key", "Output", "OK", "N", "Max abs", "Mean abs", "RMSE", "Max idx"],
        rows,
        "1Zone focus metrics for user-visible and latent heat-balance physics.",
        [0.92, 0.92, 1.8, 0.36, 0.42, 0.62, 0.62, 0.62, 0.52],
    )


def build_dynamic_bottleneck_table(dynamic: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    if dynamic.get("available"):
        for index, row in enumerate(dynamic["top_bottlenecks"][:10], start=1):
            rows.append(
                [
                    f"D{index:02d}",
                    key_label(row["key"]),
                    variable_label(row["variable"]),
                    compact_number_label(row["max_abs_delta_c"]),
                    compact_number_label(row["mean_abs_delta_c"]),
                    compact_number_label(row["rmse_delta_c"]),
                    row["max_delta_index"],
                ]
            )
    return table(
        ["ID", "Key", "Output", "Max abs", "Mean abs", "RMSE", "Max idx"],
        rows,
        "Largest active 1Zone dynamic diagnostic deltas by RMSE.",
        [0.4, 0.95, 2.15, 0.72, 0.72, 0.72, 0.55],
    )


def build_dynamic_source_split_table(dynamic: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    if dynamic.get("available"):
        for row in dynamic["inside_solve_source_split"]:
            rows.append(
                [
                    key_label(row["key"]),
                    row["sample_index"],
                    compact_number_label(row["implied_solve_numerator_delta_w"]),
                    compact_number_label(row["tracked_solve_source_delta_w"]),
                    percent_label(row["tracked_solve_source_delta_w"], row["implied_solve_numerator_delta_w"], 1),
                    compact_number_label(row["reference_air_source_delta_w"]),
                    compact_number_label(row["inside_conduction_signed_delta_w"]),
                    compact_number_label(row["inside_current_signed_delta_w"]),
                    compact_number_label(row["inside_current_outside_term_signed_delta_w"]),
                    compact_number_label(row["inside_current_inside_term_signed_delta_w"]),
                    compact_number_label(row["inside_current_cancellation_delta_w"]),
                    compact_number_label(row["inside_history_signed_delta_w"]),
                    compact_number_label(row["inside_history_delta_w"]),
                    compact_number_label(row["inside_net_longwave_delta_w"]),
                    compact_number_label(row["solve_source_residual_delta_w"]),
                ]
            )
    return table(
        [
            "Key",
            "Idx",
            "Num",
            "Tracked",
            "Cov",
            "Ref",
            "CondS",
            "CurS",
            "CurOutS",
            "CurInS",
            "CurCancel",
            "HistS",
            "Hist",
            "LW",
            "Res",
        ],
        rows,
        "Inside solve max-sample source split for the current floor-storage bottleneck. Signed current/history columns separate CTF current alignment from history handoff. W except coverage.",
        [
            0.72,
            0.32,
            0.53,
            0.62,
            0.42,
            0.5,
            0.5,
            0.5,
            0.5,
            0.5,
            0.56,
            0.5,
            0.5,
            0.48,
            0.48,
        ],
    )


def build_dynamic_timing_table(dynamic: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    if dynamic.get("available"):
        warmup = dynamic.get("heat_balance_warmup") or {}
        gate_elapsed = dynamic.get("gate_elapsed_seconds")
        energyplus_elapsed = dynamic.get("energyplus_elapsed_seconds")
        rust_residual = None
        if gate_elapsed is not None and energyplus_elapsed is not None:
            rust_residual = max(float(gate_elapsed) - float(energyplus_elapsed), 0.0)
        rows.extend(
            [
                [
                    "Rust warmup",
                    "Rust",
                    warmup.get("timestep_count", "n/a"),
                    percent_label(warmup.get("timestep_count"), dynamic.get("heat_balance_timesteps"), 1),
                    "not persisted",
                    "loop count persisted",
                ],
                [
                    "Rust run period",
                    "Rust",
                    dynamic.get("heat_balance_run_period_timesteps"),
                    percent_label(
                        dynamic.get("heat_balance_run_period_timesteps"),
                        dynamic.get("heat_balance_timesteps"),
                        1,
                    ),
                    "not persisted",
                    "loop count persisted",
                ],
                [
                    "Rust plus compare residual",
                    "Rust/report",
                    dynamic.get("heat_balance_timesteps"),
                    "100.0%",
                    "n/a" if rust_residual is None else number_label(rust_residual, 3, "s"),
                    "gate wall minus E+ elapsed",
                ],
                [
                    "Full diagnostic gate",
                    "orchestrator",
                    dynamic.get("heat_balance_timesteps"),
                    "100.0%",
                    "n/a" if gate_elapsed is None else number_label(gate_elapsed, 3, "s"),
                    "PowerShell entrypoint wall",
                ],
                [
                    "EnergyPlus oracle",
                    "EnergyPlus",
                    "n/a",
                    "n/a",
                    "n/a"
                    if energyplus_elapsed is None
                    else number_label(energyplus_elapsed, 3, "s"),
                    "eplusout.end",
                ],
            ]
        )
    return table(
        ["Phase", "Engine", "Timesteps", "Share", "Elapsed", "Source"],
        rows,
        "Stage timing evidence. Rust warmup/run-period wall-time still needs runtime phase timers.",
        [1.35, 0.85, 0.72, 0.55, 0.85, 1.55],
    )


def ideal_loads_case(evidence: dict[str, Any]) -> dict[str, Any] | None:
    for case in evidence.get("cases", []):
        if case.get("case_id") == "ideal_loads_no_oa_sensible_conformance_001":
            return case
    return None


def mean_stat_label(stats: dict[str, Any], field: str, digits: int = 3) -> str:
    value = (stats.get(field) or {}).get("mean")
    return "n/a" if value is None else number_label(value, digits, "s")


def spread_stat_label(stats: dict[str, Any], field: str) -> str:
    row = stats.get(field) or {}
    if row.get("count", 0) == 0 or row.get("min") is None or row.get("max") is None:
        return "n/a"
    return f"{number_label(row['min'], 3, 's')} - {number_label(row['max'], 3, 's')}"


def build_two_case_scope_table(evidence: dict[str, Any]) -> Table:
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    ideal = ideal_loads_case(evidence) or {}
    dynamic_status = "missing"
    if dynamic.get("available"):
        dynamic_status = "candidate pass" if dynamic.get("status") == "pass" else str(dynamic.get("status"))
    rows = [
        [
            "1Zone Uncontrolled",
            dynamic.get("case_id", "official_1zone_uncontrolled_dynamic_diagnostic_001"),
            "official EnergyPlus ExampleFile",
            dynamic_status,
            dynamic.get("series_count", "missing"),
            dynamic.get("samples", "missing"),
            "Zone/surface heat balance, CTF history, convection, radiation, solar, weather coupling",
        ],
        [
            "IdealLoadsAirSystem No OA",
            ideal.get("case_id", "ideal_loads_no_oa_sensible_conformance_001"),
            "single-zone IdealLoads fixture",
            ideal.get("status", "missing"),
            ideal.get("reported_series_count", "missing"),
            ideal.get("samples", "missing"),
            "PurchasedAir no-outdoor-air sensible branch, demand input, supply node, report outputs",
        ],
    ]
    return table(
        ["System", "Case", "Source", "Status", "Series", "Samples", "Primary comparison boundary"],
        rows,
        "Two system-level comparisons used as the main conformance evidence boundary.",
        [1.15, 1.72, 1.15, 0.72, 0.48, 0.55, 2.1],
    )


def build_ideal_loads_setup_table(evidence: dict[str, Any]) -> Table:
    case = ideal_loads_case(evidence)
    if case is None:
        return table(["Field", "Value"], [["IdealLoads case", "missing"]], "IdealLoadsAirSystem setup.", [1.8, 5.2])
    raw = case.get("raw_summary") or {}
    flags = raw.get("ideal_loads_feature_flags") or {}
    active_flags = [name for name, active in flags.items() if active]
    inactive = raw.get("inactive_branches") or []
    rows = [
        ["Case", case["case_id"]],
        ["Claim", "limited no-OA/no-limit sensible declared outputs"],
        ["Selected branch", raw.get("selected_purchased_air_branch", "n/a")],
        ["Declared branch", raw.get("declared_ideal_loads_branch", "n/a")],
        ["Active feature flags", list_label(active_flags, 8)],
        ["Inactive branches", list_label(inactive, 8)],
        ["Zone demand source", raw.get("zone_demand_fixture_mode", "n/a")],
        ["Dispatch validation", raw.get("zone_equipment_dispatch_validation", "n/a")],
        ["Dispatch path", raw.get("zone_equipment_dispatch_path", "n/a")],
    ]
    return table(["Field", "Value"], rows, "IdealLoadsAirSystem No-OA comparison setup.", [1.55, 5.55])


def build_ported_algorithm_table(evidence: dict[str, Any]) -> Table:
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    ideal = ideal_loads_case(evidence) or {}
    rows = [
        [
            "Both",
            "Time/weather/schedule ingestion",
            "ported for declared fields",
            "Schedule value and outdoor dry-bulb gates pass exactly.",
            "Controls 1Zone weather boundary and IdealLoads availability/setpoint schedules.",
        ],
        [
            "1Zone",
            "Zone air heat balance scaffold",
            "limited + diagnostic",
            dynamic_focus_metric_label(dynamic, "ZONE ONE", "Zone Mean Air Temperature"),
            "Directly affects Zone Mean Air Temperature and comfort/load-facing outputs.",
        ],
        [
            "1Zone",
            "Opaque surface state and CTF plumbing",
            "diagnostic candidate",
            dynamic_focus_metric_label(dynamic, "ZN001:FLR001", "Surface Heat Storage Rate"),
            "Controls surface temperatures, storage, conduction, and the zone convection term.",
        ],
        [
            "IdealLoads",
            "PurchasedAir no-OA sensible branch",
            "promoted for this fixture",
            f"{ideal.get('series_count', 'n/a')} conformance rows; max abs {compact_number_label(ideal.get('max_abs_delta_c'))}.",
            "Controls total/sensible/supply-air heating and cooling rates when OA, limits, and humidity branches are inactive.",
        ],
        [
            "IdealLoads",
            "Zone equipment dispatch and typed IDs",
            "validated for single equipment",
            (ideal.get("raw_summary") or {}).get("zone_equipment_dispatch_validation", "n/a"),
            "Keeps report path close to EnergyPlus ZoneEquipmentManager/PurchasedAirManager order.",
        ],
        [
            "IdealLoads",
            "Supply node update and ResultStore output handles",
            "promoted for selected rows",
            "System Node Temperature and Mass Flow Rate rows pass in no-OA fixture.",
            "Directly affects common HVAC node outputs requested by users and downstream loop coupling.",
        ],
    ]
    return table(
        ["System", "Algorithm", "Porting state", "Current evidence", "Why it matters"],
        rows,
        "Algorithms already ported or source-mapped enough to support the two comparison systems.",
        [0.72, 1.45, 0.95, 1.75, 2.15],
    )


def build_unported_impact_table(_evidence: dict[str, Any]) -> Table:
    rows = [
        [
            "1Zone",
            "Massive-wall CTF history closure",
            "not promoted",
            "Surface Heat Storage Rate, inside/outside conduction, Zone Opaque Surface Inside Faces Conduction Rate",
            "Can produce MAT drift even when no-mass gates are exact; errors accumulate through thermal storage/history.",
        ],
        [
            "1Zone",
            "Surface convection live coupling",
            "diagnostic",
            "Zone Air Heat Balance Surface Convection Rate, face temperatures, MAT",
            "Affects the dominant zone-air exchange term; mismatch can move comfort temperature and surface heat flows.",
        ],
        [
            "1Zone",
            "Exterior radiation/solar/weather boundary closure",
            "diagnostic",
            "Outside face radiation, solar gain, outside convection, roof/wall temperatures",
            "Changes surface boundary heat flow and storage, especially for roof and exterior walls.",
        ],
        [
            "1Zone",
            "Full warmup/convergence parity",
            "diagnostic",
            "All dynamic run-period outputs",
            "Different initial histories can look like algorithm error in early timesteps and bias storage terms.",
        ],
        [
            "IdealLoads",
            "Outdoor air branch beyond this No-OA fixture",
            "out of this case",
            "OA mass flow, OA sensible/latent/total rates, mixed air node state, economizer, heat recovery",
            "No-OA removes a major source of load and psychrometric branching; OA-active behavior must be judged separately.",
        ],
        [
            "IdealLoads",
            "Broad humidity and moisture-demand coupling",
            "partly separate candidate evidence",
            "Latent heating/cooling rates, supply humidity ratio, zone-air humidity, humidistat outputs",
            "Can change sensible/total split and node humidity even when dry no-OA sensible rates pass.",
        ],
        [
            "IdealLoads",
            "Meters, fuel efficiency, adaptive system timestep, loop integration",
            "outside this no-OA case",
            "Fuel energy, facility meters, AirLoopHVAC/PlantLoop-facing outputs",
            "User billing/energy outputs and loop interactions need separate gates before broad HVAC compatibility is claimed.",
        ],
    ]
    return table(
        ["System", "Gap", "State", "Affected user outputs", "Qualitative impact"],
        rows,
        "Algorithms and coupling paths not yet promoted, with expected impact on the two systems.",
        [0.65, 1.2, 0.82, 2.0, 2.45],
    )


def find_case_series(case: dict[str, Any] | None, variable: str, key: str | None = None) -> dict[str, Any] | None:
    if case is None:
        return None
    for row in case.get("all_series", []):
        if row.get("variable") == variable and (key is None or row.get("key") == key):
            return row
    return None


def target_metric_label(row: dict[str, Any] | None) -> str:
    if row is None:
        return "not in current case"
    return f"{status_label(row.get('status'))}; max {compact_number_label(row.get('max_abs_delta_c'))}; RMSE {compact_number_label(row.get('rmse_delta_c'))}"


def build_user_output_target_table(evidence: dict[str, Any]) -> Table:
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    ideal = ideal_loads_case(evidence)
    dynamic_series = dynamic.get("series", []) if dynamic.get("available") else []
    rows: list[list[Any]] = []
    dynamic_targets = [
        ("1Zone", "Zone Mean Air Temperature", "comfort/control state", "Match hourly MAT after storage, convection, and weather coupling close."),
        ("1Zone", "Surface Inside Face Temperature", "surface comfort/radiant proxy", "Match mass-surface history and inside boundary solve."),
        ("1Zone", "Surface Heat Storage Rate", "thermal mass", "Close CTF storage/history before promoting massive-wall dynamic conformance."),
        ("1Zone", "Zone Air Heat Balance Surface Convection Rate", "zone heat balance", "Close surface convection coupling because it feeds MAT directly."),
        ("1Zone", "Surface Outside Face Solar Radiation Heat Gain Rate", "solar boundary", "Keep solar/radiation diagnostics visible so MAT parity is not accidental."),
    ]
    for system, variable, output_class, target in dynamic_targets:
        row = next((item for item in dynamic_series if item.get("variable") == variable), None)
        rows.append([system, variable_label(variable), output_class, target_metric_label(row), target])

    ideal_targets = [
        ("Zone Ideal Loads Zone Total Heating Rate", "load rate", "Promote across OA, limits, humidity, and dispatch combinations, not only no-OA sensible."),
        ("Zone Ideal Loads Zone Total Cooling Rate", "load rate", "Promote across OA, economizer, heat recovery, and latent branches."),
        ("Zone Ideal Loads Supply Air Total Heating Rate", "supply rate", "Keep source-order ReportPurchasedAir rate semantics and supply-node update aligned."),
        ("System Node Temperature", "node output", "Preserve supply-node state when OA/mixed-air and humidity paths activate."),
        ("System Node Mass Flow Rate", "node output", "Preserve flow limiting and standard-density assumptions across finite-limit/OA branches."),
        ("Zone Ideal Loads Supply Air Total Heating Energy", "energy output", "Rate-to-timestep energy must stay aligned with EnergyPlus reporting semantics."),
        ("DistrictHeatingWater:Facility", "meter", "Facility meters need separate aggregation and frequency gates before broad energy claim."),
        ("Zone Ideal Loads Outdoor Air Mass Flow Rate", "outdoor air", "No-OA fixture deliberately excludes this; OA-active cases change load and node state materially."),
    ]
    for variable, output_class, target in ideal_targets:
        row = find_case_series(ideal, variable)
        rows.append(["IdealLoads", variable_label(variable), output_class, target_metric_label(row), target])

    return table(
        ["System", "User-facing output", "Class", "Current evidence", "What still has to be proven"],
        rows,
        "Conformance targets organized around outputs users are likely to request, not just rows already matched.",
        [0.72, 1.75, 0.8, 1.25, 2.55],
    )


def build_ideal_loads_boundary_table(evidence: dict[str, Any]) -> Table:
    case = ideal_loads_case(evidence)
    if case is None:
        return table(["Boundary", "State", "Impact"], [["IdealLoads", "missing", "missing"]], "IdealLoads boundary.", [1.4, 1.2, 4.4])
    rows = [
        [
            "No outdoor air",
            "active restriction",
            "Removes OA mass-flow, mixed-air, economizer, heat-recovery, and OA latent/sensible report-rate branches from this proof.",
        ],
        [
            "No flow/capacity limit",
            "active restriction",
            "The no-OA case proves the unconstrained branch; finite-limit branches need their own gates because mass flow and supply temperature can change.",
        ],
        [
            "Sensible only",
            "active restriction",
            "Latent and humidity-control outputs are not proved by this case unless explicitly included by a separate humidity branch gate.",
        ],
        [
            "Oracle demand input",
            (case.get("raw_summary") or {}).get("zone_demand_fixture_mode", "n/a"),
            "This isolates PurchasedAir parity; upstream zone heat-balance demand mismatch is classified separately.",
        ],
        [
            "Energy/fuel/meter rows",
            "diagnostic in this no-OA report",
            "Useful as evidence, but broad billing/energy conformance needs dedicated frequency and aggregation gates.",
        ],
    ]
    return table(
        ["Boundary", "State", "Why this changes conformance"],
        rows,
        "Why the IdealLoads No-OA condition is a material comparison boundary.",
        [1.35, 1.35, 4.25],
    )


def build_timing_statistics_table(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    for index, case in enumerate(evidence["cases"], start=1):
        stats = case.get("timing_statistics") or {}
        sample_count = (stats.get("release_gate_wall_seconds") or {}).get("count", 0)
        rows.append(
            [
                f"C{index:02d}",
                case_label(case["case_id"]),
                sample_count,
                mean_stat_label(stats, "rust_compare_report_wall_seconds"),
                mean_stat_label(stats, "energyplus_oracle_wall_seconds"),
                mean_stat_label(stats, "ep_cli_total_wall_seconds"),
                mean_stat_label(stats, "release_gate_wall_seconds"),
                spread_stat_label(stats, "release_gate_wall_seconds"),
            ]
        )
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    if dynamic.get("available"):
        stats = dynamic.get("timing_statistics") or {}
        sample_count = (stats.get("release_gate_wall_seconds") or {}).get("count", 0)
        rows.append(
            [
                "D01",
                "Official 1Zone dynamic",
                sample_count,
                mean_stat_label(stats, "rust_report_residual_seconds"),
                "n/a",
                "n/a",
                mean_stat_label(stats, "release_gate_wall_seconds"),
                spread_stat_label(stats, "release_gate_wall_seconds"),
            ]
        )
    return table(
        ["ID", "Case", "N", "Rust/report mean", "E+ oracle mean", "ep_cli mean", "Gate mean", "Gate min-max"],
        rows,
        "Repeated timing statistics. Promoted gates use internal ep_cli timing; dynamic residual is gate wall minus EnergyPlus self elapsed.",
        [0.42, 1.8, 0.35, 0.82, 0.82, 0.75, 0.72, 1.05],
    )


def build_timing_sample_table(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    for index, case in enumerate(evidence["cases"], start=1):
        for sample in case.get("timing_samples", []):
            rows.append(
                [
                    f"C{index:02d}",
                    sample.get("run"),
                    case_label(case["case_id"]),
                    number_label(sample.get("rust_compare_report_wall_seconds"), 3, "s"),
                    number_label(sample.get("energyplus_oracle_wall_seconds"), 3, "s"),
                    number_label(sample.get("ep_cli_total_wall_seconds"), 3, "s"),
                    number_label(sample.get("release_gate_wall_seconds"), 3, "s"),
                    number_label(sample.get("release_gate_overhead_seconds"), 3, "s"),
                ]
            )
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    if dynamic.get("available"):
        for sample in dynamic.get("timing_samples", []):
            rows.append(
                [
                    "D01",
                    sample.get("run"),
                    "Official 1Zone dynamic",
                    number_label(sample.get("rust_report_residual_seconds"), 3, "s"),
                    "self " + number_label(sample.get("energyplus_reported_elapsed_seconds"), 3, "s"),
                    "n/a",
                    number_label(sample.get("release_gate_wall_seconds"), 3, "s"),
                    "n/a",
                ]
            )
    return table(
        ["ID", "Run", "Case", "Rust/report", "E+ oracle/self", "ep_cli", "Gate", "Overhead"],
        rows,
        "Raw timing samples used for repeated timing statistics.",
        [0.42, 0.35, 1.75, 0.75, 0.75, 0.65, 0.65, 0.7],
    )


def build_series_detail(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    series_index = 1
    for case in evidence["cases"]:
        for series in case["series"]:
            rows.append(
                [
                    f"S{series_index:02d}",
                    case_label(case["case_id"]),
                    key_label(series["key"]),
                    variable_label(series["variable"]),
                    class_label(series["class"]),
                    series["samples"],
                    compact_number_label(series["max_abs_delta_c"]),
                    compact_number_label(series["rmse_delta_c"]),
                    compact_number_label(series["tolerance_max_abs_c"]),
                    status_label(series["status"]),
                ]
            )
            series_index += 1
    return table(
        [
            "ID",
            "Case",
            "Key",
            "Output",
            "Class",
            "N",
            "Max abs",
            "RMSE",
            "Tol",
            "OK",
        ],
        rows,
        "Per-series numerical evidence.",
        [0.38, 1.2, 0.75, 1.45, 0.6, 0.35, 0.65, 0.65, 0.65, 0.35],
    )


def build_metric_table(evidence: dict[str, Any]) -> Table:
    aggregate = evidence["aggregate"]
    timing = evidence["timing"]
    rows = [
        ["Cases", aggregate["case_count"]],
        ["Series", aggregate["series_count"]],
        ["Max abs delta", number_label(aggregate["max_abs_delta_c"], 12)],
        ["Max RMSE", number_label(aggregate["rmse_delta_c"], 12)],
        ["Gate status", aggregate["status"]],
        ["Rust compare/report wall", number_label(timing["rust_compare_report_wall_seconds"], 3, "s")],
        ["EnergyPlus oracle wall", number_label(timing["energyplus_oracle_wall_seconds"], 3, "s")],
    ]
    return table(["Metric", "Value"], rows, "Release evidence summary metrics.", [2.6, 2.2])


def build_artifact_paths(evidence: dict[str, Any]) -> Table:
    labels = {
        "html": "HTML evidence",
        "pdf": "PDF evidence",
        "json": "JSON evidence",
    }
    rows = [[labels.get(key, key), path] for key, path in evidence["artifacts"].items()]
    return table(["Artifact", "Path"], rows, "Generated release evidence artifacts.", [1.5, 5.6])


def build_document(evidence: dict[str, Any], charts: dict[str, Any]) -> Document:
    version = evidence["version"]
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    settings = DocumentSettings(
        metadata_author="rusted-energyplus",
        subtitle="1Zone Uncontrolled and IdealLoadsAirSystem comparison boundary",
        cover_page=True,
        page_margins=PageMargins(0.55, 0.55, 0.55, 0.55, unit="in"),
        theme=Theme(
            body_font_name="Segoe UI",
            monospace_font_name="Consolas",
            body_font_size=9.25,
            heading_sizes=(20, 16, 13, 11),
            table_alignment="center",
            figure_alignment="center",
            show_page_numbers=True,
            page_number_alignment="center",
        ),
    )
    return Document(
        f"eplus-rs {version} Conformance Gap Evidence",
        TableOfContents("Table of Contents", max_level=2),
        Chapter(
            "Comparison Scope",
            Box(
                Paragraph(
                    "This report is organized around two system-level comparisons: the official 1ZoneUncontrolled "
                    "dynamic heat-balance model and a ZoneHVAC:IdealLoadsAirSystem no-outdoor-air sensible branch. "
                    "Promoted small gates are retained as regression evidence, but the main tables emphasize outputs "
                    "that still have to be matched before broader user-facing conformance can be claimed."
                ),
                title="Claim Boundary",
                border_color="#2f6f9f",
                background_color="#f4f8fb",
                padding=0.12,
            ),
            Paragraph(
                "Generated UTC: ",
                code(evidence["generated_at_utc"]),
                ". EnergyPlus oracle: ",
                code(evidence["oracle_version"]),
                ". Promoted aggregate status: ",
                code(evidence["aggregate"]["status"]),
                ".",
            ),
            build_two_case_scope_table(evidence),
            build_metric_table(evidence),
        ),
        Chapter(
            "Outputs To Match",
            Paragraph(
                "The table below is intentionally target-oriented. A zero in a promoted subset does not by itself "
                "settle broad conformance; each row calls out the user-facing output family and the remaining proof "
                "needed for the two selected systems."
            ),
            build_user_output_target_table(evidence),
        ),
        Chapter(
            "1Zone Uncontrolled",
            Paragraph(
                "The active model is the official EnergyPlus 1ZoneUncontrolled ExampleFile with hourly outputs. The "
                "loaded artifact may be a diagnostic or conformance-candidate lane; its current status is shown in "
                "the setup table. Even when that candidate passes, broad heat-balance compatibility remains limited "
                "to the listed output families until additional constructions, schedules, and boundary conditions are "
                "covered. The focus set includes comfort-facing MAT plus storage, convection, solar, radiation, and "
                "conduction rows so a good MAT match cannot hide the source of remaining heat-balance error."
            ),
            build_dynamic_setup_table(dynamic),
            build_dynamic_error_distribution_table(dynamic),
            build_dynamic_focus_table(dynamic),
            Figure(
                charts["dynamic_bottlenecks"],
                caption="Largest 1Zone dynamic diagnostic bottlenecks by RMSE.",
                width=6.4,
                placement="H",
            ),
            build_dynamic_bottleneck_table(dynamic),
            build_dynamic_source_split_table(dynamic),
        ),
        Chapter(
            "IdealLoadsAirSystem",
            Paragraph(
                "The IdealLoadsAirSystem comparison here is deliberately a No-OA, no-limit, sensible branch. No-OA is "
                "a material boundary: enabling outdoor air activates additional mass-flow, mixed-air, economizer, heat "
                "recovery, and latent report paths that can change the conformance result even when the no-OA sensible "
                "branch is exact."
            ),
            build_ideal_loads_setup_table(evidence),
            build_ideal_loads_boundary_table(evidence),
        ),
        PageBreak(),
        Chapter(
            "Ported Algorithms",
            Paragraph(
                "Porting state is split from numeric accuracy. Rows below summarize the implemented or source-mapped "
                "algorithms that currently support the two system comparisons, along with why each algorithm matters "
                "for user-visible outputs."
            ),
            build_ported_algorithm_table(evidence),
            build_algorithm_porting_table(evidence),
            build_porting_table(evidence),
        ),
        Chapter(
            "Not Yet Ported",
            Paragraph(
                "The gaps below are the places most likely to explain remaining mismatch or to invalidate a narrow "
                "claim when the test case is expanded. The qualitative impact column states how each missing or "
                "diagnostic-only path can move the selected systems' outputs."
            ),
            build_unported_impact_table(evidence),
        ),
        Chapter(
            "Execution Time",
            Paragraph(
                "Timing rows distinguish exactly what is being measured. EnergyPlus oracle wall-clock is the "
                "energyplus.exe run that produces oracle files. Rust compare/report wall-clock starts after oracle "
                "files exist and includes Rust model/oracle loading, algorithm evaluation, comparison, and artifact "
                "writing. ep_cli total adds baseline staging, EnergyPlus oracle execution, IDF conversion, and manifest "
                "writes. Release gate wall-clock also includes cargo, PowerShell, and assertion overhead. Repeated "
                "samples are reported separately from the last-run phase breakdown."
            ),
            build_dynamic_timing_table(dynamic),
            Figure(
                charts["timing"],
                caption="Same-scope Rust evidence production and EnergyPlus oracle production wall-clock.",
                width=6.4,
                placement="H",
            ),
            build_timing_statistics_table(evidence),
            build_timing_sample_table(evidence),
            build_timing_values(evidence),
            build_phase_timing_values(evidence),
        ),
        Chapter(
            "Regression Gates",
            Paragraph(
                "These promoted gates are not the definition of broad conformance. They are regression locks for "
                "already matched declared variables, kept here so work on the two larger systems does not break the "
                "known exact subsets."
            ),
            Figure(
                charts["accuracy"],
                caption="Promoted regression-gate accuracy for declared variables only.",
                width=6.4,
                placement="H",
            ),
            build_case_matrix(evidence),
            build_series_detail(evidence),
        ),
        Chapter(
            "Next Work",
            Paragraph(
                "The next numerical target is to close the 1Zone dynamic bottleneck around mass-floor CTF "
                "storage/history, surface convection, and exterior boundary exchange. The next IdealLoads target is "
                "to keep no-OA exact while promoting OA, humidity, economizer, heat-recovery, meter, and loop-coupling "
                "rows only when each branch has its own source-order gate."
            ),
            build_artifact_paths(evidence),
        ),
        settings=settings,
    )


def write_outputs(repo_root: Path, version: str, evidence: dict[str, Any]) -> dict[str, Path]:
    evidence_root = repo_root / ".runtime" / "release-evidence" / f"v{version}"
    evidence_root.mkdir(parents=True, exist_ok=True)
    charts = create_charts(evidence)
    try:
        json_path = evidence_root / "numeric-conformance-evidence.json"
        html_path = evidence_root / "numeric-conformance-evidence.html"
        pdf_path = evidence_root / "numeric-conformance-evidence.pdf"
        document = build_document(evidence, charts)

        json_path.write_text(json.dumps(evidence, indent=2), encoding="utf-8")
        document.save_html(html_path)
        document.save_pdf(pdf_path)
    finally:
        for chart in charts.values():
            plt.close(chart)

    return {"json": json_path, "html": html_path, "pdf": pdf_path}


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    evidence = build_evidence(
        repo_root,
        args.version,
        args.skip_gate_run,
        args.run_dynamic_diagnostic,
        args.timing_repeats,
        args.dynamic_timing_repeats,
    )
    outputs = write_outputs(repo_root, args.version, evidence)

    print("Numeric conformance evidence report")
    print(f"  status: {evidence['aggregate']['status']}")
    print(f"  cases: {evidence['aggregate']['case_count']}")
    print(f"  series: {evidence['aggregate']['series_count']}")
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    if dynamic.get("available"):
        print(f"  active_dynamic_status: {dynamic['status']}")
        print(f"  active_dynamic_outputs: {dynamic['outputs']}")
    print(f"  max_abs_delta_c: {number_label(evidence['aggregate']['max_abs_delta_c'], 12)}")
    print(f"  html: {outputs['html']}")
    print(f"  pdf: {outputs['pdf']}")
    print(f"  json: {outputs['json']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
