//! Exact CP388 route and source-site accounting.

use super::State;
use super::routes::RetainedRoute;
use crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER;

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    if state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[route.predecessor_index]
            .checked_add(1)
            .is_none()
    {
        return false;
    }
    if !route.active {
        return state.inactive_transition_count.checked_add(1).is_some();
    }
    [
        state.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
        state.cooling_total_output_owned_read_count,
        state.cooling_total_output_bit_corroboration_count,
        state.cooling_sensible_heat_ratio_read_count,
        state.cooling_sensible_output_calculation_count,
        state.cooling_sensible_output_assignment_write_count,
    ]
    .into_iter()
    .all(|count| count.checked_add(1).is_some())
        && state
            .source_site_execution_count
            .checked_add(
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
            )
            .is_some()
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    state.predecessor_route_counts[route.predecessor_index] += 1;
    if !route.active {
        state.inactive_transition_count += 1;
        return;
    }
    state.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count += 1;
    state.source_site_execution_count +=
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len();
    state.cooling_total_output_owned_read_count += 1;
    state.cooling_total_output_bit_corroboration_count += 1;
    state.cooling_sensible_heat_ratio_read_count += 1;
    state.cooling_sensible_output_calculation_count += 1;
    state.cooling_sensible_output_assignment_write_count += 1;
}
