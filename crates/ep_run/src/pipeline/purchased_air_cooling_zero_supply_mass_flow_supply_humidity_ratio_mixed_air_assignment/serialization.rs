//! JSON serialization for CP426 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_count": state.zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_route_counts": state.zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp425_supply_humidity_ratio_state_owner_count": state.cp425_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp425_supply_enthalpy_state_owner_count": state.cp425_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp425_supply_temperature_state_owner_count": state.cp425_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp426_supply_humidity_ratio_state_owner_count": state.cp426_supply_humidity_ratio_state_owner_count,
        "cp329_retained_mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_owned_read_count": state.cp329_retained_mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_owned_read_count,
        "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_read_count": state.mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_read_count,
        "supply_humidity_ratio_assignment_write_count": state.supply_humidity_ratio_assignment_write_count,
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
