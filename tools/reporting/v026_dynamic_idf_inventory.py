from __future__ import annotations

import argparse
import json
import re
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Any


CUTOFF_VERSION = (0, 26)
CASE_ROOT = Path("data/conformance_cases")
DYNAMIC_FREQUENCIES = {"detailed", "timestep", "hourly", "daily", "monthly", "runperiod"}


@dataclass(frozen=True)
class CaseRow:
    case_id: str
    milestone: str
    milestone_version: tuple[int, int] | None
    comparison_class: str
    conformance_claim: bool
    source_kind: str
    source_file: str
    idf: str
    dynamic_output_count: int
    dynamic_conformance_output_count: int
    dynamic_diagnostic_output_count: int
    gate_script: str
    gate_blocking: bool
    status: str
    gap: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Inventory IDF-backed dynamic conformance status through v0.26."
    )
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument(
        "--json-output",
        type=Path,
        default=Path(".runtime/v026-dynamic-idf-inventory.json"),
    )
    parser.add_argument(
        "--markdown-output",
        type=Path,
        default=Path(".runtime/v026-dynamic-idf-inventory.md"),
    )
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        payload = tomllib.load(handle)
    if not isinstance(payload, dict):
        raise ValueError(f"Expected TOML object at {path}")
    return payload


def milestone_version(milestone: str) -> tuple[int, int] | None:
    match = re.match(r"v(\d+)\.(\d+)(?:\b|-)", milestone.strip())
    if not match:
        return None
    return (int(match.group(1)), int(match.group(2)))


def is_dynamic_output(output: dict[str, Any]) -> bool:
    frequency = str(output.get("frequency", "")).strip().lower()
    source = str(output.get("source", "")).strip().lower()
    return source == "eso" or frequency in DYNAMIC_FREQUENCIES


def classify_case(
    *,
    comparison_class: str,
    conformance_claim: bool,
    dynamic_output_count: int,
    dynamic_conformance_output_count: int,
    dynamic_diagnostic_output_count: int,
    gate_blocking: bool,
) -> tuple[str, str]:
    if dynamic_conformance_output_count > 0 and conformance_claim and gate_blocking:
        return ("dynamic-conformance-gated", "none")
    if dynamic_conformance_output_count > 0:
        return (
            "dynamic-conformance-ungated",
            "add or verify blocking gate before this can count toward the target",
        )
    if dynamic_diagnostic_output_count > 0 or dynamic_output_count > 0:
        return (
            "dynamic-diagnostic-only",
            "promote selected dynamic variables with tolerances and a blocking gate",
        )
    if comparison_class == "conformance" and conformance_claim:
        return (
            "static-or-non-eso-conformance",
            "add dynamic ESO variables and a generated comparison report",
        )
    return (
        "no-dynamic-conformance",
        "add dynamic output requests, oracle comparison, tolerances, and a blocking gate",
    )


def case_row(case_path: Path) -> CaseRow | None:
    payload = load_toml(case_path)
    milestone = str(payload.get("milestone", ""))
    version = milestone_version(milestone)
    if version is None or version > CUTOFF_VERSION:
        return None

    input_block = payload.get("input", {})
    if not isinstance(input_block, dict):
        return None
    idf = str(input_block.get("idf", "")).strip()
    if not idf:
        return None

    manifest = payload.get("manifest_v2", {})
    manifest = manifest if isinstance(manifest, dict) else {}
    outputs = payload.get("outputs", [])
    outputs = outputs if isinstance(outputs, list) else []
    dynamic_outputs = [
        output
        for output in outputs
        if isinstance(output, dict) and is_dynamic_output(output)
    ]
    dynamic_conformance_outputs = [
        output
        for output in dynamic_outputs
        if str(output.get("level", "")).strip().lower() == "conformance"
    ]
    dynamic_diagnostic_outputs = [
        output
        for output in dynamic_outputs
        if str(output.get("level", "")).strip().lower() == "diagnostic"
    ]
    gate = payload.get("gate", {})
    gate = gate if isinstance(gate, dict) else {}
    comparison_class = str(payload.get("comparison_class", ""))
    conformance_claim = bool(payload.get("conformance_claim", False))
    gate_blocking = bool(gate.get("blocking", False))
    status, gap = classify_case(
        comparison_class=comparison_class,
        conformance_claim=conformance_claim,
        dynamic_output_count=len(dynamic_outputs),
        dynamic_conformance_output_count=len(dynamic_conformance_outputs),
        dynamic_diagnostic_output_count=len(dynamic_diagnostic_outputs),
        gate_blocking=gate_blocking,
    )

    return CaseRow(
        case_id=str(payload.get("id", case_path.parent.name)),
        milestone=milestone,
        milestone_version=version,
        comparison_class=comparison_class,
        conformance_claim=conformance_claim,
        source_kind=str(manifest.get("source_kind", "")),
        source_file=str(manifest.get("source_file", "")),
        idf=idf,
        dynamic_output_count=len(dynamic_outputs),
        dynamic_conformance_output_count=len(dynamic_conformance_outputs),
        dynamic_diagnostic_output_count=len(dynamic_diagnostic_outputs),
        gate_script=str(gate.get("script", "")),
        gate_blocking=gate_blocking,
        status=status,
        gap=gap,
    )


def build_inventory(repo_root: Path) -> dict[str, Any]:
    case_paths = sorted((repo_root / CASE_ROOT).glob("*/case.toml"))
    rows = [row for path in case_paths if (row := case_row(path)) is not None]
    rows.sort(key=lambda row: (row.milestone_version or (999, 999), row.case_id))
    status_counts: dict[str, int] = {}
    for row in rows:
        status_counts[row.status] = status_counts.get(row.status, 0) + 1

    return {
        "schema": "rusted-energyplus.v026-dynamic-idf-inventory.v1",
        "cutoff_version": f"v{CUTOFF_VERSION[0]}.{CUTOFF_VERSION[1]}",
        "case_count": len(rows),
        "dynamic_conformance_gated_count": status_counts.get(
            "dynamic-conformance-gated", 0
        ),
        "gap_count": len(rows)
        - status_counts.get("dynamic-conformance-gated", 0),
        "status_counts": status_counts,
        "cases": [row.__dict__ for row in rows],
    }


def render_markdown(inventory: dict[str, Any]) -> str:
    lines = [
        "# v0.26 Dynamic IDF Inventory",
        "",
        f"cutoff_version: {inventory['cutoff_version']}",
        f"case_count: {inventory['case_count']}",
        f"dynamic_conformance_gated_count: {inventory['dynamic_conformance_gated_count']}",
        f"gap_count: {inventory['gap_count']}",
        "",
        "| case | milestone | comparison | claim | dynamic outputs | dynamic conformance | gate | status | gap |",
        "|---|---|---|---:|---:|---:|---|---|---|",
    ]
    for row in inventory["cases"]:
        lines.append(
            "| {case_id} | {milestone} | {comparison_class} | {claim} | {dynamic_output_count} | {dynamic_conformance_output_count} | {gate} | {status} | {gap} |".format(
                case_id=row["case_id"],
                milestone=row["milestone"],
                comparison_class=row["comparison_class"],
                claim=str(row["conformance_claim"]).lower(),
                dynamic_output_count=row["dynamic_output_count"],
                dynamic_conformance_output_count=row[
                    "dynamic_conformance_output_count"
                ],
                gate=row["gate_script"] or "none",
                status=row["status"],
                gap=row["gap"],
            )
        )
    lines.append("")
    lines.append(
        "This inventory is a planning aid. It does not promote any case or create a conformance claim."
    )
    return "\n".join(lines) + "\n"


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def main() -> None:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    inventory = build_inventory(repo_root)
    json_payload = json.dumps(inventory, indent=2, sort_keys=True)
    write_text(args.json_output if args.json_output.is_absolute() else repo_root / args.json_output, json_payload + "\n")
    markdown = render_markdown(inventory)
    write_text(
        args.markdown_output
        if args.markdown_output.is_absolute()
        else repo_root / args.markdown_output,
        markdown,
    )
    print(markdown)


if __name__ == "__main__":
    main()
