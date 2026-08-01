//! JSON serialization for CP385 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary,
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
        "heating_availability_guard_false_fallthrough_dehumidification_guard_false_count": state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        "heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count": state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        "heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count": state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        "heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count": state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        "humidification_control_guard_false_fallthrough_capacity_guard_false_count": state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        "humidification_control_guard_false_fallthrough_dehumidification_guard_false_count": state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        "humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count": state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        "humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count": state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        "humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count": state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        "dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count": state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        "dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count": state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        "dehumidification_control_none_maximum_assignment_capacity_guard_false_count": state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count": state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count": state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count": state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        "dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count": state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        "dehumidification_control_guard_false_fallthrough_capacity_guard_false_count": state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        "dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count": state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        "dehumidification_total_output_capacity_guard_evaluation_count": state.dehumidification_total_output_capacity_guard_evaluation_count,
        "dehumidification_total_output_capacity_guard_false_fallthrough_count": state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
        "dehumidification_total_output_maximum_capacity_assignment_count": state.dehumidification_total_output_maximum_capacity_assignment_count,
        "post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count": state.post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "cp379_retained_supply_enthalpy_owned_read_count": state.cp379_retained_supply_enthalpy_owned_read_count,
        "cp329_retained_mixed_air_enthalpy_owned_read_count": state.cp329_retained_mixed_air_enthalpy_owned_read_count,
        "mixed_air_enthalpy_read_count": state.mixed_air_enthalpy_read_count,
        "cp384_retained_cooling_total_output_owned_read_count": state.cp384_retained_cooling_total_output_owned_read_count,
        "cooling_total_output_read_count": state.cooling_total_output_read_count,
        "cp330_retained_supply_mass_flow_rate_owned_read_count": state.cp330_retained_supply_mass_flow_rate_owned_read_count,
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "specific_cooling_output_calculation_count": state.specific_cooling_output_calculation_count,
        "supply_enthalpy_difference_calculation_count": state.supply_enthalpy_difference_calculation_count,
        "supply_enthalpy_assignment_write_count": state.supply_enthalpy_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
