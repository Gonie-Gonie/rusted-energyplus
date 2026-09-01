//! JSON serialization for CP436 lifecycle evidence.

use ep_runtime::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "outdoor_air_volume_flow_assignment_count": state.outdoor_air_volume_flow_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts.as_slice(),
        "predecessor_guard_body_entry_route_counts": state.predecessor_guard_body_entry_route_counts.as_slice(),
        "heating_outdoor_air_volume_flow_assignment_route_counts": state.heating_outdoor_air_volume_flow_assignment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp435_supply_humidity_ratio_state_owner_count": state.cp435_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp435_supply_enthalpy_state_owner_count": state.cp435_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp435_supply_temperature_state_owner_count": state.cp435_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp435_outdoor_air_mass_flow_rate_owned_read_count": state.cp435_outdoor_air_mass_flow_rate_owned_read_count,
        "outdoor_air_mass_flow_rate_for_volume_flow_division_read_count": state.outdoor_air_mass_flow_rate_for_volume_flow_division_read_count,
        "begin_environment_standard_air_density_owner_count": state.begin_environment_standard_air_density_owner_count,
        "standard_air_density_for_volume_flow_division_read_count": state.standard_air_density_for_volume_flow_division_read_count,
        "outdoor_air_mass_flow_rate_standard_air_density_division_count": state.outdoor_air_mass_flow_rate_standard_air_density_division_count,
        "local_outdoor_air_volume_flow_rate_assignment_write_count": state.local_outdoor_air_volume_flow_rate_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_carries_four_exact_route_arrays_and_no_numerical_feed() {
        let source = include_str!("serialization.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("serialization.rs"), |(production, _)| {
                production
            });
        assert_eq!(source.matches("route_counts\":").count(), 4);
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
