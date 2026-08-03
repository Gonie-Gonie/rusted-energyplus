use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&EnthalpyOwnerLifecycle>,
    Option<&HumidityOwnerLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp407_validator_depends_only_on_predecessor_owners_and_runtime_identity() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
}

#[test]
fn ep_run_cp407_rejects_missing_cp385_owner_before_runtime_admission() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, None, None, Some(1))
        .expect_err("CP407 must require CP406 before owner/runtime validation");
    assert!(error.contains("CP406 evidence is missing"));
}

#[test]
fn cp407_source_counter_overflow_fails_closed() {
    let mut state = State::new(IdealLoadsAirSystemId(0));
    state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count =
        usize::MAX;
    let error = validate_source_counters(&state).expect_err("overflow must be rejected");
    assert!(error.contains("overflowed"));
}

#[test]
fn cp407_public_route_validator_rejects_private_base_route() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    state.transition_count = 1;
    state.inactive_transition_count = 1;
    state.predecessor_route_counts[21] = 1;
    predecessor.transition_count = 1;
    predecessor.inactive_transition_count = 1;
    predecessor.predecessor_route_counts[21] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}
