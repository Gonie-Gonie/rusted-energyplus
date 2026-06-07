from __future__ import annotations

import argparse
import json
import re
import subprocess
import time
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
    "Only declared v0.8/v0.9 no-mass heat-balance, v0.22 time/weather/schedule, "
    "and v0.26 internal convective gain numerical conformance variables are promoted."
)


@dataclass(frozen=True)
class CaseSpec:
    milestone: str
    command: str
    summary_path: str
    oracle_end_path: str
    oracle_err_path: str


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
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build release numerical conformance evidence.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.26.0")
    parser.add_argument("--skip-gate-run", action="store_true")
    return parser.parse_args()


def number_label(value: float | int | None, digits: int = 6, suffix: str = "") -> str:
    if value is None:
        return "n/a"
    return f"{float(value):.{digits}f}{suffix}"


def percent_label(numerator: float | None, denominator: float | None, digits: int = 3) -> str:
    if numerator is None or denominator in (None, 0):
        return "n/a"
    return f"{(float(numerator) / float(denominator)) * 100.0:.{digits}f}%"


def repo_path(repo_root: Path, relative: str) -> Path:
    return repo_root / Path(relative.replace("\\", "/"))


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


def promoted_series(summary: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for series in summary.get("series", []):
        if "output" in series:
            output = series["output"]
            tolerance = tolerance_for_class(summary, output["class"])
            first_delta = series.get("first_delta_sample") or {}
            max_delta = series.get("max_delta_sample") or {}
            rows.append(
                {
                    "key": output.get("key"),
                    "variable": output.get("variable"),
                    "class": output.get("class"),
                    "frequency": output.get("frequency"),
                    "source": output.get("source"),
                    "level": "conformance",
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

        if series.get("level") != "conformance":
            continue
        rows.append(
            {
                "key": series.get("key"),
                "variable": series.get("variable"),
                "class": series.get("class"),
                "frequency": series.get("frequency"),
                "source": series.get("source"),
                "level": series.get("level"),
                "samples": int(series.get("compared_samples", series.get("observed_samples", 0))),
                "status": series.get("status"),
                "max_abs_delta_c": float(series.get("max_abs_delta", 0.0)),
                "mean_abs_delta_c": 0.0,
                "rmse_delta_c": float(series.get("rmse_delta", 0.0)),
                "max_rel_delta": float(series.get("max_rel_delta", 0.0)),
                "tolerance_max_abs_c": float(series.get("max_abs_tolerance", 0.0)),
                "tolerance_max_rmse_c": float(series.get("max_rmse_tolerance", 0.0)),
                "first_delta_index": (series.get("first_divergence") or {}).get("index"),
                "max_delta_index": None,
            }
        )
    return rows


def load_case_report(repo_root: Path, spec: CaseSpec, skip_gate_run: bool) -> dict[str, Any]:
    gate_elapsed = 0.0 if skip_gate_run else run_dev_command(repo_root, spec.command)
    summary_path = repo_path(repo_root, spec.summary_path)
    if not summary_path.is_file():
        raise FileNotFoundError(f"Missing conformance summary: {summary_path}")

    summary = json.loads(summary_path.read_text(encoding="utf-8"))
    if summary.get("comparison_class") != "conformance" or summary.get("conformance_claim") is not True:
        raise ValueError(f"Summary is not a promoted conformance claim: {summary_path}")
    if summary.get("status") != "pass":
        raise ValueError(f"Conformance summary did not pass: {summary.get('case_id')}")

    series_reports = promoted_series(summary)
    if not series_reports:
        raise ValueError(f"Conformance summary has no promoted conformance series: {summary_path}")

    err = error_summary(repo_path(repo_root, spec.oracle_err_path))
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
        "energyplus_elapsed_seconds": elapsed_seconds(repo_path(repo_root, spec.oracle_end_path)),
        "energyplus_warnings": err["warnings"],
        "energyplus_severes": err["severes"],
        "gate_script": (summary.get("gate") or {}).get("script"),
        "source_summary_json": spec.summary_path.replace("\\", "/"),
        "source_report_md": (summary.get("report_contract") or {}).get("path"),
        "series": series_reports,
    }


def build_evidence(repo_root: Path, version: str, skip_gate_run: bool) -> dict[str, Any]:
    cases = [load_case_report(repo_root, spec, skip_gate_run) for spec in CASE_SPECS]
    all_series = [series for case in cases for series in case["series"]]
    failed_cases = [case for case in cases if case["status"] != "pass"]
    return {
        "schema_version": 1,
        "version": version,
        "oracle_version": ORACLE_VERSION,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "claim_boundary": CLAIM_BOUNDARY,
        "aggregate": {
            "status": "fail" if failed_cases else "pass",
            "case_count": len(cases),
            "series_count": len(all_series),
            "max_abs_delta_c": max((case["max_abs_delta_c"] for case in cases), default=0.0),
            "rmse_delta_c": max((case["rmse_delta_c"] for case in cases), default=0.0),
        },
        "cases": cases,
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

    height = max(2.2, 1.15 + len(rows) * 0.46)
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
    ax.set_yticks(y_values, labels)
    ax.invert_yaxis()
    ax.set_xlim(0, max_value * 1.04)
    ax.set_xlabel(x_label, fontsize=9, color="#5b6775")
    ax.set_title(title, loc="left", fontsize=13, fontweight="bold", color="#17212b", pad=10)
    style_axis(ax)
    ax.legend(loc="lower right", fontsize=8, frameon=False)
    fig.tight_layout(pad=1.0)
    return fig


def create_charts(evidence: dict[str, Any]) -> dict[str, Any]:
    accuracy_rows: list[dict[str, Any]] = []
    series_index = 1
    for case in evidence["cases"]:
        for series in case["series"]:
            accuracy_rows.append(
                {
                    "id": f"S{series_index}",
                    "primary": series["tolerance_max_abs_c"],
                    "secondary": series["max_abs_delta_c"],
                }
            )
            series_index += 1

    timing_rows = [
        {
            "id": f"C{index + 1}",
            "primary": case["gate_elapsed_seconds"],
            "secondary": case["energyplus_elapsed_seconds"] or 0.0,
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
        "Execution Time Evidence",
        timing_rows,
        "Release gate wall-clock seconds",
        "EnergyPlus elapsed seconds",
        "Seconds",
        "#3c6e9f",
        "#c77d1a",
    )
    return {"accuracy": accuracy, "timing": timing}


def table(headers: list[str], rows: list[list[Any]], caption: str) -> Table:
    string_rows = [["" if value is None else str(value) for value in row] for row in rows]
    return Table(
        headers,
        string_rows,
        caption=caption,
        header_background_color="#eef3f7",
        border_color="#d7dde5",
        alternate_row_background_color="#f8fafc",
        repeat_header_rows=True,
        split=True,
    )


def build_case_matrix(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    for index, case in enumerate(evidence["cases"], start=1):
        rows.append(
            [
                f"C{index}",
                case["milestone"],
                case["case_id"],
                case["status"],
                case["series_count"],
                case["samples"],
                case["heat_balance_timesteps"],
                number_label(case["max_abs_delta_c"], 12),
                number_label(case["rmse_delta_c"], 12),
                number_label(case["gate_elapsed_seconds"], 3, "s"),
                "n/a"
                if case["energyplus_elapsed_seconds"] is None
                else number_label(case["energyplus_elapsed_seconds"], 3, "s"),
            ]
        )
    return table(
        [
            "ID",
            "Milestone",
            "Case",
            "Status",
            "Series",
            "Samples",
            "Rust timesteps",
            "Max abs",
            "RMSE",
            "Gate wall",
            "E+ elapsed",
        ],
        rows,
        "Promoted numerical conformance case matrix.",
    )


def build_accuracy_values(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    series_index = 1
    for case in evidence["cases"]:
        for series in case["series"]:
            rows.append(
                [
                    f"S{series_index}",
                    case["milestone"],
                    case["case_id"],
                    series["key"],
                    series["variable"],
                    number_label(series["max_abs_delta_c"], 12),
                    number_label(series["tolerance_max_abs_c"], 12),
                    percent_label(series["max_abs_delta_c"], series["tolerance_max_abs_c"]),
                ]
            )
            series_index += 1
    return table(
        ["ID", "Milestone", "Case", "Key", "Variable", "Observed max abs", "Tolerance", "Utilization"],
        rows,
        "Accuracy values backing the chart.",
    )


def build_timing_values(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    for index, case in enumerate(evidence["cases"], start=1):
        rows.append(
            [
                f"C{index}",
                case["milestone"],
                case["case_id"],
                number_label(case["gate_elapsed_seconds"], 3, "s"),
                "n/a"
                if case["energyplus_elapsed_seconds"] is None
                else number_label(case["energyplus_elapsed_seconds"], 3, "s"),
                case["energyplus_warnings"],
                case["energyplus_severes"],
            ]
        )
    return table(
        ["ID", "Milestone", "Case", "Gate wall", "E+ elapsed", "E+ warnings", "E+ severes"],
        rows,
        "Execution time and EnergyPlus error summary values.",
    )


def build_series_detail(evidence: dict[str, Any]) -> Table:
    rows: list[list[Any]] = []
    series_index = 1
    for case in evidence["cases"]:
        for series in case["series"]:
            rows.append(
                [
                    f"S{series_index}",
                    case["case_id"],
                    series["key"],
                    series["variable"],
                    series["class"],
                    series["samples"],
                    number_label(series["max_abs_delta_c"], 12),
                    number_label(series["rmse_delta_c"], 12),
                    number_label(series["tolerance_max_abs_c"], 12),
                    series["status"],
                ]
            )
            series_index += 1
    return table(
        [
            "ID",
            "Case",
            "Key",
            "Variable",
            "Class",
            "Samples",
            "Max abs",
            "RMSE",
            "Max abs tolerance",
            "Status",
        ],
        rows,
        "Per-series numerical evidence.",
    )


def build_metric_table(evidence: dict[str, Any]) -> Table:
    aggregate = evidence["aggregate"]
    rows = [
        ["Cases", aggregate["case_count"]],
        ["Series", aggregate["series_count"]],
        ["Max abs delta", number_label(aggregate["max_abs_delta_c"], 12)],
        ["Max RMSE", number_label(aggregate["rmse_delta_c"], 12)],
        ["Gate status", aggregate["status"]],
    ]
    return table(["Metric", "Value"], rows, "Release evidence summary metrics.")


def build_document(evidence: dict[str, Any], charts: dict[str, Any]) -> Document:
    version = evidence["version"]
    settings = DocumentSettings(
        metadata_author="rusted-energyplus",
        subtitle="Release numerical conformance evidence",
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
        f"eplus-rs {version} Numeric Conformance Evidence",
        TableOfContents("Table of Contents", max_level=2),
        Chapter(
            "Claim Boundary",
            Box(
                Paragraph(
                    "This PDF covers only promoted numerical conformance cases. It does not claim HVAC, node, "
                    "plant, meter, fenestration, solar-radiation, warmup, sizing, or general ExampleFiles compatibility."
                ),
                title="Release Scope",
                border_color="#2f6f9f",
                background_color="#f4f8fb",
                padding=0.12,
            ),
            Paragraph(
                "The public numerical conformance claim is limited to the listed cases, variables, tolerance "
                "policies, and blocking gates. Smoke, diagnostic, typed graph, and baseline-only artifacts remain "
                "outside this report unless explicitly promoted."
            ),
        ),
        Chapter(
            "Executive Summary",
            Paragraph(
                "Generated UTC: ",
                code(evidence["generated_at_utc"]),
                ". EnergyPlus oracle: ",
                code(evidence["oracle_version"]),
                ". Report schema: ",
                code(str(evidence["schema_version"])),
                ".",
            ),
            build_metric_table(evidence),
        ),
        Chapter("Case Matrix", build_case_matrix(evidence)),
        Chapter(
            "Accuracy Evidence",
            Paragraph(
                "The figure uses compact row IDs. Numeric values are reported in the table below the figure so "
                "labels remain stable in PDF output as this report grows."
            ),
            Figure(charts["accuracy"], caption="Accuracy against declared tolerance.", width=6.8),
            build_accuracy_values(evidence),
        ),
        Chapter(
            "Execution Time Evidence",
            Paragraph(
                "Release gate wall-clock covers oracle generation, Rust comparison, and artifact writing. "
                "EnergyPlus elapsed time is read from eplusout.end and is not a portable performance benchmark."
            ),
            Figure(charts["timing"], caption="Release gate wall-clock and EnergyPlus elapsed time.", width=6.8),
            build_timing_values(evidence),
        ),
        PageBreak(),
        Chapter("Series Detail", build_series_detail(evidence)),
        Chapter(
            "Evidence Policy and Future Growth",
            Paragraph(
                "Future numerical conformance additions should enter this PDF only after they have a manifest, "
                "requested variables, tolerance policy, Rust result artifact, markdown/JSON report, and blocking gate."
            ),
            table(
                ["Rule", "Reason"],
                [
                    ["Keep parser and intake checks out", "They are development hygiene unless tied to a promoted numerical result."],
                    ["Summarize exploratory experiments", "Low-level rows should retire once a higher-level case supersedes them."],
                    ["Record divergence and utilization", "Every promoted output needs readable accuracy and tolerance evidence."],
                    ["Preserve explicit non-claims", "HVAC, node, plant, fenestration, solar, warmup, sizing, and meters need their own gates."],
                ],
                "Rules for adding future release evidence.",
            ),
        ),
        Chapter(
            "Artifact Paths",
            table(
                ["Artifact", "Path"],
                [
                    ["HTML evidence", evidence["artifacts"]["html"]],
                    ["PDF evidence", evidence["artifacts"]["pdf"]],
                    ["JSON evidence", evidence["artifacts"]["json"]],
                ],
                "Generated release evidence artifacts.",
            ),
        ),
        settings=settings,
    )


def write_outputs(repo_root: Path, version: str, evidence: dict[str, Any]) -> dict[str, Path]:
    evidence_root = repo_root / ".runtime" / "release-evidence" / f"v{version}"
    evidence_root.mkdir(parents=True, exist_ok=True)
    charts = create_charts(evidence)
    try:
        document = build_document(evidence, charts)

        json_path = evidence_root / "numeric-conformance-evidence.json"
        html_path = evidence_root / "numeric-conformance-evidence.html"
        pdf_path = evidence_root / "numeric-conformance-evidence.pdf"

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
    evidence = build_evidence(repo_root, args.version, args.skip_gate_run)
    outputs = write_outputs(repo_root, args.version, evidence)

    print("Numeric conformance evidence report")
    print(f"  status: {evidence['aggregate']['status']}")
    print(f"  cases: {evidence['aggregate']['case_count']}")
    print(f"  series: {evidence['aggregate']['series_count']}")
    print(f"  max_abs_delta_c: {number_label(evidence['aggregate']['max_abs_delta_c'], 12)}")
    print(f"  html: {outputs['html']}")
    print(f"  pdf: {outputs['pdf']}")
    print(f"  json: {outputs['json']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
