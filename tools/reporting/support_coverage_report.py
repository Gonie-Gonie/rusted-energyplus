from __future__ import annotations

import argparse
import json
import tomllib
from collections import Counter, defaultdict
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
SUPPORT_BOUNDARY = (
    "This user-facing coverage report describes only the inputs, outputs, "
    "and algorithms tracked in repository specs and conformance manifests. "
    "It is not a full EnergyPlus compatibility claim."
)


OBJECT_STATUS_LABELS = {
    "typed": "Typed input support",
    "typed_graph_only": "Typed graph only",
}

VARIABLE_STATUS_LABELS = {
    "conformance": "Tolerance-gated output",
    "diagnostic": "Diagnostic output",
    "baseline": "Baseline output",
}

ALGORITHM_STATUS_LABELS = {
    "conformance": "Limited algorithm conformance",
    "diagnostic_only": "Diagnostic projection only",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the user support coverage report.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.30.0")
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


def rel_path(repo_root: Path, path: Path) -> str:
    return path.relative_to(repo_root).as_posix()


def unique_sorted(values: list[str]) -> list[str]:
    return sorted({value for value in values if value})


def output_level(output: dict[str, Any]) -> str:
    return str(output.get("level") or "")


def output_domain(output: dict[str, Any]) -> str:
    return str(output.get("domain") or "")


def scope_domains(case: dict[str, Any]) -> list[str]:
    return [str(value) for value in (case.get("scope") or {}).get("domains", [])]


def case_tier(case: dict[str, Any]) -> str:
    return str((case.get("manifest_v2") or {}).get("tier", ""))


def source_kind(case: dict[str, Any]) -> str:
    return str((case.get("manifest_v2") or {}).get("source_kind", ""))


def status_label(status: str, labels: dict[str, str]) -> str:
    return labels.get(status, status.replace("_", " ").title() if status else "")


def support_rank(status: str) -> int:
    ranks = {
        "conformance": 0,
        "typed": 1,
        "diagnostic": 2,
        "baseline": 3,
        "typed_graph_only": 4,
        "diagnostic_only": 5,
    }
    return ranks.get(status, 99)


def variable_support_boundary(status: str, sources: list[str]) -> str:
    if status == "conformance":
        if sources == ["eio"] or ("eio" in sources and "eso" not in sources):
            return "Static EIO conformance only; no dynamic runtime response or algorithm parity claim."
        return "Tolerance-gated time-series conformance only for the listed evidence case, variable, frequency, and gate."
    if status == "diagnostic":
        return "Diagnostic comparison or extraction only; not release conformance."
    if status == "baseline":
        return "EnergyPlus oracle baseline request only; Rust numerical parity is not claimed."
    return "Tracked output request only; support boundary is not promoted."


def collect_case_rows(repo_root: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in sorted((repo_root / "data" / "conformance_cases").glob("*/case.toml")):
        case = load_toml(path)
        outputs = list(case.get("outputs", []))
        meters = list(case.get("meters", []))
        output_levels = [output_level(output) for output in outputs]
        output_levels += [output_level(meter) for meter in meters]
        domains = scope_domains(case)
        domains += [output_domain(output) for output in outputs]
        domains += [output_domain(meter) for meter in meters]
        gate = case.get("gate") or {}
        rows.append(
            {
                "case_id": str(case.get("id", path.parent.name)),
                "title": str(case.get("title", "")),
                "milestone": str(case.get("milestone", "")),
                "comparison_class": str(case.get("comparison_class", "")),
                "conformance_claim": bool(case.get("conformance_claim", False)),
                "tier": case_tier(case),
                "source_kind": source_kind(case),
                "domains": unique_sorted(domains),
                "output_levels": unique_sorted(output_levels),
                "output_count": len(outputs),
                "meter_count": len(meters),
                "gate_script": str(gate.get("script", "")),
                "gate_blocking": bool(gate.get("blocking", False)),
                "manifest": rel_path(repo_root, path),
                "outputs": outputs,
                "meters": meters,
                "notes": [str(note) for note in case.get("notes", [])],
            }
        )
    return rows


def collect_object_rows(repo_root: Path) -> list[dict[str, Any]]:
    spec = load_toml(repo_root / "specs" / "object_coverage.toml")
    rows = []
    for item in spec.get("object", []):
        status = str(item.get("status", ""))
        rows.append(
            {
                "name": str(item.get("name", "")),
                "family": str(item.get("family", "")),
                "status": status,
                "user_status": status_label(status, OBJECT_STATUS_LABELS),
                "notes": list_value(item.get("notes", [])),
                "first_evidence": str(item.get("first_evidence", item.get("first_case", ""))),
                "support_boundary": str(item.get("support_boundary", "")),
            }
        )
    return sorted(rows, key=lambda row: (support_rank(row["status"]), row["family"], row["name"]))


def collect_manifest_output_rows(case_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for case in case_rows:
        for output in case["outputs"]:
            status = output_level(output)
            rows.append(
                {
                    "case_id": case["case_id"],
                    "key": str(output.get("key", "")),
                    "variable": str(output.get("variable", "")),
                    "frequency": str(output.get("frequency", "")),
                    "class": str(output.get("class", "")),
                    "source": str(output.get("source", "")),
                    "domain": output_domain(output),
                    "status": status,
                    "user_status": status_label(status, VARIABLE_STATUS_LABELS),
                }
            )
    return rows


def collect_variable_rows(repo_root: Path, output_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    spec = load_toml(repo_root / "specs" / "variable_coverage.toml")
    by_name: dict[str, dict[str, Any]] = {}
    for item in spec.get("variable", []):
        status = str(item.get("status", ""))
        by_name[str(item.get("name", ""))] = {
            "name": str(item.get("name", "")),
            "domain": str(item.get("domain", "")),
            "status": status,
            "user_status": status_label(status, VARIABLE_STATUS_LABELS),
            "first_case": str(item.get("first_case", "")),
            "first_evidence": str(item.get("first_evidence", item.get("first_case", ""))),
            "support_boundary": str(item.get("support_boundary", "")),
            "observed_cases": [],
            "best_evidence_cases": [],
            "observed_levels": [],
            "observed_sources": [],
            "observed_frequencies": [],
        }

    observed: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for output in output_rows:
        observed[output["variable"]].append(output)

    for name, outputs in observed.items():
        best_status = sorted((output["status"] for output in outputs), key=support_rank)[0]
        best_outputs = [output for output in outputs if output["status"] == best_status]
        observed_sources = unique_sorted([output["source"] for output in outputs])
        if name not in by_name:
            by_name[name] = {
                "name": name,
                "domain": sorted({output["domain"] for output in outputs if output["domain"]})[0]
                if any(output["domain"] for output in outputs)
                else "",
                "status": best_status,
                "user_status": status_label(best_status, VARIABLE_STATUS_LABELS),
                "first_case": best_outputs[0]["case_id"],
                "first_evidence": best_outputs[0]["case_id"],
                "support_boundary": variable_support_boundary(best_status, observed_sources),
                "observed_cases": [],
                "best_evidence_cases": [],
                "observed_levels": [],
                "observed_sources": [],
                "observed_frequencies": [],
            }
        row = by_name[name]
        if support_rank(best_status) < support_rank(row["status"]):
            row["status"] = best_status
            row["user_status"] = status_label(best_status, VARIABLE_STATUS_LABELS)
        if not row["first_evidence"]:
            row["first_evidence"] = best_outputs[0]["case_id"]
        if not row["support_boundary"]:
            row["support_boundary"] = variable_support_boundary(row["status"], observed_sources)
        row["observed_cases"] = unique_sorted([output["case_id"] for output in outputs])
        row["best_evidence_cases"] = unique_sorted([output["case_id"] for output in best_outputs])
        row["observed_levels"] = unique_sorted([output["status"] for output in outputs])
        row["observed_sources"] = observed_sources
        row["observed_frequencies"] = unique_sorted([output["frequency"] for output in outputs])

    return sorted(by_name.values(), key=lambda row: (support_rank(row["status"]), row["domain"], row["name"]))


def collect_algorithm_rows(repo_root: Path) -> list[dict[str, Any]]:
    spec = load_toml(repo_root / "specs" / "algorithm_ledger.toml")
    rows = []
    for item in spec.get("algorithm", []):
        status = str(item.get("status", ""))
        rows.append(
            {
                "id": str(item.get("id", "")),
                "domain": str(item.get("domain", "")),
                "status": status,
                "user_status": status_label(status, ALGORITHM_STATUS_LABELS),
                "claim_level": str(item.get("claim_level", "")),
                "first_case": str(item.get("first_case", "")),
                "first_evidence": str(item.get("first_evidence", item.get("first_case", ""))),
                "proof_variables": [str(value) for value in item.get("proof_variables", [])],
                "source_map": str(item.get("source_map", "")),
                "rust_target": [str(value) for value in item.get("rust_target", [])],
                "support_boundary": str(item.get("support_boundary", "")),
            }
        )
    return sorted(rows, key=lambda row: (support_rank(row["status"]), row["domain"], row["id"]))


def build_support_coverage(repo_root: Path, version: str) -> dict[str, Any]:
    case_rows = collect_case_rows(repo_root)
    output_rows = collect_manifest_output_rows(case_rows)
    object_rows = collect_object_rows(repo_root)
    variable_rows = collect_variable_rows(repo_root, output_rows)
    algorithm_rows = collect_algorithm_rows(repo_root)

    object_counts = Counter(row["status"] for row in object_rows)
    variable_counts = Counter(row["status"] for row in variable_rows)
    output_counts = Counter(row["status"] for row in output_rows)
    algorithm_counts = Counter(row["status"] for row in algorithm_rows)
    domain_counts = Counter()
    for row in case_rows:
        for domain in row["domains"]:
            domain_counts[domain] += 1

    return {
        "schema_version": 1,
        "version": version,
        "oracle_version": ORACLE_VERSION,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "support_boundary": SUPPORT_BOUNDARY,
        "aggregate": {
            "input_object_count": len(object_rows),
            "typed_input_count": object_counts.get("typed", 0),
            "typed_graph_only_input_count": object_counts.get("typed_graph_only", 0),
            "input_objects_with_first_evidence_count": sum(
                1 for row in object_rows if row["first_evidence"]
            ),
            "tracked_output_variable_count": len(variable_rows),
            "output_variables_with_first_evidence_count": sum(
                1 for row in variable_rows if row["first_evidence"]
            ),
            "output_variables_with_boundary_count": sum(
                1 for row in variable_rows if row["support_boundary"]
            ),
            "manifest_output_request_count": len(output_rows),
            "conformance_output_variable_count": variable_counts.get("conformance", 0),
            "diagnostic_output_variable_count": variable_counts.get("diagnostic", 0),
            "conformance_output_request_count": output_counts.get("conformance", 0),
            "diagnostic_output_request_count": output_counts.get("diagnostic", 0),
            "algorithm_count": len(algorithm_rows),
            "algorithms_with_first_evidence_count": sum(
                1 for row in algorithm_rows if row["first_evidence"]
            ),
            "algorithms_with_boundary_count": sum(
                1 for row in algorithm_rows if row["support_boundary"]
            ),
            "conformance_algorithm_count": algorithm_counts.get("conformance", 0),
            "diagnostic_algorithm_count": algorithm_counts.get("diagnostic_only", 0),
            "case_count": len(case_rows),
            "conformance_case_count": sum(1 for row in case_rows if row["conformance_claim"]),
            "blocking_gate_count": sum(1 for row in case_rows if row["gate_blocking"]),
            "status": "pass",
        },
        "coverage_matrix": {
            "input_status_counts": dict(sorted(object_counts.items())),
            "output_variable_status_counts": dict(sorted(variable_counts.items())),
            "output_request_status_counts": dict(sorted(output_counts.items())),
            "algorithm_status_counts": dict(sorted(algorithm_counts.items())),
            "case_domain_counts": dict(sorted(domain_counts.items())),
        },
        "input_objects": object_rows,
        "output_variables": variable_rows,
        "manifest_outputs": output_rows,
        "algorithms": algorithm_rows,
        "cases": case_rows,
        "known_gaps": [
            "No full EnergyPlus ExampleFiles compatibility claim.",
            "No HVAC, plant, meter, sizing, EMS, PythonPlugin, daylighting, fenestration, solar, or warmup conformance claim.",
            "Diagnostic and typed-graph-only rows are useful evidence, but they are not user-facing numerical conformance.",
        ],
        "artifacts": {
            "markdown": f".runtime/release-evidence/v{version}/support-coverage.md",
            "html": f".runtime/release-evidence/v{version}/support-coverage-report.html",
            "pdf": f".runtime/release-evidence/v{version}/support-coverage-report.pdf",
            "json": f".runtime/release-evidence/v{version}/support-coverage-report.json",
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


def build_metric_table(report: dict[str, Any]) -> Table:
    aggregate = report["aggregate"]
    rows = [
        ["Input objects", aggregate["input_object_count"]],
        ["Typed inputs", aggregate["typed_input_count"]],
        ["Typed graph-only inputs", aggregate["typed_graph_only_input_count"]],
        ["Tracked output variables", aggregate["tracked_output_variable_count"]],
        ["Manifest output requests", aggregate["manifest_output_request_count"]],
        ["Conformance output variables", aggregate["conformance_output_variable_count"]],
        ["Conformance output requests", aggregate["conformance_output_request_count"]],
        ["Algorithms", aggregate["algorithm_count"]],
        ["Conformance algorithms", aggregate["conformance_algorithm_count"]],
        ["Diagnostic algorithms", aggregate["diagnostic_algorithm_count"]],
        ["Tracked cases", aggregate["case_count"]],
        ["Conformance cases", aggregate["conformance_case_count"]],
        ["Blocking gates", aggregate["blocking_gate_count"]],
    ]
    return doc_table(["Metric", "Value"], rows, "User support coverage summary.")


def build_input_table(report: dict[str, Any]) -> Table:
    rows = [
        [
            row["name"],
            row["family"],
            row["user_status"],
            row["first_evidence"],
            row["support_boundary"],
        ]
        for row in report["input_objects"]
    ]
    return doc_table(
        ["Input object", "Family", "Support", "First evidence", "Boundary"],
        rows,
        "Supported input object coverage.",
    )


def build_output_table(report: dict[str, Any]) -> Table:
    rows = [
        [
            row["name"],
            row["domain"],
            row["user_status"],
            row["first_evidence"],
            ", ".join(row["observed_frequencies"]),
            ", ".join(row["observed_sources"]),
            ", ".join(row["observed_levels"]),
            ", ".join(row["best_evidence_cases"][:3]),
            row["support_boundary"],
        ]
        for row in report["output_variables"]
    ]
    return doc_table(
        [
            "Output",
            "Domain",
            "Support",
            "First evidence",
            "Freq",
            "Source",
            "Levels",
            "Best cases",
            "Boundary",
        ],
        rows,
        "Supported output variable coverage.",
    )


def build_algorithm_table(report: dict[str, Any]) -> Table:
    rows = [
        [
            row["id"],
            row["domain"],
            row["user_status"],
            row["claim_level"],
            row["first_evidence"],
            ", ".join(row["proof_variables"]),
            row["source_map"],
            row["support_boundary"],
        ]
        for row in report["algorithms"]
    ]
    return doc_table(
        [
            "Algorithm",
            "Domain",
            "Support",
            "Claim level",
            "First evidence",
            "Proof outputs",
            "Source map",
            "Boundary",
        ],
        rows,
        "Supported algorithm coverage.",
    )


def build_case_table(report: dict[str, Any]) -> Table:
    rows = [
        [
            row["case_id"],
            row["tier"],
            row["source_kind"],
            row["comparison_class"],
            str(row["conformance_claim"]).lower(),
            ", ".join(row["domains"]),
            row["gate_script"],
        ]
        for row in report["cases"]
    ]
    return doc_table(
        ["Case", "Tier", "Source", "Class", "Claim", "Domains", "Gate"],
        rows,
        "Evidence case coverage.",
    )


def build_gap_table(report: dict[str, Any]) -> Table:
    return doc_table(["Known gap"], [[gap] for gap in report["known_gaps"]], "Explicit non-support boundaries.")


def create_status_chart(report: dict[str, Any]) -> Any:
    matrix = report["coverage_matrix"]
    groups = [
        ("Inputs", matrix["input_status_counts"]),
        ("Outputs", matrix["output_variable_status_counts"]),
        ("Algorithms", matrix["algorithm_status_counts"]),
    ]

    fig, axes = plt.subplots(1, 3, figsize=(8.2, 2.8), dpi=180)
    colors = {
        "conformance": "#2f6f9f",
        "typed": "#3d7f5f",
        "diagnostic": "#c77d1a",
        "baseline": "#9b6fbd",
        "typed_graph_only": "#697789",
        "diagnostic_only": "#c77d1a",
    }

    for ax, (title, counts) in zip(axes, groups, strict=True):
        labels = list(counts.keys()) or ["none"]
        values = [counts.get(label, 0) for label in labels]
        ax.bar(labels, values, color=[colors.get(label, "#697789") for label in labels])
        ax.set_title(title, loc="left", fontsize=11, fontweight="bold", color="#17212b")
        ax.grid(axis="y", color="#e3e7ed", linewidth=0.8)
        ax.set_axisbelow(True)
        ax.spines["top"].set_visible(False)
        ax.spines["right"].set_visible(False)
        ax.spines["left"].set_color("#9aa7b5")
        ax.spines["bottom"].set_color("#9aa7b5")
        ax.tick_params(axis="x", colors="#17212b", labelsize=7, rotation=25)
        ax.tick_params(axis="y", colors="#5b6775", labelsize=7)
        for index, value in enumerate(values):
            ax.text(index, value + 0.05, str(value), ha="center", va="bottom", fontsize=7, color="#17212b")

    fig.suptitle("Support Coverage Status Counts", x=0.02, ha="left", fontsize=13, fontweight="bold")
    fig.tight_layout(pad=1.0)
    return fig


def build_document(report: dict[str, Any], chart: Any) -> Document:
    version = report["version"]
    settings = DocumentSettings(
        metadata_author="rusted-energyplus",
        subtitle="User-facing input, output, and algorithm support coverage",
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
        f"eplus-rs {version} Support Coverage Report",
        TableOfContents("Table of Contents", max_level=2),
        Chapter(
            "How To Read This Report",
            Box(
                Paragraph(
                    "Use this report to decide which input objects, output variables, and algorithm families are "
                    "currently supported, diagnostic-only, or graph-only. A row marked conformance is still limited "
                    "to the listed case, variable, tolerance, and gate."
                ),
                title="User Scope",
                border_color="#2f6f9f",
                background_color="#f4f8fb",
                padding=0.12,
            ),
        ),
        Chapter(
            "Executive Summary",
            Paragraph(
                "Generated UTC: ",
                code(report["generated_at_utc"]),
                ". EnergyPlus oracle: ",
                code(report["oracle_version"]),
                ". Schema: ",
                code(str(report["schema_version"])),
                ".",
            ),
            build_metric_table(report),
            Figure(chart, caption="Input, output, and algorithm support status counts.", width=7.2),
        ),
        Chapter("Supported Inputs", build_input_table(report)),
        Chapter("Supported Outputs", build_output_table(report)),
        Chapter("Supported Algorithms", build_algorithm_table(report)),
        Chapter("Evidence Cases", build_case_table(report)),
        Chapter("Explicit Gaps", build_gap_table(report)),
        Chapter(
            "Artifact Paths",
            doc_table(
                ["Artifact", "Path"],
                [
                    ["Markdown", report["artifacts"]["markdown"]],
                    ["HTML", report["artifacts"]["html"]],
                    ["PDF", report["artifacts"]["pdf"]],
                    ["JSON", report["artifacts"]["json"]],
                ],
                "Generated user support coverage artifacts.",
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


def render_markdown(report: dict[str, Any]) -> str:
    aggregate = report["aggregate"]
    input_rows = [
        [
            row["name"],
            row["family"],
            row["user_status"],
            row["first_evidence"],
            row["support_boundary"],
        ]
        for row in report["input_objects"]
    ]
    output_rows = [
        [
            row["name"],
            row["domain"],
            row["user_status"],
            row["first_evidence"],
            ", ".join(row["observed_frequencies"]),
            ", ".join(row["observed_sources"]),
            ", ".join(row["observed_levels"]),
            row["support_boundary"],
        ]
        for row in report["output_variables"]
    ]
    algorithm_rows = [
        [
            row["id"],
            row["domain"],
            row["user_status"],
            row["claim_level"],
            row["first_evidence"],
            ", ".join(row["proof_variables"]),
            row["support_boundary"],
        ]
        for row in report["algorithms"]
    ]
    return (
        f"# Support Coverage Report v{report['version']}\n\n"
        f"- schema_version: {report['schema_version']}\n"
        f"- oracle_version: {report['oracle_version']}\n"
        f"- generated_at_utc: {report['generated_at_utc']}\n"
        f"- support_boundary: {report['support_boundary']}\n"
        f"- input_objects: {aggregate['input_object_count']}\n"
        f"- tracked_output_variables: {aggregate['tracked_output_variable_count']}\n"
        f"- manifest_output_requests: {aggregate['manifest_output_request_count']}\n"
        f"- algorithms: {aggregate['algorithm_count']}\n"
        f"- cases: {aggregate['case_count']}\n\n"
        "## Supported Inputs\n\n"
        + markdown_table(["Input", "Family", "Support", "First evidence", "Boundary"], input_rows)
        + "\n## Supported Outputs\n\n"
        + markdown_table(
            ["Output", "Domain", "Support", "First evidence", "Freq", "Source", "Levels", "Boundary"],
            output_rows,
        )
        + "\n## Supported Algorithms\n\n"
        + markdown_table(
            ["Algorithm", "Domain", "Support", "Claim level", "First evidence", "Proof outputs", "Boundary"],
            algorithm_rows,
        )
        + "\n## Explicit Gaps\n\n"
        + "\n".join(f"- {gap}" for gap in report["known_gaps"])
        + "\n"
    )


def write_outputs(repo_root: Path, version: str, report: dict[str, Any]) -> dict[str, Path]:
    evidence_root = repo_root / ".runtime" / "release-evidence" / f"v{version}"
    evidence_root.mkdir(parents=True, exist_ok=True)
    chart = create_status_chart(report)
    try:
        document = build_document(report, chart)

        json_path = evidence_root / "support-coverage-report.json"
        html_path = evidence_root / "support-coverage-report.html"
        pdf_path = evidence_root / "support-coverage-report.pdf"
        markdown_path = evidence_root / "support-coverage.md"

        json_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
        markdown_path.write_text(render_markdown(report), encoding="utf-8")
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
    report = build_support_coverage(repo_root, args.version)
    outputs = write_outputs(repo_root, args.version, report)

    aggregate = report["aggregate"]
    print("Support coverage report")
    print(f"  status: {aggregate['status']}")
    print(f"  input_objects: {aggregate['input_object_count']}")
    print(f"  output_variables: {aggregate['tracked_output_variable_count']}")
    print(f"  output_requests: {aggregate['manifest_output_request_count']}")
    print(f"  algorithms: {aggregate['algorithm_count']}")
    print(f"  cases: {aggregate['case_count']}")
    print(f"  markdown: {outputs['markdown']}")
    print(f"  html: {outputs['html']}")
    print(f"  pdf: {outputs['pdf']}")
    print(f"  json: {outputs['json']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
