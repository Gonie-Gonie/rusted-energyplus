//! Fail-closed validation helpers for CP318 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C,
    PurchasedAirCalcCoolingSensibleFlowRuntimeState, PurchasedAirCalcCoolingSensibleFlowSnapshot,
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSensibleFlowRuntimeState,
) -> Result<(), String> {
    let cooling_on_partition = checked_add(
        state.cooling_on_body_entry_count,
        state.cooling_on_fallthrough_count,
        "cooling-on partition",
    )?;
    let delta_gate_partition = checked_add(
        state.delta_temperature_comparison_satisfied_count,
        state.delta_temperature_fallthrough_count,
        "delta-temperature gate partition",
    )?;
    for (field, expected, actual) in [
        (
            "supply_mass_flow_rate_for_cool_reset_assignment_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_for_cool_reset_assignment_count,
        ),
        (
            "cooling_on_read_count",
            state.cooling_body_entry_count,
            state.cooling_on_read_count,
        ),
        (
            "cooling_on_partition",
            state.cooling_on_read_count,
            cooling_on_partition,
        ),
        (
            "cooling_on_body_entry_count",
            state.cooling_body_entry_count,
            state.cooling_on_body_entry_count,
        ),
        (
            "direct_cooling_on_fallthrough_count",
            0,
            state.cooling_on_fallthrough_count,
        ),
        (
            "zone_humidity_ratio_read_count",
            state.cooling_on_body_entry_count,
            state.zone_humidity_ratio_read_count,
        ),
        (
            "psychrometric_cp_air_evaluation_count",
            state.cooling_on_body_entry_count,
            state.psychrometric_cp_air_evaluation_count,
        ),
        (
            "cp_air_assignment_count",
            state.cooling_on_body_entry_count,
            state.cp_air_assignment_count,
        ),
        (
            "minimum_cooling_supply_air_temperature_read_count",
            state.cooling_on_body_entry_count,
            state.minimum_cooling_supply_air_temperature_read_count,
        ),
        (
            "zone_temperature_read_count",
            state.cooling_on_body_entry_count,
            state.zone_temperature_read_count,
        ),
        (
            "delta_temperature_calculation_count",
            state.cooling_on_body_entry_count,
            state.delta_temperature_calculation_count,
        ),
        (
            "delta_temperature_assignment_count",
            state.cooling_on_body_entry_count,
            state.delta_temperature_assignment_count,
        ),
        (
            "delta_temperature_for_gate_read_count",
            state.cooling_on_body_entry_count,
            state.delta_temperature_for_gate_read_count,
        ),
        (
            "delta_temperature_comparison_count",
            state.cooling_on_body_entry_count,
            state.delta_temperature_comparison_count,
        ),
        (
            "delta_temperature_gate_partition",
            state.delta_temperature_comparison_count,
            delta_gate_partition,
        ),
        (
            "delta_temperature_body_entry_count",
            state.delta_temperature_comparison_satisfied_count,
            state.delta_temperature_body_entry_count,
        ),
        (
            "zone_cooling_setpoint_load_read_count",
            state.delta_temperature_body_entry_count,
            state.zone_cooling_setpoint_load_read_count,
        ),
        (
            "cp_air_for_first_division_read_count",
            state.delta_temperature_body_entry_count,
            state.cp_air_for_first_division_read_count,
        ),
        (
            "zone_cooling_setpoint_load_over_cp_air_calculation_count",
            state.delta_temperature_body_entry_count,
            state.zone_cooling_setpoint_load_over_cp_air_calculation_count,
        ),
        (
            "delta_temperature_for_second_division_read_count",
            state.delta_temperature_body_entry_count,
            state.delta_temperature_for_second_division_read_count,
        ),
        (
            "supply_mass_flow_rate_for_cool_calculation_count",
            state.delta_temperature_body_entry_count,
            state.supply_mass_flow_rate_for_cool_calculation_count,
        ),
        (
            "supply_mass_flow_rate_for_cool_assignment_count",
            state.delta_temperature_body_entry_count,
            state.supply_mass_flow_rate_for_cool_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling sensible-flow invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(snapshot: &PurchasedAirCalcCoolingSensibleFlowSnapshot) -> bool {
    if !snapshot.cooling_body_entered {
        return usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
            == 1
            && skipped_source_shape(snapshot);
    }
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || !snapshot.supply_mass_flow_rate_for_cool_reset_assigned
        || !same_option(
            snapshot.reset_supply_mass_flow_rate_for_cool_kg_per_s,
            Some(0.0),
        )
        || !snapshot.cooling_on_read
        || snapshot.cooling_on != Some(true)
        || !snapshot.cooling_on_body_entered
    {
        return false;
    }
    let Some(humidity_ratio) = snapshot.zone_humidity_ratio else {
        return false;
    };
    if !humidity_ratio.is_finite() {
        return false;
    }
    let cp_air = energyplus_psy_cp_air_fn_w(humidity_ratio);
    let Some(minimum_supply_temperature) = snapshot.minimum_cooling_supply_air_temperature_c else {
        return false;
    };
    let Some(zone_temperature) = snapshot.zone_temperature_c else {
        return false;
    };
    if !minimum_supply_temperature.is_finite() || !zone_temperature.is_finite() {
        return false;
    }
    let delta_temperature = minimum_supply_temperature - zone_temperature;
    let gate_satisfied =
        delta_temperature < -PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C;
    let prefix_matches = snapshot.zone_humidity_ratio_read
        && snapshot.psychrometric_cp_air_evaluated
        && same_option(
            snapshot.psychrometric_cp_air_result_j_per_kg_k,
            Some(cp_air),
        )
        && snapshot.cp_air_assigned
        && same_option(snapshot.cp_air_j_per_kg_k, Some(cp_air))
        && snapshot.minimum_cooling_supply_air_temperature_read
        && snapshot.zone_temperature_read
        && snapshot.delta_temperature_calculated
        && same_option(snapshot.delta_temperature_c, Some(delta_temperature))
        && snapshot.delta_temperature_assigned
        && same_option(
            snapshot.assigned_delta_temperature_c,
            Some(delta_temperature),
        )
        && snapshot.delta_temperature_for_gate_read
        && same_option(
            snapshot.delta_temperature_for_gate_c,
            Some(delta_temperature),
        )
        && snapshot.delta_temperature_comparison_evaluated
        && snapshot.delta_temperature_below_negative_small_temp_diff == Some(gate_satisfied)
        && snapshot.delta_temperature_body_entered == gate_satisfied;
    if !prefix_matches {
        return false;
    }
    if !gate_satisfied {
        return downstream_absent(snapshot)
            && same_option(
                snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
                Some(0.0),
            );
    }
    let Some(load) = snapshot.zone_cooling_setpoint_load_w else {
        return false;
    };
    if !load.is_finite() {
        return false;
    }
    let first_division = load / cp_air;
    let calculated_flow = first_division / delta_temperature;
    snapshot.zone_cooling_setpoint_load_read
        && snapshot.cp_air_for_first_division_read
        && same_option(snapshot.cp_air_for_first_division_j_per_kg_k, Some(cp_air))
        && snapshot.zone_cooling_setpoint_load_over_cp_air_calculated
        && same_option(
            snapshot.zone_cooling_setpoint_load_over_cp_air_kg_k_per_s,
            Some(first_division),
        )
        && snapshot.delta_temperature_for_second_division_read
        && same_option(
            snapshot.delta_temperature_for_second_division_c,
            Some(delta_temperature),
        )
        && snapshot.supply_mass_flow_rate_for_cool_calculated
        && same_option(
            snapshot.calculated_supply_mass_flow_rate_for_cool_kg_per_s,
            Some(calculated_flow),
        )
        && snapshot.supply_mass_flow_rate_for_cool_assigned
        && same_option(
            snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
            Some(calculated_flow),
        )
        && same_option(
            snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            Some(calculated_flow),
        )
}

fn skipped_source_shape(snapshot: &PurchasedAirCalcCoolingSensibleFlowSnapshot) -> bool {
    !snapshot.supply_mass_flow_rate_for_cool_reset_assigned
        && snapshot
            .reset_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
        && !snapshot.cooling_on_read
        && snapshot.cooling_on.is_none()
        && !snapshot.cooling_on_body_entered
        && !snapshot.zone_humidity_ratio_read
        && snapshot.zone_humidity_ratio.is_none()
        && !snapshot.psychrometric_cp_air_evaluated
        && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
        && !snapshot.cp_air_assigned
        && snapshot.cp_air_j_per_kg_k.is_none()
        && !snapshot.minimum_cooling_supply_air_temperature_read
        && snapshot.minimum_cooling_supply_air_temperature_c.is_none()
        && !snapshot.zone_temperature_read
        && snapshot.zone_temperature_c.is_none()
        && !snapshot.delta_temperature_calculated
        && snapshot.delta_temperature_c.is_none()
        && !snapshot.delta_temperature_assigned
        && snapshot.assigned_delta_temperature_c.is_none()
        && !snapshot.delta_temperature_for_gate_read
        && snapshot.delta_temperature_for_gate_c.is_none()
        && !snapshot.delta_temperature_comparison_evaluated
        && snapshot
            .delta_temperature_below_negative_small_temp_diff
            .is_none()
        && !snapshot.delta_temperature_body_entered
        && downstream_absent(snapshot)
        && snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
}

fn downstream_absent(snapshot: &PurchasedAirCalcCoolingSensibleFlowSnapshot) -> bool {
    !snapshot.zone_cooling_setpoint_load_read
        && snapshot.zone_cooling_setpoint_load_w.is_none()
        && !snapshot.cp_air_for_first_division_read
        && snapshot.cp_air_for_first_division_j_per_kg_k.is_none()
        && !snapshot.zone_cooling_setpoint_load_over_cp_air_calculated
        && snapshot
            .zone_cooling_setpoint_load_over_cp_air_kg_k_per_s
            .is_none()
        && !snapshot.delta_temperature_for_second_division_read
        && snapshot.delta_temperature_for_second_division_c.is_none()
        && !snapshot.supply_mass_flow_rate_for_cool_calculated
        && snapshot
            .calculated_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_cool_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
}

fn same_option(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads cooling sensible-flow {label} overflowed"))
}
