//! Release validation for the bounded sensible-output maximum-capacity assignment.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
};

use super::super::calc::cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;
    let snapshot = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;

    release_identity_matches(
        binding.ideal_loads_air_system,
        binding.zone,
        call_ordinal,
        snapshot.system,
        snapshot.controlled_zone,
        snapshot.parent_call_ordinal,
    ) && release_identity_matches(
        binding.ideal_loads_air_system,
        binding.zone,
        call_ordinal,
        predecessor.system,
        predecessor.controlled_zone,
        predecessor.parent_call_ordinal,
    ) && cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
        snapshot,
    ) && snapshot_shape(&snapshot, &predecessor)
}

pub(super) fn validate_lifecycle(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;

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
    let active_partition = checked_add(
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        state.capacity_limit_sensible_output_maximum_capacity_assignment_count,
        "active_partition_overflow",
        predecessor.capacity_limit_sensible_output_guard_evaluation_count,
    )?;
    let transition_partition = checked_add(
        inherited_skips,
        active_partition,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let source_sites = checked_mul(
        state.capacity_limit_sensible_output_maximum_capacity_assignment_count,
        2,
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
            "capacity_limit_sensible_output_guard_false_fallthrough_count",
            predecessor.capacity_limit_sensible_output_guard_false_fallthrough_count,
            state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        ),
        (
            "capacity_limit_sensible_output_maximum_capacity_assignment_count",
            predecessor.capacity_limit_sensible_output_adjustment_body_entry_count,
            state.capacity_limit_sensible_output_maximum_capacity_assignment_count,
        ),
        (
            "active_partition",
            predecessor.capacity_limit_sensible_output_guard_evaluation_count,
            active_partition,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            state.capacity_limit_sensible_output_maximum_capacity_assignment_count,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            state.capacity_limit_sensible_output_maximum_capacity_assignment_count,
            state.cooling_sensible_output_assignment_write_count,
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

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER
            .len()
            != 2
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
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
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment,
        )
        || !snapshot_shape(latest, predecessor_latest)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let evaluated = predecessor.capacity_limit_sensible_output_guard_evaluated;
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment = predecessor.capacity_limit_sensible_output_adjustment_body_entered;
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
        || snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
            != predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        || snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated != evaluated
        || snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            != guard_false
        || snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered != assignment
        || snapshot.unit_off_skipped != predecessor.unit_off_skipped
        || snapshot.non_cooling_skipped != predecessor.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_guard_false_fallthrough_skipped
            != predecessor.capacity_limit_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_sensible_output_guard_false_fallthrough != guard_false
        || snapshot.capacity_limit_sensible_output_maximum_capacity_assignment_executed
            != assignment
        || evaluated != (guard_false || assignment)
        || (evaluated && guard_false == assignment)
    {
        return false;
    }

    if !evaluated {
        return !predecessor.cooling_sensible_output_read
            && predecessor.cooling_sensible_output_w.is_none()
            && !predecessor.maximum_total_cooling_capacity_read
            && predecessor.maximum_total_cooling_capacity_w.is_none()
            && !predecessor.cooling_sensible_output_maximum_capacity_comparison_evaluated
            && predecessor
                .cooling_sensible_output_at_or_above_maximum_capacity
                .is_none()
            && snapshot.preexisting_cooling_sensible_output_w.is_none()
            && !snapshot.maximum_total_cooling_capacity_read
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && !snapshot.cooling_sensible_output_assigned
            && snapshot.assigned_cooling_sensible_output_w.is_none()
            && snapshot.resulting_cooling_sensible_output_w.is_none();
    }

    let (Some(preexisting), Some(retained_capacity)) = (
        predecessor.cooling_sensible_output_w,
        predecessor.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    let comparison = preexisting >= retained_capacity;
    if !predecessor.cooling_sensible_output_read
        || !predecessor.maximum_total_cooling_capacity_read
        || !predecessor.cooling_sensible_output_maximum_capacity_comparison_evaluated
        || predecessor.cooling_sensible_output_at_or_above_maximum_capacity != Some(comparison)
        || guard_false == comparison
        || assignment != comparison
        || !active_capacity_is_reachable(retained_capacity)
        || !option_has_bits(snapshot.preexisting_cooling_sensible_output_w, preexisting)
    {
        return false;
    }

    if guard_false {
        !snapshot.maximum_total_cooling_capacity_read
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && !snapshot.cooling_sensible_output_assigned
            && snapshot.assigned_cooling_sensible_output_w.is_none()
            && option_has_bits(snapshot.resulting_cooling_sensible_output_w, preexisting)
    } else {
        snapshot.maximum_total_cooling_capacity_read
            && option_has_bits(snapshot.maximum_total_cooling_capacity_w, retained_capacity)
            && snapshot.cooling_sensible_output_assigned
            && option_has_bits(
                snapshot.assigned_cooling_sensible_output_w,
                retained_capacity,
            )
            && option_has_bits(
                snapshot.resulting_cooling_sensible_output_w,
                retained_capacity,
            )
    }
}

fn active_capacity_is_reachable(capacity_w: f64) -> bool {
    capacity_w.is_finite() && capacity_w > 0.0
}

fn snapshots_match_exact_bits(
    left:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    right:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    let values_match = [
        (
            left.preexisting_cooling_sensible_output_w,
            right.preexisting_cooling_sensible_output_w,
        ),
        (
            left.maximum_total_cooling_capacity_w,
            right.maximum_total_cooling_capacity_w,
        ),
        (
            left.resulting_cooling_sensible_output_w,
            right.resulting_cooling_sensible_output_w,
        ),
        (
            left.assigned_cooling_sensible_output_w,
            right.assigned_cooling_sensible_output_w,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_equal(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.preexisting_cooling_sensible_output_w = None;
    right_without_values.preexisting_cooling_sensible_output_w = None;
    left_without_values.maximum_total_cooling_capacity_w = None;
    right_without_values.maximum_total_cooling_capacity_w = None;
    left_without_values.resulting_cooling_sensible_output_w = None;
    right_without_values.resulting_cooling_sensible_output_w = None;
    left_without_values.assigned_cooling_sensible_output_w = None;
    right_without_values.assigned_cooling_sensible_output_w = None;
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
    Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleInvariant {
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
        let error = checked_mul(usize::MAX, 2, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleInvariant {
                ..
            }
        ));
    }

    #[test]
    fn exact_bits_preserve_nan_and_distinguish_signed_zero() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0042);
        assert!(option_has_bits(Some(nan), nan));
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }

    #[test]
    fn forged_active_capacity_domain_is_rejected() {
        for capacity in [0.0, -0.0, -1.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            assert!(!active_capacity_is_reachable(capacity));
        }
        assert!(active_capacity_is_reachable(f64::MIN_POSITIVE));
    }
}
