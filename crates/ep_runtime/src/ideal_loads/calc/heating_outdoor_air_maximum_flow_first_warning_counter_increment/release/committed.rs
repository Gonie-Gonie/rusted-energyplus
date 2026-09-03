//! Sealed bounded CP438 route and canonical warning-counter capability for CP439.

use super::super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRetainedRoute as Route;
use super::prefix::predecessor_cp437_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{prefix_and_local_shape_match, snapshots_match_bit_exact};
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as PredecessorRoute;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact,
};

/// Returns CP438's committed route and canonical counter without recursive route replay.
pub(in crate::ideal_loads::calc) fn heating_outdoor_air_maximum_flow_first_warning_counter_increment_committed_latest_route_and_outdoor_air_flow_maximum_heating_output_error_count(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<(Route, usize)> {
    let state = &unit.calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp437_snapshot(latest);
    let predecessor_route = PredecessorRoute {
        logical_index: route.logical_index,
        predecessor_guard_false_fallthrough: route.predecessor_guard_false_fallthrough,
        predecessor_guard_body_entered: route.predecessor_guard_body_entered,
        predecessor_assignment_executed: route.predecessor_assignment_executed,
        guard_evaluated: route.predecessor_first_warning_guard_evaluated,
        first_warning_branch_entered: route.predecessor_first_warning_branch_entered,
        guard_false_fallthrough: route.predecessor_first_warning_guard_false_fallthrough,
    };
    let counter_owner = &unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard;
    let counter = counter_owner.outdoor_air_flow_maximum_heating_output_error_count;
    let counter_is_exact = if route.counter_increment_executed {
        counter == 1
            && latest.assigned_outdoor_air_flow_maximum_heating_output_error_count == Some(1)
    } else if route.predecessor_first_warning_guard_evaluated {
        latest
            .outdoor_air_flow_maximum_heating_output_error_count_before
            == Some(counter)
            && latest
                .assigned_outdoor_air_flow_maximum_heating_output_error_count
                .is_none()
    } else {
        counter <= 1
            && latest
                .assigned_outdoor_air_flow_maximum_heating_output_error_count
                .is_none()
    };
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count > 0
        && state.latest_transition_ordinal == Some(state.transition_count)
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == unit.system
        && unit.controlled_zone == Some(latest.controlled_zone)
        && snapshots_match_bit_exact(latest, witness)
        && counter_owner
            .latest
            .is_some_and(|cp437| {
                heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
                    cp437,
                    predecessor,
                )
            })
        && prefix_and_local_shape_match(latest, predecessor, predecessor_route, route)
        && counter_is_exact
        && state_counts_are_consistent(state)
        && predecessor_counts_match(state, counter_owner))
    .then_some((route, counter))
}
