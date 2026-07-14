"""Validate the project contract spec and its README/current-doc mirrors."""

from __future__ import annotations

import argparse
import copy
import sys
import tomllib
from pathlib import Path
from typing import Any

from validate_algorithm_ledger import (
    collect_routines,
    command_names,
    validate_domain_completion_contract,
    validate_routine,
    variable_names,
)


EXPECTED_VERSION = "26.1.0"
EXPECTED_ROUTINE_COMPLETION_SCHEMA = "routine_completion.v1"
REQUIRED_CLAIM_REQUIREMENTS = {
    "case_manifest",
    "declared_variables_or_meters",
    "tolerance_rules",
    "energyplus_oracle_baseline",
    "rust_result_artifact",
    "compare_summary_json",
    "compare_report_md",
    "blocking_gate",
}
REQUIRED_ALLOWED_OPTIMIZATIONS = {
    "typed_ids",
    "data_structures",
    "execution_plan",
    "precompute",
    "cache",
    "output_handles",
    "trace_throttling",
    "diagnostics",
    "result_store",
    "numerical_implementation_within_tolerance",
    "code_organization",
}
REQUIRED_FORBIDDEN_CHANGES = {
    "engineering_algorithm_change",
    "algorithm_variant_delta_tuning",
    "diagnostic_probe_as_compatibility_candidate",
    "timestep_semantics_change",
    "setpoint_manager_timing_change",
    "plant_dispatch_semantics_change",
}
EXPECTED_CURRENT_STATUS_CLASSIFICATION_IDS = [
    "conformance",
    "diagnostic-only",
    "baseline-only",
    "not claimed",
]
README_REQUIRED_PHRASES = [
    "Rust-only EnergyPlus-compatible porting project.",
    "EnergyPlus 26.1.0 as the locked oracle",
    "Compatibility mode means source-order EnergyPlus algorithm behavior",
    "Engineering algorithm changes do not belong in compatibility mode.",
    "Ad-hoc user runs are not release conformance evidence.",
    "A conformance claim requires a case manifest",
]
CURRENT_DOC_REQUIRED_PHRASES = [
    "The locked oracle is EnergyPlus 26.1.0.",
    "The Rust core remains Rust-only",
    "does not change engineering algorithms in compatibility mode.",
    "The machine-readable contract is `specs/project_contract.toml`.",
    "Markdown wording, smoke tests, diagnostics, arbitrary IDF runs, and performance",
    "results do not create compatibility claims.",
    "Heat-balance, HVAC, plant, and time full-domain claims use canonical required-routine lists",
]
ALGORITHM_LEDGER_DOC_REQUIRED_PHRASES = [
    "The first 23 source-order routines form an immutable minimum seed across heat balance, HVAC, plant, and time.",
    "A heat-balance, HVAC, plant, or time full-domain claim is valid only",
]
CURRENT_STATUS_REQUIRED_PHRASES = [
    "The exact case list is generated in `docs/src/generated/conformance-case-index.md`.",
    "coverage boundaries are generated from `specs/algorithm_ledger.toml`, `specs/object_coverage.toml`, and `specs/variable_coverage.toml`.",
    "{{#include ../generated/current-status-classification.md}}",
    "{{#include ../generated/variable-coverage.md:current-status-variable-summary}}",
]
GENERATED_DOC_REQUIRED_PHRASES = [
    "Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py.",
    "Case metadata is read from `data/conformance_cases/*/case.toml`.",
    "| Case | Milestone | Class | Claim | Tier | Domains | Evidence levels | Manifest |",
    "Variable coverage is maintained in `specs/variable_coverage.toml`.",
    "Algorithm status is maintained in `specs/algorithm_ledger.toml`.",
    "Routine completion status is a separate six-step axis",
    "| Routine ID | Domain | Parent algorithm | Completion status | Required | EnergyPlus routine |",
    "| Domain | Claim key | Claimed | Inventory complete | Family-gated required routines | Ready | Blockers |",
    "| Classification | Source of truth | Current boundary |",
    "README and current-status prose are mirrors, not claim sources.",
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate project contract source-of-truth alignment.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--self-test", action="store_true")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def require_contains_all(text: str, phrases: list[str], errors: list[str], label: str) -> None:
    normalized_text = " ".join(text.split())
    for phrase in phrases:
        normalized_phrase = " ".join(phrase.split())
        require(normalized_phrase in normalized_text, errors, f"{label} missing required phrase: {phrase}")


def markdown_cell(value: Any) -> str:
    return str(value).replace("|", "\\|").replace("\n", "<br>")


def validate_contract(contract: dict[str, Any], errors: list[str]) -> None:
    oracle = contract.get("oracle", {})
    language = contract.get("language", {})
    claims = contract.get("claims", {})
    optimization = contract.get("optimization", {})
    modes = {str(mode.get("id", "")): mode for mode in contract.get("mode", [])}
    diagnostic_only = contract.get("diagnostic_only", {})
    partial_runs = contract.get("partial_runs", {})
    documentation = contract.get("documentation", {})
    current_status_classifications = contract.get("current_status_classification", [])

    require(
        contract.get("routine_completion_schema") == EXPECTED_ROUTINE_COMPLETION_SCHEMA,
        errors,
        "project contract must pin routine_completion_schema to routine_completion.v1",
    )
    require(oracle.get("energyplus_version") == EXPECTED_VERSION, errors, "project contract must pin EnergyPlus 26.1.0")
    require(oracle.get("compatibility_mode_required") is True, errors, "project contract must require compatibility mode")
    require(language.get("core") == "Rust", errors, "project contract must keep core language Rust")
    require(language.get("mixed_language_kernels") is False, errors, "project contract must forbid mixed-language kernels")
    require(claims.get("arbitrary_runs_are_release_evidence") is False, errors, "arbitrary runs must not be release evidence")
    for claim_key in [
        "broad_heat_balance_compatibility",
        "hvac_compatibility",
        "plant_compatibility",
        "time_compatibility",
    ]:
        require(isinstance(claims.get(claim_key), bool), errors, f"{claim_key} must be boolean")
    require(claims.get("full_runtime_compatibility") is False, errors, "full runtime compatibility must not be claimed")

    claim_requirements = set(str(value) for value in claims.get("requirements", []))
    require(
        REQUIRED_CLAIM_REQUIREMENTS <= claim_requirements,
        errors,
        "project contract claim requirements are incomplete",
    )

    allowed = set(str(value) for value in optimization.get("allowed", []))
    forbidden = set(str(value) for value in optimization.get("forbidden_without_experimental_mode", []))
    require(REQUIRED_ALLOWED_OPTIMIZATIONS <= allowed, errors, "allowed compatibility optimizations are incomplete")
    require(REQUIRED_FORBIDDEN_CHANGES <= forbidden, errors, "forbidden compatibility changes are incomplete")

    require(modes.get("compatibility", {}).get("claim_eligible") is True, errors, "compatibility mode must be claim eligible")
    for mode_id in ["diagnostic", "partial", "fast", "experimental"]:
        require(modes.get(mode_id, {}).get("claim_eligible") is False, errors, f"{mode_id} mode must not be claim eligible")

    require(
        diagnostic_only.get("can_be_counted_as_conformance") is False,
        errors,
        "diagnostic-only rows must not count as conformance",
    )
    require(partial_runs.get("opt_in_required") is True, errors, "partial runs must require opt in")
    require(partial_runs.get("conformance_claim") is False, errors, "partial runs must force conformance_claim=false")
    require(documentation.get("spec_role") == "machine_readable_decision_source", errors, "spec role must be source of truth")
    require(documentation.get("generated_docs_role") == "navigation", errors, "generated docs must remain navigation")
    require(documentation.get("reports_role") == "evidence", errors, "reports must be evidence")

    classification_ids = [str(item.get("id", "")) for item in current_status_classifications]
    require(
        classification_ids == EXPECTED_CURRENT_STATUS_CLASSIFICATION_IDS,
        errors,
        "current-status classifications must use the required ids and order",
    )
    for item in current_status_classifications:
        classification_id = str(item.get("id", "")) or "<missing-id>"
        require(
            bool(str(item.get("source_of_truth", "")).strip()),
            errors,
            f"{classification_id}: current-status source_of_truth must not be empty",
        )
        require(
            bool(str(item.get("current_boundary", "")).strip()),
            errors,
            f"{classification_id}: current-status current_boundary must not be empty",
        )


def run_domain_claim_self_tests(
    repo_root: Path,
    ledger: dict[str, Any],
    contract: dict[str, Any],
) -> list[str]:
    passed: list[str] = []

    def raw_routine(candidate: dict[str, Any], routine_id: str) -> dict[str, Any]:
        for algorithm in candidate.get("algorithm", []):
            value = algorithm.get("routine", {}).get(routine_id)
            if isinstance(value, dict):
                return value
        raise KeyError(routine_id)

    def completion_errors(candidate_ledger: dict[str, Any], candidate_contract: dict[str, Any]) -> list[str]:
        errors: list[str] = []
        routines = collect_routines(candidate_ledger.get("algorithm", []), errors)
        validate_domain_completion_contract(candidate_contract, routines, errors)
        return errors

    def contract_errors(candidate_contract: dict[str, Any]) -> list[str]:
        errors: list[str] = []
        validate_contract(candidate_contract, errors)
        return errors

    def shallow_routine_errors(candidate_ledger: dict[str, Any]) -> list[str]:
        errors: list[str] = []
        routines = collect_routines(candidate_ledger.get("algorithm", []), errors)
        covered_variables = variable_names(repo_root)
        commands = command_names(repo_root)
        for routine in routines:
            validate_routine(repo_root, None, routine, covered_variables, commands, errors)
        return errors

    def expect_error(name: str, errors: list[str], token: str) -> None:
        if not any(token in error for error in errors):
            raise AssertionError(f"{name}: expected error containing {token!r}; got {errors}")
        passed.append(name)

    baseline_errors = completion_errors(ledger, contract)
    if baseline_errors:
        raise AssertionError(f"baseline domain claim contract is invalid: {baseline_errors}")
    baseline_routine_errors = shallow_routine_errors(ledger)
    if baseline_routine_errors:
        raise AssertionError(f"baseline shallow routine contract is invalid: {baseline_routine_errors}")

    current_doc = (repo_root / "docs" / "src" / "current" / "project-contract.md").read_text(
        encoding="utf-8"
    )
    candidate_doc = current_doc.replace(
        "Heat-balance, HVAC, plant, and time",
        "Heat-balance, HVAC, and plant",
        1,
    )
    mirror_errors: list[str] = []
    require_contains_all(
        candidate_doc,
        CURRENT_DOC_REQUIRED_PHRASES,
        mirror_errors,
        "docs/src/current/project-contract.md",
    )
    expect_error(
        "current_doc_requires_time_domain_mirror",
        mirror_errors,
        "missing required phrase",
    )

    algorithm_ledger_doc = (
        repo_root / "docs" / "src" / "porting-map" / "algorithm-ledger.md"
    ).read_text(encoding="utf-8")
    candidate_doc = algorithm_ledger_doc.replace(
        "The first 23\nsource-order routines",
        "The first 13\nsource-order routines",
        1,
    )
    mirror_errors = []
    require_contains_all(
        candidate_doc,
        ALGORITHM_LEDGER_DOC_REQUIRED_PHRASES,
        mirror_errors,
        "docs/src/porting-map/algorithm-ledger.md",
    )
    expect_error(
        "algorithm_ledger_doc_requires_time_domain_seed",
        mirror_errors,
        "missing required phrase",
    )

    candidate_doc = algorithm_ledger_doc.replace(
        "HVAC, plant, or time full-domain claim",
        "HVAC, or plant full-domain claim",
        1,
    )
    mirror_errors = []
    require_contains_all(
        candidate_doc,
        ALGORITHM_LEDGER_DOC_REQUIRED_PHRASES,
        mirror_errors,
        "docs/src/porting-map/algorithm-ledger.md",
    )
    expect_error(
        "algorithm_ledger_doc_requires_time_domain_claim",
        mirror_errors,
        "missing required phrase",
    )

    candidate_contract = copy.deepcopy(contract)
    candidate_contract.pop("routine_completion_schema")
    expect_error(
        "routine_completion_schema_marker_required",
        contract_errors(candidate_contract),
        "routine_completion_schema to routine_completion.v1",
    )

    candidate = copy.deepcopy(ledger)
    raw_routine(candidate, "manage_heat_balance")["completion_status"] = "ported"
    expect_error("unknown_completion_status", completion_errors(candidate, contract), "unsupported routine completion_status")

    candidate = copy.deepcopy(ledger)
    first = raw_routine(candidate, "manage_heat_balance")
    second = raw_routine(candidate, "manage_surface_heat_balance")
    second["source_file"] = first["source_file"]
    second["source_routine"] = first["source_routine"]
    expect_error("duplicate_source_routine", completion_errors(candidate, contract), "duplicate routine source mapping")

    candidate_contract = copy.deepcopy(contract)
    candidate_contract["domain_claim"][0]["required_routines"] = []
    expect_error("empty_required_routines", completion_errors(ledger, candidate_contract), "required_routines must not be empty")

    candidate_contract = copy.deepcopy(contract)
    required = candidate_contract["domain_claim"][0]["required_routines"]
    required.append(required[0])
    expect_error("duplicate_required_routines", completion_errors(ledger, candidate_contract), "must not contain duplicates")

    candidate_contract = copy.deepcopy(contract)
    candidate_contract["domain_claim"][0]["required_routines"][0] = "zone_air_heat_balance"
    expect_error("algorithm_row_is_not_a_routine", completion_errors(ledger, candidate_contract), "unknown required routine")

    candidate_contract = copy.deepcopy(contract)
    plant_routine = candidate_contract["domain_claim"][2]["required_routines"][0]
    candidate_contract["domain_claim"][0]["required_routines"][0] = plant_routine
    expect_error("wrong_domain_routine", completion_errors(ledger, candidate_contract), "required routine belongs to plant")

    candidate_contract = copy.deepcopy(contract)
    candidate_contract["domain_claim"][1]["required_routines"].remove("sim_purchased_air")
    expect_error(
        "tracked_required_routine_cannot_be_omitted",
        completion_errors(ledger, candidate_contract),
        "must exactly match required_for_full_domain routine records",
    )

    candidate_ledger = copy.deepcopy(ledger)
    candidate_contract = copy.deepcopy(contract)
    heat_claim = candidate_contract["domain_claim"][0]
    candidate_contract["claims"]["broad_heat_balance_compatibility"] = True
    for routine_id in heat_claim["required_routines"]:
        raw_routine(candidate_ledger, routine_id)["completion_status"] = "family_gated"
    expect_error("claim_requires_complete_inventory", completion_errors(candidate_ledger, candidate_contract), "routine_inventory_complete=true")

    candidate_ledger = copy.deepcopy(ledger)
    candidate_contract = copy.deepcopy(contract)
    heat_claim = candidate_contract["domain_claim"][0]
    heat_claim["routine_inventory_complete"] = True
    candidate_contract["claims"]["broad_heat_balance_compatibility"] = True
    for routine_id in heat_claim["required_routines"]:
        raw_routine(candidate_ledger, routine_id)["completion_status"] = "family_gated"
    raw_routine(candidate_ledger, heat_claim["required_routines"][0])["completion_status"] = "implemented"
    expect_error("claim_rejects_implemented_routine", completion_errors(candidate_ledger, candidate_contract), "every required routine at family_gated or complete")

    candidate_ledger = copy.deepcopy(ledger)
    raw_routine(candidate_ledger, "manage_heat_balance")["completion_status"] = "family_gated"
    expect_error(
        "status_inflation_requires_phase_evidence",
        shallow_routine_errors(candidate_ledger),
        "state_mapping_ref must not be empty",
    )

    candidate_ledger = copy.deepcopy(ledger)
    candidate_contract = copy.deepcopy(contract)
    heat_claim = candidate_contract["domain_claim"][0]
    heat_claim["routine_inventory_complete"] = True
    candidate_contract["claims"]["broad_heat_balance_compatibility"] = True
    for routine_id in heat_claim["required_routines"]:
        raw_routine(candidate_ledger, routine_id)["completion_status"] = "family_gated"
    positive_errors = completion_errors(candidate_ledger, candidate_contract)
    if positive_errors:
        raise AssertionError(f"family_gated full-domain threshold should pass: {positive_errors}")
    passed.append("family_gated_full_domain_threshold")

    candidate_ledger = copy.deepcopy(ledger)
    routine = raw_routine(candidate_ledger, "sim_purchased_air")
    routine.update(
        {
            "completion_status": "family_gated",
            "state_mapping_ref": routine["source_map"],
            "read_state": ["self-test input state"],
            "write_state": ["self-test output state"],
            "history_state_ownership": "self-test runtime ownership",
            "unsupported_state": [],
            "inactive_branches": [],
            "unsupported_active_branches": [],
            "not_claimed_branches": [],
            "rust_target": ["crates/ep_runtime/src/ideal_loads/dispatch.rs::sim_purchased_air_compat"],
            "family_gate_ids": ["plant_loop_diagnostic_001"],
            "proof_variables": ["Plant Supply Side Inlet Temperature"],
        }
    )
    expect_error("diagnostic_case_cannot_family_gate", shallow_routine_errors(candidate_ledger), "family_gated requires a conformance case")

    candidate_ledger = copy.deepcopy(ledger)
    for algorithm in candidate_ledger.get("algorithm", []):
        if algorithm.get("id") == "plant_loop_state_projection":
            algorithm["family_cases"] = ["ideal_loads_no_oa_sensible_conformance_001"]
            break
    routine = raw_routine(candidate_ledger, "manage_plant_loops")
    routine.update(
        {
            "completion_status": "family_gated",
            "state_mapping_ref": routine["source_map"],
            "read_state": ["self-test input state"],
            "write_state": ["self-test output state"],
            "history_state_ownership": "self-test runtime ownership",
            "unsupported_state": [],
            "inactive_branches": [],
            "unsupported_active_branches": [],
            "not_claimed_branches": [],
            "rust_target": ["crates/ep_runtime/src/plant/state.rs::simulate_plant_state_projection"],
            "family_gate_ids": ["ideal_loads_no_oa_sensible_conformance_001"],
            "proof_variables": ["Zone Ideal Loads Zone Total Heating Rate"],
        }
    )
    expect_error(
        "cross_domain_case_cannot_family_gate",
        shallow_routine_errors(candidate_ledger),
        "scope does not cover routine domain plant",
    )

    candidate_ledger = copy.deepcopy(ledger)
    for algorithm in candidate_ledger.get("algorithm", []):
        if algorithm.get("id") == "heat_balance_manager_source_order":
            algorithm["family_cases"] = ["official_1zone_static_model_001"]
            break
    routine = raw_routine(candidate_ledger, "manage_heat_balance")
    routine.update(
        {
            "completion_status": "family_gated",
            "state_mapping_ref": routine["source_map"],
            "read_state": [routine["source_routine"]],
            "write_state": [routine["source_routine"]],
            "history_state_ownership": routine["source_routine"],
            "unsupported_state": [],
            "inactive_branches": [],
            "unsupported_active_branches": [],
            "not_claimed_branches": [],
            "rust_target": ["crates/ep_runtime/src/heat_balance/manager.rs::manage_heat_balance_source_order_stages"],
            "family_gate_ids": ["official_1zone_static_model_001"],
            "proof_variables": ["HeatTransfer Surface Area (Net)"],
        }
    )
    expect_error(
        "same_domain_case_requires_explicit_routine_coverage",
        shallow_routine_errors(candidate_ledger),
        "family gate must declare routine in routine_coverage.routine_ids",
    )

    candidate_ledger = copy.deepcopy(ledger)
    routine = raw_routine(candidate_ledger, "sim_purchased_air")
    routine.update(
        {
            "completion_status": "family_gated",
            "state_mapping_ref": routine["source_map"],
            "read_state": ["self-test input state"],
            "write_state": ["self-test output state"],
            "history_state_ownership": "self-test runtime ownership",
            "unsupported_state": [],
            "inactive_branches": [],
            "unsupported_active_branches": [],
            "not_claimed_branches": [],
            "rust_target": ["crates/ep_runtime/src/ideal_loads/dispatch.rs::sim_purchased_air_compat"],
            "family_gate_ids": ["ideal_loads_no_oa_sensible_conformance_001"],
            "proof_variables": ["Zone Ideal Loads Supply Air Total Heating Energy"],
        }
    )
    expect_error(
        "diagnostic_output_cannot_prove_family_gate",
        shallow_routine_errors(candidate_ledger),
        "proof variable is not requested by any family gate",
    )

    candidate_contract = copy.deepcopy(contract)
    candidate_contract["claims"]["full_runtime_compatibility"] = True
    expect_error("full_runtime_stays_locked", completion_errors(ledger, candidate_contract), "full runtime compatibility remains locked")
    return passed


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    contract_path = repo_root / "specs" / "project_contract.toml"
    ledger_path = repo_root / "specs" / "algorithm_ledger.toml"
    readme_path = repo_root / "README.md"
    current_doc_path = repo_root / "docs" / "src" / "current" / "project-contract.md"
    algorithm_ledger_doc_path = repo_root / "docs" / "src" / "porting-map" / "algorithm-ledger.md"
    current_status_path = repo_root / "docs" / "src" / "current" / "current-status.md"
    generated_case_index_path = repo_root / "docs" / "src" / "generated" / "conformance-case-index.md"
    generated_variable_coverage_path = repo_root / "docs" / "src" / "generated" / "variable-coverage.md"
    generated_algorithm_ledger_path = repo_root / "docs" / "src" / "generated" / "algorithm-ledger.md"
    generated_current_status_path = repo_root / "docs" / "src" / "generated" / "current-status-classification.md"
    errors: list[str] = []
    contract: dict[str, Any] = {}
    ledger: dict[str, Any] = {}
    self_test_results: list[str] = []

    require(contract_path.is_file(), errors, f"missing project contract spec: {contract_path}")
    require(ledger_path.is_file(), errors, f"missing algorithm ledger spec: {ledger_path}")
    require(readme_path.is_file(), errors, f"missing README: {readme_path}")
    require(current_doc_path.is_file(), errors, f"missing current project contract doc: {current_doc_path}")
    require(algorithm_ledger_doc_path.is_file(), errors, f"missing algorithm ledger doc: {algorithm_ledger_doc_path}")
    require(current_status_path.is_file(), errors, f"missing current status doc: {current_status_path}")
    require(generated_case_index_path.is_file(), errors, f"missing generated case index: {generated_case_index_path}")
    require(
        generated_variable_coverage_path.is_file(),
        errors,
        f"missing generated variable coverage: {generated_variable_coverage_path}",
    )
    require(
        generated_algorithm_ledger_path.is_file(),
        errors,
        f"missing generated algorithm ledger: {generated_algorithm_ledger_path}",
    )
    require(
        generated_current_status_path.is_file(),
        errors,
        f"missing generated current-status classification: {generated_current_status_path}",
    )
    if contract_path.is_file():
        contract = load_toml(contract_path)
        validate_contract(contract, errors)
    if ledger_path.is_file():
        ledger = load_toml(ledger_path)
        algorithms = ledger.get("algorithm", [])
        require(isinstance(algorithms, list), errors, "algorithm ledger must contain [[algorithm]] records")
        if isinstance(algorithms, list):
            routines = collect_routines(algorithms, errors)
            require(bool(routines), errors, "algorithm ledger must contain routine completion records")
            covered_variables = variable_names(repo_root)
            commands = command_names(repo_root)
            for routine in routines:
                validate_routine(repo_root, None, routine, covered_variables, commands, errors)
            if contract:
                validate_domain_completion_contract(contract, routines, errors)
    if readme_path.is_file():
        require_contains_all(readme_path.read_text(encoding="utf-8"), README_REQUIRED_PHRASES, errors, "README.md")
    if current_doc_path.is_file():
        require_contains_all(
            current_doc_path.read_text(encoding="utf-8"),
            CURRENT_DOC_REQUIRED_PHRASES,
            errors,
            "docs/src/current/project-contract.md",
        )
    if algorithm_ledger_doc_path.is_file():
        require_contains_all(
            algorithm_ledger_doc_path.read_text(encoding="utf-8"),
            ALGORITHM_LEDGER_DOC_REQUIRED_PHRASES,
            errors,
            "docs/src/porting-map/algorithm-ledger.md",
        )
    if current_status_path.is_file():
        require_contains_all(
            current_status_path.read_text(encoding="utf-8"),
            CURRENT_STATUS_REQUIRED_PHRASES,
            errors,
            "docs/src/current/current-status.md",
        )
    generated_text = ""
    for path in [
        generated_case_index_path,
        generated_variable_coverage_path,
        generated_algorithm_ledger_path,
        generated_current_status_path,
    ]:
        if path.is_file():
            generated_text += "\n" + path.read_text(encoding="utf-8")
    require_contains_all(generated_text, GENERATED_DOC_REQUIRED_PHRASES, errors, "generated docs")
    if generated_current_status_path.is_file():
        classification_text = generated_current_status_path.read_text(encoding="utf-8")
        for item in contract.get("current_status_classification", []):
            expected_row = (
                f"| {markdown_cell(item.get('id', ''))} | "
                f"{markdown_cell(item.get('source_of_truth', ''))} | "
                f"{markdown_cell(item.get('current_boundary', ''))} |"
            )
            require(
                expected_row in classification_text,
                errors,
                f"generated current-status classification missing row: {item.get('id', '')}",
            )

    if args.self_test and not errors:
        try:
            self_test_results = run_domain_claim_self_tests(repo_root, ledger, contract)
        except AssertionError as error:
            errors.append(f"domain claim self-test failed: {error}")

    if errors:
        print("Project contract validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Project contract check")
    print(f"  oracle: EnergyPlus {EXPECTED_VERSION}")
    print("  rust_only: valid")
    print("  compatibility_contract: valid")
    print("  routine_completion_gate: valid")
    if args.self_test:
        print(f"  mutation_self_tests: {len(self_test_results)}")
    print("  readme_alignment: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
