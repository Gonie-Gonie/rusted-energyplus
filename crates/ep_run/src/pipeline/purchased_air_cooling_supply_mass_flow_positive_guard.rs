//! Run-summary evidence for the bounded Cooling positive-supply-flow guard.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary>,
    predecessor_cp329: Option<&PurchasedAirCalcCoolingMixedAirCallLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling positive-supply-flow guard evidence"
            .to_string()
    })?;
    let predecessor = predecessor_cp329.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply-flow guard has no CP329 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply-flow guard has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply-flow guard has no coupling call count"
            .to_string()
    })?;

    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || predecessor.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply-flow guard provenance is invalid"
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
    let transition_partition = checked_add(
        skipped,
        state.cooling_body_entry_count,
        "transition partition",
    )?;
    let active_partition = checked_add(
        state.positive_supply_mass_flow_body_entry_count,
        state.active_guard_false_fallthrough_count,
        "active guard partition",
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
            "active_guard_partition",
            state.cooling_body_entry_count,
            active_partition,
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
            "cooling_body_entry_count",
            predecessor_state.cooling_call_count,
            state.cooling_body_entry_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply-flow guard has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply-flow guard has no latest CP329 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply-flow guard has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-supply-flow guard has no controlled Zone"
            .to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            expected_system,
            expected_zone,
            calls,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling positive-supply-flow guard latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    guard: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    predecessor: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    guard.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        && guard.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        && guard.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && predecessor.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && predecessor.no_oa_child_source_order
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER
        && guard.system == expected_system
        && predecessor.system == expected_system
        && guard.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && guard.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_call_executed == predecessor.cooling_call_executed
        && guard.predecessor_zero_flow_reset_body_entered
            == predecessor.predecessor_zero_flow_reset_body_entered
        && guard.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && guard.predecessor_no_outdoor_air_fallback_entered
            == predecessor.no_outdoor_air_fallback_entered
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.cooling_body_entered == predecessor.cooling_call_executed
        && snapshot_shape(guard, predecessor)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling positive-supply-flow guard {label} overflowed")
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling positive-supply-flow guard invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};

    use super::*;

    fn active_snapshot(supply: f64) -> PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
        let positive = supply > 0.0;
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_call_executed: true,
            predecessor_zero_flow_reset_body_entered: !positive,
            predecessor_active_guard_false_fallthrough: positive,
            predecessor_no_outdoor_air_fallback_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(supply),
            supply_mass_flow_rate_strictly_positive_comparison_evaluated: true,
            supply_mass_flow_rate_strictly_positive: Some(positive),
            positive_supply_mass_flow_body_entered: positive,
            active_guard_false_fallthrough: !positive,
        }
    }

    #[test]
    fn json_preserves_signed_zero_infinity_and_nan_supply_bits() {
        for (supply, expected) in [
            (0.0, "0x0000000000000000"),
            (-0.0, "0x8000000000000000"),
            (f64::INFINITY, "0x7ff0000000000000"),
            (f64::from_bits(0x7ff8_0000_0000_0042), "0x7ff8000000000042"),
        ] {
            let snapshot = active_snapshot(supply);
            let mut state =
                ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(
                    snapshot.system,
                );
            state.latest = Some(snapshot);
            let lifecycle = PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary {
                source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
                state,
            };

            let value = lifecycle_json(&lifecycle);
            assert_eq!(
                value["latest"]["supply_mass_flow_rate_kg_per_s_ieee_bits"],
                expected
            );
        }
    }
}
