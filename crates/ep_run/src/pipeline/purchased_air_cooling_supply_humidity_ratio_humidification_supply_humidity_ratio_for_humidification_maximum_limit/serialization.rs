//! JSON serialization for CP374 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitLifecycleSummary,
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
        "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count":
            state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count,
        "dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count":
            state.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count,
        "dehumidification_control_guard_false_fallthrough_count":
            state.dehumidification_control_guard_false_fallthrough_count,
        "source_site_execution_count": state.source_site_execution_count,
        "supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read_count":
            state.supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read_count,
        "maximum_heating_supply_air_humidity_ratio_for_minimum_read_count":
            state.maximum_heating_supply_air_humidity_ratio_for_minimum_read_count,
        "source_shaped_two_argument_minimum_evaluation_count":
            state.source_shaped_two_argument_minimum_evaluation_count,
        "supply_humidity_ratio_for_humidification_assignment_count":
            state.supply_humidity_ratio_for_humidification_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}
