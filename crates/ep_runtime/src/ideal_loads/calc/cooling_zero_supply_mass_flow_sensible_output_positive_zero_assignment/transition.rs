//! Pure CP427-to-CP428 zero-flow sensible-output positive-zero assignment.

use super::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP428 route over the exact 36-wide CP427 partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRetainedRoute {
    pub logical_index: usize,
    pub active: bool,
    pub predecessor_assignment_executed: bool,
    pub predecessor_entered: bool,
    pub assignment_executed: bool,
}

use PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    let active = predecessor_route.assignment_executed;
    (predecessor_route.logical_index < 36
        && predecessor_route.active == (predecessor_route.logical_index == 2)
        && predecessor_route.predecessor_entered == (predecessor_route.logical_index == 2)
        && predecessor_route.assignment_executed == predecessor_route.active
        && predecessor.cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed
            == predecessor_route.assignment_executed)
        .then_some(Route {
            logical_index: predecessor_route.logical_index,
            active,
            predecessor_assignment_executed: predecessor_route.predecessor_assignment_executed,
            predecessor_entered: predecessor_route.predecessor_entered,
            assignment_executed: active,
        })
}

pub(in crate::ideal_loads::calc) fn advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let predecessor_route =
        crate::ideal_loads::calc::cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_snapshot_route(predecessor)?;
    let route = cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_state_with_validated_route(
        state,
        predecessor,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_state_with_validated_route(
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
    crate::ideal_loads::calc::cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
}
