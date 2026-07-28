//! JSON serialization for CP344 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
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
        "capacity_limit_guard_false_fallthrough_skip_count":
            state.capacity_limit_guard_false_fallthrough_skip_count,
        "capacity_limit_sensible_output_guard_false_fallthrough_count":
            state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count":
            state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        "source_site_execution_count": state.source_site_execution_count,
        "supply_temperature_for_minimum_read_count":
            state.supply_temperature_for_minimum_read_count,
        "mixed_air_temperature_for_minimum_read_count":
            state.mixed_air_temperature_for_minimum_read_count,
        "source_shaped_two_argument_minimum_evaluation_count":
            state.source_shaped_two_argument_minimum_evaluation_count,
        "supply_temperature_assignment_write_count":
            state.supply_temperature_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_cp344_counters_and_null_latest() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.transition_count = 1;
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count = 1;
        state.source_site_execution_count = 4;
        state.supply_temperature_for_minimum_read_count = 1;
        state.mixed_air_temperature_for_minimum_read_count = 1;
        state.source_shaped_two_argument_minimum_evaluation_count = 1;
        state.supply_temperature_assignment_write_count = 1;
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary {
                source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );
        assert_eq!(value["transition_count"], 1);
        assert_eq!(
            value["capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count"],
            1
        );
        assert_eq!(value["source_site_execution_count"], 4);
        assert!(value["latest"].is_null());
    }
}
