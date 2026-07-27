//! Run-summary evidence for the bounded PurchasedAir cooling OA maximum-flow body.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
    PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowGateSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;

pub(super) use serialization::lifecycle_json;

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary>,
    predecessor_lifecycle: Option<&PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling OA maximum-flow body evidence"
            .to_string()
    })?;
    let predecessor_lifecycle = predecessor_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow body has no gate evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow body has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow body has no coupling call count".to_string()
    })?;
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skip_partition = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )
    .and_then(|partial| {
        checked_add(
            partial,
            state.active_guard_false_economizer_fallthrough_count,
            "skip partition",
        )
    })?;
    let transition_partition = checked_add(
        state.body_entry_count,
        state.body_skip_count,
        "transition partition",
    )?;
    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
        || lifecycle.recurring_warning_child_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
        || predecessor_lifecycle.source != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling OA maximum-flow body provenance is invalid".to_string(),
        );
    }
    for (field, expected, actual) in [
        (
            "transition_count",
            coupling_call_count,
            state.transition_count,
        ),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "body_entry_count",
            predecessor.maximum_cooling_flow_body_entry_count,
            state.body_entry_count,
        ),
        ("direct_body_entry_count", 0, state.body_entry_count),
        (
            "body_skip_count",
            coupling_call_count,
            state.body_skip_count,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "active_guard_false_economizer_fallthrough_count",
            predecessor.active_fallthrough_count,
            state.active_guard_false_economizer_fallthrough_count,
        ),
        ("skip_partition", state.body_skip_count, skip_partition),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "outdoor_air_mass_flow_rate_read_count",
            0,
            state.outdoor_air_mass_flow_rate_read_count,
        ),
        (
            "standard_air_density_read_count",
            0,
            state.standard_air_density_read_count,
        ),
        (
            "outdoor_air_volume_flow_calculation_count",
            0,
            state.outdoor_air_volume_flow_calculation_count,
        ),
        (
            "warning_counter_read_count",
            0,
            state.warning_counter_read_count,
        ),
        (
            "outdoor_air_flow_max_cooling_output_error_count",
            0,
            state.outdoor_air_flow_max_cooling_output_error_count,
        ),
        (
            "first_warning_branch_count",
            0,
            state.first_warning_branch_count,
        ),
        (
            "warning_counter_increment_count",
            0,
            state.warning_counter_increment_count,
        ),
        (
            "first_warning_call_site_count",
            0,
            state.first_warning_call_site_count,
        ),
        (
            "maximum_cooling_air_volume_flow_rate_read_count",
            0,
            state.maximum_cooling_air_volume_flow_rate_read_count,
        ),
        (
            "continue_warning_call_site_count",
            0,
            state.continue_warning_call_site_count,
        ),
        (
            "continue_warning_timestamp_call_site_count",
            0,
            state.continue_warning_timestamp_call_site_count,
        ),
        (
            "recurring_warning_branch_count",
            0,
            state.recurring_warning_branch_count,
        ),
        (
            "recurring_warning_call_site_count",
            0,
            state.recurring_warning_call_site_count,
        ),
        (
            "characterized_recurring_warning_index_allocation_count",
            0,
            state.characterized_recurring_warning_index_allocation_count,
        ),
        (
            "characterized_recurring_warning_index_reuse_count",
            0,
            state.characterized_recurring_warning_index_reuse_count,
        ),
        (
            "characterized_recurring_warning_occurrence_count",
            0,
            state.characterized_recurring_warning_occurrence_count,
        ),
        (
            "characterized_recurring_warning_index_allocated",
            0,
            usize::from(state.characterized_recurring_warning_index_allocated),
        ),
        (
            "outdoor_air_flow_max_cooling_output_index",
            0,
            state.outdoor_air_flow_max_cooling_output_index,
        ),
        (
            "characterized_total_warning_error_increment_count",
            0,
            state.characterized_total_warning_error_increment_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_read_count",
            0,
            state.maximum_cooling_air_mass_flow_rate_read_count,
        ),
        (
            "outdoor_air_mass_flow_clamp_assignment_count",
            0,
            state.outdoor_air_mass_flow_clamp_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling OA maximum-flow body invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    if state
        .characterized_recurring_warning_report_maximum_m3_per_s
        .is_some()
    {
        return Err(
            "direct-zone IdealLoads cooling OA maximum-flow body retained a recurring-warning maximum"
                .to_string(),
        );
    }
    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow body has no latest snapshot".to_string()
    })?;
    let latest_predecessor = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow body has no latest gate snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling OA maximum-flow body has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling OA maximum-flow body has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor.system != expected_system
        || !latest_matches_release(
            latest,
            latest_predecessor,
            expected_system,
            expected_zone,
            coupling_call_count,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling OA maximum-flow body latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    body: &PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    predecessor: &PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    let expected_unit_off = !predecessor.unit_body_entered;
    let expected_non_cooling =
        predecessor.unit_body_entered && !predecessor.predecessor_cooling_body_entered;
    let expected_active_guard_false = predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && !predecessor.maximum_cooling_flow_body_entered;
    body.source == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
        && body.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
        && body.recurring_warning_child_source
            == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
        && body.source_order == PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER
        && body.system == expected_system
        && body.system == predecessor.system
        && body.parent_call_ordinal == call_count
        && body.parent_call_ordinal == predecessor.parent_call_ordinal
        && body.controlled_zone == expected_zone
        && body.controlled_zone == predecessor.controlled_zone
        && body.unit_body_entered == predecessor.unit_body_entered
        && body.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && body.predecessor_maximum_cooling_flow_body_entered
            == predecessor.maximum_cooling_flow_body_entered
        && !predecessor.maximum_cooling_flow_body_entered
        && body.body_skipped
        && body.unit_off_skipped == expected_unit_off
        && body.non_cooling_skipped == expected_non_cooling
        && body.active_guard_false_economizer_fallthrough == expected_active_guard_false
        && skipped_shape(body)
}

fn skipped_shape(body: &PurchasedAirCalcCoolingOaMaxFlowBodySnapshot) -> bool {
    !body.outdoor_air_mass_flow_rate_read
        && body
            .outdoor_air_mass_flow_rate_before_clamp_kg_per_s
            .is_none()
        && !body.standard_air_density_read
        && body.standard_air_density_kg_per_m3.is_none()
        && !body.outdoor_air_volume_flow_rate_calculated
        && body.outdoor_air_volume_flow_rate_m3_per_s.is_none()
        && !body.warning_counter_read
        && body.warning_counter_before.is_none()
        && body.first_warning_predicate_satisfied.is_none()
        && !body.first_warning_branch_entered
        && !body.warning_counter_incremented
        && body.warning_counter_after.is_none()
        && !body.first_warning_call_site_reached
        && !body.maximum_cooling_air_volume_flow_rate_read
        && body.maximum_cooling_air_volume_flow_rate_m3_per_s.is_none()
        && !body.continue_warning_call_site_reached
        && !body.continue_warning_timestamp_call_site_reached
        && !body.recurring_warning_branch_entered
        && !body.recurring_warning_call_site_reached
        && body
            .recurring_warning_report_maximum_input_m3_per_s
            .is_none()
        && !body.characterized_recurring_warning_index_allocated_on_call
        && !body.characterized_recurring_warning_index_reused_on_call
        && body.characterized_recurring_warning_index_before.is_none()
        && body.characterized_recurring_warning_index_after.is_none()
        && body
            .characterized_recurring_warning_occurrence_ordinal
            .is_none()
        && body
            .characterized_recurring_warning_report_maximum_m3_per_s
            .is_none()
        && !body.characterized_total_warning_error_incremented
        && !body.maximum_cooling_air_mass_flow_rate_read
        && body.maximum_cooling_air_mass_flow_rate_kg_per_s.is_none()
        && !body.outdoor_air_mass_flow_clamp_assignment_performed
        && body
            .outdoor_air_mass_flow_rate_after_clamp_kg_per_s
            .is_none()
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling OA maximum-flow body {label} overflowed")
    })
}
