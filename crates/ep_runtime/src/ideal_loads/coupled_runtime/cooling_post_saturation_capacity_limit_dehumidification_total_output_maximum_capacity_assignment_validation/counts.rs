use super::routes::*;
use super::*;

pub(super) fn validate(
    state: &State,
    predecessor: &PredecessorState,
    timestep_count: usize,
) -> Result<(), Error> {
    if base_route_counts(state) != predecessor_base_route_counts(predecessor) {
        return Err(violation("predecessor_route_counters", 1, 0));
    }
    if inherited_lineage_route_counts(state) != predecessor_lineage_route_counts(predecessor) {
        return Err(violation("predecessor_lineage_route_counters", 1, 0));
    }
    if refined_route_counts(state) != predecessor_refined_route_counts(predecessor) {
        return Err(violation("predecessor_refined_route_counters", 1, 0));
    }
    if guard_false_route_counts(state) != predecessor_guard_false_route_counts(predecessor) {
        return Err(violation("predecessor_guard_false_route_counters", 1, 0));
    }
    if assignment_route_counts(state) != predecessor_body_route_counts(predecessor) {
        return Err(violation(
            "predecessor_body_assignment_route_counters",
            1,
            0,
        ));
    }

    let guard_false = checked_sum(
        &guard_false_route_counts(state),
        "guard_false_route_sum_overflow",
    )?;
    let assignments = checked_sum(
        &assignment_route_counts(state),
        "maximum_capacity_assignment_route_sum_overflow",
    )?;
    let evaluated = checked_sum(
        &[guard_false, assignments],
        "guard_evaluation_partition_overflow",
    )?;
    let source_sites = checked_mul(assignments, 2, "source_site_execution_count_overflow")?;
    let transition_partition = checked_sum(
        &refined_route_counts(state),
        "transition_partition_overflow",
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
        (
            "dehumidification_total_output_capacity_guard_evaluation_count",
            predecessor.dehumidification_total_output_capacity_guard_evaluation_count,
            state.dehumidification_total_output_capacity_guard_evaluation_count,
        ),
        (
            "guard_evaluation_partition",
            state.dehumidification_total_output_capacity_guard_evaluation_count,
            evaluated,
        ),
        (
            "dehumidification_total_output_capacity_guard_false_fallthrough_count",
            predecessor.dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
        ),
        (
            "guard_false_route_sum",
            guard_false,
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
        ),
        (
            "dehumidification_total_output_maximum_capacity_assignment_count",
            predecessor.dehumidification_total_output_capacity_adjustment_body_entry_count,
            state.dehumidification_total_output_maximum_capacity_assignment_count,
        ),
        (
            "maximum_capacity_assignment_route_sum",
            assignments,
            state.dehumidification_total_output_maximum_capacity_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "cp383_retained_maximum_total_cooling_capacity_owned_read_count",
            assignments,
            state.cp383_retained_maximum_total_cooling_capacity_owned_read_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            assignments,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_total_output_assignment_write_count",
            assignments,
            state.cooling_total_output_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_site_multiplication_overflow_fails_closed() {
        assert!(checked_mul(usize::MAX, 2, "overflow").is_err());
    }
}
