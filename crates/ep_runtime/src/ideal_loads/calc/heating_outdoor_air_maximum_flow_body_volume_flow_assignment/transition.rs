//! Pure CP435-to-CP436 heating outdoor-air volume-flow assignment.

use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP436 route over CP435's exact 36-wide base partition.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute {
    pub logical_index: usize,
    pub predecessor_guard_false_fallthrough: bool,
    pub predecessor_guard_body_entered: bool,
    pub assignment_executed: bool,
}

use PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    if predecessor_route.logical_index >= 36
        || predecessor.maximum_heating_flow_body_entered != predecessor_route.body_entered
        || predecessor.heating_outdoor_air_maximum_flow_guard_false_fallthrough
            != predecessor_route.false_fallthrough
    {
        return None;
    }
    Some(Route {
        logical_index: predecessor_route.logical_index,
        predecessor_guard_false_fallthrough: predecessor_route.false_fallthrough,
        predecessor_guard_body_entered: predecessor_route.body_entered,
        assignment_executed: predecessor_route.body_entered,
    })
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    standard_air_density_kg_per_m3: f64,
) -> Option<Snapshot> {
    let predecessor_route =
        crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_guard_snapshot_route(
            predecessor,
        )?;
    let route =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )?;
    advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_state_with_validated_route(
        state,
        predecessor,
        predecessor_route,
        standard_air_density_kg_per_m3,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    standard_air_density_kg_per_m3: f64,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        ) != Some(route)
        || !next_transition_fits(state, predecessor, route)
    {
        return None;
    }
    if route.assignment_executed
        && (!standard_air_density_kg_per_m3.is_finite()
            || standard_air_density_kg_per_m3 <= 0.0)
    {
        return None;
    }
    let outdoor_air_mass_flow_rate_kg_per_s = if route.assignment_executed {
        predecessor.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s?
    } else {
        0.0
    };
    let calculated_outdoor_air_volume_flow_rate_m3_per_s = route
        .assignment_executed
        .then(|| outdoor_air_mass_flow_rate_kg_per_s / standard_air_density_kg_per_m3);
    let snapshot = snapshot::build_snapshot(
        predecessor,
        standard_air_density_kg_per_m3,
        calculated_outdoor_air_volume_flow_rate_m3_per_s,
        route,
    );
    state.transition_count += 1;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
