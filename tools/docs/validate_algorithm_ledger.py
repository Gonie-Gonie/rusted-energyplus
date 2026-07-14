"""Validate the source-map, algorithm ledger, and port-ticket contracts."""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
from pathlib import Path
from typing import Any

from routine_completion_contract import (
    ROUTINE_COMPLETION_RANK,
    validate_domain_completion_contract,
)


ALLOWED_STATUS = {
    "source_mapped",
    "scaffold",
    "diagnostic_only",
    "conformance",
    "superseded",
}
ROUTINE_EVIDENCE_DOMAINS = {
    "heat_balance": {"heat_balance", "surface", "zone"},
    "hvac": {"hvac", "node"},
    "plant": {"plant"},
    "time": {"time", "schedule", "weather"},
}
STATE_CONTRACT_FIELDS = (
    "read_state",
    "write_state",
    "history_state_ownership",
    "unsupported_state",
    "inactive_branches",
    "unsupported_active_branches",
    "not_claimed_branches",
)
ALLOWED_PORT_TYPES = {"compatibility", "diagnostic_probe", "refactor_only"}
PORT_TICKET_REQUIRED_FIELDS = {
    "algorithm_port_ticket": ["algorithm_id", "domain", "port_type"],
    "energyplus": ["version", "source_file", "routine", "source_order_stage"],
    "rust": [
        "target_module",
        "target_function",
        "execution_stage_kind",
        "compatibility_path",
        "diagnostic_probe_used",
    ],
    "state_mapping": [
        "input_state",
        "output_state",
        "history_state_ownership",
        "unsupported_state",
        "inactive_branches",
        "unsupported_active_branches",
    ],
    "outputs": ["affected_variables", "affected_meters", "diagnostic_only_variables"],
    "evidence": ["first_target_case", "proof_variables", "tolerance_candidate", "report_path", "blocking_gate"],
    "claim_boundary": ["conformance_claim", "not_claimed_branches", "partial_run_allowed"],
}
PR_TEMPLATE_REQUIRED_TOKENS = [
    "Ticket path or PR section:",
    "Algorithm ID:",
    "Domain:",
    "Port type:",
    "EnergyPlus version:",
    "EnergyPlus source file:",
    "EnergyPlus routine:",
    "EnergyPlus source-order stage:",
    "Rust target module:",
    "Rust target function:",
    "ExecutionStageKind:",
    "Compatibility path:",
    "Diagnostic probe used:",
    "Read state:",
    "Write state:",
    "History/state ownership:",
    "Unsupported state:",
    "Inactive branches:",
    "Unsupported active branches:",
    "Affected variables:",
    "Affected meters:",
    "Diagnostic-only variables:",
    "First target case:",
    "Proof variables:",
    "Tolerance candidate:",
    "Report path:",
    "Blocking gate:",
    "Conformance claim:",
    "Not-claimed branches:",
    "Partial run allowed:",
]
PORT_TICKET_DOC_TOKENS = [
    "specs/algorithm_port_ticket_template.toml",
    "port_type = \"diagnostic_probe\"",
    "claim_boundary.conformance_claim = false",
    "state_mapping.inactive_branches",
    "state_mapping.unsupported_active_branches",
    "Compatibility code must not call diagnostic probe functions.",
    "port_ticket_mappings",
    "full gate invocation",
    "one Algorithm ID",
]
PR_WORKFLOW_REQUIRED_TOKENS = [
    "pull_request:",
    "Algorithm Port Ticket",
    "pr-port-ticket-check",
    "fetch-depth: 0",
    "ref: ${{ github.event.pull_request.head.sha }}",
    "persist-credentials: false",
    "github.event.pull_request.base.sha",
    "github.event.pull_request.head.sha",
    "project-contract-check -SelfTest",
    "generate_docs.py --repo-root . --check",
    "fetch_energyplus_reference_subset.py --repo-root . --self-test",
    "if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }",
    "fetch_energyplus_reference_subset.py --repo-root . --force",
    "algorithm-ledger-check -SelfTest",
    "-BaseSha",
    "-HeadSha",
]
PR_CHECK_DIFF_TOKENS = [
    "merge-base",
    "--name-status",
    "--find-renames=50%",
    "ConvertFrom-GitNameStatusZ",
    "ConvertFrom-GitNameStatusRecordsZ",
    "Test-AlgorithmSourceOrderPath",
    "Get-EvidenceGateScriptPaths",
    "Get-LedgerMappedScriptPaths",
    "ChangedFilesProvided",
    "Assert-TicketReferences",
    "map every head-side sensitive Rust path",
    "Assert-ChangedContractCoverage",
    "Get-ChangedTomlBlockIds",
    "Get-ChangedTomlSectionKeys",
    "Get-UnrelatedEvidenceCommandNames",
    "Get-AllowedGateCommandBoundaryNames",
    "scripts/dev/commands.json",
    "data/conformance_cases",
]
STRUCTURE_AUDIT_BY_SOURCE_ORDER_DOMAIN = {
    "heat_balance": "scripts/quality/heat-balance-structure-audit.ps1",
    "hvac": "scripts/quality/ideal-loads-structure-audit.ps1",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate source-order algorithm specs.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--self-test", action="store_true")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def path_before_anchor(value: str) -> str:
    return value.split("::", 1)[0]


def anchor_tokens(value: str) -> list[str]:
    if "::" not in value:
        return []
    return [token for token in value.split("::")[1:] if token]


def load_json(path: Path) -> dict[str, Any]:
    import json

    return json.loads(path.read_text(encoding="utf-8"))


def variable_names(repo_root: Path) -> set[str]:
    spec = load_toml(repo_root / "specs" / "variable_coverage.toml")
    return {str(item.get("name", "")).strip() for item in spec.get("variable", [])}


def command_names(repo_root: Path) -> set[str]:
    catalog = load_json(repo_root / "scripts" / "dev" / "commands.json")
    return {str(entry.get("name", "")) for entry in catalog.get("commands", [])}


def dev_command_from_gate(script: str) -> str:
    parts = script.replace("\\", "/").split()
    for index, part in enumerate(parts):
        if part.endswith("scripts/dev.cmd") or part.endswith("scripts/dev.ps1"):
            if index + 1 < len(parts):
                return parts[index + 1]
    return ""


def validate_rust_target_symbol(repo_root: Path, algorithm_id: str, target: str, errors: list[str]) -> None:
    tokens = anchor_tokens(target)
    if not tokens:
        return
    target_path = repo_root / path_before_anchor(target)
    if not target_path.is_file():
        return
    text = target_path.read_text(encoding="utf-8", errors="replace")
    for token in tokens:
        require(token in text, errors, f"{algorithm_id}: Rust target symbol token not found in {target}: {token}")


def validate_rust_implementation_symbol(repo_root: Path, routine_id: str, target: str, errors: list[str]) -> None:
    target_path = repo_root / path_before_anchor(target)
    if not target_path.is_file():
        return
    text = target_path.read_text(encoding="utf-8", errors="replace")
    for token in anchor_tokens(target):
        require(
            re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", token) is not None,
            errors,
            f"{routine_id}: Rust target symbol must be an identifier: {token}",
        )
        declaration = re.search(
            rf"\b(?:fn|struct|enum|trait|type|const|static|mod)\s+{re.escape(token)}\b",
            text,
        )
        require(
            declaration is not None,
            errors,
            f"{routine_id}: Rust implementation declaration not found in {target}: {token}",
        )


def validate_source_order_scaffold(repo_root: Path, algorithm: dict[str, Any], errors: list[str]) -> None:
    algorithm_id = str(algorithm.get("id", "")).strip()
    if "source_order" not in algorithm_id:
        return
    domain = str(algorithm.get("domain", "")).strip()
    audit_path = STRUCTURE_AUDIT_BY_SOURCE_ORDER_DOMAIN.get(domain)
    require(
        audit_path is not None,
        errors,
        f"{algorithm_id}: source-order scaffold needs a structure audit mapping for domain {domain}",
    )
    if audit_path is None:
        return

    audit_file = repo_root / audit_path
    require(audit_file.is_file(), errors, f"{algorithm_id}: structure audit missing: {audit_path}")
    check_path = repo_root / "scripts" / "quality" / "check.ps1"
    if check_path.is_file():
        check_text = check_path.read_text(encoding="utf-8", errors="replace")
        require(
            Path(audit_path).stem in check_text,
            errors,
            f"{algorithm_id}: structure audit {Path(audit_path).stem} must run from quality/check.ps1",
        )
    else:
        require(False, errors, f"{algorithm_id}: quality check wrapper missing: scripts/quality/check.ps1")

    if not audit_file.is_file():
        return
    audit_text = audit_file.read_text(encoding="utf-8", errors="replace")
    symbol_tokens = [token for target in algorithm.get("rust_target", []) for token in anchor_tokens(str(target))]
    require(
        any(token in audit_text for token in symbol_tokens),
        errors,
        f"{algorithm_id}: structure audit must reference at least one source-order rust target symbol",
    )


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def validate_algorithm(
    repo_root: Path,
    reference_root: Path,
    algorithm: dict[str, Any],
    covered_variables: set[str],
    commands: set[str],
    errors: list[str],
) -> None:
    algorithm_id = str(algorithm.get("id", "")).strip()
    prefix = algorithm_id or "<missing-id>"

    require(bool(algorithm_id), errors, "algorithm id must not be empty")
    require(bool(str(algorithm.get("domain", "")).strip()), errors, f"{prefix}: domain must not be empty")
    status = str(algorithm.get("status", "")).strip()
    require(status in ALLOWED_STATUS, errors, f"{prefix}: unsupported status {status!r}")

    source_map = str(algorithm.get("source_map", "")).strip()
    source_map_text = ""
    require(bool(source_map), errors, f"{prefix}: source_map must not be empty")
    if source_map:
        source_map_path = repo_root / path_before_anchor(source_map)
        require(source_map_path.is_file(), errors, f"{prefix}: source_map does not exist: {source_map}")
        if source_map_path.is_file():
            source_map_text = source_map_path.read_text(encoding="utf-8", errors="replace")
            require(
                "Reference version: EnergyPlus 26.1.0" in source_map_text,
                errors,
                f"{prefix}: source_map must pin EnergyPlus 26.1.0",
            )

    energyplus_sources = algorithm.get("energyplus_source", [])
    require(
        isinstance(energyplus_sources, list) and bool(energyplus_sources),
        errors,
        f"{prefix}: energyplus_source must not be empty",
    )
    for source in energyplus_sources:
        source_path = reference_root / path_before_anchor(str(source))
        require(source_path.is_file(), errors, f"{prefix}: EnergyPlus source does not exist: {source}")

    execution_plan_path = repo_root / "crates" / "ep_runtime" / "src" / "execution_plan.rs"
    execution_plan_text = execution_plan_path.read_text(encoding="utf-8", errors="replace")
    port_ticket_mappings = algorithm.get("port_ticket_mappings", [])
    require(isinstance(port_ticket_mappings, list), errors, f"{prefix}: port_ticket_mappings must be an array")
    if isinstance(port_ticket_mappings, list):
        for mapping in port_ticket_mappings:
            parts = [part.strip() for part in str(mapping).split("|")]
            require(len(parts) == 4 and all(parts), errors, f"{prefix}: invalid port_ticket_mapping: {mapping}")
            if len(parts) != 4:
                continue
            source, routine, source_stage, execution_stage = parts
            require(source in energyplus_sources, errors, f"{prefix}: ticket mapping source is not in energyplus_source: {source}")
            source_path = reference_root / source
            if source_path.is_file():
                source_text = source_path.read_text(encoding="utf-8", errors="replace")
                require(routine in source_text, errors, f"{prefix}: ticket mapping routine missing from {source}: {routine}")
            require(f"`{routine}`" in source_map_text or f"::{routine}`" in source_map_text, errors, f"{prefix}: ticket mapping routine missing from source_map: {routine}")
            require(source_stage in source_map_text, errors, f"{prefix}: ticket mapping source stage missing from source_map: {source_stage}")
            require(
                f"    {execution_stage}," in execution_plan_text,
                errors,
                f"{prefix}: ticket mapping ExecutionStageKind does not exist: {execution_stage}",
            )

    rust_targets = algorithm.get("rust_target", [])
    require(isinstance(rust_targets, list) and bool(rust_targets), errors, f"{prefix}: rust_target must not be empty")
    for target in rust_targets:
        target_path = repo_root / path_before_anchor(str(target))
        require(target_path.is_file(), errors, f"{prefix}: Rust target does not exist: {target}")
        validate_rust_target_symbol(repo_root, prefix, str(target), errors)

    first_case = str(algorithm.get("first_case", "")).strip()
    require(bool(first_case), errors, f"{prefix}: first_case must not be empty")
    case_path = repo_root / "data" / "conformance_cases" / first_case / "case.toml"
    require(case_path.is_file(), errors, f"{prefix}: first_case manifest does not exist: {first_case}")
    family_cases = algorithm.get("family_cases", [])
    if isinstance(family_cases, list):
        for raw_case_id in family_cases:
            case_id = str(raw_case_id).strip()
            if re.fullmatch(r"[a-z0-9][a-z0-9_]*", case_id) is None:
                continue
            family_case_path = repo_root / "data" / "conformance_cases" / case_id / "case.toml"
            require(
                family_case_path.is_file(),
                errors,
                f"{prefix}: family_cases manifest does not exist: {case_id}",
            )

    proof_variables = [str(value).strip() for value in algorithm.get("proof_variables", [])]
    require(bool(proof_variables), errors, f"{prefix}: proof_variables must not be empty")
    for variable in proof_variables:
        require(variable in covered_variables, errors, f"{prefix}: proof variable missing from variable coverage: {variable}")

    claim_level = str(algorithm.get("claim_level", "")).strip()
    require(bool(claim_level), errors, f"{prefix}: claim_level must not be empty")
    if claim_level != "none":
        require(
            status == "conformance",
            errors,
            f"{prefix}: claim_level {claim_level!r} requires status=conformance",
        )

    if case_path.is_file():
        case = load_toml(case_path)
        gate = case.get("gate") or {}
        gate_script = str(gate.get("script", "")).strip()
        outputs = case.get("outputs", [])
        output_variables = {str(output.get("variable", "")) for output in outputs}
        if status == "conformance":
            require(case.get("comparison_class") == "conformance", errors, f"{prefix}: conformance entry requires conformance case")
            require(case.get("conformance_claim") is True, errors, f"{prefix}: conformance entry requires conformance_claim=true")
            require(bool(gate.get("blocking")), errors, f"{prefix}: conformance claim requires blocking gate")
            require(bool(gate_script), errors, f"{prefix}: conformance claim requires a gate script")
            command = dev_command_from_gate(gate_script)
            require(command in commands, errors, f"{prefix}: gate script must call a registered dev command: {gate_script}")
            for variable in proof_variables:
                require(variable in output_variables, errors, f"{prefix}: proof variable is not requested by first_case: {variable}")
        elif status == "diagnostic_only":
            require(case.get("conformance_claim") is False, errors, f"{prefix}: diagnostic entry must not use a conformance claim")
        elif status == "scaffold":
            require(claim_level == "none", errors, f"{prefix}: scaffold entry must use claim_level=none")
            require(
                not str(algorithm.get("first_evidence", "")).strip(),
                errors,
                f"{prefix}: scaffold entry must not claim first_evidence",
            )
            boundary = str(algorithm.get("support_boundary", "")).lower()
            require("scaffold" in boundary, errors, f"{prefix}: scaffold support_boundary must state scaffold status")
            require(
                "does not add" in boundary or "no " in boundary or "not " in boundary,
                errors,
                f"{prefix}: scaffold support_boundary must state that no conformance is added",
            )
            validate_source_order_scaffold(repo_root, algorithm, errors)


def contains_symbol(text: str, symbol: str) -> bool:
    return re.search(rf"(?<![A-Za-z0-9_]){re.escape(symbol)}(?![A-Za-z0-9_])", text) is not None


def contains_cpp_routine_definition(text: str, routine: str) -> bool:
    pattern = re.compile(
        rf"(?ms)^[ \t]*"
        rf"(?:[A-Za-z_][A-Za-z0-9_:<>,*&\[\] \t]*[ \t]+)+"
        rf"(?:[A-Za-z_][A-Za-z0-9_]*::)*{re.escape(routine)}[ \t]*"
        rf"\([^;{{}}]*\)[ \t\r\n]*(?:const[ \t\r\n]+)?(?:noexcept[ \t\r\n]+)?"
        rf"(?:[ \t\r\n]|\/\/[^\r\n]*(?:\r?\n|$)|\/\*.*?\*\/)*\{{"
    )
    return pattern.search(text) is not None


def routine_state_contract(text: str, routine_id: str) -> str:
    marker = re.escape(routine_id)
    match = re.search(
        rf"(?ms)^<!-- routine-state-contract:v1 begin {marker} -->[ \t]*$"
        rf"(?P<body>.*?)"
        rf"^<!-- routine-state-contract:v1 end {marker} -->[ \t]*$",
        text,
    )
    return match.group("body") if match else ""


def is_safe_repo_relative_ref(value: str) -> bool:
    path = path_before_anchor(value).replace("\\", "/")
    parts = [part for part in path.split("/") if part]
    return bool(path) and not path.startswith("/") and re.match(r"^[A-Za-z]:", path) is None and ".." not in parts


def collect_routines(algorithms: list[dict[str, Any]], errors: list[str]) -> list[dict[str, Any]]:
    routines: list[dict[str, Any]] = []
    seen_ids: set[str] = set()
    seen_source_routines: dict[tuple[str, str], str] = {}
    for algorithm in algorithms:
        require(isinstance(algorithm, dict), errors, "algorithm entries must be TOML tables")
        if not isinstance(algorithm, dict):
            continue
        algorithm_id = str(algorithm.get("id", "")).strip()
        domain = str(algorithm.get("domain", "")).strip()
        routine_map = algorithm.get("routine", {})
        raw_family_cases = algorithm.get("family_cases", [])
        require(isinstance(raw_family_cases, list), errors, f"{algorithm_id}: family_cases must be an array")
        family_cases = (
            [str(value).strip() for value in raw_family_cases]
            if isinstance(raw_family_cases, list)
            else []
        )
        first_case = str(algorithm.get("first_case", "")).strip()
        require(
            isinstance(raw_family_cases, list)
            and all(isinstance(value, str) for value in raw_family_cases),
            errors,
            f"{algorithm_id}: family_cases must contain only string ids",
        )
        require(all(family_cases), errors, f"{algorithm_id}: family_cases must not contain empty ids")
        require(
            all(re.fullmatch(r"[a-z0-9][a-z0-9_]*", case_id) is not None for case_id in family_cases),
            errors,
            f"{algorithm_id}: family_cases must contain only valid case ids",
        )
        require(
            len(family_cases) == len(set(family_cases)),
            errors,
            f"{algorithm_id}: family_cases must not contain duplicates",
        )
        require(
            first_case not in family_cases,
            errors,
            f"{algorithm_id}: family_cases must not repeat first_case",
        )
        parent_case_ids = [case_id for case_id in [first_case, *family_cases] if case_id]
        require(isinstance(routine_map, dict), errors, f"{algorithm_id}: routine must be a TOML dotted table")
        if not isinstance(routine_map, dict):
            continue
        for routine_id, value in routine_map.items():
            prefix = str(routine_id).strip() or "<missing-routine-id>"
            require(
                re.fullmatch(r"[a-z0-9][a-z0-9_]*", prefix) is not None,
                errors,
                f"invalid routine id: {prefix}",
            )
            require(prefix not in seen_ids, errors, f"duplicate routine id: {prefix}")
            seen_ids.add(prefix)
            require(isinstance(value, dict), errors, f"{prefix}: routine record must be a TOML table")
            if not isinstance(value, dict):
                continue
            routine = dict(value)
            routine["_id"] = prefix
            routine["_algorithm_id"] = algorithm_id
            routine["_domain"] = domain
            routine["_parent_sources"] = algorithm.get("energyplus_source", [])
            routine["_parent_rust_targets"] = algorithm.get("rust_target", [])
            routine["_parent_port_ticket_mappings"] = algorithm.get("port_ticket_mappings", [])
            routine["_parent_case_ids"] = parent_case_ids
            routine["_parent_proof_variables"] = algorithm.get("proof_variables", [])
            routines.append(routine)
            source_key = (
                str(routine.get("source_file", "")).strip(),
                str(routine.get("source_routine", "")).strip(),
            )
            if all(source_key):
                require(
                    source_key not in seen_source_routines,
                    errors,
                    f"duplicate routine source mapping: {source_key[0]}::{source_key[1]} "
                    f"({seen_source_routines.get(source_key, '')}, {prefix})",
                )
                seen_source_routines[source_key] = prefix
    return routines


def validate_checked_in_ref(repo_root: Path, routine_id: str, field: str, value: str, errors: list[str]) -> str:
    text = ""
    require(bool(value), errors, f"{routine_id}: {field} must not be empty")
    if not value:
        return text
    require(is_safe_repo_relative_ref(value), errors, f"{routine_id}: {field} must be a safe repo-relative path: {value}")
    if not is_safe_repo_relative_ref(value):
        return text
    path = repo_root / path_before_anchor(value)
    require(path.is_file(), errors, f"{routine_id}: {field} does not exist: {value}")
    if path.is_file():
        text = path.read_text(encoding="utf-8", errors="replace")
        for token in anchor_tokens(value):
            require(
                contains_symbol(text, token),
                errors,
                f"{routine_id}: {field} anchor token not found in {value}: {token}",
            )
    return text


def validate_routine(
    repo_root: Path,
    reference_root: Path | None,
    routine: dict[str, Any],
    covered_variables: set[str],
    commands: set[str],
    errors: list[str],
) -> None:
    routine_id = str(routine.get("_id", "")).strip() or "<missing-routine-id>"
    status = str(routine.get("completion_status", "")).strip()
    require(
        status in ROUTINE_COMPLETION_RANK,
        errors,
        f"{routine_id}: unsupported routine completion_status {status!r}",
    )
    if status not in ROUTINE_COMPLETION_RANK:
        return
    rank = ROUTINE_COMPLETION_RANK[status]
    require(
        isinstance(routine.get("required_for_full_domain"), bool),
        errors,
        f"{routine_id}: required_for_full_domain must be boolean",
    )

    if rank >= ROUTINE_COMPLETION_RANK["source_mapped"]:
        source_file = str(routine.get("source_file", "")).strip()
        source_routine = str(routine.get("source_routine", "")).strip()
        source_map = str(routine.get("source_map", "")).strip()
        require(bool(source_file), errors, f"{routine_id}: source_file is required at source_mapped+")
        require(bool(source_routine), errors, f"{routine_id}: source_routine is required at source_mapped+")
        source_routine_valid = re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", source_routine) is not None
        require(
            source_routine_valid,
            errors,
            f"{routine_id}: source_routine must be a C++ identifier: {source_routine}",
        )
        source_file_safe = (
            is_safe_repo_relative_ref(source_file)
            and source_file.replace("\\", "/").startswith("src/EnergyPlus/")
            and source_file.lower().endswith((".cc", ".cpp", ".cxx"))
        )
        require(
            source_file_safe,
            errors,
            f"{routine_id}: source_file must be a safe path below src/EnergyPlus: {source_file}",
        )
        parent_sources = routine.get("_parent_sources", [])
        require(
            isinstance(parent_sources, list) and source_file in parent_sources,
            errors,
            f"{routine_id}: source_file must be listed by parent algorithm {routine.get('_algorithm_id', '')}: {source_file}",
        )
        if reference_root is not None and source_file_safe:
            source_path = reference_root / source_file
            require(source_path.is_file(), errors, f"{routine_id}: EnergyPlus source does not exist: {source_file}")
            if source_path.is_file() and source_routine_valid:
                source_text = source_path.read_text(encoding="utf-8", errors="replace")
                require(
                    contains_cpp_routine_definition(source_text, source_routine),
                    errors,
                    f"{routine_id}: C++ routine definition not found in {source_file}: {source_routine}",
                )
        source_map_text = validate_checked_in_ref(repo_root, routine_id, "source_map", source_map, errors)
        if source_map_text:
            require(
                "Reference version: EnergyPlus 26.1.0" in source_map_text,
                errors,
                f"{routine_id}: source_map must pin EnergyPlus 26.1.0",
            )
            if source_routine_valid:
                require(
                    contains_symbol(source_map_text, source_routine),
                    errors,
                    f"{routine_id}: source_routine missing from source_map: {source_routine}",
                )

    if rank >= ROUTINE_COMPLETION_RANK["state_mapped"]:
        state_mapping_ref = str(routine.get("state_mapping_ref", "")).strip()
        state_mapping_text = validate_checked_in_ref(repo_root, routine_id, "state_mapping_ref", state_mapping_ref, errors)
        state_contract_text = routine_state_contract(state_mapping_text, routine_id)
        require(
            bool(state_contract_text),
            errors,
            f"{routine_id}: state_mapping_ref must contain matching routine-state-contract:v1 markers",
        )
        for field in STATE_CONTRACT_FIELDS:
            require(
                f"{field}:" in state_contract_text,
                errors,
                f"{routine_id}: state contract must declare {field}:",
            )
        source_routine = str(routine.get("source_routine", "")).strip()
        if state_mapping_text and source_routine:
            require(
                contains_symbol(state_mapping_text, source_routine),
                errors,
                f"{routine_id}: source_routine missing from state_mapping_ref: {source_routine}",
            )
        for field in ["read_state", "write_state"]:
            value = routine.get(field)
            require(
                isinstance(value, list)
                and bool(value)
                and all(isinstance(item, str) and item.strip() for item in value),
                errors,
                f"{routine_id}: {field} must be a non-empty array at state_mapped+",
            )
            if isinstance(value, list):
                for item in value:
                    if isinstance(item, str) and item.strip():
                        require(
                            item.strip() != source_routine,
                            errors,
                            f"{routine_id}: {field} must not reuse source_routine as placeholder state",
                        )
                        require(
                            item.strip() in state_contract_text,
                            errors,
                            f"{routine_id}: {field} item missing from state_mapping_ref: {item.strip()}",
                        )
        history_state_ownership = str(routine.get("history_state_ownership", "")).strip()
        require(
            bool(history_state_ownership),
            errors,
            f"{routine_id}: history_state_ownership is required at state_mapped+",
        )
        if history_state_ownership:
            require(
                history_state_ownership != source_routine,
                errors,
                f"{routine_id}: history_state_ownership must not reuse source_routine as placeholder state",
            )
            require(
                history_state_ownership in state_contract_text,
                errors,
                f"{routine_id}: history_state_ownership missing from state_mapping_ref: {history_state_ownership}",
            )
        for field in ["unsupported_state", "inactive_branches", "unsupported_active_branches", "not_claimed_branches"]:
            values = routine.get(field)
            require(isinstance(values, list), errors, f"{routine_id}: {field} must be an array at state_mapped+")
            if isinstance(values, list):
                require(
                    all(isinstance(value, str) and value.strip() for value in values),
                    errors,
                    f"{routine_id}: {field} must contain only non-empty strings",
                )
                for value in values:
                    if isinstance(value, str) and value.strip():
                        require(
                            value.strip() in state_contract_text,
                            errors,
                            f"{routine_id}: {field} item missing from state_mapping_ref: {value.strip()}",
                        )

    if rank >= ROUTINE_COMPLETION_RANK["implemented"]:
        rust_targets = routine.get("rust_target", [])
        require(
            isinstance(rust_targets, list) and bool(rust_targets),
            errors,
            f"{routine_id}: rust_target must be a non-empty array at implemented+",
        )
        parent_rust_targets = routine.get("_parent_rust_targets", [])
        require(
            isinstance(parent_rust_targets, list),
            errors,
            f"{routine_id}: parent algorithm rust_target must be an array",
        )
        parent_rust_target_set = (
            {str(target).strip() for target in parent_rust_targets}
            if isinstance(parent_rust_targets, list)
            else set()
        )
        source_file = str(routine.get("source_file", "")).strip()
        source_routine = str(routine.get("source_routine", "")).strip()
        parent_port_ticket_mappings = routine.get("_parent_port_ticket_mappings", [])
        require(
            isinstance(parent_port_ticket_mappings, list),
            errors,
            f"{routine_id}: parent algorithm port_ticket_mappings must be an array",
        )
        has_ticket_mapping = False
        if isinstance(parent_port_ticket_mappings, list):
            for mapping in parent_port_ticket_mappings:
                parts = [part.strip() for part in str(mapping).split("|")]
                if len(parts) == 4 and all(parts) and parts[:2] == [source_file, source_routine]:
                    has_ticket_mapping = True
                    break
        require(
            has_ticket_mapping,
            errors,
            f"{routine_id}: parent port_ticket_mappings must map "
            f"{source_file}|{source_routine}| at implemented+",
        )
        if isinstance(rust_targets, list):
            normalized_rust_targets = [str(target).strip() for target in rust_targets]
            require(
                all(isinstance(target, str) and target.strip() for target in rust_targets),
                errors,
                f"{routine_id}: rust_target must contain only non-empty strings",
            )
            require(
                len(normalized_rust_targets) == len(set(normalized_rust_targets)),
                errors,
                f"{routine_id}: rust_target must not contain duplicates",
            )
            for target in rust_targets:
                target_value = str(target).strip()
                require(
                    target_value in parent_rust_target_set,
                    errors,
                    f"{routine_id}: Rust target must be an exact parent algorithm rust_target: {target_value}",
                )
                require(
                    is_safe_repo_relative_ref(target_value),
                    errors,
                    f"{routine_id}: Rust target must be a safe repo-relative path: {target_value}",
                )
                require(
                    bool(anchor_tokens(target_value)),
                    errors,
                    f"{routine_id}: Rust target must include a symbol anchor: {target_value}",
                )
                if not is_safe_repo_relative_ref(target_value):
                    continue
                target_path = repo_root / path_before_anchor(target_value)
                require(target_path.is_file(), errors, f"{routine_id}: Rust target does not exist: {target_value}")
                validate_rust_implementation_symbol(repo_root, routine_id, target_value, errors)

    if rank >= ROUTINE_COMPLETION_RANK["family_gated"]:
        raw_family_gate_ids = routine.get("family_gate_ids", [])
        raw_proof_variables = routine.get("proof_variables", [])
        require(isinstance(raw_family_gate_ids, list), errors, f"{routine_id}: family_gate_ids must be an array")
        require(isinstance(raw_proof_variables, list), errors, f"{routine_id}: proof_variables must be an array")
        family_gate_ids = (
            [str(value).strip() for value in raw_family_gate_ids]
            if isinstance(raw_family_gate_ids, list)
            else []
        )
        proof_variables = (
            [str(value).strip() for value in raw_proof_variables]
            if isinstance(raw_proof_variables, list)
            else []
        )
        require(bool(family_gate_ids), errors, f"{routine_id}: family_gate_ids must not be empty at family_gated+")
        require(all(family_gate_ids), errors, f"{routine_id}: family_gate_ids must not contain empty ids")
        require(
            len(family_gate_ids) == len(set(family_gate_ids)),
            errors,
            f"{routine_id}: family_gate_ids must not contain duplicates",
        )
        require(bool(proof_variables), errors, f"{routine_id}: proof_variables must not be empty at family_gated+")
        require(all(proof_variables), errors, f"{routine_id}: proof_variables must not contain empty names")
        parent_proof_variables = routine.get("_parent_proof_variables", [])
        require(
            isinstance(parent_proof_variables, list),
            errors,
            f"{routine_id}: parent algorithm proof_variables must be an array",
        )
        parent_proof_set = (
            {str(value).strip() for value in parent_proof_variables}
            if isinstance(parent_proof_variables, list)
            else set()
        )
        for variable in proof_variables:
            require(variable in covered_variables, errors, f"{routine_id}: proof variable missing from variable coverage: {variable}")
            require(
                variable in parent_proof_set,
                errors,
                f"{routine_id}: proof variable must be declared by parent algorithm: {variable}",
            )
        routine_domain = str(routine.get("_domain", "")).strip()
        allowed_evidence_domains = ROUTINE_EVIDENCE_DOMAINS.get(routine_domain, {routine_domain})
        parent_case_ids = set(str(value) for value in routine.get("_parent_case_ids", []))
        family_output_variables: set[str] = set()
        for case_id in family_gate_ids:
            require(
                re.fullmatch(r"[a-z0-9][a-z0-9_]*", case_id) is not None,
                errors,
                f"{routine_id}: invalid family gate case id: {case_id}",
            )
            if re.fullmatch(r"[a-z0-9][a-z0-9_]*", case_id) is None:
                continue
            require(
                case_id in parent_case_ids,
                errors,
                f"{routine_id}: family gate must be declared by parent algorithm {routine.get('_algorithm_id', '')}: {case_id}",
            )
            case_path = repo_root / "data" / "conformance_cases" / case_id / "case.toml"
            require(case_path.is_file(), errors, f"{routine_id}: family gate case does not exist: {case_id}")
            if not case_path.is_file():
                continue
            case = load_toml(case_path)
            gate = case.get("gate") or {}
            gate_script = str(gate.get("script", "")).strip()
            routine_coverage = case.get("routine_coverage")
            require(
                isinstance(routine_coverage, dict),
                errors,
                f"{routine_id}: family gate routine_coverage must be a table: {case_id}",
            )
            if not isinstance(routine_coverage, dict):
                routine_coverage = {}
            covered_algorithms = routine_coverage.get("algorithm_ids", [])
            covered_routines = routine_coverage.get("routine_ids", [])
            for field, values in [
                ("algorithm_ids", covered_algorithms),
                ("routine_ids", covered_routines),
            ]:
                require(
                    isinstance(values, list),
                    errors,
                    f"{routine_id}: family gate routine_coverage.{field} must be an array: {case_id}",
                )
                if isinstance(values, list):
                    normalized_values = [str(value).strip() for value in values]
                    require(
                        all(isinstance(value, str) and value.strip() for value in values),
                        errors,
                        f"{routine_id}: family gate routine_coverage.{field} "
                        f"must contain only non-empty ids: {case_id}",
                    )
                    require(
                        len(normalized_values) == len(set(normalized_values)),
                        errors,
                        f"{routine_id}: family gate routine_coverage.{field} "
                        f"must not contain duplicates: {case_id}",
                    )
            require(
                isinstance(covered_algorithms, list)
                and str(routine.get("_algorithm_id", "")) in covered_algorithms,
                errors,
                f"{routine_id}: family gate must declare parent algorithm in routine_coverage.algorithm_ids: {case_id}",
            )
            require(
                isinstance(covered_routines, list) and routine_id in covered_routines,
                errors,
                f"{routine_id}: family gate must declare routine in routine_coverage.routine_ids: {case_id}",
            )
            scope = case.get("scope") or {}
            scope_domains = {
                str(value).strip()
                for value in scope.get("domains", [])
                if str(value).strip()
            }
            require(
                bool(scope_domains & allowed_evidence_domains),
                errors,
                f"{routine_id}: family gate scope does not cover routine domain {routine_domain}: {case_id}",
            )
            require(
                routine_domain in scope_domains,
                errors,
                f"{routine_id}: family gate scope must include exact routine domain {routine_domain}: {case_id}",
            )
            output_variables = {
                str(output.get("variable", ""))
                for output in case.get("outputs", [])
                if isinstance(output, dict)
                and output.get("level") == "conformance"
                and str(output.get("domain", "")).strip() in allowed_evidence_domains
            }
            family_output_variables.update(output_variables)
            require(
                case.get("comparison_class") == "conformance",
                errors,
                f"{routine_id}: family_gated requires a conformance case: {case_id}",
            )
            require(
                case.get("conformance_claim") is True,
                errors,
                f"{routine_id}: family_gated requires conformance_claim=true: {case_id}",
            )
            require(gate.get("blocking") is True, errors, f"{routine_id}: family gate must be blocking: {case_id}")
            require(bool(gate_script), errors, f"{routine_id}: family gate script is missing: {case_id}")
            command = dev_command_from_gate(gate_script)
            require(command in commands, errors, f"{routine_id}: family gate must call a registered dev command: {gate_script}")
        for variable in proof_variables:
            require(
                variable in family_output_variables,
                errors,
                f"{routine_id}: proof variable is not requested by any family gate: {variable}",
            )

    if rank >= ROUTINE_COMPLETION_RANK["complete"]:
        completion_evidence = routine.get("completion_evidence", [])
        require(
            isinstance(completion_evidence, list)
            and bool(completion_evidence)
            and all(str(value).strip() for value in completion_evidence),
            errors,
            f"{routine_id}: completion_evidence must not be empty at complete",
        )
        if isinstance(completion_evidence, list):
            for evidence_ref in completion_evidence:
                validate_checked_in_ref(
                    repo_root,
                    routine_id,
                    "completion_evidence",
                    str(evidence_ref).strip(),
                    errors,
                )
        require(
            routine.get("unsupported_active_branches") == [],
            errors,
            f"{routine_id}: complete routine must not retain unsupported_active_branches",
        )
        require(
            routine.get("not_claimed_branches") == [],
            errors,
            f"{routine_id}: complete routine must not retain not_claimed_branches",
        )


def validate_port_ticket_contract(repo_root: Path, errors: list[str]) -> None:
    template_path = repo_root / "specs" / "algorithm_port_ticket_template.toml"
    pr_template_path = repo_root / ".github" / "pull_request_template.md"
    workflow_path = repo_root / ".github" / "workflows" / "pull-request.yml"
    pr_check_path = repo_root / "scripts" / "quality" / "pr-port-ticket-check.ps1"
    pr_check_changed_files_path = repo_root / "scripts" / "quality" / "pr-port-ticket-check" / "changed-files.ps1"
    pr_check_contract_diff_path = repo_root / "scripts" / "quality" / "pr-port-ticket-check" / "contract-diff.ps1"
    pr_check_self_test_path = repo_root / "scripts" / "quality" / "pr-port-ticket-check" / "self-tests.ps1"
    doc_path = repo_root / "docs" / "src" / "porting-map" / "algorithm-port-ticket.md"

    require(template_path.is_file(), errors, f"missing algorithm port ticket template: {template_path}")
    require(pr_template_path.is_file(), errors, f"missing PR template: {pr_template_path}")
    require(workflow_path.is_file(), errors, f"missing PR workflow: {workflow_path}")
    require(pr_check_path.is_file(), errors, f"missing PR port-ticket check: {pr_check_path}")
    require(pr_check_changed_files_path.is_file(), errors, f"missing PR port-ticket changed-file library: {pr_check_changed_files_path}")
    require(pr_check_contract_diff_path.is_file(), errors, f"missing PR port-ticket contract-diff library: {pr_check_contract_diff_path}")
    require(pr_check_self_test_path.is_file(), errors, f"missing PR port-ticket self-tests: {pr_check_self_test_path}")
    require(doc_path.is_file(), errors, f"missing algorithm port ticket docs: {doc_path}")
    if not template_path.is_file():
        return

    template = load_toml(template_path)
    for section, fields in PORT_TICKET_REQUIRED_FIELDS.items():
        value = template.get(section)
        require(isinstance(value, dict), errors, f"port ticket template missing [{section}] section")
        if not isinstance(value, dict):
            continue
        for field in fields:
            require(field in value, errors, f"port ticket template missing {section}.{field}")

    ticket = template.get("algorithm_port_ticket", {})
    port_type = str(ticket.get("port_type", "")).strip()
    require(port_type in ALLOWED_PORT_TYPES, errors, f"port ticket template has unsupported default port_type: {port_type!r}")
    require(template.get("energyplus", {}).get("version") == "26.1.0", errors, "port ticket template must pin EnergyPlus 26.1.0")
    require(template.get("rust", {}).get("compatibility_path") is True, errors, "port ticket template must default rust.compatibility_path=true")
    require(template.get("rust", {}).get("diagnostic_probe_used") is False, errors, "port ticket template must default rust.diagnostic_probe_used=false")
    require(
        template.get("claim_boundary", {}).get("conformance_claim") is False,
        errors,
        "port ticket template must default claim_boundary.conformance_claim=false",
    )
    require(
        template.get("claim_boundary", {}).get("partial_run_allowed") is False,
        errors,
        "port ticket template must default claim_boundary.partial_run_allowed=false",
    )

    if pr_template_path.is_file():
        pr_text = pr_template_path.read_text(encoding="utf-8", errors="replace")
        for token in PR_TEMPLATE_REQUIRED_TOKENS:
            require(token in pr_text, errors, f"PR template missing algorithm port ticket field token: {token}")

    if workflow_path.is_file():
        workflow_text = workflow_path.read_text(encoding="utf-8", errors="replace")
        for token in PR_WORKFLOW_REQUIRED_TOKENS:
            require(token in workflow_text, errors, f"PR workflow missing algorithm port ticket token: {token}")

    if pr_check_path.is_file():
        pr_check_text = pr_check_path.read_text(encoding="utf-8", errors="replace")
        changed_files_text = (
            pr_check_changed_files_path.read_text(encoding="utf-8", errors="replace")
            if pr_check_changed_files_path.is_file()
            else ""
        )
        contract_diff_text = (
            pr_check_contract_diff_path.read_text(encoding="utf-8", errors="replace")
            if pr_check_contract_diff_path.is_file()
            else ""
        )
        pr_check_contract_text = pr_check_text + "\n" + changed_files_text + "\n" + contract_diff_text
        for token in PR_TEMPLATE_REQUIRED_TOKENS:
            field_name = token.rstrip(":")
            require(field_name in pr_check_text, errors, f"PR port-ticket check missing required field: {field_name}")
        require(
            "source-order algorithm PRs require an Algorithm Port Ticket" in pr_check_text,
            errors,
            "PR port-ticket check must enforce source-order algorithm ticket coverage",
        )
        require(
            "Invoke-PrPortTicketSelfTest" in pr_check_text,
            errors,
            "PR port-ticket check must expose self-test coverage",
        )
        for token in PR_CHECK_DIFF_TOKENS:
            require(token in pr_check_contract_text, errors, f"PR port-ticket check missing changed-file contract token: {token}")
        require(
            "pr-port-ticket-check\\changed-files.ps1" in pr_check_text,
            errors,
            "PR port-ticket check must load its changed-file library",
        )
        require(
            "pr-port-ticket-check\\contract-diff.ps1" in pr_check_text,
            errors,
            "PR port-ticket check must load its contract-diff library",
        )
        require(
            ".reference\\energyplus-src" not in pr_check_contract_text,
            errors,
            "PR port-ticket check must run in a clean checkout without ignored reference source",
        )

    if pr_check_self_test_path.is_file():
        self_test_text = pr_check_self_test_path.read_text(encoding="utf-8", errors="replace")
        for token in [
            "runtime_source_order_cannot_opt_out",
            "docs_only_auto_pass",
            "rename_old_and_new_paths",
            "fake_ticket_location",
            "valid_conformance",
            "missing_energyplus_routine",
            "placeholder_state",
            "unrelated_algorithm_ticket",
            "ideal_loads_evidence_gate_cannot_opt_out",
            "default_template_unique_ticket_fields",
            "mapped_file_cannot_cover_unrelated_rust",
            "ticket_rust_module_must_change",
            "invalid_source_order_stage",
            "common_word_is_not_a_routine",
            "routine_from_other_source",
            "stage_from_other_algorithm",
            "unmapped_existing_rust_function",
            "non_ledger_first_case",
            "uncovered_affected_variable",
            "invalid_tolerance_candidate",
            "tilde_fence_cannot_supply_ticket",
            "html_comment_cannot_supply_ticket",
            "unclosed_html_comment_cannot_supply_ticket",
            "gate_command_with_arguments",
            "deleted_sensitive_path",
            "rename_new_sensitive_path",
            "mapped_deleted_rust_path",
            "unrelated_deleted_rust_path",
            "unrelated_deleted_gate_script",
            "mapped_deleted_gate_script",
            "capabilities_cannot_opt_out",
            "evidence_command_catalog_cannot_opt_out",
            "case_manifest_cannot_opt_out",
            "unrelated_ledger_ticket_without_base_context",
            "non_evidence_command_catalog_auto_pass",
            "valid_case_manifest_change",
            "unrelated_case_manifest_ticket",
            "rename_base_and_head_sides",
            "changed_algorithm_block_id",
            "ledger_newline_normalization",
            "evidence_command_subset",
            "capability_section_boundary",
            "capability_root_hitchhike",
            "gate_command_boundary_union",
            "new_case_command_transition",
        ]:
            require(token in self_test_text, errors, f"PR port-ticket self-test missing mutation: {token}")

    if doc_path.is_file():
        doc_text = doc_path.read_text(encoding="utf-8", errors="replace")
        for token in PORT_TICKET_DOC_TOKENS:
            require(token in doc_text, errors, f"algorithm port ticket docs missing token: {token}")


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    reference_root = repo_root / ".reference" / "energyplus-src" / "26.1.0"
    ledger_path = repo_root / "specs" / "algorithm_ledger.toml"
    project_contract_path = repo_root / "specs" / "project_contract.toml"
    errors: list[str] = []

    if args.self_test:
        from algorithm_ledger_self_tests import run_completion_self_tests

        return run_completion_self_tests(repo_root)

    require(reference_root.is_dir(), errors, f"missing EnergyPlus reference source root: {reference_root}")
    require(ledger_path.is_file(), errors, f"missing algorithm ledger spec: {ledger_path}")
    require(project_contract_path.is_file(), errors, f"missing project contract spec: {project_contract_path}")
    validate_port_ticket_contract(repo_root, errors)
    if not ledger_path.is_file():
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    spec = load_toml(ledger_path)
    algorithms = spec.get("algorithm", [])
    require(isinstance(algorithms, list) and bool(algorithms), errors, "algorithm ledger must contain at least one [[algorithm]]")
    covered_variables = variable_names(repo_root)
    commands = command_names(repo_root)

    seen_ids: set[str] = set()
    for algorithm in algorithms:
        algorithm_id = str(algorithm.get("id", "")).strip()
        require(algorithm_id not in seen_ids, errors, f"duplicate algorithm id: {algorithm_id}")
        seen_ids.add(algorithm_id)
        validate_algorithm(repo_root, reference_root, algorithm, covered_variables, commands, errors)

    routines = collect_routines(algorithms, errors)
    require(bool(routines), errors, "algorithm ledger must contain at least one routine completion record")
    for routine in routines:
        validate_routine(repo_root, reference_root, routine, covered_variables, commands, errors)
    if project_contract_path.is_file():
        validate_domain_completion_contract(load_toml(project_contract_path), routines, errors)

    if errors:
        print("Algorithm ledger validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Algorithm ledger check")
    print(f"  algorithms: {len(algorithms)}")
    print(f"  routines: {len(routines)}")
    print("  rule: No source map, no algorithm port.")
    print("  full_domain_rule: every required routine must be family_gated or complete")
    print("  port_ticket_contract: valid")
    print("  status: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
