//! JSON serialization for CP354 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleSummary,
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
        "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count":
            state.dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count,
        "dehumidification_control_humidistat_case_selected_skip_count":
            state.dehumidification_control_humidistat_case_selected_skip_count,
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count":
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        "source_site_execution_count": state.source_site_execution_count,
        "supply_humidity_ratio_for_overdrying_limit_minimum_read_count":
            state.supply_humidity_ratio_for_overdrying_limit_minimum_read_count,
        "supply_temperature_for_humidity_ratio_inversion_read_count":
            state.supply_temperature_for_humidity_ratio_inversion_read_count,
        "supply_enthalpy_for_humidity_ratio_inversion_read_count":
            state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        "psychrometric_supply_humidity_ratio_evaluation_count":
            state.psychrometric_supply_humidity_ratio_evaluation_count,
        "source_shaped_two_argument_minimum_evaluation_count":
            state.source_shaped_two_argument_minimum_evaluation_count,
        "supply_humidity_ratio_assignment_write_count":
            state.supply_humidity_ratio_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
