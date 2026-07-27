//! Pure CP314 warning-and-clamp transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState, PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingOaMaxFlowGateSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_oa_max_flow_body_state(
    state: &mut PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    predecessor: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    outdoor_air_mass_flow_rate_before_clamp_kg_per_s: f64,
    standard_air_density_kg_per_m3: f64,
    maximum_cooling_air_volume_flow_rate_m3_per_s: f64,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcCoolingOaMaxFlowBodySnapshot {
    state.transition_count += 1;
    let body_entered = predecessor.maximum_cooling_flow_body_entered;
    let body_skipped = !body_entered;

    let outdoor_air_mass_flow_rate_value_kg_per_s =
        outdoor_air_mass_flow_rate_before_clamp_kg_per_s;
    let maximum_cooling_air_mass_flow_rate_value_kg_per_s =
        maximum_cooling_air_mass_flow_rate_kg_per_s;
    let outdoor_air_mass_flow_rate_before_clamp_kg_per_s =
        body_entered.then_some(outdoor_air_mass_flow_rate_value_kg_per_s);
    let maximum_cooling_air_mass_flow_rate_kg_per_s =
        body_entered.then_some(maximum_cooling_air_mass_flow_rate_value_kg_per_s);
    let outdoor_air_volume_flow_rate_m3_per_s = body_entered
        .then(|| outdoor_air_mass_flow_rate_value_kg_per_s / standard_air_density_kg_per_m3);
    let warning_counter_before =
        body_entered.then_some(state.outdoor_air_flow_max_cooling_output_error_count);
    let first_warning_predicate_satisfied = warning_counter_before.map(|counter| counter < 1);
    let first_warning_branch_entered = first_warning_predicate_satisfied == Some(true);
    let recurring_warning_branch_entered = first_warning_predicate_satisfied == Some(false);
    let unit_off_skipped = body_skipped && !predecessor.unit_body_entered;
    let non_cooling_skipped = body_skipped
        && predecessor.unit_body_entered
        && !predecessor.predecessor_cooling_body_entered;
    let active_guard_false_economizer_fallthrough =
        body_skipped && predecessor.predecessor_cooling_body_entered;

    if first_warning_branch_entered {
        state.outdoor_air_flow_max_cooling_output_error_count += 1;
    }
    let warning_counter_after =
        body_entered.then_some(state.outdoor_air_flow_max_cooling_output_error_count);

    let characterized_recurring_warning_index_allocated_on_call =
        recurring_warning_branch_entered && !state.characterized_recurring_warning_index_allocated;
    let characterized_recurring_warning_index_reused_on_call =
        recurring_warning_branch_entered && state.characterized_recurring_warning_index_allocated;
    let mut characterized_recurring_warning_occurrence_ordinal = None;
    let mut characterized_recurring_warning_report_maximum_m3_per_s = None;
    let characterized_recurring_warning_index_before =
        recurring_warning_branch_entered.then_some(state.outdoor_air_flow_max_cooling_output_index);
    if let (true, Some(value)) = (
        recurring_warning_branch_entered,
        outdoor_air_volume_flow_rate_m3_per_s,
    ) {
        state.characterized_recurring_warning_index_allocated = true;
        state.outdoor_air_flow_max_cooling_output_index = 1;
        state.characterized_recurring_warning_occurrence_count += 1;
        let maximum = state
            .characterized_recurring_warning_report_maximum_m3_per_s
            .map_or(value, |retained| source_max(value, retained));
        state.characterized_recurring_warning_report_maximum_m3_per_s = Some(maximum);
        characterized_recurring_warning_occurrence_ordinal =
            Some(state.characterized_recurring_warning_occurrence_count);
        characterized_recurring_warning_report_maximum_m3_per_s = Some(maximum);
    }
    let characterized_recurring_warning_index_after =
        recurring_warning_branch_entered.then_some(state.outdoor_air_flow_max_cooling_output_index);

    if body_entered {
        state.body_entry_count += 1;
        state.outdoor_air_mass_flow_rate_read_count += 1;
        state.standard_air_density_read_count += 1;
        state.outdoor_air_volume_flow_calculation_count += 1;
        state.warning_counter_read_count += 1;
        state.maximum_cooling_air_mass_flow_rate_read_count += 1;
        state.outdoor_air_mass_flow_clamp_assignment_count += 1;
        state.characterized_total_warning_error_increment_count += 1;
        if first_warning_branch_entered {
            state.first_warning_branch_count += 1;
            state.warning_counter_increment_count += 1;
            state.first_warning_call_site_count += 1;
            state.maximum_cooling_air_volume_flow_rate_read_count += 1;
            state.continue_warning_call_site_count += 1;
            state.continue_warning_timestamp_call_site_count += 1;
        } else {
            state.recurring_warning_branch_count += 1;
            state.recurring_warning_call_site_count += 1;
            if characterized_recurring_warning_index_allocated_on_call {
                state.characterized_recurring_warning_index_allocation_count += 1;
            } else {
                state.characterized_recurring_warning_index_reuse_count += 1;
            }
        }
    } else {
        state.body_skip_count += 1;
        if unit_off_skipped {
            state.unit_off_skip_count += 1;
        } else if non_cooling_skipped {
            state.non_cooling_skip_count += 1;
        } else {
            state.active_guard_false_economizer_fallthrough_count += 1;
        }
    }

    let snapshot = PurchasedAirCalcCoolingOaMaxFlowBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
        recurring_warning_child_source:
            PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_maximum_cooling_flow_body_entered: body_entered,
        body_skipped,
        unit_off_skipped,
        non_cooling_skipped,
        active_guard_false_economizer_fallthrough,
        outdoor_air_mass_flow_rate_read: body_entered,
        outdoor_air_mass_flow_rate_before_clamp_kg_per_s,
        standard_air_density_read: body_entered,
        standard_air_density_kg_per_m3: body_entered.then_some(standard_air_density_kg_per_m3),
        outdoor_air_volume_flow_rate_calculated: body_entered,
        outdoor_air_volume_flow_rate_m3_per_s,
        warning_counter_read: body_entered,
        warning_counter_before,
        first_warning_predicate_satisfied,
        first_warning_branch_entered,
        warning_counter_incremented: first_warning_branch_entered,
        warning_counter_after,
        first_warning_call_site_reached: first_warning_branch_entered,
        maximum_cooling_air_volume_flow_rate_read: first_warning_branch_entered,
        maximum_cooling_air_volume_flow_rate_m3_per_s: first_warning_branch_entered
            .then_some(maximum_cooling_air_volume_flow_rate_m3_per_s),
        continue_warning_call_site_reached: first_warning_branch_entered,
        continue_warning_timestamp_call_site_reached: first_warning_branch_entered,
        recurring_warning_branch_entered,
        recurring_warning_call_site_reached: recurring_warning_branch_entered,
        recurring_warning_report_maximum_input_m3_per_s: recurring_warning_branch_entered
            .then_some(outdoor_air_volume_flow_rate_m3_per_s)
            .flatten(),
        characterized_recurring_warning_index_allocated_on_call,
        characterized_recurring_warning_index_reused_on_call,
        characterized_recurring_warning_index_before,
        characterized_recurring_warning_index_after,
        characterized_recurring_warning_occurrence_ordinal,
        characterized_recurring_warning_report_maximum_m3_per_s,
        characterized_total_warning_error_incremented: body_entered,
        maximum_cooling_air_mass_flow_rate_read: body_entered,
        maximum_cooling_air_mass_flow_rate_kg_per_s,
        outdoor_air_mass_flow_clamp_assignment_performed: body_entered,
        outdoor_air_mass_flow_rate_after_clamp_kg_per_s:
            maximum_cooling_air_mass_flow_rate_kg_per_s,
    };
    state.latest = Some(snapshot);
    snapshot
}

fn source_max(left: f64, right: f64) -> f64 {
    if left < right { right } else { left }
}
