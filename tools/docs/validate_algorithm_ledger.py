"""Validate the source-map and algorithm ledger contract."""

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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate specs/algorithm_ledger.toml.")
    parser.add_argument("--repo-root", required=True, type=Path)
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def path_before_anchor(value: str) -> str:
    return value.split("::", 1)[0]


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def validate_algorithm(repo_root: Path, reference_root: Path, algorithm: dict[str, Any], errors: list[str]) -> None:
    algorithm_id = str(algorithm.get("id", "")).strip()
    prefix = algorithm_id or "<missing-id>"

    require(bool(algorithm_id), errors, "algorithm id must not be empty")
    require(bool(str(algorithm.get("domain", "")).strip()), errors, f"{prefix}: domain must not be empty")
    status = str(algorithm.get("status", "")).strip()
    require(status in ALLOWED_STATUS, errors, f"{prefix}: unsupported status {status!r}")

    source_map = str(algorithm.get("source_map", "")).strip()
    require(bool(source_map), errors, f"{prefix}: source_map must not be empty")
    if source_map:
        source_map_path = repo_root / path_before_anchor(source_map)
        require(source_map_path.is_file(), errors, f"{prefix}: source_map does not exist: {source_map}")
        if source_map_path.is_file():
            text = source_map_path.read_text(encoding="utf-8", errors="replace")
            require(
                "Reference version: EnergyPlus 26.1.0" in text,
                errors,
                f"{prefix}: source_map must pin EnergyPlus 26.1.0",
            )

    energyplus_sources = algorithm.get("energyplus_source", [])
    require(isinstance(energyplus_sources, list) and bool(energyplus_sources), errors, f"{prefix}: energyplus_source must not be empty")
    for source in energyplus_sources:
        source_path = reference_root / path_before_anchor(str(source))
        require(source_path.is_file(), errors, f"{prefix}: EnergyPlus source does not exist: {source}")

    rust_targets = algorithm.get("rust_target", [])
    require(isinstance(rust_targets, list) and bool(rust_targets), errors, f"{prefix}: rust_target must not be empty")
    for target in rust_targets:
        target_path = repo_root / path_before_anchor(str(target))
        require(target_path.is_file(), errors, f"{prefix}: Rust target does not exist: {target}")

    first_case = str(algorithm.get("first_case", "")).strip()
    require(bool(first_case), errors, f"{prefix}: first_case must not be empty")
    case_path = repo_root / "data" / "conformance_cases" / first_case / "case.toml"
    require(case_path.is_file(), errors, f"{prefix}: first_case manifest does not exist: {first_case}")

    proof_variables = [str(value).strip() for value in algorithm.get("proof_variables", [])]
    require(bool(proof_variables), errors, f"{prefix}: proof_variables must not be empty")

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
        outputs = case.get("outputs", [])
        output_variables = {str(output.get("variable", "")) for output in outputs}
        if status == "conformance":
            require(case.get("comparison_class") == "conformance", errors, f"{prefix}: conformance entry requires conformance case")
            require(case.get("conformance_claim") is True, errors, f"{prefix}: conformance entry requires conformance_claim=true")
            require(bool((case.get("gate") or {}).get("blocking")), errors, f"{prefix}: conformance claim requires blocking gate")
            for variable in proof_variables:
                require(variable in output_variables, errors, f"{prefix}: proof variable is not requested by first_case: {variable}")
        elif status == "diagnostic_only":
            require(case.get("conformance_claim") is False, errors, f"{prefix}: diagnostic entry must not use a conformance claim")


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    reference_root = repo_root / ".reference" / "energyplus-src" / "26.1.0"
    ledger_path = repo_root / "specs" / "algorithm_ledger.toml"
    errors: list[str] = []

    require(reference_root.is_dir(), errors, f"missing EnergyPlus reference source root: {reference_root}")
    require(ledger_path.is_file(), errors, f"missing algorithm ledger spec: {ledger_path}")
    if not ledger_path.is_file():
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    spec = load_toml(ledger_path)
    algorithms = spec.get("algorithm", [])
    require(isinstance(algorithms, list) and bool(algorithms), errors, "algorithm ledger must contain at least one [[algorithm]]")

    seen_ids: set[str] = set()
    for algorithm in algorithms:
        algorithm_id = str(algorithm.get("id", "")).strip()
        require(algorithm_id not in seen_ids, errors, f"duplicate algorithm id: {algorithm_id}")
        seen_ids.add(algorithm_id)
        validate_algorithm(repo_root, reference_root, algorithm, errors)

    if errors:
        print("Algorithm ledger validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Algorithm ledger check")
    print(f"  algorithms: {len(algorithms)}")
    print("  rule: No source map, no algorithm port.")
    print("  status: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
