//! Coupled-runtime validation for CP388 sensible-output evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_predecessor_counts_match_exact_direct_cp_air_assignment,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(snapshot)
        && links_to_predecessor(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp387: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp387.state;
    let route_sum = checked_sum(
        &state.predecessor_route_counts,
        "predecessor_route_partition_overflow",
    )?;
    let assignments = state
        .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count;
    let expected_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp387.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor_cp387.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_predecessor_counts_match_exact_direct_cp_air_assignment(
            state,
            predecessor,
        )
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_owner_route_and_system_identity", 1, 0));
    }

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "predecessor_route_partition",
            state.transition_count,
            route_sum,
        ),
        (
            "inactive_transition_count",
            state.transition_count,
            state.inactive_transition_count,
        ),
        (
            "predecessor_cp_air_assignment_count",
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count,
            assignments,
        ),
        ("direct_sensible_output_assignment_count", 0, assignments),
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "cooling_total_output_owned_read_count",
            assignments,
            state.cooling_total_output_owned_read_count,
        ),
        (
            "cooling_total_output_bit_corroboration_count",
            assignments,
            state.cooling_total_output_bit_corroboration_count,
        ),
        (
            "cooling_sensible_heat_ratio_read_count",
            assignments,
            state.cooling_sensible_heat_ratio_read_count,
        ),
        (
            "cooling_sensible_output_calculation_count",
            assignments,
            state.cooling_sensible_output_calculation_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            assignments,
            state.cooling_sensible_output_assignment_write_count,
        ),
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
        latest_output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment,
    ) || !links_to_predecessor(latest, predecessor_latest)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn links_to_predecessor(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release(predecessor)
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_flags(snapshot) == predecessor_flags(predecessor)
        && snapshot.predecessor_supply_enthalpy_assignment_executed
            == predecessor.predecessor_supply_enthalpy_assignment_executed
        && snapshot.predecessor_dehumidification_control_type_read
            == predecessor.predecessor_dehumidification_control_type_read
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_dehumidification_control_switch_dispatched
            == predecessor.predecessor_dehumidification_control_switch_dispatched
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
            == predecessor.dehumidification_control_constant_sensible_heat_ratio_case_entered
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
            == predecessor.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && snapshot.predecessor_mixed_air_humidity_ratio_read
            == predecessor.mixed_air_humidity_ratio_read
        && option_bits_equal(
            snapshot.predecessor_mixed_air_humidity_ratio,
            predecessor.mixed_air_humidity_ratio,
        )
        && snapshot.predecessor_psychrometric_cp_air_evaluated
            == predecessor.psychrometric_cp_air_evaluated
        && option_bits_equal(
            snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
            predecessor.psychrometric_cp_air_result_j_per_kg_k,
        )
        && snapshot.predecessor_cp_air_assigned == predecessor.cp_air_assigned
        && option_bits_equal(
            snapshot.predecessor_cp_air_j_per_kg_k,
            predecessor.cp_air_j_per_kg_k,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && !snapshot.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
        && !snapshot.cp384_retained_cooling_total_output_owned_read
        && !snapshot.cp385_cooling_total_output_bit_corroborated
        && !snapshot.cooling_total_output_read
        && snapshot.cooling_total_output_w.is_none()
        && !snapshot.cooling_sensible_heat_ratio_read
        && snapshot.cooling_sensible_heat_ratio.is_none()
        && !snapshot.cooling_sensible_output_calculated
        && snapshot.calculated_cooling_sensible_output_w.is_none()
        && !snapshot.cooling_sensible_output_assigned
        && snapshot.cooling_sensible_output_w.is_none()
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

fn predecessor_flags(snapshot: PredecessorSnapshot) -> [bool; 20] {
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

fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    let exact_values = [
        option_bits_equal(
            left.predecessor_resulting_supply_enthalpy_j_per_kg,
            right.predecessor_resulting_supply_enthalpy_j_per_kg,
        ),
        option_bits_equal(
            left.predecessor_mixed_air_humidity_ratio,
            right.predecessor_mixed_air_humidity_ratio,
        ),
        option_bits_equal(
            left.predecessor_psychrometric_cp_air_result_j_per_kg_k,
            right.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        ),
        option_bits_equal(
            left.predecessor_cp_air_j_per_kg_k,
            right.predecessor_cp_air_j_per_kg_k,
        ),
        option_bits_equal(left.cooling_total_output_w, right.cooling_total_output_w),
        option_bits_equal(
            left.cooling_sensible_heat_ratio,
            right.cooling_sensible_heat_ratio,
        ),
        option_bits_equal(
            left.calculated_cooling_sensible_output_w,
            right.calculated_cooling_sensible_output_w,
        ),
        option_bits_equal(
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
        option_bits_equal(
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
    ];
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.predecessor_mixed_air_humidity_ratio = None;
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.predecessor_cp_air_j_per_kg_k = None;
        snapshot.cooling_total_output_w = None;
        snapshot.cooling_sensible_heat_ratio = None;
        snapshot.calculated_cooling_sensible_output_w = None;
        snapshot.cooling_sensible_output_w = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
    }
    exact_values.into_iter().all(|value| value) && left == right
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_partition_overflow_fails_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
    }

    #[test]
    fn bit_comparison_distinguishes_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
