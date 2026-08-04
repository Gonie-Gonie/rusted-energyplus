//! CP414 route, IEEE, release-shape, and overflow tests.

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::tests::{
    all_routes, predecessor_for_outcome, predecessor_with_enthalpy,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as Cp413State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance_cp413,
};
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization,
};
use crate::psychrometrics::energyplus_psy_tsat_fn_h_pb_raw;

#[test]
fn cp414_boundary_and_four_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2316",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2319",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-cp413-retained-supply-enthalpy-for-saturation-temperature",
            "read-environment-outdoor-barometric-pressure-for-saturation-temperature",
            "evaluate-psy-tsat-fn-h-pb",
            "assign-purchased-air-supply-temperature-to-saturation-temperature",
        ],
    );
}

#[test]
fn exhaustive_54_outcome_transition_and_four_route_partitions_are_exact() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    let system = ep_model::IdealLoadsAirSystemId(412);
    let mut cp413_state = Cp413State::new(system);
    let mut state = State::new(system);
    let mut expected_predecessor = [0usize; 36];
    let mut expected_guard_false = [0usize; 36];
    let mut expected_guard_body = [0usize; 36];
    let mut expected_assignment = [0usize; 36];
    let mut ordinal = 0usize;

    for route in routes {
        let outcomes: &[bool] = if route.active { &[false, true] } else { &[false] };
        for &body_entered in outcomes {
            ordinal += 1;
            let cp412 = predecessor_for_outcome(route, ordinal, body_entered);
            let cp413 = advance_cp413(&mut cp413_state, cp412).expect("valid CP413 outcome");
            let cp414_route = super::transition::predecessor_route(cp413).expect("valid CP414 route");
            let pressure = if body_entered {
                91_325.0
            } else {
                f64::from_bits(0x7ff8_0000_0000_4140)
            };
            let snapshot = advance(&mut state, cp413, pressure).expect("valid CP414 outcome");
            let index = cp414_route.logical_index;
            expected_predecessor[index] += 1;
            if cp414_route.predecessor_guard_false_fallthrough {
                expected_guard_false[index] += 1;
            }
            if cp414_route.predecessor_guard_body_entered {
                expected_guard_body[index] += 1;
            }
            if cp414_route.assignment_executed {
                expected_assignment[index] += 1;
            }

            assert_eq!(
                snapshot.post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed,
                body_entered,
            );
            assert!(option_bits_equal(
                snapshot.predecessor_cp413_resulting_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ));
            assert!(option_bits_equal(
                snapshot.predecessor_cp413_resulting_supply_enthalpy_j_per_kg,
                snapshot.resulting_supply_enthalpy_j_per_kg,
            ));
            if body_entered {
                let enthalpy = cp413
                    .resulting_supply_enthalpy_j_per_kg
                    .expect("active enthalpy owner");
                let expected = energyplus_psy_tsat_fn_h_pb_raw(enthalpy, pressure);
                assert_eq!(
                    snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                    Some(expected.to_bits()),
                );
                assert_eq!(
                    snapshot
                        .outdoor_barometric_pressure_for_saturation_temperature_pa
                        .map(f64::to_bits),
                    Some(pressure.to_bits()),
                );
            } else {
                assert!(option_bits_equal(
                    snapshot.predecessor_cp413_resulting_supply_temperature_c,
                    snapshot.resulting_supply_temperature_c,
                ));
                assert!(snapshot
                    .outdoor_barometric_pressure_for_saturation_temperature_pa
                    .is_none());
            }
            assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact(snapshot));
        }
    }

    assert_eq!(ordinal, 54);
    assert_eq!(state.transition_count, 54);
    assert_eq!(state.inactive_transition_count, 36);
    assert_eq!(state.saturation_supply_temperature_assignment_count, 18);
    assert_eq!(state.source_site_execution_count, 72);
    assert_eq!(state.cp413_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 36);
    assert_eq!(state.cp413_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.cp413_supply_temperature_state_owner_count, 51);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 33);
    assert_eq!(state.cp414_saturation_supply_temperature_state_owner_count, 18);
    for count in [
        state.cp413_retained_supply_enthalpy_owned_read_count,
        state.supply_enthalpy_for_saturation_temperature_read_count,
        state.environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count,
        state.environment_outdoor_barometric_pressure_for_saturation_temperature_read_count,
        state.psy_tsat_fn_h_pb_evaluation_count,
        state.purchased_air_supply_temperature_saturation_assignment_write_count,
    ] {
        assert_eq!(count, 18);
    }
    assert_eq!(state.predecessor_route_counts, expected_predecessor);
    assert_eq!(
        state.predecessor_guard_false_fallthrough_route_counts,
        expected_guard_false,
    );
    assert_eq!(state.predecessor_guard_body_entry_route_counts, expected_guard_body);
    assert_eq!(
        state.supply_temperature_saturation_assignment_route_counts,
        expected_assignment,
    );
}

#[test]
fn active_raw_pressure_is_retained_but_inactive_pressure_is_not_read() {
    let route = all_routes()
        .into_iter()
        .find(|route| {
            route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough
        })
        .expect("active public route");
    let active_cp412 = predecessor_for_outcome(route, 1, true);
    let active_cp413 = advance_cp413(
        &mut Cp413State::new(active_cp412.system),
        active_cp412,
    )
    .expect("active CP413");
    let pressure_nan = f64::from_bits(0x7ff8_0000_0000_4141);
    let raw = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
        active_cp413,
        pressure_nan,
    )
    .expect("raw CP414 characterization");
    assert_eq!(
        raw.outdoor_barometric_pressure_for_saturation_temperature_pa
            .map(f64::to_bits),
        Some(pressure_nan.to_bits()),
    );
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(raw));

    let inactive_cp412 = predecessor_for_outcome(route, 1, false);
    let inactive_cp413 = advance_cp413(
        &mut Cp413State::new(inactive_cp412.system),
        inactive_cp412,
    )
    .expect("guard-false CP413");
    let inactive = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
        inactive_cp413,
        pressure_nan,
    )
    .expect("inactive CP414 characterization");
    assert!(inactive
        .outdoor_barometric_pressure_for_saturation_temperature_pa
        .is_none());
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(inactive));
}

#[test]
fn ieee_enthalpy_bits_are_preserved_and_nonfinite_public_evidence_is_rejected() {
    let route = all_routes()
        .into_iter()
        .find(|route| {
            route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough
        })
        .expect("active public route");
    let enthalpy = -0.0;
    let cp412 = predecessor_with_enthalpy(route, 1, 0.03, 18.0, 101_325.0, enthalpy);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412)
        .expect("signed-zero CP413");
    let snapshot = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
        cp413,
        101_325.0,
    )
    .expect("signed-zero CP414");
    assert_eq!(
        snapshot
            .supply_enthalpy_for_saturation_temperature_j_per_kg
            .map(f64::to_bits),
        Some(enthalpy.to_bits()),
    );
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(snapshot));

    let cp412 = predecessor_with_enthalpy(
        route,
        1,
        0.03,
        18.0,
        101_325.0,
        f64::from_bits(0x7ff8_0000_0000_4142),
    );
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412)
        .expect("NaN-enthlapy CP413");
    let raw = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
        cp413,
        101_325.0,
    )
    .expect("NaN-enthalpy CP414");
    assert!(raw
        .psychrometric_saturation_supply_temperature_result_c
        .is_some_and(f64::is_nan));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(raw));

    let mut forged_result = snapshot;
    forged_result.psychrometric_saturation_supply_temperature_result_c = Some(f64::NAN);
    forged_result.assigned_saturation_supply_temperature_c = Some(f64::NAN);
    forged_result.resulting_supply_temperature_c = Some(f64::NAN);
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(forged_result));
}

#[test]
fn every_incremented_counter_overflow_is_transactional() {
    let route = all_routes()
        .into_iter()
        .find(|route| {
            route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough
        })
        .expect("active public route");
    let cp412 = predecessor_for_outcome(route, 1, true);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412)
        .expect("active-body CP413");
    let index = super::transition::predecessor_route(cp413)
        .expect("CP414 route")
        .logical_index;
    type Mutation = fn(&mut State, usize);
    let mutations: &[Mutation] = &[
        |state, _| state.transition_count = usize::MAX,
        |state, index| state.predecessor_route_counts[index] = usize::MAX,
        |state, index| state.predecessor_guard_body_entry_route_counts[index] = usize::MAX,
        |state, index| state.supply_temperature_saturation_assignment_route_counts[index] = usize::MAX,
        |state, _| state.source_site_execution_count = usize::MAX,
        |state, _| state.saturation_supply_temperature_assignment_count = usize::MAX,
        |state, _| state.cp413_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        |state, _| state.cp413_supply_enthalpy_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state, _| state.cp413_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.cp414_saturation_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.cp413_retained_supply_enthalpy_owned_read_count = usize::MAX,
        |state, _| state.supply_enthalpy_for_saturation_temperature_read_count = usize::MAX,
        |state, _| state.environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count = usize::MAX,
        |state, _| state.environment_outdoor_barometric_pressure_for_saturation_temperature_read_count = usize::MAX,
        |state, _| state.psy_tsat_fn_h_pb_evaluation_count = usize::MAX,
        |state, _| state.purchased_air_supply_temperature_saturation_assignment_write_count = usize::MAX,
    ];
    for mutate in mutations {
        let mut state = State::new(cp413.system);
        mutate(&mut state, index);
        let before = state.clone();
        assert!(advance(&mut state, cp413, 101_325.0).is_none());
        assert_eq!(state, before);
    }

    let cp412 = predecessor_for_outcome(route, 1, false);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412)
        .expect("guard-false CP413");
    let index = super::transition::predecessor_route(cp413)
        .expect("guard-false CP414 route")
        .logical_index;
    for mutate in [
        (|state: &mut State, index: usize| {
            state.predecessor_guard_false_fallthrough_route_counts[index] = usize::MAX
        }) as Mutation,
        |state: &mut State, _| state.inactive_transition_count = usize::MAX,
        |state: &mut State, _| {
            state.unchanged_supply_temperature_preservation_count = usize::MAX
        },
    ] {
        let mut state = State::new(cp413.system);
        mutate(&mut state, index);
        let before = state.clone();
        assert!(advance(&mut state, cp413, f64::NAN).is_none());
        assert_eq!(state, before);
    }

    let inactive_route = all_routes()
        .into_iter()
        .find(|route| !route.active)
        .expect("CP413-inactive route");
    let cp412 = predecessor_for_outcome(inactive_route, 1, false);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412)
        .expect("CP413-inactive snapshot");
    let mut state = State::new(cp413.system);
    state.inactive_transition_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, cp413, f64::NAN).is_none());
    assert_eq!(state, before);
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
