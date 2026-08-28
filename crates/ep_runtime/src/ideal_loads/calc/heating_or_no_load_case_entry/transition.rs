//! Pure CP429-to-CP430 heating-or-no-load case-entry transition.

use super::{
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState as State,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP430 route over the exact 36-wide CP429 partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingOrNoLoadCaseEntryRetainedRoute {
    pub logical_index: usize,
    pub active: bool,
    pub predecessor_assignment_executed: bool,
    pub predecessor_entered: bool,
    pub assignment_executed: bool,
    pub entered: bool,
}

use PurchasedAirCalcHeatingOrNoLoadCaseEntryRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_or_no_load_case_entry_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    let entered = predecessor.non_cooling_skipped;
    (predecessor_route.logical_index < 36
        && predecessor_route.active == (predecessor_route.logical_index == 2)
        && predecessor_route.predecessor_entered == (predecessor_route.logical_index == 2)
        && predecessor_route.assignment_executed == predecessor_route.active
        && predecessor
            .cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_executed
            == predecessor_route.assignment_executed
        && entered == (predecessor_route.logical_index == 1))
        .then_some(Route {
            logical_index: predecessor_route.logical_index,
            active: predecessor_route.active,
            predecessor_assignment_executed: predecessor_route.predecessor_assignment_executed,
            predecessor_entered: predecessor_route.predecessor_entered,
            assignment_executed: predecessor_route.assignment_executed,
            entered,
        })
}

pub(in crate::ideal_loads::calc) fn advance_heating_or_no_load_case_entry_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshot_route(predecessor)?;
    let route = heating_or_no_load_case_entry_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    advance_heating_or_no_load_case_entry_state_with_validated_route(
        state,
        predecessor,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_heating_or_no_load_case_entry_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || !route_matches_predecessor(predecessor, route)
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

fn route_matches_predecessor(predecessor: Predecessor, route: Route) -> bool {
    let predecessor_route = PredecessorRoute {
        logical_index: route.logical_index,
        active: route.active,
        predecessor_assignment_executed: route.predecessor_assignment_executed,
        predecessor_entered: route.predecessor_entered,
        assignment_executed: route.assignment_executed,
    };
    crate::ideal_loads::calc::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && heating_or_no_load_case_entry_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
}
