//! Cheap coupled validation for CP422 sensible-output maximum-capacity assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_predecessor_cp421_snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact(
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_predecessor_cp421_snapshot(snapshot),
            predecessor,
        )
        && local_assignment_matches(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp421: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp421.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp421.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE
        || predecessor_cp421.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER.len() != 2
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.guard_false_fallthrough_route_counts
        || state.cooling_sensible_output_maximum_capacity_assignment_route_counts
            != predecessor.adjustment_body_entry_route_counts
    {
        return Err(violation("source_predecessor_route_and_system_identity", 1, 0));
    }
    validate_counts(state, predecessor, timestep_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.cooling_sensible_output_maximum_capacity_assignment_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        let successor = checked_add(
            state.predecessor_guard_false_fallthrough_route_counts[index],
            state.cooling_sensible_output_maximum_capacity_assignment_route_counts[index],
            "route_partition_overflow",
        )?;
        let expected = usize::from(ACTIVE_LOGICAL_INDICES.contains(&index))
            .checked_mul(state.predecessor_route_counts[index])
            .ok_or_else(|| violation("route_partition_overflow", 0, usize::MAX))?;
        ensure_count(successor, expected, "route_partition")?;
    }
    let transitions = checked_sum(
        &state.predecessor_route_counts,
        "transition_partition_overflow",
    )?;
    let false_fallthroughs = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "successor_partition_overflow",
    )?;
    let assignments = checked_sum(
        &state.cooling_sensible_output_maximum_capacity_assignment_route_counts,
        "successor_partition_overflow",
    )?;
    let active = checked_add(
        false_fallthroughs,
        assignments,
        "successor_partition_overflow",
    )?;
    let inactive = transitions
        .checked_sub(active)
        .ok_or_else(|| violation("transition_partition_underflow", active, transitions))?;
    let sites = assignments
        .checked_mul(2)
        .ok_or_else(|| violation("site_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_inactive_transition_count", predecessor.inactive_transition_count, state.inactive_transition_count),
        ("predecessor_guard_false_fallthrough_count", false_fallthroughs, state.predecessor_guard_false_fallthrough_count),
        ("cp421_guard_false_fallthrough_count", predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count, false_fallthroughs),
        ("cooling_sensible_output_maximum_capacity_assignment_count", assignments, state.cooling_sensible_output_maximum_capacity_assignment_count),
        ("cp421_body_entry_count", predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count, assignments),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp420_supply_humidity_ratio_state_owner_count, state.cp421_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", state.cp421_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp420_supply_enthalpy_state_owner_count, state.cp421_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", state.cp421_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp420_supply_temperature_state_owner_count, state.cp421_supply_temperature_state_owner_count),
        ("temperature_preservation_count", state.cp421_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
        ("capacity_owned_read_count", assignments, state.cp421_retained_maximum_total_cooling_capacity_owned_read_count),
        ("capacity_read_count", assignments, state.maximum_total_cooling_capacity_for_sensible_output_assignment_read_count),
        ("assignment_write_count", assignments, state.cooling_sensible_output_maximum_capacity_assignment_write_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_assignment_matches(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated;
    let assignment = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered;
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER
        || snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed != assignment
        || snapshot.cp421_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp421_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp421_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !option_bits_equal(snapshot.predecessor_cp421_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !option_bits_equal(snapshot.predecessor_cp421_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        || !option_bits_equal(snapshot.predecessor_cp421_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        || !option_bits_equal(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !option_bits_equal(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        || !option_bits_equal(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
    {
        return false;
    }
    let preexisting = predecessor.cp420_cooling_sensible_output_for_capacity_guard_w;
    if !active {
        return snapshot
            .preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w
            .is_none()
            && !snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
            && !snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
            && snapshot
                .maximum_total_cooling_capacity_for_sensible_output_assignment_w
                .is_none()
            && !snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
            && snapshot
                .assigned_cooling_sensible_output_from_maximum_capacity_w
                .is_none()
            && snapshot
                .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w
                .is_none();
    }
    let Some(preexisting) = preexisting else {
        return false;
    };
    if !option_has_bits(
        snapshot.preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w,
        preexisting,
    ) {
        return false;
    }
    if !assignment {
        return !snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
            && !snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
            && snapshot
                .maximum_total_cooling_capacity_for_sensible_output_assignment_w
                .is_none()
            && !snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
            && snapshot
                .assigned_cooling_sensible_output_from_maximum_capacity_w
                .is_none()
            && option_has_bits(
                snapshot.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w,
                preexisting,
            );
    }
    let Some(maximum) = predecessor.maximum_total_cooling_capacity_w else {
        return false;
    };
    snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
        && snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
        && option_has_bits(
            snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_w,
            maximum,
        )
        && snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
        && option_has_bits(
            snapshot.assigned_cooling_sensible_output_from_maximum_capacity_w,
            maximum,
        )
        && option_has_bits(
            snapshot.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w,
            maximum,
        )
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_add(left: usize, right: usize, field: &'static str) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, 0, usize::MAX))
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hot_validator_uses_only_bounded_cp421_prefix_and_local_assignment() {
        let source = include_str!(
            "cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_validation.rs"
        );
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(value, _)| value);
        for required in [
            "predecessor_cp421_snapshot",
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "cooling_sensible_output_maximum_capacity_assignment_route_counts",
        ] {
            assert!(production.contains(required), "{required}");
        }
        for forbidden in [
            "snapshot_is_exact",
            "private_characterization",
            "predecessor_route(",
            "coupling.zone_sensible_cooling_rate_w",
        ] {
            assert!(!production.contains(forbidden), "{forbidden}");
        }
    }
}
