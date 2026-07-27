//! Pure CP314 predecessor and CP315 retained-state validation.

use ep_model::{IdealLoadsLimit, OutdoorAirEconomizerType, ZoneId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
    PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState, PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState, PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
};

pub(super) fn cooling_oa_max_flow_body_snapshots_bitwise_equal(
    retained: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    supplied: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
) -> bool {
    let floats_match = [
        (
            retained.outdoor_air_mass_flow_rate_before_clamp_kg_per_s,
            supplied.outdoor_air_mass_flow_rate_before_clamp_kg_per_s,
        ),
        (
            retained.standard_air_density_kg_per_m3,
            supplied.standard_air_density_kg_per_m3,
        ),
        (
            retained.outdoor_air_volume_flow_rate_m3_per_s,
            supplied.outdoor_air_volume_flow_rate_m3_per_s,
        ),
        (
            retained.maximum_cooling_air_volume_flow_rate_m3_per_s,
            supplied.maximum_cooling_air_volume_flow_rate_m3_per_s,
        ),
        (
            retained.recurring_warning_report_maximum_input_m3_per_s,
            supplied.recurring_warning_report_maximum_input_m3_per_s,
        ),
        (
            retained.characterized_recurring_warning_report_maximum_m3_per_s,
            supplied.characterized_recurring_warning_report_maximum_m3_per_s,
        ),
        (
            retained.maximum_cooling_air_mass_flow_rate_kg_per_s,
            supplied.maximum_cooling_air_mass_flow_rate_kg_per_s,
        ),
        (
            retained.outdoor_air_mass_flow_rate_after_clamp_kg_per_s,
            supplied.outdoor_air_mass_flow_rate_after_clamp_kg_per_s,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_f64_bits_equal(left, right));
    if !floats_match {
        return false;
    }

    let mut retained_without_floats = retained;
    let mut supplied_without_floats = supplied;
    retained_without_floats.outdoor_air_mass_flow_rate_before_clamp_kg_per_s = None;
    retained_without_floats.standard_air_density_kg_per_m3 = None;
    retained_without_floats.outdoor_air_volume_flow_rate_m3_per_s = None;
    retained_without_floats.maximum_cooling_air_volume_flow_rate_m3_per_s = None;
    retained_without_floats.recurring_warning_report_maximum_input_m3_per_s = None;
    retained_without_floats.characterized_recurring_warning_report_maximum_m3_per_s = None;
    retained_without_floats.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
    retained_without_floats.outdoor_air_mass_flow_rate_after_clamp_kg_per_s = None;
    supplied_without_floats.outdoor_air_mass_flow_rate_before_clamp_kg_per_s = None;
    supplied_without_floats.standard_air_density_kg_per_m3 = None;
    supplied_without_floats.outdoor_air_volume_flow_rate_m3_per_s = None;
    supplied_without_floats.maximum_cooling_air_volume_flow_rate_m3_per_s = None;
    supplied_without_floats.recurring_warning_report_maximum_input_m3_per_s = None;
    supplied_without_floats.characterized_recurring_warning_report_maximum_m3_per_s = None;
    supplied_without_floats.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
    supplied_without_floats.outdoor_air_mass_flow_rate_after_clamp_kg_per_s = None;
    retained_without_floats == supplied_without_floats
}

pub(super) fn predecessor_links_to_gate(
    predecessor: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    gate: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
) -> bool {
    predecessor.system == gate.system
        && predecessor.parent_call_ordinal == gate.parent_call_ordinal
        && predecessor.controlled_zone == gate.controlled_zone
        && predecessor.unit_body_entered == gate.unit_body_entered
        && predecessor.predecessor_cooling_body_entered == gate.predecessor_cooling_body_entered
        && predecessor.predecessor_maximum_cooling_flow_body_entered
            == gate.maximum_cooling_flow_body_entered
        && predecessor.body_skipped != gate.maximum_cooling_flow_body_entered
        && predecessor.unit_off_skipped == gate.unit_off_skipped
        && predecessor.non_cooling_skipped == gate.non_cooling_skipped
        && predecessor.active_guard_false_economizer_fallthrough
            == (gate.predecessor_cooling_body_entered && !gate.maximum_cooling_flow_body_entered)
}

pub(super) fn predecessor_is_exact_direct_release(
    predecessor: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
) -> bool {
    let provenance = predecessor.source == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
        && predecessor.recurring_warning_child_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER;
    if !provenance
        || !predecessor.body_skipped
        || predecessor.predecessor_maximum_cooling_flow_body_entered
        || !body_sites_are_skipped(predecessor)
    {
        return false;
    }

    match (
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.active_guard_false_economizer_fallthrough,
    ) {
        (true, false, false) => {
            !predecessor.unit_body_entered && !predecessor.predecessor_cooling_body_entered
        }
        (false, true, false) => {
            predecessor.unit_body_entered && !predecessor.predecessor_cooling_body_entered
        }
        (false, false, true) => {
            predecessor.unit_body_entered && predecessor.predecessor_cooling_body_entered
        }
        _ => false,
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn direct_runtime_states_are_consistent(
    guard: &PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    body: &PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    gate: &PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    predecessor: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    gate_latest: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    cooling_limit: IdealLoadsLimit,
    retained_maximum_mass_flow_rate_kg_per_s: f64,
    controlled_zone: Option<ZoneId>,
) -> bool {
    if guard.system != predecessor.system
        || body.system != predecessor.system
        || gate.system != predecessor.system
        || controlled_zone != Some(predecessor.controlled_zone)
        || !retained_maximum_mass_flow_rate_kg_per_s.is_finite()
        || retained_maximum_mass_flow_rate_kg_per_s < 0.0
        || !gate_snapshot_is_direct_release(
            gate_latest,
            cooling_limit,
            retained_maximum_mass_flow_rate_kg_per_s,
        )
    {
        return false;
    }

    let gate_selector_history = match cooling_limit {
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
    let expected_flow_reads = if matches!(
        cooling_limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    ) {
        gate.source_execution_count
    } else {
        0
    };
    let gate_history = gate_selector_history
        && gate
            .source_execution_count
            .checked_add(gate.unit_off_skip_count)
            .and_then(|count| count.checked_add(gate.non_cooling_skip_count))
            == Some(gate.transition_count)
        && gate.cooling_limit_flow_rate_comparison_count == gate.source_execution_count
        && gate
            .source_execution_count
            .checked_sub(gate.cooling_limit_flow_rate_match_count)
            == Some(gate.cooling_limit_flow_rate_and_capacity_comparison_count)
        && gate
            .cooling_limit_flow_rate_match_count
            .checked_add(gate.cooling_limit_flow_rate_and_capacity_match_count)
            == Some(gate.outdoor_air_mass_flow_rate_read_count)
        && gate.outdoor_air_mass_flow_rate_read_count == expected_flow_reads
        && gate.outdoor_air_mass_flow_rate_read_count
            == gate.maximum_cooling_air_mass_flow_rate_read_count
        && gate.maximum_cooling_air_mass_flow_rate_read_count
            == gate.strict_mass_flow_comparison_count
        && gate.strict_mass_flow_comparison_satisfied_count == 0
        && gate.maximum_cooling_flow_body_entry_count == 0
        && gate.active_fallthrough_count == gate.source_execution_count;

    let body_skip_partition = body
        .unit_off_skip_count
        .checked_add(body.non_cooling_skip_count)
        .and_then(|count| count.checked_add(body.active_guard_false_economizer_fallthrough_count))
        == Some(body.body_skip_count);
    let body_history = body.transition_count == gate.transition_count
        && body.transition_count == body.body_skip_count
        && body.body_entry_count == 0
        && body_skip_partition
        && body.unit_off_skip_count == gate.unit_off_skip_count
        && body.non_cooling_skip_count == gate.non_cooling_skip_count
        && body.active_guard_false_economizer_fallthrough_count == gate.source_execution_count
        && body_effect_counters_are_zero(body);

    let guard_partition = guard
        .guard_evaluation_count
        .checked_add(guard.unit_off_skip_count)
        .and_then(|count| count.checked_add(guard.non_cooling_skip_count))
        .and_then(|count| count.checked_add(guard.maximum_cooling_flow_body_sibling_skip_count))
        == Some(guard.transition_count);
    let guard_latest_is_valid = match (guard.transition_count, guard.latest) {
        (0, None) => true,
        (0, Some(_)) | (_, None) => false,
        (count, Some(latest)) => {
            latest.parent_call_ordinal == count
                && latest.system == guard.system
                && controlled_zone == Some(latest.controlled_zone)
                && guard_snapshot_is_exact_direct_release(latest)
                && guard_latest_route_is_recorded(guard, latest)
        }
    };
    let guard_history = guard_partition
        && guard.guard_evaluation_count == guard.economizer_type_read_count
        && guard.economizer_type_read_count == guard.no_economizer_comparison_count
        && guard.economizer_body_entry_count == 0
        && guard.no_economizer_fallthrough_count == guard.guard_evaluation_count
        && guard_latest_is_valid;

    let histories_link = guard
        .unit_off_skip_count
        .checked_add(usize::from(predecessor.unit_off_skipped))
        == Some(body.unit_off_skip_count)
        && guard
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(body.non_cooling_skip_count)
        && guard
            .maximum_cooling_flow_body_sibling_skip_count
            .checked_add(usize::from(
                predecessor.predecessor_maximum_cooling_flow_body_entered,
            ))
            == Some(body.body_entry_count)
        && guard.guard_evaluation_count.checked_add(usize::from(
            predecessor.active_guard_false_economizer_fallthrough,
        )) == Some(body.active_guard_false_economizer_fallthrough_count);

    gate_history && body_history && guard_history && histories_link
}

fn gate_snapshot_is_direct_release(
    snapshot: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    cooling_limit: IdealLoadsLimit,
    retained_maximum_mass_flow_rate_kg_per_s: f64,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER;
    if !provenance || snapshot.maximum_cooling_flow_body_entered {
        return false;
    }
    if !snapshot.predecessor_cooling_body_entered {
        return snapshot.unit_off_skipped != snapshot.unit_body_entered
            && snapshot.non_cooling_skipped == snapshot.unit_body_entered
            && gate_sites_are_skipped(snapshot);
    }
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || !snapshot.unit_body_entered
        || !snapshot.cooling_limit_flow_rate_comparison_evaluated
        || !snapshot.cooling_limit_flow_rate_read
        || snapshot.cooling_limit_flow_rate_value != Some(cooling_limit)
    {
        return false;
    }

    let first_match = cooling_limit == IdealLoadsLimit::LimitFlowRate;
    let second_match = cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let flow_active = first_match || second_match;
    let selector_shape = snapshot.cooling_limit_flow_rate_comparison_satisfied == Some(first_match)
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated != first_match
        && snapshot.cooling_limit_flow_rate_and_capacity_read != first_match
        && snapshot.cooling_limit_flow_rate_and_capacity_value
            == (!first_match).then_some(cooling_limit)
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_satisfied
            == (!first_match).then_some(second_match)
        && snapshot.cooling_flow_limit_active == Some(flow_active);
    if !selector_shape {
        return false;
    }
    if flow_active {
        snapshot.outdoor_air_mass_flow_rate_read
            && option_f64_has_bits(snapshot.outdoor_air_mass_flow_rate_kg_per_s, 0.0)
            && snapshot.maximum_cooling_air_mass_flow_rate_read
            && option_f64_has_bits(
                snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
                retained_maximum_mass_flow_rate_kg_per_s,
            )
            && snapshot.strict_mass_flow_comparison_evaluated
            && snapshot.outdoor_air_mass_flow_above_maximum == Some(false)
    } else {
        !snapshot.outdoor_air_mass_flow_rate_read
            && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.maximum_cooling_air_mass_flow_rate_read
            && snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.strict_mass_flow_comparison_evaluated
            && snapshot.outdoor_air_mass_flow_above_maximum.is_none()
    }
}

fn guard_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER;
    if !provenance {
        return false;
    }

    if snapshot.economizer_guard_evaluated {
        snapshot.unit_body_entered
            && snapshot.predecessor_cooling_body_entered
            && !snapshot.predecessor_maximum_cooling_flow_body_entered
            && snapshot.predecessor_active_guard_false_economizer_fallthrough
            && !snapshot.unit_off_skipped
            && !snapshot.non_cooling_skipped
            && !snapshot.maximum_cooling_flow_body_sibling_skipped
            && snapshot.economizer_type_read
            && snapshot.economizer_type == Some(OutdoorAirEconomizerType::NoEconomizer)
            && snapshot.no_economizer_comparison_evaluated
            && snapshot.economizer_not_no_economizer == Some(false)
            && !snapshot.economizer_body_entered
            && snapshot.no_economizer_fallthrough
    } else {
        let skip_partition = usize::from(snapshot.unit_off_skipped)
            + usize::from(snapshot.non_cooling_skipped)
            + usize::from(snapshot.maximum_cooling_flow_body_sibling_skipped)
            == 1;
        skip_partition
            && !snapshot.predecessor_active_guard_false_economizer_fallthrough
            && !snapshot.economizer_type_read
            && snapshot.economizer_type.is_none()
            && !snapshot.no_economizer_comparison_evaluated
            && snapshot.economizer_not_no_economizer.is_none()
            && !snapshot.economizer_body_entered
            && !snapshot.no_economizer_fallthrough
            && if snapshot.unit_off_skipped {
                !snapshot.unit_body_entered
                    && !snapshot.predecessor_cooling_body_entered
                    && !snapshot.predecessor_maximum_cooling_flow_body_entered
            } else if snapshot.non_cooling_skipped {
                snapshot.unit_body_entered
                    && !snapshot.predecessor_cooling_body_entered
                    && !snapshot.predecessor_maximum_cooling_flow_body_entered
            } else {
                snapshot.unit_body_entered
                    && snapshot.predecessor_cooling_body_entered
                    && snapshot.predecessor_maximum_cooling_flow_body_entered
            }
    }
}

fn guard_latest_route_is_recorded(
    state: &PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    latest: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> bool {
    if latest.economizer_guard_evaluated {
        state.guard_evaluation_count > 0
    } else if latest.unit_off_skipped {
        state.unit_off_skip_count > 0
    } else if latest.non_cooling_skipped {
        state.non_cooling_skip_count > 0
    } else {
        latest.maximum_cooling_flow_body_sibling_skipped
            && state.maximum_cooling_flow_body_sibling_skip_count > 0
    }
}

fn body_sites_are_skipped(snapshot: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot) -> bool {
    !snapshot.outdoor_air_mass_flow_rate_read
        && snapshot
            .outdoor_air_mass_flow_rate_before_clamp_kg_per_s
            .is_none()
        && !snapshot.standard_air_density_read
        && snapshot.standard_air_density_kg_per_m3.is_none()
        && !snapshot.outdoor_air_volume_flow_rate_calculated
        && snapshot.outdoor_air_volume_flow_rate_m3_per_s.is_none()
        && !snapshot.warning_counter_read
        && snapshot.warning_counter_before.is_none()
        && snapshot.first_warning_predicate_satisfied.is_none()
        && !snapshot.first_warning_branch_entered
        && !snapshot.warning_counter_incremented
        && snapshot.warning_counter_after.is_none()
        && !snapshot.first_warning_call_site_reached
        && !snapshot.maximum_cooling_air_volume_flow_rate_read
        && snapshot
            .maximum_cooling_air_volume_flow_rate_m3_per_s
            .is_none()
        && !snapshot.continue_warning_call_site_reached
        && !snapshot.continue_warning_timestamp_call_site_reached
        && !snapshot.recurring_warning_branch_entered
        && !snapshot.recurring_warning_call_site_reached
        && snapshot
            .recurring_warning_report_maximum_input_m3_per_s
            .is_none()
        && !snapshot.characterized_recurring_warning_index_allocated_on_call
        && !snapshot.characterized_recurring_warning_index_reused_on_call
        && snapshot
            .characterized_recurring_warning_index_before
            .is_none()
        && snapshot
            .characterized_recurring_warning_index_after
            .is_none()
        && snapshot
            .characterized_recurring_warning_occurrence_ordinal
            .is_none()
        && snapshot
            .characterized_recurring_warning_report_maximum_m3_per_s
            .is_none()
        && !snapshot.characterized_total_warning_error_incremented
        && !snapshot.maximum_cooling_air_mass_flow_rate_read
        && snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.outdoor_air_mass_flow_clamp_assignment_performed
        && snapshot
            .outdoor_air_mass_flow_rate_after_clamp_kg_per_s
            .is_none()
}

fn body_effect_counters_are_zero(body: &PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState) -> bool {
    body.outdoor_air_mass_flow_rate_read_count == 0
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

fn gate_sites_are_skipped(snapshot: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot) -> bool {
    !snapshot.cooling_limit_flow_rate_comparison_evaluated
        && !snapshot.cooling_limit_flow_rate_read
        && snapshot.cooling_limit_flow_rate_value.is_none()
        && snapshot
            .cooling_limit_flow_rate_comparison_satisfied
            .is_none()
        && !snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated
        && !snapshot.cooling_limit_flow_rate_and_capacity_read
        && snapshot
            .cooling_limit_flow_rate_and_capacity_value
            .is_none()
        && snapshot
            .cooling_limit_flow_rate_and_capacity_comparison_satisfied
            .is_none()
        && snapshot.cooling_flow_limit_active.is_none()
        && !snapshot.outdoor_air_mass_flow_rate_read
        && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.maximum_cooling_air_mass_flow_rate_read
        && snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.strict_mass_flow_comparison_evaluated
        && snapshot.outdoor_air_mass_flow_above_maximum.is_none()
}

fn option_f64_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_f64_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
