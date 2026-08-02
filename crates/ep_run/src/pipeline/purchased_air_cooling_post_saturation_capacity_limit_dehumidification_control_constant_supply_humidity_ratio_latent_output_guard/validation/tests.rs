use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&OwnerLifecycle>,
    Option<&CorroboratorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp402_validator_requires_cp401_cp321_and_cp340() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, None, None, Some(1))
        .expect_err("CP402 must require CP401 evidence");
    assert!(error.contains("CP401 latest evidence is missing"));
}

#[test]
fn cp402_public_route_and_three_q_plus_body_accounting_are_exact() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState::new(system);
    state.transition_count = 1;
    state
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count = 1;
    state.predecessor_route_counts[20] = 1;
    state.guard_false_fallthrough_route_counts[20] = 1;
    state.source_site_execution_count = 3;
    state.cp401_supply_enthalpy_state_owner_count = 1;
    state.unchanged_supply_enthalpy_preservation_count = 1;
    state.cp401_supply_temperature_state_owner_count = 1;
    state.unchanged_supply_temperature_preservation_count = 1;
    state.cp401_cooling_latent_output_owned_read_count = 1;
    state.cooling_latent_output_read_count = 1;
    state.cp321_maximum_total_cooling_capacity_owned_read_count = 1;
    state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count = 1;
    state.maximum_total_cooling_capacity_read_count = 1;
    state.cooling_latent_output_maximum_total_cooling_capacity_comparison_count = 1;
    state
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count = 1;
    predecessor.transition_count = 1;
    predecessor
        .dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count = 1;
    predecessor.predecessor_route_counts[20] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_ok());

    state.predecessor_route_counts[21] = 1;
    predecessor.predecessor_route_counts[21] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}

#[test]
fn cp402_retains_all_cp401_values_and_terminal_carriers() {
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment::test_snapshot(
        Some(-0.0),
        true,
    );
    let snapshot = super::super::test_snapshot(Some(-0.0), true);
    assert!(links_to_predecessor(snapshot, predecessor));
    assert!(carriers_are_preserved(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.source_order = &["forged-cp401-source-order"];
    assert!(!links_to_predecessor(snapshot, corrupted));

    let mut corrupted_owner_flag = predecessor;
    corrupted_owner_flag.cp384_retained_cooling_total_output_owned_read = false;
    assert!(!links_to_predecessor(snapshot, corrupted_owner_flag));
}
