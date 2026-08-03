//! JSON serialization for CP408 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "predecessor_guard_false_fallthrough_count": state.predecessor_guard_false_fallthrough_count,
        "predecessor_maximum_capacity_assignment_count": state.predecessor_maximum_capacity_assignment_count,
        "predecessor_else_branch_entry_count": state.predecessor_else_branch_entry_count,
        "predecessor_supply_temperature_assignment_count": state.predecessor_supply_temperature_assignment_count,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count": state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts,
        "predecessor_maximum_capacity_assignment_route_counts": state.predecessor_maximum_capacity_assignment_route_counts,
        "predecessor_else_branch_entry_route_counts": state.predecessor_else_branch_entry_route_counts,
        "predecessor_supply_temperature_assignment_route_counts": state.predecessor_supply_temperature_assignment_route_counts,
        "supply_temperature_mixed_air_limit_route_counts": state.supply_temperature_mixed_air_limit_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp407_supply_temperature_state_owner_count": state.cp407_supply_temperature_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp407_retained_supply_temperature_owned_read_count": state.cp407_retained_supply_temperature_owned_read_count,
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
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_exact_limit_counters_and_route_arrays() {
        let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState::new(IdealLoadsAirSystemId(0));
        let value = lifecycle_json(&PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            state,
        });
        for field in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_maximum_capacity_assignment_route_counts",
            "predecessor_else_branch_entry_route_counts",
            "predecessor_supply_temperature_assignment_route_counts",
            "supply_temperature_mixed_air_limit_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(30), "{field}");
        }
        assert_eq!(value["source_site_execution_count"], 0);
        assert!(value["latest"].is_null());
    }
}
