//! JSON serialization for CP414 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "saturation_supply_temperature_assignment_count": state.saturation_supply_temperature_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts.as_slice(),
        "predecessor_guard_body_entry_route_counts": state.predecessor_guard_body_entry_route_counts.as_slice(),
        "supply_temperature_saturation_assignment_route_counts": state.supply_temperature_saturation_assignment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp413_supply_humidity_ratio_state_owner_count": state.cp413_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp413_supply_enthalpy_state_owner_count": state.cp413_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp413_supply_temperature_state_owner_count": state.cp413_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp414_saturation_supply_temperature_state_owner_count": state.cp414_saturation_supply_temperature_state_owner_count,
        "cp413_retained_supply_enthalpy_owned_read_count": state.cp413_retained_supply_enthalpy_owned_read_count,
        "supply_enthalpy_for_saturation_temperature_read_count": state.supply_enthalpy_for_saturation_temperature_read_count,
        "environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count": state.environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count,
        "environment_outdoor_barometric_pressure_for_saturation_temperature_read_count": state.environment_outdoor_barometric_pressure_for_saturation_temperature_read_count,
        "psy_tsat_fn_h_pb_evaluation_count": state.psy_tsat_fn_h_pb_evaluation_count,
        "purchased_air_supply_temperature_saturation_assignment_write_count": state.purchased_air_supply_temperature_saturation_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_json_retains_four_width_36_route_arrays_and_cp414_counters() {
        let lifecycle = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState::new(IdealLoadsAirSystemId(0)),
        };
        let value = lifecycle_json(&lifecycle);
        for field in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_body_entry_route_counts",
            "supply_temperature_saturation_assignment_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(36), "{field}");
        }
        for field in [
            "saturation_supply_temperature_assignment_count",
            "cp414_saturation_supply_temperature_state_owner_count",
            "cp413_retained_supply_enthalpy_owned_read_count",
            "supply_enthalpy_for_saturation_temperature_read_count",
            "environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count",
            "environment_outdoor_barometric_pressure_for_saturation_temperature_read_count",
            "psy_tsat_fn_h_pb_evaluation_count",
            "purchased_air_supply_temperature_saturation_assignment_write_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }
        assert!(value["latest"].is_null());
    }
}
