//! CP415 boundary, route, IEEE, corruption, and overflow tests.

use super::transition::{predecessor_route, source_shaped_two_argument_minimum};
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, active_input as mixed_air_active_input,
    predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::tests::{
    all_routes, predecessor_for_outcome,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingMixedAirCallRuntimeState as MixedAirState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as Cp413State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as Cp414State,
    advance_cooling_mixed_air_call_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance_cp413,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state as advance_cp414,
};
use crate::ideal_loads::PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner;

#[test]
fn cp415_boundary_and_four_sites_are_exact() {
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2319",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2320",
    );
    assert_eq!(
        super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-temperature-for-minimum",
            "read-purchased-air-mixed-air-temperature-for-minimum",
            "apply-source-shaped-two-argument-minimum",
            "assign-purchased-air-supply-temperature",
        ],
    );
}

#[test]
fn exhaustive_54_outcome_transition_and_five_route_partitions_are_exact() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    let system = ep_model::IdealLoadsAirSystemId(412);
    let mut cp413_state = Cp413State::new(system);
    let mut cp414_state = Cp414State::new(system);
    let mut state = State::new(system);
    let mut expected_predecessor = [0usize; 36];
    let mut expected_guard_false = [0usize; 36];
    let mut expected_guard_body = [0usize; 36];
    let mut expected_predecessor_assignment = [0usize; 36];
    let mut expected_limit = [0usize; 36];
    let mut active_public_outcomes = Vec::new();
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
            let cp415_route = predecessor_route(cp414).expect("valid CP415 route");
            let owner = cp415_route
                .active
                .then(|| matching_mixed_air_owner(cp414, 17.0));
            let snapshot = advance(&mut state, cp414, owner).expect("valid CP415 outcome");
            let index = cp415_route.logical_index;
            expected_predecessor[index] += 1;
            if cp415_route.predecessor_guard_false_fallthrough {
                expected_guard_false[index] += 1;
            }
            if cp415_route.predecessor_guard_body_entered {
                expected_guard_body[index] += 1;
            }
            if cp415_route.predecessor_assignment_executed {
                expected_predecessor_assignment[index] += 1;
            }
            if cp415_route.active {
                expected_limit[index] += 1;
                if matches!(route.predecessor_index, 0..=8 | 20 | 24) {
                    active_public_outcomes.push(conceptual_index);
                }
            }

            assert_eq!(
                snapshot
                    .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed,
                body_entered,
            );
            assert!(option_bits_equal(
                snapshot.predecessor_cp414_resulting_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ));
            assert!(option_bits_equal(
                snapshot.predecessor_cp414_resulting_supply_enthalpy_j_per_kg,
                snapshot.resulting_supply_enthalpy_j_per_kg,
            ));
            if body_entered {
                let left = cp414
                    .resulting_supply_temperature_c
                    .expect("active supply-temperature owner");
                let expected = source_shaped_two_argument_minimum(left, 17.0);
                assert_eq!(
                    snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                    Some(expected.to_bits()),
                );
            } else {
                assert!(option_bits_equal(
                    snapshot.predecessor_cp414_resulting_supply_temperature_c,
                    snapshot.resulting_supply_temperature_c,
                ));
                assert!(snapshot.mixed_air_temperature_c.is_none());
            }
            assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact(snapshot));
        }
    }

    assert_eq!(ordinal, 54);
    assert_eq!(active_public_outcomes, [23, 25, 35, 37]);
    assert_eq!(state.transition_count, 54);
    assert_eq!(state.inactive_transition_count, 36);
    assert_eq!(
        state.predecessor_supply_temperature_saturation_assignment_count,
        18
    );
    assert_eq!(
        state.supply_temperature_saturation_mixed_air_limit_count,
        18
    );
    assert_eq!(state.source_site_execution_count, 72);
    assert_eq!(state.cp414_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 36);
    assert_eq!(state.cp414_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.cp414_supply_temperature_state_owner_count, 51);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 33);
    assert_eq!(
        state.cp415_mixed_air_limited_supply_temperature_state_owner_count,
        18
    );
    for count in [
        state.cp414_retained_supply_temperature_owned_read_count,
        state.supply_temperature_for_minimum_read_count,
        state.cp329_retained_mixed_air_temperature_owned_read_count,
        state.mixed_air_temperature_for_minimum_read_count,
        state.source_shaped_two_argument_minimum_evaluation_count,
        state.supply_temperature_assignment_write_count,
    ] {
        assert_eq!(count, 18);
    }
    assert_eq!(state.predecessor_route_counts, expected_predecessor);
    assert_eq!(
        state.predecessor_guard_false_fallthrough_route_counts,
        expected_guard_false,
    );
    assert_eq!(
        state.predecessor_guard_body_entry_route_counts,
        expected_guard_body
    );
    assert_eq!(
        state.predecessor_supply_temperature_saturation_assignment_route_counts,
        expected_predecessor_assignment,
    );
    assert_eq!(
        state.supply_temperature_mixed_air_limit_route_counts,
        expected_limit
    );
}

#[test]
fn source_minimum_selects_right_for_ties_and_unordered_values() {
    assert_eq!(
        source_shaped_two_argument_minimum(-0.0, 0.0).to_bits(),
        0.0f64.to_bits(),
    );
    assert_eq!(
        source_shaped_two_argument_minimum(0.0, -0.0).to_bits(),
        (-0.0f64).to_bits(),
    );
    let left_nan = f64::from_bits(0x7ff8_0000_0000_4151);
    let right = f64::from_bits(0x4004_0000_0000_0000);
    assert_eq!(
        source_shaped_two_argument_minimum(left_nan, right).to_bits(),
        right.to_bits(),
    );
    let right_nan = f64::from_bits(0x7ff8_0000_0000_4152);
    assert_eq!(
        source_shaped_two_argument_minimum(2.5, right_nan).to_bits(),
        right_nan.to_bits(),
    );
}

#[test]
fn inactive_routes_do_not_read_mixed_air_owner() {
    let route = all_routes()
        .into_iter()
        .find(|route| !route.active)
        .expect("inactive route");
    let cp412 = predecessor_for_outcome(route, 1, false);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412).expect("CP413");
    let cp414 =
        advance_cp414(&mut Cp414State::new(cp413.system), cp413, f64::NAN).expect("inactive CP414");
    let snapshot = advance(&mut State::new(cp414.system), cp414, None).expect("inactive CP415");
    assert!(!snapshot.cp414_retained_supply_temperature_owned_read);
    assert!(!snapshot.cp329_retained_mixed_air_temperature_owned_read);
    assert!(snapshot.mixed_air_temperature_c.is_none());
}

#[test]
fn snapshot_corruption_is_rejected() {
    let (cp414, owner) = active_fixture();
    let snapshot =
        advance(&mut State::new(cp414.system), cp414, Some(owner)).expect("active CP415");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact(snapshot));

    let mut corrupted = snapshot;
    corrupted.minimum_supply_temperature_c = Some(f64::from_bits(
        snapshot
            .minimum_supply_temperature_c
            .expect("minimum")
            .to_bits()
            ^ 1,
    ));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact(corrupted));

    let mut owner_corrupted = snapshot;
    owner_corrupted.cp329_retained_mixed_air_temperature_owned_read = false;
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact(owner_corrupted));
}

#[test]
fn counter_overflow_is_transactional() {
    let (cp414, owner) = active_fixture();
    let route = predecessor_route(cp414).expect("active route");
    let mut state = State::new(cp414.system);
    state.supply_temperature_mixed_air_limit_route_counts[route.logical_index] = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, cp414, Some(owner)).is_none());
    assert_eq!(state, before);
}

fn active_fixture() -> (
    crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot,
    MixedAirOwner,
){
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active public route");
    let cp412 = predecessor_for_outcome(route, 1, true);
    let cp413 = advance_cp413(&mut Cp413State::new(cp412.system), cp412).expect("CP413");
    let cp414 = advance_cp414(&mut Cp414State::new(cp413.system), cp413, 91_325.0).expect("CP414");
    let owner = matching_mixed_air_owner(cp414, 17.0);
    (cp414, owner)
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

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
