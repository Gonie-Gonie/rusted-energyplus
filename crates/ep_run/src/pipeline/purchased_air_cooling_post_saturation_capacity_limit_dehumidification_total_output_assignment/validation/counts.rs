use super::*;

pub(super) fn validate(
    state: &State,
    predecessor: &PredecessorState,
    calls: usize,
) -> Result<(), String> {
    let routes = route_counts(state);
    if routes != predecessor_route_counts(predecessor) {
        return Err("direct-zone IdealLoads CP382 predecessor route counters are invalid".into());
    }
    if routes[5..].iter().any(|count| *count != 0) {
        return Err(
            "direct-zone IdealLoads CP382 private direct-route counters are nonzero".into(),
        );
    }

    let capacity_false = capacity_false_counts(state);
    let predecessor_capacity = predecessor_capacity_partitions(predecessor);
    let dehumidification = dehumidification_partitions(state);
    let predecessor_dehumidification = predecessor_dehumidification_partitions(predecessor);
    let assignments = assignment_counts(state);
    for index in 0..5 {
        ensure(
            capacity_false[index],
            predecessor_capacity[index].1,
            "capacity false route",
        )?;
        ensure(
            dehumidification[index].0,
            predecessor_dehumidification[index].0,
            "dehumidification body route",
        )?;
        ensure(
            dehumidification[index].1,
            predecessor_dehumidification[index].1,
            "dehumidification false route",
        )?;
        ensure(
            assignments[index],
            predecessor_dehumidification[index].0,
            "assignment route",
        )?;
        ensure(
            checked_add(
                checked_add(
                    capacity_false[index],
                    assignments[index],
                    "refined base route partition",
                )?,
                dehumidification[index].1,
                "refined base route partition",
            )?,
            routes[index + 3],
            "refined base route partition",
        )?;
        ensure(
            checked_add(
                predecessor_dehumidification[index].0,
                predecessor_dehumidification[index].1,
                "predecessor dehumidification partition",
            )?,
            predecessor_capacity[index].0,
            "predecessor dehumidification partition",
        )?;
    }

    let assigned = checked_sum(&assignments, "assignment route sum")?;
    ensure(
        assigned,
        state.dehumidification_total_output_assignment_count,
        "assignment route sum",
    )?;
    ensure(
        assigned,
        predecessor.dehumidification_body_entry_count,
        "predecessor dehumidification body entries",
    )?;
    let refined = refined_route_counts(state);
    ensure(
        checked_sum(&refined, "transition partition")?,
        state.transition_count,
        "transition partition",
    )?;
    let source_sites = checked_mul(assigned, 6, "source sites")?;

    for (field, actual, expected) in [
        ("transition_count", state.transition_count, calls),
        (
            "predecessor_transition_count",
            state.transition_count,
            predecessor.transition_count,
        ),
        (
            "source_site_execution_count",
            state.source_site_execution_count,
            source_sites,
        ),
        (
            "cp330_supply_mass_flow_rate_owned_read_count",
            state.cp330_supply_mass_flow_rate_owned_read_count,
            assigned,
        ),
        (
            "cp329_same_call_supply_mass_flow_rate_bit_corroboration_count",
            state.cp329_same_call_supply_mass_flow_rate_bit_corroboration_count,
            assigned,
        ),
        (
            "cp339_same_call_supply_mass_flow_rate_bit_corroboration_count",
            state.cp339_same_call_supply_mass_flow_rate_bit_corroboration_count,
            assigned,
        ),
        (
            "supply_mass_flow_rate_read_count",
            state.supply_mass_flow_rate_read_count,
            assigned,
        ),
        (
            "cp329_mixed_air_enthalpy_owned_read_count",
            state.cp329_mixed_air_enthalpy_owned_read_count,
            assigned,
        ),
        (
            "cp329_same_call_recirculation_enthalpy_bit_corroboration_count",
            state.cp329_same_call_recirculation_enthalpy_bit_corroboration_count,
            assigned,
        ),
        (
            "cp339_same_call_mixed_air_enthalpy_bit_corroboration_count",
            state.cp339_same_call_mixed_air_enthalpy_bit_corroboration_count,
            assigned,
        ),
        (
            "mixed_air_enthalpy_read_count",
            state.mixed_air_enthalpy_read_count,
            assigned,
        ),
        (
            "cp379_post_saturation_supply_enthalpy_owned_read_count",
            state.cp379_post_saturation_supply_enthalpy_owned_read_count,
            assigned,
        ),
        (
            "cp379_same_call_supply_enthalpy_bits_corroboration_count",
            state.cp379_same_call_supply_enthalpy_bits_corroboration_count,
            assigned,
        ),
        (
            "supply_enthalpy_read_count",
            state.supply_enthalpy_read_count,
            assigned,
        ),
        (
            "enthalpy_difference_calculation_count",
            state.enthalpy_difference_calculation_count,
            assigned,
        ),
        (
            "cooling_total_output_calculation_count",
            state.cooling_total_output_calculation_count,
            assigned,
        ),
        (
            "cooling_total_output_assignment_write_count",
            state.cooling_total_output_assignment_write_count,
            assigned,
        ),
    ] {
        ensure(actual, expected, field)?;
    }
    Ok(())
}

pub(super) fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    latest: Snapshot,
) -> bool {
    let Some(index) = refined_route_index(latest) else {
        return false;
    };
    refined_route_counts(state)[index] > 0 && predecessor_refined_routes(predecessor)[index] > 0
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

fn capacity_false_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
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
        (state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count, state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count),
        (state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count, state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count),
        (state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count),
        (state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count, state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count),
        (state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count),
    ]
}

fn predecessor_dehumidification_partitions(state: &PredecessorState) -> [(usize, usize); 5] {
    [
        (state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count, state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count),
        (state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count, state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count),
        (state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count),
        (state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count, state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count),
        (state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count),
    ]
}

fn assignment_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
    ]
}

fn refined_route_counts(state: &State) -> [usize; 18] {
    let capacity_false = capacity_false_counts(state);
    let assignments = assignment_counts(state);
    let dehumidification = dehumidification_partitions(state);
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        capacity_false[0],
        assignments[0],
        dehumidification[0].1,
        capacity_false[1],
        assignments[1],
        dehumidification[1].1,
        capacity_false[2],
        assignments[2],
        dehumidification[2].1,
        capacity_false[3],
        assignments[3],
        dehumidification[3].1,
        capacity_false[4],
        assignments[4],
        dehumidification[4].1,
    ]
}

fn predecessor_refined_routes(state: &PredecessorState) -> [usize; 18] {
    let capacity = predecessor_capacity_partitions(state);
    let dehumidification = predecessor_dehumidification_partitions(state);
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
    } else if snapshot.dehumidification_total_output_assignment_executed {
        1
    } else if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        2
    } else {
        return None;
    };
    Some(3 + 3 * (base_index - 3) + successor)
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP382 {field} overflow"))
    })
}

fn checked_add(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP382 {field} overflow"))
}

fn checked_mul(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_mul(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP382 {field} overflow"))
}

fn ensure(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP382 {field} is invalid: expected {expected}, got {actual}"
        ))
    }
}
