//! JSON serialization for CP429 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "zero_supply_mass_flow_total_output_positive_zero_assignment_count": state.zero_supply_mass_flow_total_output_positive_zero_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "zero_supply_mass_flow_total_output_positive_zero_assignment_route_counts": state.zero_supply_mass_flow_total_output_positive_zero_assignment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp428_supply_humidity_ratio_state_owner_count": state.cp428_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp428_supply_enthalpy_state_owner_count": state.cp428_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp428_supply_temperature_state_owner_count": state.cp428_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp429_cooling_total_output_state_owner_count": state.cp429_cooling_total_output_state_owner_count,
        "cooling_total_output_assignment_write_count": state.cooling_total_output_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_carries_two_route_arrays_and_no_numerical_feed() {
        let source = include_str!("serialization.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("serialization.rs"), |(production, _)| {
                production
            });
        assert_eq!(source.matches("route_counts\":").count(), 2);
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
