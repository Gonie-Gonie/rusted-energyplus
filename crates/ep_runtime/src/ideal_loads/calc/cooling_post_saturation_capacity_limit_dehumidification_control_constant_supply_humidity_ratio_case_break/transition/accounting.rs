//! Exact CP409 route and sole-site accounting.

use super::State;
use super::routes::RetainedRoute;
use crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER as ORDER;

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    let index = route.predecessor_index;
    if state.transition_count.checked_add(1).is_none()
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
    if !route.active {
        return state.inactive_transition_count.checked_add(1).is_some();
    }
    state
        .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count
        .checked_add(1)
        .is_some()
        && state
            .source_site_execution_count
            .checked_add(ORDER.len())
            .is_some()
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    let index = route.predecessor_index;
    state.predecessor_route_counts[index] += 1;
    if route.predecessor_guard_false_fallthrough {
        state.predecessor_guard_false_fallthrough_count += 1;
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
    }
    if route.predecessor_maximum_capacity_assignment_executed {
        state.predecessor_maximum_capacity_assignment_count += 1;
        state.predecessor_maximum_capacity_assignment_route_counts[index] += 1;
    }
    if !route.active {
        state.inactive_transition_count += 1;
        return;
    }
    state
        .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count +=
        1;
    state.source_site_execution_count += ORDER.len();
}
