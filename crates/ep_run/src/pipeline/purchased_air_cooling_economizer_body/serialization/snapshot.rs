//! JSON serialization for one CP317 source-site snapshot.

use ep_model::IdealLoadsLimit;
use ep_runtime::PurchasedAirCalcCoolingEconomizerBodySnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(snapshot: PurchasedAirCalcCoolingEconomizerBodySnapshot) -> Value {
    let mut value = json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "source_order": snapshot.source_order,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_maximum_cooling_flow_body_entered":
            snapshot.predecessor_maximum_cooling_flow_body_entered,
        "predecessor_active_guard_false_economizer_fallthrough":
            snapshot.predecessor_active_guard_false_economizer_fallthrough,
        "predecessor_economizer_guard_evaluated":
            snapshot.predecessor_economizer_guard_evaluated,
        "predecessor_economizer_body_entered":
            snapshot.predecessor_economizer_body_entered,
        "predecessor_no_economizer_fallthrough":
            snapshot.predecessor_no_economizer_fallthrough,
        "predecessor_economizer_condition_evaluated":
            snapshot.predecessor_economizer_condition_evaluated,
        "predecessor_economizer_condition_satisfied":
            snapshot.predecessor_economizer_condition_satisfied,
        "predecessor_economizer_calculation_body_entered":
            snapshot.predecessor_economizer_calculation_body_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "maximum_cooling_flow_body_sibling_skipped":
            snapshot.maximum_cooling_flow_body_sibling_skipped,
        "no_economizer_outer_guard_fallthrough_skipped":
            snapshot.no_economizer_outer_guard_fallthrough_skipped,
        "economizer_condition_fallthrough_skipped":
            snapshot.economizer_condition_fallthrough_skipped,
        "economizer_calculation_body_executed":
            snapshot.economizer_calculation_body_executed,
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
            "outdoor_air_temperature_read": snapshot.outdoor_air_temperature_read,
            "outdoor_air_temperature_c": snapshot.outdoor_air_temperature_c,
            "zone_temperature_read": snapshot.zone_temperature_read,
            "zone_temperature_c": snapshot.zone_temperature_c,
            "delta_temperature_calculated": snapshot.delta_temperature_calculated,
            "delta_temperature_c": snapshot.delta_temperature_c,
            "delta_temperature_assigned": snapshot.delta_temperature_assigned,
            "assigned_delta_temperature_c": snapshot.assigned_delta_temperature_c,
            "delta_temperature_for_gate_read": snapshot.delta_temperature_for_gate_read,
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
            "zone_cooling_setpoint_load_read": snapshot.zone_cooling_setpoint_load_read,
            "zone_cooling_setpoint_load_w": snapshot.zone_cooling_setpoint_load_w,
            "cp_air_for_first_division_read": snapshot.cp_air_for_first_division_read,
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
            "supply_mass_flow_rate_calculated": snapshot.supply_mass_flow_rate_calculated,
            "calculated_supply_mass_flow_rate_kg_per_s":
                snapshot.calculated_supply_mass_flow_rate_kg_per_s,
            "initial_supply_mass_flow_rate_assigned":
                snapshot.initial_supply_mass_flow_rate_assigned,
            "initial_supply_mass_flow_rate_kg_per_s":
                snapshot.initial_supply_mass_flow_rate_kg_per_s,
            "cooling_limit_flow_rate_comparison_evaluated":
                snapshot.cooling_limit_flow_rate_comparison_evaluated,
            "cooling_limit_flow_rate_read": snapshot.cooling_limit_flow_rate_read,
            "cooling_limit_flow_rate_value":
                snapshot.cooling_limit_flow_rate_value.map(limit_name),
            "cooling_limit_flow_rate_comparison_satisfied":
                snapshot.cooling_limit_flow_rate_comparison_satisfied,
            "cooling_limit_flow_rate_and_capacity_comparison_evaluated":
                snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated,
            "cooling_limit_flow_rate_and_capacity_read":
                snapshot.cooling_limit_flow_rate_and_capacity_read,
            "cooling_limit_flow_rate_and_capacity_value":
                snapshot.cooling_limit_flow_rate_and_capacity_value.map(limit_name),
            "cooling_limit_flow_rate_and_capacity_comparison_satisfied":
                snapshot.cooling_limit_flow_rate_and_capacity_comparison_satisfied,
            "cooling_flow_limit_active": snapshot.cooling_flow_limit_active,
        }),
    );
    extend_object(
        &mut value,
        json!({
            "maximum_cooling_air_mass_flow_rate_read":
                snapshot.maximum_cooling_air_mass_flow_rate_read,
            "maximum_cooling_air_mass_flow_rate_kg_per_s":
                snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
            "maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated":
                snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated,
            "maximum_cooling_air_mass_flow_rate_positive":
                snapshot.maximum_cooling_air_mass_flow_rate_positive,
            "maximum_flow_clamp_body_entered": snapshot.maximum_flow_clamp_body_entered,
            "supply_mass_flow_rate_clamped": snapshot.supply_mass_flow_rate_clamped,
            "supply_mass_flow_rate_for_clamp_read":
                snapshot.supply_mass_flow_rate_for_clamp_read,
            "supply_mass_flow_rate_for_clamp_kg_per_s":
                snapshot.supply_mass_flow_rate_for_clamp_kg_per_s,
            "inner_max_evaluated": snapshot.inner_max_evaluated,
            "nonnegative_supply_mass_flow_rate_kg_per_s":
                snapshot.nonnegative_supply_mass_flow_rate_kg_per_s,
            "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read":
                snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read,
            "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s":
                snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s,
            "outer_min_evaluated": snapshot.outer_min_evaluated,
            "clamped_supply_mass_flow_rate_kg_per_s":
                snapshot.clamped_supply_mass_flow_rate_kg_per_s,
            "clamped_supply_mass_flow_rate_assigned":
                snapshot.clamped_supply_mass_flow_rate_assigned,
            "resulting_supply_mass_flow_rate_kg_per_s":
                snapshot.resulting_supply_mass_flow_rate_kg_per_s,
        }),
    );
    extend_object(
        &mut value,
        json!({
            "resulting_supply_mass_flow_rate_read":
                snapshot.resulting_supply_mass_flow_rate_read,
            "outdoor_air_mass_flow_rate_read": snapshot.outdoor_air_mass_flow_rate_read,
            "outdoor_air_mass_flow_rate_kg_per_s":
                snapshot.outdoor_air_mass_flow_rate_kg_per_s,
            "supply_above_outdoor_air_mass_flow_comparison_evaluated":
                snapshot.supply_above_outdoor_air_mass_flow_comparison_evaluated,
            "supply_mass_flow_above_outdoor_air_mass_flow":
                snapshot.supply_mass_flow_above_outdoor_air_mass_flow,
            "economizer_activation_body_entered":
                snapshot.economizer_activation_body_entered,
            "economizer_on_assigned": snapshot.economizer_on_assigned,
            "economizer_on": snapshot.economizer_on,
            "supply_mass_flow_rate_for_outdoor_air_assignment_read":
                snapshot.supply_mass_flow_rate_for_outdoor_air_assignment_read,
            "supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s":
                snapshot.supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s,
            "outdoor_air_mass_flow_rate_assigned":
                snapshot.outdoor_air_mass_flow_rate_assigned,
            "assigned_outdoor_air_mass_flow_rate_kg_per_s":
                snapshot.assigned_outdoor_air_mass_flow_rate_kg_per_s,
            "system_time_step_read": snapshot.system_time_step_read,
            "system_time_step_hours": snapshot.system_time_step_hours,
            "economizer_active_time_assigned": snapshot.economizer_active_time_assigned,
            "assigned_economizer_active_time_hours":
                snapshot.assigned_economizer_active_time_hours,
        }),
    );
    value
}

fn limit_name(limit: IdealLoadsLimit) -> &'static str {
    match limit {
        IdealLoadsLimit::NoLimit => "NoLimit",
        IdealLoadsLimit::LimitFlowRate => "LimitFlowRate",
        IdealLoadsLimit::LimitCapacity => "LimitCapacity",
        IdealLoadsLimit::LimitFlowRateAndCapacity => "LimitFlowRateAndCapacity",
    }
}

fn extend_object(target: &mut Value, extension: Value) {
    let Value::Object(extension) = extension else {
        return;
    };
    if let Value::Object(target) = target {
        target.extend(extension);
    }
}
