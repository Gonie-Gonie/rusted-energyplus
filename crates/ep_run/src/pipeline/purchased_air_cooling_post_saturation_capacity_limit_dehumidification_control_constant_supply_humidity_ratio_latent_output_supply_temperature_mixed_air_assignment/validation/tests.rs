use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&OwnerLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp403_validator_requires_cp402_and_cp329() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, None, Some(1))
        .expect_err("CP403 must require CP402 evidence");
    assert!(error.contains("CP402 latest evidence is missing"));
}

#[test]
fn cp403_public_route_and_two_site_assignment_accounting_are_exact() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState::new(system);
    state.transition_count = 1;
    state.predecessor_guard_false_fallthrough_count = 1;
    state.predecessor_route_counts[20] = 1;
    state.predecessor_guard_false_fallthrough_route_counts[20] = 1;
    state.cp402_supply_enthalpy_state_owner_count = 1;
    state.unchanged_supply_enthalpy_preservation_count = 1;
    state.cp402_supply_temperature_state_owner_count = 1;
    state.unchanged_supply_temperature_preservation_count = 1;
    predecessor.transition_count = 1;
    predecessor.predecessor_route_counts[20] = 1;
    predecessor.guard_false_fallthrough_route_counts[20] = 1;
    predecessor.cp401_supply_enthalpy_state_owner_count = 1;
    predecessor.cp401_supply_temperature_state_owner_count = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_ok());

    state.predecessor_route_counts[21] = 1;
    predecessor.predecessor_route_counts[21] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}

#[test]
fn cp403_retains_all_cp402_values_and_guard_decision() {
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard::test_snapshot(
        Some(-0.0),
        true,
    );
    let snapshot = super::super::test_snapshot(Some(-0.0), true);
    assert!(links_to_predecessor(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.source_order = &["forged-cp402-source-order"];
    assert!(!links_to_predecessor(snapshot, corrupted));

    let mut corrupted_body = predecessor;
    corrupted_body
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered = false;
    assert!(!links_to_predecessor(snapshot, corrupted_body));
}
