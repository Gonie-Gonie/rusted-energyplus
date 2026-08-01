//! JSON serialization for CP388 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count": state.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cooling_total_output_owned_read_count": state.cooling_total_output_owned_read_count,
        "cooling_total_output_bit_corroboration_count": state.cooling_total_output_bit_corroboration_count,
        "cooling_sensible_heat_ratio_read_count": state.cooling_sensible_heat_ratio_read_count,
        "cooling_sensible_output_calculation_count": state.cooling_sensible_output_calculation_count,
        "cooling_sensible_output_assignment_write_count": state.cooling_sensible_output_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_cp388_direct_complete_skip_and_thirty_route_slots() {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.transition_count = 1;
        state.inactive_transition_count = 1;
        state.predecessor_route_counts[0] = 1;
        let value = lifecycle_json(&PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        });

        assert_eq!(value["transition_count"], 1);
        assert_eq!(value["inactive_transition_count"], 1);
        assert_eq!(
            value["predecessor_route_counts"].as_array().map(Vec::len),
            Some(30)
        );
        for field in [
            "dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count",
            "source_site_execution_count",
            "cooling_total_output_owned_read_count",
            "cooling_total_output_bit_corroboration_count",
            "cooling_sensible_heat_ratio_read_count",
            "cooling_sensible_output_calculation_count",
            "cooling_sensible_output_assignment_write_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }
        assert!(value["latest"].is_null());
    }
}
