//! Release validation for the bounded cooling supply-mass-flow positive-zero reset body.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

use super::super::calc::cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_mass_flow_very_small_guard;
    let snapshot = output.calculation_cooling_supply_mass_flow_very_small_guard_body;

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(snapshot)
        && snapshots_match_exact_bits(&snapshot, &expected_snapshot(predecessor))
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
    let cooling = predecessor.cooling_body_entered;
    let body_entered = predecessor.zero_flow_reset_body_entered;
    let supply_before = if cooling {
        predecessor.supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let assigned = body_entered.then_some(0.0_f64);
    let resulting = supply_before.map(|supply| assigned.unwrap_or(supply));

    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
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
        predecessor_zero_flow_reset_body_entered: predecessor.zero_flow_reset_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        zero_flow_reset_body_entered: body_entered,
        body_skipped: !body_entered,
        active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        predecessor_supply_mass_flow_rate_kg_per_s: supply_before,
        supply_mass_flow_rate_positive_zero_assignment_performed: body_entered,
        assigned_supply_mass_flow_rate_kg_per_s: assigned,
        resulting_supply_mass_flow_rate_kg_per_s: resulting,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
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
    let transition_partition = checked_add(
        skipped,
        state.cooling_body_entry_count,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let active_partition = checked_add(
        state.zero_flow_reset_body_entry_count,
        state.active_guard_false_fallthrough_count,
        "active_partition_overflow",
        state.cooling_body_entry_count,
    )?;
    let expected_body_skips = checked_add(
        skipped,
        state.active_guard_false_fallthrough_count,
        "body_skip_partition_overflow",
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
            "supply_mass_flow_rate_positive_zero_assignment_count",
            state.zero_flow_reset_body_entry_count,
            state.supply_mass_flow_rate_positive_zero_assignment_count,
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
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(latest, &expected_snapshot(predecessor_latest))
        || !snapshots_match_exact_bits(
            latest,
            &latest_output.calculation_cooling_supply_mass_flow_very_small_guard_body,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    right: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_supply_mass_flow_rate_kg_per_s,
            right.predecessor_supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.assigned_supply_mass_flow_rate_kg_per_s,
            right.assigned_supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.resulting_supply_mass_flow_rate_kg_per_s,
            right.resulting_supply_mass_flow_rate_kg_per_s,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s = None;
        snapshot.assigned_supply_mass_flow_rate_kg_per_s = None;
        snapshot.resulting_supply_mass_flow_rate_kg_per_s = None;
    }
    values_match && left_without_values == right_without_values
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

pub(super) fn checked_add(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_add(right)
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
    Error::CalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};

    use super::*;
    use crate::ideal_loads::{
        ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    };

    fn predecessor_with_route(
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

    #[test]
    fn expected_snapshot_assigns_positive_zero_only_on_predecessor_true_route() {
        for supply in [
            -0.0,
            0.0,
            ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S,
            -1.0,
            f64::NEG_INFINITY,
        ] {
            let snapshot = expected_snapshot(predecessor_with_route(supply, true));
            assert_eq!(
                snapshot
                    .predecessor_supply_mass_flow_rate_kg_per_s
                    .map(f64::to_bits),
                Some(supply.to_bits())
            );
            assert_eq!(
                snapshot
                    .assigned_supply_mass_flow_rate_kg_per_s
                    .map(f64::to_bits),
                Some(0)
            );
            assert_eq!(
                snapshot
                    .resulting_supply_mass_flow_rate_kg_per_s
                    .map(f64::to_bits),
                Some(0)
            );
        }
    }

    #[test]
    fn expected_snapshot_retains_false_route_bits_without_assignment() {
        for supply in [
            f64::from_bits(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S.to_bits() + 1),
            f64::INFINITY,
            f64::NAN,
        ] {
            let snapshot = expected_snapshot(predecessor_with_route(supply, false));
            assert_eq!(snapshot.assigned_supply_mass_flow_rate_kg_per_s, None);
            assert_eq!(
                snapshot
                    .resulting_supply_mass_flow_rate_kg_per_s
                    .map(f64::to_bits),
                Some(supply.to_bits())
            );
        }
    }

    #[test]
    fn snapshot_comparison_detects_signed_zero_result_corruption() {
        let expected = expected_snapshot(predecessor_with_route(-0.0, true));
        let mut corrupted = expected;
        corrupted.assigned_supply_mass_flow_rate_kg_per_s = Some(-0.0);
        corrupted.resulting_supply_mass_flow_rate_kg_per_s = Some(-0.0);

        assert_eq!(expected, corrupted);
        assert!(snapshots_match_exact_bits(&expected, &expected));
        assert!(!snapshots_match_exact_bits(&expected, &corrupted));
    }
}
