use super::*;

pub(super) fn validate(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER.len()
            != 5
        || lifecycle.state.system != binding.ideal_loads_air_system
        || predecessor.state.system != binding.ideal_loads_air_system
    {
        return Err(violation("source_and_system_identity", 1, 0));
    }
    validate_counts(
        &lifecycle.state,
        &predecessor.state,
        timestep_count,
        binding.system.cooling_limit,
    )?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    if latest != snapshot::expected_snapshot(predecessor_latest, binding.system.cooling_limit)
        || latest != latest_output.calculation_cooling_post_saturation_capacity_limit_guard
        || !snapshot::matches_release(latest_output, timestep_count, binding)
        || !latest_route_has_cumulative_evidence(&lifecycle.state, &predecessor.state, latest)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &PredecessorState,
    timestep_count: usize,
    cooling_limit: IdealLoadsLimit,
) -> Result<(), Error> {
    let routes = route_counts(state);
    let predecessor_routes = predecessor_route_counts(predecessor);
    if routes != predecessor_routes {
        return Err(violation("predecessor_route_counters", 1, 0));
    }
    let transition_partition = checked_sum(&routes, "transition_partition_overflow")?;
    let active = predecessor.local_supply_enthalpy_after_saturation_limit_assignment_count;
    let active_from_routes = checked_sum(&routes[3..], "active_route_partition_overflow")?;
    let capacity_matches = if cooling_limit == IdealLoadsLimit::LimitCapacity {
        active
    } else {
        0
    };
    let second_comparisons = checked_sub(
        active,
        capacity_matches,
        "second_comparison_partition_underflow",
    )?;
    let combined_matches = if cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity {
        active
    } else {
        0
    };
    let body_entries = checked_add(
        capacity_matches,
        combined_matches,
        "body_entry_partition_overflow",
    )?;
    let false_fallthroughs = checked_sub(active, body_entries, "active_false_partition_underflow")?;
    let source_sites = checked_add(
        checked_add(
            checked_mul(active, 2, "first_source_site_execution_count_overflow")?,
            checked_mul(
                second_comparisons,
                2,
                "second_source_site_execution_count_overflow",
            )?,
            "source_site_execution_count_overflow",
        )?,
        body_entries,
        "source_site_execution_count_overflow",
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
        ("active_route_partition", active, active_from_routes),
        (
            "capacity_limit_guard_evaluation_count",
            active,
            state.capacity_limit_guard_evaluation_count,
        ),
        (
            "configured_cooling_limit_owned_read_count",
            active,
            state.configured_cooling_limit_owned_read_count,
        ),
        (
            "cp337_same_call_selector_lineage_corroboration_count",
            active,
            state.cp337_same_call_selector_lineage_corroboration_count,
        ),
        (
            "first_cooling_limit_read_count",
            active,
            state.first_cooling_limit_read_count,
        ),
        (
            "cooling_limit_capacity_comparison_count",
            active,
            state.cooling_limit_capacity_comparison_count,
        ),
        (
            "cooling_limit_capacity_match_count",
            capacity_matches,
            state.cooling_limit_capacity_match_count,
        ),
        (
            "second_cooling_limit_read_count",
            second_comparisons,
            state.second_cooling_limit_read_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            second_comparisons,
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_match_count",
            combined_matches,
            state.cooling_limit_flow_rate_and_capacity_match_count,
        ),
        (
            "capacity_limit_body_entry_count",
            body_entries,
            state.capacity_limit_body_entry_count,
        ),
        (
            "cooling_limit_rejected_count",
            false_fallthroughs,
            state.cooling_limit_rejected_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            false_fallthroughs,
            state.active_guard_false_fallthrough_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_active_route_partitions(state, &routes[3..], body_entries > 0)
}

fn validate_active_route_partitions(
    state: &State,
    active_routes: &[usize],
    selected: bool,
) -> Result<(), Error> {
    let partitions = [
        (
            state.heating_availability_guard_false_fallthrough_body_entry_count,
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        ),
        (
            state.humidification_control_guard_false_fallthrough_body_entry_count,
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        ),
        (
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        ),
        (
            state.dehumidification_control_none_maximum_assignment_body_entry_count,
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_body_entry_count,
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        ),
    ];
    for (index, (&route, (body, rejected))) in active_routes.iter().zip(partitions).enumerate() {
        ensure_count(
            body,
            if selected { route } else { 0 },
            route_body_field(index),
        )?;
        ensure_count(
            rejected,
            if selected { 0 } else { route },
            route_false_field(index),
        )?;
    }
    Ok(())
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    latest: Snapshot,
) -> bool {
    let Some(index) = route_flags(latest).into_iter().position(|flag| flag) else {
        return false;
    };
    route_counts(state)[index] > 0 && predecessor_route_counts(predecessor)[index] > 0
}

fn route_counts(state: &State) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn predecessor_route_counts(state: &PredecessorState) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn route_flags(snapshot: Snapshot) -> [bool; 8] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ]
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
}

fn checked_add(left: usize, right: usize, field: &'static str) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, 0, usize::MAX))
}

fn checked_sub(left: usize, right: usize, field: &'static str) -> Result<usize, Error> {
    left.checked_sub(right)
        .ok_or_else(|| violation(field, 0, usize::MAX))
}

fn checked_mul(left: usize, right: usize, field: &'static str) -> Result<usize, Error> {
    left.checked_mul(right)
        .ok_or_else(|| violation(field, 0, usize::MAX))
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn route_body_field(index: usize) -> &'static str {
    [
        "heating_availability_route_body_count",
        "humidification_control_route_body_count",
        "dehumidification_humidistat_route_body_count",
        "dehumidification_none_route_body_count",
        "dehumidification_guard_false_route_body_count",
    ][index]
}

fn route_false_field(index: usize) -> &'static str {
    [
        "heating_availability_route_guard_false_count",
        "humidification_control_route_guard_false_count",
        "dehumidification_humidistat_route_guard_false_count",
        "dehumidification_none_route_guard_false_count",
        "dehumidification_guard_false_route_guard_false_count",
    ][index]
}
