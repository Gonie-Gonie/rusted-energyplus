//! JSON serialization for CP420 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;
use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "predecessor_supply_temperature_saturation_assignment_count": state.predecessor_supply_temperature_saturation_assignment_count,
        "predecessor_supply_temperature_saturation_mixed_air_limit_count": state.predecessor_supply_temperature_saturation_mixed_air_limit_count,
        "predecessor_supply_humidity_ratio_assignment_count": state.predecessor_supply_humidity_ratio_assignment_count,
        "predecessor_supply_enthalpy_assignment_count": state.predecessor_supply_enthalpy_assignment_count,
        "predecessor_dehumidification_guard_else_branch_entry_count": state.predecessor_dehumidification_guard_else_branch_entry_count,
        "predecessor_dehumidification_guard_else_branch_cp_air_assignment_count": state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_count,
        "dehumidification_guard_else_branch_sensible_output_assignment_count": state.dehumidification_guard_else_branch_sensible_output_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts.as_slice(),
        "predecessor_guard_body_entry_route_counts": state.predecessor_guard_body_entry_route_counts.as_slice(),
        "predecessor_supply_temperature_saturation_assignment_route_counts": state.predecessor_supply_temperature_saturation_assignment_route_counts.as_slice(),
        "predecessor_supply_temperature_mixed_air_limit_route_counts": state.predecessor_supply_temperature_mixed_air_limit_route_counts.as_slice(),
        "predecessor_supply_humidity_ratio_assignment_route_counts": state.predecessor_supply_humidity_ratio_assignment_route_counts.as_slice(),
        "predecessor_supply_enthalpy_assignment_route_counts": state.predecessor_supply_enthalpy_assignment_route_counts.as_slice(),
        "predecessor_dehumidification_guard_else_branch_entry_route_counts": state.predecessor_dehumidification_guard_else_branch_entry_route_counts.as_slice(),
        "predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts": state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts.as_slice(),
        "dehumidification_guard_else_branch_sensible_output_assignment_route_counts": state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp419_supply_humidity_ratio_state_owner_count": state.cp419_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp419_supply_enthalpy_state_owner_count": state.cp419_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp419_supply_temperature_state_owner_count": state.cp419_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "supply_mass_flow_rate_owned_read_count": state.supply_mass_flow_rate_owned_read_count,
        "supply_mass_flow_rate_bit_corroboration_count": state.supply_mass_flow_rate_bit_corroboration_count,
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "cp_air_owned_read_count": state.cp_air_owned_read_count,
        "cp_air_read_count": state.cp_air_read_count,
        "supply_mass_flow_rate_times_cp_air_calculation_count": state.supply_mass_flow_rate_times_cp_air_calculation_count,
        "mixed_air_temperature_owned_read_count": state.mixed_air_temperature_owned_read_count,
        "mixed_air_temperature_read_count": state.mixed_air_temperature_read_count,
        "supply_temperature_owned_read_count": state.supply_temperature_owned_read_count,
        "supply_temperature_read_count": state.supply_temperature_read_count,
        "mixed_air_minus_supply_temperature_calculation_count": state.mixed_air_minus_supply_temperature_calculation_count,
        "cooling_sensible_output_calculation_count": state.cooling_sensible_output_calculation_count,
        "cooling_sensible_output_assignment_write_count": state.cooling_sensible_output_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn serializer_source_carries_ten_route_arrays_and_no_numerical_dto() {
        let source = include_str!("serialization.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("serialization.rs"), |(production, _)| {
                production
            });
        assert_eq!(source.matches("route_counts\":").count(), 10);
        for forbidden in [
            "zone_sensible_cooling_rate_w",
            "prediction",
            "feedback",
            "reports",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
