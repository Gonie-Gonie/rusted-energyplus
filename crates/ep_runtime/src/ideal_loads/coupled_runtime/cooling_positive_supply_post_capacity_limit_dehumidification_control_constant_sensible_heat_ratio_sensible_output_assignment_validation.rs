//! Release validation for the bounded constant-SHR sensible-output assignment.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment;
    let snapshot = output
        .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
    let expected = expected_snapshot(predecessor);

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && snapshots_match_exact_bits(&snapshot, &expected)
}

pub(super) fn validate_lifecycle(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let assigned = state
        .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count;
    let transition_partition = checked_sum(
        &[
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
            assigned,
            state.dehumidification_control_humidistat_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ],
        "transition_partition_overflow",
    )?;
    let source_sites = assigned
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", usize::MAX, assigned))?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            predecessor.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completed_skip_count",
            predecessor.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
        ),
        (
            "constant_sensible_heat_ratio_sensible_output_assignment_count",
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count,
            assigned,
        ),
        (
            "humidistat_case_selected_skip_count",
            predecessor.dehumidification_control_humidistat_case_selected_skip_count,
            state.dehumidification_control_humidistat_case_selected_skip_count,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "direct_constant_sensible_heat_ratio_sensible_output_assignment_count",
            0,
            assigned,
        ),
        (
            "direct_humidistat_case_selected_skip_count",
            0,
            state.dehumidification_control_humidistat_case_selected_skip_count,
        ),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assigned,
            state.supply_mass_flow_rate_read_count,
        ),
        ("cp_air_read_count", assigned, state.cp_air_read_count),
        (
            "supply_mass_flow_rate_times_cp_air_calculation_count",
            assigned,
            state.supply_mass_flow_rate_times_cp_air_calculation_count,
        ),
        (
            "mixed_air_temperature_read_count",
            assigned,
            state.mixed_air_temperature_read_count,
        ),
        (
            "supply_temperature_read_count",
            assigned,
            state.supply_temperature_read_count,
        ),
        (
            "mixed_air_minus_supply_temperature_calculation_count",
            assigned,
            state.mixed_air_minus_supply_temperature_calculation_count,
        ),
        (
            "cooling_sensible_output_calculation_count",
            assigned,
            state.cooling_sensible_output_calculation_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            assigned,
            state.cooling_sensible_output_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    if binding.system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(violation("direct_binding_selector_is_none", 1, 0));
    }
    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let expected = expected_snapshot(*predecessor_latest);

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
            .len()
            != 8
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(latest, &expected)
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

pub(super) fn expected_snapshot(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot
{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type:
            predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_selected_skip:
            predecessor.dehumidification_control_humidistat_case_selected_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed:
            false,
        dehumidification_control_humidistat_case_selected_skip:
            predecessor.dehumidification_control_humidistat_case_selected_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        supply_mass_flow_rate_read: false,
        supply_mass_flow_rate_kg_per_s: None,
        cp_air_read: false,
        cp_air_j_per_kg_k: None,
        supply_mass_flow_rate_times_cp_air_calculated: false,
        supply_mass_flow_rate_times_cp_air_w_per_k: None,
        mixed_air_temperature_read: false,
        mixed_air_temperature_c: None,
        supply_temperature_read: false,
        supply_temperature_c: None,
        mixed_air_minus_supply_temperature_calculated: false,
        mixed_air_minus_supply_temperature_k: None,
        cooling_sensible_output_calculated: false,
        calculated_cooling_sensible_output_w: None,
        cooling_sensible_output_assigned: false,
        cooling_sensible_output_w: None,
    }
}

pub(super) fn snapshots_match_exact_bits(
    left:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
    right:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
) -> bool {
    let values_match = [
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k),
        (
            left.supply_mass_flow_rate_times_cp_air_w_per_k,
            right.supply_mass_flow_rate_times_cp_air_w_per_k,
        ),
        (left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        (left.supply_temperature_c, right.supply_temperature_c),
        (
            left.mixed_air_minus_supply_temperature_k,
            right.mixed_air_minus_supply_temperature_k,
        ),
        (
            left.calculated_cooling_sensible_output_w,
            right.calculated_cooling_sensible_output_w,
        ),
        (
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.cp_air_j_per_kg_k = None;
        snapshot.supply_mass_flow_rate_times_cp_air_w_per_k = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.supply_temperature_c = None;
        snapshot.mixed_air_minus_supply_temperature_k = None;
        snapshot.calculated_cooling_sensible_output_w = None;
        snapshot.cooling_sensible_output_w = None;
    }
    values_match && left_without_values == right_without_values
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, usize::MAX, *value))
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
    Error::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_partition_overflow_fails_closed() {
        let error = checked_sum(&[usize::MAX, 1], "test_partition")
            .expect_err("partition overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleInvariant {
                field: "test_partition",
                ..
            }
        ));
    }

    #[test]
    fn source_counter_corruption_is_rejected() {
        let error = ensure_count(1, 2, "cooling_sensible_output_assignment_write_count")
            .expect_err("self-inconsistent write history must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleInvariant {
                field: "cooling_sensible_output_assignment_write_count",
                expected: 2,
                actual: 1,
            }
        ));
    }
}
