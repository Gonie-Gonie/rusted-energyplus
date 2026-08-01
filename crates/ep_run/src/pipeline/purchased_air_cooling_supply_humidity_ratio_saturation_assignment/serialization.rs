//! JSON serialization for CP377 saturation-assignment lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary,
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
        "heating_availability_guard_false_fallthrough_count":
            state.heating_availability_guard_false_fallthrough_count,
        "humidification_control_guard_false_fallthrough_count":
            state.humidification_control_guard_false_fallthrough_count,
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count":
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count":
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        "dehumidification_control_guard_false_fallthrough_count":
            state.dehumidification_control_guard_false_fallthrough_count,
        "source_site_execution_count": state.source_site_execution_count,
        "purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count":
            state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count,
        "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count":
            state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count,
        "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count":
            state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count,
        "local_saturation_supply_humidity_ratio_assignment_count":
            state.local_saturation_supply_humidity_ratio_assignment_count,
        "cp334_supply_temperature_mixed_air_limit_owner_count":
            state.cp334_supply_temperature_mixed_air_limit_owner_count,
        "cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count":
            state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        "environment_outdoor_barometric_pressure_owner_count":
            state.environment_outdoor_barometric_pressure_owner_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_four_sites_and_owner_partition() {
        let mut state =
            PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.transition_count = 2;
        state.heating_availability_guard_false_fallthrough_count = 2;
        state.source_site_execution_count = 8;
        state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count = 2;
        state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count = 2;
        state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count = 2;
        state.local_saturation_supply_humidity_ratio_assignment_count = 2;
        state.cp334_supply_temperature_mixed_air_limit_owner_count = 2;
        state.environment_outdoor_barometric_pressure_owner_count = 2;
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary {
                source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
                first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );
        assert_eq!(value["source_site_execution_count"], 8);
        assert_eq!(
            value["environment_outdoor_barometric_pressure_owner_count"],
            2
        );
        assert!(value["latest"].is_null());
    }
}
