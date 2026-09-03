//! Transactional CP439 route and single call-site accounting.

use super::{Predecessor, Route, State};

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor, route: Route) -> bool {
    let index = route.logical_index;
    if index >= 36 || state.transition_count.checked_add(1).is_none() {
        return false;
    }
    let arrays = [
        state.predecessor_route_counts[index],
        state.predecessor_guard_false_fallthrough_route_counts[index],
        state.predecessor_guard_body_entry_route_counts[index],
        state.predecessor_volume_flow_assignment_route_counts[index],
        state.predecessor_first_warning_guard_false_fallthrough_route_counts[index],
        state.predecessor_first_warning_branch_entry_route_counts[index],
        state.predecessor_first_warning_counter_increment_route_counts[index],
        state.heating_outdoor_air_maximum_flow_first_warning_call_route_counts[index],
    ];
    if arrays.into_iter().any(|count| count.checked_add(1).is_none()) {
        return false;
    }
    let preserved = [
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.cp438_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.cp438_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.cp438_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ];
    if preserved.into_iter().any(|(present, owner, unchanged)| {
        present && (owner.checked_add(1).is_none() || unchanged.checked_add(1).is_none())
    }) {
        return false;
    }
    if route.first_warning_call_site_reached {
        state
            .heating_outdoor_air_maximum_flow_first_warning_call_site_count
            .checked_add(1)
            .is_some()
            && state.source_site_execution_count.checked_add(1).is_some()
            && state
                .cp438_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count
                .checked_add(1)
                .is_some()
            && state
                .unchanged_outdoor_air_flow_maximum_heating_output_error_count_preservation_count
                .checked_add(1)
                .is_some()
    } else {
        state.inactive_transition_count.checked_add(1).is_some()
    }
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    state.predecessor_route_counts[index] += 1;
    state.predecessor_guard_false_fallthrough_route_counts[index] +=
        usize::from(route.predecessor_guard_false_fallthrough);
    state.predecessor_guard_body_entry_route_counts[index] +=
        usize::from(route.predecessor_guard_body_entered);
    state.predecessor_volume_flow_assignment_route_counts[index] +=
        usize::from(route.predecessor_assignment_executed);
    state.predecessor_first_warning_guard_false_fallthrough_route_counts[index] +=
        usize::from(route.predecessor_first_warning_guard_false_fallthrough);
    state.predecessor_first_warning_branch_entry_route_counts[index] +=
        usize::from(route.predecessor_first_warning_branch_entered);
    state.predecessor_first_warning_counter_increment_route_counts[index] +=
        usize::from(route.predecessor_counter_increment_executed);
    state.heating_outdoor_air_maximum_flow_first_warning_call_route_counts[index] +=
        usize::from(route.first_warning_call_site_reached);
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp438_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp438_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp438_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
    if route.first_warning_call_site_reached {
        state.heating_outdoor_air_maximum_flow_first_warning_call_site_count += 1;
        state.source_site_execution_count += 1;
        state.cp438_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count += 1;
        state.unchanged_outdoor_air_flow_maximum_heating_output_error_count_preservation_count += 1;
    } else {
        state.inactive_transition_count += 1;
    }
}
