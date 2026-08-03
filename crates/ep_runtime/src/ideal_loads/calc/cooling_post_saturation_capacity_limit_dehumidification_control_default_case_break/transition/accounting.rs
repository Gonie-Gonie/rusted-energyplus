//! Exact CP410 retained-route and zero-site accounting.

use super::State;
use super::routes::RetainedRoute;

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    let index = route.predecessor_index;
    if state.dehumidification_control_default_case_break_count != 0
        || state.source_site_execution_count != 0
        || state.transition_count.checked_add(1).is_none()
        || state.inactive_transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    if route.predecessor_guard_false_fallthrough
        && (state
            .predecessor_guard_false_fallthrough_count
            .checked_add(1)
            .is_none()
            || state.predecessor_guard_false_fallthrough_route_counts[index]
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    if route.predecessor_maximum_capacity_assignment_executed
        && (state
            .predecessor_maximum_capacity_assignment_count
            .checked_add(1)
            .is_none()
            || state.predecessor_maximum_capacity_assignment_route_counts[index]
                .checked_add(1)
                .is_none())
    {
        return false;
    }
    true
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    let index = route.predecessor_index;
    state.inactive_transition_count += 1;
    state.predecessor_route_counts[index] += 1;
    if route.predecessor_guard_false_fallthrough {
        state.predecessor_guard_false_fallthrough_count += 1;
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    }
    if route.predecessor_maximum_capacity_assignment_executed {
        state.predecessor_maximum_capacity_assignment_count += 1;
        state.predecessor_maximum_capacity_assignment_route_counts[index] += 1;
    }
}
