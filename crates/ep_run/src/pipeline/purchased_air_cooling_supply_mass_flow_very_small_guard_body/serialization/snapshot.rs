//! JSON serialization for one CP328 snapshot.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
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
        "predecessor_ems_supply_mass_flow_override_body_entered":
            snapshot.predecessor_ems_supply_mass_flow_override_body_entered,
        "predecessor_ems_supply_mass_flow_override_body_skipped":
            snapshot.predecessor_ems_supply_mass_flow_override_body_skipped,
        "predecessor_ems_disabled_fallthrough":
            snapshot.predecessor_ems_disabled_fallthrough,
        "predecessor_supply_mass_flow_limit_body_entered":
            snapshot.predecessor_supply_mass_flow_limit_body_entered,
        "predecessor_supply_mass_flow_limit_body_skipped":
            snapshot.predecessor_supply_mass_flow_limit_body_skipped,
        "predecessor_supply_mass_flow_limit_active_guard_false_fallthrough":
            snapshot.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough,
        "predecessor_zero_flow_reset_body_entered":
            snapshot.predecessor_zero_flow_reset_body_entered,
        "predecessor_active_guard_false_fallthrough":
            snapshot.predecessor_active_guard_false_fallthrough,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "zero_flow_reset_body_entered": snapshot.zero_flow_reset_body_entered,
        "body_skipped": snapshot.body_skipped,
        "active_guard_false_fallthrough": snapshot.active_guard_false_fallthrough,
        "predecessor_supply_mass_flow_rate_kg_per_s":
            snapshot.predecessor_supply_mass_flow_rate_kg_per_s,
        "predecessor_supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.predecessor_supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_positive_zero_assignment_performed":
            snapshot.supply_mass_flow_rate_positive_zero_assignment_performed,
        "assigned_supply_mass_flow_rate_kg_per_s":
            snapshot.assigned_supply_mass_flow_rate_kg_per_s,
        "assigned_supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.assigned_supply_mass_flow_rate_kg_per_s),
        "resulting_supply_mass_flow_rate_kg_per_s":
            snapshot.resulting_supply_mass_flow_rate_kg_per_s,
        "resulting_supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.resulting_supply_mass_flow_rate_kg_per_s),
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}
