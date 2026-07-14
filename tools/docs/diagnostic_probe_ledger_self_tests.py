#!/usr/bin/env python3
"""Mutation self-tests for the diagnostic-probe lifecycle validator."""

from __future__ import annotations

import copy
from pathlib import Path
from typing import Any

from validate_diagnostic_probe_ledger import load_toml, validate_probe_ledger


def run_self_tests(repo_root: Path) -> int:
    baseline = load_toml(repo_root / "specs" / "diagnostic_probe_ledger.toml")
    baseline_errors = validate_probe_ledger(repo_root, baseline)
    if baseline_errors:
        raise AssertionError(f"baseline diagnostic probe ledger is invalid: {baseline_errors}")

    passed: list[str] = []

    def expect_error(name: str, candidate: dict[str, Any], token: str) -> None:
        errors = validate_probe_ledger(repo_root, candidate)
        if not any(token in error for error in errors):
            raise AssertionError(f"{name}: expected error containing {token!r}; got {errors}")
        passed.append(name)

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["hypothesis_id"] = "missing-hypothesis"
    expect_error("orphan_active_hypothesis", candidate, "orphan active hypothesis mapping")

    candidate = copy.deepcopy(baseline)
    duplicate = copy.deepcopy(candidate["active_probe"][0])
    duplicate["id"] = "duplicate-hypothesis-lane"
    duplicate["command"] = "duplicate-hypothesis-lane"
    duplicate["script"] = "scripts/compare/duplicate-hypothesis-lane.ps1"
    duplicate["suite_lane"] = "duplicate-hypothesis-lane.ps1"
    candidate["active_probe"].append(duplicate)
    expect_error("duplicate_hypothesis_mapping", candidate, "duplicate active hypothesis mapping")

    for field, token in [
        ("id", "duplicate active probe id"),
        ("command", "duplicate active probe command"),
        ("script", "duplicate active probe script"),
        ("suite_lane", "duplicate active probe suite_lane"),
    ]:
        candidate = copy.deepcopy(baseline)
        duplicate = copy.deepcopy(candidate["active_probe"][0])
        duplicate["hypothesis_id"] = "missing-hypothesis"
        for unique_field in ["id", "command", "script", "suite_lane"]:
            if unique_field != field:
                duplicate[unique_field] = f"unique-{unique_field}"
        candidate["active_probe"].append(duplicate)
        expect_error(f"duplicate_active_{field}", candidate, token)

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"] = []
    expect_error("unresolved_hypothesis_missing_active_lane", candidate, "missing active")

    for status in ["closed", "resolved"]:
        candidate = copy.deepcopy(baseline)
        candidate["hypothesis"][0]["status"] = status
        candidate["hypothesis"][0]["resolution"] = "Mutation-test result."
        candidate["hypothesis"][0]["evidence_ref"] = "mutation-test-evidence"
        expect_error(f"{status}_hypothesis_cannot_remain_active", candidate, f"active probe references {status} hypothesis")

    candidate = copy.deepcopy(baseline)
    candidate["closed_selector"].pop()
    expect_error("missing_rust_selector_classification", candidate, "Rust diagnostic selector classification mismatch")

    candidate = copy.deepcopy(baseline)
    candidate["closed_script"].pop()
    expect_error("missing_probe_script_classification", candidate, "probe wrapper classification mismatch")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["suite_lane"] = "missing-active-suite-lane.ps1"
    expect_error("missing_active_suite_lane", candidate, "active suite lane mismatch")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["command"] = "missing-active-probe-command"
    expect_error("missing_active_command_mapping", candidate, "active probe command is not in commands.json")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["source_state"] = "DifferentScalarState"
    expect_error("active_source_state_mismatch", candidate, "source_state must match hypothesis")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["selector_kind"] = "typo"
    expect_error("active_selector_kind_must_be_locked", candidate, "selector_kind must be compatibility-observation")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["selector"] = "DoesNotExist"
    candidate["active_probe"][0]["cli_name"] = "does-not-exist"
    expect_error("active_selector_cli_pair_must_exist", candidate, "exact compatibility selector/CLI pair not found")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["ctf_seed_policy"] = "drifted-policy"
    expect_error("active_wrapper_ctf_policy_must_match", candidate, "wrapper diagnosticArgs CtfSeedPolicy does not match ledger")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["warmup_minimum_days"] = 0
    expect_error("active_warmup_days_must_be_positive", candidate, "warmup_minimum_days must be a positive integer")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["observation_only"] = False
    expect_error("active_wrapper_must_be_observation_only", candidate, "observation_only must be true")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["observation_key"] = "DRIFTED:KEY"
    expect_error("active_observation_key_must_match", candidate, "wrapper observation key does not match ledger")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["minimum_abs_delta_c"] = 2.0e-9
    expect_error("active_observation_delta_must_match", candidate, "wrapper minimum delta does not match ledger")

    candidate = copy.deepcopy(baseline)
    candidate["hypothesis"][0]["source_state"] = "MissingEnergyPlusState"
    candidate["active_probe"][0]["source_state"] = "MissingEnergyPlusState"
    expect_error("missing_source_state_in_reference_routine", candidate, "source_state not found in source routine")

    candidate = copy.deepcopy(baseline)
    candidate["hypothesis"][0]["owner_routine_id"] = "missing_owner_routine"
    expect_error("missing_algorithm_owner_routine", candidate, "owner_routine_id is not owned by algorithm")

    candidate = copy.deepcopy(baseline)
    candidate["hypothesis"][0]["source_map_anchor"] = "diagnostic-probe-hypothesis:missing"
    expect_error("missing_source_map_anchor", candidate, "source-map anchor not found")

    candidate = copy.deepcopy(baseline)
    candidate["hypothesis"] = []
    candidate["active_probe"] = []
    expect_error("source_map_hypothesis_cannot_be_unledgered", candidate, "source-map hypothesis anchor set mismatch")

    candidate = copy.deepcopy(baseline)
    candidate["closed_selector"][0]["evidence_ref"] = "docs/src/porting-map/heat-balance-source-map.md#missing-evidence"
    expect_error("closed_selector_evidence_cannot_drift", candidate, "evidence_ref must match historical_result_evidence")

    candidate = copy.deepcopy(baseline)
    candidate["closed_script"][0]["evidence_ref"] = "docs/src/porting-map/heat-balance-source-map.md#missing-evidence"
    expect_error("closed_script_evidence_cannot_drift", candidate, "evidence_ref must match historical_result_evidence")

    candidate = copy.deepcopy(baseline)
    candidate["historical_result_evidence"] = "docs/src/porting-map/heat-balance-source-map.md#missing-evidence"
    expect_error("historical_evidence_anchor_must_exist", candidate, "historical_result_evidence anchor not found")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["report_path"] = "../outside/report.json"
    expect_error("active_report_path_must_be_safe", candidate, "report_path must be a safe repository-relative .json path")

    candidate = copy.deepcopy(baseline)
    candidate["active_probe"][0]["report_path"] = ".runtime/drifted/26.1.0/case/compare/compare-digest.json"
    expect_error("active_report_location_must_match_wrapper", candidate, "wrapper does not contain declared report output root")

    print("Diagnostic probe ledger self-test")
    print(f"  mutations: {len(passed)}")
    for name in passed:
        print(f"  OK {name}")
    return 0


if __name__ == "__main__":
    raise SystemExit(run_self_tests(Path.cwd()))
