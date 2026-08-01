//! CP384 cumulative counter and preserved 23-route validation.

use super::*;

pub(super) fn validate(
    state: &State,
    predecessor: &PredecessorState,
    calls: usize,
) -> Result<(), String> {
    let routes = route_counts(state);
    if routes != predecessor_route_counts(predecessor) {
        return Err("direct-zone IdealLoads CP384 predecessor route counters are invalid".into());
    }
    if routes[5..].iter().any(|count| *count != 0) {
        return Err(
            "direct-zone IdealLoads CP384 private direct-route counters are nonzero".into(),
        );
    }

    let lineage = lineage_counts(state);
    let predecessor_lineage = predecessor_lineage_counts(predecessor);
    for index in 0..5 {
        for field_index in 0..4 {
            ensure(
                lineage[index][field_index],
                predecessor_lineage[index][field_index],
                "inherited lineage counter",
            )?;
        }
        ensure(
            lineage[index][4],
            predecessor_lineage[index][4],
            "maximum-capacity assignment route",
        )?;
        ensure(
            checked_add(
                lineage[index][3],
                lineage[index][4],
                "guard outcome partition",
            )?,
            lineage[index][2],
            "guard outcome partition",
        )?;
        ensure(
            checked_sum(
                &[
                    lineage[index][0],
                    lineage[index][1],
                    lineage[index][3],
                    lineage[index][4],
                ],
                "base route partition",
            )?,
            routes[index + 3],
            "base route partition",
        )?;
    }

    let guard_false = checked_sum(&lineage.map(|counts| counts[3]), "guard-false route sum")?;
    let assignments = checked_sum(&lineage.map(|counts| counts[4]), "assignment route sum")?;
    let evaluations = checked_add(guard_false, assignments, "guard evaluation sum")?;
    ensure(
        checked_sum(&refined_route_counts(state), "transition partition")?,
        state.transition_count,
        "transition partition",
    )?;
    let source_sites = checked_mul(assignments, 2, "source sites")?;

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
            evaluations,
        ),
        (
            "predecessor_guard_evaluation_count",
            state.dehumidification_total_output_capacity_guard_evaluation_count,
            predecessor.dehumidification_total_output_capacity_guard_evaluation_count,
        ),
        (
            "dehumidification_total_output_capacity_guard_false_fallthrough_count",
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
            guard_false,
        ),
        (
            "predecessor_guard_false_count",
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
            predecessor.dehumidification_total_output_capacity_guard_false_fallthrough_count,
        ),
        (
            "dehumidification_total_output_maximum_capacity_assignment_count",
            state.dehumidification_total_output_maximum_capacity_assignment_count,
            assignments,
        ),
        (
            "predecessor_body_entry_count",
            state.dehumidification_total_output_maximum_capacity_assignment_count,
            predecessor.dehumidification_total_output_capacity_adjustment_body_entry_count,
        ),
        (
            "source_site_execution_count",
            state.source_site_execution_count,
            source_sites,
        ),
        (
            "cp383_retained_maximum_total_cooling_capacity_owned_read_count",
            state.cp383_retained_maximum_total_cooling_capacity_owned_read_count,
            assignments,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            state.maximum_total_cooling_capacity_read_count,
            assignments,
        ),
        (
            "cooling_total_output_assignment_write_count",
            state.cooling_total_output_assignment_write_count,
            assignments,
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
    refined_route_counts(state)[index] > 0
        && predecessor_refined_route_counts(predecessor)[index] > 0
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

fn lineage_counts(state: &State) -> [[usize; 5]; 5] {
    [
        [state.heating_availability_guard_false_fallthrough_capacity_guard_false_count, state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count],
        [state.humidification_control_guard_false_fallthrough_capacity_guard_false_count, state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count],
        [state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count],
        [state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count, state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count],
        [state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count],
    ]
}

fn predecessor_lineage_counts(state: &PredecessorState) -> [[usize; 5]; 5] {
    [
        [state.heating_availability_guard_false_fallthrough_capacity_guard_false_count, state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count],
        [state.humidification_control_guard_false_fallthrough_capacity_guard_false_count, state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count],
        [state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count],
        [state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count, state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count],
        [state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count, state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count],
    ]
}

fn refined_route_counts(state: &State) -> [usize; 23] {
    let lineage = lineage_counts(state);
    let mut refined = [0; 23];
    refined[..3].copy_from_slice(&route_counts(state)[..3]);
    for (index, counts) in lineage.into_iter().enumerate() {
        let offset = 3 + 4 * index;
        refined[offset] = counts[0];
        refined[offset + 1] = counts[1];
        refined[offset + 2] = counts[3];
        refined[offset + 3] = counts[4];
    }
    refined
}

fn predecessor_refined_route_counts(state: &PredecessorState) -> [usize; 23] {
    let lineage = predecessor_lineage_counts(state);
    let mut refined = [0; 23];
    refined[..3].copy_from_slice(&predecessor_route_counts(state)[..3]);
    for (index, counts) in lineage.into_iter().enumerate() {
        let offset = 3 + 4 * index;
        refined[offset] = counts[0];
        refined[offset + 1] = counts[1];
        refined[offset + 2] = counts[3];
        refined[offset + 3] = counts[4];
    }
    refined
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
    } else if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        1
    } else if snapshot.dehumidification_total_output_capacity_guard_false_fallthrough {
        2
    } else if snapshot.dehumidification_total_output_maximum_capacity_assignment_executed {
        3
    } else {
        return None;
    };
    Some(3 + 4 * (base_index - 3) + successor)
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP384 {field} overflow"))
    })
}

fn checked_add(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP384 {field} overflow"))
}

fn checked_mul(left: usize, right: usize, field: &str) -> Result<usize, String> {
    left.checked_mul(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP384 {field} overflow"))
}

fn ensure(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP384 {field} is invalid: expected {expected}, got {actual}"
        ))
    }
}
