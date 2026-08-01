//! JSON serialization for CP389 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;
#[cfg(test)]
pub(in crate::pipeline) use snapshot::test_snapshot;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count": state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp379_supply_temperature_state_owner_count": state.cp379_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "mixed_air_temperature_owned_read_count": state.mixed_air_temperature_owned_read_count,
        "cooling_sensible_output_owned_read_count": state.cooling_sensible_output_owned_read_count,
        "cp_air_owned_read_count": state.cp_air_owned_read_count,
        "supply_mass_flow_rate_owned_read_count": state.supply_mass_flow_rate_owned_read_count,
        "supply_mass_flow_rate_bit_corroboration_count": state.supply_mass_flow_rate_bit_corroboration_count,
        "air_capacity_rate_calculation_count": state.air_capacity_rate_calculation_count,
        "sensible_temperature_drop_calculation_count": state.sensible_temperature_drop_calculation_count,
        "supply_temperature_calculation_count": state.supply_temperature_calculation_count,
        "supply_temperature_assignment_write_count": state.supply_temperature_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_direct_skip_and_thirty_route_slots() {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.transition_count = 1;
        state.inactive_transition_count = 1;
        state.predecessor_route_counts[0] = 1;
        let value = lifecycle_json(&PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        });
        assert_eq!(value["transition_count"], 1);
        assert_eq!(value["inactive_transition_count"], 1);
        assert_eq!(
            value["predecessor_route_counts"].as_array().map(Vec::len),
            Some(30)
        );
        assert_eq!(
            value["dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count"],
            0
        );
        assert!(value["latest"].is_null());
    }
}
