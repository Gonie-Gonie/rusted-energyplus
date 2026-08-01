use super::routes::*;
use super::*;

pub(super) fn validate(
    state: &State,
    predecessor: &PredecessorState,
    timestep_count: usize,
) -> Result<(), Error> {
    if base_route_counts(state) != predecessor_base_route_counts(predecessor)
        || inherited_lineage_route_counts(state) != predecessor_lineage_route_counts(predecessor)
        || guard_false_route_counts(state) != predecessor_guard_false_route_counts(predecessor)
        || assignment_route_counts(state) != predecessor_assignment_route_counts(predecessor)
        || refined_route_counts(state) != predecessor_refined_route_counts(predecessor)
    {
        return Err(violation("predecessor_route_counters", 1, 0));
    }

    let guard_false = checked_sum(
        &guard_false_route_counts(state),
        "guard_false_route_sum_overflow",
    )?;
    let assignments = checked_sum(
        &assignment_route_counts(state),
        "assignment_route_sum_overflow",
    )?;
    let retained = checked_add(guard_false, assignments, "retained_route_sum_overflow")?;
    let source_sites = checked_mul(assignments, 6, "source_site_execution_count_overflow")?;
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
            retained,
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
            predecessor.dehumidification_total_output_maximum_capacity_assignment_count,
            state.dehumidification_total_output_maximum_capacity_assignment_count,
        ),
        (
            "post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count",
            assignments,
            state
                .post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "cp379_retained_supply_enthalpy_owned_read_count",
            retained,
            state.cp379_retained_supply_enthalpy_owned_read_count,
        ),
        (
            "cp329_retained_mixed_air_enthalpy_owned_read_count",
            assignments,
            state.cp329_retained_mixed_air_enthalpy_owned_read_count,
        ),
        (
            "mixed_air_enthalpy_read_count",
            assignments,
            state.mixed_air_enthalpy_read_count,
        ),
        (
            "cp384_retained_cooling_total_output_owned_read_count",
            assignments,
            state.cp384_retained_cooling_total_output_owned_read_count,
        ),
        (
            "cooling_total_output_read_count",
            assignments,
            state.cooling_total_output_read_count,
        ),
        (
            "cp330_retained_supply_mass_flow_rate_owned_read_count",
            assignments,
            state.cp330_retained_supply_mass_flow_rate_owned_read_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assignments,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "specific_cooling_output_calculation_count",
            assignments,
            state.specific_cooling_output_calculation_count,
        ),
        (
            "supply_enthalpy_difference_calculation_count",
            assignments,
            state.supply_enthalpy_difference_calculation_count,
        ),
        (
            "supply_enthalpy_assignment_write_count",
            assignments,
            state.supply_enthalpy_assignment_write_count,
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

fn checked_add(left: usize, right: usize, field: &'static str) -> Result<usize, Error> {
    left.checked_add(right)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_site_multiplication_overflow_fails_closed() {
        assert!(checked_mul(usize::MAX, 6, "overflow").is_err());
    }
}
