//! Checked CP411 route, preservation, owner, and source-site accounting.

use super::routes::{
    predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature, route_is_active, RetainedRoute,
};
use super::State;
use crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER as ORDER;

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    let index = route.predecessor_index;
    if index >= state.predecessor_route_counts.len()
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    let route_counts = [
        (
            route.predecessor_guard_false_fallthrough,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            route.predecessor_guard_false_fallthrough,
            state.predecessor_guard_false_fallthrough_route_counts[index],
        ),
        (
            route.predecessor_maximum_capacity_assignment_executed,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            route.predecessor_maximum_capacity_assignment_executed,
            state.predecessor_maximum_capacity_assignment_route_counts[index],
        ),
        (!route_is_active(route), state.inactive_transition_count),
        (
            route_is_active(route),
            state.supply_humidity_ratio_pre_saturation_original_assignment_count,
        ),
        (
            route_is_active(route),
            state.supply_humidity_ratio_pre_saturation_original_assignment_route_counts[index],
        ),
    ];
    if route_counts
        .into_iter()
        .any(|(used, count)| used && count.checked_add(1).is_none())
    {
        return false;
    }
    let owner_pairs = [
        (
            predecessor_has_supply_humidity_ratio(route),
            state.cp410_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor_has_supply_enthalpy(index),
            state.cp410_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor_has_supply_temperature(index),
            state.cp410_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    if owner_pairs.into_iter().any(|(present, owners, preserved)| {
        present && (owners.checked_add(1).is_none() || preserved.checked_add(1).is_none())
    }) {
        return false;
    }
    !route_is_active(route)
        || (state
            .source_site_execution_count
            .checked_add(ORDER.len())
            .is_some()
            && [
                state.cp410_retained_supply_humidity_ratio_owned_read_count,
                state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count,
                state.local_supply_humidity_ratio_original_assignment_write_count,
            ]
            .into_iter()
            .all(|count| count.checked_add(1).is_some()))
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    let index = route.predecessor_index;
    state.transition_count += 1;
    state.predecessor_route_counts[index] += 1;
    if route.predecessor_guard_false_fallthrough {
        state.predecessor_guard_false_fallthrough_count += 1;
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    }
    if route.predecessor_maximum_capacity_assignment_executed {
        state.predecessor_maximum_capacity_assignment_count += 1;
        state.predecessor_maximum_capacity_assignment_route_counts[index] += 1;
    }
    if predecessor_has_supply_humidity_ratio(route) {
        state.cp410_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor_has_supply_enthalpy(index) {
        state.cp410_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor_has_supply_temperature(index) {
        state.cp410_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if route_is_active(route) {
        state.supply_humidity_ratio_pre_saturation_original_assignment_count += 1;
        state.supply_humidity_ratio_pre_saturation_original_assignment_route_counts[index] += 1;
        state.source_site_execution_count += ORDER.len();
        state.cp410_retained_supply_humidity_ratio_owned_read_count += 1;
        state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count += 1;
        state.local_supply_humidity_ratio_original_assignment_write_count += 1;
    } else {
        state.inactive_transition_count += 1;
    }
}
