//! Cheap coupled validation for CP423 sensible-output supply-temperature assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_predecessor_cp422_snapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshots_match_bit_exact,
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
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact(
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_predecessor_cp422_snapshot(snapshot),
            predecessor,
        )
        && local_assignment_matches(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp422: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp422.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp422.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || predecessor_cp422.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len() != 8
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.cooling_sensible_output_supply_temperature_assignment_route_counts
            != predecessor.cooling_sensible_output_maximum_capacity_assignment_route_counts
    {
        return Err(violation("source_predecessor_route_and_system_identity", 1, 0));
    }
    validate_counts(state, predecessor, timestep_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
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
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.cooling_sensible_output_supply_temperature_assignment_route_counts,
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
            state.cooling_sensible_output_supply_temperature_assignment_route_counts[index],
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
        &state.cooling_sensible_output_supply_temperature_assignment_route_counts,
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
    let temperature_preservations = state
        .cp422_supply_temperature_state_owner_count
        .checked_sub(assignments)
        .ok_or_else(|| {
            violation(
                "temperature_preservation_underflow",
                assignments,
                state.cp422_supply_temperature_state_owner_count,
            )
        })?;
    let sites = assignments
        .checked_mul(8)
        .ok_or_else(|| violation("site_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_inactive_transition_count", predecessor.inactive_transition_count, state.inactive_transition_count),
        ("predecessor_guard_false_fallthrough_count", false_fallthroughs, state.predecessor_guard_false_fallthrough_count),
        ("cp422_guard_false_fallthrough_count", predecessor.predecessor_guard_false_fallthrough_count, false_fallthroughs),
        ("supply_temperature_assignment_count", assignments, state.cooling_sensible_output_supply_temperature_assignment_count),
        ("cp422_assignment_count", predecessor.cooling_sensible_output_maximum_capacity_assignment_count, assignments),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp421_supply_humidity_ratio_state_owner_count, state.cp422_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", state.cp422_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp421_supply_enthalpy_state_owner_count, state.cp422_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", state.cp422_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp421_supply_temperature_state_owner_count, state.cp422_supply_temperature_state_owner_count),
        ("temperature_preservation_count", temperature_preservations, state.unchanged_supply_temperature_preservation_count),
        ("cp423_temperature_owner_count", assignments, state.cp423_sensible_output_supply_temperature_state_owner_count),
        ("mixed_air_owned_read_count", assignments, state.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read_count),
        ("mixed_air_read_count", assignments, state.mixed_air_temperature_for_sensible_output_supply_temperature_read_count),
        ("cooling_output_owned_read_count", assignments, state.cp422_retained_cooling_sensible_output_owned_read_count),
        ("cooling_output_read_count", assignments, state.cooling_sensible_output_for_supply_temperature_read_count),
        ("mass_flow_owned_read_count", assignments, state.cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read_count),
        ("mass_flow_corroboration_count", assignments, state.cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroboration_count),
        ("mass_flow_read_count", assignments, state.supply_mass_flow_rate_for_sensible_output_supply_temperature_read_count),
        ("cp_air_owned_read_count", assignments, state.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read_count),
        ("cp_air_read_count", assignments, state.cp_air_for_sensible_output_supply_temperature_read_count),
        ("air_capacity_rate_count", assignments, state.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculation_count),
        ("temperature_drop_count", assignments, state.cooling_sensible_output_over_air_capacity_rate_calculation_count),
        ("temperature_calculation_count", assignments, state.sensible_output_supply_temperature_calculation_count),
        ("temperature_write_count", assignments, state.sensible_output_supply_temperature_assignment_write_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_assignment_matches(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let assignment = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed;
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        || snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed != assignment
        || snapshot.cp422_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp422_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp422_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !option_bits_equal(snapshot.predecessor_cp422_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !option_bits_equal(snapshot.predecessor_cp422_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        || !option_bits_equal(snapshot.predecessor_cp422_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        || !option_bits_equal(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !option_bits_equal(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
    {
        return false;
    }
    if !assignment {
        return local_rhs_is_empty(snapshot)
            && option_bits_equal(
                snapshot.resulting_supply_temperature_c,
                predecessor.resulting_supply_temperature_c,
            );
    }
    let (Some(mixed), Some(cooling), Some(mass_flow), Some(cp_air)) = (
        predecessor.mixed_air_temperature_for_sensible_output_c,
        predecessor.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w,
        predecessor.supply_mass_flow_rate_kg_per_s,
        predecessor.cp_air_j_per_kg_k,
    ) else {
        return false;
    };
    let capacity_rate = mass_flow * cp_air;
    let drop = cooling / capacity_rate;
    let calculated = mixed - drop;
    snapshot.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read
        && snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_read
        && option_has_bits(
            snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c,
            mixed,
        )
        && snapshot.cp422_retained_cooling_sensible_output_owned_read
        && snapshot.cooling_sensible_output_for_supply_temperature_read
        && option_has_bits(
            snapshot.cooling_sensible_output_for_supply_temperature_w,
            cooling,
        )
        && snapshot
            .cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read
        && snapshot
            .cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated
        && snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_read
        && option_has_bits(
            snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s,
            mass_flow,
        )
        && snapshot.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read
        && snapshot.cp_air_for_sensible_output_supply_temperature_read
        && option_has_bits(
            snapshot.cp_air_for_sensible_output_supply_temperature_j_per_kg_k,
            cp_air,
        )
        && snapshot
            .supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated
        && option_has_bits(
            snapshot
                .supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k,
            capacity_rate,
        )
        && snapshot.cooling_sensible_output_over_air_capacity_rate_calculated
        && option_has_bits(
            snapshot.cooling_sensible_output_over_air_capacity_rate_k,
            drop,
        )
        && snapshot.sensible_output_supply_temperature_calculated
        && option_has_bits(
            snapshot.calculated_sensible_output_supply_temperature_c,
            calculated,
        )
        && snapshot.sensible_output_supply_temperature_assignment_performed
        && option_has_bits(
            snapshot.assigned_sensible_output_supply_temperature_c,
            calculated,
        )
        && option_has_bits(snapshot.resulting_supply_temperature_c, calculated)
}

fn local_rhs_is_empty(snapshot: Snapshot) -> bool {
    !snapshot.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read
        && !snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_read
        && snapshot
            .mixed_air_temperature_for_sensible_output_supply_temperature_c
            .is_none()
        && !snapshot.cp422_retained_cooling_sensible_output_owned_read
        && !snapshot.cooling_sensible_output_for_supply_temperature_read
        && snapshot
            .cooling_sensible_output_for_supply_temperature_w
            .is_none()
        && !snapshot
            .cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read
        && !snapshot
            .cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated
        && !snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_read
        && snapshot
            .supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s
            .is_none()
        && !snapshot.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read
        && !snapshot.cp_air_for_sensible_output_supply_temperature_read
        && snapshot
            .cp_air_for_sensible_output_supply_temperature_j_per_kg_k
            .is_none()
        && !snapshot
            .supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated
        && snapshot
            .supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k
            .is_none()
        && !snapshot.cooling_sensible_output_over_air_capacity_rate_calculated
        && snapshot
            .cooling_sensible_output_over_air_capacity_rate_k
            .is_none()
        && !snapshot.sensible_output_supply_temperature_calculated
        && snapshot
            .calculated_sensible_output_supply_temperature_c
            .is_none()
        && !snapshot.sensible_output_supply_temperature_assignment_performed
        && snapshot
            .assigned_sensible_output_supply_temperature_c
            .is_none()
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hot_validator_uses_only_bounded_cp422_prefix_and_local_assignment() {
        let source = include_str!(
            "cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_validation.rs"
        );
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(value, _)| value);
        for required in [
            "predecessor_cp422_snapshot",
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "cooling_sensible_output_supply_temperature_assignment_route_counts",
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
