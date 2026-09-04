//! Pure CP439-to-CP440 continue-warning-call-site transition.

use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot as Snapshot,
};
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallRetainedRoute as PredecessorRoute;
use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot as Predecessor;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP440 route over CP439's exact 36-wide base partition.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallRetainedRoute {
    pub logical_index: usize,
    pub predecessor_guard_false_fallthrough: bool,
    pub predecessor_guard_body_entered: bool,
    pub predecessor_assignment_executed: bool,
    pub predecessor_first_warning_guard_evaluated: bool,
    pub predecessor_first_warning_branch_entered: bool,
    pub predecessor_first_warning_guard_false_fallthrough: bool,
    pub predecessor_counter_increment_executed: bool,
    pub predecessor_first_warning_call_site_reached: bool,
    pub continue_warning_call_site_reached: bool,
}

use PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_outdoor_air_maximum_flow_continue_warning_call_route_from_committed_predecessor(
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
            != predecessor_route.predecessor_first_warning_guard_evaluated
        || predecessor.heating_outdoor_air_maximum_flow_first_warning_branch_entered
            != predecessor_route.predecessor_first_warning_branch_entered
        || predecessor.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
            != predecessor_route.predecessor_first_warning_guard_false_fallthrough
        || predecessor
            .heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed
            != predecessor_route.predecessor_counter_increment_executed
        || predecessor.heating_outdoor_air_maximum_flow_first_warning_call_site_reached
            != predecessor_route.first_warning_call_site_reached
    {
        return None;
    }
    Some(Route {
        logical_index: predecessor_route.logical_index,
        predecessor_guard_false_fallthrough: predecessor_route
            .predecessor_guard_false_fallthrough,
        predecessor_guard_body_entered: predecessor_route.predecessor_guard_body_entered,
        predecessor_assignment_executed: predecessor_route.predecessor_assignment_executed,
        predecessor_first_warning_guard_evaluated: predecessor_route
            .predecessor_first_warning_guard_evaluated,
        predecessor_first_warning_branch_entered: predecessor_route
            .predecessor_first_warning_branch_entered,
        predecessor_first_warning_guard_false_fallthrough: predecessor_route
            .predecessor_first_warning_guard_false_fallthrough,
        predecessor_counter_increment_executed: predecessor_route
            .predecessor_counter_increment_executed,
        predecessor_first_warning_call_site_reached: predecessor_route
            .first_warning_call_site_reached,
        continue_warning_call_site_reached: predecessor_route.first_warning_call_site_reached,
    })
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_continue_warning_call_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_first_warning_call_snapshot_route(predecessor)?;
    let route = heating_outdoor_air_maximum_flow_continue_warning_call_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    advance_heating_outdoor_air_maximum_flow_continue_warning_call_state_with_validated_route(
        state,
        predecessor,
        predecessor_route,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_continue_warning_call_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || heating_outdoor_air_maximum_flow_continue_warning_call_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        ) != Some(route)
        || !next_transition_fits(state, predecessor, route)
    {
        return None;
    }
    let snapshot = snapshot::build_snapshot(predecessor, route);
    state.transition_count += 1;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
