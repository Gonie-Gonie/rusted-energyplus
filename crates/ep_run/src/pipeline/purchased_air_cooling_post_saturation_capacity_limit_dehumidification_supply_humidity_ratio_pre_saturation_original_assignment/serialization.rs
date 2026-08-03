//! JSON serialization for CP411 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;
#[cfg(test)]
pub(in crate::pipeline) use snapshot::test_snapshot;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary,
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
        "supply_humidity_ratio_pre_saturation_original_assignment_count": state.supply_humidity_ratio_pre_saturation_original_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts,
        "predecessor_maximum_capacity_assignment_route_counts": state.predecessor_maximum_capacity_assignment_route_counts,
        "supply_humidity_ratio_pre_saturation_original_assignment_route_counts": state.supply_humidity_ratio_pre_saturation_original_assignment_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp410_supply_humidity_ratio_state_owner_count": state.cp410_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp410_supply_enthalpy_state_owner_count": state.cp410_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp410_supply_temperature_state_owner_count": state.cp410_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp410_retained_supply_humidity_ratio_owned_read_count": state.cp410_retained_supply_humidity_ratio_owned_read_count,
        "purchased_air_supply_humidity_ratio_before_saturation_limit_read_count": state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count,
        "local_supply_humidity_ratio_original_assignment_write_count": state.local_supply_humidity_ratio_original_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_assignment_accounting_and_all_route_partitions() {
        let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        let value = lifecycle_json(&PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        });
        for field in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_maximum_capacity_assignment_route_counts",
            "supply_humidity_ratio_pre_saturation_original_assignment_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(30), "{field}");
        }
        assert_eq!(
            value["supply_humidity_ratio_pre_saturation_original_assignment_count"],
            0
        );
        assert_eq!(value["source_site_execution_count"], 0);
        assert!(value["latest"].is_null());
    }
}
