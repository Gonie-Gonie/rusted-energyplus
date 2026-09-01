//! Pure CP437-to-CP438 heating outdoor-air maximum-flow first-warning counter increment.

use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot as Snapshot,
};
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as PredecessorRoute;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as CounterOwner,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Predecessor,
};

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP438 route over CP437's exact 36-wide base partition.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRetainedRoute
{
    pub logical_index: usize,
    pub predecessor_guard_false_fallthrough: bool,
    pub predecessor_guard_body_entered: bool,
    pub predecessor_assignment_executed: bool,
    pub predecessor_first_warning_guard_evaluated: bool,
    pub predecessor_first_warning_branch_entered: bool,
    pub predecessor_first_warning_guard_false_fallthrough: bool,
    pub counter_increment_executed: bool,
}

use PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    if predecessor_route.logical_index >= 36
        || predecessor.heating_outdoor_air_maximum_flow_guard_false_fallthrough
            != predecessor_route.predecessor_guard_false_fallthrough
        || predecessor.maximum_heating_flow_body_entered
            != predecessor_route.predecessor_guard_body_entered
        || predecessor.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed
            != predecessor_route.predecessor_assignment_executed
        || predecessor.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated
            != predecessor_route.guard_evaluated
        || predecessor.heating_outdoor_air_maximum_flow_first_warning_branch_entered
            != predecessor_route.first_warning_branch_entered
        || predecessor.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
            != predecessor_route.guard_false_fallthrough
    {
        return None;
    }
    Some(Route {
        logical_index: predecessor_route.logical_index,
        predecessor_guard_false_fallthrough: predecessor_route.predecessor_guard_false_fallthrough,
        predecessor_guard_body_entered: predecessor_route.predecessor_guard_body_entered,
        predecessor_assignment_executed: predecessor_route.predecessor_assignment_executed,
        predecessor_first_warning_guard_evaluated: predecessor_route.guard_evaluated,
        predecessor_first_warning_branch_entered: predecessor_route.first_warning_branch_entered,
        predecessor_first_warning_guard_false_fallthrough: predecessor_route
            .guard_false_fallthrough,
        counter_increment_executed: predecessor_route.first_warning_branch_entered,
    })
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment_state(
    state: &mut State,
    counter_owner: &mut CounterOwner,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_route(predecessor)?;
    let route =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )?;
    advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment_state_with_validated_route(
        state,
        counter_owner,
        predecessor,
        predecessor_route,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment_state_with_validated_route(
    state: &mut State,
    counter_owner: &mut CounterOwner,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || counter_owner.system != predecessor.system
        || heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        ) != Some(route)
        || !counter_owner_matches_predecessor(counter_owner, predecessor, route)
        || !next_transition_fits(state, predecessor, route)
    {
        return None;
    }
    let assigned_counter = if route.counter_increment_executed {
        Some(
            counter_owner
                .outdoor_air_flow_maximum_heating_output_error_count
                .checked_add(1)?,
        )
    } else {
        None
    };
    if route.counter_increment_executed && assigned_counter != Some(1) {
        return None;
    }
    let snapshot = snapshot::build_snapshot(predecessor, assigned_counter, route);
    if let Some(assigned) = assigned_counter {
        counter_owner.outdoor_air_flow_maximum_heating_output_error_count = assigned;
    }
    state.transition_count += 1;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn counter_owner_matches_predecessor(
    counter_owner: &CounterOwner,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    if !route.predecessor_first_warning_guard_evaluated {
        return predecessor
            .outdoor_air_flow_maximum_heating_output_error_count_before
            .is_none();
    }
    predecessor.outdoor_air_flow_maximum_heating_output_error_count_state_owned
        && predecessor.outdoor_air_flow_maximum_heating_output_error_count_read
        && predecessor.outdoor_air_flow_maximum_heating_output_error_count_before
            == Some(counter_owner.outdoor_air_flow_maximum_heating_output_error_count)
        && (!route.counter_increment_executed
            || (counter_owner.outdoor_air_flow_maximum_heating_output_error_count == 0
                && predecessor.outdoor_air_flow_maximum_heating_output_error_count_less_than_one
                    == Some(true)))
}
