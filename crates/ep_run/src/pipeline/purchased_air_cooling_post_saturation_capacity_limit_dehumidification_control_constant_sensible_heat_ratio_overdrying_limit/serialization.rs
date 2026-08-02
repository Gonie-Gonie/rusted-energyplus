//! JSON serialization for CP391 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;
#[cfg(test)]
pub(in crate::pipeline) use snapshot::test_snapshot;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count": state.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp390_supply_enthalpy_state_owner_count": state.cp390_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "supply_enthalpy_owned_read_count": state.supply_enthalpy_owned_read_count,
        "supply_enthalpy_for_overdrying_limit_maximum_read_count": state.supply_enthalpy_for_overdrying_limit_maximum_read_count,
        "supply_temperature_owned_read_count": state.supply_temperature_owned_read_count,
        "supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count": state.supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count,
        "psychrometric_minimum_supply_enthalpy_evaluation_count": state.psychrometric_minimum_supply_enthalpy_evaluation_count,
        "source_shaped_two_argument_maximum_evaluation_count": state.source_shaped_two_argument_maximum_evaluation_count,
        "supply_enthalpy_assignment_write_count": state.supply_enthalpy_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_direct_skip_and_thirty_route_slots() {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.transition_count = 1;
        state.inactive_transition_count = 1;
        state.predecessor_route_counts[0] = 1;
        let value = lifecycle_json(&PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
            state,
        });
        assert_eq!(value["transition_count"], 1);
        assert_eq!(value["inactive_transition_count"], 1);
        assert_eq!(
            value["predecessor_route_counts"].as_array().map(Vec::len),
            Some(30)
        );
        assert_eq!(
            value["dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count"],
            0
        );
        assert!(value["latest"].is_null());
    }
}
