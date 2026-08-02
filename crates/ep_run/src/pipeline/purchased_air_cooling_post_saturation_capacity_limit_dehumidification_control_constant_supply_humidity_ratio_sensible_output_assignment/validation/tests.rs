use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&FlowOwnerLifecycle>,
    Option<&MixedOwnerLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp400_validator_requires_cp399_cp330_and_cp329() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, None, None, Some(1))
        .expect_err("CP400 must require CP399 evidence");
    assert!(error.contains("CP399 evidence is missing"));
}

#[test]
fn cp400_public_route_accounting_is_exact() {
    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    state.transition_count = 1;
    state
        .dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count =
        1;
    state.predecessor_route_counts[20] = 1;
    state.source_site_execution_count = 8;
    state.cp399_supply_enthalpy_state_owner_count = 1;
    state.unchanged_supply_enthalpy_preservation_count = 1;
    state.cp399_supply_temperature_state_owner_count = 1;
    state.unchanged_supply_temperature_preservation_count = 1;
    state.supply_mass_flow_rate_owned_read_count = 1;
    state.supply_mass_flow_rate_bit_corroboration_count = 1;
    state.supply_mass_flow_rate_read_count = 1;
    state.cp_air_owned_read_count = 1;
    state.cp_air_read_count = 1;
    state.supply_mass_flow_rate_times_cp_air_calculation_count = 1;
    state.mixed_air_temperature_owned_read_count = 1;
    state.mixed_air_temperature_read_count = 1;
    state.supply_temperature_owned_read_count = 1;
    state.supply_temperature_read_count = 1;
    state.mixed_air_minus_supply_temperature_calculation_count = 1;
    state.cooling_sensible_output_calculation_count = 1;
    state.cooling_sensible_output_assignment_write_count = 1;
    predecessor.transition_count = 1;
    predecessor.dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count = 1;
    predecessor.predecessor_route_counts[20] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_ok());

    state.predecessor_route_counts[27] = 1;
    predecessor.predecessor_route_counts[27] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}

#[test]
fn cp400_retains_all_cp399_values_and_terminal_carriers() {
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment::test_snapshot(
        Some(-0.0),
        true,
    );
    let snapshot = super::super::test_snapshot(Some(-0.0), true);
    assert!(links_to_predecessor(snapshot, predecessor));
    assert!(carriers_are_preserved(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.source_order = &["forged-cp399-source-order"];
    assert!(!links_to_predecessor(snapshot, corrupted));
}
