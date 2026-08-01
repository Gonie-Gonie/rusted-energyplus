//! JSON serialization for CP382 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary,
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
        "heating_availability_guard_false_fallthrough_capacity_guard_false_count": state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        "heating_availability_guard_false_fallthrough_dehumidification_body_entry_count": state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        "heating_availability_guard_false_fallthrough_dehumidification_guard_false_count": state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        "heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count": state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        "humidification_control_guard_false_fallthrough_capacity_guard_false_count": state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        "humidification_control_guard_false_fallthrough_dehumidification_body_entry_count": state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        "humidification_control_guard_false_fallthrough_dehumidification_guard_false_count": state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        "humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count": state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        "dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count": state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        "dehumidification_control_none_maximum_assignment_capacity_guard_false_count": state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count": state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count": state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count": state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        "dehumidification_control_guard_false_fallthrough_capacity_guard_false_count": state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        "dehumidification_total_output_assignment_count": state.dehumidification_total_output_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "cp330_supply_mass_flow_rate_owned_read_count": state.cp330_supply_mass_flow_rate_owned_read_count,
        "cp329_same_call_supply_mass_flow_rate_bit_corroboration_count": state.cp329_same_call_supply_mass_flow_rate_bit_corroboration_count,
        "cp339_same_call_supply_mass_flow_rate_bit_corroboration_count": state.cp339_same_call_supply_mass_flow_rate_bit_corroboration_count,
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "cp329_mixed_air_enthalpy_owned_read_count": state.cp329_mixed_air_enthalpy_owned_read_count,
        "cp329_same_call_recirculation_enthalpy_bit_corroboration_count": state.cp329_same_call_recirculation_enthalpy_bit_corroboration_count,
        "cp339_same_call_mixed_air_enthalpy_bit_corroboration_count": state.cp339_same_call_mixed_air_enthalpy_bit_corroboration_count,
        "mixed_air_enthalpy_read_count": state.mixed_air_enthalpy_read_count,
        "cp379_post_saturation_supply_enthalpy_owned_read_count": state.cp379_post_saturation_supply_enthalpy_owned_read_count,
        "cp379_same_call_supply_enthalpy_bits_corroboration_count": state.cp379_same_call_supply_enthalpy_bits_corroboration_count,
        "supply_enthalpy_read_count": state.supply_enthalpy_read_count,
        "enthalpy_difference_calculation_count": state.enthalpy_difference_calculation_count,
        "cooling_total_output_calculation_count": state.cooling_total_output_calculation_count,
        "cooling_total_output_assignment_write_count": state.cooling_total_output_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
