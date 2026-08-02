//! JSON serialization for CP401 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;
#[cfg(test)]
pub(in crate::pipeline) use snapshot::test_snapshot;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count": state.dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp400_supply_humidity_ratio_state_owner_count": state.cp400_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp400_supply_enthalpy_state_owner_count": state.cp400_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp400_supply_temperature_state_owner_count": state.cp400_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cooling_total_output_owned_read_count": state.cooling_total_output_owned_read_count,
        "cooling_total_output_bit_corroboration_count": state.cooling_total_output_bit_corroboration_count,
        "cooling_total_output_read_count": state.cooling_total_output_read_count,
        "cooling_sensible_output_owned_read_count": state.cooling_sensible_output_owned_read_count,
        "cooling_sensible_output_read_count": state.cooling_sensible_output_read_count,
        "cooling_latent_output_calculation_count": state.cooling_latent_output_calculation_count,
        "cooling_latent_output_assignment_write_count": state.cooling_latent_output_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
