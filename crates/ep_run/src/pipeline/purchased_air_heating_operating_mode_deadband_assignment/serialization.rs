//! JSON serialization for CP434 lifecycle evidence.

use ep_runtime::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "heating_operating_mode_deadband_assignment_count": state.heating_operating_mode_deadband_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "heating_operating_mode_deadband_assignment_route_counts": state.heating_operating_mode_deadband_assignment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp433_supply_humidity_ratio_state_owner_count": state.cp433_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp433_supply_enthalpy_state_owner_count": state.cp433_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp433_supply_temperature_state_owner_count": state.cp433_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp434_heating_operating_mode_state_owner_count": state.cp434_heating_operating_mode_state_owner_count,
        "heating_operating_mode_assignment_write_count": state.heating_operating_mode_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_carries_exactly_two_route_arrays_and_no_numerical_feed() {
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
            "calculation.mode",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
