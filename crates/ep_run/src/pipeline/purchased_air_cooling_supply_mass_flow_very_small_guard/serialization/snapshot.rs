//! JSON serialization for one CP327 snapshot.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
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
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s": snapshot.supply_mass_flow_rate_kg_per_s,
        "supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "hvac_very_small_mass_flow_read": snapshot.hvac_very_small_mass_flow_read,
        "hvac_very_small_mass_flow_source": snapshot.hvac_very_small_mass_flow_source,
        "hvac_very_small_mass_flow_kg_per_s": snapshot.hvac_very_small_mass_flow_kg_per_s,
        "hvac_very_small_mass_flow_kg_per_s_ieee_bits":
            ieee_bits(snapshot.hvac_very_small_mass_flow_kg_per_s),
        "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated":
            snapshot
                .supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated,
        "supply_mass_flow_rate_at_or_below_very_small_mass_flow":
            snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow,
        "zero_flow_reset_body_entered": snapshot.zero_flow_reset_body_entered,
        "active_guard_false_fallthrough": snapshot.active_guard_false_fallthrough,
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}
