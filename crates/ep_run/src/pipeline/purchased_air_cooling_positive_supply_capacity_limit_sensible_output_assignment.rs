//! Run-summary evidence for the bounded capacity-limit sensible-output assignment.

use ep_model::IdealLoadsLimit;
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
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

#[allow(clippy::too_many_arguments)]
pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    >,
    predecessor_cp338: Option<
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    >,
    selector_cp337: Option<
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    >,
    supply_flow_cp330: Option<&PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary>,
    mixed_air_cp329: Option<&PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    supply_enthalpy_cp336: Option<
        &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    >,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    cooling_limit: Option<IdealLoadsLimit>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose capacity-limit sensible-output assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp338.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no CP338 evidence"
            .to_string()
    })?;
    let selector = selector_cp337.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no CP337 evidence"
            .to_string()
    })?;
    let supply_flow = supply_flow_cp330.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no CP330 evidence"
            .to_string()
    })?;
    let mixed_air = mixed_air_cp329.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no CP329 evidence"
            .to_string()
    })?;
    let supply_enthalpy = supply_enthalpy_cp336.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no CP336 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no initialization evidence"
            .to_string()
    })?;
    let cooling_limit = cooling_limit.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no fixed cooling selector"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || selector.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        || selector.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || supply_flow.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || supply_flow.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || mixed_air.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || supply_enthalpy.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || supply_enthalpy.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads capacity-limit sensible-output assignment provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let selector_state = &selector.state;
    let supply_flow_state = &supply_flow.state;
    let mixed_air_state = &mixed_air.state;
    let supply_enthalpy_state = &supply_enthalpy.state;
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
        state.capacity_limit_sensible_output_assignment_count,
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
            "selector_transition_count",
            selector_state.transition_count,
            state.transition_count,
        ),
        (
            "supply_flow_transition_count",
            supply_flow_state.transition_count,
            state.transition_count,
        ),
        (
            "mixed_air_transition_count",
            mixed_air_state.transition_count,
            state.transition_count,
        ),
        (
            "supply_enthalpy_transition_count",
            supply_enthalpy_state.transition_count,
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
            predecessor_state.capacity_limit_guard_false_fallthrough_skip_count,
            state.capacity_limit_guard_false_fallthrough_skip_count,
        ),
        (
            "capacity_limit_sensible_output_assignment_count",
            predecessor_state.capacity_limit_cp_air_assignment_count,
            state.capacity_limit_sensible_output_assignment_count,
        ),
        (
            "selector_capacity_limit_body_entry_count",
            selector_state.capacity_limit_body_entry_count,
            state.capacity_limit_sensible_output_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no latest CP338 snapshot"
            .to_string()
    })?;
    let selector_latest = selector_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no latest CP337 snapshot"
            .to_string()
    })?;
    let supply_flow_latest = supply_flow_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no latest CP330 snapshot"
            .to_string()
    })?;
    let mixed_air_latest = mixed_air_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no latest CP329 snapshot"
            .to_string()
    })?;
    let supply_enthalpy_latest = supply_enthalpy_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no latest CP336 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit sensible-output assignment has no controlled Zone"
            .to_string()
    })?;

    if state.system != expected_system
        || predecessor_state.system != expected_system
        || selector_state.system != expected_system
        || supply_flow_state.system != expected_system
        || mixed_air_state.system != expected_system
        || supply_enthalpy_state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            selector_latest,
            supply_flow_latest,
            mixed_air_latest,
            supply_enthalpy_latest,
            expected_system,
            expected_zone,
            cooling_limit,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads capacity-limit sensible-output assignment latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn latest_matches_release(
    assignment: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    selector: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    supply_flow: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    supply_enthalpy: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    cooling_limit: IdealLoadsLimit,
    call_count: usize,
) -> bool {
    assignment.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        && assignment.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && assignment.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER
        && selector.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        && selector.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && selector.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER
        && supply_flow.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        && supply_flow.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        && supply_flow.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER
        && mixed_air.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && mixed_air.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && mixed_air.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && mixed_air.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && mixed_air.no_oa_child_source_order
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER
        && supply_enthalpy.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        && supply_enthalpy.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && supply_enthalpy.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
        && [
            assignment.system,
            predecessor.system,
            selector.system,
            supply_flow.system,
            mixed_air.system,
            supply_enthalpy.system,
        ]
        .into_iter()
        .all(|system| system == expected_system)
        && [
            assignment.parent_call_ordinal,
            predecessor.parent_call_ordinal,
            selector.parent_call_ordinal,
            supply_flow.parent_call_ordinal,
            mixed_air.parent_call_ordinal,
            supply_enthalpy.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == call_count)
        && [
            assignment.controlled_zone,
            predecessor.controlled_zone,
            selector.controlled_zone,
            supply_flow.controlled_zone,
            mixed_air.controlled_zone,
            supply_enthalpy.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        && selector_matches_fixed_limit(selector, cooling_limit)
        && snapshot_shape(
            assignment,
            predecessor,
            supply_flow,
            mixed_air,
            supply_enthalpy,
        )
}

fn selector_matches_fixed_limit(
    selector: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    if !selector.capacity_limit_guard_evaluated {
        return selector.first_cooling_limit.is_none()
            && selector.second_cooling_limit.is_none()
            && !selector.capacity_limit_body_entered;
    }
    selector.first_cooling_limit == Some(cooling_limit)
        && match cooling_limit {
            IdealLoadsLimit::LimitCapacity => {
                selector.cooling_limit_capacity == Some(true)
                    && selector.second_cooling_limit.is_none()
                    && selector.capacity_limit_body_entered
            }
            IdealLoadsLimit::LimitFlowRateAndCapacity => {
                selector.cooling_limit_capacity == Some(false)
                    && selector.second_cooling_limit == Some(cooling_limit)
                    && selector.cooling_limit_flow_rate_and_capacity == Some(true)
                    && selector.capacity_limit_body_entered
            }
            IdealLoadsLimit::NoLimit | IdealLoadsLimit::LimitFlowRate => {
                selector.cooling_limit_capacity == Some(false)
                    && selector.second_cooling_limit == Some(cooling_limit)
                    && selector.cooling_limit_flow_rate_and_capacity == Some(false)
                    && selector.active_guard_false_fallthrough
            }
        }
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads capacity-limit sensible-output assignment {label} overflowed"
        )
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads capacity-limit sensible-output assignment invariant {field} expected {expected}, got {actual}"
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
