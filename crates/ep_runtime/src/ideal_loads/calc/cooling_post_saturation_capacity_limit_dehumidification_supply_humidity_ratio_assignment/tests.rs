//! CP416 boundary, route, IEEE, corruption, and overflow tests.

use super::transition::predecessor_route;
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, active_input as mixed_air_active_input,
    predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::tests::{
    all_routes, predecessor_for_outcome, predecessor_with_enthalpy,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingMixedAirCallRuntimeState as MixedAirState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as Cp413State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as Cp414State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState as Cp415State,
    advance_cooling_mixed_air_call_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance_cp413,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state as advance_cp414,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_state as advance_cp415,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot as Predecessor,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

#[test]
fn cp416_boundary_and_four_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2320",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2321",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-humidity-ratio-inversion",
            "read-local-supply-enthalpy-for-post-saturation-capacity-limit-dehumidification-humidity-ratio-inversion",
            "evaluate-psy-w-fn-tdb-h-for-post-saturation-capacity-limit-dehumidification",
            "assign-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-dehumidification",
        ],
    );
}

#[test]
fn exhaustive_54_outcomes_and_six_route_partitions_are_exact() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    let system = ep_model::IdealLoadsAirSystemId(412);
    let mut cp413_state = Cp413State::new(system);
    let mut cp414_state = Cp414State::new(system);
    let mut cp415_state = Cp415State::new(system);
    let mut state = State::new(system);
    let mut expected_predecessor = [0usize; 36];
    let mut expected_guard_false = [0usize; 36];
    let mut expected_guard_body = [0usize; 36];
    let mut expected_saturation_assignment = [0usize; 36];
    let mut expected_mixed_air_limit = [0usize; 36];
    let mut expected_humidity_assignment = [0usize; 36];
    let mut active_public_outcomes = Vec::new();
    let mut snapshots = Vec::new();
    let mut ordinal = 0usize;

    for route in routes {
        let outcomes: &[bool] = if route.active {
            &[false, true]
        } else {
            &[false]
        };
        for &body_entered in outcomes {
            let conceptual_index = ordinal;
            ordinal += 1;
            let cp412 = predecessor_for_outcome(route, ordinal, body_entered);
            let cp413 = advance_cp413(&mut cp413_state, cp412).expect("valid CP413 outcome");
            let cp414 =
                advance_cp414(&mut cp414_state, cp413, 91_325.0).expect("valid CP414 outcome");
            let owner = body_entered.then(|| matching_mixed_air_owner(cp414, 17.0));
            let cp415 = advance_cp415(&mut cp415_state, cp414, owner).expect("valid CP415 outcome");
            let cp416_route = predecessor_route(cp415).expect("valid CP416 route");
            let snapshot = advance(&mut state, cp415).expect("valid CP416 outcome");
            let index = cp416_route.logical_index;

            expected_predecessor[index] += 1;
            if cp416_route.predecessor_guard_false_fallthrough {
                expected_guard_false[index] += 1;
            }
            if cp416_route.predecessor_guard_body_entered {
                expected_guard_body[index] += 1;
            }
            if cp416_route.predecessor_saturation_temperature_assignment_executed {
                expected_saturation_assignment[index] += 1;
            }
            if cp416_route.predecessor_saturation_temperature_mixed_air_limit_executed {
                expected_mixed_air_limit[index] += 1;
            }
            if cp416_route.active {
                expected_humidity_assignment[index] += 1;
                if matches!(route.predecessor_index, 0..=8 | 20 | 24) {
                    active_public_outcomes.push(conceptual_index);
                }
            }

            assert_snapshot_matches_predecessor(snapshot, cp415, cp416_route.active);
            assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact(snapshot));
            snapshots.push(snapshot);
        }
    }

    assert_eq!(ordinal, 54);
    assert_eq!(active_public_outcomes, [23, 25, 35, 37]);
    assert_eq!(state.transition_count, 54);
    assert_eq!(state.inactive_transition_count, 36);
    assert_eq!(
        state.predecessor_supply_temperature_saturation_assignment_count,
        18,
    );
    assert_eq!(
        state.predecessor_supply_temperature_saturation_mixed_air_limit_count,
        18,
    );
    assert_eq!(state.supply_humidity_ratio_assignment_count, 18);
    assert_eq!(state.source_site_execution_count, 72);
    assert_eq!(state.cp415_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 18);
    assert_eq!(state.cp415_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.cp415_supply_temperature_state_owner_count, 51);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 51);
    assert_eq!(
        state.cp416_psychrometric_supply_humidity_ratio_state_owner_count,
        18,
    );
    for count in active_site_counters(&state) {
        assert_eq!(count, 18);
    }
    assert_eq!(state.predecessor_route_counts, expected_predecessor);
    assert_eq!(
        state.predecessor_guard_false_fallthrough_route_counts,
        expected_guard_false,
    );
    assert_eq!(
        state.predecessor_guard_body_entry_route_counts,
        expected_guard_body,
    );
    assert_eq!(
        state.predecessor_supply_temperature_saturation_assignment_route_counts,
        expected_saturation_assignment,
    );
    assert_eq!(
        state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        expected_mixed_air_limit,
    );
    assert_eq!(
        state.supply_humidity_ratio_assignment_route_counts,
        expected_humidity_assignment,
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| snapshot.resulting_supply_humidity_ratio.is_some())
            .count(),
        36,
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| snapshot.resulting_supply_enthalpy_j_per_kg.is_some())
            .count(),
        41,
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| snapshot.resulting_supply_temperature_c.is_some())
            .count(),
        51,
    );
}

#[test]
fn public_route_transition_preserves_inverse_edge_bit_semantics() {
    let extreme_temperature = -2_000.0;
    let dry_air_enthalpy = 1.004_84e3 * extreme_temperature;
    let pole_c = -2_500_940.0 / 1_858.95;
    let enthalpy_nan = f64::from_bits(0x7ff8_0000_0000_0416);
    let temperature_nan = f64::from_bits(0x7ff8_0000_0000_4160);
    let cases = [
        (0.0, extreme_temperature),
        (dry_air_enthalpy - 1.0, extreme_temperature),
        (dry_air_enthalpy, extreme_temperature),
        (0.0, pole_c),
        (enthalpy_nan, 0.0),
        (f64::INFINITY, 0.0),
        (f64::NEG_INFINITY, 0.0),
        (0.0, temperature_nan),
        (enthalpy_nan, f64::INFINITY),
        (0.0, f64::NEG_INFINITY),
    ];
    let results = cases.map(|(enthalpy, mixed_air_temperature)| {
        let predecessor = active_fixture_with_operands(enthalpy, mixed_air_temperature);
        let temperature = predecessor
            .resulting_supply_temperature_c
            .expect("temperature");
        let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP416");
        assert_snapshot_matches_predecessor(snapshot, predecessor, true);
        assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact(snapshot));
        let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
        (temperature, expected)
    });

    assert_eq!(results[0].1.to_bits(), 1.0e-5f64.to_bits());
    assert!(results[1].1 > 0.0 && results[1].1 < 1.0e-5);
    assert_eq!(results[2].1.to_bits(), (-0.0f64).to_bits());
    assert_eq!(results[3].1, f64::INFINITY);
    assert!(results[4].1.is_nan() && results[7].1.is_nan());
    assert_eq!(results[5].1, f64::INFINITY);
    assert_eq!(results[6].1.to_bits(), 1.0e-5f64.to_bits());
    assert!(results[8].0.is_infinite() && results[9].0.is_infinite());
}

#[test]
fn inactive_routes_do_not_read_or_evaluate_operands() {
    let route = all_routes()
        .into_iter()
        .find(|route| !route.active)
        .expect("inactive route");
    let cp412 = predecessor_for_outcome(route, 1, false);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412).expect("CP413");
    let cp414 = advance_cp414(&mut Cp414State::new(cp413.system), cp413, f64::NAN).expect("CP414");
    let cp415 = advance_cp415(&mut Cp415State::new(cp414.system), cp414, None).expect("CP415");
    let snapshot = advance(&mut State::new(cp415.system), cp415).expect("inactive CP416");

    assert!(!snapshot.cp415_retained_supply_temperature_owned_read);
    assert!(!snapshot.supply_temperature_for_humidity_ratio_inversion_read);
    assert!(snapshot.supply_temperature_c.is_none());
    assert!(!snapshot.cp415_retained_supply_enthalpy_owned_read);
    assert!(!snapshot.supply_enthalpy_for_humidity_ratio_inversion_read);
    assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.psychrometric_supply_humidity_ratio_evaluated);
    assert!(snapshot.psychrometric_supply_humidity_ratio.is_none());
    assert!(!snapshot.supply_humidity_ratio_assignment_performed);
    assert!(snapshot.assigned_supply_humidity_ratio.is_none());
    assert!(option_bits_equal(
        snapshot.predecessor_cp415_resulting_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ));
}

#[test]
fn snapshot_corruption_is_rejected() {
    let predecessor = active_fixture();
    let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("active CP416");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact(snapshot));

    let mut value_corrupted = snapshot;
    value_corrupted.assigned_supply_humidity_ratio = Some(f64::from_bits(
        snapshot
            .assigned_supply_humidity_ratio
            .expect("assigned humidity ratio")
            .to_bits()
            ^ 1,
    ));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact(value_corrupted));

    let mut owner_corrupted = snapshot;
    owner_corrupted.cp415_retained_supply_enthalpy_owned_read = false;
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact(owner_corrupted));
}

#[test]
fn active_counter_overflow_is_transactional() {
    let predecessor = active_fixture();
    let route = predecessor_route(predecessor).expect("active route");
    #[rustfmt::skip]
    let setters: &[fn(&mut State, usize)] = &[
        |state, _| state.transition_count = usize::MAX,
        |state, index| state.predecessor_route_counts[index] = usize::MAX,
        |state, index| state.predecessor_guard_body_entry_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_temperature_saturation_assignment_count = usize::MAX,
        |state, index| state.predecessor_supply_temperature_saturation_assignment_route_counts[index] = usize::MAX,
        |state, _| state.predecessor_supply_temperature_saturation_mixed_air_limit_count = usize::MAX,
        |state, index| state.predecessor_supply_temperature_mixed_air_limit_route_counts[index] = usize::MAX,
        |state, _| state.supply_humidity_ratio_assignment_count = usize::MAX,
        |state, index| state.supply_humidity_ratio_assignment_route_counts[index] = usize::MAX,
        |state, _| state.source_site_execution_count = usize::MAX,
        |state, _| state.cp415_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state, _| state.cp415_supply_enthalpy_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state, _| state.cp415_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        |state, _| state.cp416_psychrometric_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state, _| state.cp415_retained_supply_temperature_owned_read_count = usize::MAX,
        |state, _| state.supply_temperature_for_humidity_ratio_inversion_read_count = usize::MAX,
        |state, _| state.cp415_retained_supply_enthalpy_owned_read_count = usize::MAX,
        |state, _| state.supply_enthalpy_for_humidity_ratio_inversion_read_count = usize::MAX,
        |state, _| state.psychrometric_supply_humidity_ratio_evaluation_count = usize::MAX,
        |state, _| state.supply_humidity_ratio_assignment_write_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(predecessor.system);
        set_overflow(&mut state, route.logical_index);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn inactive_counter_overflow_is_transactional() {
    let predecessor = inactive_fixture();
    let route = predecessor_route(predecessor).expect("guard-false route");
    let setters: &[fn(&mut State, usize)] = &[
        |state, index| state.predecessor_guard_false_fallthrough_route_counts[index] = usize::MAX,
        |state, _| state.inactive_transition_count = usize::MAX,
        |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(predecessor.system);
        set_overflow(&mut state, route.logical_index);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

fn assert_snapshot_matches_predecessor(snapshot: Snapshot, predecessor: Predecessor, active: bool) {
    assert_eq!(
        snapshot
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed,
        active,
    );
    assert!(option_bits_equal(
        snapshot.predecessor_cp415_resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ));
    assert!(option_bits_equal(
        snapshot.predecessor_cp415_resulting_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    ));
    if active {
        let temperature = predecessor
            .resulting_supply_temperature_c
            .expect("active temperature owner");
        let enthalpy = predecessor
            .resulting_supply_enthalpy_j_per_kg
            .expect("active enthalpy owner");
        let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
        assert_eq!(
            snapshot.supply_temperature_c.map(f64::to_bits),
            Some(temperature.to_bits()),
        );
        assert_eq!(
            snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
            Some(enthalpy.to_bits()),
        );
        for value in [
            snapshot.psychrometric_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
        }
    } else {
        assert!(option_bits_equal(
            snapshot.predecessor_cp415_resulting_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ));
    }
}

fn active_fixture() -> Predecessor {
    active_fixture_with_operands(31_000.0, 17.0)
}

fn active_fixture_with_operands(enthalpy: f64, mixed_air_temperature_c: f64) -> Predecessor {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active public route");
    let cp412 = predecessor_with_enthalpy(route, 1, 0.03, 18.0, 101_325.0, enthalpy);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412).expect("CP413");
    let cp414 = advance_cp414(&mut Cp414State::new(cp413.system), cp413, 101_325.0).expect("CP414");
    let owner = matching_mixed_air_owner(cp414, mixed_air_temperature_c);
    advance_cp415(&mut Cp415State::new(cp414.system), cp414, Some(owner)).expect("CP415")
}

fn inactive_fixture() -> Predecessor {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("guard-false public route");
    let cp412 = predecessor_for_outcome(route, 1, false);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412).expect("CP413");
    let cp414 = advance_cp414(&mut Cp414State::new(cp413.system), cp413, 91_325.0).expect("CP414");
    advance_cp415(&mut Cp415State::new(cp414.system), cp414, None).expect("CP415")
}

fn matching_mixed_air_owner(
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot,
    mixed_air_temperature_c: f64,
) -> MixedAirOwner {
    let mixed_predecessor = mixed_air_predecessor(MixedAirRoute::CoolingFallthrough);
    let mut owner = advance_cooling_mixed_air_call_state(
        &mut MixedAirState::new(mixed_predecessor.system),
        mixed_predecessor,
        Some(mixed_air_active_input(0.25)),
    );
    owner.system = predecessor.system;
    owner.parent_call_ordinal = predecessor.parent_call_ordinal;
    owner.controlled_zone = predecessor.controlled_zone;
    owner.mixed_air_temperature_c = Some(mixed_air_temperature_c);
    owner
}

fn active_site_counters(state: &State) -> [usize; 6] {
    [
        state.cp415_retained_supply_temperature_owned_read_count,
        state.supply_temperature_for_humidity_ratio_inversion_read_count,
        state.cp415_retained_supply_enthalpy_owned_read_count,
        state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        state.psychrometric_supply_humidity_ratio_evaluation_count,
        state.supply_humidity_ratio_assignment_write_count,
    ]
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
