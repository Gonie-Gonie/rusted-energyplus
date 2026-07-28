//! JSON serialization for CP343 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
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
        "capacity_limit_sensible_output_supply_temperature_assignment_count":
            state.capacity_limit_sensible_output_supply_temperature_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "supply_enthalpy_for_dry_bulb_inversion_read_count":
            state.supply_enthalpy_for_dry_bulb_inversion_read_count,
        "supply_humidity_ratio_for_dry_bulb_inversion_read_count":
            state.supply_humidity_ratio_for_dry_bulb_inversion_read_count,
        "psychrometric_supply_temperature_evaluation_count":
            state.psychrometric_supply_temperature_evaluation_count,
        "supply_temperature_assignment_write_count":
            state.supply_temperature_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_counters_and_latest_snapshot() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.transition_count = 1;
        state.capacity_limit_sensible_output_supply_temperature_assignment_count = 1;
        state.source_site_execution_count = 4;
        state.supply_enthalpy_for_dry_bulb_inversion_read_count = 1;
        state.supply_humidity_ratio_for_dry_bulb_inversion_read_count = 1;
        state.psychrometric_supply_temperature_evaluation_count = 1;
        state.supply_temperature_assignment_write_count = 1;
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary {
                source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );
        assert_eq!(value["transition_count"], 1);
        assert_eq!(
            value["capacity_limit_sensible_output_supply_temperature_assignment_count"],
            1
        );
        assert_eq!(value["source_site_execution_count"], 4);
        assert!(value["latest"].is_null());
    }
}
