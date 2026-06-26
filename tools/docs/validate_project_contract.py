"""Validate the project contract spec and its README/current-doc mirrors."""

from __future__ import annotations

import argparse
import sys
import tomllib
from pathlib import Path
from typing import Any


EXPECTED_VERSION = "26.1.0"
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
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate project contract source-of-truth alignment.")
    parser.add_argument("--repo-root", required=True, type=Path)
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


def validate_contract(contract: dict[str, Any], errors: list[str]) -> None:
    oracle = contract.get("oracle", {})
    language = contract.get("language", {})
    claims = contract.get("claims", {})
    optimization = contract.get("optimization", {})
    modes = {str(mode.get("id", "")): mode for mode in contract.get("mode", [])}
    diagnostic_only = contract.get("diagnostic_only", {})
    partial_runs = contract.get("partial_runs", {})
    documentation = contract.get("documentation", {})

    require(oracle.get("energyplus_version") == EXPECTED_VERSION, errors, "project contract must pin EnergyPlus 26.1.0")
    require(oracle.get("compatibility_mode_required") is True, errors, "project contract must require compatibility mode")
    require(language.get("core") == "Rust", errors, "project contract must keep core language Rust")
    require(language.get("mixed_language_kernels") is False, errors, "project contract must forbid mixed-language kernels")
    require(claims.get("arbitrary_runs_are_release_evidence") is False, errors, "arbitrary runs must not be release evidence")
    require(claims.get("full_runtime_compatibility") is False, errors, "full runtime compatibility must not be claimed")
    require(claims.get("hvac_compatibility") is False, errors, "HVAC compatibility must not be broadly claimed")
    require(claims.get("plant_compatibility") is False, errors, "plant compatibility must not be broadly claimed")

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


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    contract_path = repo_root / "specs" / "project_contract.toml"
    readme_path = repo_root / "README.md"
    current_doc_path = repo_root / "docs" / "src" / "current" / "project-contract.md"
    errors: list[str] = []

    require(contract_path.is_file(), errors, f"missing project contract spec: {contract_path}")
    require(readme_path.is_file(), errors, f"missing README: {readme_path}")
    require(current_doc_path.is_file(), errors, f"missing current project contract doc: {current_doc_path}")
    if contract_path.is_file():
        validate_contract(load_toml(contract_path), errors)
    if readme_path.is_file():
        require_contains_all(readme_path.read_text(encoding="utf-8"), README_REQUIRED_PHRASES, errors, "README.md")
    if current_doc_path.is_file():
        require_contains_all(
            current_doc_path.read_text(encoding="utf-8"),
            CURRENT_DOC_REQUIRED_PHRASES,
            errors,
            "docs/src/current/project-contract.md",
        )

    if errors:
        print("Project contract validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Project contract check")
    print(f"  oracle: EnergyPlus {EXPECTED_VERSION}")
    print("  rust_only: valid")
    print("  compatibility_contract: valid")
    print("  readme_alignment: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
