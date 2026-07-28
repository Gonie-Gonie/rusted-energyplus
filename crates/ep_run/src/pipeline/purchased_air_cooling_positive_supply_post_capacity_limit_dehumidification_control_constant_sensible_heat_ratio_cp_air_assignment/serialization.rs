//! JSON serialization for CP349 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary,
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
        "dehumidification_control_none_case_completed_skip_count":
            state.dehumidification_control_none_case_completed_skip_count,
        "dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count":
            state.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count,
        "dehumidification_control_humidistat_case_selected_skip_count":
            state.dehumidification_control_humidistat_case_selected_skip_count,
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count":
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        "source_site_execution_count": state.source_site_execution_count,
        "mixed_air_humidity_ratio_read_count": state.mixed_air_humidity_ratio_read_count,
        "psychrometric_cp_air_evaluation_count":
            state.psychrometric_cp_air_evaluation_count,
        "cp_air_assignment_write_count": state.cp_air_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_cp349_direct_none_complete_skip_and_zero_source_counters() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.transition_count = 3;
        state.dehumidification_control_none_case_completed_skip_count = 3;
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary {
                source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );

        assert_eq!(
            value["dehumidification_control_none_case_completed_skip_count"],
            3
        );
        assert_eq!(
            value["dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count"],
            0
        );
        for field in [
            "source_site_execution_count",
            "mixed_air_humidity_ratio_read_count",
            "psychrometric_cp_air_evaluation_count",
            "cp_air_assignment_write_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }
        assert!(value["latest"].is_null());
    }
}
