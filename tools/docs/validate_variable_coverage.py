"""Validate output-variable coverage counts and claim boundaries."""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
from collections import Counter
from pathlib import Path
from typing import Any


ALLOWED_STATUS = {"conformance", "diagnostic", "baseline"}
CURRENT_STATUS_COUNT_RE = re.compile(
    r"tracks\s+(?P<conformance>\d+)\s+conformance output variables,\s+"
    r"(?P<diagnostic>\d+)\s+diagnostic output variables,\s+and\s+"
    r"(?P<baseline>\d+)\s+baseline output variables,\s+for\s+"
    r"(?P<total>\d+)\s+tracked output variables",
    re.IGNORECASE | re.DOTALL,
)
README_DIRECT_COUNT_RE = re.compile(
    r"(?i)(?:\b\d+\s+(?:conformance|diagnostic|baseline|tracked)\s+output variables"
    r"|\b\d+\s+passed release-evidence series)"
)
BRANCH_TOKENS = (
    "no-oa",
    "capacity-limit",
    "flow-limit",
    "constant",
    "humidistat",
    "outdoor-air",
    "economizer",
    "heat-recovery",
    "reportpurchasedair",
    "fuel-efficiency",
    "facility meter",
    "meter",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate specs/variable_coverage.toml.")
    parser.add_argument("--repo-root", required=True, type=Path)
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def request_variable(request: dict[str, Any]) -> str:
    return str(request.get("variable", request.get("name", "")))


def case_requests(case: dict[str, Any]) -> list[dict[str, Any]]:
    return [*case.get("outputs", []), *case.get("meters", [])]


def request_has_tolerance(request: dict[str, Any]) -> bool:
    return any(key in request for key in ("abs_tol", "rmse_tol", "rel_tol"))


def case_has_tolerance(case: dict[str, Any]) -> bool:
    return bool(case.get("tolerances")) or any(request_has_tolerance(request) for request in case_requests(case))


def first_matching_request(case: dict[str, Any], variable: str) -> dict[str, Any] | None:
    for request in case_requests(case):
        if request_variable(request) == variable:
            return request
    return None


def matching_requests(case: dict[str, Any], variable: str) -> list[dict[str, Any]]:
    return [request for request in case_requests(case) if request_variable(request) == variable]


def load_cases(repo_root: Path) -> dict[str, dict[str, Any]]:
    cases: dict[str, dict[str, Any]] = {}
    for path in (repo_root / "data" / "conformance_cases").glob("*/case.toml"):
        case = load_toml(path)
        case_id = str(case.get("id", path.parent.name))
        cases[case_id] = case
    return cases


def validate_generated_summary(
    repo_root: Path,
    counts: Counter[str],
    total: int,
    errors: list[str],
) -> None:
    generated_path = repo_root / "docs" / "src" / "generated" / "variable-coverage.md"
    require(generated_path.is_file(), errors, f"missing generated variable coverage doc: {generated_path}")
    if not generated_path.is_file():
        return
    text = read_text(generated_path)
    for status in ("conformance", "diagnostic", "baseline"):
        require(
            f"| {status} | {counts.get(status, 0)} |" in text,
            errors,
            f"generated variable coverage summary missing {status} count {counts.get(status, 0)}",
        )
    require(f"| total | {total} |" in text, errors, f"generated variable coverage summary missing total count {total}")


def validate_current_status(
    repo_root: Path,
    counts: Counter[str],
    total: int,
    errors: list[str],
) -> None:
    current_path = repo_root / "docs" / "src" / "current" / "current-status.md"
    require(current_path.is_file(), errors, f"missing current status doc: {current_path}")
    if not current_path.is_file():
        return
    text = read_text(current_path)
    match = CURRENT_STATUS_COUNT_RE.search(" ".join(text.split()))
    require(match is not None, errors, "current-status.md must state generated variable status counts")
    if match is None:
        return
    expected = {
        "conformance": counts.get("conformance", 0),
        "diagnostic": counts.get("diagnostic", 0),
        "baseline": counts.get("baseline", 0),
        "total": total,
    }
    for key, value in expected.items():
        found = int(match.group(key))
        require(found == value, errors, f"current-status.md {key} variable count {found} != generated {value}")


def validate_readme(repo_root: Path, errors: list[str]) -> None:
    readme_path = repo_root / "README.md"
    require(readme_path.is_file(), errors, f"missing README: {readme_path}")
    if not readme_path.is_file():
        return
    match = README_DIRECT_COUNT_RE.search(read_text(readme_path))
    require(match is None, errors, f"README.md must not hard-code variable coverage counts: {match.group(0) if match else ''}")


def validate_variable(
    variable: dict[str, Any],
    cases: dict[str, dict[str, Any]],
    all_conformance_variables: set[str],
    errors: list[str],
) -> None:
    name = str(variable.get("name", "")).strip()
    prefix = name or "<missing-name>"
    status = str(variable.get("status", "")).strip()
    first_case = str(variable.get("first_case", "")).strip()
    first_evidence = str(variable.get("first_evidence", first_case)).strip()
    boundary = str(variable.get("support_boundary", "")).strip()
    boundary_lower = boundary.lower()

    require(bool(name), errors, "variable name must not be empty")
    require(bool(str(variable.get("domain", "")).strip()), errors, f"{prefix}: domain must not be empty")
    require(status in ALLOWED_STATUS, errors, f"{prefix}: unsupported status {status!r}")
    require(bool(first_case), errors, f"{prefix}: first_case must not be empty")
    require(bool(first_evidence), errors, f"{prefix}: first_evidence must not be empty")
    require(bool(boundary), errors, f"{prefix}: support_boundary must not be empty")

    case = cases.get(first_case)
    require(case is not None, errors, f"{prefix}: first_case does not exist: {first_case}")
    require(first_evidence in cases, errors, f"{prefix}: first_evidence does not exist: {first_evidence}")
    if case is None:
        return

    requests = matching_requests(case, name)
    require(bool(requests), errors, f"{prefix}: first_case does not request this variable")

    if status == "conformance":
        conformance_request = next(
            (request for request in requests if str(request.get("level", "")) == "conformance"),
            None,
        )
        require(
            conformance_request is not None,
            errors,
            f"{prefix}: conformance variable first_case request must be level=conformance",
        )
        require(case.get("comparison_class") == "conformance", errors, f"{prefix}: conformance variable requires conformance case")
        require(case.get("conformance_claim") is True, errors, f"{prefix}: conformance variable requires conformance_claim=true")
        require(bool((case.get("report") or {}).get("path")), errors, f"{prefix}: conformance variable requires report path")
        require(bool((case.get("gate") or {}).get("script")), errors, f"{prefix}: conformance variable requires gate script")
        require(case_has_tolerance(case), errors, f"{prefix}: conformance variable requires tolerance metadata")
        require(
            "tolerance-gated" in boundary_lower or "static eio conformance" in boundary_lower,
            errors,
            f"{prefix}: conformance boundary must state tolerance-gated or static EIO scope",
        )
    elif status == "diagnostic":
        require(
            name not in all_conformance_variables,
            errors,
            f"{prefix}: diagnostic variable is requested as conformance elsewhere",
        )
        require("diagnostic" in boundary_lower, errors, f"{prefix}: diagnostic boundary must state diagnostic scope")
    elif status == "baseline":
        require(
            name not in all_conformance_variables,
            errors,
            f"{prefix}: baseline variable is requested as conformance elsewhere",
        )
        require("baseline" in boundary_lower, errors, f"{prefix}: baseline boundary must state baseline scope")
        require("supported" not in boundary_lower, errors, f"{prefix}: baseline boundary must not say supported")

    if name.startswith("Zone Ideal Loads") or first_case.startswith("ideal_loads_"):
        require(
            any(token in boundary_lower for token in BRANCH_TOKENS),
            errors,
            f"{prefix}: IdealLoads boundary must identify branch-level scope",
        )

    if first_case.startswith("official_1zone_uncontrolled") or "1zoneuncontrolled" in boundary_lower:
        if status == "diagnostic":
            require("diagnostic" in boundary_lower, errors, f"{prefix}: 1Zone diagnostic boundary must be explicit")
        else:
            require(
                "candidate" in boundary_lower
                or "compatibility lane" in boundary_lower
                or "official 1zoneuncontrolled dynamic" in boundary_lower,
                errors,
                f"{prefix}: 1Zone conformance boundary must distinguish the official dynamic candidate",
            )

    name_lower = name.lower()
    if name.startswith("Zone Ideal Loads") and "fuel energy rate" in name_lower:
        require(
            "fuel" in boundary_lower and "rate" in boundary_lower,
            errors,
            f"{prefix}: fuel-energy rate boundary must state fuel and rate layers",
        )
    elif name.startswith("Zone Ideal Loads") and "fuel energy" in name_lower:
        require("fuel" in boundary_lower, errors, f"{prefix}: fuel-energy boundary must state fuel layer")
    elif name.startswith("Zone Ideal Loads") and " energy" in name_lower:
        require("energy" in boundary_lower, errors, f"{prefix}: energy boundary must state energy layer")

    if name.startswith("Zone Ideal Loads") and "meter" in boundary_lower:
        require(
            "meter" in boundary_lower and ("energy" in boundary_lower or "fuel" in boundary_lower or "aggregation" in boundary_lower),
            errors,
            f"{prefix}: meter boundary must separate meter aggregation from rate/energy/fuel layers",
        )


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    spec_path = repo_root / "specs" / "variable_coverage.toml"
    errors: list[str] = []

    require(spec_path.is_file(), errors, f"missing variable coverage spec: {spec_path}")
    if not spec_path.is_file():
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    spec = load_toml(spec_path)
    variables = spec.get("variable", [])
    require(isinstance(variables, list) and bool(variables), errors, "variable coverage must contain at least one [[variable]]")
    counts = Counter(str(variable.get("status", "")) for variable in variables)
    total = len(variables)

    seen_names: set[str] = set()
    cases = load_cases(repo_root)
    all_conformance_variables = {
        request_variable(request)
        for case in cases.values()
        for request in case_requests(case)
        if str(request.get("level", "")) == "conformance"
    }

    for variable in variables:
        name = str(variable.get("name", "")).strip()
        require(name not in seen_names, errors, f"duplicate variable coverage entry: {name}")
        seen_names.add(name)
        validate_variable(variable, cases, all_conformance_variables, errors)

    validate_generated_summary(repo_root, counts, total, errors)
    validate_current_status(repo_root, counts, total, errors)
    validate_readme(repo_root, errors)

    if errors:
        print("Variable coverage validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Variable coverage check")
    print(f"  total: {total}")
    print(f"  conformance: {counts.get('conformance', 0)}")
    print(f"  diagnostic: {counts.get('diagnostic', 0)}")
    print(f"  baseline: {counts.get('baseline', 0)}")
    print("  status: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
