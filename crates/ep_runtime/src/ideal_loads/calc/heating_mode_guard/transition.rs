//! Pure CP430-to-CP431 heating-mode-guard transition.

use super::{
    PurchasedAirCalcHeatingModeGuardRuntimeState as State,
    PurchasedAirCalcHeatingModeGuardSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_entry_gate::PurchasedAirTemperatureControlType;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands as Numeric,
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRetainedRoute as PredecessorRoute,
};
use crate::ideal_loads::PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Predecessor;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// Active source inputs acquired through the split sealed CP312 capabilities.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcHeatingModeGuardActiveInput {
    pub numeric: Numeric,
    pub temperature_control_type: Option<PurchasedAirTemperatureControlType>,
}

/// One retained CP431 route over the exact 36-wide CP430 partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcHeatingModeGuardRetainedRoute {
    pub logical_index: usize,
    pub predecessor_active: bool,
    pub predecessor_assignment_executed: bool,
    pub predecessor_entered: bool,
    pub predecessor_total_output_assignment_executed: bool,
    pub predecessor_heating_or_no_load_case_entered: bool,
    pub guard_evaluated: bool,
    pub sensible_comparison_satisfied: bool,
    pub single_cool_blocked: bool,
    pub body_entered: bool,
    pub false_fallthrough: bool,
}

use PurchasedAirCalcHeatingModeGuardRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn heating_mode_guard_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    input: Option<PurchasedAirCalcHeatingModeGuardActiveInput>,
) -> Option<Route> {
    let active = predecessor_route.entered;
    if predecessor_route.logical_index >= 36
        || predecessor_route.active != (predecessor_route.logical_index == 2)
        || predecessor_route.predecessor_assignment_executed
            != predecessor
                .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed
        || predecessor.cooling_supply_mass_flow_positive_guard_else_branch_entered
            != (predecessor_route.logical_index == 2)
        || predecessor_route.predecessor_entered
            != predecessor.cooling_supply_mass_flow_positive_guard_else_branch_entered
        || predecessor_route.assignment_executed != predecessor_route.active
        || predecessor
            .cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_executed
            != (predecessor_route.logical_index == 2)
        || predecessor_route.assignment_executed
            != predecessor
                .cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_executed
        || predecessor.non_cooling_skipped != active
        || predecessor.heating_or_no_load_case_entered != active
        || active != (predecessor_route.logical_index == 1)
    {
        return None;
    }
    let (sensible, blocked, body, fallthrough) = if active {
        let input = input?;
        let sensible = input.numeric.minimum_outdoor_air_sensible_output_w
            < input.numeric.heating_setpoint_demand_w;
        if input.temperature_control_type.is_some() != sensible {
            return None;
        }
        let permits = input
            .temperature_control_type
            .map(|value| value != PurchasedAirTemperatureControlType::SingleCool);
        let blocked = permits == Some(false);
        let body = permits == Some(true);
        (sensible, blocked, body, !body)
    } else {
        if input.is_some() {
            return None;
        }
        (false, false, false, false)
    };
    Some(Route {
        logical_index: predecessor_route.logical_index,
        predecessor_active: predecessor_route.active,
        predecessor_assignment_executed: predecessor_route.predecessor_assignment_executed,
        predecessor_entered: predecessor_route.predecessor_entered,
        predecessor_total_output_assignment_executed: predecessor_route.assignment_executed,
        predecessor_heating_or_no_load_case_entered: predecessor_route.entered,
        guard_evaluated: active,
        sensible_comparison_satisfied: sensible,
        single_cool_blocked: blocked,
        body_entered: body,
        false_fallthrough: fallthrough,
    })
}

pub(in crate::ideal_loads::calc) fn advance_heating_mode_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<PurchasedAirCalcHeatingModeGuardActiveInput>,
) -> Option<Snapshot> {
    let predecessor_route = predecessor_route(predecessor)?;
    let route =
        heating_mode_guard_route_from_committed_predecessor(predecessor, predecessor_route, input)?;
    advance_heating_mode_guard_state_with_validated_route(
        state,
        predecessor,
        predecessor_route,
        input,
        route,
    )
}

pub(in crate::ideal_loads::calc) fn advance_heating_mode_guard_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    input: Option<PurchasedAirCalcHeatingModeGuardActiveInput>,
    route: Route,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || heating_mode_guard_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
            input,
        ) != Some(route)
        || !next_transition_fits(state, predecessor, route)
    {
        return None;
    }
    let snapshot = snapshot::build_snapshot(predecessor, input, route);
    state.transition_count += 1;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(super) fn predecessor_route(predecessor: Predecessor) -> Option<PredecessorRoute> {
    let cp429 =
        crate::ideal_loads::heating_or_no_load_case_entry_predecessor_cp429_snapshot(predecessor);
    let cp429_route = crate::ideal_loads::calc::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshot_route(cp429)?;
    crate::ideal_loads::calc::heating_or_no_load_case_entry_route_from_committed_predecessor(
        cp429,
        cp429_route,
    )
}
