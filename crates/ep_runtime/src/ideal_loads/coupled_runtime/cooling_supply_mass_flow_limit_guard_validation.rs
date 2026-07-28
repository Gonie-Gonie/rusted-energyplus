//! Release validation for the bounded cooling supply mass-flow limit guard.

use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    IdealLoadsSensibleMode,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};

use super::super::calc::cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_mass_flow_ems_override_body;
    let snapshot = output.calculation_cooling_supply_mass_flow_limit_guard;
    let initialized_maximum = output
        .initialization
        .maximum_cooling_air_mass_flow_rate_kg_per_s;
    let numerical_cooling =
        output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling;

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && predecessor.cooling_body_entered == numerical_cooling
        && cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(snapshot)
        && selected_option_has_exact_bits(
            snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
            snapshot.maximum_cooling_air_mass_flow_rate_read,
            initialized_maximum,
        )
        && snapshot
            == expected_snapshot(
                predecessor,
                binding.system.cooling_limit,
                initialized_maximum,
                call_ordinal,
                binding,
            )
}

fn selected_option_has_exact_bits(value: Option<f64>, selected: bool, expected: f64) -> bool {
    if selected {
        value.is_some_and(|value| value.to_bits() == expected.to_bits())
    } else {
        value.is_none()
    }
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    cooling_limit: IdealLoadsLimit,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let flow_rate = cooling && cooling_limit == IdealLoadsLimit::LimitFlowRate;
    let read_second = cooling && !flow_rate;
    let flow_rate_and_capacity =
        read_second && cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let selected = flow_rate || flow_rate_and_capacity;
    let positive = selected && maximum_cooling_air_mass_flow_rate_kg_per_s > 0.0;

    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
        system: binding.ideal_loads_air_system,
        parent_call_ordinal: call_ordinal,
        controlled_zone: binding.zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_ems_supply_mass_flow_override_body_entered: predecessor
            .predecessor_ems_supply_mass_flow_override_body_entered,
        predecessor_ems_supply_mass_flow_override_body_skipped: predecessor.body_skipped,
        predecessor_ems_disabled_fallthrough: predecessor.ems_disabled_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        first_cooling_limit_read: cooling,
        first_cooling_limit: cooling.then_some(cooling_limit),
        cooling_limit_flow_rate_comparison_evaluated: cooling,
        cooling_limit_flow_rate: cooling.then_some(flow_rate),
        second_cooling_limit_read: read_second,
        second_cooling_limit: read_second.then_some(cooling_limit),
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: read_second,
        cooling_limit_flow_rate_and_capacity: read_second.then_some(flow_rate_and_capacity),
        cooling_limit_condition_satisfied: cooling.then_some(selected),
        maximum_cooling_air_mass_flow_rate_read: selected,
        maximum_cooling_air_mass_flow_rate_kg_per_s: selected
            .then_some(maximum_cooling_air_mass_flow_rate_kg_per_s),
        maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: selected,
        maximum_cooling_air_mass_flow_rate_strictly_positive: selected.then_some(positive),
        supply_mass_flow_limit_body_entered: positive,
        active_guard_false_fallthrough: cooling && !positive,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    timestep_count: usize,
    numerical_cooling_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let cooling = predecessor.cooling_body_entry_count;
    let first_matches = if binding.system.cooling_limit == IdealLoadsLimit::LimitFlowRate {
        cooling
    } else {
        0
    };
    let second_reads = cooling - first_matches;
    let combined_matches =
        if binding.system.cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity {
            cooling
        } else {
            0
        };
    let selected = first_matches + combined_matches;
    let rejected = cooling - selected;
    let positive = if latest_output
        .initialization
        .maximum_cooling_air_mass_flow_rate_kg_per_s
        > 0.0
    {
        selected
    } else {
        0
    };
    let not_positive = selected - positive;
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
        state.supply_mass_flow_limit_body_entry_count,
        state.active_guard_false_fallthrough_count,
        "active_partition_overflow",
        cooling,
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
            cooling,
            state.cooling_body_entry_count,
        ),
        (
            "numerical_cooling_count",
            numerical_cooling_count,
            state.cooling_body_entry_count,
        ),
        (
            "first_cooling_limit_read_count",
            cooling,
            state.first_cooling_limit_read_count,
        ),
        (
            "cooling_limit_flow_rate_comparison_count",
            cooling,
            state.cooling_limit_flow_rate_comparison_count,
        ),
        (
            "cooling_limit_flow_rate_match_count",
            first_matches,
            state.cooling_limit_flow_rate_match_count,
        ),
        (
            "second_cooling_limit_read_count",
            second_reads,
            state.second_cooling_limit_read_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            second_reads,
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_match_count",
            combined_matches,
            state.cooling_limit_flow_rate_and_capacity_match_count,
        ),
        (
            "cooling_limit_rejected_count",
            rejected,
            state.cooling_limit_rejected_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_read_count",
            selected,
            state.maximum_cooling_air_mass_flow_rate_read_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_positive_comparison_count",
            selected,
            state.maximum_cooling_air_mass_flow_rate_positive_comparison_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_strictly_positive_count",
            positive,
            state.maximum_cooling_air_mass_flow_rate_strictly_positive_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_not_positive_count",
            not_positive,
            state.maximum_cooling_air_mass_flow_rate_not_positive_count,
        ),
        (
            "supply_mass_flow_limit_body_entry_count",
            positive,
            state.supply_mass_flow_limit_body_entry_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            cooling - positive,
            state.active_guard_false_fallthrough_count,
        ),
        ("active_partition", cooling, active_partition),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(
            latest,
            &latest_output.calculation_cooling_supply_mass_flow_limit_guard,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    right: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) -> bool {
    let values_match = match (
        left.maximum_cooling_air_mass_flow_rate_kg_per_s,
        right.maximum_cooling_air_mass_flow_rate_kg_per_s,
    ) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    };
    let mut left_without_value = *left;
    let mut right_without_value = *right;
    left_without_value.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
    right_without_value.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
    values_match && left_without_value == right_without_value
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
    Error::CalcCoolingSupplyMassFlowLimitGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

    use super::*;

    fn active_zero_snapshot(
        maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
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
            first_cooling_limit_read: true,
            first_cooling_limit: Some(IdealLoadsLimit::LimitFlowRate),
            cooling_limit_flow_rate_comparison_evaluated: true,
            cooling_limit_flow_rate: Some(true),
            second_cooling_limit_read: false,
            second_cooling_limit: None,
            cooling_limit_flow_rate_and_capacity_comparison_evaluated: false,
            cooling_limit_flow_rate_and_capacity: None,
            cooling_limit_condition_satisfied: Some(true),
            maximum_cooling_air_mass_flow_rate_read: true,
            maximum_cooling_air_mass_flow_rate_kg_per_s: Some(
                maximum_cooling_air_mass_flow_rate_kg_per_s,
            ),
            maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: true,
            maximum_cooling_air_mass_flow_rate_strictly_positive: Some(false),
            supply_mass_flow_limit_body_entered: false,
            active_guard_false_fallthrough: true,
        }
    }

    #[test]
    fn latest_snapshot_signed_zero_corruption_fails_closed() {
        let positive_zero = active_zero_snapshot(0.0);
        let negative_zero = active_zero_snapshot(-0.0);

        assert_eq!(positive_zero, negative_zero);
        assert_ne!(
            positive_zero
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .expect("positive zero")
                .to_bits(),
            negative_zero
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .expect("negative zero")
                .to_bits()
        );
        assert!(snapshots_match_exact_bits(&positive_zero, &positive_zero));
        assert!(!snapshots_match_exact_bits(&positive_zero, &negative_zero));
    }
}
