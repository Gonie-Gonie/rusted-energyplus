"""Validate the source-map, algorithm ledger, and port-ticket contracts."""

from __future__ import annotations

import argparse
import sys
import tomllib
from pathlib import Path
from typing import Any


ALLOWED_STATUS = {
    "source_mapped",
    "scaffold",
    "diagnostic_only",
    "conformance",
    "superseded",
}
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
    errors: list[str] = []

    require(reference_root.is_dir(), errors, f"missing EnergyPlus reference source root: {reference_root}")
    require(ledger_path.is_file(), errors, f"missing algorithm ledger spec: {ledger_path}")
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

    if errors:
        print("Algorithm ledger validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Algorithm ledger check")
    print(f"  algorithms: {len(algorithms)}")
    print("  rule: No source map, no algorithm port.")
    print("  port_ticket_contract: valid")
    print("  status: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
