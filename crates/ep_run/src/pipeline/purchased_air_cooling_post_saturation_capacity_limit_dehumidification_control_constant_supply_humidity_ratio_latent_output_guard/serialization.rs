//! JSON serialization for CP402 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;
#[cfg(test)]
pub(in crate::pipeline) use snapshot::test_snapshot;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count": state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "guard_false_fallthrough_route_counts": state.guard_false_fallthrough_route_counts,
        "adjustment_body_entry_route_counts": state.adjustment_body_entry_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp401_supply_humidity_ratio_state_owner_count": state.cp401_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp401_supply_enthalpy_state_owner_count": state.cp401_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp401_supply_temperature_state_owner_count": state.cp401_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp401_cooling_latent_output_owned_read_count": state.cp401_cooling_latent_output_owned_read_count,
        "cooling_latent_output_read_count": state.cooling_latent_output_read_count,
        "cp321_maximum_total_cooling_capacity_owned_read_count": state.cp321_maximum_total_cooling_capacity_owned_read_count,
        "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count": state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        "maximum_total_cooling_capacity_read_count": state.maximum_total_cooling_capacity_read_count,
        "cooling_latent_output_maximum_total_cooling_capacity_comparison_count": state.cooling_latent_output_maximum_total_cooling_capacity_comparison_count,
        "cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count": state.cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count": state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count": state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}
