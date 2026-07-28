//! Release validation for the bounded cooling positive supply-mass-flow guard.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

use super::super::calc::cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_mixed_air_call;
    let snapshot = output.calculation_cooling_supply_mass_flow_positive_guard;

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(snapshot)
        && snapshots_match_exact_bits(&snapshot, &expected_snapshot(predecessor))
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
    let cooling = predecessor.cooling_call_executed;
    let supply = if cooling {
        predecessor.supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let strictly_positive = supply.map(source_strictly_positive);

    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_call_executed: predecessor.cooling_call_executed,
        predecessor_zero_flow_reset_body_entered: predecessor
            .predecessor_zero_flow_reset_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor
            .predecessor_active_guard_false_fallthrough,
        predecessor_no_outdoor_air_fallback_entered: predecessor.no_outdoor_air_fallback_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        supply_mass_flow_rate_read: cooling,
        supply_mass_flow_rate_kg_per_s: supply,
        supply_mass_flow_rate_strictly_positive_comparison_evaluated: cooling,
        supply_mass_flow_rate_strictly_positive: strictly_positive,
        positive_supply_mass_flow_body_entered: strictly_positive == Some(true),
        active_guard_false_fallthrough: strictly_positive == Some(false),
    }
}

fn source_strictly_positive(value: f64) -> bool {
    value > 0.0
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
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
        state.positive_supply_mass_flow_body_entry_count,
        state.active_guard_false_fallthrough_count,
        "guard_partition_overflow",
        state.cooling_body_entry_count,
    )?;
    let unconditional_active_sites = checked_mul(
        state.cooling_body_entry_count,
        2,
        "unconditional_active_source_site_count_overflow",
        state.source_site_execution_count,
    )?;
    let source_sites = checked_add(
        unconditional_active_sites,
        state.positive_supply_mass_flow_body_entry_count,
        "source_site_execution_count_overflow",
        state.source_site_execution_count,
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
            predecessor.cooling_call_count,
            state.cooling_body_entry_count,
        ),
        (
            "guard_partition",
            state.cooling_body_entry_count,
            guard_partition,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "supply_mass_flow_rate_strictly_positive_comparison_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_strictly_positive_comparison_count,
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
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(latest, &expected_snapshot(predecessor_latest))
        || !snapshots_match_exact_bits(
            latest,
            &latest_output.calculation_cooling_supply_mass_flow_positive_guard,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    right: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    let value_matches = options_have_exact_bits(
        left.supply_mass_flow_rate_kg_per_s,
        right.supply_mass_flow_rate_kg_per_s,
    );
    let mut left_without_value = *left;
    let mut right_without_value = *right;
    left_without_value.supply_mass_flow_rate_kg_per_s = None;
    right_without_value.supply_mass_flow_rate_kg_per_s = None;
    value_matches && left_without_value == right_without_value
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
    Error::CalcCoolingSupplyMassFlowPositiveGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_comparison_preserves_nan_signed_zero_and_infinity_semantics() {
        for (value, expected) in [
            (f64::NEG_INFINITY, false),
            (-1.0, false),
            (-0.0, false),
            (0.0, false),
            (f64::from_bits(1), true),
            (1.0, true),
            (f64::INFINITY, true),
            (f64::from_bits(0x7ff8_0000_0000_00a1), false),
            (f64::from_bits(0xfff8_0000_0000_00b2), false),
        ] {
            assert_eq!(source_strictly_positive(value), expected);
        }
    }

    #[test]
    fn source_site_count_multiplication_overflow_fails_closed() {
        let error = checked_mul(usize::MAX, 2, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");

        assert!(matches!(
            error,
            Error::CalcCoolingSupplyMassFlowPositiveGuardLifecycleInvariant {
                field: "test_source_site_count_overflow",
                expected: usize::MAX,
                actual: usize::MAX,
            }
        ));
    }

    #[test]
    fn snapshot_comparison_detects_signed_zero_bit_corruption() {
        let snapshot = PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
            system: ep_model::IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ep_model::ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_call_executed: true,
            predecessor_zero_flow_reset_body_entered: true,
            predecessor_active_guard_false_fallthrough: false,
            predecessor_no_outdoor_air_fallback_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(0.0),
            supply_mass_flow_rate_strictly_positive_comparison_evaluated: true,
            supply_mass_flow_rate_strictly_positive: Some(false),
            positive_supply_mass_flow_body_entered: false,
            active_guard_false_fallthrough: true,
        };
        let mut negative_zero = snapshot;
        negative_zero.supply_mass_flow_rate_kg_per_s = Some(-0.0);

        assert_eq!(snapshot, negative_zero);
        assert!(snapshots_match_exact_bits(&snapshot, &snapshot));
        assert!(!snapshots_match_exact_bits(&snapshot, &negative_zero));
    }
}
