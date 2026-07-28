//! Fail-closed validation helpers for CP340 evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
) -> Result<(), String> {
    let evaluations = state.capacity_limit_sensible_output_guard_evaluation_count;
    let expected_sites = evaluations
        .checked_mul(3)
        .and_then(|sites| {
            sites.checked_add(
                state.capacity_limit_sensible_output_adjustment_body_entry_count,
            )
        })
        .ok_or_else(|| {
            "direct-zone IdealLoads capacity-limit sensible-output guard source-site count overflowed"
                .to_string()
        })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "cooling_sensible_output_read_count",
            evaluations,
            state.cooling_sensible_output_read_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            evaluations,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_sensible_output_maximum_capacity_comparison_count",
            evaluations,
            state.cooling_sensible_output_maximum_capacity_comparison_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads capacity-limit sensible-output guard invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
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

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_sensible_output_guard_evaluation_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn exact_bits_distinguish_signed_zero_capacity() {
        assert!(option_has_bits(Some(-0.0), -0.0));
        assert!(!option_has_bits(Some(-0.0), 0.0));
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
