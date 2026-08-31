//! JSON serialization for CP435 lifecycle evidence.

use ep_runtime::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "heating_outdoor_air_maximum_flow_guard_evaluation_count": state.heating_outdoor_air_maximum_flow_guard_evaluation_count,
        "heating_outdoor_air_maximum_flow_guard_false_fallthrough_count": state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_count,
        "maximum_heating_flow_body_entry_count": state.maximum_heating_flow_body_entry_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts": state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts.as_slice(),
        "maximum_heating_flow_body_entry_route_counts": state.maximum_heating_flow_body_entry_route_counts.as_slice(),
        "heating_limit_flow_rate_comparison_count": state.heating_limit_flow_rate_comparison_count,
        "heating_limit_flow_rate_match_count": state.heating_limit_flow_rate_match_count,
        "heating_limit_flow_rate_and_capacity_comparison_count": state.heating_limit_flow_rate_and_capacity_comparison_count,
        "heating_limit_flow_rate_and_capacity_match_count": state.heating_limit_flow_rate_and_capacity_match_count,
        "heating_flow_limit_selector_rejection_count": state.heating_flow_limit_selector_rejection_count,
        "cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count": state.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count,
        "outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count": state.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count,
        "maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count": state.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count,
        "outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count": state.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count,
        "outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count": state.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count,
        "source_site_execution_count": state.source_site_execution_count,
        "cp434_supply_humidity_ratio_state_owner_count": state.cp434_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp434_supply_enthalpy_state_owner_count": state.cp434_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp434_supply_temperature_state_owner_count": state.cp434_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_carries_exactly_three_route_arrays_and_no_numerical_feed() {
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
            "calculation.mode",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
