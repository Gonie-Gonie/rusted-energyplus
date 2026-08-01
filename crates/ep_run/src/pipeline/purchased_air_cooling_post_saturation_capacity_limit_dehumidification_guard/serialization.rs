//! JSON serialization for CP381 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "positive_guard_false_fallthrough_skip_count": state.positive_guard_false_fallthrough_skip_count,
        "heating_availability_guard_false_fallthrough_count": state.heating_availability_guard_false_fallthrough_count,
        "humidification_control_guard_false_fallthrough_count": state.humidification_control_guard_false_fallthrough_count,
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count": state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count": state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        "dehumidification_control_guard_false_fallthrough_count": state.dehumidification_control_guard_false_fallthrough_count,
        "heating_availability_guard_false_fallthrough_body_entry_count": state.heating_availability_guard_false_fallthrough_body_entry_count,
        "heating_availability_guard_false_fallthrough_capacity_guard_false_count": state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        "humidification_control_guard_false_fallthrough_body_entry_count": state.humidification_control_guard_false_fallthrough_body_entry_count,
        "humidification_control_guard_false_fallthrough_capacity_guard_false_count": state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        "dehumidification_control_humidistat_maximum_assignment_body_entry_count": state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
        "dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count": state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        "dehumidification_control_none_maximum_assignment_body_entry_count": state.dehumidification_control_none_maximum_assignment_body_entry_count,
        "dehumidification_control_none_maximum_assignment_capacity_guard_false_count": state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        "dehumidification_control_guard_false_fallthrough_body_entry_count": state.dehumidification_control_guard_false_fallthrough_body_entry_count,
        "dehumidification_control_guard_false_fallthrough_capacity_guard_false_count": state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        "heating_availability_guard_false_fallthrough_dehumidification_body_entry_count": state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        "heating_availability_guard_false_fallthrough_dehumidification_guard_false_count": state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        "humidification_control_guard_false_fallthrough_dehumidification_body_entry_count": state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        "humidification_control_guard_false_fallthrough_dehumidification_guard_false_count": state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count": state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count": state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        "dehumidification_guard_evaluation_count": state.dehumidification_guard_evaluation_count,
        "source_site_execution_count": state.source_site_execution_count,
        "cp378_supply_humidity_ratio_saturation_limit_owned_read_count": state.cp378_supply_humidity_ratio_saturation_limit_owned_read_count,
        "cp379_same_call_supply_humidity_ratio_bit_corroboration_count": state.cp379_same_call_supply_humidity_ratio_bit_corroboration_count,
        "purchased_air_supply_humidity_ratio_read_count": state.purchased_air_supply_humidity_ratio_read_count,
        "cp329_mixed_air_humidity_ratio_owned_read_count": state.cp329_mixed_air_humidity_ratio_owned_read_count,
        "purchased_air_mixed_air_humidity_ratio_read_count": state.purchased_air_mixed_air_humidity_ratio_read_count,
        "supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count": state.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count,
        "supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count": state.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count,
        "dehumidification_body_entry_count": state.dehumidification_body_entry_count,
        "dehumidification_guard_false_fallthrough_count": state.dehumidification_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}
