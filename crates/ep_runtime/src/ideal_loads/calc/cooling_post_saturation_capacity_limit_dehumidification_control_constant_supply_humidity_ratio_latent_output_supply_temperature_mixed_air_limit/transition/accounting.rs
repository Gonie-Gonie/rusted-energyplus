//! Checked CP408 route, owner, preservation, and four-site accounting.

use super::State;
use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature,
};
use crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER as SOURCE_ORDER;

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
    let guard_false = route.predecessor_guard_evaluated
        && !route.predecessor_maximum_capacity_assignment_executed;
    let checked = [
        (!route.active, state.inactive_transition_count),
        (guard_false, state.predecessor_guard_false_fallthrough_count),
        (guard_false, state.predecessor_guard_false_fallthrough_route_counts[index]),
        (
            route.predecessor_maximum_capacity_assignment_executed,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            route.predecessor_maximum_capacity_assignment_executed,
            state.predecessor_maximum_capacity_assignment_route_counts[index],
        ),
        (route.active, state.predecessor_else_branch_entry_count),
        (route.active, state.predecessor_else_branch_entry_route_counts[index]),
        (route.active, state.predecessor_supply_temperature_assignment_count),
        (route.active, state.predecessor_supply_temperature_assignment_route_counts[index]),
        (
            route.active,
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count,
        ),
        (route.active, state.supply_temperature_mixed_air_limit_route_counts[index]),
        (
            predecessor_has_supply_temperature(index),
            state.cp407_supply_temperature_state_owner_count,
        ),
        (
            predecessor_has_supply_humidity_ratio(route),
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor_has_supply_enthalpy(index),
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor_has_supply_temperature(index) && !route.active,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    if checked
        .into_iter()
        .any(|(used, count)| used && count.checked_add(1).is_none())
    {
        return false;
    }
    if !route.active {
        return true;
    }
    [
        state.cp407_retained_supply_temperature_owned_read_count,
        state.supply_temperature_for_minimum_read_count,
        state.cp329_retained_mixed_air_temperature_owned_read_count,
        state.mixed_air_temperature_for_minimum_read_count,
        state.source_shaped_two_argument_minimum_evaluation_count,
        state.supply_temperature_assignment_write_count,
    ]
    .into_iter()
    .all(|count| count.checked_add(1).is_some())
        && state
            .source_site_execution_count
            .checked_add(SOURCE_ORDER.len())
            .is_some()
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    let index = route.predecessor_index;
    let guard_false = route.predecessor_guard_evaluated
        && !route.predecessor_maximum_capacity_assignment_executed;
    state.transition_count += 1;
    state.predecessor_route_counts[index] += 1;
    if guard_false {
        state.predecessor_guard_false_fallthrough_count += 1;
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    }
    if route.predecessor_maximum_capacity_assignment_executed {
        state.predecessor_maximum_capacity_assignment_count += 1;
        state.predecessor_maximum_capacity_assignment_route_counts[index] += 1;
    }
    if predecessor_has_supply_temperature(index) {
        state.cp407_supply_temperature_state_owner_count += 1;
        if !route.active {
            state.unchanged_supply_temperature_preservation_count += 1;
        }
    }
    if predecessor_has_supply_humidity_ratio(route) {
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor_has_supply_enthalpy(index) {
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if !route.active {
        state.inactive_transition_count += 1;
        return;
    }
    state.predecessor_else_branch_entry_count += 1;
    state.predecessor_else_branch_entry_route_counts[index] += 1;
    state.predecessor_supply_temperature_assignment_count += 1;
    state.predecessor_supply_temperature_assignment_route_counts[index] += 1;
    state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count += 1;
    state.supply_temperature_mixed_air_limit_route_counts[index] += 1;
    state.source_site_execution_count += SOURCE_ORDER.len();
    state.cp407_retained_supply_temperature_owned_read_count += 1;
    state.supply_temperature_for_minimum_read_count += 1;
    state.cp329_retained_mixed_air_temperature_owned_read_count += 1;
    state.mixed_air_temperature_for_minimum_read_count += 1;
    state.source_shaped_two_argument_minimum_evaluation_count += 1;
    state.supply_temperature_assignment_write_count += 1;
}
