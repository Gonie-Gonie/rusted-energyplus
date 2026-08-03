use ep_model::IdealLoadsAirSystemId;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
};

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp405_validator_depends_only_on_cp404_and_runtime_identity() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
}

#[test]
fn ep_run_cp405_rejects_missing_cp404_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1))
        .expect_err("CP405 must require CP404 evidence");
    assert!(error.contains("CP404 evidence is missing"));
}

#[test]
fn public_route_validator_rejects_private_routes_and_counter_drift() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    assert!(validate_public_route_contract(&state, &predecessor).is_ok());

    state.predecessor_route_counts[21] = 1;
    predecessor.predecessor_route_counts[21] = 1;
    predecessor.supply_humidity_ratio_assignment_route_counts[21] = 1;
    state.cooling_latent_output_maximum_capacity_assignment_route_counts[21] = 1;
    state.transition_count = 1;
    predecessor.transition_count = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}

#[test]
fn checked_accounting_fails_closed_on_overflow() {
    assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
    let mut counts = [0usize; 30];
    counts[20] = usize::MAX;
    counts[24] = 1;
    assert!(checked_selected_sum(&counts, &[20, 24], "overflow").is_err());
}
