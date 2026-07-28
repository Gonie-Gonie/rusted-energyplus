//! Release validation for the bounded cooling supply mass-flow limit body.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    IdealLoadsSensibleMode,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};

use super::super::calc::cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_mass_flow_limit_guard;
    let source_supply = output
        .calculation_cooling_supply_mass_flow_maximum
        .resulting_supply_mass_flow_rate_kg_per_s;
    let initialized_maximum = output
        .initialization
        .maximum_cooling_air_mass_flow_rate_kg_per_s;
    let snapshot = output.calculation_cooling_supply_mass_flow_limit_body;
    let numerical_cooling =
        output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling;

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && predecessor.cooling_body_entered == numerical_cooling
        && (!predecessor.supply_mass_flow_limit_body_entered || initialized_maximum > 0.0)
        && cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release(snapshot)
        && snapshots_match_exact_bits(
            &snapshot,
            &expected_snapshot(
                predecessor,
                source_supply,
                initialized_maximum,
                call_ordinal,
                binding,
            ),
        )
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    source_supply: Option<f64>,
    initialized_maximum: f64,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
    let cooling = predecessor.cooling_body_entered;
    let body_entered = predecessor.supply_mass_flow_limit_body_entered;
    let supply_before = source_supply.filter(|_| cooling);
    let maximum = body_entered.then_some(initialized_maximum);
    let minimum = supply_before
        .zip(maximum)
        .map(|(supply, maximum)| source_min(supply, maximum));
    let resulting = supply_before.map(|supply| minimum.unwrap_or(supply));

    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
        system: binding.ideal_loads_air_system,
        parent_call_ordinal: call_ordinal,
        controlled_zone: binding.zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_ems_supply_mass_flow_override_body_entered: predecessor
            .predecessor_ems_supply_mass_flow_override_body_entered,
        predecessor_ems_supply_mass_flow_override_body_skipped: predecessor
            .predecessor_ems_supply_mass_flow_override_body_skipped,
        predecessor_ems_disabled_fallthrough: predecessor.predecessor_ems_disabled_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        supply_mass_flow_limit_body_entered: body_entered,
        body_skipped: !body_entered,
        active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        supply_mass_flow_rate_for_minimum_read: body_entered,
        supply_mass_flow_rate_before_limit_kg_per_s: if body_entered {
            supply_before
        } else {
            None
        },
        maximum_cooling_air_mass_flow_rate_for_minimum_read: body_entered,
        maximum_cooling_air_mass_flow_rate_kg_per_s: maximum,
        source_shaped_two_argument_minimum_evaluated: body_entered,
        minimum_supply_mass_flow_rate_kg_per_s: minimum,
        supply_mass_flow_rate_assignment_performed: body_entered,
        assigned_supply_mass_flow_rate_kg_per_s: minimum,
        resulting_supply_mass_flow_rate_kg_per_s: resulting,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    timestep_count: usize,
    numerical_cooling_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let body_entries = predecessor.supply_mass_flow_limit_body_entry_count;
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
    let body_partition = checked_add(
        state.supply_mass_flow_limit_body_entry_count,
        state.body_skip_count,
        "body_partition_overflow",
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
        ("body_partition", state.transition_count, body_partition),
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
            "numerical_cooling_count",
            numerical_cooling_count,
            state.cooling_body_entry_count,
        ),
        (
            "supply_mass_flow_limit_body_entry_count",
            body_entries,
            state.supply_mass_flow_limit_body_entry_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            predecessor.active_guard_false_fallthrough_count,
            state.active_guard_false_fallthrough_count,
        ),
        (
            "supply_mass_flow_rate_for_minimum_read_count",
            body_entries,
            state.supply_mass_flow_rate_for_minimum_read_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_for_minimum_read_count",
            body_entries,
            state.maximum_cooling_air_mass_flow_rate_for_minimum_read_count,
        ),
        (
            "source_shaped_two_argument_minimum_evaluation_count",
            body_entries,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "supply_mass_flow_rate_assignment_count",
            body_entries,
            state.supply_mass_flow_rate_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(
            latest,
            &latest_output.calculation_cooling_supply_mass_flow_limit_body,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    right: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    let values_match = [
        (
            left.supply_mass_flow_rate_before_limit_kg_per_s,
            right.supply_mass_flow_rate_before_limit_kg_per_s,
        ),
        (
            left.maximum_cooling_air_mass_flow_rate_kg_per_s,
            right.maximum_cooling_air_mass_flow_rate_kg_per_s,
        ),
        (
            left.minimum_supply_mass_flow_rate_kg_per_s,
            right.minimum_supply_mass_flow_rate_kg_per_s,
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
        snapshot.supply_mass_flow_rate_before_limit_kg_per_s = None;
        snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
        snapshot.minimum_supply_mass_flow_rate_kg_per_s = None;
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

fn source_min(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
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
    Error::CalcCoolingSupplyMassFlowLimitBodyLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};

    use super::*;

    fn snapshot_with_result(result: f64) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
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
    fn latest_result_signed_zero_corruption_fails_closed() {
        let positive_zero = snapshot_with_result(0.0);
        let negative_zero = snapshot_with_result(-0.0);

        assert_eq!(positive_zero, negative_zero);
        assert!(snapshots_match_exact_bits(&positive_zero, &positive_zero));
        assert!(!snapshots_match_exact_bits(&positive_zero, &negative_zero));
    }
}
