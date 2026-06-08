from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ORACLE_VERSION = "26.1.0"
CASE_ID = "official_1zone_uncontrolled_dynamic_diagnostic_001"
EXPECTED_SERIES_COUNT = 31


@dataclass(frozen=True)
class ProbeLane:
    lane: str
    summary_path: Path


@dataclass(frozen=True)
class FocusMetric:
    key: str
    variable: str


LANES = (
    ProbeLane(
        lane="default",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-warmup-min20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-warmup-min20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-surface-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-surface-first",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-surface-first"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-surface-first-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-surface-first-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="analytical",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-analytical"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="analytical-surface-first",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-analytical-surface-first"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="third-order",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-third-order"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="warmup-min20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-warmup-min20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
)

FOCUS_METRICS = (
    FocusMetric("ZONE ONE", "Zone Mean Air Temperature"),
    FocusMetric("ZONE ONE", "Zone Air Heat Balance Internal Convective Heat Gain Rate"),
    FocusMetric("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate"),
    FocusMetric("ZONE ONE", "Zone Air Heat Balance Air Energy Storage Rate"),
    FocusMetric("ZN001:FLR001", "Surface Inside Face Temperature"),
    FocusMetric("ZN001:FLR001", "Surface Outside Face Temperature"),
    FocusMetric("ZN001:FLR001", "Surface Inside Face Conduction Heat Transfer Rate"),
    FocusMetric("ZN001:FLR001", "Surface Outside Face Conduction Heat Transfer Rate"),
    FocusMetric("ZN001:FLR001", "Surface Heat Storage Rate"),
    FocusMetric("ZN001:ROOF001", "Surface Inside Face Temperature"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Temperature"),
    FocusMetric("ZONE ONE", "Zone Opaque Surface Inside Faces Conduction Rate"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Incident Solar Radiation Rate per Area"),
)

REFERENCE_LANES = {
    "all-ctf": "default",
    "all-ctf-warmup-min20": "all-ctf",
    "all-ctf-surface-iter3": "all-ctf",
    "all-ctf-analytical-surface-first": "all-ctf",
    "all-ctf-analytical-coupled": "all-ctf-analytical-surface-first",
    "all-ctf-analytical-coupled-iter3": "all-ctf-analytical-coupled",
    "all-ctf-analytical-coupled-previous-inside-iter3": "all-ctf-analytical-coupled-iter3",
    "all-ctf-analytical-surface-first-iter3": "all-ctf-analytical-surface-first",
    "analytical": "default",
    "analytical-surface-first": "default",
    "third-order": "default",
    "warmup-min20": "default",
}

# The rollup is an interpretation aid, not a tolerance gate. Treat tiny probe
# movements as unchanged so warmup/no-op probes do not look more decisive than
# they are.
MOVEMENT_EPSILON = 1.0


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        payload = json.load(handle)
    if not isinstance(payload, dict):
        raise ValueError(f"Expected object JSON at {path}")
    return payload


def series_output_label(series: dict[str, Any]) -> str:
    output = series.get("output", {})
    if not isinstance(output, dict):
        return "none"
    return f"{output.get('key', 'none')} / {output.get('variable', 'none')}"


def find_series(summary: dict[str, Any], metric: FocusMetric) -> dict[str, Any] | None:
    for series in summary.get("series", []):
        if not isinstance(series, dict):
            continue
        output = series.get("output", {})
        if not isinstance(output, dict):
            continue
        if output.get("key") == metric.key and output.get("variable") == metric.variable:
            return series
    return None


def focus_metric_rows(summary: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for metric in FOCUS_METRICS:
        series = find_series(summary, metric)
        if series is None:
            rows.append(
                {
                    "key": metric.key,
                    "variable": metric.variable,
                    "label": f"{metric.key} / {metric.variable}",
                    "status": "missing",
                    "samples": None,
                    "rmse_delta_c": None,
                    "max_abs_delta_c": None,
                    "mean_abs_delta_c": None,
                }
            )
            continue
        rows.append(
            {
                "key": metric.key,
                "variable": metric.variable,
                "label": series_output_label(series),
                "status": series.get("status"),
                "samples": series.get("samples"),
                "rmse_delta_c": series.get("rmse_delta_c"),
                "max_abs_delta_c": series.get("max_abs_delta_c"),
                "mean_abs_delta_c": series.get("mean_abs_delta_c"),
            }
        )
    return rows


def metric_identity(metric: dict[str, Any]) -> tuple[str, str]:
    return (str(metric.get("key", "")), str(metric.get("variable", "")))


def numeric(value: Any) -> float | None:
    if isinstance(value, (int, float)):
        return float(value)
    return None


def annotate_default_focus_deltas(lanes: list[dict[str, Any]]) -> None:
    default_lane = next((lane for lane in lanes if lane.get("lane") == "default"), None)
    if default_lane is None:
        return

    baselines: dict[tuple[str, str], float] = {}
    for metric in default_lane.get("focus_metrics", []):
        rmse = numeric(metric.get("rmse_delta_c"))
        if rmse is not None:
            baselines[metric_identity(metric)] = rmse

    for lane in lanes:
        for metric in lane.get("focus_metrics", []):
            rmse = numeric(metric.get("rmse_delta_c"))
            baseline = baselines.get(metric_identity(metric))
            if lane.get("lane") == "default" or rmse is None or baseline is None:
                metric["rmse_vs_default"] = None
                metric["rmse_ratio_vs_default"] = None
                continue
            metric["rmse_vs_default"] = rmse - baseline
            metric["rmse_ratio_vs_default"] = rmse / baseline if baseline != 0 else None


def empty_focus_movement_rollup(reference_lane: str | None) -> dict[str, Any]:
    return {
        "reference_lane": reference_lane,
        "improved_focus_count": 0,
        "worsened_focus_count": 0,
        "unchanged_focus_count": 0,
        "missing_focus_count": 0,
        "largest_rmse_improvement": None,
        "largest_rmse_regression": None,
    }


def annotate_reference_focus_movements(lanes: list[dict[str, Any]]) -> None:
    lane_by_name = {str(lane.get("lane")): lane for lane in lanes}
    metrics_by_lane: dict[str, dict[tuple[str, str], dict[str, Any]]] = {}
    for lane in lanes:
        lane_name = str(lane.get("lane"))
        metrics_by_lane[lane_name] = {
            metric_identity(metric): metric
            for metric in lane.get("focus_metrics", [])
            if isinstance(metric, dict)
        }

    for lane in lanes:
        lane_name = str(lane.get("lane"))
        reference_lane = REFERENCE_LANES.get(lane_name)
        lane["reference_lane"] = reference_lane
        lane["focus_movement_vs_reference"] = []
        lane["focus_movement_rollup"] = empty_focus_movement_rollup(reference_lane)

        if reference_lane is None or reference_lane not in lane_by_name:
            continue

        reference_metrics = metrics_by_lane.get(reference_lane, {})
        movements: list[dict[str, Any]] = []
        rollup = empty_focus_movement_rollup(reference_lane)
        largest_improvement: dict[str, Any] | None = None
        largest_regression: dict[str, Any] | None = None

        for metric in lane.get("focus_metrics", []):
            if not isinstance(metric, dict):
                continue
            reference_metric = reference_metrics.get(metric_identity(metric))
            rmse = numeric(metric.get("rmse_delta_c"))
            reference_rmse = (
                numeric(reference_metric.get("rmse_delta_c"))
                if isinstance(reference_metric, dict)
                else None
            )
            movement = {
                "key": metric.get("key"),
                "variable": metric.get("variable"),
                "label": metric.get("label"),
                "rmse_delta_c": rmse,
                "reference_rmse_delta_c": reference_rmse,
                "rmse_vs_reference": None,
                "rmse_ratio_vs_reference": None,
                "direction": "missing",
            }
            if rmse is None or reference_rmse is None:
                rollup["missing_focus_count"] += 1
                movements.append(movement)
                continue

            delta = rmse - reference_rmse
            movement["rmse_vs_reference"] = delta
            movement["rmse_ratio_vs_reference"] = (
                rmse / reference_rmse if reference_rmse != 0 else None
            )
            if delta < -MOVEMENT_EPSILON:
                movement["direction"] = "improved"
                rollup["improved_focus_count"] += 1
                if (
                    largest_improvement is None
                    or delta < numeric(largest_improvement.get("rmse_vs_reference"))
                ):
                    largest_improvement = movement
            elif delta > MOVEMENT_EPSILON:
                movement["direction"] = "worsened"
                rollup["worsened_focus_count"] += 1
                if (
                    largest_regression is None
                    or delta > numeric(largest_regression.get("rmse_vs_reference"))
                ):
                    largest_regression = movement
            else:
                movement["direction"] = "unchanged"
                rollup["unchanged_focus_count"] += 1
            movements.append(movement)

        rollup["largest_rmse_improvement"] = largest_improvement
        rollup["largest_rmse_regression"] = largest_regression
        lane["focus_movement_vs_reference"] = movements
        lane["focus_movement_rollup"] = rollup


def best_focus_metric_rows(lanes: list[dict[str, Any]]) -> list[dict[str, Any]]:
    default_metrics = {
        metric_identity(metric): metric
        for lane in lanes
        if lane.get("lane") == "default"
        for metric in lane.get("focus_metrics", [])
        if isinstance(metric, dict)
    }
    rows = []
    for metric in FOCUS_METRICS:
        identity = (metric.key, metric.variable)
        candidates = []
        for lane in lanes:
            for focus_metric in lane.get("focus_metrics", []):
                if not isinstance(focus_metric, dict):
                    continue
                if metric_identity(focus_metric) != identity:
                    continue
                rmse = numeric(focus_metric.get("rmse_delta_c"))
                if rmse is not None:
                    candidates.append((rmse, lane, focus_metric))
        if not candidates:
            rows.append(
                {
                    "key": metric.key,
                    "variable": metric.variable,
                    "label": f"{metric.key} / {metric.variable}",
                    "best_lane": None,
                    "best_rmse_delta_c": None,
                    "default_rmse_delta_c": None,
                    "rmse_vs_default": None,
                }
            )
            continue

        best_rmse, best_lane, best_metric = min(candidates, key=lambda item: item[0])
        default_rmse = numeric(default_metrics.get(identity, {}).get("rmse_delta_c"))
        rows.append(
            {
                "key": metric.key,
                "variable": metric.variable,
                "label": best_metric.get("label", f"{metric.key} / {metric.variable}"),
                "best_lane": best_lane.get("lane"),
                "best_rmse_delta_c": best_rmse,
                "default_rmse_delta_c": default_rmse,
                "rmse_vs_default": (
                    best_rmse - default_rmse if default_rmse is not None else None
                ),
            }
        )
    return rows


def lane_row(repo_root: Path, lane: ProbeLane) -> dict[str, Any] | None:
    summary_path = repo_root / lane.summary_path
    if not summary_path.exists():
        return None
    summary = load_json(summary_path)
    bottlenecks = summary.get("bottlenecks", [])
    top = bottlenecks[0] if bottlenecks else {}
    output = top.get("output", {}) if isinstance(top, dict) else {}
    return {
        "lane": lane.lane,
        "path": str(lane.summary_path).replace("\\", "/"),
        "status": summary.get("status"),
        "artifact_status": artifact_status(summary.get("series_count")),
        "zone_air_algorithm": summary.get("zone_air_algorithm", "unknown"),
        "ctf_seed_policy": summary.get("ctf_seed", {}).get("policy", "unknown"),
        "surface_iteration_count": summary.get("surface_iteration_count", 1),
        "samples": summary.get("samples"),
        "series_count": summary.get("series_count"),
        "top_key": output.get("key", "none"),
        "top_variable": output.get("variable", "none"),
        "top_rmse_delta_c": top.get("rmse_delta_c"),
        "top_max_abs_delta_c": top.get("max_abs_delta_c"),
        "max_abs_delta_c": summary.get("max_abs_delta_c"),
        "rmse_delta_c": summary.get("rmse_delta_c"),
        "focus_metrics": focus_metric_rows(summary),
    }


def build_summary(repo_root: Path) -> dict[str, Any]:
    lanes = [row for lane in LANES if (row := lane_row(repo_root, lane)) is not None]
    annotate_default_focus_deltas(lanes)
    annotate_reference_focus_movements(lanes)
    return {
        "schema": "rusted-energyplus.dynamic-heat-balance-probe-summary.v5",
        "oracle_version": ORACLE_VERSION,
        "case_id": CASE_ID,
        "expected_series_count": EXPECTED_SERIES_COUNT,
        "lane_count": len(lanes),
        "lanes": lanes,
        "best_focus_metrics": best_focus_metric_rows(lanes),
    }


def artifact_status(series_count: Any) -> str:
    if not isinstance(series_count, int):
        return "missing-series-count"
    if series_count != EXPECTED_SERIES_COUNT:
        return f"stale-series-count-{series_count}"
    return "current"


def fmt_number(value: Any) -> str:
    if isinstance(value, (int, float)):
        return f"{value:.6f}"
    return "none"


def fmt_signed_number(value: Any) -> str:
    if isinstance(value, (int, float)):
        return f"{value:+.6f}"
    return "none"


def fmt_movement(metric: Any) -> str:
    if not isinstance(metric, dict):
        return "none"
    return "{label} ({delta})".format(
        label=metric.get("label", "none"),
        delta=fmt_signed_number(metric.get("rmse_vs_reference")),
    )


def render_markdown(summary: dict[str, Any]) -> str:
    lines = [
        "# Official Dynamic Heat-Balance Probe Summary",
        "",
        f"case_id: {summary['case_id']}",
        f"oracle_version: {summary['oracle_version']}",
        f"expected_series_count: {summary['expected_series_count']}",
        "",
        "| lane | algorithm | CTF seed | surface passes | series | artifact | top output | top RMSE | top max abs | status |",
        "|---|---|---|---:|---:|---|---|---:|---:|---|",
    ]
    for lane in summary["lanes"]:
        top_output = f"{lane['top_key']} / {lane['top_variable']}"
        lines.append(
            "| {lane} | {algorithm} | {ctf} | {surface_passes} | {series_count} | {artifact_status} | {top} | {rmse} | {max_abs} | {status} |".format(
                lane=lane["lane"],
                algorithm=lane["zone_air_algorithm"],
                ctf=lane["ctf_seed_policy"],
                surface_passes=lane["surface_iteration_count"],
                series_count=lane.get("series_count") or "none",
                artifact_status=lane.get("artifact_status", "unknown"),
                top=top_output,
                rmse=fmt_number(lane["top_rmse_delta_c"]),
                max_abs=fmt_number(lane["top_max_abs_delta_c"]),
                status=lane["status"],
            )
        )
    lines.extend(
        [
            "",
            "## Probe Interpretation",
            "",
            "RMSE movement is measured against each probe lane's nearest reference lane.",
            "",
            "| lane | reference | improved | worsened | unchanged | largest improvement | largest regression |",
            "|---|---|---:|---:|---:|---|---|",
        ]
    )
    for lane in summary["lanes"]:
        rollup = lane.get("focus_movement_rollup", {})
        if not isinstance(rollup, dict):
            rollup = {}
        lines.append(
            "| {lane} | {reference} | {improved} | {worsened} | {unchanged} | {largest_improvement} | {largest_regression} |".format(
                lane=lane["lane"],
                reference=rollup.get("reference_lane") or "none",
                improved=rollup.get("improved_focus_count", 0),
                worsened=rollup.get("worsened_focus_count", 0),
                unchanged=rollup.get("unchanged_focus_count", 0),
                largest_improvement=fmt_movement(
                    rollup.get("largest_rmse_improvement")
                ),
                largest_regression=fmt_movement(
                    rollup.get("largest_rmse_regression")
                ),
            )
        )
    lines.extend(
        [
            "",
            "## Best Focus Metrics",
            "",
            "| output | best lane | best RMSE | RMSE vs default | default RMSE |",
            "|---|---|---:|---:|---:|",
        ]
    )
    for metric in summary.get("best_focus_metrics", []):
        if not isinstance(metric, dict):
            continue
        lines.append(
            "| {output} | {lane} | {best_rmse} | {vs_default} | {default_rmse} |".format(
                output=metric.get("label", "none"),
                lane=metric.get("best_lane") or "none",
                best_rmse=fmt_number(metric.get("best_rmse_delta_c")),
                vs_default=fmt_signed_number(metric.get("rmse_vs_default")),
                default_rmse=fmt_number(metric.get("default_rmse_delta_c")),
            )
        )
    lines.extend(
        [
            "",
            "## Focus Metrics",
            "",
            "| lane | output | RMSE | RMSE vs default | ratio | max abs | mean abs | status |",
            "|---|---|---:|---:|---:|---:|---:|---|",
        ]
    )
    for lane in summary["lanes"]:
        for metric in lane.get("focus_metrics", []):
            lines.append(
                "| {lane} | {output} | {rmse} | {vs_default} | {ratio} | {max_abs} | {mean_abs} | {status} |".format(
                    lane=lane["lane"],
                    output=metric["label"],
                    rmse=fmt_number(metric["rmse_delta_c"]),
                    vs_default=fmt_number(metric.get("rmse_vs_default")),
                    ratio=fmt_number(metric.get("rmse_ratio_vs_default")),
                    max_abs=fmt_number(metric["max_abs_delta_c"]),
                    mean_abs=fmt_number(metric["mean_abs_delta_c"]),
                    status=metric["status"],
                )
            )
    lines.append("")
    return "\n".join(lines)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Summarize official dynamic heat-balance diagnostic probe lanes."
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path.cwd(),
        help="Repository root containing .runtime artifacts.",
    )
    parser.add_argument("--json-output", type=Path, default=None)
    parser.add_argument("--markdown-output", type=Path, default=None)
    return parser.parse_args()


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    summary = build_summary(repo_root)
    if not summary["lanes"]:
        print("No official dynamic heat-balance probe summaries found.")
        return 1

    markdown = render_markdown(summary)
    if args.json_output:
        write_text(args.json_output, json.dumps(summary, indent=2) + "\n")
    if args.markdown_output:
        write_text(args.markdown_output, markdown)
    print(markdown)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
