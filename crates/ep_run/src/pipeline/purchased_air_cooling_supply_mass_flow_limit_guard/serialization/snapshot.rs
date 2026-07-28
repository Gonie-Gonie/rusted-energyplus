//! JSON serialization for one CP325 snapshot.

use ep_model::IdealLoadsLimit;
use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
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
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "first_cooling_limit_read": snapshot.first_cooling_limit_read,
        "first_cooling_limit": snapshot.first_cooling_limit.map(limit_name),
        "cooling_limit_flow_rate_comparison_evaluated":
            snapshot.cooling_limit_flow_rate_comparison_evaluated,
        "cooling_limit_flow_rate": snapshot.cooling_limit_flow_rate,
        "second_cooling_limit_read": snapshot.second_cooling_limit_read,
        "second_cooling_limit": snapshot.second_cooling_limit.map(limit_name),
        "cooling_limit_flow_rate_and_capacity_comparison_evaluated":
            snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        "cooling_limit_flow_rate_and_capacity":
            snapshot.cooling_limit_flow_rate_and_capacity,
        "cooling_limit_condition_satisfied": snapshot.cooling_limit_condition_satisfied,
        "maximum_cooling_air_mass_flow_rate_read":
            snapshot.maximum_cooling_air_mass_flow_rate_read,
        "maximum_cooling_air_mass_flow_rate_kg_per_s":
            snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
        "maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s),
        "maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated":
            snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated,
        "maximum_cooling_air_mass_flow_rate_strictly_positive":
            snapshot.maximum_cooling_air_mass_flow_rate_strictly_positive,
        "supply_mass_flow_limit_body_entered":
            snapshot.supply_mass_flow_limit_body_entered,
        "active_guard_false_fallthrough": snapshot.active_guard_false_fallthrough,
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}

fn limit_name(limit: IdealLoadsLimit) -> &'static str {
    match limit {
        IdealLoadsLimit::NoLimit => "NoLimit",
        IdealLoadsLimit::LimitFlowRate => "LimitFlowRate",
        IdealLoadsLimit::LimitCapacity => "LimitCapacity",
        IdealLoadsLimit::LimitFlowRateAndCapacity => "LimitFlowRateAndCapacity",
    }
}
