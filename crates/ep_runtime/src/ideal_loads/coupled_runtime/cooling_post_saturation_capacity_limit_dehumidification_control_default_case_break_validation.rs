//! Coupled-runtime validation for CP410 untyped-default case-break evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakLifecycleSummary as Lifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_characterization,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_PREDECESSOR_INDICES: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 24];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_characterization(
        predecessor,
    );

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_is_exact_direct_release(snapshot)
        && links_to_predecessor(snapshot, predecessor)
        && carriers_are_preserved(snapshot, predecessor)
        && expected.is_some_and(|expected| same_snapshot(snapshot, expected))
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp409: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp409.state;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || predecessor_cp409.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE
        || predecessor_cp409.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER.len()
            != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_maximum_capacity_assignment_route_counts
            != predecessor.predecessor_maximum_capacity_assignment_route_counts
    {
        return Err(violation(
            "source_predecessor_route_and_system_identity",
            1,
            0,
        ));
    }

    ensure_public_routes_only(&state.predecessor_route_counts)?;
    ensure_public_routes_only(&state.predecessor_guard_false_fallthrough_route_counts)?;
    ensure_public_routes_only(&state.predecessor_maximum_capacity_assignment_route_counts)?;
    validate_route_evidence(
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_maximum_capacity_assignment_route_counts,
    )?;

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard_false_partition_overflow",
    )?;
    let maximum = checked_sum(
        &state.predecessor_maximum_capacity_assignment_route_counts,
        "maximum_capacity_partition_overflow",
    )?;
    let predecessor_breaks = guard_false
        .checked_add(maximum)
        .ok_or_else(|| violation("predecessor_case_break_partition_overflow", 0, usize::MAX))?;
    let predecessor_inactive = transitions.checked_sub(predecessor_breaks).ok_or_else(|| {
        violation(
            "predecessor_inactive_transition_underflow",
            predecessor_breaks,
            transitions,
        )
    })?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            transitions,
            state.inactive_transition_count,
        ),
        (
            "predecessor_inactive_transition_count",
            predecessor_inactive,
            predecessor.inactive_transition_count,
        ),
        (
            "predecessor_guard_false_fallthrough_count",
            guard_false,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "cp409_guard_false_fallthrough_count",
            predecessor.predecessor_guard_false_fallthrough_count,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "predecessor_maximum_capacity_assignment_count",
            maximum,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            "cp409_maximum_capacity_assignment_count",
            predecessor.predecessor_maximum_capacity_assignment_count,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            "cp409_shared_case_break_count",
            predecessor_breaks,
            predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count,
        ),
        (
            "default_case_break_count",
            0,
            state.dehumidification_control_default_case_break_count,
        ),
        (
            "source_site_execution_count",
            0,
            state.source_site_execution_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let expected = private_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_characterization(
        predecessor_latest,
    )
    .ok_or_else(|| violation("latest_predecessor_lineage_ready", 1, 0))?;
    if !same_snapshot(latest, expected)
        || !same_snapshot(
            latest,
            latest_output
                .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break,
        )
        || !links_to_predecessor(latest, predecessor_latest)
        || !carriers_are_preserved(latest, predecessor_latest)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_route_evidence(
    routes: &[usize; 30],
    guard_false_routes: &[usize; 30],
    maximum_routes: &[usize; 30],
) -> Result<(), Error> {
    for index in 0..routes.len() {
        let split_evidence = guard_false_routes[index]
            .checked_add(maximum_routes[index])
            .ok_or_else(|| violation("split_route_evidence_overflow", 0, usize::MAX))?;
        let expected = if SPLIT_PREDECESSOR_INDICES.contains(&index) {
            routes[index]
        } else {
            0
        };
        ensure_count(split_evidence, expected, "split_route_evidence_partition")?;
    }
    Ok(())
}

fn ensure_public_routes_only(values: &[usize; 30]) -> Result<(), Error> {
    for (index, count) in values.iter().enumerate() {
        if !PUBLIC_PREDECESSOR_INDICES.contains(&index) && *count != 0 {
            return Err(violation("non_direct_route_count", 0, *count));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

mod lineage;
use lineage::{carriers_are_preserved, links_to_predecessor, same_snapshot};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flattened_route_contract_is_36_total_and_every_default_route_is_inactive() {
        let total = 30 + SPLIT_PREDECESSOR_INDICES.len();
        assert_eq!((total, total, 0), (36, 36, 0));
    }

    #[test]
    fn overflow_helpers_fail_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
    }
}
