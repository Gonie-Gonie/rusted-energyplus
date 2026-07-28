//! Run-summary evidence for the bounded PurchasedAir cooling positive-zero reset body.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary>,
    predecessor_cp327: Option<&PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling positive-zero reset-body evidence"
            .to_string()
    })?;
    let predecessor_cp327 = predecessor_cp327.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-zero reset body has no CP327 evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-zero reset body has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-zero reset body has no coupling call count"
            .to_string()
    })?;

    if coupling_call_count == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE
        || predecessor_cp327.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE
        || predecessor_cp327.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling positive-zero reset-body provenance is invalid"
                .to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor = &predecessor_cp327.state;
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
        state.zero_flow_reset_body_entry_count,
        state.active_guard_false_fallthrough_count,
        "active partition",
    )?;
    let expected_body_skips = checked_add(
        skipped,
        state.active_guard_false_fallthrough_count,
        "body-skip partition",
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
            "active_partition",
            state.cooling_body_entry_count,
            active_partition,
        ),
        (
            "body_skip_count",
            expected_body_skips,
            state.body_skip_count,
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
        (
            "zero_flow_reset_body_entry_count",
            predecessor.zero_flow_reset_body_entry_count,
            state.zero_flow_reset_body_entry_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            predecessor.active_guard_false_fallthrough_count,
            state.active_guard_false_fallthrough_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-zero reset body has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-zero reset body has no latest CP327 snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling positive-zero reset body has no declared system"
                .to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling positive-zero reset body has no controlled Zone".to_string()
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
            "direct-zone IdealLoads cooling positive-zero reset-body latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    body: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    body.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE
        && body.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE
        && body.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER
        && body.system == expected_system
        && predecessor.system == expected_system
        && body.parent_call_ordinal == call_count
        && predecessor.parent_call_ordinal == call_count
        && body.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && body.unit_body_entered == predecessor.unit_body_entered
        && body.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && body.predecessor_ems_supply_mass_flow_override_body_entered
            == predecessor.predecessor_ems_supply_mass_flow_override_body_entered
        && body.predecessor_ems_supply_mass_flow_override_body_skipped
            == predecessor.predecessor_ems_supply_mass_flow_override_body_skipped
        && body.predecessor_ems_disabled_fallthrough
            == predecessor.predecessor_ems_disabled_fallthrough
        && body.predecessor_supply_mass_flow_limit_body_entered
            == predecessor.predecessor_supply_mass_flow_limit_body_entered
        && body.predecessor_supply_mass_flow_limit_body_skipped
            == predecessor.predecessor_supply_mass_flow_limit_body_skipped
        && body.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
            == predecessor.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
        && body.predecessor_zero_flow_reset_body_entered
            == predecessor.zero_flow_reset_body_entered
        && body.predecessor_active_guard_false_fallthrough
            == predecessor.active_guard_false_fallthrough
        && body.unit_off_skipped == predecessor.unit_off_skipped
        && body.non_cooling_skipped == predecessor.non_cooling_skipped
        && body.cooling_body_entered == predecessor.cooling_body_entered
        && snapshot_shape(body, predecessor)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling positive-zero reset body {label} overflowed")
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling positive-zero reset-body invariant {field} expected {expected}, got {actual}"
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

    fn active_predecessor(
        supply: f64,
        body_entered: bool,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
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
            supply_mass_flow_rate_at_or_below_very_small_mass_flow: Some(body_entered),
            zero_flow_reset_body_entered: body_entered,
            active_guard_false_fallthrough: !body_entered,
        }
    }

    fn body_snapshot(
        predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
        let body_entered = predecessor.zero_flow_reset_body_entered;
        let assigned = body_entered.then_some(0.0_f64);
        let supply = predecessor.supply_mass_flow_rate_kg_per_s;
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
            system: predecessor.system,
            parent_call_ordinal: predecessor.parent_call_ordinal,
            controlled_zone: predecessor.controlled_zone,
            unit_body_entered: predecessor.unit_body_entered,
            predecessor_cooling_body_entered: predecessor.cooling_body_entered,
            predecessor_ems_supply_mass_flow_override_body_entered: predecessor
                .predecessor_ems_supply_mass_flow_override_body_entered,
            predecessor_ems_supply_mass_flow_override_body_skipped: predecessor
                .predecessor_ems_supply_mass_flow_override_body_skipped,
            predecessor_ems_disabled_fallthrough: predecessor.predecessor_ems_disabled_fallthrough,
            predecessor_supply_mass_flow_limit_body_entered: predecessor
                .predecessor_supply_mass_flow_limit_body_entered,
            predecessor_supply_mass_flow_limit_body_skipped: predecessor
                .predecessor_supply_mass_flow_limit_body_skipped,
            predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: predecessor
                .predecessor_supply_mass_flow_limit_active_guard_false_fallthrough,
            predecessor_zero_flow_reset_body_entered: body_entered,
            predecessor_active_guard_false_fallthrough: predecessor
                .active_guard_false_fallthrough,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            zero_flow_reset_body_entered: body_entered,
            body_skipped: !body_entered,
            active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
            predecessor_supply_mass_flow_rate_kg_per_s: supply,
            supply_mass_flow_rate_positive_zero_assignment_performed: body_entered,
            assigned_supply_mass_flow_rate_kg_per_s: assigned,
            resulting_supply_mass_flow_rate_kg_per_s: supply
                .map(|supply| assigned.unwrap_or(supply)),
        }
    }

    #[test]
    fn json_preserves_false_route_nan_and_true_route_positive_zero_bits() {
        for (supply, body_entered, expected_result) in [
            (f64::NAN, false, "0x7ff8000000000000"),
            (-0.0, true, "0x0000000000000000"),
        ] {
            let snapshot = body_snapshot(active_predecessor(supply, body_entered));
            let mut state =
                ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState::new(
                    snapshot.system,
                );
            state.latest = Some(snapshot);
            let lifecycle =
                PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary {
                    source:
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
                    first_excluded_source:
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
                    state,
                };

            let value = lifecycle_json(&lifecycle);
            assert_eq!(
                value["latest"]["predecessor_supply_mass_flow_rate_kg_per_s_ieee_bits"],
                format!("0x{:016x}", supply.to_bits())
            );
            assert_eq!(
                value["latest"]["resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"],
                expected_result
            );
        }
    }
}
