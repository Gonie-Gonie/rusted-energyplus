//! Pure CP382-to-CP383 post-saturation total-output capacity guard.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Predecessor;

mod accounting;
mod routes;

use accounting::{increment_counts, next_transition_fits};
pub(in crate::ideal_loads::calc) use routes::{
    PredecessorRoute, predecessor_route, predecessor_route_is_active,
};
use routes::retained_route;

/// Release-validated same-call numerical owners for line 2268.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput
{
    pub cooling_total_output_w: f64,
    pub maximum_total_cooling_capacity_w: f64,
    pub cp382_cooling_total_output_owned_read: bool,
    pub cp321_maximum_total_cooling_capacity_owned_read: bool,
    pub cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: bool,
}

struct PreparedGuard {
    cooling_total_output_w: Option<f64>,
    maximum_total_cooling_capacity_w: Option<f64>,
    strictly_greater: Option<bool>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput,
    >,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let active = predecessor_route_is_active(predecessor_route);
    let prepared = prepare_guard(active, predecessor.cooling_total_output_w, input)?;
    let body_entered = prepared.strictly_greater == Some(true);
    let guard_false = prepared.strictly_greater == Some(false);
    let route = retained_route(predecessor_route, body_entered)?;
    if !next_transition_fits(state, predecessor_route, route, body_entered) {
        return None;
    }

    state.transition_count += 1;
    increment_counts(state, predecessor_route, route, active, body_entered);

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor
            .dehumidification_total_output_assignment_executed,
        dehumidification_total_output_capacity_guard_evaluated: active,
        cp382_cooling_total_output_owned_read: active,
        cooling_total_output_read: active,
        cooling_total_output_w: prepared.cooling_total_output_w,
        cp321_maximum_total_cooling_capacity_owned_read: active,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: active,
        maximum_total_cooling_capacity_read: active,
        maximum_total_cooling_capacity_w: prepared.maximum_total_cooling_capacity_w,
        cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated: active,
        cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity: prepared
            .strictly_greater,
        dehumidification_total_output_capacity_adjustment_body_entered: body_entered,
        dehumidification_total_output_capacity_guard_false_fallthrough: guard_false,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn prepare_guard(
    active: bool,
    predecessor_cooling_total_output_w: Option<f64>,
    input: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput,
    >,
) -> Option<PreparedGuard> {
    if !active {
        return (input.is_none() && predecessor_cooling_total_output_w.is_none()).then_some(PreparedGuard {
            cooling_total_output_w: None,
            maximum_total_cooling_capacity_w: None,
            strictly_greater: None,
        });
    }
    let input = input?;
    let predecessor_output = predecessor_cooling_total_output_w?;
    if !input.cp382_cooling_total_output_owned_read
        || !input.cp321_maximum_total_cooling_capacity_owned_read
        || !input.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
    {
        return None;
    }
    if input.cooling_total_output_w.to_bits() != predecessor_output.to_bits() {
        return None;
    }
    Some(PreparedGuard {
        cooling_total_output_w: Some(input.cooling_total_output_w),
        maximum_total_cooling_capacity_w: Some(input.maximum_total_cooling_capacity_w),
        strictly_greater: Some(
            input.cooling_total_output_w > input.maximum_total_cooling_capacity_w,
        ),
    })
}
