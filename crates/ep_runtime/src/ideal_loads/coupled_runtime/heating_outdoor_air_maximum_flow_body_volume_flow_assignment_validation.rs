//! Cheap coupled validation for CP436 heating outdoor-air volume-flow-assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as PredecessorState,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as PredecessorSnapshot,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_predecessor_cp435_snapshot,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_is_exact,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact,
    heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_heating_outdoor_air_maximum_flow_guard;
    let snapshot = output.calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
            heating_outdoor_air_maximum_flow_body_volume_flow_assignment_predecessor_cp435_snapshot(
                snapshot,
            ),
            predecessor,
        )
        && heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_is_exact(snapshot)
        && local_public_skip_matches(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp435: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp435.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp435.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE
        || predecessor_cp435.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE_ORDER.len()
            != 4
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.maximum_heating_flow_body_entry_route_counts
    {
        return Err(violation(
            "source_owner_predecessor_and_route_identity",
            1,
            0,
        ));
    }
    validate_counts(state, predecessor, timestep_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment,
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
        &state.predecessor_guard_body_entry_route_counts,
        &state.heating_outdoor_air_volume_flow_assignment_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        ensure_count(
            state.heating_outdoor_air_volume_flow_assignment_route_counts[index],
            state.predecessor_guard_body_entry_route_counts[index],
            "assignment_route_refinement",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "transition_overflow")?;
    let assignments = checked_sum(
        &state.heating_outdoor_air_volume_flow_assignment_route_counts,
        "assignment_overflow",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| violation("transition_partition_underflow", assignments, transitions))?;
    let source_sites = assignments
        .checked_mul(4)
        .ok_or_else(|| violation("source_site_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "assignment_count",
            assignments,
            state.outdoor_air_volume_flow_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp434_supply_humidity_ratio_state_owner_count,
            state.cp435_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp435_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp434_supply_enthalpy_state_owner_count,
            state.cp435_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp435_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp434_supply_temperature_state_owner_count,
            state.cp435_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp435_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp435_outdoor_air_owner_read_count",
            assignments,
            state.cp435_outdoor_air_mass_flow_rate_owned_read_count,
        ),
        (
            "outdoor_air_division_read_count",
            assignments,
            state.outdoor_air_mass_flow_rate_for_volume_flow_division_read_count,
        ),
        (
            "standard_density_owner_count",
            assignments,
            state.begin_environment_standard_air_density_owner_count,
        ),
        (
            "standard_density_division_read_count",
            assignments,
            state.standard_air_density_for_volume_flow_division_read_count,
        ),
        (
            "division_count",
            assignments,
            state.outdoor_air_mass_flow_rate_standard_air_density_division_count,
        ),
        (
            "assignment_write_count",
            assignments,
            state.local_outdoor_air_volume_flow_rate_assignment_write_count,
        ),
        ("public_assignment_count", 0, assignments),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_public_skip_matches(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    !predecessor.maximum_heating_flow_body_entered
        && !snapshot.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed
        && !snapshot.cp435_retained_outdoor_air_mass_flow_rate_owned_read
        && !snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read
        && snapshot
            .outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s
            .is_none()
        && !snapshot.begin_environment_standard_air_density_owned_read
        && !snapshot.standard_air_density_for_outdoor_air_volume_flow_division_read
        && snapshot
            .standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3
            .is_none()
        && !snapshot.outdoor_air_mass_flow_rate_standard_air_density_division_evaluated
        && snapshot
            .calculated_outdoor_air_volume_flow_rate_m3_per_s
            .is_none()
        && !snapshot.local_outdoor_air_volume_flow_rate_assignment_performed
        && snapshot
            .assigned_outdoor_air_volume_flow_rate_m3_per_s
            .is_none()
        && option_bits_equal(
            snapshot.predecessor_cp435_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.predecessor_cp435_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.predecessor_cp435_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && option_bits_equal(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
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
    Error::CalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_uses_four_route_arrays_and_keeps_numerical_input_out() {
        let source = include_str!(
            "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_validation.rs"
        );
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(production, _)| production);
        for required in [
            "predecessor_cp435_snapshot",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_body_entry_route_counts",
            "heating_outdoor_air_volume_flow_assignment_route_counts",
            "standard_air_density_for_outdoor_air_volume_flow_division_read",
        ] {
            assert!(production.contains(required), "{required}");
        }
        assert!(!production.contains("private_characterization"));
        assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    }
}
