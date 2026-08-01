//! Coupled-runtime validation for CP375 humidification maximum-assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Snapshot,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit;
    let snapshot = output
        .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && snapshots_match_exact_bits(snapshot, expected_snapshot(predecessor))
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || lifecycle.state.system != binding.ideal_loads_air_system
        || predecessor.state.system != binding.ideal_loads_air_system
    {
        return Err(violation("source_and_system_identity", 1, 0));
    }
    validate_count_lineage(&lifecycle.state, &predecessor.state, timestep_count)?;
    validate_route_partition(&lifecycle.state)?;
    validate_source_counters(&lifecycle.state)?;

    let latest = lifecycle
        .state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if predecessor_latest
        != &latest_output
            .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit
        || !cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release(
            *predecessor_latest,
        )
        || !cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
            *latest,
        )
        || !snapshots_match_exact_bits(*latest, expected_snapshot(*predecessor_latest))
        || latest
            != &latest_output
                .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment
        || !snapshot_matches_release(latest_output, timestep_count, binding)
        || !latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            *predecessor_latest,
        )
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_count_lineage(
    state: &State,
    predecessor: &PredecessorState,
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
            predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count,
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count",
            predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count,
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "dehumidification_control_guard_false_fallthrough_count",
            predecessor.dehumidification_control_guard_false_fallthrough_count,
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
        (
            "purchased_air_supply_humidity_ratio_assignment_count",
            predecessor.supply_humidity_ratio_for_humidification_assignment_count,
            state.purchased_air_supply_humidity_ratio_assignment_count,
        ),
        (
            "direct_humidistat_maximum_limit_count",
            0,
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "direct_none_maximum_limit_count",
            0,
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "direct_dehumidification_guard_false_fallthrough_count",
            0,
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
        (
            "direct_assignment_count",
            0,
            state.purchased_air_supply_humidity_ratio_assignment_count,
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
    ensure_count(partition, state.transition_count, "transition_partition")?;
    let assignments = checked_sum(&[
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
    ])?;
    ensure_count(
        assignments,
        state.purchased_air_supply_humidity_ratio_assignment_count,
        "assignment_route_partition",
    )
}

fn validate_source_counters(state: &State) -> Result<(), Error> {
    let assignments = state.purchased_air_supply_humidity_ratio_assignment_count;
    let source_sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count",
            assignments,
            state.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count,
        ),
        (
            "supply_humidity_ratio_for_humidification_for_supply_maximum_read_count",
            assignments,
            state.supply_humidity_ratio_for_humidification_for_supply_maximum_read_count,
        ),
        (
            "source_shaped_two_argument_maximum_evaluation_count",
            assignments,
            state.source_shaped_two_argument_maximum_evaluation_count,
        ),
        (
            "purchased_air_supply_humidity_ratio_assignment_count",
            assignments,
            state.purchased_air_supply_humidity_ratio_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    latest: PredecessorSnapshot,
) -> bool {
    let pair = if latest.unit_off_skipped {
        (state.unit_off_skip_count, predecessor.unit_off_skip_count)
    } else if latest.non_cooling_skipped {
        (state.non_cooling_skip_count, predecessor.non_cooling_skip_count)
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
        .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed
    {
        (
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count,
        )
    } else if latest
        .dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed
    {
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count,
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

pub(in crate::ideal_loads) fn expected_snapshot(predecessor: PredecessorSnapshot) -> Snapshot {
    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor.predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: predecessor.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip: predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: predecessor.predecessor_heating_on_read,
        predecessor_heating_on: predecessor.predecessor_heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: predecessor.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough: predecessor.predecessor_heating_on_guard_false_fallthrough,
        predecessor_humidification_control_type_read: predecessor.predecessor_humidification_control_type_read,
        predecessor_humidification_control_type: predecessor.predecessor_humidification_control_type,
        predecessor_humidification_control_type_humidistat: predecessor.predecessor_humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered: predecessor.predecessor_humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough: predecessor.predecessor_humidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type_first_read: predecessor.predecessor_dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type: predecessor.predecessor_first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat: predecessor.predecessor_dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read: predecessor.predecessor_dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type: predecessor.predecessor_second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none: predecessor.predecessor_dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered: predecessor.predecessor_dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough: predecessor.predecessor_dehumidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed: predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed,
        predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed: predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed,
        predecessor_resulting_supply_humidity_ratio_for_humidification: predecessor.resulting_supply_humidity_ratio_for_humidification,
        dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed: false,
        dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed: false,
        purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read: false,
        purchased_air_supply_humidity_ratio_before_humidification_supply_maximum: None,
        supply_humidity_ratio_for_humidification_for_supply_maximum_read: false,
        supply_humidity_ratio_for_humidification_for_supply_maximum: None,
        source_shaped_two_argument_maximum_evaluated: false,
        maximum_supply_humidity_ratio: None,
        purchased_air_supply_humidity_ratio_assignment_performed: false,
        assigned_supply_humidity_ratio: None,
        resulting_supply_humidity_ratio: None,
    }
}

fn snapshots_match_exact_bits(left: Snapshot, right: Snapshot) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_supply_humidity_ratio_for_humidification,
            right.predecessor_resulting_supply_humidity_ratio_for_humidification,
        ),
        (
            left.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
            right.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
        ),
        (
            left.supply_humidity_ratio_for_humidification_for_supply_maximum,
            right.supply_humidity_ratio_for_humidification_for_supply_maximum,
        ),
        (
            left.maximum_supply_humidity_ratio,
            right.maximum_supply_humidity_ratio,
        ),
        (
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_equal(left, right));
    let mut left_without_values = left;
    let mut right_without_values = right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification = None;
        snapshot.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum = None;
        snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum = None;
        snapshot.maximum_supply_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left_without_values == right_without_values
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
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
    Error::CalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
