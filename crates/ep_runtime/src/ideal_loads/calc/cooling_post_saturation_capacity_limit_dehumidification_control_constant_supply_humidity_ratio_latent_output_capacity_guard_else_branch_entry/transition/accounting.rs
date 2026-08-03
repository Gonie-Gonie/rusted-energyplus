//! Checked CP406 route and sole source-site accounting.

use super::routes::RetainedRoute;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState as State,
};

pub(super) fn next_transition_fits(state: &State, route: RetainedRoute) -> bool {
    let index = route.predecessor_index;
    if index >= state.predecessor_route_counts.len()
        || state.transition_count.checked_add(1).is_none()
        || state.predecessor_route_counts[index].checked_add(1).is_none()
    {
        return false;
    }
    let else_entered = route.guard_evaluated && !route.assignment_executed;
    let checked = [
        (!else_entered, state.inactive_transition_count),
        (
            else_entered,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            else_entered,
            state.predecessor_guard_false_fallthrough_route_counts[index],
        ),
        (
            route.assignment_executed,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            route.assignment_executed,
            state.predecessor_maximum_capacity_assignment_route_counts[index],
        ),
        (
            else_entered,
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count,
        ),
        (else_entered, state.else_branch_entry_route_counts[index]),
    ];
    if checked
        .into_iter()
        .any(|(used, count)| used && count.checked_add(1).is_none())
    {
        return false;
    }
    !else_entered
        || state
            .source_site_execution_count
            .checked_add(SOURCE_ORDER.len())
            .is_some()
}

pub(super) fn increment_counts(state: &mut State, route: RetainedRoute) {
    let index = route.predecessor_index;
    let else_entered = route.guard_evaluated && !route.assignment_executed;
    state.transition_count += 1;
    state.predecessor_route_counts[index] += 1;
    if else_entered {
        state.predecessor_guard_false_fallthrough_count += 1;
        state.predecessor_guard_false_fallthrough_route_counts[index] += 1;
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count += 1;
        state.else_branch_entry_route_counts[index] += 1;
        state.source_site_execution_count += SOURCE_ORDER.len();
    } else {
        state.inactive_transition_count += 1;
        if route.assignment_executed {
            state.predecessor_maximum_capacity_assignment_count += 1;
            state.predecessor_maximum_capacity_assignment_route_counts[index] += 1;
        }
    }
}
