//! Checked CP438 route and single-site increment accounting.

use super::{Predecessor, Route, State};

fn fits(value: usize, increment: bool) -> bool {
    !increment || value.checked_add(1).is_some()
}

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor, route: Route) -> bool {
    let index = route.logical_index;
    let increment = route.counter_increment_executed;
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index]
            .checked_add(1)
            .is_none()
        || !fits(state.inactive_transition_count, !increment)
        || !fits(
            state.outdoor_air_flow_maximum_heating_output_error_count_increment_count,
            increment,
        )
        || !fits(state.source_site_execution_count, increment)
        || !fits(
            state.predecessor_guard_false_fallthrough_route_counts[index],
            route.predecessor_guard_false_fallthrough,
        )
        || !fits(
            state.predecessor_guard_body_entry_route_counts[index],
            route.predecessor_guard_body_entered,
        )
        || !fits(
            state.predecessor_volume_flow_assignment_route_counts[index],
            route.predecessor_assignment_executed,
        )
        || !fits(
            state.predecessor_first_warning_guard_false_fallthrough_route_counts[index],
            route.predecessor_first_warning_guard_false_fallthrough,
        )
        || !fits(
            state.predecessor_first_warning_branch_entry_route_counts[index],
            route.predecessor_first_warning_branch_entered,
        )
        || !fits(
            state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts
                [index],
            increment,
        )
        || !fits(
            state.cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
            increment,
        )
        || !fits(
            state.outdoor_air_flow_maximum_heating_output_error_count_increment_write_count,
            increment,
        )
    {
        return false;
    }
    [
        (
            predecessor.resulting_supply_humidity_ratio.is_some(),
            state.cp437_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
            state.cp437_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            predecessor.resulting_supply_temperature_c.is_some(),
            state.cp437_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ]
    .into_iter()
    .all(|(present, owner, unchanged)| fits(owner, present) && fits(unchanged, present))
        && route.predecessor_assignment_executed == route.predecessor_first_warning_guard_evaluated
        && route.predecessor_first_warning_guard_evaluated
            == (route.predecessor_first_warning_branch_entered
                || route.predecessor_first_warning_guard_false_fallthrough)
        && !(route.predecessor_first_warning_branch_entered
            && route.predecessor_first_warning_guard_false_fallthrough)
        && increment == route.predecessor_first_warning_branch_entered
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
    let increment = route.counter_increment_executed;
    state.predecessor_route_counts[index] += 1;
    if route.predecessor_guard_false_fallthrough {
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    }
    if route.predecessor_guard_body_entered {
        state.predecessor_guard_body_entry_route_counts[index] += 1;
    }
    if route.predecessor_assignment_executed {
        state.predecessor_volume_flow_assignment_route_counts[index] += 1;
    }
    if route.predecessor_first_warning_guard_false_fallthrough {
        state.predecessor_first_warning_guard_false_fallthrough_route_counts[index] += 1;
    }
    if route.predecessor_first_warning_branch_entered {
        state.predecessor_first_warning_branch_entry_route_counts[index] += 1;
    }
    if increment {
        state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts
            [index] += 1;
        state.outdoor_air_flow_maximum_heating_output_error_count_increment_count += 1;
        state.source_site_execution_count += 1;
        state.cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count += 1;
        state.outdoor_air_flow_maximum_heating_output_error_count_increment_write_count += 1;
    } else {
        state.inactive_transition_count += 1;
    }
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp437_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp437_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp437_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
}
