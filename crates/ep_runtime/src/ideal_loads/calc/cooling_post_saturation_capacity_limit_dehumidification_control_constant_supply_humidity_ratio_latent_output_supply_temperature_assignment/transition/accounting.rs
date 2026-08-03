//! Checked CP407 route, owner, preservation, and four-site accounting.

use super::State;
use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_temperature,
    resulting_has_supply_humidity_ratio,
};
use crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER as SOURCE_ORDER;

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
        (!route.assignment_executed, state.inactive_transition_count),
        (guard_false, state.predecessor_guard_false_fallthrough_count),
        (
            guard_false,
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
        (route.assignment_executed, state.predecessor_else_branch_entry_count),
        (
            route.assignment_executed,
            state.predecessor_else_branch_entry_route_counts[index],
        ),
        (
            route.assignment_executed,
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count,
        ),
        (
            route.assignment_executed,
            state.supply_temperature_assignment_route_counts[index],
        ),
        (
            predecessor_has_supply_temperature(index),
            state.cp406_preexisting_supply_temperature_state_owner_count,
        ),
        (
            resulting_has_supply_humidity_ratio(route),
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor_has_supply_enthalpy(index),
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor_has_supply_temperature(index) && !route.assignment_executed,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    if checked
        .into_iter()
        .any(|(used, count)| used && count.checked_add(1).is_none())
    {
        return false;
    }
    if !route.assignment_executed {
        return true;
    }
    [
        state.cp385_retained_supply_enthalpy_owned_read_count,
        state.cp406_same_call_supply_enthalpy_bit_corroboration_count,
        state.supply_enthalpy_for_dry_bulb_inversion_read_count,
        state.cp378_retained_supply_humidity_ratio_owned_read_count,
        state.supply_humidity_ratio_for_dry_bulb_inversion_read_count,
        state.psychrometric_supply_temperature_evaluation_count,
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
        state.cp406_preexisting_supply_temperature_state_owner_count += 1;
        if !route.assignment_executed {
            state.unchanged_supply_temperature_preservation_count += 1;
        }
    }
    if resulting_has_supply_humidity_ratio(route) {
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor_has_supply_enthalpy(index) {
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if !route.assignment_executed {
        state.inactive_transition_count += 1;
        return;
    }
    state.predecessor_else_branch_entry_count += 1;
    state.predecessor_else_branch_entry_route_counts[index] += 1;
    state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count += 1;
    state.supply_temperature_assignment_route_counts[index] += 1;
    state.source_site_execution_count += SOURCE_ORDER.len();
    state.cp385_retained_supply_enthalpy_owned_read_count += 1;
    state.cp406_same_call_supply_enthalpy_bit_corroboration_count += 1;
    state.supply_enthalpy_for_dry_bulb_inversion_read_count += 1;
    state.cp378_retained_supply_humidity_ratio_owned_read_count += 1;
    state.supply_humidity_ratio_for_dry_bulb_inversion_read_count += 1;
    state.psychrometric_supply_temperature_evaluation_count += 1;
    state.supply_temperature_assignment_write_count += 1;
}
