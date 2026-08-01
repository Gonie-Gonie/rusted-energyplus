use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn validate(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    supply_owner: &SupplyOwnerLifecycle,
    supply_corroborator: &SupplyCorroboratorLifecycle,
    mixed_air_owner: &MixedAirLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || supply_owner.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        || supply_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || supply_corroborator.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        || supply_corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || mixed_air_owner.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER.len()
            != 4
        || [
            lifecycle.state.system,
            predecessor.state.system,
            supply_owner.state.system,
            supply_corroborator.state.system,
            mixed_air_owner.state.system,
        ]
        .into_iter()
        .any(|system| system != binding.ideal_loads_air_system)
    {
        return Err(violation("source_owner_and_system_identity", 1, 0));
    }

    validate_counts(&lifecycle.state, &predecessor.state, timestep_count)?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let supply_owner_latest = supply_owner
        .state
        .latest
        .ok_or_else(|| violation("cp378_latest_owner_snapshot_ready", 1, 0))?;
    let supply_corroborator_latest = supply_corroborator
        .state
        .latest
        .ok_or_else(|| violation("cp379_latest_corroborator_snapshot_ready", 1, 0))?;
    let mixed_air_owner_latest = mixed_air_owner
        .state
        .latest
        .ok_or_else(|| violation("cp329_latest_owner_snapshot_ready", 1, 0))?;
    let expected = snapshot::expected_snapshot(
        predecessor_latest,
        supply_owner_latest,
        supply_corroborator_latest,
        mixed_air_owner_latest,
    )
    .ok_or_else(|| violation("latest_owner_lineage_ready", 1, 0))?;

    if !snapshot::snapshots_match_exact_bits(latest, expected)
        || !snapshot::snapshots_match_exact_bits(
            latest,
            latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard,
        )
        || !snapshot::matches_release(latest_output, timestep_count, binding)
        || !latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            &supply_owner.state,
            &supply_corroborator.state,
            &mixed_air_owner.state,
            latest,
        )
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &PredecessorState,
    timestep_count: usize,
) -> Result<(), Error> {
    let routes = route_counts(state);
    let predecessor_routes = predecessor_route_counts(predecessor);
    if routes != predecessor_routes {
        return Err(violation("predecessor_route_counters", 1, 0));
    }

    let capacity_partitions = capacity_partitions(state);
    let predecessor_capacity_partitions = predecessor_capacity_partitions(predecessor);
    if capacity_partitions != predecessor_capacity_partitions {
        return Err(violation("predecessor_capacity_route_partitions", 1, 0));
    }
    let active_routes = &routes[3..];
    for (&route, &(body, rejected)) in active_routes.iter().zip(capacity_partitions.iter()) {
        ensure_count(
            checked_add(body, rejected, "capacity_route_partition_overflow")?,
            route,
            "capacity_route_partition",
        )?;
    }

    let dehumidification_partitions = dehumidification_partitions(state);
    for (&(body, _), &(dehumidifying, false_fallthrough)) in capacity_partitions
        .iter()
        .zip(dehumidification_partitions.iter())
    {
        ensure_count(
            checked_add(
                dehumidifying,
                false_fallthrough,
                "dehumidification_route_partition_overflow",
            )?,
            body,
            "dehumidification_route_partition",
        )?;
    }

    let active = predecessor.capacity_limit_body_entry_count;
    let matches = state.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count;
    let false_fallthroughs = checked_sub(
        active,
        matches,
        "dehumidification_false_partition_underflow",
    )?;
    let sites = checked_add(
        checked_mul(active, 3, "operand_and_comparison_source_sites_overflow")?,
        matches,
        "source_site_execution_count_overflow",
    )?;
    let refined_routes = refined_route_counts(state);
    let transition_partition = checked_sum(&refined_routes, "transition_partition_overflow")?;

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
            "dehumidification_guard_evaluation_count",
            active,
            state.dehumidification_guard_evaluation_count,
        ),
        (
            "cp378_supply_humidity_ratio_saturation_limit_owned_read_count",
            active,
            state.cp378_supply_humidity_ratio_saturation_limit_owned_read_count,
        ),
        (
            "cp379_same_call_supply_humidity_ratio_bit_corroboration_count",
            active,
            state.cp379_same_call_supply_humidity_ratio_bit_corroboration_count,
        ),
        (
            "purchased_air_supply_humidity_ratio_read_count",
            active,
            state.purchased_air_supply_humidity_ratio_read_count,
        ),
        (
            "cp329_mixed_air_humidity_ratio_owned_read_count",
            active,
            state.cp329_mixed_air_humidity_ratio_owned_read_count,
        ),
        (
            "purchased_air_mixed_air_humidity_ratio_read_count",
            active,
            state.purchased_air_mixed_air_humidity_ratio_read_count,
        ),
        (
            "supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count",
            active,
            state.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count,
        ),
        (
            "dehumidification_body_entry_count",
            matches,
            state.dehumidification_body_entry_count,
        ),
        (
            "dehumidification_guard_false_fallthrough_count",
            false_fallthroughs,
            state.dehumidification_guard_false_fallthrough_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    supply_owner: &SupplyOwnerState,
    supply_corroborator: &SupplyCorroboratorState,
    mixed_air_owner: &MixedAirState,
    latest: Snapshot,
) -> bool {
    let Some(index) = refined_route_index(latest) else {
        return false;
    };
    let base_index = if index < 3 {
        index
    } else {
        3 + (index - 3) / 3
    };
    refined_route_counts(state)[index] > 0
        && predecessor_route_counts(predecessor)[base_index] > 0
        && supply_owner_route_counts(supply_owner)[base_index] > 0
        && supply_corroborator_route_counts(supply_corroborator)[base_index] > 0
        && (base_index < 2 || mixed_air_owner.no_outdoor_air_fallback_count > 0)
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

fn supply_owner_route_counts(state: &SupplyOwnerState) -> [usize; 8] {
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

fn supply_corroborator_route_counts(state: &SupplyCorroboratorState) -> [usize; 8] {
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

fn capacity_partitions(state: &State) -> [(usize, usize); 5] {
    [
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
    ]
}

fn predecessor_capacity_partitions(state: &PredecessorState) -> [(usize, usize); 5] {
    [
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
    ]
}

fn dehumidification_partitions(state: &State) -> [(usize, usize); 5] {
    [
        (
            state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
            state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        ),
        (
            state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        ),
        (
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        ),
        (
            state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        ),
    ]
}

fn refined_route_counts(state: &State) -> [usize; 18] {
    let capacity = capacity_partitions(state);
    let dehumidification = dehumidification_partitions(state);
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        capacity[0].1,
        dehumidification[0].0,
        dehumidification[0].1,
        capacity[1].1,
        dehumidification[1].0,
        dehumidification[1].1,
        capacity[2].1,
        dehumidification[2].0,
        dehumidification[2].1,
        capacity[3].1,
        dehumidification[3].0,
        dehumidification[3].1,
        capacity[4].1,
        dehumidification[4].0,
        dehumidification[4].1,
    ]
}

fn refined_route_index(snapshot: Snapshot) -> Option<usize> {
    let base = [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ];
    if base.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    let base_index = base.into_iter().position(|flag| flag)?;
    if base_index < 3 {
        return Some(base_index);
    }
    let successor = if snapshot.predecessor_active_capacity_limit_guard_false_fallthrough {
        0
    } else if snapshot.dehumidification_body_entered {
        1
    } else if snapshot.dehumidification_guard_false_fallthrough {
        2
    } else {
        return None;
    };
    Some(3 + 3 * (base_index - 3) + successor)
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
