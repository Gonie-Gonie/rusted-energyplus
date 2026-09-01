//! JSON serialization for CP438 lifecycle evidence.

use ep_runtime::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "outdoor_air_flow_maximum_heating_output_error_count_increment_count": state.outdoor_air_flow_maximum_heating_output_error_count_increment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts.as_slice(),
        "predecessor_guard_body_entry_route_counts": state.predecessor_guard_body_entry_route_counts.as_slice(),
        "predecessor_volume_flow_assignment_route_counts": state.predecessor_volume_flow_assignment_route_counts.as_slice(),
        "predecessor_first_warning_guard_false_fallthrough_route_counts": state.predecessor_first_warning_guard_false_fallthrough_route_counts.as_slice(),
        "predecessor_first_warning_branch_entry_route_counts": state.predecessor_first_warning_branch_entry_route_counts.as_slice(),
        "heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts": state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp437_supply_humidity_ratio_state_owner_count": state.cp437_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp437_supply_enthalpy_state_owner_count": state.cp437_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp437_supply_temperature_state_owner_count": state.cp437_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count": state.cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        "outdoor_air_flow_maximum_heating_output_error_count_increment_write_count": state.outdoor_air_flow_maximum_heating_output_error_count_increment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_carries_seven_exact_route_arrays_and_no_numerical_feed() {
        let source = include_str!("serialization.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("serialization.rs"), |(production, _)| {
                production
            });
        assert_eq!(source.matches("route_counts\":").count(), 7);
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
