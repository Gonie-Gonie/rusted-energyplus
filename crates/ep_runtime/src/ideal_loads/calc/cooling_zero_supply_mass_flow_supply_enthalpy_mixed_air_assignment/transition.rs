//! Pure CP424-to-CP425 zero-flow supply-enthalpy assignment.

use super::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP425 route over the exact 36-wide CP424 partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRetainedRoute {
    pub logical_index: usize,
    pub active: bool,
    pub predecessor_assignment_executed: bool,
    pub predecessor_entered: bool,
    pub assignment_executed: bool,
}

use PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    let active = predecessor_route.entered;
    (predecessor_route.logical_index < 36
        && predecessor_route.active
            == matches!(predecessor_route.logical_index, 4 | 7 | 10 | 13 | 16)
        && predecessor_route.entered == (predecessor_route.logical_index == 2)
        && predecessor.cooling_supply_mass_flow_positive_guard_else_branch_entered
            == predecessor_route.entered)
        .then_some(Route {
            logical_index: predecessor_route.logical_index,
            active,
            predecessor_assignment_executed: predecessor_route.assignment_executed,
            predecessor_entered: predecessor_route.entered,
            assignment_executed: active,
        })
}

pub(in crate::ideal_loads::calc) fn advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    mixed_air_enthalpy_j_per_kg: Option<f64>,
) -> Option<Snapshot> {
    let predecessor_route =
        crate::ideal_loads::calc::cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshot_route(predecessor)?;
    let route = cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_state_with_validated_route(
        state,
        predecessor,
        route,
        mixed_air_enthalpy_j_per_kg,
    )
}

pub(in crate::ideal_loads::calc) fn advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    route: Route,
    mixed_air_enthalpy_j_per_kg: Option<f64>,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || route.assignment_executed != mixed_air_enthalpy_j_per_kg.is_some()
        || !route_matches_predecessor(predecessor, route)
        || !next_transition_fits(state, predecessor, route)
    {
        return None;
    }
    let snapshot = snapshot::build_snapshot(predecessor, route, mixed_air_enthalpy_j_per_kg);
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
        active: matches!(route.logical_index, 4 | 7 | 10 | 13 | 16),
        assignment_executed: route.predecessor_assignment_executed,
        entered: route.predecessor_entered,
    };
    crate::ideal_loads::calc::cooling_supply_mass_flow_positive_guard_else_branch_entry_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
}
