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
EXAMPLEFILE_SOURCE_KIND = "energy-plus-examplefile"
NON_ACTIVE_TARGET_ROLES = {
    "already-gated",
    "not-a-dynamic-physics-target",
    "static-model-only",
}


CASE_TARGETS = {
    "construction_materials_001": (
        "static-input-fixture",
        "not-a-dynamic-physics-target",
        "keep as smoke/static intake evidence unless a runtime material-state output is added",
    ),
    "surface_geometry_001": (
        "static-input-fixture",
        "not-a-dynamic-physics-target",
        "keep as smoke/static geometry evidence unless a runtime geometry-derived output is added",
    ),
    "heat_balance_nomass_001": (
        "heat-balance",
        "already-gated",
        "maintain declared no-mass MAT conformance while the transient CTF tracker advances",
    ),
    "surface_temperature_nomass_001": (
        "heat-balance",
        "already-gated",
        "maintain declared no-mass surface-state conformance while transient CTF parity advances",
    ),
    "ideal_loads_thermostat_001": (
        "hvac-ideal-loads",
        "system-scope-diagnostic",
        "promote only after IdealLoads thermal response and node flow values have source-mapped parity",
    ),
    "air_side_node_diagnostic_001": (
        "hvac-node",
        "system-scope-diagnostic",
        "promote only after node projection values have source-mapped runtime parity",
    ),
    "plant_loop_diagnostic_001": (
        "plant-loop",
        "system-scope-diagnostic",
        "promote only after plant loop mass-flow, temperature, and equipment rates are simulated",
    ),
    "official_1zone_uncontrolled_baseline_001": (
        "official-examplefile-heat-balance",
        "blocked-by-official-dynamic-tracker",
        "use the official dynamic 1Zone diagnostic before promoting ExampleFile hourly variables",
    ),
    "schedule_constant_001": (
        "time-schedule",
        "already-gated",
        "maintain declared Schedule Value conformance",
    ),
    "weather_fields_001": (
        "weather",
        "already-gated",
        "maintain dry-bulb conformance; promote additional weather fields only with tolerances",
    ),
    "official_1zone_static_model_001": (
        "official-examplefile-static",
        "static-model-only",
        "keep static EIO conformance separate from the dynamic ExampleFile target",
    ),
    "internal_gains_001": (
        "internal-gain",
        "already-gated",
        "maintain convective-gain trace conformance; radiant/latent response remains future work",
    ),
}


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
    target_domain: str
    target_role: str
    next_action: str
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


def target_metadata(case_id: str, status: str, gap: str) -> tuple[str, str, str]:
    target = CASE_TARGETS.get(case_id)
    if target is not None:
        return target
    if status == "dynamic-conformance-gated":
        return ("runtime", "already-gated", "maintain existing blocking gate")
    if status == "dynamic-diagnostic-only":
        return ("runtime", "promotion-candidate", gap)
    return ("runtime", "unclassified-gap", gap)


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
    case_id = str(payload.get("id", case_path.parent.name))
    target_domain, target_role, next_action = target_metadata(case_id, status, gap)

    return CaseRow(
        case_id=case_id,
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
        target_domain=target_domain,
        target_role=target_role,
        next_action=next_action,
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
    target_role_counts: dict[str, int] = {}
    target_domain_counts: dict[str, int] = {}
    source_kind_counts: dict[str, int] = {}
    for row in rows:
        status_counts[row.status] = status_counts.get(row.status, 0) + 1
        target_role_counts[row.target_role] = (
            target_role_counts.get(row.target_role, 0) + 1
        )
        target_domain_counts[row.target_domain] = (
            target_domain_counts.get(row.target_domain, 0) + 1
        )
        source_kind = row.source_kind or "unspecified"
        source_kind_counts[source_kind] = source_kind_counts.get(source_kind, 0) + 1

    active_dynamic_gap_rows = [
        row
        for row in rows
        if row.dynamic_output_count > 0
        and row.status != "dynamic-conformance-gated"
        and row.target_role not in NON_ACTIVE_TARGET_ROLES
    ]
    examplefile_rows = [
        row for row in rows if row.source_kind == EXAMPLEFILE_SOURCE_KIND
    ]
    examplefile_dynamic_rows = [
        row for row in examplefile_rows if row.dynamic_output_count > 0
    ]

    return {
        "schema": "rusted-energyplus.v026-dynamic-idf-inventory.v3",
        "cutoff_version": f"v{CUTOFF_VERSION[0]}.{CUTOFF_VERSION[1]}",
        "case_count": len(rows),
        "dynamic_conformance_gated_count": status_counts.get(
            "dynamic-conformance-gated", 0
        ),
        "gap_count": len(rows)
        - status_counts.get("dynamic-conformance-gated", 0),
        "active_dynamic_gap_count": len(active_dynamic_gap_rows),
        "energyplus_examplefile_case_count": len(examplefile_rows),
        "energyplus_examplefile_dynamic_candidate_count": len(
            examplefile_dynamic_rows
        ),
        "energyplus_examplefile_dynamic_gated_count": len(
            [
                row
                for row in examplefile_dynamic_rows
                if row.status == "dynamic-conformance-gated"
            ]
        ),
        "status_counts": status_counts,
        "target_role_counts": target_role_counts,
        "target_domain_counts": target_domain_counts,
        "source_kind_counts": source_kind_counts,
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
        f"active_dynamic_gap_count: {inventory['active_dynamic_gap_count']}",
        f"energyplus_examplefile_dynamic_candidate_count: {inventory['energyplus_examplefile_dynamic_candidate_count']}",
        f"energyplus_examplefile_dynamic_gated_count: {inventory['energyplus_examplefile_dynamic_gated_count']}",
        "",
        "## Target Roles",
        "",
        "| role | count |",
        "|---|---:|",
    ]
    for role, count in sorted(inventory["target_role_counts"].items()):
        lines.append(f"| {role} | {count} |")
    lines.extend(
        [
            "",
            "## Source Kinds",
            "",
            "| source kind | count |",
            "|---|---:|",
        ]
    )
    for source_kind, count in sorted(inventory["source_kind_counts"].items()):
        lines.append(f"| {source_kind} | {count} |")
    lines.extend(
        [
            "",
            "## Cases",
            "",
            "| case | milestone | source | IDF | domain | role | dynamic outputs | dynamic conformance | status | next action |",
            "|---|---|---|---|---|---|---:|---:|---|---|",
        ]
    )
    for row in inventory["cases"]:
        lines.append(
            "| {case_id} | {milestone} | {source_kind} | {idf} | {target_domain} | {target_role} | {dynamic_output_count} | {dynamic_conformance_output_count} | {status} | {next_action} |".format(
                case_id=row["case_id"],
                milestone=row["milestone"],
                source_kind=row["source_kind"] or "unspecified",
                idf=row["idf"],
                target_domain=row["target_domain"],
                target_role=row["target_role"],
                dynamic_output_count=row["dynamic_output_count"],
                dynamic_conformance_output_count=row[
                    "dynamic_conformance_output_count"
                ],
                status=row["status"],
                next_action=row["next_action"],
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
