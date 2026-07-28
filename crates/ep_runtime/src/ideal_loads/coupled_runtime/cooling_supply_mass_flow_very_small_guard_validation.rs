//! Release validation for the bounded cooling supply mass-flow very-small guard.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

use super::super::calc::cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_mass_flow_limit_body;
    let snapshot = output.calculation_cooling_supply_mass_flow_very_small_guard;

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(snapshot)
        && snapshots_match_exact_bits(&snapshot, &expected_snapshot(predecessor))
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let supply = if cooling {
        predecessor.resulting_supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let threshold = cooling.then_some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S);
    let comparison = supply
        .zip(threshold)
        .map(|(supply, threshold)| supply <= threshold);

    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
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
            .supply_mass_flow_limit_body_entered,
        predecessor_supply_mass_flow_limit_body_skipped: predecessor.body_skipped,
        predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: predecessor
            .active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        supply_mass_flow_rate_read: cooling,
        supply_mass_flow_rate_kg_per_s: supply,
        hvac_very_small_mass_flow_read: cooling,
        hvac_very_small_mass_flow_source: cooling
            .then_some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE),
        hvac_very_small_mass_flow_kg_per_s: threshold,
        supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated: cooling,
        supply_mass_flow_rate_at_or_below_very_small_mass_flow: comparison,
        zero_flow_reset_body_entered: comparison == Some(true),
        active_guard_false_fallthrough: comparison == Some(false),
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
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
    let guard_partition = checked_add(
        state.zero_flow_reset_body_entry_count,
        state.active_guard_false_fallthrough_count,
        "guard_partition_overflow",
        state.cooling_body_entry_count,
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
            "guard_partition",
            state.cooling_body_entry_count,
            guard_partition,
        ),
        (
            "supply_mass_flow_rate_read_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "hvac_very_small_mass_flow_read_count",
            state.cooling_body_entry_count,
            state.hvac_very_small_mass_flow_read_count,
        ),
        (
            "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count,
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
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(latest, &expected_snapshot(predecessor_latest))
        || !snapshots_match_exact_bits(
            latest,
            &latest_output.calculation_cooling_supply_mass_flow_very_small_guard,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    right: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    let values_match = [
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.hvac_very_small_mass_flow_kg_per_s,
            right.hvac_very_small_mass_flow_kg_per_s,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.hvac_very_small_mass_flow_kg_per_s = None;
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
    Error::CalcCoolingSupplyMassFlowVerySmallGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};

    use super::*;

    fn predecessor_with_result(
        result: f64,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
            source: crate::ideal_loads::
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
            first_excluded_source: crate::ideal_loads::
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
            source_order: crate::ideal_loads::
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_ems_supply_mass_flow_override_body_entered: false,
            predecessor_ems_supply_mass_flow_override_body_skipped: true,
            predecessor_ems_disabled_fallthrough: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_limit_body_entered: false,
            body_skipped: true,
            active_guard_false_fallthrough: true,
            supply_mass_flow_rate_for_minimum_read: false,
            supply_mass_flow_rate_before_limit_kg_per_s: None,
            maximum_cooling_air_mass_flow_rate_for_minimum_read: false,
            maximum_cooling_air_mass_flow_rate_kg_per_s: None,
            source_shaped_two_argument_minimum_evaluated: false,
            minimum_supply_mass_flow_rate_kg_per_s: None,
            supply_mass_flow_rate_assignment_performed: false,
            assigned_supply_mass_flow_rate_kg_per_s: None,
            resulting_supply_mass_flow_rate_kg_per_s: Some(result),
        }
    }

    #[test]
    fn expected_snapshot_preserves_cp326_bits_and_characterizes_threshold_cases() {
        for (supply, expected) in [
            (-0.0, true),
            (0.0, true),
            (ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, true),
            (
                f64::from_bits(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S.to_bits() + 1),
                false,
            ),
            (-1.0, true),
            (f64::NAN, false),
        ] {
            let snapshot = expected_snapshot(predecessor_with_result(supply));
            assert_eq!(
                snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
                Some(supply.to_bits())
            );
            assert_eq!(
                snapshot
                    .hvac_very_small_mass_flow_kg_per_s
                    .map(f64::to_bits),
                Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S.to_bits())
            );
            assert_eq!(
                snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow,
                Some(expected)
            );
            assert_eq!(snapshot.zero_flow_reset_body_entered, expected);
            assert_eq!(snapshot.active_guard_false_fallthrough, !expected);
        }
    }

    #[test]
    fn snapshot_comparison_detects_signed_zero_bit_corruption() {
        let positive_zero = expected_snapshot(predecessor_with_result(0.0));
        let negative_zero = expected_snapshot(predecessor_with_result(-0.0));

        assert_eq!(positive_zero, negative_zero);
        assert!(snapshots_match_exact_bits(&positive_zero, &positive_zero));
        assert!(!snapshots_match_exact_bits(&positive_zero, &negative_zero));
    }
}
