//! Pure CP313 predecessor and CP314 retained-state validation.

use ep_model::IdealLoadsLimit;

use super::super::PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
    PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState, PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
};

pub(super) fn predecessor_is_exact_direct_fallthrough(
    predecessor: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    cooling_limit: IdealLoadsLimit,
    retained_maximum_mass_flow_rate_kg_per_s: f64,
) -> bool {
    let provenance = predecessor.source == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER;
    if !provenance || predecessor.maximum_cooling_flow_body_entered {
        return false;
    }
    if !predecessor.predecessor_cooling_body_entered {
        let skip_partition = predecessor.unit_off_skipped != predecessor.unit_body_entered
            && predecessor.non_cooling_skipped == predecessor.unit_body_entered;
        return skip_partition && predecessor_sites_are_skipped(predecessor);
    }
    if predecessor.unit_off_skipped
        || predecessor.non_cooling_skipped
        || !predecessor.unit_body_entered
        || !predecessor.cooling_limit_flow_rate_comparison_evaluated
        || !predecessor.cooling_limit_flow_rate_read
        || predecessor.cooling_limit_flow_rate_value != Some(cooling_limit)
    {
        return false;
    }
    let first_match = cooling_limit == IdealLoadsLimit::LimitFlowRate;
    let second_match = cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let flow_active = first_match || second_match;
    let selector_shape = predecessor.cooling_limit_flow_rate_comparison_satisfied
        == Some(first_match)
        && predecessor.cooling_limit_flow_rate_and_capacity_comparison_evaluated != first_match
        && predecessor.cooling_limit_flow_rate_and_capacity_read != first_match
        && predecessor.cooling_limit_flow_rate_and_capacity_value
            == (!first_match).then_some(cooling_limit)
        && predecessor.cooling_limit_flow_rate_and_capacity_comparison_satisfied
            == (!first_match).then_some(second_match)
        && predecessor.cooling_flow_limit_active == Some(flow_active);
    if !selector_shape {
        return false;
    }
    if flow_active {
        predecessor.outdoor_air_mass_flow_rate_read
            && option_f64_has_bits(predecessor.outdoor_air_mass_flow_rate_kg_per_s, 0.0)
            && predecessor.maximum_cooling_air_mass_flow_rate_read
            && option_f64_has_bits(
                predecessor.maximum_cooling_air_mass_flow_rate_kg_per_s,
                retained_maximum_mass_flow_rate_kg_per_s,
            )
            && predecessor.strict_mass_flow_comparison_evaluated
            && predecessor.outdoor_air_mass_flow_above_maximum == Some(false)
    } else {
        !predecessor.outdoor_air_mass_flow_rate_read
            && predecessor.outdoor_air_mass_flow_rate_kg_per_s.is_none()
            && !predecessor.maximum_cooling_air_mass_flow_rate_read
            && predecessor
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_none()
            && !predecessor.strict_mass_flow_comparison_evaluated
            && predecessor.outdoor_air_mass_flow_above_maximum.is_none()
    }
}

pub(super) fn direct_runtime_states_are_consistent(
    body: &PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    gate: &PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    predecessor: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let cp313_selector_history = match cooling_limit {
        IdealLoadsLimit::LimitFlowRate => {
            gate.cooling_limit_flow_rate_match_count == gate.source_execution_count
                && gate.cooling_limit_flow_rate_and_capacity_comparison_count == 0
                && gate.cooling_limit_flow_rate_and_capacity_match_count == 0
        }
        IdealLoadsLimit::LimitFlowRateAndCapacity => {
            gate.cooling_limit_flow_rate_match_count == 0
                && gate.cooling_limit_flow_rate_and_capacity_comparison_count
                    == gate.source_execution_count
                && gate.cooling_limit_flow_rate_and_capacity_match_count
                    == gate.source_execution_count
        }
        IdealLoadsLimit::NoLimit | IdealLoadsLimit::LimitCapacity => {
            gate.cooling_limit_flow_rate_match_count == 0
                && gate.cooling_limit_flow_rate_and_capacity_comparison_count
                    == gate.source_execution_count
                && gate.cooling_limit_flow_rate_and_capacity_match_count == 0
        }
    };
    let cp313_partitions = gate
        .source_execution_count
        .checked_add(gate.unit_off_skip_count)
        .and_then(|count| count.checked_add(gate.non_cooling_skip_count))
        == Some(gate.transition_count)
        && gate.cooling_limit_flow_rate_comparison_count == gate.source_execution_count
        && gate.outdoor_air_mass_flow_rate_read_count
            == if matches!(
                cooling_limit,
                IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
            ) {
                gate.source_execution_count
            } else {
                0
            }
        && gate.outdoor_air_mass_flow_rate_read_count
            == gate.maximum_cooling_air_mass_flow_rate_read_count
        && gate.maximum_cooling_air_mass_flow_rate_read_count
            == gate.strict_mass_flow_comparison_count
        && gate.strict_mass_flow_comparison_satisfied_count == 0
        && gate.maximum_cooling_flow_body_entry_count == 0
        && gate.active_fallthrough_count == gate.source_execution_count;
    let cp314_skip_partition = body
        .unit_off_skip_count
        .checked_add(body.non_cooling_skip_count)
        .and_then(|count| count.checked_add(body.active_guard_false_economizer_fallthrough_count))
        == Some(body.body_skip_count);
    cp313_selector_history
        && cp313_partitions
        && body.transition_count == body.body_skip_count
        && body.body_entry_count == 0
        && cp314_skip_partition
        && body
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(gate.unit_off_skip_count)
        && body
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(gate.non_cooling_skip_count)
        && body
            .active_guard_false_economizer_fallthrough_count
            .checked_add(usize::from(predecessor.predecessor_cooling_body_entered))
            == Some(gate.source_execution_count)
        && body.outdoor_air_mass_flow_rate_read_count == 0
        && body.standard_air_density_read_count == 0
        && body.outdoor_air_volume_flow_calculation_count == 0
        && body.warning_counter_read_count == 0
        && body.outdoor_air_flow_max_cooling_output_error_count == 0
        && body.first_warning_branch_count == 0
        && body.warning_counter_increment_count == 0
        && body.first_warning_call_site_count == 0
        && body.maximum_cooling_air_volume_flow_rate_read_count == 0
        && body.continue_warning_call_site_count == 0
        && body.continue_warning_timestamp_call_site_count == 0
        && body.recurring_warning_branch_count == 0
        && body.recurring_warning_call_site_count == 0
        && body.characterized_recurring_warning_index_allocation_count == 0
        && body.characterized_recurring_warning_index_reuse_count == 0
        && body.characterized_recurring_warning_occurrence_count == 0
        && !body.characterized_recurring_warning_index_allocated
        && body.outdoor_air_flow_max_cooling_output_index == 0
        && body
            .characterized_recurring_warning_report_maximum_m3_per_s
            .is_none()
        && body.characterized_total_warning_error_increment_count == 0
        && body.maximum_cooling_air_mass_flow_rate_read_count == 0
        && body.outdoor_air_mass_flow_clamp_assignment_count == 0
}

pub(super) fn gate_snapshots_bitwise_equal(
    retained: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    supplied: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
) -> bool {
    let floats_match = option_f64_bits_equal(
        retained.outdoor_air_mass_flow_rate_kg_per_s,
        supplied.outdoor_air_mass_flow_rate_kg_per_s,
    ) && option_f64_bits_equal(
        retained.maximum_cooling_air_mass_flow_rate_kg_per_s,
        supplied.maximum_cooling_air_mass_flow_rate_kg_per_s,
    );
    let mut retained_without_floats = retained;
    let mut supplied_without_floats = supplied;
    retained_without_floats.outdoor_air_mass_flow_rate_kg_per_s = None;
    retained_without_floats.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
    supplied_without_floats.outdoor_air_mass_flow_rate_kg_per_s = None;
    supplied_without_floats.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
    floats_match && retained_without_floats == supplied_without_floats
}

pub(super) fn option_f64_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn predecessor_sites_are_skipped(
    predecessor: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
) -> bool {
    !predecessor.cooling_limit_flow_rate_comparison_evaluated
        && !predecessor.cooling_limit_flow_rate_read
        && predecessor.cooling_limit_flow_rate_value.is_none()
        && predecessor
            .cooling_limit_flow_rate_comparison_satisfied
            .is_none()
        && !predecessor.cooling_limit_flow_rate_and_capacity_comparison_evaluated
        && !predecessor.cooling_limit_flow_rate_and_capacity_read
        && predecessor
            .cooling_limit_flow_rate_and_capacity_value
            .is_none()
        && predecessor
            .cooling_limit_flow_rate_and_capacity_comparison_satisfied
            .is_none()
        && predecessor.cooling_flow_limit_active.is_none()
        && !predecessor.outdoor_air_mass_flow_rate_read
        && predecessor.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !predecessor.maximum_cooling_air_mass_flow_rate_read
        && predecessor
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
        && !predecessor.strict_mass_flow_comparison_evaluated
        && predecessor.outdoor_air_mass_flow_above_maximum.is_none()
}

fn option_f64_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
