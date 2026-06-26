"""Validate conformance claim source-of-truth links across specs and manifests."""

from __future__ import annotations

import argparse
import json
import sys
import tomllib
from pathlib import Path
from typing import Any


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate conformance claim source-of-truth links.")
    parser.add_argument("--repo-root", required=True, type=Path)
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def request_name(request: dict[str, Any]) -> str:
    return str(request.get("variable", request.get("name", ""))).strip()


def case_requests(case: dict[str, Any]) -> list[dict[str, Any]]:
    return [*case.get("outputs", []), *case.get("meters", [])]


def output_requests(case: dict[str, Any]) -> list[dict[str, Any]]:
    return list(case.get("outputs", []))


def request_has_tolerance(request: dict[str, Any]) -> bool:
    return any(key in request for key in ("abs_tol", "rmse_tol", "rel_tol"))


def case_has_tolerance(case: dict[str, Any]) -> bool:
    return bool(case.get("tolerances")) or any(request_has_tolerance(request) for request in case_requests(case))


def conformance_requests(case: dict[str, Any]) -> list[dict[str, Any]]:
    return [request for request in case_requests(case) if str(request.get("level", "")) == "conformance"]


def conformance_output_variables(case: dict[str, Any]) -> set[str]:
    return {
        request_name(request)
        for request in output_requests(case)
        if str(request.get("level", "")) == "conformance"
    }


def dev_command_from_gate(script: str) -> str:
    parts = script.replace("\\", "/").split()
    for index, part in enumerate(parts):
        if part.endswith("scripts/dev.cmd") or part.endswith("scripts/dev.ps1"):
            if index + 1 < len(parts):
                return parts[index + 1]
    return ""


def load_cases(repo_root: Path, errors: list[str]) -> dict[str, dict[str, Any]]:
    case_root = repo_root / "data" / "conformance_cases"
    require(case_root.is_dir(), errors, f"missing conformance case directory: {case_root}")
    cases: dict[str, dict[str, Any]] = {}
    if not case_root.is_dir():
        return cases
    for path in sorted(case_root.glob("*/case.toml")):
        case = load_toml(path)
        case_id = str(case.get("id", "")).strip()
        prefix = case_id or path.parent.name
        require(bool(case_id), errors, f"{path}: case id must not be empty")
        require(case_id == path.parent.name, errors, f"{prefix}: case id must match directory name")
        require(case_id not in cases, errors, f"duplicate case id: {case_id}")
        cases[case_id] = case
    require(bool(cases), errors, "no conformance case manifests found")
    return cases


def validate_case(case_id: str, case: dict[str, Any], commands: set[str], errors: list[str]) -> None:
    comparison_class = str(case.get("comparison_class", "")).strip()
    conformance_claim = case.get("conformance_claim")
    gate = case.get("gate") or {}
    report = case.get("report") or {}
    gate_script = str(gate.get("script", "")).strip()
    gate_command = dev_command_from_gate(gate_script)
    conformance_rows = conformance_requests(case)

    require(conformance_claim in {True, False}, errors, f"{case_id}: conformance_claim must be boolean")

    if conformance_claim is True:
        require(comparison_class == "conformance", errors, f"{case_id}: conformance claim requires comparison_class=conformance")
        require(bool(conformance_rows), errors, f"{case_id}: conformance claim requires conformance-level outputs or meters")
        require(case_has_tolerance(case), errors, f"{case_id}: conformance claim requires tolerance metadata")
        require(bool(report.get("path")), errors, f"{case_id}: conformance claim requires report.path")
        require(bool(gate_script), errors, f"{case_id}: conformance claim requires gate.script")
        require(gate.get("blocking") is True, errors, f"{case_id}: conformance claim requires gate.blocking=true")
        require(gate_command in commands, errors, f"{case_id}: gate script must call a registered dev command: {gate_script}")
    else:
        require(not conformance_rows, errors, f"{case_id}: non-conformance case must not request conformance-level evidence")
        if comparison_class == "diagnostic-only":
            require(conformance_claim is False, errors, f"{case_id}: diagnostic-only case must keep conformance_claim=false")
        if any(str(request.get("level", "")) == "baseline" for request in case_requests(case)):
            require(conformance_claim is False, errors, f"{case_id}: baseline-only evidence must keep conformance_claim=false")


def validate_variable_coverage(
    variables: list[dict[str, Any]],
    cases: dict[str, dict[str, Any]],
    errors: list[str],
) -> set[str]:
    conformance_names: set[str] = set()
    for variable in variables:
        name = str(variable.get("name", "")).strip()
        status = str(variable.get("status", "")).strip()
        first_case = str(variable.get("first_case", "")).strip()
        prefix = name or "<missing-variable>"
        if status != "conformance":
            continue
        conformance_names.add(name)
        case = cases.get(first_case)
        require(case is not None, errors, f"{prefix}: conformance variable first_case does not exist: {first_case}")
        if case is None:
            continue
        require(case.get("conformance_claim") is True, errors, f"{prefix}: conformance variable requires conformance_claim=true case")
        require(bool((case.get("report") or {}).get("path")), errors, f"{prefix}: conformance variable requires report path")
        require((case.get("gate") or {}).get("blocking") is True, errors, f"{prefix}: conformance variable requires blocking gate")
        require(case_has_tolerance(case), errors, f"{prefix}: conformance variable requires tolerance metadata")
        requested = {
            request_name(request)
            for request in output_requests(case)
            if str(request.get("level", "")) == "conformance"
        }
        require(name in requested, errors, f"{prefix}: first_case must request the variable at level=conformance")
    return conformance_names


def validate_algorithm_ledger(
    repo_root: Path,
    algorithms: list[dict[str, Any]],
    cases: dict[str, dict[str, Any]],
    conformance_variable_names: set[str],
    errors: list[str],
) -> int:
    conformance_count = 0
    for algorithm in algorithms:
        status = str(algorithm.get("status", "")).strip()
        if status != "conformance":
            continue
        conformance_count += 1
        algorithm_id = str(algorithm.get("id", "")).strip() or "<missing-algorithm>"
        rust_targets = [str(target).strip() for target in algorithm.get("rust_target", [])]
        proof_variables = [str(variable).strip() for variable in algorithm.get("proof_variables", [])]
        first_case = str(algorithm.get("first_case", "")).strip()
        case = cases.get(first_case)
        rust_anchor_count = 0

        require(bool(rust_targets), errors, f"{algorithm_id}: conformance algorithm requires rust_target entries")
        for target in rust_targets:
            target_file = target.split("::", 1)[0].strip()
            require(bool(target_file), errors, f"{algorithm_id}: rust_target must include a file path: {target}")
            if target_file:
                require((repo_root / target_file).is_file(), errors, f"{algorithm_id}: rust_target file does not exist: {target_file}")
            if target_file.startswith("crates/") and "::" in target:
                rust_anchor_count += 1
        require(rust_anchor_count > 0, errors, f"{algorithm_id}: conformance algorithm requires at least one Rust module/function anchor")
        require(bool(proof_variables), errors, f"{algorithm_id}: conformance algorithm requires proof_variables")
        for variable in proof_variables:
            require(
                variable in conformance_variable_names,
                errors,
                f"{algorithm_id}: proof variable must be status=conformance in specs/variable_coverage.toml: {variable}",
            )
        require(case is not None, errors, f"{algorithm_id}: first_case manifest does not exist: {first_case}")
        if case is not None:
            require(case.get("conformance_claim") is True, errors, f"{algorithm_id}: first_case must be conformance_claim=true")
    return conformance_count


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    errors: list[str] = []

    variable_path = repo_root / "specs" / "variable_coverage.toml"
    algorithm_path = repo_root / "specs" / "algorithm_ledger.toml"
    commands_path = repo_root / "scripts" / "dev" / "commands.json"

    require(variable_path.is_file(), errors, f"missing variable coverage spec: {variable_path}")
    require(algorithm_path.is_file(), errors, f"missing algorithm ledger spec: {algorithm_path}")
    require(commands_path.is_file(), errors, f"missing dev command catalog: {commands_path}")
    cases = load_cases(repo_root, errors)

    variables = load_toml(variable_path).get("variable", []) if variable_path.is_file() else []
    algorithms = load_toml(algorithm_path).get("algorithm", []) if algorithm_path.is_file() else []
    commands = {
        str(command.get("name", "")).strip()
        for command in load_json(commands_path).get("commands", [])
    } if commands_path.is_file() else set()

    conformance_variable_names = validate_variable_coverage(variables, cases, errors)
    for case_id, case in cases.items():
        validate_case(case_id, case, commands, errors)
    conformance_algorithm_count = validate_algorithm_ledger(repo_root, algorithms, cases, conformance_variable_names, errors)

    if errors:
        print("Claim source validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    conformance_case_count = sum(1 for case in cases.values() if case.get("conformance_claim") is True)
    print("Claim source check")
    print("  source_of_truth: case manifests + variable_coverage + algorithm_ledger")
    print(f"  cases: {len(cases)}")
    print(f"  conformance_cases: {conformance_case_count}")
    print(f"  conformance_variables: {len(conformance_variable_names)}")
    print(f"  conformance_algorithms: {conformance_algorithm_count}")
    print("  status: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
