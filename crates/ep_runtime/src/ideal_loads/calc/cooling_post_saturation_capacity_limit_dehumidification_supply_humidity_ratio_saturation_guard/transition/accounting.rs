//! Transactional CP413 route, owner, and conditional-site accounting.

use super::State;
use super::routes::{
    RetainedRoute, logical_route_index, predecessor_has_supply_enthalpy,
    predecessor_has_supply_humidity_ratio, predecessor_has_supply_temperature,
};

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    let index = logical_route_index(route);
    if state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
    {
        return false;
    }
    let owner_pairs = [
        (
            predecessor_has_supply_humidity_ratio(route),
            state.cp412_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor_has_supply_enthalpy(route.predecessor_index),
            state.cp412_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor_has_supply_temperature(route.predecessor_index),
            state.cp412_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    if owner_pairs.into_iter().any(|(present, owners, preserved)| {
        present && (owners.checked_add(1).is_none() || preserved.checked_add(1).is_none())
    }) {
        return false;
    }
    if !route.active {
        return state.inactive_transition_count.checked_add(1).is_some();
    }
    let common = [
        state.saturation_supply_humidity_ratio_guard_evaluation_count,
        state.cp412_saturation_supply_humidity_ratio_owned_read_count,
        state.saturation_supply_humidity_ratio_for_guard_read_count,
        state.cp411_original_supply_humidity_ratio_owned_read_count,
        state.cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count,
        state.original_supply_humidity_ratio_for_guard_read_count,
        state.saturation_original_supply_humidity_ratio_comparison_count,
    ];
    common.into_iter().all(|count| count.checked_add(1).is_some())
        && state
            .source_site_execution_count
            .checked_add(3 + usize::from(route.body_entered))
            .is_some()
        && if route.body_entered {
            state.guard_body_entry_route_counts[index]
                .checked_add(1)
                .is_some()
                && state
                    .saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count
                    .checked_add(1)
                    .is_some()
                && state
                    .saturation_supply_humidity_ratio_guard_body_entry_count
                    .checked_add(1)
                    .is_some()
        } else {
            state.guard_false_fallthrough_route_counts[index]
                .checked_add(1)
                .is_some()
                && state
                    .saturation_supply_humidity_ratio_guard_false_fallthrough_count
                    .checked_add(1)
                    .is_some()
        }
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    let index = logical_route_index(route);
    state.predecessor_route_counts[index] += 1;
    if predecessor_has_supply_humidity_ratio(route) {
        state.cp412_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor_has_supply_enthalpy(route.predecessor_index) {
        state.cp412_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor_has_supply_temperature(route.predecessor_index) {
        state.cp412_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if !route.active {
        state.inactive_transition_count += 1;
        return;
    }
    state.saturation_supply_humidity_ratio_guard_evaluation_count += 1;
    state.cp412_saturation_supply_humidity_ratio_owned_read_count += 1;
    state.saturation_supply_humidity_ratio_for_guard_read_count += 1;
    state.cp411_original_supply_humidity_ratio_owned_read_count += 1;
    state.cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count += 1;
    state.original_supply_humidity_ratio_for_guard_read_count += 1;
    state.saturation_original_supply_humidity_ratio_comparison_count += 1;
    state.source_site_execution_count += 3 + usize::from(route.body_entered);
    if route.body_entered {
        state.guard_body_entry_route_counts[index] += 1;
        state.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count += 1;
        state.saturation_supply_humidity_ratio_guard_body_entry_count += 1;
    } else {
        state.guard_false_fallthrough_route_counts[index] += 1;
        state.saturation_supply_humidity_ratio_guard_false_fallthrough_count += 1;
    }
}
