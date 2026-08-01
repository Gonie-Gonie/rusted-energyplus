//! CP380 thirteen-route, short-circuit, and selector-owner tests.

use ep_model::IdealLoadsLimit;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_guard_state as advance,
    cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release,
};
use super::{active_input, predecessor_for_route};

#[test]
fn cp380_retains_thirteen_conceptual_routes() {
    for predecessor_route in 0..8 {
        let limits: &[IdealLoadsLimit] = if predecessor_route < 3 {
            &[IdealLoadsLimit::NoLimit]
        } else {
            &[IdealLoadsLimit::LimitCapacity, IdealLoadsLimit::NoLimit]
        };
        for &limit in limits {
            let predecessor = predecessor_for_route(predecessor_route, 1);
            let mut state = State::new(predecessor.system);
            let input = (predecessor_route >= 3).then(|| active_input(limit).unwrap());
            let snapshot = advance(&mut state, predecessor, input).expect("valid CP380 route");
            assert_eq!(
                state.latest_route,
                Some(expected_route(predecessor_route, limit))
            );
            assert_eq!(state.transition_count, 1);
            assert_eq!(
                cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(
                    snapshot,
                ),
                predecessor_route <= 4,
            );
        }
    }
}

#[test]
fn cp380_four_selectors_obey_three_five_four_source_site_counts() {
    for (limit, expected_sites, expected_second, expected_body, expected_false) in [
        (IdealLoadsLimit::LimitCapacity, 15, 0, 5, 0),
        (IdealLoadsLimit::LimitFlowRateAndCapacity, 25, 5, 5, 0),
        (IdealLoadsLimit::NoLimit, 20, 5, 0, 5),
        (IdealLoadsLimit::LimitFlowRate, 20, 5, 0, 5),
    ] {
        let mut state = State::new(predecessor_for_route(0, 1).system);
        for route in 0..8 {
            let predecessor = predecessor_for_route(route, route + 1);
            let input = (route >= 3).then(|| active_input(limit).unwrap());
            advance(&mut state, predecessor, input).expect("valid inherited route");
        }
        assert_eq!(state.transition_count, 8);
        assert_eq!(state.capacity_limit_guard_evaluation_count, 5);
        assert_eq!(state.configured_cooling_limit_owned_read_count, 5);
        assert_eq!(
            state.cp337_same_call_selector_lineage_corroboration_count,
            5
        );
        assert_eq!(state.first_cooling_limit_read_count, 5);
        assert_eq!(state.cooling_limit_capacity_comparison_count, 5);
        assert_eq!(state.second_cooling_limit_read_count, expected_second);
        assert_eq!(
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
            expected_second,
        );
        assert_eq!(state.capacity_limit_body_entry_count, expected_body);
        assert_eq!(state.cooling_limit_rejected_count, expected_false);
        assert_eq!(state.active_guard_false_fallthrough_count, expected_false);
        assert_eq!(state.source_site_execution_count, expected_sites);
        assert_active_route_parity(&state);
    }
}

#[test]
fn cp380_short_circuits_second_selector_read_and_nulls_all_skip_fields() {
    let predecessor = predecessor_for_route(4, 1);
    let mut state = State::new(predecessor.system);
    let capacity = advance(
        &mut state,
        predecessor,
        active_input(IdealLoadsLimit::LimitCapacity),
    )
    .expect("capacity route");
    assert_eq!(
        capacity.first_cooling_limit,
        Some(IdealLoadsLimit::LimitCapacity)
    );
    assert_eq!(capacity.cooling_limit_capacity, Some(true));
    assert!(!capacity.second_cooling_limit_read);
    assert!(capacity.second_cooling_limit.is_none());
    assert!(capacity.cooling_limit_flow_rate_and_capacity.is_none());
    assert!(capacity.capacity_limit_body_entered);
    assert_eq!(state.source_site_execution_count, 3);

    for route in 0..3 {
        let predecessor = predecessor_for_route(route, 1);
        let mut state = State::new(predecessor.system);
        let skipped = advance(&mut state, predecessor, None).expect("skip route");
        assert!(!skipped.capacity_limit_guard_evaluated);
        assert!(!skipped.configured_cooling_limit_owned_read);
        assert!(!skipped.cp337_same_call_selector_lineage_corroborated);
        assert!(!skipped.first_cooling_limit_read);
        assert!(skipped.first_cooling_limit.is_none());
        assert!(!skipped.second_cooling_limit_read);
        assert!(skipped.second_cooling_limit.is_none());
        assert!(skipped.cooling_limit_condition_satisfied.is_none());
        assert!(!skipped.capacity_limit_body_entered);
        assert!(!skipped.active_guard_false_fallthrough);
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn cp380_rejects_identity_placeholder_and_uncorroborated_input_transactionally() {
    let active_predecessor = predecessor_for_route(4, 1);

    let mut wrong_system = State::new(ep_model::IdealLoadsAirSystemId(99));
    let before = wrong_system.clone();
    assert!(
        advance(
            &mut wrong_system,
            active_predecessor,
            active_input(IdealLoadsLimit::NoLimit),
        )
        .is_none()
    );
    assert_eq!(wrong_system, before);

    let skipped_predecessor = predecessor_for_route(0, 1);
    let mut skipped = State::new(skipped_predecessor.system);
    let before = skipped.clone();
    assert!(
        advance(
            &mut skipped,
            skipped_predecessor,
            active_input(IdealLoadsLimit::NoLimit),
        )
        .is_none()
    );
    assert_eq!(skipped, before);

    let mut uncorroborated = State::new(active_predecessor.system);
    let before = uncorroborated.clone();
    assert!(
        advance(
            &mut uncorroborated,
            active_predecessor,
            Some(ActiveInput {
                cooling_limit: IdealLoadsLimit::NoLimit,
                cp337_same_call_selector_lineage_corroborated: false,
            }),
        )
        .is_none()
    );
    assert_eq!(uncorroborated, before);
}

#[test]
fn cp380_pure_transition_does_not_read_any_numerical_predecessor_field() {
    let mut predecessor = predecessor_for_route(4, 1);
    predecessor.predecessor_resulting_supply_humidity_ratio = Some(f64::NAN);
    predecessor.supply_temperature_c = Some(f64::INFINITY);
    predecessor.supply_humidity_ratio = Some(f64::NEG_INFINITY);
    predecessor.psychrometric_supply_enthalpy_j_per_kg = Some(f64::NAN);
    predecessor.assigned_supply_enthalpy_j_per_kg = Some(f64::INFINITY);
    predecessor.resulting_supply_enthalpy_j_per_kg = Some(f64::NEG_INFINITY);
    let mut state = State::new(predecessor.system);
    let snapshot = advance(
        &mut state,
        predecessor,
        active_input(IdealLoadsLimit::NoLimit),
    )
    .expect("CP380 owns only the typed selector");
    assert_eq!(snapshot.first_cooling_limit, Some(IdealLoadsLimit::NoLimit));
    assert!(snapshot.active_guard_false_fallthrough);
}

fn expected_route(predecessor: usize, limit: IdealLoadsLimit) -> Route {
    let body = matches!(
        limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    );
    match (predecessor, body) {
        (0, _) => Route::UnitOff,
        (1, _) => Route::NonCooling,
        (2, _) => Route::PositiveGuardFalseFallthrough,
        (3, true) => Route::HeatingAvailabilityGuardFalseFallthroughBodyEntered,
        (3, false) => Route::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough,
        (4, true) => Route::HumidificationControlGuardFalseFallthroughBodyEntered,
        (4, false) => Route::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough,
        (5, true) => Route::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered,
        (5, false) => {
            Route::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough
        }
        (6, true) => Route::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered,
        (6, false) => {
            Route::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough
        }
        (7, true) => Route::DehumidificationControlGuardFalseFallthroughBodyEntered,
        (7, false) => Route::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough,
        _ => unreachable!(),
    }
}

fn assert_active_route_parity(state: &State) {
    for (inherited, body, guard_false) in [
        (
            state.heating_availability_guard_false_fallthrough_count,
            state.heating_availability_guard_false_fallthrough_body_entry_count,
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        ),
        (
            state.humidification_control_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_body_entry_count,
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        ),
        (
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        ),
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            state.dehumidification_control_none_maximum_assignment_body_entry_count,
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            state.dehumidification_control_guard_false_fallthrough_body_entry_count,
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        ),
    ] {
        assert_eq!(body + guard_false, inherited);
    }
}
