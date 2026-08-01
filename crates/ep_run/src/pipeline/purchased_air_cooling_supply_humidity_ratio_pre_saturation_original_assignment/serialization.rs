//! JSON serialization for CP376 pre-saturation original-assignment lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary,
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
        "purchased_air_supply_humidity_ratio_before_saturation_limit_read_count":
            state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count,
        "local_original_supply_humidity_ratio_before_saturation_limit_assignment_count":
            state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count,
        "cp375_maximum_assignment_owner_count": state.cp375_maximum_assignment_owner_count,
        "cp347_none_case_owner_count": state.cp347_none_case_owner_count,
        "cp356_constant_shr_owner_count": state.cp356_constant_shr_owner_count,
        "cp362_humidistat_owner_count": state.cp362_humidistat_owner_count,
        "cp365_constant_supply_humidity_ratio_owner_count":
            state.cp365_constant_supply_humidity_ratio_owner_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_two_sites_and_owner_partition() {
        let mut state =
            PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.transition_count = 3;
        state.heating_availability_guard_false_fallthrough_count = 3;
        state.source_site_execution_count = 6;
        state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count = 3;
        state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count = 3;
        state.cp347_none_case_owner_count = 3;
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary {
                source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
                first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );

        assert_eq!(value["transition_count"], 3);
        assert_eq!(value["source_site_execution_count"], 6);
        assert_eq!(
            value["purchased_air_supply_humidity_ratio_before_saturation_limit_read_count"],
            3
        );
        assert_eq!(
            value["local_original_supply_humidity_ratio_before_saturation_limit_assignment_count"],
            3
        );
        assert_eq!(value["cp347_none_case_owner_count"], 3);
        assert!(value["latest"].is_null());
    }
}
