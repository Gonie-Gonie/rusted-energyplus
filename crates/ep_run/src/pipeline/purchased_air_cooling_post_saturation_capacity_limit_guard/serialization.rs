//! JSON serialization for CP380 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "positive_guard_false_fallthrough_skip_count":
            state.positive_guard_false_fallthrough_skip_count,
        "heating_availability_guard_false_fallthrough_count":
            state.heating_availability_guard_false_fallthrough_count,
        "humidification_control_guard_false_fallthrough_count":
            state.humidification_control_guard_false_fallthrough_count,
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count":
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count":
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        "dehumidification_control_guard_false_fallthrough_count":
            state.dehumidification_control_guard_false_fallthrough_count,
        "heating_availability_guard_false_fallthrough_body_entry_count":
            state.heating_availability_guard_false_fallthrough_body_entry_count,
        "heating_availability_guard_false_fallthrough_capacity_guard_false_count":
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        "humidification_control_guard_false_fallthrough_body_entry_count":
            state.humidification_control_guard_false_fallthrough_body_entry_count,
        "humidification_control_guard_false_fallthrough_capacity_guard_false_count":
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        "dehumidification_control_humidistat_maximum_assignment_body_entry_count":
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
        "dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count":
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        "dehumidification_control_none_maximum_assignment_body_entry_count":
            state.dehumidification_control_none_maximum_assignment_body_entry_count,
        "dehumidification_control_none_maximum_assignment_capacity_guard_false_count":
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        "dehumidification_control_guard_false_fallthrough_body_entry_count":
            state.dehumidification_control_guard_false_fallthrough_body_entry_count,
        "dehumidification_control_guard_false_fallthrough_capacity_guard_false_count":
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        "capacity_limit_guard_evaluation_count": state.capacity_limit_guard_evaluation_count,
        "source_site_execution_count": state.source_site_execution_count,
        "configured_cooling_limit_owned_read_count":
            state.configured_cooling_limit_owned_read_count,
        "cp337_same_call_selector_lineage_corroboration_count":
            state.cp337_same_call_selector_lineage_corroboration_count,
        "first_cooling_limit_read_count": state.first_cooling_limit_read_count,
        "cooling_limit_capacity_comparison_count":
            state.cooling_limit_capacity_comparison_count,
        "cooling_limit_capacity_match_count": state.cooling_limit_capacity_match_count,
        "second_cooling_limit_read_count": state.second_cooling_limit_read_count,
        "cooling_limit_flow_rate_and_capacity_comparison_count":
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        "cooling_limit_flow_rate_and_capacity_match_count":
            state.cooling_limit_flow_rate_and_capacity_match_count,
        "cooling_limit_rejected_count": state.cooling_limit_rejected_count,
        "capacity_limit_body_entry_count": state.capacity_limit_body_entry_count,
        "active_guard_false_fallthrough_count": state.active_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}
