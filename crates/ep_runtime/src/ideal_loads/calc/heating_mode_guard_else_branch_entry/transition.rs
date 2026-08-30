//! Pure CP432-to-CP433 heating-mode guard else-branch entry transition.

use super::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP433 route over CP432's exact 36-wide base partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingModeGuardElseBranchEntryRetainedRoute {
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
    pub entered: bool,
}

use PurchasedAirCalcHeatingModeGuardElseBranchEntryRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_mode_guard_else_branch_entry_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    let entered = predecessor_route.predecessor_heating_mode_guard_false_fallthrough;
    (predecessor_route.logical_index < 36
        && predecessor.heating_mode_guard_false_fallthrough
            == predecessor_route.predecessor_heating_mode_guard_false_fallthrough
        && predecessor.heating_operating_mode_heat_assignment_executed
            == predecessor_route.assignment_executed
        && predecessor.heating_operating_mode_heat_assignment_performed
            == predecessor_route.assignment_executed
        && entered == predecessor.heating_mode_guard_false_fallthrough)
        .then_some(Route {
            logical_index: predecessor_route.logical_index,
            predecessor_active: predecessor_route.predecessor_active,
            predecessor_assignment_executed: predecessor_route.predecessor_assignment_executed,
            predecessor_entered: predecessor_route.predecessor_entered,
            predecessor_total_output_assignment_executed: predecessor_route
                .predecessor_total_output_assignment_executed,
            predecessor_heating_or_no_load_case_entered: predecessor_route
                .predecessor_heating_or_no_load_case_entered,
            predecessor_heating_mode_guard_evaluated: predecessor_route
                .predecessor_heating_mode_guard_evaluated,
            predecessor_sensible_comparison_satisfied: predecessor_route
                .predecessor_sensible_comparison_satisfied,
            predecessor_single_cool_blocked: predecessor_route.predecessor_single_cool_blocked,
            predecessor_heating_operating_mode_body_entered: predecessor_route
                .predecessor_heating_operating_mode_body_entered,
            predecessor_heating_mode_guard_false_fallthrough: predecessor_route
                .predecessor_heating_mode_guard_false_fallthrough,
            assignment_executed: predecessor_route.assignment_executed,
            entered,
        })
}

pub(in crate::ideal_loads::calc) fn advance_heating_mode_guard_else_branch_entry_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::heating_operating_mode_heat_assignment_snapshot_route(predecessor)?;
    let route = heating_mode_guard_else_branch_entry_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    advance_heating_mode_guard_else_branch_entry_state_with_validated_route(
        state,
        predecessor,
        predecessor_route,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_heating_mode_guard_else_branch_entry_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || heating_mode_guard_else_branch_entry_route_from_committed_predecessor(
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
