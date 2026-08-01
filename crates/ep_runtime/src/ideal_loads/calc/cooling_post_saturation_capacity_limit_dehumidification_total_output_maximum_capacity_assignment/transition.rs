//! Pure CP383-to-CP384 maximum-capacity assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Predecessor;

mod accounting;
mod routes;

use accounting::{increment_counts, next_transition_fits};
pub(in crate::ideal_loads::calc) use routes::{
    PredecessorRoute, predecessor_route, predecessor_route_is_assignment,
};
use routes::retained_route;

struct PreparedValues {
    preexisting: Option<f64>,
    maximum: Option<f64>,
    assigned: Option<f64>,
    resulting: Option<f64>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let assignment = predecessor_route_is_assignment(predecessor_route);
    let guard_evaluated = predecessor.dehumidification_total_output_capacity_guard_evaluated;
    let guard_false = predecessor
        .dehumidification_total_output_capacity_guard_false_fallthrough;
    let prepared = prepare_values(predecessor, guard_evaluated, assignment)?;
    let route = retained_route(predecessor_route);
    if !next_transition_fits(state, predecessor_route, route, assignment) {
        return None;
    }

    state.transition_count += 1;
    increment_counts(state, predecessor_route, route, guard_evaluated, guard_false, assignment);

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor.dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: guard_false,
        dehumidification_total_output_capacity_guard_false_fallthrough: guard_false,
        dehumidification_total_output_maximum_capacity_assignment_executed: assignment,
        preexisting_cooling_total_output_w: prepared.preexisting,
        cp383_retained_maximum_total_cooling_capacity_owned_read: assignment,
        maximum_total_cooling_capacity_read: assignment,
        maximum_total_cooling_capacity_w: prepared.maximum,
        cooling_total_output_assigned: assignment,
        assigned_cooling_total_output_w: prepared.assigned,
        resulting_cooling_total_output_w: prepared.resulting,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn prepare_values(
    predecessor: Predecessor,
    guard_evaluated: bool,
    assignment: bool,
) -> Option<PreparedValues> {
    if !guard_evaluated {
        return Some(PreparedValues {
            preexisting: None,
            maximum: None,
            assigned: None,
            resulting: None,
        });
    }
    let preexisting = predecessor.cooling_total_output_w?;
    if !assignment {
        return Some(PreparedValues {
            preexisting: Some(preexisting),
            maximum: None,
            assigned: None,
            resulting: Some(preexisting),
        });
    }
    let maximum = predecessor.maximum_total_cooling_capacity_w?;
    Some(PreparedValues {
        preexisting: Some(preexisting),
        maximum: Some(maximum),
        assigned: Some(maximum),
        resulting: Some(maximum),
    })
}
