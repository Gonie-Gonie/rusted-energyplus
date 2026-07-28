//! JSON serialization for one CP324 body snapshot.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
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
        "predecessor_ems_supply_mass_flow_override_guard_false_fallthrough":
            snapshot.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "body_skipped": snapshot.body_skipped,
        "ems_disabled_fallthrough": snapshot.ems_disabled_fallthrough,
        "ems_supply_mass_flow_override_value_read":
            snapshot.ems_supply_mass_flow_override_value_read,
        "ems_supply_mass_flow_override_value_kg_per_s":
            snapshot.ems_supply_mass_flow_override_value_kg_per_s,
        "supply_mass_flow_rate_override_assignment_performed":
            snapshot.supply_mass_flow_rate_override_assignment_performed,
        "assigned_supply_mass_flow_rate_kg_per_s":
            snapshot.assigned_supply_mass_flow_rate_kg_per_s,
        "outdoor_air_mass_flow_rate_for_minimum_read":
            snapshot.outdoor_air_mass_flow_rate_for_minimum_read,
        "outdoor_air_mass_flow_rate_before_override_kg_per_s":
            snapshot.outdoor_air_mass_flow_rate_before_override_kg_per_s,
        "supply_mass_flow_rate_for_minimum_read":
            snapshot.supply_mass_flow_rate_for_minimum_read,
        "supply_mass_flow_rate_for_minimum_kg_per_s":
            snapshot.supply_mass_flow_rate_for_minimum_kg_per_s,
        "source_shaped_two_argument_minimum_evaluated":
            snapshot.source_shaped_two_argument_minimum_evaluated,
        "minimum_outdoor_air_mass_flow_rate_kg_per_s":
            snapshot.minimum_outdoor_air_mass_flow_rate_kg_per_s,
        "outdoor_air_mass_flow_rate_assignment_performed":
            snapshot.outdoor_air_mass_flow_rate_assignment_performed,
        "assigned_outdoor_air_mass_flow_rate_kg_per_s":
            snapshot.assigned_outdoor_air_mass_flow_rate_kg_per_s,
    })
}
