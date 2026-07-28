//! JSON serialization for one CP336 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_no_outdoor_air_fallback_entered":
            snapshot.predecessor_no_outdoor_air_fallback_entered,
        "predecessor_positive_supply_mass_flow_body_entered":
            snapshot.predecessor_positive_supply_mass_flow_body_entered,
        "predecessor_active_guard_false_fallthrough":
            snapshot.predecessor_active_guard_false_fallthrough,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "supply_enthalpy_assignment_executed":
            snapshot.supply_enthalpy_assignment_executed,
        "supply_temperature_for_enthalpy_read":
            snapshot.supply_temperature_for_enthalpy_read,
        "supply_temperature_c": snapshot.supply_temperature_c,
        "supply_temperature_c_ieee_bits": ieee_bits(snapshot.supply_temperature_c),
        "supply_humidity_ratio_for_enthalpy_read":
            snapshot.supply_humidity_ratio_for_enthalpy_read,
        "supply_humidity_ratio": snapshot.supply_humidity_ratio,
        "supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.supply_humidity_ratio),
        "psychrometric_supply_enthalpy_evaluated":
            snapshot.psychrometric_supply_enthalpy_evaluated,
        "psychrometric_supply_enthalpy_result_j_per_kg":
            snapshot.psychrometric_supply_enthalpy_result_j_per_kg,
        "psychrometric_supply_enthalpy_result_j_per_kg_ieee_bits":
            ieee_bits(snapshot.psychrometric_supply_enthalpy_result_j_per_kg),
        "supply_enthalpy_assigned": snapshot.supply_enthalpy_assigned,
        "supply_enthalpy_j_per_kg": snapshot.supply_enthalpy_j_per_kg,
        "supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.supply_enthalpy_j_per_kg),
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}
