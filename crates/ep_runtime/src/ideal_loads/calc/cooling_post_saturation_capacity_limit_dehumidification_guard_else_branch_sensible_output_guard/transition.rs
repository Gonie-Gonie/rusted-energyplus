//! Pure CP420-to-CP421 sensible-output maximum-capacity guard.

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentCommittedRoute as PredecessorRoute;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_route_matches_snapshot;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP421 successor route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute {
    pub logical_index: usize,
    pub active: bool,
    pub body_entered: bool,
}

/// Release-validated same-call operands for the line-2332 comparison.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardActiveInput {
    pub cooling_sensible_output_w: f64,
    pub maximum_total_cooling_capacity_w: f64,
    pub cp420_cooling_sensible_output_owned_read: bool,
    pub cp321_maximum_total_cooling_capacity_owned_read: bool,
    pub cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: bool,
}

use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardActiveInput as ActiveInput;
use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor(
    predecessor: Predecessor,
    route: PredecessorRoute,
) -> Option<Route> {
    let active = matches!(route.logical_index, 4 | 7 | 10 | 13 | 16);
    (route.logical_index < 36
        && route.active == active
        && route.active
            == predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed)
        .then_some(Route {
            logical_index: route.logical_index,
            active,
            body_entered: false,
        })
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_route(predecessor)?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state_with_validated_route(
        state,
        predecessor,
        route,
        input,
    )
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    base_route: Route,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system
        || base_route.body_entered
        || !route_matches_predecessor_bounded(predecessor, base_route)
    {
        return None;
    }
    let prepared = prepare_guard(predecessor, base_route, input)?;
    let route = Route {
        body_entered: prepared.comparison == Some(true),
        ..base_route
    };
    if !next_transition_fits(state, predecessor, route) {
        return None;
    }
    let ordinal = state.transition_count + 1;
    let snapshot = snapshot::build_snapshot(
        predecessor,
        route,
        prepared.cooling_sensible_output_w,
        prepared.maximum_total_cooling_capacity_w,
        prepared.comparison,
    );
    state.transition_count = ordinal;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(ordinal);
    Some(snapshot)
}

fn route_matches_predecessor_bounded(predecessor: Predecessor, route: Route) -> bool {
    let cp420_route = PredecessorRoute {
        logical_index: route.logical_index,
        predecessor_guard_false_fallthrough: predecessor
            .saturation_supply_humidity_ratio_guard_false_fallthrough,
        predecessor_guard_body_entered: predecessor
            .saturation_supply_humidity_ratio_guard_body_entered,
        predecessor_saturation_temperature_assignment_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed,
        predecessor_saturation_temperature_mixed_air_limit_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed,
        predecessor_supply_humidity_ratio_assignment_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed,
        active: route.active,
    };
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_route_matches_snapshot(
        predecessor,
        cp420_route,
    )
}

struct PreparedGuard {
    cooling_sensible_output_w: Option<f64>,
    maximum_total_cooling_capacity_w: Option<f64>,
    comparison: Option<bool>,
}

fn prepare_guard(
    predecessor: Predecessor,
    route: Route,
    input: Option<ActiveInput>,
) -> Option<PreparedGuard> {
    if !route.active {
        return (input.is_none() && predecessor.cooling_sensible_output_w.is_none()).then_some(
            PreparedGuard {
                cooling_sensible_output_w: None,
                maximum_total_cooling_capacity_w: None,
                comparison: None,
            },
        );
    }
    let input = input?;
    let predecessor_output = predecessor.cooling_sensible_output_w?;
    if !input.cp420_cooling_sensible_output_owned_read
        || !input.cp321_maximum_total_cooling_capacity_owned_read
        || !input.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
        || input.cooling_sensible_output_w.to_bits() != predecessor_output.to_bits()
    {
        return None;
    }
    Some(PreparedGuard {
        cooling_sensible_output_w: Some(input.cooling_sensible_output_w),
        maximum_total_cooling_capacity_w: Some(input.maximum_total_cooling_capacity_w),
        comparison: Some(source_greater_than_or_equal(
            input.cooling_sensible_output_w,
            input.maximum_total_cooling_capacity_w,
        )),
    })
}

pub(super) fn source_greater_than_or_equal(left: f64, right: f64) -> bool {
    left >= right
}
