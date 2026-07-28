//! JSON serialization for CP334 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
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
        "supply_temperature_mixed_air_limit_count":
            state.supply_temperature_mixed_air_limit_count,
        "source_site_execution_count": state.source_site_execution_count,
        "supply_temperature_for_minimum_read_count":
            state.supply_temperature_for_minimum_read_count,
        "mixed_air_temperature_for_minimum_read_count":
            state.mixed_air_temperature_for_minimum_read_count,
        "source_shaped_two_argument_minimum_evaluation_count":
            state.source_shaped_two_argument_minimum_evaluation_count,
        "supply_temperature_assignment_count":
            state.supply_temperature_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}
