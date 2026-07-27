//! JSON serialization for bounded CP314 lifecycle evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
};
use serde_json::{Value, json};

pub(crate) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "recurring_warning_child_source": lifecycle.recurring_warning_child_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "body_entry_count": state.body_entry_count,
        "body_skip_count": state.body_skip_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "active_guard_false_economizer_fallthrough_count":
            state.active_guard_false_economizer_fallthrough_count,
        "outdoor_air_mass_flow_rate_read_count":
            state.outdoor_air_mass_flow_rate_read_count,
        "standard_air_density_read_count": state.standard_air_density_read_count,
        "outdoor_air_volume_flow_calculation_count":
            state.outdoor_air_volume_flow_calculation_count,
        "warning_counter_read_count": state.warning_counter_read_count,
        "outdoor_air_flow_max_cooling_output_error_count":
            state.outdoor_air_flow_max_cooling_output_error_count,
        "first_warning_branch_count": state.first_warning_branch_count,
        "warning_counter_increment_count": state.warning_counter_increment_count,
        "first_warning_call_site_count": state.first_warning_call_site_count,
        "maximum_cooling_air_volume_flow_rate_read_count":
            state.maximum_cooling_air_volume_flow_rate_read_count,
        "continue_warning_call_site_count": state.continue_warning_call_site_count,
        "continue_warning_timestamp_call_site_count":
            state.continue_warning_timestamp_call_site_count,
        "recurring_warning_branch_count": state.recurring_warning_branch_count,
        "recurring_warning_call_site_count": state.recurring_warning_call_site_count,
        "characterized_recurring_warning_index_allocation_count":
            state.characterized_recurring_warning_index_allocation_count,
        "characterized_recurring_warning_index_reuse_count":
            state.characterized_recurring_warning_index_reuse_count,
        "characterized_recurring_warning_occurrence_count":
            state.characterized_recurring_warning_occurrence_count,
        "characterized_recurring_warning_index_allocated":
            state.characterized_recurring_warning_index_allocated,
        "outdoor_air_flow_max_cooling_output_index":
            state.outdoor_air_flow_max_cooling_output_index,
        "characterized_recurring_warning_report_maximum_m3_per_s":
            state.characterized_recurring_warning_report_maximum_m3_per_s,
        "characterized_total_warning_error_increment_count":
            state.characterized_total_warning_error_increment_count,
        "maximum_cooling_air_mass_flow_rate_read_count":
            state.maximum_cooling_air_mass_flow_rate_read_count,
        "outdoor_air_mass_flow_clamp_assignment_count":
            state.outdoor_air_mass_flow_clamp_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}

fn snapshot_json(snapshot: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot) -> Value {
    let mut value = json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "recurring_warning_child_source": snapshot.recurring_warning_child_source,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "source_order": snapshot.source_order,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_maximum_cooling_flow_body_entered":
            snapshot.predecessor_maximum_cooling_flow_body_entered,
        "body_skipped": snapshot.body_skipped,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "active_guard_false_economizer_fallthrough":
            snapshot.active_guard_false_economizer_fallthrough,
        "outdoor_air_mass_flow_rate_read": snapshot.outdoor_air_mass_flow_rate_read,
        "outdoor_air_mass_flow_rate_before_clamp_kg_per_s":
            snapshot.outdoor_air_mass_flow_rate_before_clamp_kg_per_s,
        "standard_air_density_read": snapshot.standard_air_density_read,
        "standard_air_density_kg_per_m3": snapshot.standard_air_density_kg_per_m3,
        "outdoor_air_volume_flow_rate_calculated":
            snapshot.outdoor_air_volume_flow_rate_calculated,
        "outdoor_air_volume_flow_rate_m3_per_s":
            snapshot.outdoor_air_volume_flow_rate_m3_per_s,
    });
    extend_object(
        &mut value,
        json!({
            "warning_counter_read": snapshot.warning_counter_read,
            "warning_counter_before": snapshot.warning_counter_before,
            "first_warning_predicate_satisfied": snapshot.first_warning_predicate_satisfied,
            "first_warning_branch_entered": snapshot.first_warning_branch_entered,
            "warning_counter_incremented": snapshot.warning_counter_incremented,
            "warning_counter_after": snapshot.warning_counter_after,
            "first_warning_call_site_reached": snapshot.first_warning_call_site_reached,
            "maximum_cooling_air_volume_flow_rate_read":
                snapshot.maximum_cooling_air_volume_flow_rate_read,
            "maximum_cooling_air_volume_flow_rate_m3_per_s":
                snapshot.maximum_cooling_air_volume_flow_rate_m3_per_s,
            "continue_warning_call_site_reached": snapshot.continue_warning_call_site_reached,
            "continue_warning_timestamp_call_site_reached":
                snapshot.continue_warning_timestamp_call_site_reached,
            "recurring_warning_branch_entered": snapshot.recurring_warning_branch_entered,
            "recurring_warning_call_site_reached": snapshot.recurring_warning_call_site_reached,
            "recurring_warning_report_maximum_input_m3_per_s":
                snapshot.recurring_warning_report_maximum_input_m3_per_s,
            "characterized_recurring_warning_index_allocated_on_call":
                snapshot.characterized_recurring_warning_index_allocated_on_call,
            "characterized_recurring_warning_index_reused_on_call":
                snapshot.characterized_recurring_warning_index_reused_on_call,
            "characterized_recurring_warning_index_before":
                snapshot.characterized_recurring_warning_index_before,
        }),
    );
    extend_object(
        &mut value,
        json!({
            "characterized_recurring_warning_index_after":
                snapshot.characterized_recurring_warning_index_after,
            "characterized_recurring_warning_occurrence_ordinal":
                snapshot.characterized_recurring_warning_occurrence_ordinal,
            "characterized_recurring_warning_report_maximum_m3_per_s":
                snapshot.characterized_recurring_warning_report_maximum_m3_per_s,
            "characterized_total_warning_error_incremented":
                snapshot.characterized_total_warning_error_incremented,
            "maximum_cooling_air_mass_flow_rate_read":
                snapshot.maximum_cooling_air_mass_flow_rate_read,
            "maximum_cooling_air_mass_flow_rate_kg_per_s":
                snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
            "outdoor_air_mass_flow_clamp_assignment_performed":
                snapshot.outdoor_air_mass_flow_clamp_assignment_performed,
            "outdoor_air_mass_flow_rate_after_clamp_kg_per_s":
                snapshot.outdoor_air_mass_flow_rate_after_clamp_kg_per_s,
        }),
    );
    value
}

fn extend_object(target: &mut Value, extension: Value) {
    let Value::Object(extension) = extension else {
        return;
    };
    if let Value::Object(target) = target {
        target.extend(extension);
    }
}
