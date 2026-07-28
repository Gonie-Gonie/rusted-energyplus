//! Release validation for the bounded capacity-limit sensible-output guard.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
};

use super::super::calc::cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment;
    let capacity = output.calculation_cooling_capacity_zero_flow_reset;
    let snapshot = output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && release_identity_matches(
            binding.ideal_loads_air_system,
            binding.zone,
            call_ordinal,
            predecessor.system,
            predecessor.controlled_zone,
            predecessor.parent_call_ordinal,
        )
        && release_identity_matches(
            binding.ideal_loads_air_system,
            binding.zone,
            call_ordinal,
            capacity.system,
            capacity.controlled_zone,
            capacity.parent_call_ordinal,
        )
        && cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
            snapshot,
        )
        && snapshot_shape(&snapshot, &predecessor, &capacity)
}

pub(super) fn validate_lifecycle(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    capacity_lifecycle: &PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let capacity = &capacity_lifecycle.state;

    let inherited_skips = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let inherited_skips = checked_add(
        inherited_skips,
        state.positive_guard_false_fallthrough_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let inherited_skips = checked_add(
        inherited_skips,
        state.capacity_limit_guard_false_fallthrough_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let comparison_partition = checked_add(
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        state.capacity_limit_sensible_output_adjustment_body_entry_count,
        "comparison_partition_overflow",
        state.capacity_limit_sensible_output_guard_evaluation_count,
    )?;
    let transition_partition = checked_add(
        inherited_skips,
        comparison_partition,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let three_active_sites = checked_mul(
        state.capacity_limit_sensible_output_guard_evaluation_count,
        3,
        "source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;
    let source_sites = checked_add(
        three_active_sites,
        state.capacity_limit_sensible_output_adjustment_body_entry_count,
        "source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "capacity_transition_count",
            capacity.transition_count,
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
            "capacity_limit_guard_false_fallthrough_skip_count",
            predecessor.capacity_limit_guard_false_fallthrough_skip_count,
            state.capacity_limit_guard_false_fallthrough_skip_count,
        ),
        (
            "capacity_limit_sensible_output_guard_evaluation_count",
            predecessor.capacity_limit_sensible_output_assignment_count,
            state.capacity_limit_sensible_output_guard_evaluation_count,
        ),
        (
            "comparison_partition",
            state.capacity_limit_sensible_output_guard_evaluation_count,
            comparison_partition,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "cooling_sensible_output_read_count",
            state.capacity_limit_sensible_output_guard_evaluation_count,
            state.cooling_sensible_output_read_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            state.capacity_limit_sensible_output_guard_evaluation_count,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_sensible_output_maximum_capacity_comparison_count",
            state.capacity_limit_sensible_output_guard_evaluation_count,
            state.cooling_sensible_output_maximum_capacity_comparison_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let capacity_latest = capacity
        .latest
        .as_ref()
        .ok_or_else(|| violation("capacity_latest_release_snapshot_ready", 1, 0))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER
            .len()
            != 4
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || capacity.system != binding.ideal_loads_air_system
        || !release_identity_matches(
            binding.ideal_loads_air_system,
            binding.zone,
            timestep_count,
            latest.system,
            latest.controlled_zone,
            latest.parent_call_ordinal,
        )
        || !release_identity_matches(
            binding.ideal_loads_air_system,
            binding.zone,
            timestep_count,
            predecessor_latest.system,
            predecessor_latest.controlled_zone,
            predecessor_latest.parent_call_ordinal,
        )
        || !release_identity_matches(
            binding.ideal_loads_air_system,
            binding.zone,
            timestep_count,
            capacity_latest.system,
            capacity_latest.controlled_zone,
            capacity_latest.parent_call_ordinal,
        )
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_capacity_limit_sensible_output_guard,
        )
        || !snapshot_shape(
            latest,
            predecessor_latest,
            capacity_latest,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    capacity: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    let evaluated = predecessor.capacity_limit_sensible_output_assignment_executed;
    if snapshot.unit_body_entered != predecessor.unit_body_entered
        || snapshot.predecessor_cooling_body_entered != predecessor.predecessor_cooling_body_entered
        || snapshot.predecessor_no_outdoor_air_fallback_entered
            != predecessor.predecessor_no_outdoor_air_fallback_entered
        || snapshot.predecessor_positive_supply_mass_flow_body_entered
            != predecessor.predecessor_positive_supply_mass_flow_body_entered
        || snapshot.predecessor_active_guard_false_fallthrough
            != predecessor.predecessor_active_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_guard_evaluated
            != predecessor.predecessor_capacity_limit_guard_evaluated
        || snapshot.predecessor_capacity_limit_body_entered
            != predecessor.predecessor_capacity_limit_body_entered
        || snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            != predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_cp_air_assignment_executed
            != predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        || snapshot.predecessor_capacity_limit_sensible_output_assignment_executed != evaluated
        || snapshot.unit_off_skipped != predecessor.unit_off_skipped
        || snapshot.non_cooling_skipped != predecessor.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_guard_false_fallthrough_skipped
            != predecessor.capacity_limit_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_sensible_output_guard_evaluated != evaluated
    {
        return false;
    }

    if !evaluated {
        return !snapshot.cooling_sensible_output_read
            && snapshot.cooling_sensible_output_w.is_none()
            && !snapshot.maximum_total_cooling_capacity_read
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && !snapshot.cooling_sensible_output_maximum_capacity_comparison_evaluated
            && snapshot
                .cooling_sensible_output_at_or_above_maximum_capacity
                .is_none()
            && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
            && !snapshot.capacity_limit_sensible_output_adjustment_body_entered;
    }

    let (Some(cooling_sensible_output_w), Some(maximum_total_cooling_capacity_w)) = (
        predecessor.cooling_sensible_output_w,
        capacity.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    let expected_comparison = cooling_sensible_output_w >= maximum_total_cooling_capacity_w;

    capacity.maximum_total_cooling_capacity_read
        && capacity.maximum_total_cooling_capacity_comparison_evaluated
        && active_capacity_lineage_is_reachable(
            maximum_total_cooling_capacity_w,
            capacity.maximum_total_cooling_capacity_equal_to_zero,
            capacity.zero_cooling_capacity_body_entered,
        )
        && snapshot.cooling_sensible_output_read
        && option_has_bits(
            snapshot.cooling_sensible_output_w,
            cooling_sensible_output_w,
        )
        && snapshot.maximum_total_cooling_capacity_read
        && option_has_bits(
            snapshot.maximum_total_cooling_capacity_w,
            maximum_total_cooling_capacity_w,
        )
        && snapshot.cooling_sensible_output_maximum_capacity_comparison_evaluated
        && snapshot.cooling_sensible_output_at_or_above_maximum_capacity
            == Some(expected_comparison)
        && snapshot.capacity_limit_sensible_output_guard_false_fallthrough != expected_comparison
        && snapshot.capacity_limit_sensible_output_adjustment_body_entered == expected_comparison
}

fn active_capacity_lineage_is_reachable(
    capacity_w: f64,
    equal_to_zero: Option<bool>,
    zero_body_entered: bool,
) -> bool {
    capacity_w.is_finite() && capacity_w > 0.0 && equal_to_zero == Some(false) && !zero_body_entered
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    right: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let values_match = option_bits_equal(
        left.cooling_sensible_output_w,
        right.cooling_sensible_output_w,
    ) && option_bits_equal(
        left.maximum_total_cooling_capacity_w,
        right.maximum_total_cooling_capacity_w,
    );
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.cooling_sensible_output_w = None;
    right_without_values.cooling_sensible_output_w = None;
    left_without_values.maximum_total_cooling_capacity_w = None;
    right_without_values.maximum_total_cooling_capacity_w = None;
    values_match && left_without_values == right_without_values
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[allow(clippy::too_many_arguments)]
fn release_identity_matches(
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    expected_ordinal: usize,
    system: ep_model::IdealLoadsAirSystemId,
    zone: ep_model::ZoneId,
    ordinal: usize,
) -> bool {
    system == expected_system && zone == expected_zone && ordinal == expected_ordinal
}

fn checked_add(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn checked_mul(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_mul(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_site_count_overflow_fails_closed() {
        let error = checked_mul(usize::MAX, 3, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleInvariant { .. }
        ));
    }

    #[test]
    fn exact_bits_distinguish_signed_zero_capacity() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }

    #[test]
    fn active_signed_zero_capacity_lineage_is_rejected() {
        assert!(!active_capacity_lineage_is_reachable(0.0, Some(true), true));
        assert!(!active_capacity_lineage_is_reachable(
            -0.0,
            Some(true),
            true
        ));
        assert!(active_capacity_lineage_is_reachable(
            1.0,
            Some(false),
            false
        ));
    }
}
