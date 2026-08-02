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
fn public_cp401_validator_requires_cp400_cp384_and_cp385() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, None, None, Some(1))
        .expect_err("CP401 must require CP400 evidence");
    assert!(error.contains("CP400 evidence is missing"));
}

#[test]
fn cp401_public_route_accounting_is_exact() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    state.transition_count = 1;
    state.dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count =
        1;
    state.predecessor_route_counts[20] = 1;
    state.source_site_execution_count = 4;
    state.cp400_supply_enthalpy_state_owner_count = 1;
    state.unchanged_supply_enthalpy_preservation_count = 1;
    state.cp400_supply_temperature_state_owner_count = 1;
    state.unchanged_supply_temperature_preservation_count = 1;
    state.cooling_total_output_owned_read_count = 1;
    state.cooling_total_output_bit_corroboration_count = 1;
    state.cooling_total_output_read_count = 1;
    state.cooling_sensible_output_owned_read_count = 1;
    state.cooling_sensible_output_read_count = 1;
    state.cooling_latent_output_calculation_count = 1;
    state.cooling_latent_output_assignment_write_count = 1;
    predecessor.transition_count = 1;
    predecessor
        .dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count =
        1;
    predecessor.predecessor_route_counts[20] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_ok());

    state.predecessor_route_counts[27] = 1;
    predecessor.predecessor_route_counts[27] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}

#[test]
fn cp401_retains_all_cp400_values_and_terminal_carriers() {
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment::test_snapshot(
        Some(-0.0),
        true,
    );
    let snapshot = super::super::test_snapshot(Some(-0.0), true);
    assert!(links_to_predecessor(snapshot, predecessor));
    assert!(carriers_are_preserved(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.source_order = &["forged-cp400-source-order"];
    assert!(!links_to_predecessor(snapshot, corrupted));
}
