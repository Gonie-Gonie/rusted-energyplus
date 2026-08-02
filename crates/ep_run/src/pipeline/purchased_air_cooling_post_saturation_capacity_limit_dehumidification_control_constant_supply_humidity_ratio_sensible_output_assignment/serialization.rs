//! JSON serialization for CP400 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;
#[cfg(test)]
pub(in crate::pipeline) use snapshot::test_snapshot;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count": state.dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp399_supply_humidity_ratio_state_owner_count": state.cp399_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp399_supply_enthalpy_state_owner_count": state.cp399_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp399_supply_temperature_state_owner_count": state.cp399_supply_temperature_state_owner_count,
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
