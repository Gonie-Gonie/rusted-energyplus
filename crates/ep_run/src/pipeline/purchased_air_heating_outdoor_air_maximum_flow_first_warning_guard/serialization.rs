//! JSON serialization for CP437 lifecycle evidence.

use ep_runtime::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "guard_evaluation_count": state.guard_evaluation_count,
        "first_warning_branch_entry_count": state.first_warning_branch_entry_count,
        "guard_false_fallthrough_count": state.guard_false_fallthrough_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts.as_slice(),
        "predecessor_guard_body_entry_route_counts": state.predecessor_guard_body_entry_route_counts.as_slice(),
        "predecessor_volume_flow_assignment_route_counts": state.predecessor_volume_flow_assignment_route_counts.as_slice(),
        "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts": state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts.as_slice(),
        "heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts": state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp436_supply_humidity_ratio_state_owner_count": state.cp436_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp436_supply_enthalpy_state_owner_count": state.cp436_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp436_supply_temperature_state_owner_count": state.cp436_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "outdoor_air_flow_maximum_heating_output_error_count": state.outdoor_air_flow_maximum_heating_output_error_count,
        "outdoor_air_flow_maximum_heating_output_error_count_state_owner_count": state.outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        "outdoor_air_flow_maximum_heating_output_error_count_read_count": state.outdoor_air_flow_maximum_heating_output_error_count_read_count,
        "outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count": state.outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_carries_six_exact_route_arrays_and_no_numerical_feed() {
        let source = include_str!("serialization.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("serialization.rs"), |(production, _)| {
                production
            });
        assert_eq!(source.matches("route_counts\":").count(), 6);
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
