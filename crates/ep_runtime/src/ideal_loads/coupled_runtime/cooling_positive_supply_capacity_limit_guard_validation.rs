//! Release validation for the bounded cooling positive-supply capacity-limit guard.

use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
};

use super::super::calc::cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_positive_supply_enthalpy_assignment;
    let snapshot = output.calculation_cooling_positive_supply_capacity_limit_guard;
    let expected = expected_snapshot(predecessor, binding.system.cooling_limit);

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot)
        && snapshot == expected
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
    let active = predecessor.supply_enthalpy_assignment_executed;
    let capacity_match = active && cooling_limit == IdealLoadsLimit::LimitCapacity;
    let second_comparison = active && !capacity_match;
    let combined_match =
        second_comparison && cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let condition_satisfied = capacity_match || combined_match;

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor
            .predecessor_active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_evaluated: active,
        first_cooling_limit_read: active,
        first_cooling_limit: active.then_some(cooling_limit),
        cooling_limit_capacity_comparison_evaluated: active,
        cooling_limit_capacity: active.then_some(capacity_match),
        second_cooling_limit_read: second_comparison,
        second_cooling_limit: second_comparison.then_some(cooling_limit),
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: second_comparison,
        cooling_limit_flow_rate_and_capacity: second_comparison.then_some(combined_match),
        cooling_limit_condition_satisfied: active.then_some(condition_satisfied),
        cooling_limit_rejected: active && !condition_satisfied,
        capacity_limit_body_entered: active && condition_satisfied,
        active_guard_false_fallthrough: active && !condition_satisfied,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let skipped = checked_add(
        skipped,
        state.positive_guard_false_fallthrough_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        skipped,
        state.capacity_limit_guard_evaluation_count,
        "transition_partition_overflow",
        timestep_count,
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
            "capacity_limit_guard_evaluation_count",
            predecessor.supply_enthalpy_assignment_count,
            state.capacity_limit_guard_evaluation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_selector_counters(state, binding.system.cooling_limit)?;

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let expected = expected_snapshot(*predecessor_latest, binding.system.cooling_limit);
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER.len() != 5
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || *latest != expected
        || *latest != latest_output.calculation_cooling_positive_supply_capacity_limit_guard
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_selector_counters(
    state: &crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    cooling_limit: IdealLoadsLimit,
) -> Result<(), Error> {
    let active = state.capacity_limit_guard_evaluation_count;
    let capacity_matches = if cooling_limit == IdealLoadsLimit::LimitCapacity {
        active
    } else {
        0
    };
    let second_comparisons = checked_sub(
        active,
        capacity_matches,
        "second_comparison_partition_underflow",
        state.second_cooling_limit_read_count,
    )?;
    let combined_matches = if cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity {
        active
    } else {
        0
    };
    let body_entries = checked_add(
        capacity_matches,
        combined_matches,
        "body_entry_partition_overflow",
        state.capacity_limit_body_entry_count,
    )?;
    let false_fallthroughs = checked_sub(
        active,
        body_entries,
        "active_false_partition_underflow",
        state.active_guard_false_fallthrough_count,
    )?;
    let first_sites = checked_mul(
        active,
        2,
        "first_source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;
    let second_sites = checked_mul(
        second_comparisons,
        2,
        "second_source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;
    let source_sites = checked_add(
        first_sites,
        second_sites,
        "source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;
    let source_sites = checked_add(
        source_sites,
        body_entries,
        "source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;

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
            false_fallthroughs,
            state.cooling_limit_rejected_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            false_fallthroughs,
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

fn checked_add(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn checked_sub(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_sub(right)
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
    Error::CalcCoolingPositiveSupplyCapacityLimitGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn lifecycle_arithmetic_overflow_and_underflow_fail_closed() {
        for error in [
            checked_add(usize::MAX, 1, "test_add_overflow", usize::MAX)
                .expect_err("addition overflow must fail closed"),
            checked_sub(0, 1, "test_sub_underflow", 0)
                .expect_err("subtraction underflow must fail closed"),
            checked_mul(usize::MAX, 2, "test_mul_overflow", usize::MAX)
                .expect_err("multiplication overflow must fail closed"),
        ] {
            assert!(matches!(
                error,
                Error::CalcCoolingPositiveSupplyCapacityLimitGuardLifecycleInvariant { .. }
            ));
        }
    }

    #[test]
    fn fixed_selector_history_rejects_self_consistent_corruption() {
        let mut state =
            crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_guard_evaluation_count = 2;
        state.source_site_execution_count = 6;
        state.first_cooling_limit_read_count = 2;
        state.cooling_limit_capacity_comparison_count = 2;
        state.cooling_limit_capacity_match_count = 2;
        state.capacity_limit_body_entry_count = 2;
        validate_selector_counters(&state, IdealLoadsLimit::LimitCapacity)
            .expect("fixed capacity selector history");

        state.cooling_limit_capacity_match_count = 1;
        state.second_cooling_limit_read_count = 1;
        state.cooling_limit_flow_rate_and_capacity_comparison_count = 1;
        state.cooling_limit_flow_rate_and_capacity_match_count = 1;
        state.source_site_execution_count = 8;
        assert!(matches!(
            validate_selector_counters(&state, IdealLoadsLimit::LimitCapacity),
            Err(
                Error::CalcCoolingPositiveSupplyCapacityLimitGuardLifecycleInvariant {
                    field: "cooling_limit_capacity_match_count",
                    expected: 2,
                    actual: 1,
                }
            )
        ));
    }
}
