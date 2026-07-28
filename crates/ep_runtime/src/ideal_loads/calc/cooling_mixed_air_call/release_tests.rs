use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallError,
    PurchasedAirCalcCoolingMixedAirCallRecirculationInput,
    advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
    advance_direct_no_oa_calc_cooling_humidification_flow,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body,
    moist_air_enthalpy_j_per_kg,
};

#[derive(Clone, Copy)]
enum Corruption {
    Temperature(f64),
    HumidityRatio(f64),
    EnthalpyProjectionOverflow,
}

#[test]
fn active_nonfinite_recirculation_inputs_fail_before_any_cp329_mutation() {
    let (runtime, system, predecessor, zone_state) = release_case();
    let cases = [
        (
            Corruption::Temperature(f64::from_bits(0x7ff8_0000_0000_00a1)),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::Temperature,
        ),
        (
            Corruption::Temperature(f64::INFINITY),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::Temperature,
        ),
        (
            Corruption::Temperature(f64::NEG_INFINITY),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::Temperature,
        ),
        (
            Corruption::HumidityRatio(f64::from_bits(0xfff8_0000_0000_00b2)),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::HumidityRatio,
        ),
        (
            Corruption::HumidityRatio(f64::INFINITY),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::HumidityRatio,
        ),
        (
            Corruption::HumidityRatio(f64::NEG_INFINITY),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::HumidityRatio,
        ),
        (
            Corruption::EnthalpyProjectionOverflow,
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::EnthalpyProjection,
        ),
    ];

    for (corruption, expected_input) in cases {
        let mut corrupted_zone_state = zone_state.clone();
        match corruption {
            Corruption::Temperature(value) => {
                corrupted_zone_state.mean_air_temperature_c = value;
            }
            Corruption::HumidityRatio(value) => {
                corrupted_zone_state.air_humidity_ratio = value;
            }
            Corruption::EnthalpyProjectionOverflow => {
                corrupted_zone_state.mean_air_temperature_c = f64::MAX / 2.0;
                corrupted_zone_state.air_humidity_ratio = 0.0;
                assert!(corrupted_zone_state.mean_air_temperature_c.is_finite());
                assert!(corrupted_zone_state.air_humidity_ratio.is_finite());
                assert!(
                    !moist_air_enthalpy_j_per_kg(
                        corrupted_zone_state.mean_air_temperature_c,
                        corrupted_zone_state.air_humidity_ratio,
                    )
                    .is_finite()
                );
            }
        }

        let mut case_runtime = runtime.clone();
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_mixed_air_call(
                &mut case_runtime,
                &system,
                predecessor,
                &corrupted_zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingMixedAirCallError::NonFiniteRecirculationState {
                    system: system.id,
                    input: expected_input,
                }
            )
        );
        assert_eq!(case_runtime, before);
    }
}

#[test]
fn core_counter_products_reject_overflow_instead_of_saturating() {
    assert!(!super::release::counter_product_matches(
        usize::MAX,
        usize::MAX,
        2
    ));
    assert!(super::release::counter_product_matches(18, 2, 9));
}

fn install_active_cp328_at_ordinal(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    mut predecessor: crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    ordinal: usize,
    cooling_body_entry_count: usize,
) -> crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
    predecessor.parent_call_ordinal = ordinal;
    {
        let unit = runtime
            .units
            .get_mut(&predecessor.system)
            .expect("known unit");
        unit.init_call_count = ordinal;
        unit.calc_entry.call_count = ordinal;
        let minimum_oa = unit
            .calc_minimum_oa_prefix
            .latest
            .as_mut()
            .expect("minimum OA");
        minimum_oa.parent_call_ordinal = ordinal;
        unit.calc_minimum_oa_prefix.transition_count = ordinal;

        let state = &mut unit.calc_cooling_supply_mass_flow_very_small_guard_body;
        state.transition_count = ordinal;
        state.unit_off_skip_count = ordinal - cooling_body_entry_count;
        state.non_cooling_skip_count = 0;
        state.cooling_body_entry_count = cooling_body_entry_count;
        state.latest = Some(predecessor);
    }
    runtime.set_cooling_supply_mass_flow_very_small_guard_body_latest_witness(
        predecessor.system,
        predecessor,
    );
    predecessor
}

fn redistribute_completed_mixed_air_history_to_unit_off(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystemId,
) {
    let state = &mut runtime
        .units
        .get_mut(&system)
        .expect("known unit")
        .calc_cooling_mixed_air_call;
    assert_eq!(state.cooling_call_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    state.cooling_call_count = 0;
    state.unit_off_skip_count = 1;
    state.caller_source_site_execution_count = 0;
    state.child_source_site_execution_count = 0;
    state.state_reference_bind_count = 0;
    state.purchased_air_number_read_count = 0;
    state.outdoor_air_mass_flow_rate_read_count = 0;
    state.supply_mass_flow_rate_read_count = 0;
    state.mixed_air_output_reference_bind_count = 0;
    state.operating_mode_read_count = 0;
    state.mixed_air_child_call_count = 0;
    state.no_outdoor_air_fallback_count = 0;
    state.recirculation_enthalpy_projection_count = 0;
    state.mixed_air_output_assignment_count = 0;
    state.heat_recovery_output_positive_zero_assignment_count = 0;
}

#[test]
fn public_cp329_completed_and_pending_history_redistribution_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor, zone_state) = release_case();
    let snapshot = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP329");
    redistribute_completed_mixed_air_history_to_unit_off(&mut runtime, system.id);

    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !super::release::completed_direct_cooling_mixed_air_call_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            runtime.cooling_mixed_air_call_latest_witness(system.id),
        )
    );

    let next_predecessor = install_active_cp328_at_ordinal(&mut runtime, predecessor, 2, 2);

    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !super::release::pending_mixed_air_history_links_to_predecessor_for_test(
            unit,
            next_predecessor,
        )
    );

    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_mixed_air_call(
            &mut runtime,
            &system,
            next_predecessor,
            &zone_state,
        ),
        Err(
            PurchasedAirCalcCoolingMixedAirCallError::RuntimeStateInvariantViolation {
                system: system.id,
            }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn active_cp329_child_site_increment_overflow_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor, zone_state) = release_case();
    let latest = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP329");

    let prior_cooling_calls = usize::MAX / 22;
    let next_ordinal = prior_cooling_calls + 1;
    let mut prior_snapshot = latest;
    prior_snapshot.parent_call_ordinal = prior_cooling_calls;
    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_mixed_air_call;
        state.transition_count = prior_cooling_calls;
        state.cooling_call_count = prior_cooling_calls;
        state.unit_off_skip_count = 0;
        state.non_cooling_skip_count = 0;
        state.caller_source_site_execution_count = prior_cooling_calls * 9;
        state.child_source_site_execution_count = prior_cooling_calls * 22;
        state.state_reference_bind_count = prior_cooling_calls;
        state.purchased_air_number_read_count = prior_cooling_calls;
        state.outdoor_air_mass_flow_rate_read_count = prior_cooling_calls;
        state.supply_mass_flow_rate_read_count = prior_cooling_calls;
        state.mixed_air_output_reference_bind_count = prior_cooling_calls * 3;
        state.operating_mode_read_count = prior_cooling_calls;
        state.mixed_air_child_call_count = prior_cooling_calls;
        state.no_outdoor_air_fallback_count = prior_cooling_calls;
        state.recirculation_enthalpy_projection_count = prior_cooling_calls;
        state.mixed_air_output_assignment_count = prior_cooling_calls * 3;
        state.heat_recovery_output_positive_zero_assignment_count = prior_cooling_calls * 2;
        state.latest = Some(prior_snapshot);
        state.latest_route =
            Some(super::PurchasedAirCalcCoolingMixedAirCallRetainedRoute::NoOutdoorAirFallback);
        state.latest_transition_ordinal = Some(prior_cooling_calls);
    }
    runtime.set_cooling_mixed_air_call_latest_witness(system.id, prior_snapshot);
    let next_predecessor =
        install_active_cp328_at_ordinal(&mut runtime, predecessor, next_ordinal, next_ordinal);

    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        super::release::pending_mixed_air_history_links_to_predecessor_for_test(
            unit,
            next_predecessor,
        )
    );
    assert!(!super::release::next_mixed_air_transition_fits_for_test(
        &unit.calc_cooling_mixed_air_call,
        next_predecessor,
    ));

    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_mixed_air_call(
            &mut runtime,
            &system,
            next_predecessor,
            &zone_state,
        ),
        Err(
            PurchasedAirCalcCoolingMixedAirCallError::RuntimeStateInvariantViolation {
                system: system.id,
            }
        )
    );
    assert_eq!(runtime, before);
}

pub(in crate::ideal_loads::calc) fn release_case() -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    crate::heat_balance::state::ZoneHeatBalanceState,
) {
    release_case_with_demand(-1_000.0)
}

pub(in crate::ideal_loads::calc) fn release_case_with_demand(
    cooling_demand_w: f64,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    crate::heat_balance::state::ZoneHeatBalanceState,
) {
    let (mut runtime, system, sensible) =
        super::super::cooling_dehumidification_flow_release_tests::release_case(cooling_demand_w);
    let dehumidification =
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, sensible)
            .expect("CP319");
    let humidification = advance_direct_no_oa_calc_cooling_humidification_flow(
        &mut runtime,
        &system,
        dehumidification,
    )
    .expect("CP320");
    let reset = advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
        &mut runtime,
        &system,
        humidification,
    )
    .expect("CP321");
    let maximum =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(&mut runtime, &system, reset)
            .expect("CP322");
    let ems_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
        &mut runtime,
        &system,
        maximum,
    )
    .expect("CP323");
    let ems_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
        &mut runtime,
        &system,
        ems_guard,
    )
    .expect("CP324");
    let limit_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
        &mut runtime,
        &system,
        ems_body,
    )
    .expect("CP325");
    let limit_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
        &mut runtime,
        &system,
        limit_guard,
    )
    .expect("CP326");
    let very_small_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
        &mut runtime,
        &system,
        limit_body,
    )
    .expect("CP327");
    let very_small_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
        &mut runtime,
        &system,
        very_small_guard,
    )
    .expect("CP328");
    let zone_state = super::super::cooling_sensible_flow_release_tests::zone_state(
        very_small_body.controlled_zone,
    );
    (runtime, system, very_small_body, zone_state)
}

pub(in crate::ideal_loads::calc) fn install_completed_active_case_at_ordinal(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingMixedAirCallSnapshot,
    ordinal: usize,
) -> crate::ideal_loads::PurchasedAirCalcCoolingMixedAirCallSnapshot {
    install_completed_case_at_ordinal(runtime, snapshot, ordinal, ordinal - 1, 0, 1)
}

pub(in crate::ideal_loads::calc) fn install_completed_case_at_ordinal(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    mut snapshot: crate::ideal_loads::PurchasedAirCalcCoolingMixedAirCallSnapshot,
    ordinal: usize,
    unit_off_skip_count: usize,
    non_cooling_skip_count: usize,
    cooling_call_count: usize,
) -> crate::ideal_loads::PurchasedAirCalcCoolingMixedAirCallSnapshot {
    assert_eq!(
        unit_off_skip_count
            .checked_add(non_cooling_skip_count)
            .and_then(|count| count.checked_add(cooling_call_count)),
        Some(ordinal)
    );
    snapshot.parent_call_ordinal = ordinal;
    {
        let unit = runtime
            .units
            .get_mut(&snapshot.system)
            .expect("known system");
        unit.init_call_count = ordinal;
        unit.calc_entry.call_count = ordinal;

        let state = &mut unit.calc_cooling_mixed_air_call;
        state.transition_count = ordinal;
        state.cooling_call_count = cooling_call_count;
        state.unit_off_skip_count = unit_off_skip_count;
        state.non_cooling_skip_count = non_cooling_skip_count;
        state.caller_source_site_execution_count = cooling_call_count * 9;
        state.child_source_site_execution_count = cooling_call_count * 22;
        state.state_reference_bind_count = cooling_call_count;
        state.purchased_air_number_read_count = cooling_call_count;
        state.outdoor_air_mass_flow_rate_read_count = cooling_call_count;
        state.supply_mass_flow_rate_read_count = cooling_call_count;
        state.mixed_air_output_reference_bind_count = cooling_call_count * 3;
        state.operating_mode_read_count = cooling_call_count;
        state.mixed_air_child_call_count = cooling_call_count;
        state.no_outdoor_air_fallback_count = cooling_call_count;
        state.recirculation_enthalpy_projection_count = cooling_call_count;
        state.mixed_air_output_assignment_count = cooling_call_count * 3;
        state.heat_recovery_output_positive_zero_assignment_count = cooling_call_count * 2;
        state.latest = Some(snapshot);
        state.latest_route = Some(if snapshot.unit_off_skipped {
            super::PurchasedAirCalcCoolingMixedAirCallRetainedRoute::UnitOff
        } else if snapshot.non_cooling_skipped {
            super::PurchasedAirCalcCoolingMixedAirCallRetainedRoute::NonCooling
        } else {
            super::PurchasedAirCalcCoolingMixedAirCallRetainedRoute::NoOutdoorAirFallback
        });
        state.latest_transition_ordinal = Some(ordinal);
    }
    runtime.set_cooling_mixed_air_call_latest_witness(snapshot.system, snapshot);
    snapshot
}
