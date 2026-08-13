//! JSON serialization for CP421 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count": state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "guard_false_fallthrough_route_counts": state.guard_false_fallthrough_route_counts.as_slice(),
        "adjustment_body_entry_route_counts": state.adjustment_body_entry_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp420_supply_humidity_ratio_state_owner_count": state.cp420_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp420_supply_enthalpy_state_owner_count": state.cp420_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp420_supply_temperature_state_owner_count": state.cp420_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp420_cooling_sensible_output_owned_read_count": state.cp420_cooling_sensible_output_owned_read_count,
        "cooling_sensible_output_read_count": state.cooling_sensible_output_read_count,
        "cp321_maximum_total_cooling_capacity_owned_read_count": state.cp321_maximum_total_cooling_capacity_owned_read_count,
        "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count": state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        "maximum_total_cooling_capacity_read_count": state.maximum_total_cooling_capacity_read_count,
        "cooling_sensible_output_maximum_total_cooling_capacity_comparison_count": state.cooling_sensible_output_maximum_total_cooling_capacity_comparison_count,
        "cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count": state.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count,
        "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count": state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count,
        "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count": state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_carries_three_route_arrays_and_no_numerical_feed() {
        let source = include_str!("serialization.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("serialization.rs"), |(production, _)| {
                production
            });
        assert_eq!(source.matches("route_counts\":").count(), 3);
        for forbidden in [
            "DirectZonePurchasedAirCouplingInput",
            "numerical_dto",
            "prediction",
            "feedback",
            "nodes",
            "loads",
            "reports",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
