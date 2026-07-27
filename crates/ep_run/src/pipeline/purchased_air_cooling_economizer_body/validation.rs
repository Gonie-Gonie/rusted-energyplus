//! Fail-closed validation helpers for CP317 direct-release evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
};

pub(super) fn validate_zero_source_counters(
    state: &PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
) -> Result<(), String> {
    for (field, actual) in [
        (
            "zone_humidity_ratio_read_count",
            state.zone_humidity_ratio_read_count,
        ),
        (
            "psychrometric_cp_air_evaluation_count",
            state.psychrometric_cp_air_evaluation_count,
        ),
        ("cp_air_assignment_count", state.cp_air_assignment_count),
        (
            "outdoor_air_temperature_read_count",
            state.outdoor_air_temperature_read_count,
        ),
        (
            "zone_temperature_read_count",
            state.zone_temperature_read_count,
        ),
        (
            "delta_temperature_calculation_count",
            state.delta_temperature_calculation_count,
        ),
        (
            "delta_temperature_assignment_count",
            state.delta_temperature_assignment_count,
        ),
        (
            "delta_temperature_for_gate_read_count",
            state.delta_temperature_for_gate_read_count,
        ),
        (
            "delta_temperature_comparison_count",
            state.delta_temperature_comparison_count,
        ),
        (
            "delta_temperature_comparison_satisfied_count",
            state.delta_temperature_comparison_satisfied_count,
        ),
        (
            "delta_temperature_body_entry_count",
            state.delta_temperature_body_entry_count,
        ),
        (
            "delta_temperature_fallthrough_count",
            state.delta_temperature_fallthrough_count,
        ),
        (
            "zone_cooling_setpoint_load_read_count",
            state.zone_cooling_setpoint_load_read_count,
        ),
        (
            "cp_air_for_first_division_read_count",
            state.cp_air_for_first_division_read_count,
        ),
        (
            "zone_cooling_setpoint_load_over_cp_air_calculation_count",
            state.zone_cooling_setpoint_load_over_cp_air_calculation_count,
        ),
        (
            "delta_temperature_for_second_division_read_count",
            state.delta_temperature_for_second_division_read_count,
        ),
        (
            "supply_mass_flow_rate_calculation_count",
            state.supply_mass_flow_rate_calculation_count,
        ),
        (
            "initial_supply_mass_flow_rate_assignment_count",
            state.initial_supply_mass_flow_rate_assignment_count,
        ),
        (
            "cooling_limit_flow_rate_read_count",
            state.cooling_limit_flow_rate_read_count,
        ),
        (
            "cooling_limit_flow_rate_comparison_count",
            state.cooling_limit_flow_rate_comparison_count,
        ),
        (
            "cooling_limit_flow_rate_match_count",
            state.cooling_limit_flow_rate_match_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_read_count",
            state.cooling_limit_flow_rate_and_capacity_read_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_match_count",
            state.cooling_limit_flow_rate_and_capacity_match_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_read_count",
            state.maximum_cooling_air_mass_flow_rate_read_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_positive_comparison_count",
            state.maximum_cooling_air_mass_flow_rate_positive_comparison_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_positive_count",
            state.maximum_cooling_air_mass_flow_rate_positive_count,
        ),
        (
            "maximum_flow_clamp_body_entry_count",
            state.maximum_flow_clamp_body_entry_count,
        ),
        (
            "supply_mass_flow_rate_for_clamp_read_count",
            state.supply_mass_flow_rate_for_clamp_read_count,
        ),
        (
            "inner_max_evaluation_count",
            state.inner_max_evaluation_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count",
            state.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count,
        ),
        (
            "outer_min_evaluation_count",
            state.outer_min_evaluation_count,
        ),
        (
            "supply_mass_flow_rate_clamp_count",
            state.supply_mass_flow_rate_clamp_count,
        ),
        (
            "clamped_supply_mass_flow_rate_assignment_count",
            state.clamped_supply_mass_flow_rate_assignment_count,
        ),
        (
            "resulting_supply_mass_flow_rate_read_count",
            state.resulting_supply_mass_flow_rate_read_count,
        ),
        (
            "outdoor_air_mass_flow_rate_read_count",
            state.outdoor_air_mass_flow_rate_read_count,
        ),
        (
            "supply_above_outdoor_air_mass_flow_comparison_count",
            state.supply_above_outdoor_air_mass_flow_comparison_count,
        ),
        (
            "supply_above_outdoor_air_mass_flow_comparison_satisfied_count",
            state.supply_above_outdoor_air_mass_flow_comparison_satisfied_count,
        ),
        (
            "economizer_activation_body_entry_count",
            state.economizer_activation_body_entry_count,
        ),
        (
            "outdoor_air_mass_flow_comparison_fallthrough_count",
            state.outdoor_air_mass_flow_comparison_fallthrough_count,
        ),
        (
            "economizer_on_assignment_count",
            state.economizer_on_assignment_count,
        ),
        (
            "supply_mass_flow_rate_for_outdoor_air_assignment_read_count",
            state.supply_mass_flow_rate_for_outdoor_air_assignment_read_count,
        ),
        (
            "outdoor_air_mass_flow_rate_assignment_count",
            state.outdoor_air_mass_flow_rate_assignment_count,
        ),
        (
            "system_time_step_read_count",
            state.system_time_step_read_count,
        ),
        (
            "economizer_active_time_assignment_count",
            state.economizer_active_time_assignment_count,
        ),
    ] {
        if actual != 0 {
            return Err(format!(
                "direct-zone IdealLoads cooling economizer body invariant {field} expected 0, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn skipped_shape(body: &PurchasedAirCalcCoolingEconomizerBodySnapshot) -> bool {
    !body.zone_humidity_ratio_read
        && body.zone_humidity_ratio.is_none()
        && !body.psychrometric_cp_air_evaluated
        && body.psychrometric_cp_air_result_j_per_kg_k.is_none()
        && !body.cp_air_assigned
        && body.cp_air_j_per_kg_k.is_none()
        && !body.outdoor_air_temperature_read
        && body.outdoor_air_temperature_c.is_none()
        && !body.zone_temperature_read
        && body.zone_temperature_c.is_none()
        && !body.delta_temperature_calculated
        && body.delta_temperature_c.is_none()
        && !body.delta_temperature_assigned
        && body.assigned_delta_temperature_c.is_none()
        && !body.delta_temperature_for_gate_read
        && body.delta_temperature_for_gate_c.is_none()
        && !body.delta_temperature_comparison_evaluated
        && body
            .delta_temperature_below_negative_small_temp_diff
            .is_none()
        && !body.delta_temperature_body_entered
        && !body.zone_cooling_setpoint_load_read
        && body.zone_cooling_setpoint_load_w.is_none()
        && !body.cp_air_for_first_division_read
        && body.cp_air_for_first_division_j_per_kg_k.is_none()
        && !body.zone_cooling_setpoint_load_over_cp_air_calculated
        && body
            .zone_cooling_setpoint_load_over_cp_air_kg_k_per_s
            .is_none()
        && !body.delta_temperature_for_second_division_read
        && body.delta_temperature_for_second_division_c.is_none()
        && !body.supply_mass_flow_rate_calculated
        && body.calculated_supply_mass_flow_rate_kg_per_s.is_none()
        && !body.initial_supply_mass_flow_rate_assigned
        && body.initial_supply_mass_flow_rate_kg_per_s.is_none()
        && !body.cooling_limit_flow_rate_comparison_evaluated
        && !body.cooling_limit_flow_rate_read
        && body.cooling_limit_flow_rate_value.is_none()
        && body.cooling_limit_flow_rate_comparison_satisfied.is_none()
        && !body.cooling_limit_flow_rate_and_capacity_comparison_evaluated
        && !body.cooling_limit_flow_rate_and_capacity_read
        && body.cooling_limit_flow_rate_and_capacity_value.is_none()
        && body
            .cooling_limit_flow_rate_and_capacity_comparison_satisfied
            .is_none()
        && body.cooling_flow_limit_active.is_none()
        && !body.maximum_cooling_air_mass_flow_rate_read
        && body.maximum_cooling_air_mass_flow_rate_kg_per_s.is_none()
        && !body.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated
        && body.maximum_cooling_air_mass_flow_rate_positive.is_none()
        && !body.maximum_flow_clamp_body_entered
        && !body.supply_mass_flow_rate_clamped
        && !body.supply_mass_flow_rate_for_clamp_read
        && body.supply_mass_flow_rate_for_clamp_kg_per_s.is_none()
        && !body.inner_max_evaluated
        && body.nonnegative_supply_mass_flow_rate_kg_per_s.is_none()
        && !body.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read
        && body
            .maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s
            .is_none()
        && !body.outer_min_evaluated
        && body.clamped_supply_mass_flow_rate_kg_per_s.is_none()
        && !body.clamped_supply_mass_flow_rate_assigned
        && body.resulting_supply_mass_flow_rate_kg_per_s.is_none()
        && !body.resulting_supply_mass_flow_rate_read
        && !body.outdoor_air_mass_flow_rate_read
        && body.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !body.supply_above_outdoor_air_mass_flow_comparison_evaluated
        && body.supply_mass_flow_above_outdoor_air_mass_flow.is_none()
        && !body.economizer_activation_body_entered
        && !body.economizer_on_assigned
        && body.economizer_on.is_none()
        && !body.supply_mass_flow_rate_for_outdoor_air_assignment_read
        && body
            .supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s
            .is_none()
        && !body.outdoor_air_mass_flow_rate_assigned
        && body.assigned_outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !body.system_time_step_read
        && body.system_time_step_hours.is_none()
        && !body.economizer_active_time_assigned
        && body.assigned_economizer_active_time_hours.is_none()
}
