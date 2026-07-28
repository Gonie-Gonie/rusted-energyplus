//! JSON serialization for one CP332 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
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
        "supply_temperature_assignment_executed":
            snapshot.supply_temperature_assignment_executed,
        "zone_cooling_setpoint_load_read": snapshot.zone_cooling_setpoint_load_read,
        "zone_cooling_setpoint_load_w": snapshot.zone_cooling_setpoint_load_w,
        "zone_cooling_setpoint_load_w_ieee_bits":
            ieee_bits(snapshot.zone_cooling_setpoint_load_w),
        "cp_air_read": snapshot.cp_air_read,
        "cp_air_j_per_kg_k": snapshot.cp_air_j_per_kg_k,
        "cp_air_j_per_kg_k_ieee_bits": ieee_bits(snapshot.cp_air_j_per_kg_k),
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s": snapshot.supply_mass_flow_rate_kg_per_s,
        "supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "cp_air_times_supply_mass_flow_rate_calculated":
            snapshot.cp_air_times_supply_mass_flow_rate_calculated,
        "cp_air_times_supply_mass_flow_rate_w_per_k":
            snapshot.cp_air_times_supply_mass_flow_rate_w_per_k,
        "cp_air_times_supply_mass_flow_rate_w_per_k_ieee_bits":
            ieee_bits(snapshot.cp_air_times_supply_mass_flow_rate_w_per_k),
        "zone_cooling_setpoint_load_over_denominator_calculated":
            snapshot.zone_cooling_setpoint_load_over_denominator_calculated,
        "zone_cooling_setpoint_load_over_denominator_c":
            snapshot.zone_cooling_setpoint_load_over_denominator_c,
        "zone_cooling_setpoint_load_over_denominator_c_ieee_bits":
            ieee_bits(snapshot.zone_cooling_setpoint_load_over_denominator_c),
        "zone_node_temperature_read": snapshot.zone_node_temperature_read,
        "zone_node_temperature_c": snapshot.zone_node_temperature_c,
        "zone_node_temperature_c_ieee_bits": ieee_bits(snapshot.zone_node_temperature_c),
        "supply_temperature_calculated": snapshot.supply_temperature_calculated,
        "calculated_supply_temperature_c": snapshot.calculated_supply_temperature_c,
        "calculated_supply_temperature_c_ieee_bits":
            ieee_bits(snapshot.calculated_supply_temperature_c),
        "supply_temperature_assigned": snapshot.supply_temperature_assigned,
        "supply_temperature_c": snapshot.supply_temperature_c,
        "supply_temperature_c_ieee_bits": ieee_bits(snapshot.supply_temperature_c),
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}
