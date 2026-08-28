//! Transactional CP432 route, assignment, and owner accounting.

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
    let preserved = [
        (predecessor.resulting_supply_humidity_ratio.is_some(), state.cp431_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        (predecessor.resulting_supply_enthalpy_j_per_kg.is_some(), state.cp431_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        (predecessor.resulting_supply_temperature_c.is_some(), state.cp431_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
    ];
    if preserved.into_iter().any(|(present, owner, unchanged)| {
        present && (owner.checked_add(1).is_none() || unchanged.checked_add(1).is_none())
    }) {
        return false;
    }
    if !route.predecessor_heating_mode_guard_evaluated {
        return state.inactive_transition_count.checked_add(1).is_some();
    }
    if state
        .predecessor_heating_mode_guard_evaluation_count
        .checked_add(1)
        .is_none()
        || state.predecessor_heating_mode_guard_evaluation_route_counts[index]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    if route.predecessor_heating_mode_guard_false_fallthrough {
        state
            .predecessor_heating_mode_guard_false_fallthrough_count
            .checked_add(1)
            .is_some()
            && state.predecessor_heating_mode_guard_false_fallthrough_route_counts[index]
                .checked_add(1)
                .is_some()
    } else {
        route.assignment_executed
            && state
                .heating_operating_mode_heat_assignment_count
                .checked_add(1)
                .is_some()
            && state.heating_operating_mode_heat_assignment_route_counts[index]
                .checked_add(1)
                .is_some()
            && state.source_site_execution_count.checked_add(1).is_some()
            && state
                .cp432_heating_operating_mode_state_owner_count
                .checked_add(1)
                .is_some()
            && state
                .heating_operating_mode_assignment_write_count
                .checked_add(1)
                .is_some()
    }
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp431_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp431_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp431_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if !route.predecessor_heating_mode_guard_evaluated {
        state.inactive_transition_count += 1;
        return;
    }
    state.predecessor_heating_mode_guard_evaluation_count += 1;
    state.predecessor_heating_mode_guard_evaluation_route_counts[index] += 1;
    if route.predecessor_heating_mode_guard_false_fallthrough {
        state.predecessor_heating_mode_guard_false_fallthrough_count += 1;
        state.predecessor_heating_mode_guard_false_fallthrough_route_counts[index] += 1;
    } else {
        state.heating_operating_mode_heat_assignment_count += 1;
        state.heating_operating_mode_heat_assignment_route_counts[index] += 1;
        state.source_site_execution_count += 1;
        state.cp432_heating_operating_mode_state_owner_count += 1;
        state.heating_operating_mode_assignment_write_count += 1;
    }
}
