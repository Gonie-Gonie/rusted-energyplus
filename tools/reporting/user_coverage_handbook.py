from __future__ import annotations

import argparse
import json
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


CLAIM_BOUNDARY = (
    "This handbook is a user-facing reading guide for generated coverage data. "
    "It does not add numerical conformance beyond the cases, variables, "
    "tolerances, reports, and gates already promoted in release evidence."
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the user coverage handbook.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.32.0")
    return parser.parse_args()


def load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        raise FileNotFoundError(f"Required input JSON is missing: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def evidence_root(repo_root: Path, version: str) -> Path:
    return repo_root / ".runtime" / "release-evidence" / f"v{version}"


def status_bucket(row: dict[str, Any]) -> str:
    return str(row.get("status", ""))


def take_rows(rows: list[dict[str, Any]], status: str) -> list[dict[str, Any]]:
    return [row for row in rows if status_bucket(row) == status]


def sorted_rows(rows: list[dict[str, Any]], *keys: str) -> list[dict[str, Any]]:
    return sorted(rows, key=lambda row: tuple(str(row.get(key, "")) for key in keys))


def boundary_text(row: dict[str, Any]) -> str:
    return str(row.get("support_boundary") or row.get("notes") or "")


def build_handbook(repo_root: Path, version: str) -> dict[str, Any]:
    root = evidence_root(repo_root, version)
    support = load_json(root / "support-coverage-report.json")
    index = load_json(root / "conformance-index-report.json")

    input_rows = list(support.get("input_objects", []))
    output_rows = list(support.get("output_variables", []))
    algorithm_rows = list(support.get("algorithms", []))
    case_rows = list(support.get("cases", []))

    typed_inputs = sorted_rows(take_rows(input_rows, "typed"), "family", "name")
    structural_inputs = sorted_rows(take_rows(input_rows, "typed_graph_only"), "family", "name")
    conformance_outputs = sorted_rows(take_rows(output_rows, "conformance"), "domain", "name")
    diagnostic_outputs = sorted_rows(take_rows(output_rows, "diagnostic"), "domain", "name")
    baseline_outputs = sorted_rows(take_rows(output_rows, "baseline"), "domain", "name")
    conformance_algorithms = sorted_rows(take_rows(algorithm_rows, "conformance"), "domain", "id")
    diagnostic_algorithms = sorted_rows(take_rows(algorithm_rows, "diagnostic_only"), "domain", "id")
    promoted_cases = sorted_rows(
        [row for row in case_rows if bool(row.get("conformance_claim", False))],
        "case_id",
    )

    support_aggregate = support.get("aggregate") or {}
    index_aggregate = index.get("aggregate") or {}
    return {
        "schema_version": 1,
        "version": version,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "claim_boundary": CLAIM_BOUNDARY,
        "source_artifacts": {
            "support_coverage_json": f".runtime/release-evidence/v{version}/support-coverage-report.json",
            "conformance_index_json": f".runtime/release-evidence/v{version}/conformance-index-report.json",
        },
        "aggregate": {
            "input_object_count": support_aggregate.get("input_object_count", len(input_rows)),
            "typed_input_count": support_aggregate.get("typed_input_count", len(typed_inputs)),
            "structural_input_count": support_aggregate.get("typed_graph_only_input_count", len(structural_inputs)),
            "tracked_output_variable_count": support_aggregate.get("tracked_output_variable_count", len(output_rows)),
            "conformance_output_variable_count": support_aggregate.get(
                "conformance_output_variable_count",
                len(conformance_outputs),
            ),
            "diagnostic_output_variable_count": support_aggregate.get(
                "diagnostic_output_variable_count",
                len(diagnostic_outputs),
            ),
            "baseline_output_variable_count": len(baseline_outputs),
            "algorithm_count": support_aggregate.get("algorithm_count", len(algorithm_rows)),
            "conformance_algorithm_count": support_aggregate.get(
                "conformance_algorithm_count",
                len(conformance_algorithms),
            ),
            "diagnostic_algorithm_count": support_aggregate.get(
                "diagnostic_algorithm_count",
                len(diagnostic_algorithms),
            ),
            "case_count": support_aggregate.get("case_count", len(case_rows)),
            "conformance_case_count": support_aggregate.get("conformance_case_count", len(promoted_cases)),
            "index_output_request_count": index_aggregate.get("output_count", 0),
            "status": "pass",
        },
        "user_decision_rules": [
            "Use conformance rows only when the input, output, frequency, evidence case, tolerance, and gate match your intended scope.",
            "Treat diagnostic, baseline, and typed-graph-only rows as visibility into development progress, not as compatibility claims.",
            "For unsupported objects or outputs, prefer an explicit gap over extrapolating from a neighboring row.",
            "Use the release evidence manifest to confirm that this handbook and its source reports were published as release assets.",
        ],
        "supported_inputs": typed_inputs,
        "structural_inputs": structural_inputs,
        "conformance_outputs": conformance_outputs,
        "diagnostic_outputs": diagnostic_outputs,
        "baseline_outputs": baseline_outputs,
        "conformance_algorithms": conformance_algorithms,
        "diagnostic_algorithms": diagnostic_algorithms,
        "promoted_cases": promoted_cases,
        "known_gaps": list(support.get("known_gaps", [])),
        "artifacts": {
            "markdown": f".runtime/release-evidence/v{version}/user-coverage-handbook.md",
            "html": f".runtime/release-evidence/v{version}/user-coverage-handbook.html",
            "pdf": f".runtime/release-evidence/v{version}/user-coverage-handbook.pdf",
            "json": f".runtime/release-evidence/v{version}/user-coverage-handbook.json",
        },
    }


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


def build_metric_table(handbook: dict[str, Any]) -> Table:
    aggregate = handbook["aggregate"]
    rows = [
        ["Typed input objects", aggregate["typed_input_count"]],
        ["Structural input objects", aggregate["structural_input_count"]],
        ["Conformance output variables", aggregate["conformance_output_variable_count"]],
        ["Diagnostic output variables", aggregate["diagnostic_output_variable_count"]],
        ["Baseline output variables", aggregate["baseline_output_variable_count"]],
        ["Conformance algorithms", aggregate["conformance_algorithm_count"]],
        ["Diagnostic algorithms", aggregate["diagnostic_algorithm_count"]],
        ["Promoted conformance cases", aggregate["conformance_case_count"]],
        ["Manifest output requests", aggregate["index_output_request_count"]],
        ["Status", aggregate["status"]],
    ]
    return doc_table(["Metric", "Value"], rows, "User coverage handbook summary.")


def input_table(rows: list[dict[str, Any]], caption: str) -> Table:
    return doc_table(
        ["Input object", "Family", "First evidence", "Boundary"],
        [
            [
                row.get("name", ""),
                row.get("family", ""),
                row.get("first_evidence", ""),
                boundary_text(row),
            ]
            for row in rows
        ],
        caption,
    )


def output_table(rows: list[dict[str, Any]], caption: str) -> Table:
    return doc_table(
        ["Output variable", "Domain", "First evidence", "Observed freq", "Boundary"],
        [
            [
                row.get("name", ""),
                row.get("domain", ""),
                row.get("first_evidence", ""),
                ", ".join(row.get("observed_frequencies", [])),
                boundary_text(row),
            ]
            for row in rows
        ],
        caption,
    )


def algorithm_table(rows: list[dict[str, Any]], caption: str) -> Table:
    return doc_table(
        ["Algorithm", "Domain", "First evidence", "Proof outputs", "Boundary"],
        [
            [
                row.get("id", ""),
                row.get("domain", ""),
                row.get("first_evidence", ""),
                ", ".join(row.get("proof_variables", [])),
                boundary_text(row),
            ]
            for row in rows
        ],
        caption,
    )


def case_table(rows: list[dict[str, Any]]) -> Table:
    return doc_table(
        ["Case", "Title", "Milestone", "Domains", "Gate"],
        [
            [
                row.get("case_id", ""),
                row.get("title", ""),
                row.get("milestone", ""),
                ", ".join(row.get("domains", [])),
                row.get("gate_script", ""),
            ]
            for row in rows
        ],
        "Promoted conformance cases that define the current numerical claim.",
    )


def create_scope_chart(handbook: dict[str, Any]) -> Any:
    aggregate = handbook["aggregate"]
    labels = [
        "Inputs\ntyped",
        "Inputs\nstructural",
        "Outputs\nconformance",
        "Outputs\ndiagnostic",
        "Outputs\nbaseline",
        "Algorithms\nconformance",
        "Algorithms\ndiagnostic",
    ]
    values = [
        aggregate["typed_input_count"],
        aggregate["structural_input_count"],
        aggregate["conformance_output_variable_count"],
        aggregate["diagnostic_output_variable_count"],
        aggregate["baseline_output_variable_count"],
        aggregate["conformance_algorithm_count"],
        aggregate["diagnostic_algorithm_count"],
    ]
    colors = ["#2f6f9f", "#7a8c9f", "#2d7d46", "#b17921", "#697789", "#2d7d46", "#b17921"]
    fig, ax = plt.subplots(figsize=(7.2, 3.4))
    ax.bar(labels, values, color=colors)
    ax.set_title("Current User Coverage Counts", loc="left", fontsize=13, fontweight="bold")
    ax.grid(axis="y", color="#e3e7ed", linewidth=0.8)
    ax.set_axisbelow(True)
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["left"].set_color("#9aa7b5")
    ax.spines["bottom"].set_color("#9aa7b5")
    ax.tick_params(axis="x", labelsize=8, colors="#17212b")
    ax.tick_params(axis="y", labelsize=8, colors="#5b6775")
    for index, value in enumerate(values):
        ax.text(index, value + 0.3, str(value), ha="center", va="bottom", fontsize=8, color="#17212b")
    fig.tight_layout(pad=1.0)
    return fig


def build_document(handbook: dict[str, Any], chart: Any) -> Document:
    version = handbook["version"]
    settings = DocumentSettings(
        metadata_author="rusted-energyplus",
        subtitle="User-facing support scope handbook",
        cover_page=True,
        page_margins=PageMargins(0.55, 0.55, 0.55, 0.55, unit="in"),
        theme=Theme(
            body_font_name="Segoe UI",
            monospace_font_name="Consolas",
            body_font_size=8.6,
            heading_sizes=(20, 16, 13, 11),
            table_alignment="center",
            figure_alignment="center",
            show_page_numbers=True,
            page_number_alignment="center",
        ),
    )
    return Document(
        f"eplus-rs {version} User Coverage Handbook",
        TableOfContents("Table of Contents", max_level=2),
        Chapter(
            "What This Handbook Answers",
            Box(
                Paragraph(
                    "Use this handbook before running an IDF through eplus-rs. It summarizes which inputs, "
                    "outputs, and algorithm families are currently supported, diagnostic-only, baseline-only, "
                    "or structural. It is intentionally user-facing; the detailed matrix remains in the "
                    "Support Coverage Report."
                ),
                title="User Coverage Handbook",
                border_color="#2f6f9f",
                background_color="#f4f8fb",
                padding=0.12,
            ),
            Paragraph("Claim boundary: ", code(handbook["claim_boundary"])),
        ),
        Chapter(
            "Executive Summary",
            Paragraph(
                "Generated UTC: ",
                code(handbook["generated_at_utc"]),
                ". Source support JSON: ",
                code(handbook["source_artifacts"]["support_coverage_json"]),
                ".",
            ),
            build_metric_table(handbook),
            Figure(chart, caption="Current user coverage by support class.", width=7.2),
        ),
        Chapter(
            "Decision Rules",
            doc_table(
                ["Rule"],
                [[rule] for rule in handbook["user_decision_rules"]],
                "How to read support rows without overstating compatibility.",
            ),
        ),
        Chapter("Typed Inputs", input_table(handbook["supported_inputs"], "Input objects with typed support.")),
        Chapter(
            "Structural Inputs",
            input_table(handbook["structural_inputs"], "Input objects represented structurally without numerical conformance."),
        ),
        Chapter(
            "Conformance Outputs",
            output_table(handbook["conformance_outputs"], "Output variables with promoted tolerance-gated evidence."),
        ),
        Chapter(
            "Diagnostic and Baseline Outputs",
            output_table(handbook["diagnostic_outputs"], "Diagnostic output variables."),
            output_table(handbook["baseline_outputs"], "Baseline-only output variables."),
        ),
        Chapter(
            "Algorithm Scope",
            algorithm_table(handbook["conformance_algorithms"], "Algorithm families with limited conformance evidence."),
            algorithm_table(handbook["diagnostic_algorithms"], "Algorithm families with diagnostic-only projection evidence."),
        ),
        Chapter("Promoted Cases", case_table(handbook["promoted_cases"])),
        Chapter(
            "Known Gaps",
            doc_table(["Gap"], [[gap] for gap in handbook["known_gaps"]], "Explicit non-claims carried from support coverage."),
        ),
        Chapter(
            "Artifact Paths",
            doc_table(
                ["Artifact", "Path"],
                [
                    ["Markdown", handbook["artifacts"]["markdown"]],
                    ["HTML", handbook["artifacts"]["html"]],
                    ["PDF", handbook["artifacts"]["pdf"]],
                    ["JSON", handbook["artifacts"]["json"]],
                ],
                "Generated user coverage handbook artifacts.",
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


def render_markdown(handbook: dict[str, Any]) -> str:
    aggregate = handbook["aggregate"]
    input_rows = [
        [row["name"], row["family"], row["first_evidence"], boundary_text(row)]
        for row in handbook["supported_inputs"]
    ]
    output_rows = [
        [row["name"], row["domain"], row["first_evidence"], boundary_text(row)]
        for row in handbook["conformance_outputs"]
    ]
    algorithm_rows = [
        [row["id"], row["domain"], row["first_evidence"], boundary_text(row)]
        for row in handbook["conformance_algorithms"]
    ]
    return (
        f"# User Coverage Handbook v{handbook['version']}\n\n"
        f"- schema_version: {handbook['schema_version']}\n"
        f"- generated_at_utc: {handbook['generated_at_utc']}\n"
        f"- claim_boundary: {handbook['claim_boundary']}\n"
        f"- typed_inputs: {aggregate['typed_input_count']}\n"
        f"- structural_inputs: {aggregate['structural_input_count']}\n"
        f"- conformance_outputs: {aggregate['conformance_output_variable_count']}\n"
        f"- diagnostic_outputs: {aggregate['diagnostic_output_variable_count']}\n"
        f"- baseline_outputs: {aggregate['baseline_output_variable_count']}\n"
        f"- conformance_algorithms: {aggregate['conformance_algorithm_count']}\n"
        f"- diagnostic_algorithms: {aggregate['diagnostic_algorithm_count']}\n\n"
        "## Decision Rules\n\n"
        + "\n".join(f"- {rule}" for rule in handbook["user_decision_rules"])
        + "\n\n## Typed Inputs\n\n"
        + markdown_table(["Input", "Family", "First evidence", "Boundary"], input_rows)
        + "\n## Conformance Outputs\n\n"
        + markdown_table(["Output", "Domain", "First evidence", "Boundary"], output_rows)
        + "\n## Conformance Algorithms\n\n"
        + markdown_table(["Algorithm", "Domain", "First evidence", "Boundary"], algorithm_rows)
        + "\n## Known Gaps\n\n"
        + "\n".join(f"- {gap}" for gap in handbook["known_gaps"])
        + "\n"
    )


def write_outputs(repo_root: Path, version: str, handbook: dict[str, Any]) -> dict[str, Path]:
    root = evidence_root(repo_root, version)
    root.mkdir(parents=True, exist_ok=True)
    chart = create_scope_chart(handbook)
    try:
        document = build_document(handbook, chart)
        json_path = root / "user-coverage-handbook.json"
        html_path = root / "user-coverage-handbook.html"
        pdf_path = root / "user-coverage-handbook.pdf"
        markdown_path = root / "user-coverage-handbook.md"

        json_path.write_text(json.dumps(handbook, indent=2), encoding="utf-8")
        markdown_path.write_text(render_markdown(handbook), encoding="utf-8")
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
    handbook = build_handbook(repo_root, args.version)
    outputs = write_outputs(repo_root, args.version, handbook)
    aggregate = handbook["aggregate"]

    print("User coverage handbook")
    print(f"  status: {aggregate['status']}")
    print(f"  typed_inputs: {aggregate['typed_input_count']}")
    print(f"  conformance_outputs: {aggregate['conformance_output_variable_count']}")
    print(f"  conformance_algorithms: {aggregate['conformance_algorithm_count']}")
    print(f"  markdown: {outputs['markdown']}")
    print(f"  html: {outputs['html']}")
    print(f"  pdf: {outputs['pdf']}")
    print(f"  json: {outputs['json']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
