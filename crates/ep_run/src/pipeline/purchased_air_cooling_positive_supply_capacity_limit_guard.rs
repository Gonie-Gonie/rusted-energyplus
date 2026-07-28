//! Run-summary evidence for the bounded cooling positive-supply capacity-limit guard.

use ep_model::IdealLoadsLimit;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary>,
    predecessor_cp336: Option<
        &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    >,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    model_cooling_limit: Option<IdealLoadsLimit>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling positive-supply capacity-limit guard evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp336.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard has no CP336 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard has no initialization evidence"
            .to_string()
    })?;
    let cooling_limit = model_cooling_limit.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard has no typed cooling-limit selector"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply capacity-limit guard provenance is invalid"
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
        "guard-false partition",
    )?;
    let transition_partition = checked_add(
        skipped,
        state.capacity_limit_guard_evaluation_count,
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
            "capacity_limit_guard_evaluation_count",
            predecessor_state.supply_enthalpy_assignment_count,
            state.capacity_limit_guard_evaluation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state, cooling_limit)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard has no latest CP336 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply capacity-limit guard has no controlled Zone"
            .to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            cooling_limit,
            expected_system,
            expected_zone,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply capacity-limit guard latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    guard: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    cooling_limit: IdealLoadsLimit,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    guard.source == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        && guard.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && guard.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
        && guard.system == expected_system
        && predecessor.system == expected_system
        && guard.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && guard.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && guard.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && guard.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && guard.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot_shape(guard, predecessor, cooling_limit)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply capacity-limit guard {label} overflowed"
        )
    })
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState;
    use serde_json::Value;

    use super::*;

    #[test]
    fn lifecycle_json_locks_all_four_selector_names_and_lazy_site_shapes() {
        for (limit, name, expected_second, expected_sites) in [
            (IdealLoadsLimit::LimitCapacity, "LimitCapacity", None, 3),
            (
                IdealLoadsLimit::LimitFlowRateAndCapacity,
                "LimitFlowRateAndCapacity",
                Some("LimitFlowRateAndCapacity"),
                5,
            ),
            (IdealLoadsLimit::NoLimit, "NoLimit", Some("NoLimit"), 4),
            (
                IdealLoadsLimit::LimitFlowRate,
                "LimitFlowRate",
                Some("LimitFlowRate"),
                4,
            ),
        ] {
            let value = lifecycle_json(&active_lifecycle(limit));
            let latest = &value["latest"];
            assert_eq!(value["source_site_execution_count"], expected_sites);
            assert_eq!(latest["first_cooling_limit"], name);
            assert_eq!(latest["second_cooling_limit"].as_str(), expected_second);
            assert_eq!(
                latest["second_cooling_limit_read"],
                expected_second.is_some()
            );
            assert_eq!(
                latest["cooling_limit_flow_rate_and_capacity_comparison_evaluated"],
                expected_second.is_some()
            );
            assert_exact_snapshot_keys(latest);
        }
    }

    #[test]
    fn lifecycle_json_keeps_inherited_skip_selector_evidence_completely_null() {
        for skips in [
            (true, false, false),
            (false, true, false),
            (false, false, true),
        ] {
            let value = lifecycle_json(&skipped_lifecycle(skips));
            let latest = &value["latest"];
            assert_eq!(value["source_site_execution_count"], 0);
            for field in [
                "capacity_limit_guard_evaluated",
                "first_cooling_limit_read",
                "cooling_limit_capacity_comparison_evaluated",
                "second_cooling_limit_read",
                "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
                "cooling_limit_rejected",
                "capacity_limit_body_entered",
                "active_guard_false_fallthrough",
            ] {
                assert_eq!(latest[field], false, "{field}");
            }
            for field in [
                "first_cooling_limit",
                "cooling_limit_capacity",
                "second_cooling_limit",
                "cooling_limit_flow_rate_and_capacity",
                "cooling_limit_condition_satisfied",
            ] {
                assert!(latest[field].is_null(), "{field}");
            }
            assert_exact_snapshot_keys(latest);
        }
    }

    fn active_lifecycle(
        limit: IdealLoadsLimit,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary {
        let capacity_match = limit == IdealLoadsLimit::LimitCapacity;
        let second_comparison = !capacity_match;
        let combined_match =
            second_comparison && limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
        let selected = capacity_match || combined_match;
        let mut state = PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.transition_count = 1;
        state.capacity_limit_guard_evaluation_count = 1;
        state.source_site_execution_count =
            2 + 2 * usize::from(second_comparison) + usize::from(selected);
        state.first_cooling_limit_read_count = 1;
        state.cooling_limit_capacity_comparison_count = 1;
        state.cooling_limit_capacity_match_count = usize::from(capacity_match);
        state.second_cooling_limit_read_count = usize::from(second_comparison);
        state.cooling_limit_flow_rate_and_capacity_comparison_count =
            usize::from(second_comparison);
        state.cooling_limit_flow_rate_and_capacity_match_count = usize::from(combined_match);
        state.cooling_limit_rejected_count = usize::from(!selected);
        state.capacity_limit_body_entry_count = usize::from(selected);
        state.active_guard_false_fallthrough_count = usize::from(!selected);
        state.latest = Some(snapshot(
            true,
            false,
            false,
            false,
            limit,
            capacity_match,
            second_comparison,
            combined_match,
            selected,
        ));
        lifecycle(state)
    }

    fn skipped_lifecycle(
        (unit_off, non_cooling, positive_skip): (bool, bool, bool),
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary {
        let mut state = PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.transition_count = 1;
        state.unit_off_skip_count = usize::from(unit_off);
        state.non_cooling_skip_count = usize::from(non_cooling);
        state.positive_guard_false_fallthrough_skip_count = usize::from(positive_skip);
        state.latest = Some(snapshot(
            false,
            unit_off,
            non_cooling,
            positive_skip,
            IdealLoadsLimit::NoLimit,
            false,
            false,
            false,
            false,
        ));
        lifecycle(state)
    }

    fn lifecycle(
        state: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary {
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn snapshot(
        active: bool,
        unit_off: bool,
        non_cooling: bool,
        positive_skip: bool,
        limit: IdealLoadsLimit,
        capacity_match: bool,
        second_comparison: bool,
        combined_match: bool,
        selected: bool,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: !unit_off,
            predecessor_cooling_body_entered: active || positive_skip,
            predecessor_no_outdoor_air_fallback_entered: active || positive_skip,
            predecessor_positive_supply_mass_flow_body_entered: active,
            predecessor_active_guard_false_fallthrough: positive_skip,
            unit_off_skipped: unit_off,
            non_cooling_skipped: non_cooling,
            positive_guard_false_fallthrough_skipped: positive_skip,
            capacity_limit_guard_evaluated: active,
            first_cooling_limit_read: active,
            first_cooling_limit: active.then_some(limit),
            cooling_limit_capacity_comparison_evaluated: active,
            cooling_limit_capacity: active.then_some(capacity_match),
            second_cooling_limit_read: active && second_comparison,
            second_cooling_limit: (active && second_comparison).then_some(limit),
            cooling_limit_flow_rate_and_capacity_comparison_evaluated: active
                && second_comparison,
            cooling_limit_flow_rate_and_capacity: (active && second_comparison)
                .then_some(combined_match),
            cooling_limit_condition_satisfied: active.then_some(selected),
            cooling_limit_rejected: active && !selected,
            capacity_limit_body_entered: active && selected,
            active_guard_false_fallthrough: active && !selected,
        }
    }

    fn assert_exact_snapshot_keys(value: &Value) {
        let actual: BTreeSet<_> = value
            .as_object()
            .expect("CP337 latest object")
            .keys()
            .map(String::as_str)
            .collect();
        let expected: BTreeSet<_> = [
            "source",
            "first_excluded_source",
            "source_order",
            "system",
            "parent_call_ordinal",
            "controlled_zone",
            "unit_body_entered",
            "predecessor_cooling_body_entered",
            "predecessor_no_outdoor_air_fallback_entered",
            "predecessor_positive_supply_mass_flow_body_entered",
            "predecessor_active_guard_false_fallthrough",
            "unit_off_skipped",
            "non_cooling_skipped",
            "positive_guard_false_fallthrough_skipped",
            "capacity_limit_guard_evaluated",
            "first_cooling_limit_read",
            "first_cooling_limit",
            "cooling_limit_capacity_comparison_evaluated",
            "cooling_limit_capacity",
            "second_cooling_limit_read",
            "second_cooling_limit",
            "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
            "cooling_limit_flow_rate_and_capacity",
            "cooling_limit_condition_satisfied",
            "cooling_limit_rejected",
            "capacity_limit_body_entered",
            "active_guard_false_fallthrough",
        ]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
        assert!(
            actual.iter().all(|key| !key.contains("ieee")),
            "CP337 must not expose numerical IEEE fields"
        );
    }
}
