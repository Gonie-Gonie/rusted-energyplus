from __future__ import annotations

import argparse
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from oodocs import (
    Box,
    Chapter,
    Document,
    DocumentSettings,
    PageMargins,
    Paragraph,
    Table,
    TableOfContents,
    Theme,
    code,
)


CLAIM_BOUNDARY = (
    "This manifest records release package and evidence assets. It does not "
    "add or promote numerical conformance by itself."
)
GITHUB_RELEASE_POLICY = (
    "The release zip and every generated file under "
    ".runtime/release-evidence/vX.Y.Z are intended to be GitHub Release assets."
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the release evidence asset manifest.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.32.0")
    parser.add_argument("--target", default="windows-x64")
    return parser.parse_args()


def rel_path(repo_root: Path, path: Path) -> str:
    return path.relative_to(repo_root).as_posix()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def content_type(path: Path) -> str:
    match path.suffix.lower():
        case ".zip":
            return "application/zip"
        case ".pdf":
            return "application/pdf"
        case ".html":
            return "text/html"
        case ".json":
            return "application/json"
        case ".md":
            return "text/markdown"
        case _:
            return "application/octet-stream"


def load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {}
    return json.loads(path.read_text(encoding="utf-8"))


def evidence_path(repo_root: Path, version: str, name: str) -> Path:
    return repo_root / ".runtime" / "release-evidence" / f"v{version}" / name


def minor_version(version: str) -> int:
    parts = version.split(".")
    if len(parts) < 2:
        return 0
    try:
        return int(parts[1])
    except ValueError:
        return 0


def expected_asset_specs(repo_root: Path, version: str, target: str) -> list[dict[str, Any]]:
    package = repo_root / "dist" / f"eplus-rs-v{version}-{target}.zip"
    specs = [
        {
            "role": "binary-package",
            "path": package,
            "produced_by": f".\\scripts\\dev.cmd package -Version {version}",
            "user_purpose": "Runnable CLI package with docs, specs, scripts, and test data.",
        },
        {
            "role": "numeric-conformance-pdf",
            "path": evidence_path(repo_root, version, "numeric-conformance-evidence.pdf"),
            "produced_by": f".\\scripts\\dev.cmd conformance-evidence-report -Version {version}",
            "user_purpose": "Readable numerical conformance evidence for promoted tolerance-gated cases.",
        },
        {
            "role": "numeric-conformance-html",
            "path": evidence_path(repo_root, version, "numeric-conformance-evidence.html"),
            "produced_by": f".\\scripts\\dev.cmd conformance-evidence-report -Version {version}",
            "user_purpose": "Browser-readable numerical conformance evidence.",
        },
        {
            "role": "numeric-conformance-json",
            "path": evidence_path(repo_root, version, "numeric-conformance-evidence.json"),
            "produced_by": f".\\scripts\\dev.cmd conformance-evidence-report -Version {version}",
            "user_purpose": "Machine-readable numerical conformance aggregate and series detail.",
        },
        {
            "role": "conformance-index-pdf",
            "path": evidence_path(repo_root, version, "conformance-index-report.pdf"),
            "produced_by": f".\\scripts\\dev.cmd conformance-index-report -Version {version}",
            "user_purpose": "Readable manifest, case, and output-request coverage index.",
        },
        {
            "role": "conformance-index-html",
            "path": evidence_path(repo_root, version, "conformance-index-report.html"),
            "produced_by": f".\\scripts\\dev.cmd conformance-index-report -Version {version}",
            "user_purpose": "Browser-readable conformance coverage index.",
        },
        {
            "role": "conformance-index-json",
            "path": evidence_path(repo_root, version, "conformance-index-report.json"),
            "produced_by": f".\\scripts\\dev.cmd conformance-index-report -Version {version}",
            "user_purpose": "Machine-readable case, domain, output, and gate coverage matrix.",
        },
        {
            "role": "conformance-index-markdown",
            "path": evidence_path(repo_root, version, "conformance-index.md"),
            "produced_by": f".\\scripts\\dev.cmd conformance-index-report -Version {version}",
            "user_purpose": "Lightweight text copy of the conformance index.",
        },
        {
            "role": "support-coverage-pdf",
            "path": evidence_path(repo_root, version, "support-coverage-report.pdf"),
            "produced_by": f".\\scripts\\dev.cmd support-coverage-report -Version {version}",
            "user_purpose": "Readable user coverage for supported inputs, outputs, and algorithms.",
        },
        {
            "role": "support-coverage-html",
            "path": evidence_path(repo_root, version, "support-coverage-report.html"),
            "produced_by": f".\\scripts\\dev.cmd support-coverage-report -Version {version}",
            "user_purpose": "Browser-readable support coverage report.",
        },
        {
            "role": "support-coverage-json",
            "path": evidence_path(repo_root, version, "support-coverage-report.json"),
            "produced_by": f".\\scripts\\dev.cmd support-coverage-report -Version {version}",
            "user_purpose": "Machine-readable input, output, algorithm, and case support coverage.",
        },
        {
            "role": "support-coverage-markdown",
            "path": evidence_path(repo_root, version, "support-coverage.md"),
            "produced_by": f".\\scripts\\dev.cmd support-coverage-report -Version {version}",
            "user_purpose": "Lightweight text copy of the user support coverage report.",
        },
    ]
    if minor_version(version) >= 32:
        specs.extend(
            [
                {
                    "role": "user-coverage-handbook-pdf",
                    "path": evidence_path(repo_root, version, "user-coverage-handbook.pdf"),
                    "produced_by": f".\\scripts\\dev.cmd user-coverage-handbook -Version {version}",
                    "user_purpose": "Readable user guide to currently supported inputs, outputs, and algorithms.",
                },
                {
                    "role": "user-coverage-handbook-html",
                    "path": evidence_path(repo_root, version, "user-coverage-handbook.html"),
                    "produced_by": f".\\scripts\\dev.cmd user-coverage-handbook -Version {version}",
                    "user_purpose": "Browser-readable user coverage handbook.",
                },
                {
                    "role": "user-coverage-handbook-json",
                    "path": evidence_path(repo_root, version, "user-coverage-handbook.json"),
                    "produced_by": f".\\scripts\\dev.cmd user-coverage-handbook -Version {version}",
                    "user_purpose": "Machine-readable user coverage decision rules and scope slices.",
                },
                {
                    "role": "user-coverage-handbook-markdown",
                    "path": evidence_path(repo_root, version, "user-coverage-handbook.md"),
                    "produced_by": f".\\scripts\\dev.cmd user-coverage-handbook -Version {version}",
                    "user_purpose": "Lightweight text copy of the user coverage handbook.",
                },
            ]
        )
    return specs


def materialize_asset(repo_root: Path, spec: dict[str, Any]) -> dict[str, Any]:
    path = Path(spec["path"])
    exists = path.exists() and path.is_file()
    return {
        "role": spec["role"],
        "path": rel_path(repo_root, path) if path.is_absolute() else path.as_posix(),
        "github_release_asset": True,
        "required": True,
        "exists": exists,
        "size_bytes": path.stat().st_size if exists else 0,
        "sha256": sha256_file(path) if exists else "",
        "content_type": content_type(path),
        "produced_by": spec["produced_by"],
        "user_purpose": spec["user_purpose"],
    }


def build_report_summaries(repo_root: Path, version: str) -> dict[str, Any]:
    numeric = load_json(evidence_path(repo_root, version, "numeric-conformance-evidence.json"))
    index = load_json(evidence_path(repo_root, version, "conformance-index-report.json"))
    support = load_json(evidence_path(repo_root, version, "support-coverage-report.json"))
    handbook = load_json(evidence_path(repo_root, version, "user-coverage-handbook.json"))

    numeric_aggregate = numeric.get("aggregate") or {}
    index_aggregate = index.get("aggregate") or {}
    support_aggregate = support.get("aggregate") or {}
    handbook_aggregate = handbook.get("aggregate") or {}
    return {
        "numeric_conformance": {
            "status": numeric_aggregate.get("status", "missing"),
            "cases": numeric_aggregate.get("case_count", 0),
            "series": numeric_aggregate.get("series_count", 0),
            "max_abs_delta_c": numeric_aggregate.get("max_abs_delta_c", 0),
            "max_rmse_delta_c": numeric_aggregate.get("rmse_delta_c", 0),
        },
        "conformance_index": {
            "status": index_aggregate.get("status", "missing"),
            "cases": index_aggregate.get("case_count", 0),
            "conformance_cases": index_aggregate.get("conformance_case_count", 0),
            "outputs": index_aggregate.get("output_count", 0),
            "meters": index_aggregate.get("meter_count", 0),
        },
        "support_coverage": {
            "status": support_aggregate.get("status", "missing"),
            "input_objects": support_aggregate.get("input_object_count", 0),
            "output_variables": support_aggregate.get("tracked_output_variable_count", 0),
            "output_requests": support_aggregate.get("manifest_output_request_count", 0),
            "algorithms": support_aggregate.get("algorithm_count", 0),
        },
        "user_coverage_handbook": {
            "status": handbook_aggregate.get("status", "missing"),
            "typed_inputs": handbook_aggregate.get("typed_input_count", 0),
            "conformance_outputs": handbook_aggregate.get("conformance_output_variable_count", 0),
            "conformance_algorithms": handbook_aggregate.get("conformance_algorithm_count", 0),
        },
    }


def build_manifest(repo_root: Path, version: str, target: str) -> dict[str, Any]:
    assets = [materialize_asset(repo_root, spec) for spec in expected_asset_specs(repo_root, version, target)]
    missing = [asset for asset in assets if not asset["exists"]]
    total_size = sum(asset["size_bytes"] for asset in assets)
    return {
        "schema_version": 1,
        "version": version,
        "target": target,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "claim_boundary": CLAIM_BOUNDARY,
        "github_release_policy": GITHUB_RELEASE_POLICY,
        "aggregate": {
            "required_asset_count": len(assets),
            "present_required_asset_count": len(assets) - len(missing),
            "missing_required_asset_count": len(missing),
            "total_required_asset_size_bytes": total_size,
            "status": "pass" if not missing else "fail",
        },
        "assets": assets,
        "report_summaries": build_report_summaries(repo_root, version),
        "artifacts": {
            "markdown": f".runtime/release-evidence/v{version}/release-evidence-manifest.md",
            "html": f".runtime/release-evidence/v{version}/release-evidence-manifest.html",
            "pdf": f".runtime/release-evidence/v{version}/release-evidence-manifest.pdf",
            "json": f".runtime/release-evidence/v{version}/release-evidence-manifest.json",
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


def size_label(size: int) -> str:
    if size >= 1024 * 1024:
        return f"{size / (1024 * 1024):.2f} MiB"
    if size >= 1024:
        return f"{size / 1024:.1f} KiB"
    return f"{size} B"


def build_metric_table(manifest: dict[str, Any]) -> Table:
    aggregate = manifest["aggregate"]
    rows = [
        ["Required release assets", aggregate["required_asset_count"]],
        ["Present assets", aggregate["present_required_asset_count"]],
        ["Missing assets", aggregate["missing_required_asset_count"]],
        ["Total required asset size", size_label(aggregate["total_required_asset_size_bytes"])],
        ["Manifest status", aggregate["status"]],
    ]
    return doc_table(["Metric", "Value"], rows, "Release asset checklist summary.")


def build_asset_table(manifest: dict[str, Any]) -> Table:
    rows = [
        [
            asset["role"],
            asset["path"],
            "yes" if asset["exists"] else "missing",
            size_label(asset["size_bytes"]),
            asset["sha256"][:16] if asset["sha256"] else "",
            asset["user_purpose"],
        ]
        for asset in manifest["assets"]
    ]
    return doc_table(
        ["Role", "Path", "Exists", "Size", "SHA256 prefix", "User purpose"],
        rows,
        "Expected release package and GitHub Release evidence assets.",
    )


def build_summary_table(manifest: dict[str, Any]) -> Table:
    summaries = manifest["report_summaries"]
    rows = [
        [
            "Numeric conformance",
            summaries["numeric_conformance"]["status"],
            f"{summaries['numeric_conformance']['cases']} cases, "
            f"{summaries['numeric_conformance']['series']} series",
        ],
        [
            "Conformance index",
            summaries["conformance_index"]["status"],
            f"{summaries['conformance_index']['cases']} cases, "
            f"{summaries['conformance_index']['outputs']} outputs",
        ],
        [
            "Support coverage",
            summaries["support_coverage"]["status"],
            f"{summaries['support_coverage']['input_objects']} inputs, "
            f"{summaries['support_coverage']['output_variables']} outputs, "
            f"{summaries['support_coverage']['algorithms']} algorithms",
        ],
    ]
    handbook_summary = summaries.get("user_coverage_handbook") or {}
    if handbook_summary.get("status") != "missing":
        rows.append(
            [
                "User coverage handbook",
                handbook_summary["status"],
                f"{handbook_summary['typed_inputs']} typed inputs, "
                f"{handbook_summary['conformance_outputs']} conformance outputs, "
                f"{handbook_summary['conformance_algorithms']} conformance algorithms",
            ]
        )
    return doc_table(["Report", "Status", "Scope summary"], rows, "Summaries read from generated JSON evidence.")


def build_document(manifest: dict[str, Any]) -> Document:
    version = manifest["version"]
    settings = DocumentSettings(
        metadata_author="rusted-energyplus",
        subtitle="Release package and evidence asset manifest",
        cover_page=True,
        page_margins=PageMargins(0.55, 0.55, 0.55, 0.55, unit="in"),
        theme=Theme(
            body_font_name="Segoe UI",
            monospace_font_name="Consolas",
            body_font_size=8.8,
            heading_sizes=(20, 16, 13, 11),
            table_alignment="center",
            show_page_numbers=True,
            page_number_alignment="center",
        ),
    )
    return Document(
        f"eplus-rs {version} Release Evidence Manifest",
        TableOfContents("Table of Contents", max_level=2),
        Chapter(
            "Release Asset Policy",
            Box(
                Paragraph(
                    "Use this manifest as the release asset checklist. It records the package, numerical "
                    "evidence, conformance index, and support coverage files that should accompany the "
                    "GitHub Release."
                ),
                title="GitHub Release Asset Manifest",
                border_color="#2f6f9f",
                background_color="#f4f8fb",
                padding=0.12,
            ),
            Paragraph("Claim boundary: ", code(manifest["claim_boundary"])),
            Paragraph("Upload policy: ", code(manifest["github_release_policy"])),
        ),
        Chapter(
            "Executive Summary",
            Paragraph(
                "Generated UTC: ",
                code(manifest["generated_at_utc"]),
                ". Target: ",
                code(manifest["target"]),
                ". Schema: ",
                code(str(manifest["schema_version"])),
                ".",
            ),
            build_metric_table(manifest),
            build_summary_table(manifest),
        ),
        Chapter("Asset Checklist", build_asset_table(manifest)),
        Chapter(
            "How Users Should Read Assets",
            doc_table(
                ["Asset family", "Primary user question"],
                [
                    ["Binary package", "Can I run this release locally with its packaged docs and specs?"],
                    ["Numeric evidence", "Which promoted outputs match the oracle under declared tolerances?"],
                    ["Conformance index", "Which cases, outputs, domains, and gates are tracked?"],
                    ["Support coverage", "Which inputs, outputs, and algorithm families are supported, diagnostic, or graph-only?"],
                    ["User coverage handbook", "How should users decide whether their IDF, requested outputs, and algorithms are in scope?"],
                ],
                "User-facing purpose of each release asset family.",
            ),
        ),
        Chapter(
            "Manifest Outputs",
            doc_table(
                ["Artifact", "Path"],
                [
                    ["Markdown", manifest["artifacts"]["markdown"]],
                    ["HTML", manifest["artifacts"]["html"]],
                    ["PDF", manifest["artifacts"]["pdf"]],
                    ["JSON", manifest["artifacts"]["json"]],
                ],
                "Generated release evidence manifest artifacts.",
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


def render_markdown(manifest: dict[str, Any]) -> str:
    aggregate = manifest["aggregate"]
    asset_rows = [
        [
            asset["role"],
            asset["path"],
            "yes" if asset["exists"] else "missing",
            asset["size_bytes"],
            asset["sha256"],
            asset["user_purpose"],
        ]
        for asset in manifest["assets"]
    ]
    return (
        f"# Release Evidence Manifest v{manifest['version']}\n\n"
        f"- schema_version: {manifest['schema_version']}\n"
        f"- target: {manifest['target']}\n"
        f"- generated_at_utc: {manifest['generated_at_utc']}\n"
        f"- claim_boundary: {manifest['claim_boundary']}\n"
        f"- github_release_policy: {manifest['github_release_policy']}\n"
        f"- required_assets: {aggregate['required_asset_count']}\n"
        f"- present_assets: {aggregate['present_required_asset_count']}\n"
        f"- missing_assets: {aggregate['missing_required_asset_count']}\n"
        f"- status: {aggregate['status']}\n\n"
        "## Asset Checklist\n\n"
        + markdown_table(["Role", "Path", "Exists", "Size bytes", "SHA256", "Purpose"], asset_rows)
    )


def write_outputs(repo_root: Path, version: str, manifest: dict[str, Any]) -> dict[str, Path]:
    evidence_root = repo_root / ".runtime" / "release-evidence" / f"v{version}"
    evidence_root.mkdir(parents=True, exist_ok=True)
    document = build_document(manifest)

    json_path = evidence_root / "release-evidence-manifest.json"
    html_path = evidence_root / "release-evidence-manifest.html"
    pdf_path = evidence_root / "release-evidence-manifest.pdf"
    markdown_path = evidence_root / "release-evidence-manifest.md"

    json_path.write_text(json.dumps(manifest, indent=2), encoding="utf-8")
    markdown_path.write_text(render_markdown(manifest), encoding="utf-8")
    document.save_html(html_path)
    document.save_pdf(pdf_path)

    return {
        "json": json_path,
        "html": html_path,
        "pdf": pdf_path,
        "markdown": markdown_path,
    }


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    manifest = build_manifest(repo_root, args.version, args.target)
    outputs = write_outputs(repo_root, args.version, manifest)

    aggregate = manifest["aggregate"]
    print("Release evidence manifest")
    print(f"  status: {aggregate['status']}")
    print(f"  required_assets: {aggregate['required_asset_count']}")
    print(f"  present_assets: {aggregate['present_required_asset_count']}")
    print(f"  missing_assets: {aggregate['missing_required_asset_count']}")
    print(f"  markdown: {outputs['markdown']}")
    print(f"  html: {outputs['html']}")
    print(f"  pdf: {outputs['pdf']}")
    print(f"  json: {outputs['json']}")
    return 0 if aggregate["status"] == "pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
