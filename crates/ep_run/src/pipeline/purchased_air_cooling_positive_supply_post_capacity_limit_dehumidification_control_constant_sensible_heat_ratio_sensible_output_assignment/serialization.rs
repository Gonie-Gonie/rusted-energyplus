//! JSON serialization for CP350 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary,
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
        "dehumidification_control_none_case_completed_skip_count":
            state.dehumidification_control_none_case_completed_skip_count,
        "dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count":
            state.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
        "dehumidification_control_humidistat_case_selected_skip_count":
            state.dehumidification_control_humidistat_case_selected_skip_count,
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count":
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        "source_site_execution_count": state.source_site_execution_count,
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "cp_air_read_count": state.cp_air_read_count,
        "supply_mass_flow_rate_times_cp_air_calculation_count":
            state.supply_mass_flow_rate_times_cp_air_calculation_count,
        "mixed_air_temperature_read_count": state.mixed_air_temperature_read_count,
        "supply_temperature_read_count": state.supply_temperature_read_count,
        "mixed_air_minus_supply_temperature_calculation_count":
            state.mixed_air_minus_supply_temperature_calculation_count,
        "cooling_sensible_output_calculation_count":
            state.cooling_sensible_output_calculation_count,
        "cooling_sensible_output_assignment_write_count":
            state.cooling_sensible_output_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
