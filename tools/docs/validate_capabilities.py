"""Validate capability registry wiring, evidence mappings, and artifact fields."""

from __future__ import annotations

import argparse
import sys
import tomllib
from pathlib import Path
from typing import Any


EXPECTED_CAPABILITY_IDS = {
    "official_1zone_uncontrolled_declared_heat_balance",
    "ideal_loads_no_oa_sensible",
    "ideal_loads_finite_limits",
    "ideal_loads_constant_shr",
    "ideal_loads_humidity_selected_branches",
    "ideal_loads_outdoor_air_selected_branches",
}
EXPECTED_RUNTIME_SELECTED_CAPABILITY_IDS = EXPECTED_CAPABILITY_IDS - {
    "ideal_loads_outdoor_air_selected_branches",
}
REQUIRED_CAPABILITY_FIELDS = {
    "id",
    "required_objects",
    "forbidden_active_features",
    "algorithms",
    "claim_boundary",
    "run_state",
    "evidence_cases",
}
EXPECTED_OFFICIAL_CASES = {
    "official_1zone_uncontrolled_dynamic_conformance_candidate_001",
}
CURRENT_STATUS_PHRASES = [
    "official `1ZoneUncontrolled` dynamic source-order compatibility rows",
    "declared-variable compatibility only",
    "The current compatibility-mode arbitrary runtime covers the official",
    "`1ZoneUncontrolled` heat-balance path",
    "IdealLoads outdoor-air, economizer, heat-recovery",
    "remain outside arbitrary-run compatibility",
]
ALLOWED_MANIFEST_OUTPUT_LEVELS = {"baseline", "diagnostic", "conformance"}
COMPATIBILITY_ALGORITHM_STATUSES = {"conformance", "scaffold"}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate specs/capabilities.toml.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--self-test", action="store_true")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def load_case_manifests(repo_root: Path, errors: list[str]) -> dict[str, dict[str, Any]]:
    cases: dict[str, dict[str, Any]] = {}
    for path in sorted((repo_root / "data" / "conformance_cases").glob("*/case.toml")):
        case = load_toml(path)
        directory_id = path.parent.name
        raw_case_id = case.get("id")
        case_id = raw_case_id.strip() if isinstance(raw_case_id, str) else ""
        require(bool(case_id), errors, f"case manifest has empty id: {path}")
        require(
            case_id == directory_id,
            errors,
            f"case manifest id must match its directory: {case_id!r} != {directory_id!r}",
        )
        storage_id = case_id or directory_id
        require(storage_id not in cases, errors, f"duplicate case manifest id: {storage_id}")
        if storage_id not in cases:
            cases[storage_id] = case
    return cases


def values(item: dict[str, Any], key: str) -> list[str]:
    return [str(value) for value in item.get(key, [])]


def wildcard_match(pattern: str, value: str) -> bool:
    if pattern == "*":
        return True
    if "*" not in pattern:
        return pattern == value

    parts = [part for part in pattern.split("*") if part]
    if not parts:
        return True

    search_start = 0
    for index, part in enumerate(parts):
        position = value[search_start:].find(part)
        if position < 0:
            return False
        if index == 0 and not pattern.startswith("*") and position != 0:
            return False
        search_start += position + len(part)
    return pattern.endswith("*") or value.endswith(parts[-1])


def rule_matches(rule: dict[str, Any], object_type: str) -> bool:
    patterns = values(rule, "object_patterns")
    except_patterns = values(rule, "except_object_patterns")
    return any(wildcard_match(pattern, object_type) for pattern in patterns) and not any(
        wildcard_match(pattern, object_type) for pattern in except_patterns
    )


def capability_for_ideal_loads_case(case_id: str) -> str:
    if "outdoor_air" in case_id:
        return "ideal_loads_outdoor_air_selected_branches"
    if "constant_shr" in case_id:
        return "ideal_loads_constant_shr"
    if (
        "capacity_limit" in case_id
        or "flow_limit" in case_id
        or "flow_capacity_limit" in case_id
    ):
        return "ideal_loads_finite_limits"
    if "constant_supply_humidity" in case_id or "humidistat" in case_id:
        return "ideal_loads_humidity_selected_branches"
    return "ideal_loads_no_oa_sensible"


def conformance_claim_case_ids(cases: dict[str, dict[str, Any]], prefix: str) -> set[str]:
    return {
        case_id
        for case_id, case in cases.items()
        if case_id.startswith(prefix) and case.get("conformance_claim") is True
    }


def validate_capabilities(
    registry: dict[str, Any],
    run_states: dict[str, Any],
    cases: dict[str, dict[str, Any]],
    errors: list[str],
) -> None:
    capabilities = registry.get("capability", [])
    capability_ids = [str(item.get("id", "")) for item in capabilities]
    capability_by_id = {str(item.get("id", "")): item for item in capabilities}
    run_state_ids = {str(item.get("id", "")) for item in run_states.get("state", [])}

    require(len(capability_ids) == len(set(capability_ids)), errors, "capability ids must be unique")
    require(
        set(capability_ids) == EXPECTED_CAPABILITY_IDS,
        errors,
        f"capability ids changed without updating the B2 contract: {sorted(capability_ids)}",
    )

    for capability in capabilities:
        capability_id = str(capability.get("id", ""))
        for field in REQUIRED_CAPABILITY_FIELDS:
            require(
                bool(capability.get(field)),
                errors,
                f"capability {capability_id} must define non-empty {field}",
            )
        require(
            str(capability.get("run_state", "")) in run_state_ids,
            errors,
            f"capability {capability_id} references unknown run_state {capability.get('run_state')}",
        )
        require(
            str(capability.get("support_level", "")) == "compatibility",
            errors,
            f"capability {capability_id} must keep support_level=compatibility",
        )
        for case_id in values(capability, "evidence_cases"):
            case = cases.get(case_id)
            require(case is not None, errors, f"capability {capability_id} references missing evidence case {case_id}")
            if case is not None:
                require(
                    case.get("conformance_claim") is True,
                    errors,
                    f"capability {capability_id} evidence case {case_id} must be conformance_claim=true",
                )
                require(
                    case.get("comparison_class") == "conformance",
                    errors,
                    f"capability {capability_id} evidence case {case_id} must use comparison_class=conformance",
                )

    official_capability = capability_by_id.get("official_1zone_uncontrolled_declared_heat_balance")
    if official_capability is not None:
        official_cases = set(values(official_capability, "evidence_cases"))
        require(
            official_cases == EXPECTED_OFFICIAL_CASES,
            errors,
            "official 1Zone capability must match the declared current-status conformance candidate",
        )

    ideal_claims = conformance_claim_case_ids(cases, "ideal_loads_")
    expected_by_capability: dict[str, set[str]] = {
        capability_id: set() for capability_id in EXPECTED_CAPABILITY_IDS if capability_id.startswith("ideal_loads_")
    }
    for case_id in ideal_claims:
        expected_by_capability[capability_for_ideal_loads_case(case_id)].add(case_id)

    for capability_id, expected_cases in expected_by_capability.items():
        capability = capability_by_id.get(capability_id)
        if capability is None:
            continue
        actual_cases = set(values(capability, "evidence_cases"))
        require(
            actual_cases == expected_cases,
            errors,
            f"{capability_id} evidence_cases mismatch: expected {sorted(expected_cases)}, found {sorted(actual_cases)}",
        )


def validate_spec_cross_references(
    registry: dict[str, Any],
    ledger: dict[str, Any],
    variable_coverage: dict[str, Any],
    cases: dict[str, dict[str, Any]],
    errors: list[str],
) -> None:
    algorithms = ledger.get("algorithm", [])
    algorithm_ids = [str(item.get("id", "")).strip() for item in algorithms]
    algorithm_by_id = {
        str(item.get("id", "")).strip(): item
        for item in algorithms
        if str(item.get("id", "")).strip()
    }
    require(all(algorithm_ids), errors, "algorithm ledger ids must be non-empty")
    require(len(algorithm_ids) == len(set(algorithm_ids)), errors, "algorithm ledger ids must be unique")

    variables = variable_coverage.get("variable", [])
    variable_names = [str(item.get("name", "")).strip() for item in variables]
    variable_by_name = {
        str(item.get("name", "")).strip(): item
        for item in variables
        if str(item.get("name", "")).strip()
    }
    require(all(variable_names), errors, "variable coverage names must be non-empty")
    require(len(variable_names) == len(set(variable_names)), errors, "variable coverage names must be unique")

    for capability in registry.get("capability", []):
        capability_id = str(capability.get("id", "")).strip() or "<missing-id>"
        algorithm_refs = values(capability, "algorithms")
        evidence_refs = values(capability, "evidence_cases")
        require(
            len(algorithm_refs) == len(set(algorithm_refs)),
            errors,
            f"capability {capability_id} algorithm references must be unique",
        )
        require(
            len(evidence_refs) == len(set(evidence_refs)),
            errors,
            f"capability {capability_id} evidence case references must be unique",
        )

        for algorithm_id in algorithm_refs:
            algorithm = algorithm_by_id.get(algorithm_id)
            require(
                algorithm is not None,
                errors,
                f"capability {capability_id} references unknown algorithm ledger id {algorithm_id}",
            )
            if algorithm is not None and capability.get("support_level") == "compatibility":
                status = str(algorithm.get("status", ""))
                require(
                    status in COMPATIBILITY_ALGORITHM_STATUSES,
                    errors,
                    f"compatibility capability {capability_id} references algorithm {algorithm_id} with status {status!r}",
                )

        for case_id in evidence_refs:
            case = cases.get(case_id)
            require(
                case is not None,
                errors,
                f"capability {capability_id} references missing evidence case {case_id}",
            )
            if case is not None:
                require(
                    case.get("conformance_claim") is True,
                    errors,
                    f"capability {capability_id} evidence case {case_id} must be conformance_claim=true",
                )
                require(
                    case.get("comparison_class") == "conformance",
                    errors,
                    f"capability {capability_id} evidence case {case_id} must use comparison_class=conformance",
                )

    for consumed_object in registry.get("consumed_object", []):
        consumed_id = str(consumed_object.get("id", "")).strip() or "<missing-id>"
        algorithm_refs = values(consumed_object, "algorithms")
        require(
            len(algorithm_refs) == len(set(algorithm_refs)),
            errors,
            f"consumed_object {consumed_id} algorithm references must be unique",
        )
        for algorithm_id in algorithm_refs:
            require(
                algorithm_id in algorithm_by_id,
                errors,
                f"consumed_object {consumed_id} references unknown algorithm ledger id {algorithm_id}",
            )

    for algorithm in algorithms:
        algorithm_id = str(algorithm.get("id", "")).strip() or "<missing-id>"
        status = str(algorithm.get("status", "")).strip()
        first_case_id = str(algorithm.get("first_case", "")).strip()
        first_evidence_id = str(algorithm.get("first_evidence", "")).strip()
        require(
            bool(first_case_id),
            errors,
            f"algorithm {algorithm_id} first_case must be non-empty",
        )
        first_case = cases.get(first_case_id)
        require(
            first_case is not None,
            errors,
            f"algorithm {algorithm_id} first_case missing from case manifests: {first_case_id}",
        )
        if status == "scaffold":
            require(
                not first_evidence_id,
                errors,
                f"scaffold algorithm {algorithm_id} must not claim first_evidence",
            )
        else:
            require(
                bool(first_evidence_id),
                errors,
                f"algorithm {algorithm_id} first_evidence must be non-empty",
            )
        if first_evidence_id:
            require(
                first_evidence_id in cases,
                errors,
                f"algorithm {algorithm_id} first_evidence missing from case manifests: {first_evidence_id}",
            )

        proof_variables = values(algorithm, "proof_variables")
        require(
            len(proof_variables) == len(set(proof_variables)),
            errors,
            f"algorithm {algorithm_id} proof variable references must be unique",
        )
        for variable_name in proof_variables:
            variable = variable_by_name.get(variable_name)
            require(
                variable is not None,
                errors,
                f"algorithm {algorithm_id} proof variable missing from variable coverage: {variable_name}",
            )
            if variable is not None and status == "conformance":
                require(
                    variable.get("status") == "conformance",
                    errors,
                    f"conformance algorithm {algorithm_id} proof variable must have conformance coverage: {variable_name}",
                )
            if first_case is not None and status == "conformance":
                first_case_levels = {
                    str(output.get("level", "")).strip()
                    for output in first_case.get("outputs", [])
                    if str(output.get("variable", "")).strip() == variable_name
                }
                require(
                    bool(first_case_levels),
                    errors,
                    f"conformance algorithm {algorithm_id} proof variable is not requested by first_case {first_case_id}: {variable_name}",
                )

    manifest_output_cases: dict[str, set[str]] = {}
    conformance_output_cases: dict[str, set[str]] = {}
    for case_id, case in cases.items():
        require(
            str(case.get("id", "")).strip() == case_id,
            errors,
            f"case manifest map key/id mismatch: {case_id!r} != {case.get('id')!r}",
        )
        for index, output in enumerate(case.get("outputs", [])):
            variable_name = str(output.get("variable", "")).strip()
            level = str(output.get("level", "")).strip()
            prefix = f"{case_id} outputs[{index}]"
            require(bool(variable_name), errors, f"{prefix} must name a variable")
            require(
                level in ALLOWED_MANIFEST_OUTPUT_LEVELS,
                errors,
                f"{prefix} has unsupported evidence level {level!r}",
            )
            if not variable_name:
                continue
            manifest_output_cases.setdefault(variable_name, set()).add(case_id)
            if level == "conformance":
                conformance_output_cases.setdefault(variable_name, set()).add(case_id)

    for variable_name, case_ids in sorted(manifest_output_cases.items()):
        variable = variable_by_name.get(variable_name)
        require(
            variable is not None,
            errors,
            f"manifest output variable missing from variable coverage: {variable_name} ({', '.join(sorted(case_ids))})",
        )
    for variable_name in sorted(variable_by_name):
        require(
            variable_name in manifest_output_cases,
            errors,
            f"variable coverage entry is not requested by any case manifest: {variable_name}",
        )
    for variable_name, case_ids in sorted(conformance_output_cases.items()):
        variable = variable_by_name.get(variable_name)
        if variable is not None:
            require(
                variable.get("status") == "conformance",
                errors,
                f"conformance manifest output must have conformance coverage: {variable_name} ({', '.join(sorted(case_ids))})",
            )
    for variable_name, variable in sorted(variable_by_name.items()):
        first_case_id = str(variable.get("first_case", "")).strip()
        first_evidence_id = str(variable.get("first_evidence", "")).strip()
        require(
            bool(first_case_id),
            errors,
            f"variable coverage {variable_name} first_case must be non-empty",
        )
        require(
            bool(first_evidence_id),
            errors,
            f"variable coverage {variable_name} first_evidence must be non-empty",
        )
        first_case = cases.get(first_case_id)
        require(
            first_case is not None,
            errors,
            f"variable coverage {variable_name} first_case missing from case manifests: {first_case_id}",
        )
        require(
            first_evidence_id in cases,
            errors,
            f"variable coverage {variable_name} first_evidence missing from case manifests: {first_evidence_id}",
        )
        if first_case is not None:
            first_case_levels = {
                str(output.get("level", "")).strip()
                for output in first_case.get("outputs", [])
                if str(output.get("variable", "")).strip() == variable_name
            }
            require(
                bool(first_case_levels),
                errors,
                f"variable coverage {variable_name} is not requested by first_case {first_case_id}",
            )
            if variable.get("status") == "conformance":
                require(
                    "conformance" in first_case_levels,
                    errors,
                    f"conformance variable coverage {variable_name} is not a conformance output of first_case {first_case_id}",
                )
        if variable.get("status") == "conformance":
            require(
                variable_name in conformance_output_cases,
                errors,
                f"conformance variable coverage has no conformance-level manifest output: {variable_name}",
            )


def cross_spec_self_test_fixture() -> tuple[
    dict[str, Any],
    dict[str, Any],
    dict[str, Any],
    dict[str, dict[str, Any]],
]:
    registry = {
        "capability": [
            {
                "id": "fixture_capability",
                "support_level": "compatibility",
                "algorithms": ["fixture_algorithm"],
                "evidence_cases": ["fixture_case"],
            }
        ],
        "consumed_object": [
            {
                "id": "fixture_consumed_object",
                "object_type": "Fixture:Consumed",
                "algorithms": ["fixture_algorithm"],
                "reason": "Fixture object is consumed by the fixture algorithm.",
            }
        ],
    }
    ledger = {
        "algorithm": [
            {
                "id": "fixture_algorithm",
                "status": "conformance",
                "first_case": "fixture_case",
                "first_evidence": "fixture_case",
                "proof_variables": ["Fixture Output"],
            }
        ]
    }
    variable_coverage = {
        "variable": [
            {
                "name": "Fixture Output",
                "status": "conformance",
                "first_case": "fixture_case",
                "first_evidence": "fixture_case",
            }
        ]
    }
    cases = {
        "fixture_case": {
            "id": "fixture_case",
            "comparison_class": "conformance",
            "conformance_claim": True,
            "outputs": [
                {
                    "variable": "Fixture Output",
                    "level": "conformance",
                }
            ],
        }
    }
    return registry, ledger, variable_coverage, cases


def validate_cross_spec_self_test(errors: list[str]) -> None:
    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    fixture_errors: list[str] = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    validate_rules(registry, fixture_errors)
    require(not fixture_errors, errors, f"cross-spec valid-fixture self-test failed: {fixture_errors}")

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    registry["capability"][0]["algorithms"] = ["missing_algorithm"]
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("references unknown algorithm ledger id" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject an unknown capability algorithm",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    registry["consumed_object"][0]["algorithms"] = ["missing_algorithm"]
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any(
            "consumed_object fixture_consumed_object references unknown algorithm" in error
            for error in fixture_errors
        ),
        errors,
        "cross-spec self-test did not reject an unknown consumed-object algorithm",
    )

    registry, _, _, _ = cross_spec_self_test_fixture()
    del registry["consumed_object"][0]["reason"]
    fixture_errors = []
    validate_rules(registry, fixture_errors)
    require(
        any(
            "consumed_object fixture_consumed_object must define non-empty reason" in error
            for error in fixture_errors
        ),
        errors,
        "cross-spec self-test did not reject incomplete consumed-object metadata",
    )

    registry, _, _, _ = cross_spec_self_test_fixture()
    registry["partial_rule"] = [
        {
            "id": "fixture_partial",
            "object_patterns": ["Fixture:Consumed"],
            "eligible_state": "partial_supported_run",
            "reason": "inactive fixture",
        }
    ]
    fixture_errors = []
    validate_rules(registry, fixture_errors)
    require(
        any("must not remain in partial_rule" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject consumed-object partial-rule overlap",
    )

    registry, _, _, _ = cross_spec_self_test_fixture()
    registry["arbitrary_run"] = {
        "ignored_raw_only_objects": {"objects": ["Fixture:Consumed"]}
    }
    fixture_errors = []
    validate_rules(registry, fixture_errors)
    require(
        any(
            "must not remain in arbitrary_run.ignored_raw_only_objects" in error
            for error in fixture_errors
        ),
        errors,
        "cross-spec self-test did not reject consumed-object ignored-list overlap",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    registry["capability"][0]["evidence_cases"] = ["missing_case"]
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("references missing evidence case" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject a missing capability evidence case",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    variable_coverage["variable"] = []
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("manifest output variable missing from variable coverage" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject an uncovered manifest output",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    ledger["algorithm"][0]["proof_variables"] = ["Missing Proof Output"]
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("proof variable missing from variable coverage" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject an uncovered algorithm proof variable",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    ledger["algorithm"][0]["first_case"] = "missing_case"
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("algorithm fixture_algorithm first_case missing" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject a missing algorithm first_case",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    ledger["algorithm"][0]["first_evidence"] = "missing_case"
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("algorithm fixture_algorithm first_evidence missing" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject a missing algorithm first_evidence",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    variable_coverage["variable"][0]["first_case"] = "missing_case"
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("variable coverage Fixture Output first_case missing" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject a missing variable first_case",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    variable_coverage["variable"][0]["first_evidence"] = "missing_case"
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("variable coverage Fixture Output first_evidence missing" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject a missing variable first_evidence",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    variable_coverage["variable"][0]["status"] = "diagnostic"
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any(
            "conformance algorithm fixture_algorithm proof variable must have conformance coverage"
            in error
            for error in fixture_errors
        ),
        errors,
        "cross-spec self-test did not reject conformance coverage status drift",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    variable_coverage["variable"].append(
        {
            "name": "Orphan Fixture Output",
            "status": "diagnostic",
            "first_case": "fixture_case",
            "first_evidence": "fixture_case",
        }
    )
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("variable coverage entry is not requested" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject an orphan variable coverage entry",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    cases["fixture_case"]["id"] = "different_case"
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("case manifest map key/id mismatch" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject a mismatched manifest id",
    )

    registry, ledger, variable_coverage, cases = cross_spec_self_test_fixture()
    del cases["fixture_case"]["id"]
    fixture_errors = []
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, fixture_errors)
    require(
        any("case manifest map key/id mismatch" in error for error in fixture_errors),
        errors,
        "cross-spec self-test did not reject a missing manifest id",
    )


def validate_rules(registry: dict[str, Any], errors: list[str]) -> None:
    capabilities = registry.get("capability", [])
    unsupported_rules = registry.get("unsupported_rule", [])
    partial_rules = registry.get("partial_rule", [])
    consumed_objects = registry.get("consumed_object", [])

    consumed_ids = [str(item.get("id", "")).strip() for item in consumed_objects]
    consumed_object_types = [
        str(item.get("object_type", "")).strip() for item in consumed_objects
    ]
    require(all(consumed_ids), errors, "consumed_object ids must be non-empty")
    require(
        len(consumed_ids) == len(set(consumed_ids)),
        errors,
        "consumed_object ids must be unique",
    )
    require(
        all(consumed_object_types),
        errors,
        "consumed_object object_type values must be non-empty",
    )
    require(
        len(consumed_object_types) == len(set(consumed_object_types)),
        errors,
        "consumed_object object_type values must be unique",
    )

    partial_patterns = {
        pattern for rule in partial_rules for pattern in values(rule, "object_patterns")
    }
    ignored_raw_only_objects = set(
        values(
            registry.get("arbitrary_run", {}).get("ignored_raw_only_objects", {}),
            "objects",
        )
    )
    for consumed_object in consumed_objects:
        consumed_id = str(consumed_object.get("id", "")).strip() or "<missing-id>"
        object_type = str(consumed_object.get("object_type", "")).strip()
        for field in ("id", "object_type", "algorithms", "reason"):
            require(
                bool(consumed_object.get(field)),
                errors,
                f"consumed_object {consumed_id} must define non-empty {field}",
            )
        require(
            object_type not in partial_patterns,
            errors,
            f"consumed_object {consumed_id} object_type {object_type!r} must not remain in partial_rule object_patterns",
        )
        require(
            object_type not in ignored_raw_only_objects,
            errors,
            f"consumed_object {consumed_id} object_type {object_type!r} must not remain in arbitrary_run.ignored_raw_only_objects",
        )

    required_objects = sorted(
        {
            object_type
            for capability in capabilities
            for object_type in values(capability, "required_objects")
        }
    )
    for rule in unsupported_rules:
        rule_id = str(rule.get("id", ""))
        require(bool(rule.get("reason")), errors, f"unsupported_rule {rule_id} must explain its boundary")
        for pattern in values(rule, "object_patterns"):
            require(pattern != "*", errors, f"unsupported_rule {rule_id} must not use catch-all pattern '*'")
        conflicts = [
            object_type for object_type in required_objects if rule_matches(rule, object_type)
        ]
        require(
            not conflicts,
            errors,
            f"unsupported_rule {rule_id} conflicts with required capability objects: {conflicts}",
        )

    for rule in partial_rules:
        rule_id = str(rule.get("id", ""))
        patterns = values(rule, "object_patterns")
        reason = str(rule.get("reason", ""))
        require(
            str(rule.get("eligible_state", "")) == "partial_supported_run",
            errors,
            f"partial_rule {rule_id} must force eligible_state=partial_supported_run",
        )
        require(
            not any(pattern in {"ZoneHVAC:*", "PlantLoop", "AirLoopHVAC*"} for pattern in patterns),
            errors,
            f"partial_rule {rule_id} is too broad for semantic runtime objects",
        )
        require(
            "without changing simulation semantics" in reason or "inactive" in reason,
            errors,
            f"partial_rule {rule_id} must explain why no active semantics change",
        )


def validate_rust_wiring(repo_root: Path, errors: list[str]) -> None:
    support_registry = read_text(repo_root / "crates" / "ep_run" / "src" / "support_registry.rs")
    support = read_text(repo_root / "crates" / "ep_run" / "src" / "support.rs")
    runtime_boundaries = read_text(
        repo_root / "crates" / "ep_run" / "src" / "support" / "runtime_boundaries.rs"
    )
    pipeline = read_text(repo_root / "crates" / "ep_run" / "src" / "pipeline.rs")

    require(
        'include_str!("../../../specs/capabilities.toml")' in support_registry,
        errors,
        "support registry must embed specs/capabilities.toml",
    )
    require(
        "load_embedded_capability_registry()" in support,
        errors,
        "SupportAssessment must load the embedded capability registry",
    )
    for capability_id in EXPECTED_RUNTIME_SELECTED_CAPABILITY_IDS:
        require(
            capability_id in runtime_boundaries,
            errors,
            f"runtime boundary code must reference capability id {capability_id}",
        )
    for token in [
        "matched_capability_ids",
        "matched_capabilities",
        "failed_capability_ids",
        "capability_registry_loaded",
    ]:
        require(token in support, errors, f"SupportAssessment must expose {token}")
        require(token in pipeline, errors, f"run-summary support block must expose {token}")
    require(
        "CapabilityRegistryCapabilityMissing" in support,
        errors,
        "missing registry capabilities must produce a support diagnostic",
    )


def validate_current_status(repo_root: Path, errors: list[str]) -> None:
    text = " ".join(read_text(repo_root / "docs" / "src" / "current" / "current-status.md").split())
    for phrase in CURRENT_STATUS_PHRASES:
        require(
            " ".join(phrase.split()) in text,
            errors,
            f"current-status.md missing capability boundary phrase: {phrase}",
        )


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    registry = load_toml(repo_root / "specs" / "capabilities.toml")
    ledger = load_toml(repo_root / "specs" / "algorithm_ledger.toml")
    variable_coverage = load_toml(repo_root / "specs" / "variable_coverage.toml")
    run_states = load_toml(repo_root / "specs" / "run_result_states.toml")
    errors: list[str] = []
    cases = load_case_manifests(repo_root, errors)

    validate_capabilities(registry, run_states, cases, errors)
    validate_spec_cross_references(registry, ledger, variable_coverage, cases, errors)
    if args.self_test:
        validate_cross_spec_self_test(errors)
    validate_rules(registry, errors)
    validate_rust_wiring(repo_root, errors)
    validate_current_status(repo_root, errors)

    if errors:
        print("Capability registry validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    capability_ids = sorted(str(item.get("id", "")) for item in registry.get("capability", []))
    algorithm_ref_count = sum(
        len(values(capability, "algorithms"))
        for capability in registry.get("capability", [])
    ) + sum(
        len(values(consumed_object, "algorithms"))
        for consumed_object in registry.get("consumed_object", [])
    )
    evidence_ref_count = sum(
        len(values(capability, "evidence_cases"))
        for capability in registry.get("capability", [])
    )
    output_variable_count = len(
        {
            str(output.get("variable", "")).strip()
            for case in cases.values()
            for output in case.get("outputs", [])
            if str(output.get("variable", "")).strip()
        }
    )
    print("Capability registry and spec cross-check")
    print(f"  capability_ids: {', '.join(capability_ids)}")
    print(f"  unsupported_rules: {len(registry.get('unsupported_rule', []))}")
    print(f"  partial_rules: {len(registry.get('partial_rule', []))}")
    print(f"  consumed_objects: {len(registry.get('consumed_object', []))}")
    print(f"  algorithm_ledger_references: {algorithm_ref_count}")
    print(f"  evidence_case_references: {evidence_ref_count}")
    print(f"  covered_manifest_output_variables: {output_variable_count}")
    print("  rust_wiring: valid")
    print("  conformance_case_mapping: valid")
    print(f"  mutation_self_test: {'pass' if args.self_test else 'not requested'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
