//! Pure CP423-to-CP424 positive-supply guard else-entry transition.

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP424 route over the exact 36-wide CP423 partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRetainedRoute {
    pub logical_index: usize,
    pub active: bool,
    pub assignment_executed: bool,
    pub entered: bool,
}

use PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn cooling_supply_mass_flow_positive_guard_else_branch_entry_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    let entered = predecessor.positive_guard_false_fallthrough_skipped;
    (predecessor_route.logical_index < 36
        && predecessor_route.active
            == matches!(predecessor_route.logical_index, 4 | 7 | 10 | 13 | 16)
        && entered == (predecessor_route.logical_index == 2))
        .then_some(Route {
            logical_index: predecessor_route.logical_index,
            active: predecessor_route.active,
            assignment_executed: predecessor_route.assignment_executed,
            entered,
        })
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_mass_flow_positive_guard_else_branch_entry_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshot_route(predecessor)?;
    let route = cooling_supply_mass_flow_positive_guard_else_branch_entry_route_from_committed_predecessor(predecessor, predecessor_route)?;
    advance_cooling_supply_mass_flow_positive_guard_else_branch_entry_state_with_validated_route(state, predecessor, route)
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_mass_flow_positive_guard_else_branch_entry_state_with_validated_route(
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
        assignment_executed: route.assignment_executed,
    };
    if !crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) {
        return false;
    }
    cooling_supply_mass_flow_positive_guard_else_branch_entry_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
}
