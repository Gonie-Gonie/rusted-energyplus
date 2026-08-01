//! CP383 cumulative counter and 23-route partition validation.

use super::*;

pub(super) fn validate(
    state: &State,
    predecessor: &PredecessorState,
    calls: usize,
) -> Result<(), String> {
    let routes = route_counts(state);
    if routes != predecessor_route_counts(predecessor) {
        return Err("direct-zone IdealLoads CP383 predecessor route counters are invalid".into());
    }
    if routes[5..].iter().any(|count| *count != 0) {
        return Err(
            "direct-zone IdealLoads CP383 private direct-route counters are nonzero".into(),
        );
    }

    let inherited = inherited_lineage_counts(state);
    let predecessor_inherited = predecessor_lineage_counts(predecessor);
    let outcomes = guard_outcome_counts(state);
    for index in 0..5 {
        for field_index in 0..3 {
            ensure(
                inherited[index][field_index],
                predecessor_inherited[index][field_index],
                "inherited lineage route",
            )?;
        }
        ensure(
            checked_add(
                outcomes[index].0,
                outcomes[index].1,
                "guard outcome partition",
            )?,
            inherited[index][2],
            "guard outcome partition",
        )?;
        ensure(
            inherited[index][2],
            predecessor_dehumidification_body_counts(predecessor)[index],
            "assignment/body-entry equivalence",
        )?;
        ensure(
            checked_add(
                checked_add(
                    inherited[index][0],
                    inherited[index][2],
                    "base route partition",
                )?,
                inherited[index][1],
                "base route partition",
            )?,
            routes[index + 3],
            "base route partition",
        )?;
    }

    let assignments = checked_sum(
        &inherited.map(|lineage| lineage[2]),
        "guard evaluation route sum",
    )?;
    let bodies = checked_sum(&outcomes.map(|outcome| outcome.0), "guard body route sum")?;
    let guard_false = checked_sum(&outcomes.map(|outcome| outcome.1), "guard false route sum")?;
    ensure(
        checked_add(bodies, guard_false, "guard outcome sum")?,
        assignments,
        "guard outcome sum",
    )?;
    ensure(
        checked_sum(&refined_route_counts(state), "transition partition")?,
        state.transition_count,
        "transition partition",
    )?;
    let source_sites = checked_add(
        checked_mul(assignments, 3, "source sites")?,
        bodies,
        "source sites",
    )?;

    for (field, actual, expected) in [
        ("transition_count", state.transition_count, calls),
        (
            "predecessor_transition_count",
            state.transition_count,
            predecessor.transition_count,
        ),
        (
            "dehumidification_total_output_capacity_guard_evaluation_count",
            state.dehumidification_total_output_capacity_guard_evaluation_count,
            assignments,
        ),
        (
            "source_site_execution_count",
            state.source_site_execution_count,
            source_sites,
        ),
        (
            "cp382_cooling_total_output_owned_read_count",
            state.cp382_cooling_total_output_owned_read_count,
            assignments,
        ),
        (
            "cooling_total_output_read_count",
            state.cooling_total_output_read_count,
            assignments,
        ),
        (
            "cp321_maximum_total_cooling_capacity_owned_read_count",
            state.cp321_maximum_total_cooling_capacity_owned_read_count,
            assignments,
        ),
        (
            "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count",
            state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
            assignments,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            state.maximum_total_cooling_capacity_read_count,
            assignments,
        ),
        (
            "cooling_total_output_maximum_total_cooling_capacity_comparison_count",
            state.cooling_total_output_maximum_total_cooling_capacity_comparison_count,
            assignments,
        ),
        (
            "cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count",
            state.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count,
            bodies,
        ),
        (
            "dehumidification_total_output_capacity_adjustment_body_entry_count",
            state.dehumidification_total_output_capacity_adjustment_body_entry_count,
            bodies,
        ),
        (
            "dehumidification_total_output_capacity_guard_false_fallthrough_count",
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
            guard_false,
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
    let Some((current, predecessor_index)) = refined_route_indices(latest) else {
        return false;
    };
    refined_route_counts(state)[current] > 0
        && predecessor_refined_route_counts(predecessor)[predecessor_index] > 0
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

fn inherited_lineage_counts(state: &State) -> [[usize; 3]; 5] {
    [
        [state.heating_availability_guard_false_fallthrough_capacity_guard_false_count, state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count],
        [state.humidification_control_guard_false_fallthrough_capacity_guard_false_count, state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count],
        [state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count],
        [state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count, state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count],
        [state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count],
    ]
}

fn predecessor_lineage_counts(state: &PredecessorState) -> [[usize; 3]; 5] {
    [
        [state.heating_availability_guard_false_fallthrough_capacity_guard_false_count, state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count],
        [state.humidification_control_guard_false_fallthrough_capacity_guard_false_count, state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count],
        [state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count],
        [state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count, state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count],
        [state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count],
    ]
}

fn predecessor_dehumidification_body_counts(state: &PredecessorState) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
    ]
}

fn guard_outcome_counts(state: &State) -> [(usize, usize); 5] {
    [
        (state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count),
        (state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count),
        (state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count),
        (state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count),
        (state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count),
    ]
}

fn refined_route_counts(state: &State) -> [usize; 23] {
    let inherited = inherited_lineage_counts(state);
    let outcomes = guard_outcome_counts(state);
    let mut refined = [0; 23];
    refined[..3].copy_from_slice(&route_counts(state)[..3]);
    for index in 0..5 {
        let offset = 3 + 4 * index;
        refined[offset] = inherited[index][0];
        refined[offset + 1] = inherited[index][1];
        refined[offset + 2] = outcomes[index].1;
        refined[offset + 3] = outcomes[index].0;
    }
    refined
}

fn predecessor_refined_route_counts(state: &PredecessorState) -> [usize; 18] {
    let inherited = predecessor_lineage_counts(state);
    let mut refined = [0; 18];
    refined[..3].copy_from_slice(&predecessor_route_counts(state)[..3]);
    for (index, lineage) in inherited.into_iter().enumerate() {
        let offset = 3 + 3 * index;
        refined[offset] = lineage[0];
        refined[offset + 1] = lineage[2];
        refined[offset + 2] = lineage[1];
    }
    refined
}

fn refined_route_indices(snapshot: Snapshot) -> Option<(usize, usize)> {
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
        return Some((base_index, base_index));
    }
    let lineage = base_index - 3;
    if snapshot.predecessor_active_capacity_limit_guard_false_fallthrough {
        return Some((3 + 4 * lineage, 3 + 3 * lineage));
    }
    if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        return Some((4 + 4 * lineage, 5 + 3 * lineage));
    }
    if snapshot.dehumidification_total_output_capacity_guard_false_fallthrough {
        return Some((5 + 4 * lineage, 4 + 3 * lineage));
    }
    if snapshot.dehumidification_total_output_capacity_adjustment_body_entered {
        return Some((6 + 4 * lineage, 4 + 3 * lineage));
    }
    None
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP383 {field} overflow"))
    })
}

fn checked_add(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP383 {field} overflow"))
}

fn checked_mul(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_mul(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP383 {field} overflow"))
}

fn ensure(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP383 {field} is invalid: expected {expected}, got {actual}"
        ))
    }
}
