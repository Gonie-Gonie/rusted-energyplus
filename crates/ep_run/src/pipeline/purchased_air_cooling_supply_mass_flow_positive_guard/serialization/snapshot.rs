//! JSON serialization for one CP330 snapshot.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_call_executed": snapshot.predecessor_cooling_call_executed,
        "predecessor_zero_flow_reset_body_entered":
            snapshot.predecessor_zero_flow_reset_body_entered,
        "predecessor_active_guard_false_fallthrough":
            snapshot.predecessor_active_guard_false_fallthrough,
        "predecessor_no_outdoor_air_fallback_entered":
            snapshot.predecessor_no_outdoor_air_fallback_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s": snapshot.supply_mass_flow_rate_kg_per_s,
        "supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_strictly_positive_comparison_evaluated":
            snapshot.supply_mass_flow_rate_strictly_positive_comparison_evaluated,
        "supply_mass_flow_rate_strictly_positive":
            snapshot.supply_mass_flow_rate_strictly_positive,
        "positive_supply_mass_flow_body_entered":
            snapshot.positive_supply_mass_flow_body_entered,
        "active_guard_false_fallthrough": snapshot.active_guard_false_fallthrough,
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}
