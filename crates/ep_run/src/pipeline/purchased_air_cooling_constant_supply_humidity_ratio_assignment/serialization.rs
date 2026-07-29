//! JSON serialization for CP365 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
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
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count":
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        "dehumidification_control_humidistat_case_completed_skip_count":
            state.dehumidification_control_humidistat_case_completed_skip_count,
        "dehumidification_control_constant_supply_humidity_ratio_assignment_count":
            state.dehumidification_control_constant_supply_humidity_ratio_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "minimum_cooling_supply_air_humidity_ratio_read_count":
            state.minimum_cooling_supply_air_humidity_ratio_read_count,
        "supply_humidity_ratio_assignment_count":
            state.supply_humidity_ratio_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}
