//! JSON serialization for CP317 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    let mut value = json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "body_execution_count": state.body_execution_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "maximum_cooling_flow_body_sibling_skip_count":
            state.maximum_cooling_flow_body_sibling_skip_count,
        "no_economizer_outer_guard_fallthrough_skip_count":
            state.no_economizer_outer_guard_fallthrough_skip_count,
        "economizer_condition_fallthrough_skip_count":
            state.economizer_condition_fallthrough_skip_count,
    });
    extend_object(
        &mut value,
        json!({
            "zone_humidity_ratio_read_count": state.zone_humidity_ratio_read_count,
            "psychrometric_cp_air_evaluation_count":
                state.psychrometric_cp_air_evaluation_count,
            "cp_air_assignment_count": state.cp_air_assignment_count,
            "outdoor_air_temperature_read_count": state.outdoor_air_temperature_read_count,
            "zone_temperature_read_count": state.zone_temperature_read_count,
            "delta_temperature_calculation_count": state.delta_temperature_calculation_count,
            "delta_temperature_assignment_count": state.delta_temperature_assignment_count,
            "delta_temperature_for_gate_read_count":
                state.delta_temperature_for_gate_read_count,
            "delta_temperature_comparison_count": state.delta_temperature_comparison_count,
            "delta_temperature_comparison_satisfied_count":
                state.delta_temperature_comparison_satisfied_count,
            "delta_temperature_body_entry_count": state.delta_temperature_body_entry_count,
            "delta_temperature_fallthrough_count": state.delta_temperature_fallthrough_count,
            "zone_cooling_setpoint_load_read_count":
                state.zone_cooling_setpoint_load_read_count,
            "cp_air_for_first_division_read_count":
                state.cp_air_for_first_division_read_count,
            "zone_cooling_setpoint_load_over_cp_air_calculation_count":
                state.zone_cooling_setpoint_load_over_cp_air_calculation_count,
            "delta_temperature_for_second_division_read_count":
                state.delta_temperature_for_second_division_read_count,
        }),
    );
    extend_object(
        &mut value,
        json!({
            "supply_mass_flow_rate_calculation_count":
                state.supply_mass_flow_rate_calculation_count,
            "initial_supply_mass_flow_rate_assignment_count":
                state.initial_supply_mass_flow_rate_assignment_count,
            "cooling_limit_flow_rate_read_count":
                state.cooling_limit_flow_rate_read_count,
            "cooling_limit_flow_rate_comparison_count":
                state.cooling_limit_flow_rate_comparison_count,
            "cooling_limit_flow_rate_match_count":
                state.cooling_limit_flow_rate_match_count,
            "cooling_limit_flow_rate_and_capacity_read_count":
                state.cooling_limit_flow_rate_and_capacity_read_count,
            "cooling_limit_flow_rate_and_capacity_comparison_count":
                state.cooling_limit_flow_rate_and_capacity_comparison_count,
            "cooling_limit_flow_rate_and_capacity_match_count":
                state.cooling_limit_flow_rate_and_capacity_match_count,
            "maximum_cooling_air_mass_flow_rate_read_count":
                state.maximum_cooling_air_mass_flow_rate_read_count,
            "maximum_cooling_air_mass_flow_rate_positive_comparison_count":
                state.maximum_cooling_air_mass_flow_rate_positive_comparison_count,
            "maximum_cooling_air_mass_flow_rate_positive_count":
                state.maximum_cooling_air_mass_flow_rate_positive_count,
            "maximum_flow_clamp_body_entry_count":
                state.maximum_flow_clamp_body_entry_count,
            "supply_mass_flow_rate_for_clamp_read_count":
                state.supply_mass_flow_rate_for_clamp_read_count,
            "inner_max_evaluation_count": state.inner_max_evaluation_count,
            "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count":
                state.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count,
            "outer_min_evaluation_count": state.outer_min_evaluation_count,
        }),
    );
    extend_object(
        &mut value,
        json!({
            "supply_mass_flow_rate_clamp_count": state.supply_mass_flow_rate_clamp_count,
            "clamped_supply_mass_flow_rate_assignment_count":
                state.clamped_supply_mass_flow_rate_assignment_count,
            "resulting_supply_mass_flow_rate_read_count":
                state.resulting_supply_mass_flow_rate_read_count,
            "outdoor_air_mass_flow_rate_read_count":
                state.outdoor_air_mass_flow_rate_read_count,
            "supply_above_outdoor_air_mass_flow_comparison_count":
                state.supply_above_outdoor_air_mass_flow_comparison_count,
            "supply_above_outdoor_air_mass_flow_comparison_satisfied_count":
                state.supply_above_outdoor_air_mass_flow_comparison_satisfied_count,
            "economizer_activation_body_entry_count":
                state.economizer_activation_body_entry_count,
            "outdoor_air_mass_flow_comparison_fallthrough_count":
                state.outdoor_air_mass_flow_comparison_fallthrough_count,
            "economizer_on_assignment_count": state.economizer_on_assignment_count,
            "supply_mass_flow_rate_for_outdoor_air_assignment_read_count":
                state.supply_mass_flow_rate_for_outdoor_air_assignment_read_count,
            "outdoor_air_mass_flow_rate_assignment_count":
                state.outdoor_air_mass_flow_rate_assignment_count,
            "system_time_step_read_count": state.system_time_step_read_count,
            "economizer_active_time_assignment_count":
                state.economizer_active_time_assignment_count,
            "latest": state.latest.map(snapshot_json),
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
