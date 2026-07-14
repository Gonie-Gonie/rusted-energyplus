#!/usr/bin/env python3
"""Mutation self-tests for routine completion and full-domain claim gates."""

from __future__ import annotations

import copy
from pathlib import Path
from typing import Any
from unittest.mock import patch

import validate_algorithm_ledger as ledger_validator

from validate_algorithm_ledger import (
    collect_routines,
    command_names,
    load_toml,
    validate_domain_completion_contract,
    validate_routine,
    variable_names,
)


def find_raw_routine(spec: dict[str, Any], routine_id: str) -> dict[str, Any]:
    for algorithm in spec.get("algorithm", []):
        routine = algorithm.get("routine", {}).get(routine_id)
        if isinstance(routine, dict):
            return routine
    raise KeyError(routine_id)


def run_completion_self_tests(repo_root: Path) -> int:
    reference_root = repo_root / ".reference" / "energyplus-src" / "26.1.0"
    ledger = load_toml(repo_root / "specs" / "algorithm_ledger.toml")
    contract = load_toml(repo_root / "specs" / "project_contract.toml")
    covered_variables = variable_names(repo_root)
    commands = command_names(repo_root)
    passed: list[str] = []

    def routine_errors(candidate: dict[str, Any]) -> list[str]:
        errors: list[str] = []
        routines = collect_routines(candidate.get("algorithm", []), errors)
        for routine in routines:
            validate_routine(repo_root, reference_root, routine, covered_variables, commands, errors)
        return errors

    def domain_errors(candidate_ledger: dict[str, Any], candidate_contract: dict[str, Any]) -> list[str]:
        errors: list[str] = []
        routines = collect_routines(candidate_ledger.get("algorithm", []), errors)
        validate_domain_completion_contract(candidate_contract, routines, errors)
        return errors

    def expect_error(name: str, errors: list[str], token: str) -> None:
        if not any(token in error for error in errors):
            raise AssertionError(f"{name}: expected error containing {token!r}; got {errors}")
        passed.append(name)

    def fill_state_mapping(routine: dict[str, Any]) -> None:
        routine_ids = {
            "ManageHeatBalance": "manage_heat_balance",
            "SimPurchasedAir": "sim_purchased_air",
            "ManagePlantLoops": "manage_plant_loops",
        }
        routine_id = routine_ids[routine["source_routine"]]
        routine["state_mapping_ref"] = "tools/docs/testdata/routine-state-map-v1.md"
        routine["read_state"] = [f"test.{routine_id}.read"]
        routine["write_state"] = [f"test.{routine_id}.write"]
        routine["history_state_ownership"] = f"test.{routine_id}.history"
        routine["unsupported_state"] = []
        routine["inactive_branches"] = []
        routine["unsupported_active_branches"] = []
        routine["not_claimed_branches"] = []

    baseline_routine_errors = routine_errors(ledger)
    if baseline_routine_errors:
        raise AssertionError(f"baseline routine contract is invalid: {baseline_routine_errors}")
    baseline_domain_errors = domain_errors(ledger, contract)
    if baseline_domain_errors:
        raise AssertionError(f"baseline domain completion contract is invalid: {baseline_domain_errors}")

    candidate = copy.deepcopy(ledger)
    find_raw_routine(candidate, "manage_heat_balance")["completion_status"] = "ported"
    expect_error("unknown_completion_status", routine_errors(candidate), "unsupported routine completion_status")

    candidate = copy.deepcopy(ledger)
    source = find_raw_routine(candidate, "manage_heat_balance")
    duplicate = find_raw_routine(candidate, "manage_surface_heat_balance")
    duplicate["source_file"] = source["source_file"]
    duplicate["source_routine"] = source["source_routine"]
    expect_error("duplicate_source_routine", routine_errors(candidate), "duplicate routine source mapping")

    candidate = copy.deepcopy(ledger)
    find_raw_routine(candidate, "manage_heat_balance").pop("source_map")
    expect_error("source_mapped_requires_source_map", routine_errors(candidate), "source_map must not be empty")

    candidate = copy.deepcopy(ledger)
    find_raw_routine(candidate, "manage_heat_balance")["source_file"] = "../HeatBalanceManager.cc"
    expect_error("source_path_must_stay_in_reference_tree", routine_errors(candidate), "safe path below src/EnergyPlus")

    candidate = copy.deepcopy(ledger)
    find_raw_routine(candidate, "manage_plant_loops")["source_routine"] = "state"
    expect_error(
        "common_identifier_is_not_source_routine",
        routine_errors(candidate),
        "C++ routine definition not found",
    )

    candidate = copy.deepcopy(ledger)
    find_raw_routine(candidate, "manage_heat_balance")["completion_status"] = "state_mapped"
    expect_error("state_mapped_requires_state_contract", routine_errors(candidate), "state_mapping_ref must not be empty")

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "state_mapped"
    routine["state_mapping_ref"] = routine["source_map"]
    routine["read_state"] = [routine["source_routine"]]
    routine["write_state"] = [routine["source_routine"]]
    routine["history_state_ownership"] = routine["source_routine"]
    routine["unsupported_state"] = []
    routine["inactive_branches"] = []
    routine["unsupported_active_branches"] = []
    routine["not_claimed_branches"] = []
    expect_error(
        "state_mapping_rejects_source_routine_placeholder",
        routine_errors(candidate),
        "must not reuse source_routine as placeholder state",
    )

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "state_mapped"
    fill_state_mapping(routine)
    routine["state_mapping_ref"] = routine["source_map"]
    expect_error(
        "state_mapping_requires_structured_routine_markers",
        routine_errors(candidate),
        "must contain matching routine-state-contract:v1 markers",
    )

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "state_mapped"
    fill_state_mapping(routine)
    routine["read_state"] = ["undocumented state token"]
    expect_error(
        "state_mapping_items_must_be_documented",
        routine_errors(candidate),
        "read_state item missing from state_mapping_ref",
    )

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "implemented"
    fill_state_mapping(routine)
    expect_error("implemented_requires_rust_target", routine_errors(candidate), "rust_target must be a non-empty array")

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "implemented"
    fill_state_mapping(routine)
    routine["rust_target"] = ["crates/ep_runtime/src/heat_balance/manager.rs"]
    expect_error("implemented_requires_rust_symbol", routine_errors(candidate), "must include a symbol anchor")

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "implemented"
    fill_state_mapping(routine)
    routine["rust_target"] = ["crates/ep_runtime/src/heat_balance/manager.rs::pub"]
    expect_error(
        "common_token_is_not_rust_implementation",
        routine_errors(candidate),
        "Rust implementation declaration not found",
    )

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "implemented"
    fill_state_mapping(routine)
    routine["rust_target"] = [
        "crates/ep_runtime/src/heat_balance/air_manager.rs::manage_air_heat_balance_stage"
    ]
    expect_error(
        "implemented_target_must_belong_to_parent_algorithm",
        routine_errors(candidate),
        "must be an exact parent algorithm rust_target",
    )

    candidate = copy.deepcopy(ledger)
    for algorithm in candidate.get("algorithm", []):
        if algorithm.get("id") == "heat_balance_manager_source_order":
            algorithm["port_ticket_mappings"] = []
            break
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "implemented"
    fill_state_mapping(routine)
    routine["rust_target"] = [
        "crates/ep_runtime/src/heat_balance/manager.rs::manage_heat_balance_source_order_stages"
    ]
    expect_error(
        "implemented_requires_parent_port_ticket_mapping",
        routine_errors(candidate),
        "parent port_ticket_mappings must map",
    )

    candidate = copy.deepcopy(ledger)
    for algorithm in candidate.get("algorithm", []):
        if algorithm.get("id") == "heat_balance_manager_source_order":
            algorithm["family_cases"] = [
                "official_1zone_static_model_001",
                "official_1zone_static_model_001",
            ]
            break
    expect_error(
        "family_cases_must_be_unique",
        routine_errors(candidate),
        "family_cases must not contain duplicates",
    )

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "implemented"
    fill_state_mapping(routine)
    routine["rust_target"] = [
        "crates/ep_runtime/src/heat_balance/manager.rs::manage_heat_balance_source_order_stages"
    ]
    positive_errors = routine_errors(candidate)
    if positive_errors:
        raise AssertionError(f"ticketable implemented routine positive case failed: {positive_errors}")
    passed.append("ticketable_implemented_routine_positive")

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "sim_purchased_air")
    routine["completion_status"] = "family_gated"
    fill_state_mapping(routine)
    routine["rust_target"] = ["crates/ep_runtime/src/ideal_loads/dispatch.rs::sim_purchased_air_compat"]
    routine["family_gate_ids"] = ["plant_loop_diagnostic_001"]
    routine["proof_variables"] = ["Plant Supply Side Inlet Temperature"]
    expect_error("diagnostic_case_cannot_family_gate", routine_errors(candidate), "family_gated requires a conformance case")

    candidate = copy.deepcopy(ledger)
    for algorithm in candidate.get("algorithm", []):
        if algorithm.get("id") == "plant_loop_state_projection":
            algorithm["family_cases"] = ["ideal_loads_no_oa_sensible_conformance_001"]
            break
    routine = find_raw_routine(candidate, "manage_plant_loops")
    routine["completion_status"] = "family_gated"
    fill_state_mapping(routine)
    routine["rust_target"] = ["crates/ep_runtime/src/plant/state.rs::simulate_plant_state_projection"]
    routine["family_gate_ids"] = ["ideal_loads_no_oa_sensible_conformance_001"]
    routine["proof_variables"] = ["Zone Ideal Loads Zone Total Heating Rate"]
    expect_error("cross_domain_case_cannot_family_gate", routine_errors(candidate), "scope does not cover routine domain plant")

    candidate = copy.deepcopy(ledger)
    for algorithm in candidate.get("algorithm", []):
        if algorithm.get("id") == "heat_balance_manager_source_order":
            algorithm["family_cases"] = ["official_1zone_static_model_001"]
            break
    routine = find_raw_routine(candidate, "manage_heat_balance")
    routine["completion_status"] = "family_gated"
    fill_state_mapping(routine)
    routine["rust_target"] = ["crates/ep_runtime/src/heat_balance/manager.rs::manage_heat_balance_source_order_stages"]
    routine["family_gate_ids"] = ["official_1zone_static_model_001"]
    routine["proof_variables"] = ["HeatTransfer Surface Area (Net)"]
    expect_error(
        "same_domain_case_requires_explicit_routine_coverage",
        routine_errors(candidate),
        "family gate must declare routine in routine_coverage.routine_ids",
    )

    static_case = load_toml(
        repo_root / "data" / "conformance_cases" / "official_1zone_static_model_001" / "case.toml"
    )
    static_case["routine_coverage"] = {
        "algorithm_ids": ["heat_balance_manager_source_order"],
        "routine_ids": ["manage_heat_balance"],
    }
    with patch.object(ledger_validator, "load_toml", return_value=static_case):
        static_case_errors = routine_errors(candidate)
    expect_error(
        "static_surface_case_cannot_gate_heat_balance_routine",
        static_case_errors,
        "family gate scope must include exact routine domain heat_balance",
    )
    expect_error(
        "routine_proof_must_belong_to_parent_algorithm",
        static_case_errors,
        "proof variable must be declared by parent algorithm",
    )

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "sim_purchased_air")
    routine["completion_status"] = "family_gated"
    fill_state_mapping(routine)
    routine["rust_target"] = ["crates/ep_runtime/src/ideal_loads/dispatch.rs::sim_purchased_air_compat"]
    routine["family_gate_ids"] = ["ideal_loads_no_oa_sensible_conformance_001"]
    routine["proof_variables"] = ["Zone Ideal Loads Supply Air Total Heating Energy"]
    expect_error(
        "diagnostic_output_cannot_prove_family_gate",
        routine_errors(candidate),
        "proof variable is not requested by any family gate",
    )

    candidate = copy.deepcopy(ledger)
    routine = find_raw_routine(candidate, "sim_purchased_air")
    routine["completion_status"] = "complete"
    fill_state_mapping(routine)
    routine["rust_target"] = ["crates/ep_runtime/src/ideal_loads/dispatch.rs::sim_purchased_air_compat"]
    routine["family_gate_ids"] = ["ideal_loads_no_oa_sensible_conformance_001"]
    routine["proof_variables"] = ["Zone Ideal Loads Zone Total Heating Rate"]
    routine["completion_evidence"] = [routine["source_map"]]
    routine["unsupported_active_branches"] = ["self-test active branch"]
    expect_error("complete_rejects_unsupported_active_branch", routine_errors(candidate), "must not retain unsupported_active_branches")

    candidate_contract = copy.deepcopy(contract)
    candidate_contract["domain_claim"][0]["required_routines"] = []
    expect_error("empty_required_routines", domain_errors(ledger, candidate_contract), "required_routines must not be empty")

    candidate_contract = copy.deepcopy(contract)
    required = candidate_contract["domain_claim"][0]["required_routines"]
    required.append(required[0])
    expect_error("duplicate_required_routine", domain_errors(ledger, candidate_contract), "must not contain duplicates")

    candidate_contract = copy.deepcopy(contract)
    candidate_contract["domain_claim"][0]["required_routines"][0] = "zone_air_heat_balance"
    expect_error("algorithm_row_cannot_replace_routine", domain_errors(ledger, candidate_contract), "unknown required routine")

    candidate_contract = copy.deepcopy(contract)
    plant_routine = candidate_contract["domain_claim"][2]["required_routines"][0]
    candidate_contract["domain_claim"][0]["required_routines"][0] = plant_routine
    expect_error("wrong_domain_required_routine", domain_errors(ledger, candidate_contract), "required routine belongs to plant")

    candidate_contract = copy.deepcopy(contract)
    candidate_contract["domain_claim"][1]["required_routines"].remove("sim_purchased_air")
    expect_error(
        "tracked_required_routine_cannot_be_omitted",
        domain_errors(ledger, candidate_contract),
        "must exactly match required_for_full_domain routine records",
    )

    candidate_ledger = copy.deepcopy(ledger)
    candidate_contract = copy.deepcopy(contract)
    find_raw_routine(candidate_ledger, "sim_purchased_air")["required_for_full_domain"] = False
    candidate_contract["domain_claim"][1]["required_routines"].remove("sim_purchased_air")
    expect_error(
        "minimum_required_seed_cannot_be_removed_from_both_specs",
        domain_errors(candidate_ledger, candidate_contract),
        "must retain immutable minimum seed",
    )

    candidate_ledger = copy.deepcopy(ledger)
    candidate_contract = copy.deepcopy(contract)
    heat_claim = candidate_contract["domain_claim"][0]
    heat_claim["routine_inventory_complete"] = True
    candidate_contract["claims"]["broad_heat_balance_compatibility"] = True
    for routine_id in heat_claim["required_routines"]:
        find_raw_routine(candidate_ledger, routine_id)["completion_status"] = "family_gated"
    find_raw_routine(candidate_ledger, heat_claim["required_routines"][0])["completion_status"] = "implemented"
    expect_error("full_domain_rejects_implemented_routine", domain_errors(candidate_ledger, candidate_contract), "every required routine at family_gated or complete")

    candidate_ledger = copy.deepcopy(ledger)
    candidate_contract = copy.deepcopy(contract)
    heat_claim = candidate_contract["domain_claim"][0]
    candidate_contract["claims"]["broad_heat_balance_compatibility"] = True
    for routine_id in heat_claim["required_routines"]:
        find_raw_routine(candidate_ledger, routine_id)["completion_status"] = "family_gated"
    expect_error("full_domain_rejects_incomplete_inventory", domain_errors(candidate_ledger, candidate_contract), "routine_inventory_complete=true")

    candidate_ledger = copy.deepcopy(ledger)
    candidate_contract = copy.deepcopy(contract)
    heat_claim = candidate_contract["domain_claim"][0]
    heat_claim["routine_inventory_complete"] = True
    candidate_contract["claims"]["broad_heat_balance_compatibility"] = True
    for routine_id in heat_claim["required_routines"]:
        find_raw_routine(candidate_ledger, routine_id)["completion_status"] = "family_gated"
    positive_errors = domain_errors(candidate_ledger, candidate_contract)
    if positive_errors:
        raise AssertionError(f"family_gated full-domain positive case failed: {positive_errors}")
    passed.append("family_gated_full_domain_positive")

    candidate_contract = copy.deepcopy(contract)
    candidate_contract["claims"]["full_runtime_compatibility"] = True
    expect_error("full_runtime_remains_locked", domain_errors(ledger, candidate_contract), "full runtime compatibility remains locked")

    print("Algorithm routine completion self-test")
    print(f"  mutations: {len(passed)}")
    for name in passed:
        print(f"  OK {name}")
    return 0
