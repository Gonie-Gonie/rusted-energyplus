//! JSON serialization for CP405 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "predecessor_guard_false_fallthrough_count": state.predecessor_guard_false_fallthrough_count,
        "cooling_latent_output_maximum_capacity_assignment_count": state.cooling_latent_output_maximum_capacity_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts,
        "cooling_latent_output_maximum_capacity_assignment_route_counts": state.cooling_latent_output_maximum_capacity_assignment_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp404_supply_humidity_ratio_state_owner_count": state.cp404_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp404_supply_enthalpy_state_owner_count": state.cp404_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp404_supply_temperature_state_owner_count": state.cp404_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp404_retained_maximum_total_cooling_capacity_owned_read_count": state.cp404_retained_maximum_total_cooling_capacity_owned_read_count,
        "maximum_total_cooling_capacity_read_count": state.maximum_total_cooling_capacity_read_count,
        "cooling_latent_output_assignment_write_count": state.cooling_latent_output_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
