//! Pure CP436-to-CP437 heating outdoor-air maximum-flow first-warning guard.

use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP437 route over CP436's exact 36-wide base partition.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute
{
    pub logical_index: usize,
    pub predecessor_guard_false_fallthrough: bool,
    pub predecessor_guard_body_entered: bool,
    pub predecessor_assignment_executed: bool,
    pub guard_evaluated: bool,
    pub first_warning_branch_entered: bool,
    pub guard_false_fallthrough: bool,
}

use PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    outdoor_air_flow_maximum_heating_output_error_count: usize,
) -> Option<Route> {
    if predecessor_route.logical_index >= 36
        || predecessor.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed
            != predecessor_route.assignment_executed
        || predecessor.maximum_heating_flow_body_entered
            != predecessor_route.predecessor_guard_body_entered
        || predecessor.heating_outdoor_air_maximum_flow_guard_false_fallthrough
            != predecessor_route.predecessor_guard_false_fallthrough
    {
        return None;
    }
    let guard_evaluated = predecessor_route.assignment_executed;
    let first_warning_branch_entered =
        guard_evaluated && outdoor_air_flow_maximum_heating_output_error_count < 1;
    let guard_false_fallthrough = guard_evaluated && !first_warning_branch_entered;
    Some(Route {
        logical_index: predecessor_route.logical_index,
        predecessor_guard_false_fallthrough: predecessor_route.predecessor_guard_false_fallthrough,
        predecessor_guard_body_entered: predecessor_route.predecessor_guard_body_entered,
        predecessor_assignment_executed: predecessor_route.assignment_executed,
        guard_evaluated,
        first_warning_branch_entered,
        guard_false_fallthrough,
    })
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_first_warning_guard_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_route(predecessor)?;
    let route =
        heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
            state.outdoor_air_flow_maximum_heating_output_error_count,
        )?;
    advance_heating_outdoor_air_maximum_flow_first_warning_guard_state_with_validated_route(
        state,
        predecessor,
        predecessor_route,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_first_warning_guard_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
            state.outdoor_air_flow_maximum_heating_output_error_count,
        ) != Some(route)
        || !next_transition_fits(state, predecessor, route)
    {
        return None;
    }
    let warning_counter_before = route
        .guard_evaluated
        .then_some(state.outdoor_air_flow_maximum_heating_output_error_count);
    let snapshot = snapshot::build_snapshot(predecessor, warning_counter_before, route);
    state.transition_count += 1;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
