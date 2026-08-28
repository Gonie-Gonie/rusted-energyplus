//! Pure CP431-to-CP432 heating operating-mode Heat assignment.

use super::{
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcHeatingModeGuardSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcHeatingModeGuardRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP432 route over CP431's exact 36-wide base partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute {
    pub logical_index: usize,
    pub predecessor_active: bool,
    pub predecessor_assignment_executed: bool,
    pub predecessor_entered: bool,
    pub predecessor_total_output_assignment_executed: bool,
    pub predecessor_heating_or_no_load_case_entered: bool,
    pub predecessor_heating_mode_guard_evaluated: bool,
    pub predecessor_sensible_comparison_satisfied: bool,
    pub predecessor_single_cool_blocked: bool,
    pub predecessor_heating_operating_mode_body_entered: bool,
    pub predecessor_heating_mode_guard_false_fallthrough: bool,
    pub assignment_executed: bool,
}

use PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_operating_mode_heat_assignment_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    let assignment_executed = predecessor_route.body_entered;
    (predecessor_route.logical_index < 36
        && predecessor.heating_mode_guard_evaluated == predecessor_route.guard_evaluated
        && predecessor
            .minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand
            == predecessor_route
                .guard_evaluated
                .then_some(predecessor_route.sensible_comparison_satisfied)
        && predecessor.single_cool_blocked == predecessor_route.single_cool_blocked
        && predecessor.heating_operating_mode_body_entered == predecessor_route.body_entered
        && predecessor.heating_mode_guard_false_fallthrough == predecessor_route.false_fallthrough
        && predecessor_route.body_entered == assignment_executed)
        .then_some(Route {
            logical_index: predecessor_route.logical_index,
            predecessor_active: predecessor_route.predecessor_active,
            predecessor_assignment_executed: predecessor_route.predecessor_assignment_executed,
            predecessor_entered: predecessor_route.predecessor_entered,
            predecessor_total_output_assignment_executed: predecessor_route
                .predecessor_total_output_assignment_executed,
            predecessor_heating_or_no_load_case_entered: predecessor_route
                .predecessor_heating_or_no_load_case_entered,
            predecessor_heating_mode_guard_evaluated: predecessor_route.guard_evaluated,
            predecessor_sensible_comparison_satisfied: predecessor_route
                .sensible_comparison_satisfied,
            predecessor_single_cool_blocked: predecessor_route.single_cool_blocked,
            predecessor_heating_operating_mode_body_entered: predecessor_route.body_entered,
            predecessor_heating_mode_guard_false_fallthrough: predecessor_route.false_fallthrough,
            assignment_executed,
        })
}

pub(in crate::ideal_loads::calc) fn advance_heating_operating_mode_heat_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::heating_mode_guard_snapshot_route(predecessor)?;
    let route = heating_operating_mode_heat_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    advance_heating_operating_mode_heat_assignment_state_with_validated_route(
        state,
        predecessor,
        predecessor_route,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_heating_operating_mode_heat_assignment_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || heating_operating_mode_heat_assignment_route_from_committed_predecessor(
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
