//! JSON serialization for one CP318 source-site snapshot.

use ep_runtime::PurchasedAirCalcCoolingSensibleFlowSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(snapshot: PurchasedAirCalcCoolingSensibleFlowSnapshot) -> Value {
    let mut value = json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "source_order": snapshot.source_order,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_maximum_cooling_flow_body_sibling_skipped":
            snapshot.predecessor_maximum_cooling_flow_body_sibling_skipped,
        "predecessor_no_economizer_outer_guard_fallthrough_skipped":
            snapshot.predecessor_no_economizer_outer_guard_fallthrough_skipped,
        "predecessor_economizer_condition_fallthrough_skipped":
            snapshot.predecessor_economizer_condition_fallthrough_skipped,
        "predecessor_economizer_calculation_body_executed":
            snapshot.predecessor_economizer_calculation_body_executed,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "supply_mass_flow_rate_for_cool_reset_assigned":
            snapshot.supply_mass_flow_rate_for_cool_reset_assigned,
        "reset_supply_mass_flow_rate_for_cool_kg_per_s":
            snapshot.reset_supply_mass_flow_rate_for_cool_kg_per_s,
        "cooling_on_read": snapshot.cooling_on_read,
        "cooling_on": snapshot.cooling_on,
        "cooling_on_body_entered": snapshot.cooling_on_body_entered,
    });
    extend_object(
        &mut value,
        json!({
            "zone_humidity_ratio_read": snapshot.zone_humidity_ratio_read,
            "zone_humidity_ratio": snapshot.zone_humidity_ratio,
            "psychrometric_cp_air_evaluated": snapshot.psychrometric_cp_air_evaluated,
            "psychrometric_cp_air_result_j_per_kg_k":
                snapshot.psychrometric_cp_air_result_j_per_kg_k,
            "cp_air_assigned": snapshot.cp_air_assigned,
            "cp_air_j_per_kg_k": snapshot.cp_air_j_per_kg_k,
            "minimum_cooling_supply_air_temperature_read":
                snapshot.minimum_cooling_supply_air_temperature_read,
            "minimum_cooling_supply_air_temperature_c":
                snapshot.minimum_cooling_supply_air_temperature_c,
            "zone_temperature_read": snapshot.zone_temperature_read,
            "zone_temperature_c": snapshot.zone_temperature_c,
            "delta_temperature_calculated": snapshot.delta_temperature_calculated,
            "delta_temperature_c": snapshot.delta_temperature_c,
            "delta_temperature_assigned": snapshot.delta_temperature_assigned,
            "assigned_delta_temperature_c": snapshot.assigned_delta_temperature_c,
            "delta_temperature_for_gate_read":
                snapshot.delta_temperature_for_gate_read,
            "delta_temperature_for_gate_c": snapshot.delta_temperature_for_gate_c,
            "delta_temperature_comparison_evaluated":
                snapshot.delta_temperature_comparison_evaluated,
            "delta_temperature_below_negative_small_temp_diff":
                snapshot.delta_temperature_below_negative_small_temp_diff,
            "delta_temperature_body_entered": snapshot.delta_temperature_body_entered,
        }),
    );
    extend_object(
        &mut value,
        json!({
            "zone_cooling_setpoint_load_read":
                snapshot.zone_cooling_setpoint_load_read,
            "zone_cooling_setpoint_load_w": snapshot.zone_cooling_setpoint_load_w,
            "cp_air_for_first_division_read":
                snapshot.cp_air_for_first_division_read,
            "cp_air_for_first_division_j_per_kg_k":
                snapshot.cp_air_for_first_division_j_per_kg_k,
            "zone_cooling_setpoint_load_over_cp_air_calculated":
                snapshot.zone_cooling_setpoint_load_over_cp_air_calculated,
            "zone_cooling_setpoint_load_over_cp_air_kg_k_per_s":
                snapshot.zone_cooling_setpoint_load_over_cp_air_kg_k_per_s,
            "delta_temperature_for_second_division_read":
                snapshot.delta_temperature_for_second_division_read,
            "delta_temperature_for_second_division_c":
                snapshot.delta_temperature_for_second_division_c,
            "supply_mass_flow_rate_for_cool_calculated":
                snapshot.supply_mass_flow_rate_for_cool_calculated,
            "calculated_supply_mass_flow_rate_for_cool_kg_per_s":
                snapshot.calculated_supply_mass_flow_rate_for_cool_kg_per_s,
            "supply_mass_flow_rate_for_cool_assigned":
                snapshot.supply_mass_flow_rate_for_cool_assigned,
            "assigned_supply_mass_flow_rate_for_cool_kg_per_s":
                snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
            "resulting_supply_mass_flow_rate_for_cool_kg_per_s":
                snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        }),
    );
    value
}

fn extend_object(target: &mut Value, extension: Value) {
    let Value::Object(extension) = extension else {
        return;
    };
    if let Value::Object(target) = target {
        target.extend(extension);
    }
}
