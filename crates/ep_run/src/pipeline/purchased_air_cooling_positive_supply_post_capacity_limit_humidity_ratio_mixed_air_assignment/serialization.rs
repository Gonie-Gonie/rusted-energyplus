//! JSON serialization for CP345 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
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
        "assignment_after_capacity_limit_guard_false_fallthrough_count":
            state.assignment_after_capacity_limit_guard_false_fallthrough_count,
        "assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count":
            state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        "assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count":
            state.assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        "post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count":
            state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "mixed_air_humidity_ratio_read_count":
            state.mixed_air_humidity_ratio_read_count,
        "supply_humidity_ratio_assignment_count":
            state.supply_humidity_ratio_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_cp345_route_and_source_counters() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.transition_count = 3;
        state.assignment_after_capacity_limit_guard_false_fallthrough_count = 1;
        state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count = 1;
        state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count =
            1;
        state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count = 3;
        state.source_site_execution_count = 6;
        state.mixed_air_humidity_ratio_read_count = 3;
        state.supply_humidity_ratio_assignment_count = 3;
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary {
                source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );

        assert_eq!(
            value["assignment_after_capacity_limit_guard_false_fallthrough_count"],
            1
        );
        assert_eq!(
            value["assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count"],
            1
        );
        assert_eq!(
            value["assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count"],
            1
        );
        assert_eq!(
            value["post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count"],
            3
        );
        assert_eq!(value["source_site_execution_count"], 6);
        assert!(value["latest"].is_null());
    }
}
