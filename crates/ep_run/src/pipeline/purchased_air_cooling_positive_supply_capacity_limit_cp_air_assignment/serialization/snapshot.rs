//! JSON serialization for one CP338 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
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
        "predecessor_capacity_limit_guard_evaluated":
            snapshot.predecessor_capacity_limit_guard_evaluated,
        "predecessor_capacity_limit_body_entered":
            snapshot.predecessor_capacity_limit_body_entered,
        "predecessor_active_capacity_limit_guard_false_fallthrough":
            snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        "capacity_limit_guard_false_fallthrough_skipped":
            snapshot.capacity_limit_guard_false_fallthrough_skipped,
        "capacity_limit_cp_air_assignment_executed":
            snapshot.capacity_limit_cp_air_assignment_executed,
        "mixed_air_humidity_ratio_read": snapshot.mixed_air_humidity_ratio_read,
        "mixed_air_humidity_ratio": snapshot.mixed_air_humidity_ratio,
        "mixed_air_humidity_ratio_ieee_bits": ieee_bits(snapshot.mixed_air_humidity_ratio),
        "psychrometric_cp_air_evaluated": snapshot.psychrometric_cp_air_evaluated,
        "psychrometric_cp_air_result_j_per_kg_k":
            snapshot.psychrometric_cp_air_result_j_per_kg_k,
        "psychrometric_cp_air_result_j_per_kg_k_ieee_bits":
            ieee_bits(snapshot.psychrometric_cp_air_result_j_per_kg_k),
        "cp_air_assigned": snapshot.cp_air_assigned,
        "cp_air_j_per_kg_k": snapshot.cp_air_j_per_kg_k,
        "cp_air_j_per_kg_k_ieee_bits": ieee_bits(snapshot.cp_air_j_per_kg_k),
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}
