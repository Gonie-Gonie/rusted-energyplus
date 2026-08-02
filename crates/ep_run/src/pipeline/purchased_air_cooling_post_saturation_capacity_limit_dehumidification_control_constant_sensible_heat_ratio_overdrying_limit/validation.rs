//! Fail-closed validation for CP391 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot as PredecessorSnapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp390: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP391 evidence is missing".to_string())?;
    let predecessor = predecessor_cp390
        .ok_or_else(|| "direct-zone IdealLoads CP391 CP390 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP391 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP391 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
    {
        return Err("direct-zone IdealLoads CP391 provenance is invalid".to_string());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let assignments =
        state.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count;
    let inactive = state
        .transition_count
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP391 inactive partition underflowed".to_string())?;
    let route_sum = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let active = checked_sum(
        &[
            state.predecessor_route_counts[18],
            state.predecessor_route_counts[22],
            state.predecessor_route_counts[28],
        ],
        "active route partition",
    )?;
    let owner_count = checked_sum(
        &[
            state.predecessor_route_counts[5],
            state.predecessor_route_counts[8],
            state.predecessor_route_counts[11],
            state.predecessor_route_counts[14],
            state.predecessor_route_counts[17],
            checked_sum(
                &state.predecessor_route_counts[18..],
                "CP390 enthalpy owner tail",
            )?,
        ],
        "CP390 enthalpy owner partition",
    )?;
    let unchanged = owner_count.checked_sub(assignments).ok_or_else(|| {
        "direct-zone IdealLoads CP391 unchanged owner partition underflowed".to_string()
    })?;
    let sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER.len())
        .ok_or_else(|| "direct-zone IdealLoads CP391 site count overflowed".to_string())?;
    validate_all_public_inactive_contract(state, predecessor_state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, route_sum),
        ("active_route_partition", active, assignments),
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
            "cp390_supply_enthalpy_state_owner_count",
            owner_count,
            state.cp390_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            unchanged,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    if state.predecessor_route_counts != predecessor_state.predecessor_route_counts {
        return Err("direct-zone IdealLoads CP391 route lineage is invalid".to_string());
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP391 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP391 controlled Zone is missing".to_string())?;
    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP391 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP391 CP390 latest evidence is missing".to_string()
    })?;
    if state.system != system
        || predecessor_state.system != system
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !links_to_predecessor(latest, predecessor_latest)
        || !direct_skip_shape(latest)
        || latest.cp390_retained_supply_enthalpy_state_owned
            != predecessor_latest.resulting_supply_enthalpy_j_per_kg.is_some()
        || !option_bits_equal(
            latest.preexisting_supply_enthalpy_j_per_kg,
            predecessor_latest.resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_equal(
            latest.resulting_supply_enthalpy_j_per_kg,
            predecessor_latest.resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_equal(
            latest.resulting_supply_temperature_c,
            predecessor_latest.resulting_supply_temperature_c,
        )
    {
        return Err("direct-zone IdealLoads CP391 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn direct_skip_shape(snapshot: Snapshot) -> bool {
    !snapshot.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
        && !snapshot.cp390_retained_supply_enthalpy_owned_read
        && !snapshot.supply_enthalpy_for_overdrying_limit_maximum_read
        && snapshot
            .supply_enthalpy_before_overdrying_limit_j_per_kg
            .is_none()
        && !snapshot.cp390_retained_supply_temperature_owned_read
        && !snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read
        && snapshot.supply_temperature_c.is_none()
        && !snapshot.psychrometric_minimum_supply_enthalpy_evaluated
        && snapshot
            .psychrometric_minimum_supply_enthalpy_j_per_kg
            .is_none()
        && !snapshot.source_shaped_two_argument_maximum_evaluated
        && snapshot.maximum_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_assignment_performed
        && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
}

fn validate_all_public_inactive_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    for (field, actual) in [
        (
            "direct_assignment_count",
            state.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count,
        ),
        (
            "predecessor_assignment_count",
            predecessor.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count,
        ),
        (
            "supply_enthalpy_owned_read_count",
            state.supply_enthalpy_owned_read_count,
        ),
        (
            "supply_enthalpy_for_overdrying_limit_maximum_read_count",
            state.supply_enthalpy_for_overdrying_limit_maximum_read_count,
        ),
        (
            "supply_temperature_owned_read_count",
            state.supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count",
            state.supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count,
        ),
        (
            "psychrometric_minimum_supply_enthalpy_evaluation_count",
            state.psychrometric_minimum_supply_enthalpy_evaluation_count,
        ),
        (
            "source_shaped_two_argument_maximum_evaluation_count",
            state.source_shaped_two_argument_maximum_evaluation_count,
        ),
        (
            "supply_enthalpy_assignment_write_count",
            state.supply_enthalpy_assignment_write_count,
        ),
    ] {
        ensure_count(actual, 0, field)?;
    }
    Ok(())
}

fn links_to_predecessor(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    predecessor.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_flags(snapshot) == inherited_predecessor_flags(predecessor)
        && cp389_flags(snapshot) == predecessor_cp389_flags(predecessor)
        && cp390_flags(snapshot) == predecessor_cp390_flags(predecessor)
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && predecessor_values(snapshot)
            .into_iter()
            .zip(predecessor_snapshot_values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
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

fn predecessor_cp389_flags(snapshot: PredecessorSnapshot) -> [bool; 30] {
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

fn cp390_flags(snapshot: Snapshot) -> [bool; 9] {
    [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed,
        snapshot.cp389_retained_supply_temperature_state_owned,
        snapshot.cp389_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_read,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.cp389_mixed_air_temperature_bit_corroborated,
        snapshot.mixed_air_temperature_for_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_temperature_assignment_performed,
    ]
}

fn predecessor_cp390_flags(snapshot: PredecessorSnapshot) -> [bool; 9] {
    [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed,
        snapshot.cp389_retained_supply_temperature_state_owned,
        snapshot.cp389_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_read,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.cp389_mixed_air_temperature_bit_corroborated,
        snapshot.mixed_air_temperature_for_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_temperature_assignment_performed,
    ]
}

fn predecessor_values(snapshot: Snapshot) -> [Option<f64>; 25] {
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
        snapshot.predecessor_cp390_resulting_supply_enthalpy_j_per_kg,
        snapshot.preexisting_supply_temperature_c,
        snapshot.supply_temperature_before_mixed_air_limit_c,
        snapshot.mixed_air_temperature_c,
        snapshot.minimum_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.predecessor_cp390_resulting_supply_temperature_c,
    ]
}

fn predecessor_snapshot_values(snapshot: PredecessorSnapshot) -> [Option<f64>; 25] {
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
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.preexisting_supply_temperature_c,
        snapshot.supply_temperature_before_mixed_air_limit_c,
        snapshot.mixed_air_temperature_c,
        snapshot.minimum_supply_temperature_c,
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
            .ok_or_else(|| format!("direct-zone IdealLoads CP391 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP391 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
