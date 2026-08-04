//! CP413 checked-counter transactional tests.

use super::{all_routes, predecessor_for_outcome, predecessor_for_route};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::transition::{
    routes::{RetainedRoute, logical_route_index},
    test_next_transition_fits,
};

#[test]
fn active_false_counter_overflow_is_transactional() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active split route");
    let index = logical_route_index(route);
    type Mutation = fn(&mut State, usize);
    let mutations: &[Mutation] = &[
        |state, _| state.transition_count = usize::MAX,
        |state, index| state.predecessor_route_counts[index] = usize::MAX,
        |state, _| state.source_site_execution_count = usize::MAX,
        |state, _| state.cp412_supply_humidity_ratio_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
        |state, _| state.cp412_supply_enthalpy_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |state, _| state.cp412_supply_temperature_state_owner_count = usize::MAX,
        |state, _| state.unchanged_supply_temperature_preservation_count = usize::MAX,
        |state, _| state.saturation_supply_humidity_ratio_guard_evaluation_count = usize::MAX,
        |state, _| state.cp412_saturation_supply_humidity_ratio_owned_read_count = usize::MAX,
        |state, _| state.saturation_supply_humidity_ratio_for_guard_read_count = usize::MAX,
        |state, _| state.cp411_original_supply_humidity_ratio_owned_read_count = usize::MAX,
        |state, _| {
            state.cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count =
                usize::MAX
        },
        |state, _| state.original_supply_humidity_ratio_for_guard_read_count = usize::MAX,
        |state, _| state.saturation_original_supply_humidity_ratio_comparison_count = usize::MAX,
        |state, index| state.guard_false_fallthrough_route_counts[index] = usize::MAX,
        |state, _| {
            state.saturation_supply_humidity_ratio_guard_false_fallthrough_count = usize::MAX
        },
    ];
    for mutate in mutations {
        let predecessor = predecessor_for_outcome(route, 1, false);
        let mut state = State::new(predecessor.system);
        mutate(&mut state, index);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, route));
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn active_body_and_inactive_overflow_are_transactional() {
    let base = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 24 && route.predecessor_guard_false_fallthrough)
        .expect("active public route");
    let body_route = RetainedRoute {
        body_entered: true,
        ..base
    };
    let index = logical_route_index(body_route);
    type Mutation = fn(&mut State, usize);
    for mutate in [
        (|state: &mut State, index: usize| {
            state.guard_body_entry_route_counts[index] = usize::MAX
        })
            as Mutation,
        |state: &mut State, _| {
            state.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count = usize::MAX
        },
        |state: &mut State, _| {
            state.saturation_supply_humidity_ratio_guard_body_entry_count = usize::MAX
        },
    ] {
        let predecessor = predecessor_for_outcome(base, 1, true);
        let mut state = State::new(predecessor.system);
        mutate(&mut state, index);
        let before = state.clone();
        assert!(!test_next_transition_fits(&state, body_route));
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }

    let inactive = all_routes()[0];
    let predecessor = predecessor_for_route(inactive, 1);
    let mut state = State::new(predecessor.system);
    state.inactive_transition_count = usize::MAX;
    let before = state.clone();
    assert!(!test_next_transition_fits(&state, inactive));
    assert!(advance(&mut state, predecessor).is_none());
    assert_eq!(state, before);
}
