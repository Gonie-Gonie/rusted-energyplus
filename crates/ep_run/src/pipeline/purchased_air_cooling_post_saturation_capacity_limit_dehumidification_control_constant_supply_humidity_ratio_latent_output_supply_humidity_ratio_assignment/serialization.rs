//! JSON serialization for CP404 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "predecessor_guard_false_fallthrough_count": state.predecessor_guard_false_fallthrough_count,
        "supply_humidity_ratio_assignment_count": state.supply_humidity_ratio_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts,
        "supply_humidity_ratio_assignment_route_counts": state.supply_humidity_ratio_assignment_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp403_supply_humidity_ratio_state_owner_count": state.cp403_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp403_supply_enthalpy_state_owner_count": state.cp403_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp403_supply_temperature_state_owner_count": state.cp403_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "supply_temperature_owned_read_count": state.supply_temperature_owned_read_count,
        "supply_temperature_for_humidity_ratio_inversion_read_count": state.supply_temperature_for_humidity_ratio_inversion_read_count,
        "supply_enthalpy_owned_read_count": state.supply_enthalpy_owned_read_count,
        "cp385_same_call_supply_enthalpy_bit_corroboration_count": state.cp385_same_call_supply_enthalpy_bit_corroboration_count,
        "supply_enthalpy_for_humidity_ratio_inversion_read_count": state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        "psychrometric_supply_humidity_ratio_evaluation_count": state.psychrometric_supply_humidity_ratio_evaluation_count,
        "supply_humidity_ratio_assignment_write_count": state.supply_humidity_ratio_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
