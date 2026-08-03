//! JSON serialization for CP406 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleSummary,
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
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count": state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts,
        "predecessor_maximum_capacity_assignment_route_counts": state.predecessor_maximum_capacity_assignment_route_counts,
        "else_branch_entry_route_counts": state.else_branch_entry_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_serializes_exact_else_partition_and_route_arrays() {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.transition_count = 1;
        state.predecessor_guard_false_fallthrough_count = 1;
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count = 1;
        state.predecessor_route_counts[20] = 1;
        state.predecessor_guard_false_fallthrough_route_counts[20] = 1;
        state.else_branch_entry_route_counts[20] = 1;
        state.source_site_execution_count = 1;
        let value = lifecycle_json(&PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
            state,
        });
        assert_eq!(value["transition_count"], 1);
        assert_eq!(value["inactive_transition_count"], 0);
        assert_eq!(value["predecessor_guard_false_fallthrough_count"], 1);
        assert_eq!(value["predecessor_maximum_capacity_assignment_count"], 0);
        assert_eq!(
            value["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count"],
            1
        );
        for field in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_maximum_capacity_assignment_route_counts",
            "else_branch_entry_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(30), "{field}");
        }
        assert_eq!(value["source_site_execution_count"], 1);
        assert!(value["latest"].is_null());
    }
}
