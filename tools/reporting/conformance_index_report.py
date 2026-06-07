from __future__ import annotations

import argparse
import json
import tomllib
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from oodocs import (
    Box,
    Chapter,
    Document,
    DocumentSettings,
    Figure,
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


ORACLE_VERSION = "26.1.0"
CLAIM_BOUNDARY = (
    "The conformance index is a release coverage map. It does not promote new "
    "numerical conformance by itself."
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the release conformance index report.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.31.0")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def list_value(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, list):
        return ", ".join(str(item) for item in value)
    return str(value)


def unique_sorted(values: list[str]) -> list[str]:
    return sorted({value for value in values if value})


def rel_path(repo_root: Path, path: Path) -> str:
    return path.relative_to(repo_root).as_posix()


def case_tier(case: dict[str, Any]) -> str:
    return str((case.get("manifest_v2") or {}).get("tier", ""))


def source_kind(case: dict[str, Any]) -> str:
    return str((case.get("manifest_v2") or {}).get("source_kind", ""))


def scope_domains(case: dict[str, Any]) -> list[str]:
    return [str(value) for value in (case.get("scope") or {}).get("domains", [])]


def output_domain(output: dict[str, Any]) -> str:
    return str(output.get("domain") or "")


def output_level(output: dict[str, Any]) -> str:
    return str(output.get("level") or "")


def collect_case_rows(repo_root: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in sorted((repo_root / "data" / "conformance_cases").glob("*/case.toml")):
        case = load_toml(path)
        outputs = list(case.get("outputs", []))
        meters = list(case.get("meters", []))
        levels = [output_level(output) for output in outputs]
        levels += [output_level(meter) for meter in meters]
        domains = scope_domains(case)
        domains += [output_domain(output) for output in outputs]
        domains += [output_domain(meter) for meter in meters]
        report = case.get("report") or {}
        gate = case.get("gate") or {}
        rows.append(
            {
                "case_id": str(case.get("id", path.parent.name)),
                "title": str(case.get("title", "")),
                "milestone": str(case.get("milestone", "")),
                "comparison_class": str(case.get("comparison_class", "")),
                "conformance_claim": bool(case.get("conformance_claim", False)),
                "oracle_version": str(case.get("oracle_version", "")),
                "tier": case_tier(case),
                "source_kind": source_kind(case),
                "domains": unique_sorted(domains),
                "output_levels": unique_sorted(levels),
                "outputs": outputs,
                "meters": meters,
                "output_count": len(outputs),
                "meter_count": len(meters),
                "waiver_count": len(case.get("waivers", [])),
                "report_path": str(report.get("path", "")),
                "gate_script": str(gate.get("script", "")),
                "gate_blocking": bool(gate.get("blocking", False)),
                "manifest": rel_path(repo_root, path),
            }
        )
    return rows


def coverage_matrix(case_rows: list[dict[str, Any]]) -> dict[str, Any]:
    tier_counts = Counter(row["tier"] or "unspecified" for row in case_rows)
    class_counts = Counter(row["comparison_class"] for row in case_rows)
    domain_counts: Counter[str] = Counter()
    output_domain_counts: Counter[str] = Counter()
    meter_domain_counts: Counter[str] = Counter()
    level_counts: Counter[str] = Counter()

    for row in case_rows:
        for domain in row["domains"]:
            domain_counts[domain] += 1
        for output in row["outputs"]:
            output_domain_counts[output_domain(output) or "unspecified"] += 1
            level_counts[output_level(output) or "unspecified"] += 1
        for meter in row["meters"]:
            meter_domain_counts[output_domain(meter) or "unspecified"] += 1
            level_counts[output_level(meter) or "unspecified"] += 1

    return {
        "tier_counts": dict(sorted(tier_counts.items())),
        "comparison_class_counts": dict(sorted(class_counts.items())),
        "domain_case_counts": dict(sorted(domain_counts.items())),
        "output_domain_counts": dict(sorted(output_domain_counts.items())),
        "meter_domain_counts": dict(sorted(meter_domain_counts.items())),
        "level_counts": dict(sorted(level_counts.items())),
    }


def collect_output_rows(case_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for case in case_rows:
        for output in case["outputs"]:
            rows.append(
                {
                    "case_id": case["case_id"],
                    "key": str(output.get("key", "")),
                    "variable": str(output.get("variable", "")),
                    "frequency": str(output.get("frequency", "")),
                    "class": str(output.get("class", "")),
                    "source": str(output.get("source", "")),
                    "domain": output_domain(output),
                    "level": output_level(output),
                    "abs_tol": output.get("abs_tol"),
                    "rmse_tol": output.get("rmse_tol"),
                    "rel_tol": output.get("rel_tol"),
                }
            )
    return rows


def collect_meter_rows(case_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for case in case_rows:
        for meter in case["meters"]:
            rows.append(
                {
                    "case_id": case["case_id"],
                    "name": str(meter.get("name", "")),
                    "frequency": str(meter.get("frequency", "")),
                    "source": str(meter.get("source", "")),
                    "domain": output_domain(meter),
                    "level": output_level(meter),
                    "abs_tol": meter.get("abs_tol"),
                    "rmse_tol": meter.get("rmse_tol"),
                    "rel_tol": meter.get("rel_tol"),
                }
            )
    return rows


def build_conformance_index(repo_root: Path, version: str) -> dict[str, Any]:
    case_rows = collect_case_rows(repo_root)
    output_rows = collect_output_rows(case_rows)
    meter_rows = collect_meter_rows(case_rows)
    conformance_cases = [row for row in case_rows if row["conformance_claim"]]
    blocking_gates = [row for row in case_rows if row["gate_blocking"]]
    missing_report_contracts = [row["case_id"] for row in case_rows if not row["report_path"]]
    missing_gate_contracts = [row["case_id"] for row in case_rows if not row["gate_script"]]

    return {
        "schema_version": 1,
        "version": version,
        "oracle_version": ORACLE_VERSION,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "claim_boundary": CLAIM_BOUNDARY,
        "aggregate": {
            "case_count": len(case_rows),
            "conformance_case_count": len(conformance_cases),
            "baseline_or_diagnostic_case_count": len(case_rows) - len(conformance_cases),
            "output_count": len(output_rows),
            "meter_count": len(meter_rows),
            "blocking_gate_count": len(blocking_gates),
            "missing_report_contract_count": len(missing_report_contracts),
            "missing_gate_contract_count": len(missing_gate_contracts),
            "status": "pass",
        },
        "coverage_matrix": coverage_matrix(case_rows),
        "cases": case_rows,
        "outputs": output_rows,
        "meters": meter_rows,
        "known_gaps": {
            "missing_report_contracts": missing_report_contracts,
            "missing_gate_contracts": missing_gate_contracts,
        },
        "artifacts": {
            "markdown": f".runtime/release-evidence/v{version}/conformance-index.md",
            "html": f".runtime/release-evidence/v{version}/conformance-index-report.html",
            "pdf": f".runtime/release-evidence/v{version}/conformance-index-report.pdf",
            "json": f".runtime/release-evidence/v{version}/conformance-index-report.json",
        },
    }


def number_label(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.12g}"
    return str(value)


def doc_table(headers: list[str], rows: list[list[Any]], caption: str) -> Table:
    return Table(
        headers,
        [["" if value is None else str(value) for value in row] for row in rows],
        caption=caption,
        header_background_color="#eef3f7",
        border_color="#d7dde5",
        alternate_row_background_color="#f8fafc",
        repeat_header_rows=True,
        split=True,
    )


def build_metric_table(index: dict[str, Any]) -> Table:
    aggregate = index["aggregate"]
    rows = [
        ["Cases", aggregate["case_count"]],
        ["Conformance cases", aggregate["conformance_case_count"]],
        ["Baseline or diagnostic cases", aggregate["baseline_or_diagnostic_case_count"]],
        ["Requested outputs", aggregate["output_count"]],
        ["Requested meters", aggregate["meter_count"]],
        ["Blocking gates", aggregate["blocking_gate_count"]],
        ["Report contracts missing", aggregate["missing_report_contract_count"]],
        ["Gate contracts missing", aggregate["missing_gate_contract_count"]],
    ]
    return doc_table(["Metric", "Value"], rows, "Release conformance index summary.")


def build_case_table(index: dict[str, Any]) -> Table:
    rows = [
        [
            case["case_id"],
            case["milestone"],
            case["tier"],
            case["comparison_class"],
            str(case["conformance_claim"]).lower(),
            ", ".join(case["domains"]),
            ", ".join(case["output_levels"]),
            case["output_count"],
            case["meter_count"],
        ]
        for case in index["cases"]
    ]
    return doc_table(
        ["Case", "Milestone", "Tier", "Class", "Claim", "Domains", "Levels", "Outputs", "Meters"],
        rows,
        "Case coverage matrix.",
    )


def build_output_table(index: dict[str, Any]) -> Table:
    rows = [
        [
            output["case_id"],
            output["key"],
            output["variable"],
            output["frequency"],
            output["class"],
            output["domain"],
            output["level"],
            number_label(output["abs_tol"]),
            number_label(output["rmse_tol"]),
            number_label(output["rel_tol"]),
        ]
        for output in index["outputs"]
    ]
    return doc_table(
        ["Case", "Key", "Variable", "Freq", "Class", "Domain", "Level", "Abs tol", "RMSE tol", "Rel tol"],
        rows,
        "Requested output coverage matrix.",
    )


def build_meter_table(index: dict[str, Any]) -> Table:
    rows = [
        [
            meter["case_id"],
            meter["name"],
            meter["frequency"],
            meter["source"],
            meter["domain"],
            meter["level"],
            number_label(meter["abs_tol"]),
            number_label(meter["rmse_tol"]),
            number_label(meter["rel_tol"]),
        ]
        for meter in index["meters"]
    ]
    if not rows:
        rows = [["", "No meter requests are promoted or baseline-tracked yet.", "", "", "", "", "", "", ""]]
    return doc_table(
        ["Case", "Meter", "Freq", "Source", "Domain", "Level", "Abs tol", "RMSE tol", "Rel tol"],
        rows,
        "Requested meter coverage matrix.",
    )


def build_contract_table(index: dict[str, Any]) -> Table:
    rows = [
        [
            case["case_id"],
            case["report_path"],
            case["gate_script"],
            str(case["gate_blocking"]).lower(),
            case["manifest"],
        ]
        for case in index["cases"]
    ]
    return doc_table(
        ["Case", "Report", "Gate", "Blocking", "Manifest"],
        rows,
        "Report and gate contract matrix.",
    )


def build_domain_table(index: dict[str, Any]) -> Table:
    matrix = index["coverage_matrix"]
    domains = sorted(
        set(matrix["domain_case_counts"])
        | set(matrix["output_domain_counts"])
        | set(matrix["meter_domain_counts"])
    )
    rows = [
        [
            domain,
            matrix["domain_case_counts"].get(domain, 0),
            matrix["output_domain_counts"].get(domain, 0),
            matrix["meter_domain_counts"].get(domain, 0),
        ]
        for domain in domains
    ]
    return doc_table(["Domain", "Cases", "Outputs", "Meters"], rows, "Domain coverage matrix.")


def create_domain_chart(index: dict[str, Any]) -> Any:
    matrix = index["coverage_matrix"]
    domains = sorted(
        set(matrix["domain_case_counts"])
        | set(matrix["output_domain_counts"])
        | set(matrix["meter_domain_counts"])
    )
    if not domains:
        domains = ["none"]
    output_counts = [matrix["output_domain_counts"].get(domain, 0) for domain in domains]
    meter_counts = [matrix["meter_domain_counts"].get(domain, 0) for domain in domains]
    max_value = max(output_counts + meter_counts + [1])

    height = max(2.4, 1.1 + len(domains) * 0.35)
    fig, ax = plt.subplots(figsize=(7.2, height), dpi=180)
    y_values = list(range(len(domains)))
    ax.barh([y - 0.14 for y in y_values], output_counts, height=0.22, color="#2f6f9f", label="Outputs")
    ax.barh([y + 0.14 for y in y_values], meter_counts, height=0.22, color="#c77d1a", label="Meters")
    ax.set_yticks(y_values, domains)
    ax.invert_yaxis()
    ax.set_xlim(0, max_value + 1)
    ax.set_xlabel("Requested series count", fontsize=9, color="#5b6775")
    ax.set_title("Coverage by Evidence Domain", loc="left", fontsize=13, fontweight="bold", color="#17212b")
    ax.grid(axis="x", color="#e3e7ed", linewidth=0.8)
    ax.set_axisbelow(True)
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["left"].set_color("#9aa7b5")
    ax.spines["bottom"].set_color("#9aa7b5")
    ax.tick_params(axis="x", colors="#5b6775", labelsize=8)
    ax.tick_params(axis="y", colors="#17212b", labelsize=9, length=0)
    ax.legend(loc="lower right", fontsize=8, frameon=False)
    fig.tight_layout(pad=1.0)
    return fig


def build_document(index: dict[str, Any], chart: Any) -> Document:
    version = index["version"]
    settings = DocumentSettings(
        metadata_author="rusted-energyplus",
        subtitle="Release conformance index and coverage matrices",
        cover_page=True,
        page_margins=PageMargins(0.55, 0.55, 0.55, 0.55, unit="in"),
        theme=Theme(
            body_font_name="Segoe UI",
            monospace_font_name="Consolas",
            body_font_size=9.0,
            heading_sizes=(20, 16, 13, 11),
            table_alignment="center",
            figure_alignment="center",
            show_page_numbers=True,
            page_number_alignment="center",
        ),
    )
    return Document(
        f"eplus-rs {version} Conformance Index Report",
        TableOfContents("Table of Contents", max_level=2),
        Chapter(
            "Claim Boundary",
            Box(
                Paragraph(
                    "This report maps manifests, output requests, meter requests, report contracts, and gate contracts. "
                    "It is not a numerical conformance claim unless a case is already promoted by manifest, tolerance, "
                    "compare report, and blocking gate."
                ),
                title="Release Scope",
                border_color="#2f6f9f",
                background_color="#f4f8fb",
                padding=0.12,
            ),
        ),
        Chapter(
            "Executive Summary",
            Paragraph(
                "Generated UTC: ",
                code(index["generated_at_utc"]),
                ". EnergyPlus oracle: ",
                code(index["oracle_version"]),
                ". Report schema: ",
                code(str(index["schema_version"])),
                ".",
            ),
            build_metric_table(index),
        ),
        Chapter("Case Coverage Matrix", build_case_table(index)),
        Chapter(
            "Domain Coverage",
            Figure(chart, caption="Requested output and meter coverage by domain.", width=6.8),
            build_domain_table(index),
        ),
        Chapter("Output Coverage Matrix", build_output_table(index)),
        Chapter("Meter Coverage Matrix", build_meter_table(index)),
        Chapter("Report and Gate Contracts", build_contract_table(index)),
        Chapter(
            "Artifact Paths",
            doc_table(
                ["Artifact", "Path"],
                [
                    ["Markdown index", index["artifacts"]["markdown"]],
                    ["HTML report", index["artifacts"]["html"]],
                    ["PDF report", index["artifacts"]["pdf"]],
                    ["JSON report", index["artifacts"]["json"]],
                ],
                "Generated release conformance index artifacts.",
            ),
        ),
        settings=settings,
    )


def markdown_cell(value: Any) -> str:
    return str(value).replace("|", "\\|").replace("\n", "<br>")


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    lines = ["| " + " | ".join(headers) + " |"]
    lines.append("|" + "|".join(["---"] * len(headers)) + "|")
    for row in rows:
        lines.append("| " + " | ".join(markdown_cell(value) for value in row) + " |")
    return "\n".join(lines) + "\n"


def render_markdown(index: dict[str, Any]) -> str:
    aggregate = index["aggregate"]
    case_rows = [
        [
            case["case_id"],
            case["milestone"],
            case["tier"],
            case["comparison_class"],
            str(case["conformance_claim"]).lower(),
            ", ".join(case["domains"]),
            ", ".join(case["output_levels"]),
        ]
        for case in index["cases"]
    ]
    domain_rows = [
        [
            domain,
            index["coverage_matrix"]["domain_case_counts"].get(domain, 0),
            index["coverage_matrix"]["output_domain_counts"].get(domain, 0),
            index["coverage_matrix"]["meter_domain_counts"].get(domain, 0),
        ]
        for domain in sorted(
            set(index["coverage_matrix"]["domain_case_counts"])
            | set(index["coverage_matrix"]["output_domain_counts"])
            | set(index["coverage_matrix"]["meter_domain_counts"])
        )
    ]
    return (
        f"# Conformance Index Report v{index['version']}\n\n"
        f"- schema_version: {index['schema_version']}\n"
        f"- oracle_version: {index['oracle_version']}\n"
        f"- generated_at_utc: {index['generated_at_utc']}\n"
        f"- claim_boundary: {index['claim_boundary']}\n"
        f"- cases: {aggregate['case_count']}\n"
        f"- conformance_cases: {aggregate['conformance_case_count']}\n"
        f"- outputs: {aggregate['output_count']}\n"
        f"- meters: {aggregate['meter_count']}\n\n"
        "## Cases\n\n"
        + markdown_table(["Case", "Milestone", "Tier", "Class", "Claim", "Domains", "Levels"], case_rows)
        + "\n## Domain Coverage\n\n"
        + markdown_table(["Domain", "Cases", "Outputs", "Meters"], domain_rows)
    )


def write_outputs(repo_root: Path, version: str, index: dict[str, Any]) -> dict[str, Path]:
    evidence_root = repo_root / ".runtime" / "release-evidence" / f"v{version}"
    evidence_root.mkdir(parents=True, exist_ok=True)
    chart = create_domain_chart(index)
    try:
        document = build_document(index, chart)

        json_path = evidence_root / "conformance-index-report.json"
        html_path = evidence_root / "conformance-index-report.html"
        pdf_path = evidence_root / "conformance-index-report.pdf"
        markdown_path = evidence_root / "conformance-index.md"

        json_path.write_text(json.dumps(index, indent=2), encoding="utf-8")
        markdown_path.write_text(render_markdown(index), encoding="utf-8")
        document.save_html(html_path)
        document.save_pdf(pdf_path)
    finally:
        plt.close(chart)

    return {
        "json": json_path,
        "html": html_path,
        "pdf": pdf_path,
        "markdown": markdown_path,
    }


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    index = build_conformance_index(repo_root, args.version)
    outputs = write_outputs(repo_root, args.version, index)

    print("Conformance index report")
    print(f"  status: {index['aggregate']['status']}")
    print(f"  cases: {index['aggregate']['case_count']}")
    print(f"  conformance_cases: {index['aggregate']['conformance_case_count']}")
    print(f"  outputs: {index['aggregate']['output_count']}")
    print(f"  meters: {index['aggregate']['meter_count']}")
    print(f"  markdown: {outputs['markdown']}")
    print(f"  html: {outputs['html']}")
    print(f"  pdf: {outputs['pdf']}")
    print(f"  json: {outputs['json']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
