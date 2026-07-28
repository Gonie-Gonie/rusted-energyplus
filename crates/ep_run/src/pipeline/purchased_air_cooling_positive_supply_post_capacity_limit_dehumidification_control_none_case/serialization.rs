//! JSON serialization for CP347 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
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
        "dehumidification_control_none_case_completion_count":
            state.dehumidification_control_none_case_completion_count,
        "dehumidification_control_constant_sensible_heat_ratio_case_selection_count":
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        "dehumidification_control_humidistat_case_selection_count":
            state.dehumidification_control_humidistat_case_selection_count,
        "dehumidification_control_constant_supply_humidity_ratio_case_selection_count":
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        "source_site_execution_count": state.source_site_execution_count,
        "dehumidification_control_none_case_entry_count":
            state.dehumidification_control_none_case_entry_count,
        "mixed_air_humidity_ratio_read_count": state.mixed_air_humidity_ratio_read_count,
        "supply_humidity_ratio_assignment_count":
            state.supply_humidity_ratio_assignment_count,
        "dehumidification_control_none_case_break_count":
            state.dehumidification_control_none_case_break_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_cp347_route_and_four_source_counters() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.transition_count = 3;
        state.dehumidification_control_none_case_completion_count = 3;
        state.source_site_execution_count = 12;
        state.dehumidification_control_none_case_entry_count = 3;
        state.mixed_air_humidity_ratio_read_count = 3;
        state.supply_humidity_ratio_assignment_count = 3;
        state.dehumidification_control_none_case_break_count = 3;
        let value = lifecycle_json(
            &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary {
                source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
                state,
            },
        );

        assert_eq!(
            value["dehumidification_control_none_case_completion_count"],
            3
        );
        assert_eq!(value["source_site_execution_count"], 12);
        assert_eq!(value["dehumidification_control_none_case_break_count"], 3);
        assert!(value["latest"].is_null());
    }
}
