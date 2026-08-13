//! Pure CP421-to-CP422 maximum-capacity assignment transition.

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute as PredecessorRoute;

mod accounting;
mod snapshot;

use accounting::{increment_counts, next_transition_fits};

/// One retained CP422 successor route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute {
    pub logical_index: usize,
    pub active: bool,
    pub assignment_executed: bool,
}

/// Release-validated same-call values for line 2333.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentActiveInput {
    pub preexisting_cooling_sensible_output_w: f64,
    pub maximum_total_cooling_capacity_w: f64,
    pub cp421_retained_maximum_total_cooling_capacity_owned_read: bool,
}

use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentActiveInput as ActiveInput;
use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_route_from_committed_predecessor(
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
) -> Option<Route> {
    let active = matches!(predecessor_route.logical_index, 4 | 7 | 10 | 13 | 16);
    let assignment_executed = active && predecessor_route.body_entered;
    (predecessor_route.logical_index < 36
        && predecessor_route.active == active
        && predecessor
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
            == active
        && predecessor
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
            == assignment_executed
        && predecessor
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough
            == (active && !assignment_executed))
        .then_some(Route {
            logical_index: predecessor_route.logical_index,
            active,
            assignment_executed,
        })
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let predecessor_route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot_route(predecessor)?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_route_from_committed_predecessor(predecessor, predecessor_route)?;
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_state_with_validated_route(state, predecessor, route, input)
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_state_with_validated_route(
    state: &mut State,
    predecessor: Predecessor,
    route: Route,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system || !route_matches_predecessor(predecessor, route) {
        return None;
    }
    let prepared = prepare_values(predecessor, route, input)?;
    if !next_transition_fits(state, predecessor, route) {
        return None;
    }
    let ordinal = state.transition_count + 1;
    let result = if route.assignment_executed {
        prepared.maximum_total_cooling_capacity_w
    } else {
        prepared.preexisting_cooling_sensible_output_w
    };
    let snapshot = snapshot::build_snapshot(
        predecessor,
        route,
        prepared.preexisting_cooling_sensible_output_w,
        prepared.maximum_total_cooling_capacity_w,
        result,
    );
    state.transition_count = ordinal;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(ordinal);
    Some(snapshot)
}

fn route_matches_predecessor(predecessor: Predecessor, route: Route) -> bool {
    if route.logical_index >= 36 {
        return false;
    }
    let expected_active = matches!(route.logical_index, 4 | 7 | 10 | 13 | 16);
    let raw_comparison = if expected_active {
        let (Some(output), Some(capacity)) = (
            predecessor.cp420_cooling_sensible_output_for_capacity_guard_w,
            predecessor.maximum_total_cooling_capacity_w,
        ) else {
            return false;
        };
        Some(output >= capacity)
    } else {
        None
    };
    let expected_assignment = raw_comparison == Some(true);
    route.active == expected_active
        && route.assignment_executed == expected_assignment
        && predecessor
            .cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity
            == raw_comparison
        && predecessor
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
            == expected_active
        && predecessor
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
            == expected_assignment
        && predecessor
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough
            == (expected_active && !expected_assignment)
}

struct PreparedValues {
    preexisting_cooling_sensible_output_w: Option<f64>,
    maximum_total_cooling_capacity_w: Option<f64>,
}

fn prepare_values(
    predecessor: Predecessor,
    route: Route,
    input: Option<ActiveInput>,
) -> Option<PreparedValues> {
    if !route.active {
        return input.is_none().then_some(PreparedValues {
            preexisting_cooling_sensible_output_w: None,
            maximum_total_cooling_capacity_w: None,
        });
    }
    let input = input?;
    let predecessor_output = predecessor.cp420_cooling_sensible_output_for_capacity_guard_w?;
    let predecessor_capacity = predecessor.maximum_total_cooling_capacity_w?;
    if !input.cp421_retained_maximum_total_cooling_capacity_owned_read
        || input.preexisting_cooling_sensible_output_w.to_bits() != predecessor_output.to_bits()
        || input.maximum_total_cooling_capacity_w.to_bits() != predecessor_capacity.to_bits()
    {
        return None;
    }
    Some(PreparedValues {
        preexisting_cooling_sensible_output_w: Some(input.preexisting_cooling_sensible_output_w),
        maximum_total_cooling_capacity_w: route
            .assignment_executed
            .then_some(input.maximum_total_cooling_capacity_w),
    })
}
