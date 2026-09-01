//! Checked CP437 route and source-site accounting.

use super::{Predecessor, Route, State};

fn fits(value: usize, increment: bool) -> bool {
    !increment || value.checked_add(1).is_some()
}

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor, route: Route) -> bool {
    let index = route.logical_index;
    let source_sites =
        usize::from(route.guard_evaluated) * 2 + usize::from(route.first_warning_branch_entered);
    if index >= 36
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
        || state
            .source_site_execution_count
            .checked_add(source_sites)
            .is_none()
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
            state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts[index],
            route.guard_false_fallthrough,
        )
        || !fits(
            state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts[index],
            route.first_warning_branch_entered,
        )
        || !fits(state.inactive_transition_count, !route.guard_evaluated)
        || !fits(state.guard_evaluation_count, route.guard_evaluated)
        || !fits(
            state.first_warning_branch_entry_count,
            route.first_warning_branch_entered,
        )
        || !fits(
            state.guard_false_fallthrough_count,
            route.guard_false_fallthrough,
        )
    {
        return false;
    }
    [
        state.outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        state.outdoor_air_flow_maximum_heating_output_error_count_read_count,
        state.outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count,
    ]
    .into_iter()
    .all(|count| fits(count, route.guard_evaluated))
        && [
            (
                predecessor.resulting_supply_humidity_ratio.is_some(),
                state.cp436_supply_humidity_ratio_state_owner_count,
                state.unchanged_supply_humidity_ratio_preservation_count,
            ),
            (
                predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
                state.cp436_supply_enthalpy_state_owner_count,
                state.unchanged_supply_enthalpy_preservation_count,
            ),
            (
                predecessor.resulting_supply_temperature_c.is_some(),
                state.cp436_supply_temperature_state_owner_count,
                state.unchanged_supply_temperature_preservation_count,
            ),
        ]
        .into_iter()
        .all(|(present, owner, unchanged)| fits(owner, present) && fits(unchanged, present))
        && route.predecessor_assignment_executed == route.guard_evaluated
        && route.guard_evaluated
            == (route.first_warning_branch_entered || route.guard_false_fallthrough)
        && !(route.first_warning_branch_entered && route.guard_false_fallthrough)
}

pub(super) fn increment_counts(state: &mut State, predecessor: Predecessor, route: Route) {
    let index = route.logical_index;
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
    if route.guard_evaluated {
        state.guard_evaluation_count += 1;
        state.outdoor_air_flow_maximum_heating_output_error_count_state_owner_count += 1;
        state.outdoor_air_flow_maximum_heating_output_error_count_read_count += 1;
        state.outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count +=
            1;
        state.source_site_execution_count += 2;
    } else {
        state.inactive_transition_count += 1;
    }
    if route.first_warning_branch_entered {
        state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts[index] += 1;
        state.first_warning_branch_entry_count += 1;
        state.source_site_execution_count += 1;
    }
    if route.guard_false_fallthrough {
        state
            .heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts
            [index] += 1;
        state.guard_false_fallthrough_count += 1;
    }
    if predecessor.resulting_supply_humidity_ratio.is_some() {
        state.cp436_supply_humidity_ratio_state_owner_count += 1;
        state.unchanged_supply_humidity_ratio_preservation_count += 1;
    }
    if predecessor.resulting_supply_enthalpy_j_per_kg.is_some() {
        state.cp436_supply_enthalpy_state_owner_count += 1;
        state.unchanged_supply_enthalpy_preservation_count += 1;
    }
    if predecessor.resulting_supply_temperature_c.is_some() {
        state.cp436_supply_temperature_state_owner_count += 1;
        state.unchanged_supply_temperature_preservation_count += 1;
    }
}
