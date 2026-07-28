//! JSON serialization for one CP323 guard snapshot.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
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
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "ems_supply_mass_flow_override_flag_read":
            snapshot.ems_supply_mass_flow_override_flag_read,
        "ems_supply_mass_flow_override_enabled":
            snapshot.ems_supply_mass_flow_override_enabled,
        "ems_supply_mass_flow_override_guard_evaluated":
            snapshot.ems_supply_mass_flow_override_guard_evaluated,
        "ems_supply_mass_flow_override_body_entered":
            snapshot.ems_supply_mass_flow_override_body_entered,
        "ems_supply_mass_flow_override_guard_false_fallthrough":
            snapshot.ems_supply_mass_flow_override_guard_false_fallthrough,
    })
}
