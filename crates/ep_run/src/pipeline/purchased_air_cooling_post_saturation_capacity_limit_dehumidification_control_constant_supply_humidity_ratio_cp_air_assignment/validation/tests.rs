use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot, psychrometrics::energyplus_psy_cp_air_fn_w,
};

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&OwnerLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp399_validator_requires_cp398_and_cp329_and_accepts_active_none_routes() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    state.transition_count = 1;
    state.dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count = 1;
    state.predecessor_route_counts[20] = 1;
    state.source_site_execution_count = 3;
    state.mixed_air_humidity_ratio_read_count = 1;
    state.psychrometric_cp_air_evaluation_count = 1;
    state.cp_air_assignment_write_count = 1;
    predecessor.transition_count = 1;
    predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entry_count = 1;
    predecessor.predecessor_route_counts[20] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_ok());

    state.predecessor_route_counts[27] = 1;
    predecessor.predecessor_route_counts[27] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}

#[test]
fn ep_run_cp399_rejects_missing_cp398_and_cp329_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, None, Some(1))
        .expect_err("CP399 must require CP398 evidence");
    assert!(error.contains("CP398 evidence is missing"));
}

#[test]
fn cp399_links_cp398_and_cp329_owner_bit_exactly() {
    let humidity_ratio = 0.008;
    let cp_air = energyplus_psy_cp_air_fn_w(humidity_ratio);
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry::test_snapshot(
        None,
        true,
    );
    let mut snapshot = super::super::test_snapshot(None, true);
    snapshot.mixed_air_humidity_ratio = Some(humidity_ratio);
    snapshot.psychrometric_cp_air_result_j_per_kg_k = Some(cp_air);
    snapshot.cp_air_j_per_kg_k = Some(cp_air);
    let owner = active_owner(
        snapshot.system,
        snapshot.parent_call_ordinal,
        snapshot.controlled_zone,
        humidity_ratio,
    );

    assert!(links_to_predecessor(snapshot, predecessor));
    assert!(assignment_shape_is_exact(snapshot, predecessor, owner));
    assert!(carriers_are_preserved(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.source_order = &["forged-cp398-source-order"];
    assert!(!links_to_predecessor(snapshot, corrupted));

    let mut corrupted_owner = owner;
    corrupted_owner.mixed_air_humidity_ratio = Some(f64::from_bits(humidity_ratio.to_bits() ^ 1));
    assert!(!assignment_shape_is_exact(
        snapshot,
        predecessor,
        corrupted_owner
    ));
}

fn active_owner(
    system: IdealLoadsAirSystemId,
    parent_call_ordinal: usize,
    controlled_zone: ZoneId,
    humidity_ratio: f64,
) -> PurchasedAirCalcCoolingMixedAirCallSnapshot {
    PurchasedAirCalcCoolingMixedAirCallSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
        child_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
        no_oa_child_source_order:
            PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
        system,
        parent_call_ordinal,
        controlled_zone,
        unit_body_entered: true,
        predecessor_cooling_body_entered: true,
        predecessor_zero_flow_reset_body_entered: false,
        predecessor_active_guard_false_fallthrough: true,
        unit_off_skipped: false,
        non_cooling_skipped: false,
        cooling_call_executed: true,
        state_reference_bound: true,
        purchased_air_number_read: true,
        outdoor_air_mass_flow_rate_read: true,
        outdoor_air_mass_flow_rate_kg_per_s: Some(0.0),
        supply_mass_flow_rate_read: true,
        supply_mass_flow_rate_kg_per_s: Some(0.25),
        mixed_air_temperature_output_reference_bound: true,
        mixed_air_humidity_ratio_output_reference_bound: true,
        mixed_air_enthalpy_output_reference_bound: true,
        operating_mode_read: true,
        operating_mode: None,
        calc_purch_air_mixed_air_called: true,
        purchased_air_alias_bound: true,
        outdoor_air_node_number_copied: true,
        outdoor_air_node: None,
        recirculation_node_number_copied: true,
        recirculation_node: None,
        recirculation_mass_flow_rate_initialized: true,
        initial_recirculation_mass_flow_rate_kg_per_s: Some(0.0),
        recirculation_temperature_read: true,
        recirculation_temperature_c: Some(20.0),
        recirculation_humidity_ratio_read: true,
        recirculation_humidity_ratio: Some(humidity_ratio),
        recirculation_enthalpy_projection_read: true,
        recirculation_enthalpy_projection_j_per_kg: Some(40_000.0),
        outdoor_air_initialization_guard_evaluated: true,
        outdoor_air_enabled: Some(false),
        outdoor_air_inlet_temperature_c: Some(0.0),
        outdoor_air_inlet_humidity_ratio: Some(0.0),
        outdoor_air_inlet_enthalpy_j_per_kg: Some(0.0),
        outdoor_air_after_heat_recovery_temperature_c: Some(0.0),
        outdoor_air_after_heat_recovery_humidity_ratio: Some(0.0),
        outdoor_air_after_heat_recovery_enthalpy_j_per_kg: Some(0.0),
        heat_recovery_on_false_assigned: true,
        heat_recovery_on: Some(false),
        outdoor_air_active_guard_first_operand_evaluated: true,
        outdoor_air_mass_flow_positive_comparison_evaluated: false,
        no_outdoor_air_fallback_entered: true,
        child_supply_mass_flow_rate_read: true,
        child_supply_mass_flow_rate_kg_per_s: Some(0.25),
        recirculation_mass_flow_rate_assigned_from_supply: true,
        resulting_recirculation_mass_flow_rate_kg_per_s: Some(0.25),
        mixed_air_temperature_assigned: true,
        mixed_air_temperature_c: Some(20.0),
        mixed_air_humidity_ratio_assigned: true,
        mixed_air_humidity_ratio: Some(humidity_ratio),
        mixed_air_enthalpy_projection_assigned: true,
        mixed_air_enthalpy_projection_j_per_kg: Some(40_000.0),
        heat_recovery_sensible_output_positive_zero_assigned: true,
        heat_recovery_sensible_output_w: Some(0.0),
        heat_recovery_latent_output_positive_zero_assigned: true,
        heat_recovery_latent_output_w: Some(0.0),
    }
}
