use super::*;

pub(super) fn validate(
    state: &State,
    predecessor: &PredecessorState,
    supply_owner: &SupplyOwnerState,
    supply_corroborator: &SupplyCorroboratorState,
    _mixed_air_owner: &MixedAirState,
    calls: usize,
) -> Result<(), String> {
    let routes = route_counts(state);
    let predecessor_routes = predecessor_route_counts(predecessor);
    if routes != predecessor_routes
        || routes != supply_owner_route_counts(supply_owner)
        || routes != supply_corroborator_route_counts(supply_corroborator)
    {
        return Err("direct-zone IdealLoads CP381 predecessor route counters are invalid".into());
    }
    if routes[5..].iter().any(|count| *count != 0) {
        return Err(
            "direct-zone IdealLoads CP381 private direct-route counters are nonzero".into(),
        );
    }

    let capacity = capacity_partitions(state);
    if capacity != predecessor_capacity_partitions(predecessor) {
        return Err("direct-zone IdealLoads CP381 capacity-route partitions are invalid".into());
    }
    for (&route, &(body, rejected)) in routes[3..].iter().zip(capacity.iter()) {
        ensure(
            checked_add(body, rejected, "capacity route partition")?,
            route,
            "capacity route partition",
        )?;
    }

    let dehumidification = dehumidification_partitions(state);
    for (&(body, _), &(dehumidifying, false_fallthrough)) in
        capacity.iter().zip(dehumidification.iter())
    {
        ensure(
            checked_add(
                dehumidifying,
                false_fallthrough,
                "dehumidification route partition",
            )?,
            body,
            "dehumidification route partition",
        )?;
    }

    let active = predecessor.capacity_limit_body_entry_count;
    let matches = state.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count;
    let false_fallthroughs = checked_sub(active, matches, "dehumidification partition")?;
    let body_route_sum = checked_sum(
        &dehumidification.map(|partition| partition.0),
        "dehumidification body route sum",
    )?;
    let false_route_sum = checked_sum(
        &dehumidification.map(|partition| partition.1),
        "dehumidification false route sum",
    )?;
    ensure(body_route_sum, matches, "dehumidification body route sum")?;
    ensure(
        false_route_sum,
        false_fallthroughs,
        "dehumidification false route sum",
    )?;

    let sites = checked_add(
        checked_mul(active, 3, "CP381 source sites")?,
        matches,
        "CP381 source sites",
    )?;
    let refined = refined_route_counts(state);
    ensure(
        checked_sum(&refined, "CP381 transition partition")?,
        state.transition_count,
        "CP381 transition partition",
    )?;

    for (field, actual, expected) in [
        ("transition_count", state.transition_count, calls),
        (
            "predecessor_transition_count",
            state.transition_count,
            predecessor.transition_count,
        ),
        (
            "dehumidification_guard_evaluation_count",
            state.dehumidification_guard_evaluation_count,
            active,
        ),
        (
            "cp378_supply_humidity_ratio_saturation_limit_owned_read_count",
            state.cp378_supply_humidity_ratio_saturation_limit_owned_read_count,
            active,
        ),
        (
            "cp379_same_call_supply_humidity_ratio_bit_corroboration_count",
            state.cp379_same_call_supply_humidity_ratio_bit_corroboration_count,
            active,
        ),
        (
            "purchased_air_supply_humidity_ratio_read_count",
            state.purchased_air_supply_humidity_ratio_read_count,
            active,
        ),
        (
            "cp329_mixed_air_humidity_ratio_owned_read_count",
            state.cp329_mixed_air_humidity_ratio_owned_read_count,
            active,
        ),
        (
            "purchased_air_mixed_air_humidity_ratio_read_count",
            state.purchased_air_mixed_air_humidity_ratio_read_count,
            active,
        ),
        (
            "supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count",
            state.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count,
            active,
        ),
        (
            "dehumidification_body_entry_count",
            state.dehumidification_body_entry_count,
            matches,
        ),
        (
            "dehumidification_guard_false_fallthrough_count",
            state.dehumidification_guard_false_fallthrough_count,
            false_fallthroughs,
        ),
        (
            "source_site_execution_count",
            state.source_site_execution_count,
            sites,
        ),
    ] {
        ensure(actual, expected, field)?;
    }
    Ok(())
}

pub(super) fn latest_route_has_cumulative_evidence(
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

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP381 {field} overflow"))
    })
}

fn checked_add(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP381 {field} overflow"))
}

fn checked_sub(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_sub(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP381 {field} underflow"))
}

fn checked_mul(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_mul(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP381 {field} overflow"))
}

fn ensure(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP381 {field} is invalid: expected {expected}, got {actual}"
        ))
    }
}
