//! Run-summary evidence for the bounded capacity-limit `CpAir` assignment.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    >,
    predecessor_cp337: Option<
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    >,
    predecessor_cp329: Option<&PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose capacity-limit CpAir assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp337.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no CP337 evidence".to_string()
    })?;
    let mixed_air = predecessor_cp329.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no CP329 evidence".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || mixed_air.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads capacity-limit CpAir assignment provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let skipped = checked_add(
        skipped,
        state.positive_guard_false_fallthrough_skip_count,
        "positive-guard-false partition",
    )?;
    let skipped = checked_add(
        skipped,
        state.capacity_limit_guard_false_fallthrough_skip_count,
        "capacity-guard-false partition",
    )?;
    let transition_partition = checked_add(
        skipped,
        state.capacity_limit_cp_air_assignment_count,
        "transition partition",
    )?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        (
            "mixed_air_transition_count",
            mixed_air.state.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "unit_off_skip_count",
            predecessor_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor_state.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            predecessor_state.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "capacity_limit_guard_false_fallthrough_skip_count",
            predecessor_state.active_guard_false_fallthrough_count,
            state.capacity_limit_guard_false_fallthrough_skip_count,
        ),
        (
            "capacity_limit_cp_air_assignment_count",
            predecessor_state.capacity_limit_body_entry_count,
            state.capacity_limit_cp_air_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no latest CP337 snapshot"
            .to_string()
    })?;
    let mixed_air_latest = mixed_air.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no latest CP329 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no declared system".to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit CpAir assignment has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || mixed_air.state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            mixed_air_latest,
            expected_system,
            expected_zone,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads capacity-limit CpAir assignment latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    assignment: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    assignment.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE
        && assignment.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && assignment.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER
        && mixed_air.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && mixed_air.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && mixed_air.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && mixed_air.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && mixed_air.no_oa_child_source_order
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER
        && assignment.system == expected_system
        && predecessor.system == expected_system
        && mixed_air.system == expected_system
        && assignment.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && mixed_air.parent_call_ordinal == call_count
        && assignment.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && mixed_air.controlled_zone == expected_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot_shape(assignment, predecessor, mixed_air)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads capacity-limit CpAir assignment {label} overflowed")
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads capacity-limit CpAir assignment invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_partition_overflow_fails_closed() {
        let error = checked_add(usize::MAX, 1, "test partition")
            .expect_err("partition overflow must fail closed");
        assert!(error.contains("overflowed"));
    }
}
