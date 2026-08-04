//! JSON serialization for CP415 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "predecessor_supply_temperature_saturation_assignment_count": state.predecessor_supply_temperature_saturation_assignment_count,
        "supply_temperature_saturation_mixed_air_limit_count": state.supply_temperature_saturation_mixed_air_limit_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts.as_slice(),
        "predecessor_guard_body_entry_route_counts": state.predecessor_guard_body_entry_route_counts.as_slice(),
        "predecessor_supply_temperature_saturation_assignment_route_counts": state.predecessor_supply_temperature_saturation_assignment_route_counts.as_slice(),
        "supply_temperature_mixed_air_limit_route_counts": state.supply_temperature_mixed_air_limit_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp414_supply_humidity_ratio_state_owner_count": state.cp414_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp414_supply_enthalpy_state_owner_count": state.cp414_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp414_supply_temperature_state_owner_count": state.cp414_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp415_mixed_air_limited_supply_temperature_state_owner_count": state.cp415_mixed_air_limited_supply_temperature_state_owner_count,
        "cp414_retained_supply_temperature_owned_read_count": state.cp414_retained_supply_temperature_owned_read_count,
        "supply_temperature_for_minimum_read_count": state.supply_temperature_for_minimum_read_count,
        "cp329_retained_mixed_air_temperature_owned_read_count": state.cp329_retained_mixed_air_temperature_owned_read_count,
        "mixed_air_temperature_for_minimum_read_count": state.mixed_air_temperature_for_minimum_read_count,
        "source_shaped_two_argument_minimum_evaluation_count": state.source_shaped_two_argument_minimum_evaluation_count,
        "supply_temperature_assignment_write_count": state.supply_temperature_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_json_retains_five_width_36_route_arrays_and_cp415_counters() {
        let lifecycle = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState::new(IdealLoadsAirSystemId(0)),
        };
        let value = lifecycle_json(&lifecycle);
        for field in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_body_entry_route_counts",
            "predecessor_supply_temperature_saturation_assignment_route_counts",
            "supply_temperature_mixed_air_limit_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(36), "{field}");
        }
        for field in [
            "predecessor_supply_temperature_saturation_assignment_count",
            "supply_temperature_saturation_mixed_air_limit_count",
            "source_site_execution_count",
            "cp415_mixed_air_limited_supply_temperature_state_owner_count",
            "cp414_retained_supply_temperature_owned_read_count",
            "cp329_retained_mixed_air_temperature_owned_read_count",
            "source_shaped_two_argument_minimum_evaluation_count",
            "supply_temperature_assignment_write_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }
        assert!(value["latest"].is_null());
    }
}
