//! Coupled-runtime validation for CP376 pre-saturation original-assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary as OwnerLifecycle,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot as OwnerSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;
    let owner = output
        .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    let snapshot =
        output.calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
    [snapshot.system, predecessor.system, owner.system]
        .into_iter()
        .all(|system| system == binding.ideal_loads_air_system)
        && [
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal,
            owner.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == call_ordinal)
        && [snapshot.controlled_zone, predecessor.controlled_zone, owner.controlled_zone]
            .into_iter()
            .all(|zone| zone == binding.zone)
        && cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release(
            owner,
        )
        && snapshot_links_to_direct_predecessor_and_owner(snapshot, predecessor, owner)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    owner: &OwnerLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || owner.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        || owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        || lifecycle.state.system != binding.ideal_loads_air_system
        || predecessor.state.system != binding.ideal_loads_air_system
        || owner.state.system != binding.ideal_loads_air_system
    {
        return Err(violation("source_and_system_identity", 1, 0));
    }
    validate_count_lineage(
        &lifecycle.state,
        &predecessor.state,
        owner
            .state
            .dehumidification_control_none_case_completion_count,
        timestep_count,
    )?;
    validate_route_partition(&lifecycle.state)?;
    validate_source_and_owner_counters(&lifecycle.state)?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let owner_latest = owner
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !predecessor_snapshots_match_exact_bits(
        predecessor_latest,
        latest_output
            .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
    ) || !owner_snapshots_match_exact_bits(
        owner_latest,
        latest_output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
    ) || !snapshots_match_exact_bits(
        latest,
        latest_output.calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
    )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
        || !latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            predecessor_latest,
        )
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_count_lineage(
    state: &State,
    predecessor: &PredecessorState,
    owner_completion_count: usize,
    timestep_count: usize,
) -> Result<(), Error> {
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
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
            "positive_guard_false_fallthrough_skip_count",
            predecessor.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "heating_availability_guard_false_fallthrough_count",
            predecessor.heating_availability_guard_false_fallthrough_count,
            state.heating_availability_guard_false_fallthrough_count,
        ),
        (
            "humidification_control_guard_false_fallthrough_count",
            predecessor.humidification_control_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_count,
        ),
        (
            "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count",
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count",
            predecessor
                .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "dehumidification_control_guard_false_fallthrough_count",
            predecessor.dehumidification_control_guard_false_fallthrough_count,
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
        (
            "direct_cp347_owner_completion_count",
            owner_completion_count,
            state.cp347_none_case_owner_count,
        ),
        (
            "direct_humidistat_maximum_assignment_count",
            0,
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "direct_none_maximum_assignment_count",
            0,
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "direct_dehumidification_guard_false_fallthrough_count",
            0,
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_partition(state: &State) -> Result<(), Error> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_and_owner_counters(state: &State) -> Result<(), Error> {
    let assignments = checked_sum(&[
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ])?;
    let owner_reads = checked_sum(&[
        state.cp375_maximum_assignment_owner_count,
        state.cp347_none_case_owner_count,
        state.cp356_constant_shr_owner_count,
        state.cp362_humidistat_owner_count,
        state.cp365_constant_supply_humidity_ratio_owner_count,
    ])?;
    let source_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("owner_read_partition", assignments, owner_reads),
        (
            "direct_cp375_owner_count",
            0,
            state.cp375_maximum_assignment_owner_count,
        ),
        (
            "direct_cp356_owner_count",
            0,
            state.cp356_constant_shr_owner_count,
        ),
        (
            "direct_cp362_owner_count",
            0,
            state.cp362_humidistat_owner_count,
        ),
        (
            "direct_cp365_owner_count",
            0,
            state.cp365_constant_supply_humidity_ratio_owner_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "purchased_air_supply_humidity_ratio_before_saturation_limit_read_count",
            assignments,
            state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count,
        ),
        (
            "local_original_supply_humidity_ratio_before_saturation_limit_assignment_count",
            assignments,
            state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn snapshot_links_to_direct_predecessor_and_owner(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    owner: crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    let routes_match = snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.heating_availability_guard_false_fallthrough
            == predecessor.predecessor_heating_on_guard_false_fallthrough
        && snapshot.humidification_control_guard_false_fallthrough
            == predecessor.predecessor_humidification_control_guard_false_fallthrough
        && snapshot.dehumidification_control_humidistat_maximum_assignment_executed
            == predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed
        && snapshot.dehumidification_control_none_maximum_assignment_executed
            == predecessor
                .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed
        && snapshot.dehumidification_control_guard_false_fallthrough
            == predecessor.predecessor_dehumidification_control_guard_false_fallthrough;
    let predecessor_matches = snapshot.predecessor_dehumidification_control_type
        == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_purchased_air_supply_humidity_ratio_assignment_performed
            == predecessor.purchased_air_supply_humidity_ratio_assignment_performed
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        );
    let active = !(snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped);
    let values_match = if active {
        let Some(owner_value) = owner.resulting_supply_humidity_ratio else {
            return false;
        };
        !snapshot.cp375_maximum_assignment_owned_read
            && snapshot.cp347_none_case_owned_read
            && !snapshot.cp356_constant_shr_owned_read
            && !snapshot.cp362_humidistat_owned_read
            && !snapshot.cp365_constant_supply_humidity_ratio_owned_read
            && snapshot.purchased_air_supply_humidity_ratio_read
            && snapshot.local_supply_humidity_ratio_original_assignment_performed
            && [
                snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
                snapshot.assigned_supply_humidity_ratio_original,
                snapshot.resulting_supply_humidity_ratio_original,
            ]
            .into_iter()
            .all(|value| option_bits_equal(value, Some(owner_value)))
    } else {
        !snapshot.cp375_maximum_assignment_owned_read
            && !snapshot.cp347_none_case_owned_read
            && !snapshot.cp356_constant_shr_owned_read
            && !snapshot.cp362_humidistat_owned_read
            && !snapshot.cp365_constant_supply_humidity_ratio_owned_read
            && !snapshot.purchased_air_supply_humidity_ratio_read
            && !snapshot.local_supply_humidity_ratio_original_assignment_performed
            && snapshot
                .purchased_air_supply_humidity_ratio_before_saturation_check
                .is_none()
            && snapshot.assigned_supply_humidity_ratio_original.is_none()
            && snapshot.resulting_supply_humidity_ratio_original.is_none()
    };
    routes_match && predecessor_matches && values_match
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    latest: PredecessorSnapshot,
) -> bool {
    let pair = if latest.unit_off_skipped {
        (state.unit_off_skip_count, predecessor.unit_off_skip_count)
    } else if latest.non_cooling_skipped {
        (
            state.non_cooling_skip_count,
            predecessor.non_cooling_skip_count,
        )
    } else if latest.positive_guard_false_fallthrough_skipped {
        (
            state.positive_guard_false_fallthrough_skip_count,
            predecessor.positive_guard_false_fallthrough_skip_count,
        )
    } else if latest.predecessor_heating_on_guard_false_fallthrough {
        (
            state.heating_availability_guard_false_fallthrough_count,
            predecessor.heating_availability_guard_false_fallthrough_count,
        )
    } else if latest.predecessor_humidification_control_guard_false_fallthrough {
        (
            state.humidification_control_guard_false_fallthrough_count,
            predecessor.humidification_control_guard_false_fallthrough_count,
        )
    } else if latest
        .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed
    {
        (
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        )
    } else if latest.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed
    {
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            predecessor
                .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        )
    } else if latest.predecessor_dehumidification_control_guard_false_fallthrough {
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            predecessor.dehumidification_control_guard_false_fallthrough_count,
        )
    } else {
        return false;
    };
    pair.0 > 0 && pair.1 > 0
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[rustfmt::skip]
pub(super) fn predecessor_snapshots_match_exact_bits(mut left: PredecessorSnapshot, mut right: PredecessorSnapshot) -> bool {
    let values_match = [
        (left.predecessor_resulting_supply_humidity_ratio_for_humidification, right.predecessor_resulting_supply_humidity_ratio_for_humidification),
        (left.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum, right.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum),
        (left.supply_humidity_ratio_for_humidification_for_supply_maximum, right.supply_humidity_ratio_for_humidification_for_supply_maximum),
        (left.maximum_supply_humidity_ratio, right.maximum_supply_humidity_ratio),
        (left.assigned_supply_humidity_ratio, right.assigned_supply_humidity_ratio),
        (left.resulting_supply_humidity_ratio, right.resulting_supply_humidity_ratio),
    ].into_iter().all(|(left, right)| option_bits_equal(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification = None; snapshot.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum = None;
        snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum = None; snapshot.maximum_supply_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None; snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

#[rustfmt::skip]
pub(super) fn owner_snapshots_match_exact_bits(mut left: OwnerSnapshot, mut right: OwnerSnapshot) -> bool {
    let values_match = [
        (left.predecessor_assigned_supply_humidity_ratio, right.predecessor_assigned_supply_humidity_ratio),
        (left.mixed_air_humidity_ratio, right.mixed_air_humidity_ratio),
        (left.assigned_supply_humidity_ratio, right.assigned_supply_humidity_ratio),
        (left.resulting_supply_humidity_ratio, right.resulting_supply_humidity_ratio),
    ].into_iter().all(|(left, right)| option_bits_equal(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_assigned_supply_humidity_ratio = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

#[rustfmt::skip]
pub(super) fn snapshots_match_exact_bits(mut left: Snapshot, mut right: Snapshot) -> bool {
    let values_match = [
        (left.predecessor_resulting_supply_humidity_ratio, right.predecessor_resulting_supply_humidity_ratio),
        (left.purchased_air_supply_humidity_ratio_before_saturation_check, right.purchased_air_supply_humidity_ratio_before_saturation_check),
        (left.assigned_supply_humidity_ratio_original, right.assigned_supply_humidity_ratio_original),
        (left.resulting_supply_humidity_ratio_original, right.resulting_supply_humidity_ratio_original),
    ].into_iter().all(|(left, right)| option_bits_equal(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio = None;
        snapshot.purchased_air_supply_humidity_ratio_before_saturation_check = None;
        snapshot.assigned_supply_humidity_ratio_original = None;
        snapshot.resulting_supply_humidity_ratio_original = None;
    }
    values_match && left == right
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("counter_partition_overflow", 0, usize::MAX))
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
    Error::CalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
