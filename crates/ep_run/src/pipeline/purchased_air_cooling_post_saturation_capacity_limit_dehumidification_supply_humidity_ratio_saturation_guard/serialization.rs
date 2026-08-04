//! JSON serialization for CP413 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "saturation_supply_humidity_ratio_guard_evaluation_count": state.saturation_supply_humidity_ratio_guard_evaluation_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "guard_false_fallthrough_route_counts": state.guard_false_fallthrough_route_counts.as_slice(),
        "guard_body_entry_route_counts": state.guard_body_entry_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp412_supply_humidity_ratio_state_owner_count": state.cp412_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp412_supply_enthalpy_state_owner_count": state.cp412_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp412_supply_temperature_state_owner_count": state.cp412_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp412_saturation_supply_humidity_ratio_owned_read_count": state.cp412_saturation_supply_humidity_ratio_owned_read_count,
        "saturation_supply_humidity_ratio_for_guard_read_count": state.saturation_supply_humidity_ratio_for_guard_read_count,
        "cp411_original_supply_humidity_ratio_owned_read_count": state.cp411_original_supply_humidity_ratio_owned_read_count,
        "cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count": state.cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count,
        "original_supply_humidity_ratio_for_guard_read_count": state.original_supply_humidity_ratio_for_guard_read_count,
        "saturation_original_supply_humidity_ratio_comparison_count": state.saturation_original_supply_humidity_ratio_comparison_count,
        "saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count": state.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count,
        "saturation_supply_humidity_ratio_guard_body_entry_count": state.saturation_supply_humidity_ratio_guard_body_entry_count,
        "saturation_supply_humidity_ratio_guard_false_fallthrough_count": state.saturation_supply_humidity_ratio_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_json_retains_three_width_36_route_arrays_and_cp413_counters() {
        let lifecycle = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE,
            state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState::new(IdealLoadsAirSystemId(0)),
        };
        let value = lifecycle_json(&lifecycle);
        for field in [
            "predecessor_route_counts",
            "guard_false_fallthrough_route_counts",
            "guard_body_entry_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(36), "{field}");
        }
        for field in [
            "saturation_supply_humidity_ratio_guard_evaluation_count",
            "cp412_saturation_supply_humidity_ratio_owned_read_count",
            "saturation_supply_humidity_ratio_for_guard_read_count",
            "cp411_original_supply_humidity_ratio_owned_read_count",
            "cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count",
            "original_supply_humidity_ratio_for_guard_read_count",
            "saturation_original_supply_humidity_ratio_comparison_count",
            "saturation_supply_humidity_ratio_guard_body_entry_count",
            "saturation_supply_humidity_ratio_guard_false_fallthrough_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }
        assert!(value["latest"].is_null());
    }
}
