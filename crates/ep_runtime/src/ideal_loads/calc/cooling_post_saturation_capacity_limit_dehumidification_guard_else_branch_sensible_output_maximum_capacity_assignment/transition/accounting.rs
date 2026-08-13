//! Transactional CP422 route, owner, and assignment-site accounting.

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
            state.cp421_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.cp421_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.cp421_supply_temperature_state_owner_count,
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
    if !route.assignment_executed {
        return state
            .predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(1)
            .is_some()
            && state
                .predecessor_guard_false_fallthrough_count
                .checked_add(1)
                .is_some();
    }
    [
        state.cooling_sensible_output_maximum_capacity_assignment_count,
        state.cp421_retained_maximum_total_cooling_capacity_owned_read_count,
        state.maximum_total_cooling_capacity_for_sensible_output_assignment_read_count,
        state.cooling_sensible_output_maximum_capacity_assignment_write_count,
    ]
    .into_iter()
    .all(|count| count.checked_add(1).is_some())
        && state
            .cooling_sensible_output_maximum_capacity_assignment_route_counts[index]
            .checked_add(1)
            .is_some()
        && state.source_site_execution_count.checked_add(2).is_some()
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp421_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp421_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp421_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if !route.active {
        state.inactive_transition_count += 1;
    } else if !route.assignment_executed {
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
        state.predecessor_guard_false_fallthrough_count += 1;
    } else {
        state.cooling_sensible_output_maximum_capacity_assignment_route_counts[index] += 1;
        state.cooling_sensible_output_maximum_capacity_assignment_count += 1;
        state.source_site_execution_count += 2;
        state.cp421_retained_maximum_total_cooling_capacity_owned_read_count += 1;
        state.maximum_total_cooling_capacity_for_sensible_output_assignment_read_count += 1;
        state.cooling_sensible_output_maximum_capacity_assignment_write_count += 1;
    }
}
