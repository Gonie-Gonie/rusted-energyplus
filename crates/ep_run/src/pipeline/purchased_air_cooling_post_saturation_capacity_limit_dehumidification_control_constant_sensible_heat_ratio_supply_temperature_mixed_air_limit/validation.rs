//! Fail-closed validation for CP390 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp389: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP390 evidence is missing".to_string())?;
    let predecessor = predecessor_cp389
        .ok_or_else(|| "direct-zone IdealLoads CP390 CP389 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP390 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP390 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
    {
        return Err("direct-zone IdealLoads CP390 provenance is invalid".to_string());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let assignments = state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count;
    let inactive = state
        .transition_count
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP390 inactive partition underflowed".to_string())?;
    let route_sum = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let owner_count = checked_sum(
        &state.predecessor_route_counts[3..],
        "CP389 temperature owner partition",
    )?;
    let unchanged = owner_count.checked_sub(assignments).ok_or_else(|| {
        "direct-zone IdealLoads CP390 unchanged owner partition underflowed".to_string()
    })?;
    let sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER.len())
        .ok_or_else(|| "direct-zone IdealLoads CP390 site count overflowed".to_string())?;
    validate_all_public_inactive_contract(state, predecessor_state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, route_sum),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp389_supply_temperature_state_owner_count",
            owner_count,
            state.cp389_supply_temperature_state_owner_count,
        ),
        (
            "predecessor_cp379_supply_temperature_state_owner_count",
            predecessor_state.cp379_supply_temperature_state_owner_count,
            state.cp389_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            unchanged,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    if state.predecessor_route_counts != predecessor_state.predecessor_route_counts {
        return Err("direct-zone IdealLoads CP390 route lineage is invalid".to_string());
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP390 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP390 controlled Zone is missing".to_string())?;
    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP390 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP390 CP389 latest evidence is missing".to_string()
    })?;
    if state.system != system
        || predecessor_state.system != system
        || latest.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || latest.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || latest.source_order != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !links_to_predecessor(latest, predecessor_latest)
        || !direct_skip_shape(latest)
        || !option_bits_equal(latest.resulting_supply_enthalpy_j_per_kg, predecessor_latest.resulting_supply_enthalpy_j_per_kg)
        || latest.cp389_retained_supply_temperature_state_owned
            != predecessor_latest.resulting_supply_temperature_c.is_some()
        || !option_bits_equal(latest.preexisting_supply_temperature_c, predecessor_latest.resulting_supply_temperature_c)
        || !option_bits_equal(latest.resulting_supply_temperature_c, predecessor_latest.resulting_supply_temperature_c)
    {
        return Err("direct-zone IdealLoads CP390 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn links_to_predecessor(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    predecessor.source == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_flags(snapshot) == inherited_predecessor_flags(predecessor)
        && cp389_flags(snapshot) == predecessor_flags(predecessor)
        && snapshot.predecessor_dehumidification_control_type == predecessor.predecessor_dehumidification_control_type
        && predecessor_values(snapshot)
            .into_iter()
            .zip(predecessor_snapshot_values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
}

fn direct_skip_shape(snapshot: Snapshot) -> bool {
    !snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed
        && !snapshot.cp389_retained_supply_temperature_owned_read
        && !snapshot.supply_temperature_for_minimum_read
        && snapshot.supply_temperature_before_mixed_air_limit_c.is_none()
        && !snapshot.cp329_retained_mixed_air_temperature_owned_read
        && !snapshot.cp389_mixed_air_temperature_bit_corroborated
        && !snapshot.mixed_air_temperature_for_minimum_read
        && snapshot.mixed_air_temperature_c.is_none()
        && !snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.minimum_supply_temperature_c.is_none()
        && !snapshot.supply_temperature_assignment_performed
        && snapshot.assigned_supply_temperature_c.is_none()
}

fn validate_all_public_inactive_contract(
    state: &ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState,
    predecessor: &ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState,
) -> Result<(), String> {
    for (field, actual) in [
        (
            "direct_assignment_count",
            state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count,
        ),
        (
            "predecessor_assignment_count",
            predecessor.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count,
        ),
        (
            "supply_temperature_owned_read_count",
            state.supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_for_minimum_read_count",
            state.supply_temperature_for_minimum_read_count,
        ),
        (
            "mixed_air_temperature_owned_read_count",
            state.mixed_air_temperature_owned_read_count,
        ),
        (
            "mixed_air_temperature_bit_corroboration_count",
            state.mixed_air_temperature_bit_corroboration_count,
        ),
        (
            "mixed_air_temperature_for_minimum_read_count",
            state.mixed_air_temperature_for_minimum_read_count,
        ),
        (
            "source_shaped_two_argument_minimum_evaluation_count",
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            state.supply_temperature_assignment_write_count,
        ),
    ] {
        ensure_count(actual, 0, field)?;
    }
    Ok(())
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn inherited_predecessor_flags(snapshot: PredecessorSnapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn cp389_flags(snapshot: Snapshot) -> [bool; 30] {
    [
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        snapshot.predecessor_mixed_air_humidity_ratio_read,
        snapshot.predecessor_psychrometric_cp_air_evaluated,
        snapshot.predecessor_cp_air_assigned,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        snapshot.predecessor_cp384_retained_cooling_total_output_owned_read,
        snapshot.predecessor_cp385_cooling_total_output_bit_corroborated,
        snapshot.predecessor_cooling_total_output_read,
        snapshot.predecessor_cooling_sensible_heat_ratio_read,
        snapshot.predecessor_cooling_sensible_output_calculated,
        snapshot.predecessor_cooling_sensible_output_assigned,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed,
        snapshot.predecessor_cp379_retained_supply_temperature_state_owned,
        snapshot.predecessor_cp329_retained_mixed_air_temperature_owned_read,
        snapshot.predecessor_mixed_air_temperature_read,
        snapshot.predecessor_cp388_retained_cooling_sensible_output_owned_read,
        snapshot.predecessor_cooling_sensible_output_read,
        snapshot.predecessor_cp387_retained_cp_air_owned_read,
        snapshot.predecessor_cp_air_read,
        snapshot.predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        snapshot.predecessor_supply_mass_flow_rate_read,
        snapshot.predecessor_cp_air_times_supply_mass_flow_rate_calculated,
        snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_calculated,
        snapshot.predecessor_supply_temperature_calculated,
        snapshot.predecessor_supply_temperature_assigned,
    ]
}

fn predecessor_flags(snapshot: PredecessorSnapshot) -> [bool; 30] {
    [
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        snapshot.predecessor_mixed_air_humidity_ratio_read,
        snapshot.predecessor_psychrometric_cp_air_evaluated,
        snapshot.predecessor_cp_air_assigned,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        snapshot.predecessor_cp384_retained_cooling_total_output_owned_read,
        snapshot.predecessor_cp385_cooling_total_output_bit_corroborated,
        snapshot.predecessor_cooling_total_output_read,
        snapshot.predecessor_cooling_sensible_heat_ratio_read,
        snapshot.predecessor_cooling_sensible_output_calculated,
        snapshot.predecessor_cooling_sensible_output_assigned,
        snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed,
        snapshot.cp379_retained_supply_temperature_state_owned,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.mixed_air_temperature_read,
        snapshot.cp388_retained_cooling_sensible_output_owned_read,
        snapshot.cooling_sensible_output_read,
        snapshot.cp387_retained_cp_air_owned_read,
        snapshot.cp_air_read,
        snapshot.cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.cp329_supply_mass_flow_rate_bit_corroborated,
        snapshot.supply_mass_flow_rate_read,
        snapshot.cp_air_times_supply_mass_flow_rate_calculated,
        snapshot.cooling_sensible_output_over_air_capacity_rate_calculated,
        snapshot.supply_temperature_calculated,
        snapshot.supply_temperature_assigned,
    ]
}

fn predecessor_values(snapshot: Snapshot) -> [Option<f64>; 18] {
    [
        snapshot.predecessor_mixed_air_humidity_ratio,
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        snapshot.predecessor_cp_air_j_per_kg_k,
        snapshot.predecessor_cooling_total_output_w,
        snapshot.predecessor_cooling_sensible_heat_ratio,
        snapshot.predecessor_calculated_cooling_sensible_output_w,
        snapshot.predecessor_cooling_sensible_output_w,
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_preexisting_supply_temperature_c,
        snapshot.predecessor_mixed_air_temperature_c,
        snapshot.predecessor_cp389_cooling_sensible_output_w,
        snapshot.predecessor_cp389_cp_air_j_per_kg_k,
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s,
        snapshot.predecessor_cp_air_times_supply_mass_flow_rate_w_per_k,
        snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_k,
        snapshot.predecessor_calculated_supply_temperature_c,
        snapshot.predecessor_assigned_supply_temperature_c,
        snapshot.predecessor_resulting_supply_temperature_c,
    ]
}

fn predecessor_snapshot_values(snapshot: PredecessorSnapshot) -> [Option<f64>; 18] {
    [
        snapshot.predecessor_mixed_air_humidity_ratio,
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        snapshot.predecessor_cp_air_j_per_kg_k,
        snapshot.predecessor_cooling_total_output_w,
        snapshot.predecessor_cooling_sensible_heat_ratio,
        snapshot.predecessor_calculated_cooling_sensible_output_w,
        snapshot.predecessor_cooling_sensible_output_w,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.preexisting_supply_temperature_c,
        snapshot.mixed_air_temperature_c,
        snapshot.cooling_sensible_output_w,
        snapshot.cp_air_j_per_kg_k,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.cp_air_times_supply_mass_flow_rate_w_per_k,
        snapshot.cooling_sensible_output_over_air_capacity_rate_k,
        snapshot.calculated_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP390 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP390 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
