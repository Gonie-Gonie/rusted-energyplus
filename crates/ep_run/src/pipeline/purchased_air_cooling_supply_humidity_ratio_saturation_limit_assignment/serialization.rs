//! JSON serialization for CP378 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary,
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
        "source_site_execution_count": state.source_site_execution_count,
        "local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count":
            state.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count,
        "local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count":
            state.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count,
        "source_shaped_two_argument_minimum_evaluation_count":
            state.source_shaped_two_argument_minimum_evaluation_count,
        "purchased_air_supply_humidity_ratio_saturation_limit_assignment_count":
            state.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count,
        "cp376_original_supply_humidity_ratio_owner_count":
            state.cp376_original_supply_humidity_ratio_owner_count,
        "cp377_saturation_supply_humidity_ratio_owner_count":
            state.cp377_saturation_supply_humidity_ratio_owner_count,
        "latest": state.latest.map(snapshot_json),
    })
}
