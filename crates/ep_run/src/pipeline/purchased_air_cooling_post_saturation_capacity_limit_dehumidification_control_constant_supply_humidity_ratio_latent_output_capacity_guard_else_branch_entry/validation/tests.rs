use ep_model::IdealLoadsAirSystemId;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
};

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp406_validator_depends_only_on_cp405_and_runtime_identity() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
}

#[test]
fn ep_run_cp406_rejects_missing_cp405_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1))
        .expect_err("CP406 must require CP405 evidence");
    assert!(error.contains("CP405 evidence is missing"));
}

#[test]
fn public_route_validator_accepts_else_entry_and_rejects_private_routes() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    state.transition_count = 1;
    state.predecessor_guard_false_fallthrough_count = 1;
    state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count = 1;
    state.predecessor_route_counts[20] = 1;
    state.predecessor_guard_false_fallthrough_route_counts[20] = 1;
    state.else_branch_entry_route_counts[20] = 1;
    state.source_site_execution_count = 1;
    predecessor.transition_count = 1;
    predecessor.predecessor_route_counts[20] = 1;
    predecessor.predecessor_guard_false_fallthrough_count = 1;
    predecessor.predecessor_guard_false_fallthrough_route_counts[20] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_ok());

    state.predecessor_route_counts[21] = 1;
    predecessor.predecessor_route_counts[21] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}

#[test]
fn checked_accounting_fails_closed_on_overflow() {
    assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
}
