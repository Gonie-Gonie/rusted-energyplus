//! JSON serialization for CP422 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "predecessor_guard_false_fallthrough_count": state.predecessor_guard_false_fallthrough_count,
        "cooling_sensible_output_maximum_capacity_assignment_count": state.cooling_sensible_output_maximum_capacity_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts.as_slice(),
        "cooling_sensible_output_maximum_capacity_assignment_route_counts": state.cooling_sensible_output_maximum_capacity_assignment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp421_supply_humidity_ratio_state_owner_count": state.cp421_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp421_supply_enthalpy_state_owner_count": state.cp421_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp421_supply_temperature_state_owner_count": state.cp421_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp421_retained_maximum_total_cooling_capacity_owned_read_count": state.cp421_retained_maximum_total_cooling_capacity_owned_read_count,
        "maximum_total_cooling_capacity_for_sensible_output_assignment_read_count": state.maximum_total_cooling_capacity_for_sensible_output_assignment_read_count,
        "cooling_sensible_output_maximum_capacity_assignment_write_count": state.cooling_sensible_output_maximum_capacity_assignment_write_count,
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
