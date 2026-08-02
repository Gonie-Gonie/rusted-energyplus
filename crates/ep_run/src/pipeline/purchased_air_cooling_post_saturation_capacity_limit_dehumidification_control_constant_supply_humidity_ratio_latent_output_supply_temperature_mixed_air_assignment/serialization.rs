//! JSON serialization for CP403 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;
#[cfg(test)]
pub(in crate::pipeline) use snapshot::test_snapshot;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "predecessor_guard_false_fallthrough_count": state.predecessor_guard_false_fallthrough_count,
        "supply_temperature_mixed_air_assignment_count": state.supply_temperature_mixed_air_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts,
        "supply_temperature_mixed_air_assignment_route_counts": state.supply_temperature_mixed_air_assignment_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "cp402_supply_humidity_ratio_state_owner_count": state.cp402_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp402_supply_enthalpy_state_owner_count": state.cp402_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp402_supply_temperature_state_owner_count": state.cp402_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp329_mixed_air_temperature_owned_read_count": state.cp329_mixed_air_temperature_owned_read_count,
        "cp402_same_call_mixed_air_temperature_bit_corroboration_count": state.cp402_same_call_mixed_air_temperature_bit_corroboration_count,
        "mixed_air_temperature_read_count": state.mixed_air_temperature_read_count,
        "supply_temperature_assignment_write_count": state.supply_temperature_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
