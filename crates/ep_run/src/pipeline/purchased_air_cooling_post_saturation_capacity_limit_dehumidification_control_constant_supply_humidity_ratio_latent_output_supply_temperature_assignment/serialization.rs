//! JSON serialization for CP407 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleSummary,
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
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count": state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts,
        "predecessor_maximum_capacity_assignment_route_counts": state.predecessor_maximum_capacity_assignment_route_counts,
        "predecessor_else_branch_entry_route_counts": state.predecessor_else_branch_entry_route_counts,
        "supply_temperature_assignment_route_counts": state.supply_temperature_assignment_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp385_retained_supply_enthalpy_owned_read_count": state.cp385_retained_supply_enthalpy_owned_read_count,
        "cp406_same_call_supply_enthalpy_bit_corroboration_count": state.cp406_same_call_supply_enthalpy_bit_corroboration_count,
        "supply_enthalpy_for_dry_bulb_inversion_read_count": state.supply_enthalpy_for_dry_bulb_inversion_read_count,
        "cp378_retained_supply_humidity_ratio_owned_read_count": state.cp378_retained_supply_humidity_ratio_owned_read_count,
        "supply_humidity_ratio_for_dry_bulb_inversion_read_count": state.supply_humidity_ratio_for_dry_bulb_inversion_read_count,
        "psychrometric_supply_temperature_evaluation_count": state.psychrometric_supply_temperature_evaluation_count,
        "supply_temperature_assignment_write_count": state.supply_temperature_assignment_write_count,
        "cp406_preexisting_supply_temperature_state_owner_count": state.cp406_preexisting_supply_temperature_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_exact_assignment_counters_and_route_arrays() {
        let mut state =
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.transition_count = 1;
        state.predecessor_guard_false_fallthrough_count = 1;
        state.predecessor_else_branch_entry_count = 1;
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count = 1;
        state.predecessor_route_counts[20] = 1;
        state.predecessor_guard_false_fallthrough_route_counts[20] = 1;
        state.predecessor_else_branch_entry_route_counts[20] = 1;
        state.supply_temperature_assignment_route_counts[20] = 1;
        state.source_site_execution_count = 4;
        state.cp385_retained_supply_enthalpy_owned_read_count = 1;
        state.cp406_same_call_supply_enthalpy_bit_corroboration_count = 1;
        state.supply_enthalpy_for_dry_bulb_inversion_read_count = 1;
        state.cp378_retained_supply_humidity_ratio_owned_read_count = 1;
        state.supply_humidity_ratio_for_dry_bulb_inversion_read_count = 1;
        state.psychrometric_supply_temperature_evaluation_count = 1;
        state.supply_temperature_assignment_write_count = 1;
        state.cp406_preexisting_supply_temperature_state_owner_count = 1;
        state.unchanged_supply_humidity_ratio_preservation_count = 1;
        state.unchanged_supply_enthalpy_preservation_count = 1;
        let value = lifecycle_json(&PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        });
        assert_eq!(value["transition_count"], 1);
        assert_eq!(
            value["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count"],
            1
        );
        for field in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_maximum_capacity_assignment_route_counts",
            "predecessor_else_branch_entry_route_counts",
            "supply_temperature_assignment_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(30), "{field}");
        }
        assert_eq!(value["source_site_execution_count"], 4);
        assert!(value["latest"].is_null());
    }
}
