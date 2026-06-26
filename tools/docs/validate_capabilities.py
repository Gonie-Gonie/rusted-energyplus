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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate specs/capabilities.toml.")
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


def conformance_claim_case_ids(repo_root: Path, prefix: str) -> set[str]:
    case_ids: set[str] = set()
    for path in (repo_root / "data" / "conformance_cases").glob(f"{prefix}*/case.toml"):
        data = load_toml(path)
        if data.get("conformance_claim") is True:
            case_ids.add(str(data.get("id", path.parent.name)))
    return case_ids


def validate_capabilities(
    repo_root: Path,
    registry: dict[str, Any],
    run_states: dict[str, Any],
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
            case_path = repo_root / "data" / "conformance_cases" / case_id / "case.toml"
            require(case_path.is_file(), errors, f"capability {capability_id} references missing evidence case {case_id}")
            if case_path.is_file():
                case = load_toml(case_path)
                require(
                    case.get("conformance_claim") is True,
                    errors,
                    f"capability {capability_id} evidence case {case_id} must be conformance_claim=true",
                )

    official_cases = set(values(capability_by_id["official_1zone_uncontrolled_declared_heat_balance"], "evidence_cases"))
    require(
        official_cases == EXPECTED_OFFICIAL_CASES,
        errors,
        "official 1Zone capability must match the declared current-status conformance candidate",
    )

    ideal_claims = conformance_claim_case_ids(repo_root, "ideal_loads_")
    expected_by_capability: dict[str, set[str]] = {
        capability_id: set() for capability_id in EXPECTED_CAPABILITY_IDS if capability_id.startswith("ideal_loads_")
    }
    for case_id in ideal_claims:
        expected_by_capability[capability_for_ideal_loads_case(case_id)].add(case_id)

    for capability_id, expected_cases in expected_by_capability.items():
        actual_cases = set(values(capability_by_id[capability_id], "evidence_cases"))
        require(
            actual_cases == expected_cases,
            errors,
            f"{capability_id} evidence_cases mismatch: expected {sorted(expected_cases)}, found {sorted(actual_cases)}",
        )


def validate_rules(registry: dict[str, Any], errors: list[str]) -> None:
    capabilities = registry.get("capability", [])
    unsupported_rules = registry.get("unsupported_rule", [])
    partial_rules = registry.get("partial_rule", [])

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
    run_states = load_toml(repo_root / "specs" / "run_result_states.toml")
    errors: list[str] = []

    validate_capabilities(repo_root, registry, run_states, errors)
    validate_rules(registry, errors)
    validate_rust_wiring(repo_root, errors)
    validate_current_status(repo_root, errors)

    if errors:
        print("Capability registry validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    capability_ids = sorted(str(item.get("id", "")) for item in registry.get("capability", []))
    print("Capability registry check")
    print(f"  capability_ids: {', '.join(capability_ids)}")
    print(f"  unsupported_rules: {len(registry.get('unsupported_rule', []))}")
    print(f"  partial_rules: {len(registry.get('partial_rule', []))}")
    print("  rust_wiring: valid")
    print("  conformance_case_mapping: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
