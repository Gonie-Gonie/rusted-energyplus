from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ORACLE_VERSION = "26.1.0"
CASE_ID = "official_1zone_uncontrolled_dynamic_diagnostic_001"


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
    FocusMetric("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate"),
    FocusMetric("ZONE ONE", "Zone Air Heat Balance Air Energy Storage Rate"),
    FocusMetric("ZN001:FLR001", "Surface Inside Face Conduction Heat Transfer Rate"),
    FocusMetric("ZONE ONE", "Zone Opaque Surface Inside Faces Conduction Rate"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Incident Solar Radiation Rate per Area"),
)


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
        "zone_air_algorithm": summary.get("zone_air_algorithm", "unknown"),
        "ctf_seed_policy": summary.get("ctf_seed", {}).get("policy", "unknown"),
        "surface_iteration_count": summary.get("surface_iteration_count", 1),
        "samples": summary.get("samples"),
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
    return {
        "schema": "rusted-energyplus.dynamic-heat-balance-probe-summary.v2",
        "oracle_version": ORACLE_VERSION,
        "case_id": CASE_ID,
        "lane_count": len(lanes),
        "lanes": lanes,
    }


def fmt_number(value: Any) -> str:
    if isinstance(value, (int, float)):
        return f"{value:.6f}"
    return "none"


def render_markdown(summary: dict[str, Any]) -> str:
    lines = [
        "# Official Dynamic Heat-Balance Probe Summary",
        "",
        f"case_id: {summary['case_id']}",
        f"oracle_version: {summary['oracle_version']}",
        "",
        "| lane | algorithm | CTF seed | surface passes | top output | top RMSE | top max abs | status |",
        "|---|---|---|---:|---|---:|---:|---|",
    ]
    for lane in summary["lanes"]:
        top_output = f"{lane['top_key']} / {lane['top_variable']}"
        lines.append(
            "| {lane} | {algorithm} | {ctf} | {surface_passes} | {top} | {rmse} | {max_abs} | {status} |".format(
                lane=lane["lane"],
                algorithm=lane["zone_air_algorithm"],
                ctf=lane["ctf_seed_policy"],
                surface_passes=lane["surface_iteration_count"],
                top=top_output,
                rmse=fmt_number(lane["top_rmse_delta_c"]),
                max_abs=fmt_number(lane["top_max_abs_delta_c"]),
                status=lane["status"],
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
