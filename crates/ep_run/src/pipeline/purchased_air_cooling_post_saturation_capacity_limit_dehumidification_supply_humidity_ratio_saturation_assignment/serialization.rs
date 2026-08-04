//! JSON serialization for CP412 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;
#[cfg(test)]
pub(in crate::pipeline) use snapshot::test_snapshot;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleSummary,
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
        "predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count": state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count,
        "supply_humidity_ratio_saturation_assignment_count": state.supply_humidity_ratio_saturation_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts,
        "predecessor_maximum_capacity_assignment_route_counts": state.predecessor_maximum_capacity_assignment_route_counts,
        "predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts": state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts,
        "supply_humidity_ratio_saturation_assignment_route_counts": state.supply_humidity_ratio_saturation_assignment_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp411_supply_humidity_ratio_state_owner_count": state.cp411_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp411_supply_enthalpy_state_owner_count": state.cp411_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp411_supply_temperature_state_owner_count": state.cp411_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp411_retained_supply_temperature_owned_read_count": state.cp411_retained_supply_temperature_owned_read_count,
        "purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count": state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count,
        "environment_outdoor_barometric_pressure_owner_count": state.environment_outdoor_barometric_pressure_owner_count,
        "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count": state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count,
        "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count": state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count,
        "local_saturation_supply_humidity_ratio_assignment_write_count": state.local_saturation_supply_humidity_ratio_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_json_retains_all_five_route_arrays_and_cp412_counters() {
        let lifecycle = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState::new(IdealLoadsAirSystemId(0)),
        };
        let value = lifecycle_json(&lifecycle);
        for field in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_maximum_capacity_assignment_route_counts",
            "predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts",
            "supply_humidity_ratio_saturation_assignment_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(30), "{field}");
        }
        for field in [
            "cp411_retained_supply_temperature_owned_read_count",
            "purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count",
            "environment_outdoor_barometric_pressure_owner_count",
            "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count",
            "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count",
            "local_saturation_supply_humidity_ratio_assignment_write_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }
        assert!(value["latest"].is_null());
    }
}
