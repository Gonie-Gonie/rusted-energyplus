//! JSON serialization for CP431 lifecycle evidence.

use ep_runtime::PurchasedAirCalcHeatingModeGuardLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcHeatingModeGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "heating_mode_guard_evaluation_count": state.heating_mode_guard_evaluation_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "heating_mode_guard_evaluation_route_counts": state.heating_mode_guard_evaluation_route_counts.as_slice(),
        "heating_operating_mode_body_entry_route_counts": state.heating_operating_mode_body_entry_route_counts.as_slice(),
        "heating_mode_guard_false_fallthrough_route_counts": state.heating_mode_guard_false_fallthrough_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp430_supply_humidity_ratio_state_owner_count": state.cp430_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp430_supply_enthalpy_state_owner_count": state.cp430_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp430_supply_temperature_state_owner_count": state.cp430_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count": state.cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count,
        "cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count": state.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count,
        "minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count": state.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count,
        "cp310_retained_heating_setpoint_demand_owner_read_count": state.cp310_retained_heating_setpoint_demand_owner_read_count,
        "heating_setpoint_demand_for_heating_mode_guard_read_count": state.heating_setpoint_demand_for_heating_mode_guard_read_count,
        "minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count": state.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count,
        "minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count": state.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count,
        "prevalidated_temperature_control_type_owner_read_count": state.prevalidated_temperature_control_type_owner_read_count,
        "temperature_control_type_read_after_sensible_comparison_short_circuit_count": state.temperature_control_type_read_after_sensible_comparison_short_circuit_count,
        "temperature_control_type_single_cool_comparison_count": state.temperature_control_type_single_cool_comparison_count,
        "temperature_control_type_permits_heating_count": state.temperature_control_type_permits_heating_count,
        "single_cool_block_count": state.single_cool_block_count,
        "heating_operating_mode_body_entry_count": state.heating_operating_mode_body_entry_count,
        "heating_mode_guard_false_fallthrough_count": state.heating_mode_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_carries_four_route_arrays_and_no_numerical_feed() {
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
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
