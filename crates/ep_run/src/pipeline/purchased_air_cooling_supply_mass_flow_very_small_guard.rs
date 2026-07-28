//! Run-summary evidence for the bounded PurchasedAir cooling very-small-flow guard.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary>,
    predecessor_cp326: Option<&PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling very-small-flow guard evidence"
            .to_string()
    })?;
    let predecessor_cp326 = predecessor_cp326.ok_or_else(|| {
        "direct-zone IdealLoads cooling very-small-flow guard has no CP326 evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling very-small-flow guard has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling very-small-flow guard has no coupling call count"
            .to_string()
    })?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp326.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE
        || predecessor_cp326.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling very-small-flow guard provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor = &predecessor_cp326.state;
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
    let guard_partition = checked_add(
        state.zero_flow_reset_body_entry_count,
        state.active_guard_false_fallthrough_count,
        "guard partition",
    )?;
    for (field, expected, actual) in [
        (
            "transition_count",
            coupling_call_count,
            state.transition_count,
        ),
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
            "guard_partition",
            state.cooling_body_entry_count,
            guard_partition,
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
            "cooling_body_entry_count",
            predecessor.cooling_body_entry_count,
            state.cooling_body_entry_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling very-small-flow guard has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling very-small-flow guard has no latest CP326 snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling very-small-flow guard has no declared system"
                .to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling very-small-flow guard has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor_latest,
            expected_system,
            expected_zone,
            coupling_call_count,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling very-small-flow guard latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    guard: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    guard.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE
        && guard.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE
        && guard.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER
        && guard.system == expected_system
        && predecessor.system == expected_system
        && guard.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && guard.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && guard.predecessor_ems_supply_mass_flow_override_body_entered
            == predecessor.predecessor_ems_supply_mass_flow_override_body_entered
        && guard.predecessor_ems_supply_mass_flow_override_body_skipped
            == predecessor.predecessor_ems_supply_mass_flow_override_body_skipped
        && guard.predecessor_ems_disabled_fallthrough
            == predecessor.predecessor_ems_disabled_fallthrough
        && guard.predecessor_supply_mass_flow_limit_body_entered
            == predecessor.supply_mass_flow_limit_body_entered
        && guard.predecessor_supply_mass_flow_limit_body_skipped == predecessor.body_skipped
        && guard.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
            == predecessor.active_guard_false_fallthrough
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.cooling_body_entered == predecessor.cooling_body_entered
        && snapshot_shape(guard, predecessor.resulting_supply_mass_flow_rate_kg_per_s)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling very-small-flow guard {label} overflowed")
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling very-small-flow guard invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    };

    use super::*;

    fn active_snapshot(supply: f64) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
        let comparison = supply <= ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S;
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_ems_supply_mass_flow_override_body_entered: false,
            predecessor_ems_supply_mass_flow_override_body_skipped: true,
            predecessor_ems_disabled_fallthrough: true,
            predecessor_supply_mass_flow_limit_body_entered: false,
            predecessor_supply_mass_flow_limit_body_skipped: true,
            predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(supply),
            hvac_very_small_mass_flow_read: true,
            hvac_very_small_mass_flow_source: Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE),
            hvac_very_small_mass_flow_kg_per_s: Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S),
            supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated: true,
            supply_mass_flow_rate_at_or_below_very_small_mass_flow: Some(comparison),
            zero_flow_reset_body_entered: comparison,
            active_guard_false_fallthrough: !comparison,
        }
    }

    #[test]
    fn json_preserves_signed_zero_and_nan_supply_bits() {
        for (supply, expected) in [
            (0.0, "0x0000000000000000"),
            (-0.0, "0x8000000000000000"),
            (f64::NAN, "0x7ff8000000000000"),
        ] {
            let snapshot = active_snapshot(supply);
            let mut state =
                ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState::new(
                    snapshot.system,
                );
            state.latest = Some(snapshot);
            let lifecycle =
                PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary {
                    source:
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
                    first_excluded_source:
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
                    state,
                };

            let value = lifecycle_json(&lifecycle);
            let bits = &value["latest"]["supply_mass_flow_rate_kg_per_s_ieee_bits"];
            assert_eq!(bits, expected);
            assert_eq!(bits.as_str().map(str::len), Some(18));
        }
    }
}
