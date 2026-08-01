//! Coupled-runtime validation for CP390 supply-temperature mixed-air-limit evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
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
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment;
    let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(snapshot)
        && links_to_predecessor(snapshot, predecessor)
        && snapshot.cp389_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && option_bits_equal(
            snapshot.preexisting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && option_bits_equal(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp389: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp389.state;
    let assignments = state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count;
    let inactive = state
        .transition_count
        .checked_sub(assignments)
        .ok_or_else(|| {
            violation(
                "inactive_transition_underflow",
                assignments,
                state.transition_count,
            )
        })?;
    let route_sum = checked_sum(
        &state.predecessor_route_counts,
        "predecessor_route_partition_overflow",
    )?;
    let expected_owners = checked_sum(
        &state.predecessor_route_counts[3..],
        "cp389_supply_temperature_state_owner_count_overflow",
    )?;
    let expected_unchanged = expected_owners.checked_sub(assignments).ok_or_else(|| {
        violation(
            "unchanged_supply_temperature_preservation_underflow",
            assignments,
            expected_owners,
        )
    })?;
    let expected_sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;

    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp389.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || predecessor_cp389.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_owner_route_and_system_identity", 1, 0));
    }

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("predecessor_transition_count", predecessor.transition_count, state.transition_count),
        ("predecessor_route_partition", state.transition_count, route_sum),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_supply_temperature_assignment_count", predecessor.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count, assignments),
        ("direct_supply_temperature_mixed_air_limit_count", 0, assignments),
        ("source_site_execution_count", expected_sites, state.source_site_execution_count),
        ("cp389_supply_temperature_state_owner_count", expected_owners, state.cp389_supply_temperature_state_owner_count),
        ("predecessor_cp379_supply_temperature_state_owner_count", predecessor.cp379_supply_temperature_state_owner_count, state.cp389_supply_temperature_state_owner_count),
        ("unchanged_supply_temperature_preservation_count", expected_unchanged, state.unchanged_supply_temperature_preservation_count),
        ("supply_temperature_owned_read_count", assignments, state.supply_temperature_owned_read_count),
        ("supply_temperature_for_minimum_read_count", assignments, state.supply_temperature_for_minimum_read_count),
        ("mixed_air_temperature_owned_read_count", assignments, state.mixed_air_temperature_owned_read_count),
        ("mixed_air_temperature_bit_corroboration_count", assignments, state.mixed_air_temperature_bit_corroboration_count),
        ("mixed_air_temperature_for_minimum_read_count", assignments, state.mixed_air_temperature_for_minimum_read_count),
        ("source_shaped_two_argument_minimum_evaluation_count", assignments, state.source_shaped_two_argument_minimum_evaluation_count),
        ("supply_temperature_assignment_write_count", assignments, state.supply_temperature_assignment_write_count),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    if !same_snapshot(
        latest,
        latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit,
    ) || !links_to_predecessor(latest, predecessor_latest)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
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

fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    let values_match = snapshot_values(left)
        .into_iter()
        .zip(snapshot_values(right))
        .all(|(left, right)| option_bits_equal(left, right));
    clear_values(&mut left);
    clear_values(&mut right);
    values_match && left == right
}

fn snapshot_values(snapshot: Snapshot) -> [Option<f64>; 25] {
    let predecessor = predecessor_values(snapshot);
    [
        predecessor[0],
        predecessor[1],
        predecessor[2],
        predecessor[3],
        predecessor[4],
        predecessor[5],
        predecessor[6],
        predecessor[7],
        predecessor[8],
        predecessor[9],
        predecessor[10],
        predecessor[11],
        predecessor[12],
        predecessor[13],
        predecessor[14],
        predecessor[15],
        predecessor[16],
        predecessor[17],
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.preexisting_supply_temperature_c,
        snapshot.supply_temperature_before_mixed_air_limit_c,
        snapshot.mixed_air_temperature_c,
        snapshot.minimum_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn clear_values(snapshot: &mut Snapshot) {
    snapshot.predecessor_mixed_air_humidity_ratio = None;
    snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k = None;
    snapshot.predecessor_cp_air_j_per_kg_k = None;
    snapshot.predecessor_cooling_total_output_w = None;
    snapshot.predecessor_cooling_sensible_heat_ratio = None;
    snapshot.predecessor_calculated_cooling_sensible_output_w = None;
    snapshot.predecessor_cooling_sensible_output_w = None;
    snapshot.predecessor_resulting_supply_enthalpy_j_per_kg = None;
    snapshot.predecessor_preexisting_supply_temperature_c = None;
    snapshot.predecessor_mixed_air_temperature_c = None;
    snapshot.predecessor_cp389_cooling_sensible_output_w = None;
    snapshot.predecessor_cp389_cp_air_j_per_kg_k = None;
    snapshot.predecessor_supply_mass_flow_rate_kg_per_s = None;
    snapshot.predecessor_cp_air_times_supply_mass_flow_rate_w_per_k = None;
    snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_k = None;
    snapshot.predecessor_calculated_supply_temperature_c = None;
    snapshot.predecessor_assigned_supply_temperature_c = None;
    snapshot.predecessor_resulting_supply_temperature_c = None;
    snapshot.resulting_supply_enthalpy_j_per_kg = None;
    snapshot.preexisting_supply_temperature_c = None;
    snapshot.supply_temperature_before_mixed_air_limit_c = None;
    snapshot.mixed_air_temperature_c = None;
    snapshot.minimum_supply_temperature_c = None;
    snapshot.assigned_supply_temperature_c = None;
    snapshot.resulting_supply_temperature_c = None;
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
