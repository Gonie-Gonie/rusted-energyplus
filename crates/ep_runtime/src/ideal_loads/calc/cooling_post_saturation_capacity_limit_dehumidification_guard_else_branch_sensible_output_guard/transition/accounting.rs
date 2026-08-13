//! Transactional CP421 route, owner, and conditional-site accounting.

use super::{Predecessor, Route, State};

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    let index = route.logical_index;
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
    {
        return false;
    }
    let owner_pairs = [
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.cp420_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.cp420_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.cp420_supply_temperature_state_owner_count,
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
        state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count,
        state.cp420_cooling_sensible_output_owned_read_count,
        state.cooling_sensible_output_read_count,
        state.cp321_maximum_total_cooling_capacity_owned_read_count,
        state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        state.maximum_total_cooling_capacity_read_count,
        state.cooling_sensible_output_maximum_total_cooling_capacity_comparison_count,
    ];
    common
        .into_iter()
        .all(|count| count.checked_add(1).is_some())
        && state
            .source_site_execution_count
            .checked_add(3 + usize::from(route.body_entered))
            .is_some()
        && if route.body_entered {
            state.adjustment_body_entry_route_counts[index]
                .checked_add(1)
                .is_some()
                && state
                    .cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count
                    .checked_add(1)
                    .is_some()
                && state
                    .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count
                    .checked_add(1)
                    .is_some()
        } else {
            state.guard_false_fallthrough_route_counts[index]
                .checked_add(1)
                .is_some()
                && state
                    .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count
                    .checked_add(1)
                    .is_some()
        }
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp420_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp420_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp420_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if !route.active {
        state.inactive_transition_count += 1;
        return;
    }
    state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count += 1;
    state.cp420_cooling_sensible_output_owned_read_count += 1;
    state.cooling_sensible_output_read_count += 1;
    state.cp321_maximum_total_cooling_capacity_owned_read_count += 1;
    state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count += 1;
    state.maximum_total_cooling_capacity_read_count += 1;
    state.cooling_sensible_output_maximum_total_cooling_capacity_comparison_count += 1;
    state.source_site_execution_count += 3 + usize::from(route.body_entered);
    if route.body_entered {
        state.adjustment_body_entry_route_counts[index] += 1;
        state.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count += 1;
        state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count += 1;
    } else {
        state.guard_false_fallthrough_route_counts[index] += 1;
        state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count += 1;
    }
}
