//! Fail-closed validation helpers for CP337 direct-release evidence.

use ep_model::IdealLoadsLimit;
use ep_runtime::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    cooling_limit: IdealLoadsLimit,
) -> Result<(), String> {
    let active = state.capacity_limit_guard_evaluation_count;
    let capacity_matches = if cooling_limit == IdealLoadsLimit::LimitCapacity {
        active
    } else {
        0
    };
    let second_comparisons = checked_sub(active, capacity_matches, "second-comparison partition")?;
    let combined_matches = if cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity {
        active
    } else {
        0
    };
    let second_false = checked_sub(
        second_comparisons,
        combined_matches,
        "second-result partition",
    )?;
    let body_entries = checked_add(capacity_matches, combined_matches, "body-entry partition")?;
    let active_false = checked_sub(active, body_entries, "active-false partition")?;
    let first_sites = checked_mul(active, 2, "first-selector source-site count")?;
    let second_sites = checked_mul(second_comparisons, 2, "second-selector source-site count")?;
    let source_sites = checked_add(first_sites, second_sites, "source-site count")?;
    let source_sites = checked_add(source_sites, body_entries, "source-site count")?;

    for (field, expected, actual) in [
        (
            "first_cooling_limit_read_count",
            active,
            state.first_cooling_limit_read_count,
        ),
        (
            "cooling_limit_capacity_comparison_count",
            active,
            state.cooling_limit_capacity_comparison_count,
        ),
        (
            "cooling_limit_capacity_match_count",
            capacity_matches,
            state.cooling_limit_capacity_match_count,
        ),
        (
            "second_cooling_limit_read_count",
            second_comparisons,
            state.second_cooling_limit_read_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            second_comparisons,
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_match_count",
            combined_matches,
            state.cooling_limit_flow_rate_and_capacity_match_count,
        ),
        (
            "capacity_limit_body_entry_count",
            body_entries,
            state.capacity_limit_body_entry_count,
        ),
        (
            "cooling_limit_rejected_count",
            second_false,
            state.cooling_limit_rejected_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            active_false,
            state.active_guard_false_fallthrough_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let active = predecessor.supply_enthalpy_assignment_executed;
    if snapshot.capacity_limit_guard_evaluated != active {
        return false;
    }
    if !active {
        return skipped_source_shape(snapshot);
    }

    let capacity_match = cooling_limit == IdealLoadsLimit::LimitCapacity;
    let second_comparison = !capacity_match;
    let combined_match =
        second_comparison && cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let condition_satisfied = capacity_match || combined_match;
    snapshot.first_cooling_limit_read
        && snapshot.first_cooling_limit == Some(cooling_limit)
        && snapshot.cooling_limit_capacity_comparison_evaluated
        && snapshot.cooling_limit_capacity == Some(capacity_match)
        && snapshot.second_cooling_limit_read == second_comparison
        && snapshot.second_cooling_limit == second_comparison.then_some(cooling_limit)
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated == second_comparison
        && snapshot.cooling_limit_flow_rate_and_capacity
            == second_comparison.then_some(combined_match)
        && snapshot.cooling_limit_condition_satisfied == Some(condition_satisfied)
        && snapshot.cooling_limit_rejected != condition_satisfied
        && snapshot.capacity_limit_body_entered == condition_satisfied
        && snapshot.active_guard_false_fallthrough != condition_satisfied
}

fn skipped_source_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> bool {
    !snapshot.first_cooling_limit_read
        && snapshot.first_cooling_limit.is_none()
        && !snapshot.cooling_limit_capacity_comparison_evaluated
        && snapshot.cooling_limit_capacity.is_none()
        && !snapshot.second_cooling_limit_read
        && snapshot.second_cooling_limit.is_none()
        && !snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated
        && snapshot.cooling_limit_flow_rate_and_capacity.is_none()
        && snapshot.cooling_limit_condition_satisfied.is_none()
        && !snapshot.cooling_limit_rejected
        && !snapshot.capacity_limit_body_entered
        && !snapshot.active_guard_false_fallthrough
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| overflow(label))
}

fn checked_sub(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_sub(right).ok_or_else(|| overflow(label))
}

fn checked_mul(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| overflow(label))
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling positive-supply capacity-limit guard invariant {field} expected {expected}, got {actual}"
        ))
    }
}

fn overflow(label: &str) -> String {
    format!(
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard {label} overflowed"
    )
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn source_counter_overflow_and_impossible_match_fail_closed() {
        let mut state = PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.capacity_limit_guard_evaluation_count = usize::MAX;
        assert!(
            validate_source_counters(&state, IdealLoadsLimit::NoLimit)
                .expect_err("counter overflow must be rejected")
                .contains("overflowed")
        );

        state.capacity_limit_guard_evaluation_count = 0;
        state.cooling_limit_capacity_match_count = 1;
        assert!(
            validate_source_counters(&state, IdealLoadsLimit::NoLimit)
                .expect_err("impossible match count must be rejected")
                .contains("cooling_limit_capacity_match_count")
        );
    }

    #[test]
    fn fixed_selector_history_rejects_self_consistent_corruption() {
        let mut state = PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.capacity_limit_guard_evaluation_count = 2;
        state.source_site_execution_count = 6;
        state.first_cooling_limit_read_count = 2;
        state.cooling_limit_capacity_comparison_count = 2;
        state.cooling_limit_capacity_match_count = 2;
        state.capacity_limit_body_entry_count = 2;
        validate_source_counters(&state, IdealLoadsLimit::LimitCapacity)
            .expect("fixed capacity selector history");

        state.cooling_limit_capacity_match_count = 1;
        state.second_cooling_limit_read_count = 1;
        state.cooling_limit_flow_rate_and_capacity_comparison_count = 1;
        state.cooling_limit_flow_rate_and_capacity_match_count = 1;
        state.source_site_execution_count = 8;
        let error = validate_source_counters(&state, IdealLoadsLimit::LimitCapacity)
            .expect_err("self-consistent mixed history must be rejected");
        assert!(error.contains("cooling_limit_capacity_match_count"));
    }
}
