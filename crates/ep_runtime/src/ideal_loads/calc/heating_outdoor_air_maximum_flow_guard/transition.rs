//! Pure CP434-to-CP435 heating outdoor-air maximum-flow guard.

use ep_model::IdealLoadsLimit;

use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP435 route over CP434's exact 36-wide base partition.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute {
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
    pub predecessor_heating_operating_mode_heat_assignment_executed: bool,
    pub predecessor_heating_mode_guard_else_branch_entered: bool,
    pub predecessor_heating_operating_mode_deadband_assignment_executed: bool,
    pub guard_evaluated: bool,
    pub heating_limit_flow_rate_comparison_satisfied: bool,
    pub heating_limit_flow_rate_and_capacity_comparison_evaluated: bool,
    pub heating_limit_flow_rate_and_capacity_comparison_satisfied: bool,
    pub heating_flow_limit_active: bool,
    pub heating_flow_limit_selector_rejected: bool,
    pub strict_mass_flow_comparison_evaluated: bool,
    pub body_entered: bool,
    pub false_fallthrough: bool,
}

use PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    heating_limit: IdealLoadsLimit,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    maximum_heating_air_mass_flow_rate_kg_per_s: f64,
) -> Option<Route> {
    let active = predecessor_route.predecessor_heating_or_no_load_case_entered;
    if predecessor_route.logical_index >= 36
        || predecessor.heating_operating_mode_deadband_assignment_executed
            != predecessor_route.assignment_executed
        || predecessor.heating_or_no_load_case_entered != active
    {
        return None;
    }
    let first = active && heating_limit == IdealLoadsLimit::LimitFlowRate;
    let second_evaluated = active && !first;
    let second = second_evaluated && heating_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let flow_active = first || second;
    let body = flow_active
        && outdoor_air_mass_flow_rate_kg_per_s > maximum_heating_air_mass_flow_rate_kg_per_s;
    Some(Route {
        logical_index: predecessor_route.logical_index,
        predecessor_active: predecessor_route.predecessor_active,
        predecessor_assignment_executed: predecessor_route.predecessor_assignment_executed,
        predecessor_entered: predecessor_route.predecessor_entered,
        predecessor_total_output_assignment_executed: predecessor_route
            .predecessor_total_output_assignment_executed,
        predecessor_heating_or_no_load_case_entered: active,
        predecessor_heating_mode_guard_evaluated: predecessor_route
            .predecessor_heating_mode_guard_evaluated,
        predecessor_sensible_comparison_satisfied: predecessor_route
            .predecessor_sensible_comparison_satisfied,
        predecessor_single_cool_blocked: predecessor_route.predecessor_single_cool_blocked,
        predecessor_heating_operating_mode_body_entered: predecessor_route
            .predecessor_heating_operating_mode_body_entered,
        predecessor_heating_mode_guard_false_fallthrough: predecessor_route
            .predecessor_heating_mode_guard_false_fallthrough,
        predecessor_heating_operating_mode_heat_assignment_executed: predecessor_route
            .predecessor_heating_operating_mode_heat_assignment_executed,
        predecessor_heating_mode_guard_else_branch_entered: predecessor_route
            .predecessor_heating_mode_guard_else_branch_entered,
        predecessor_heating_operating_mode_deadband_assignment_executed: predecessor_route
            .assignment_executed,
        guard_evaluated: active,
        heating_limit_flow_rate_comparison_satisfied: first,
        heating_limit_flow_rate_and_capacity_comparison_evaluated: second_evaluated,
        heating_limit_flow_rate_and_capacity_comparison_satisfied: second,
        heating_flow_limit_active: flow_active,
        heating_flow_limit_selector_rejected: active && !flow_active,
        strict_mass_flow_comparison_evaluated: flow_active,
        body_entered: body,
        false_fallthrough: active && !body,
    })
}

pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    heating_limit: IdealLoadsLimit,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    maximum_heating_air_mass_flow_rate_kg_per_s: f64,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::heating_operating_mode_deadband_assignment_snapshot_route(predecessor)?;
    let route = heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
        heating_limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_heating_air_mass_flow_rate_kg_per_s,
    )?;
    advance_heating_outdoor_air_maximum_flow_guard_state_with_validated_route(
        state,
        predecessor,
        predecessor_route,
        heating_limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_heating_air_mass_flow_rate_kg_per_s,
        route,
    )
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads::calc) fn advance_heating_outdoor_air_maximum_flow_guard_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    heating_limit: IdealLoadsLimit,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    maximum_heating_air_mass_flow_rate_kg_per_s: f64,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
            heating_limit,
            outdoor_air_mass_flow_rate_kg_per_s,
            maximum_heating_air_mass_flow_rate_kg_per_s,
        ) != Some(route)
        || !next_transition_fits(state, predecessor, route)
    {
        return None;
    }
    let snapshot = snapshot::build_snapshot(
        predecessor,
        heating_limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_heating_air_mass_flow_rate_kg_per_s,
        route,
    );
    state.transition_count += 1;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
