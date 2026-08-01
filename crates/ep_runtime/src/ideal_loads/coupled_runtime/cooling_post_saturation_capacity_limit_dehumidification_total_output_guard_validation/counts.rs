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
    if predecessor_lineage_route_counts(state) != predecessor_refined_route_counts(predecessor) {
        return Err(violation("predecessor_refined_route_counters", 1, 0));
    }

    let predecessor_routes = predecessor_refined_route_counts(predecessor);
    let predecessor_assignments = [
        predecessor_routes[4],
        predecessor_routes[7],
        predecessor_routes[10],
        predecessor_routes[13],
        predecessor_routes[16],
    ];
    let assignments = route_assignment_counts(state);
    if assignments != predecessor_assignments {
        return Err(violation("predecessor_assignment_route_counters", 1, 0));
    }
    let guard_false = route_guard_false_counts(state);
    let body = route_body_entry_counts(state);
    for ((assignment, guard_false), body) in assignments.into_iter().zip(guard_false).zip(body) {
        let partition = checked_sum(&[guard_false, body], "active_comparison_partition_overflow")?;
        ensure_count(partition, assignment, "active_comparison_partition")?;
    }

    let evaluated = checked_sum(&assignments, "guard_evaluation_route_sum_overflow")?;
    let guard_false_total = checked_sum(&guard_false, "guard_false_route_sum_overflow")?;
    let body_total = checked_sum(&body, "body_entry_route_sum_overflow")?;
    let three_active_sites = checked_mul(evaluated, 3, "source_site_execution_count_overflow")?;
    let sites = checked_sum(
        &[three_active_sites, body_total],
        "source_site_execution_count_overflow",
    )?;
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
            predecessor.dehumidification_total_output_assignment_count,
            state.dehumidification_total_output_capacity_guard_evaluation_count,
        ),
        (
            "guard_evaluation_route_sum",
            evaluated,
            state.dehumidification_total_output_capacity_guard_evaluation_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp382_cooling_total_output_owned_read_count",
            evaluated,
            state.cp382_cooling_total_output_owned_read_count,
        ),
        (
            "cooling_total_output_read_count",
            evaluated,
            state.cooling_total_output_read_count,
        ),
        (
            "cp321_maximum_total_cooling_capacity_owned_read_count",
            evaluated,
            state.cp321_maximum_total_cooling_capacity_owned_read_count,
        ),
        (
            "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count",
            evaluated,
            state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            evaluated,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_total_output_maximum_total_cooling_capacity_comparison_count",
            evaluated,
            state.cooling_total_output_maximum_total_cooling_capacity_comparison_count,
        ),
        (
            "cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count",
            body_total,
            state.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count,
        ),
        (
            "dehumidification_total_output_capacity_adjustment_body_entry_count",
            body_total,
            state.dehumidification_total_output_capacity_adjustment_body_entry_count,
        ),
        (
            "dehumidification_total_output_capacity_guard_false_fallthrough_count",
            guard_false_total,
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count,
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
        assert!(checked_mul(usize::MAX, 3, "overflow").is_err());
    }
}
