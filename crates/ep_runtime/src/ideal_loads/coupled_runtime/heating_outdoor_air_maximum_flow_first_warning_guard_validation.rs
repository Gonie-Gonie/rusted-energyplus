//! Cheap coupled validation for CP437 heating outdoor-air first-warning-guard evidence.

#[rustfmt::skip]
use crate::ideal_loads::{DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE_ORDER, PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentLifecycleSummary as PredecessorLifecycle, PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as PredecessorState, PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as PredecessorSnapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardLifecycleSummary as Lifecycle, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as State, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Snapshot, heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact, heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot, heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_is_exact, heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;
const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment;
    let snapshot = output.calculation_heating_outdoor_air_maximum_flow_first_warning_guard;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact(
            heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot(
                snapshot,
            ),
            predecessor,
        )
        && heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_is_exact(snapshot)
        && local_public_skip_matches(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp436: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp436.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp436.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE
        || predecessor_cp436.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE_ORDER
            .len()
            != 3
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.predecessor_guard_body_entry_route_counts
        || state.predecessor_volume_flow_assignment_route_counts
            != predecessor.heating_outdoor_air_volume_flow_assignment_route_counts
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
    if !heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_outdoor_air_maximum_flow_first_warning_guard,
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
        &state.predecessor_volume_flow_assignment_route_counts,
        &state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts,
        &state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        let local_partition = state
            .heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts
            [index]
            .checked_add(
                state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts
                    [index],
            )
            .ok_or_else(|| violation("guard_route_partition_overflow", 0, usize::MAX))?;
        ensure_count(
            local_partition,
            state.predecessor_volume_flow_assignment_route_counts[index],
            "guard_route_refinement",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "transition_overflow")?;
    let evaluations = checked_sum(
        &state.predecessor_volume_flow_assignment_route_counts,
        "guard_evaluation_overflow",
    )?;
    let first_warning_entries = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts,
        "first_warning_branch_entry_overflow",
    )?;
    let false_fallthroughs = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts,
        "guard_false_fallthrough_overflow",
    )?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| violation("transition_partition_underflow", evaluations, transitions))?;
    let source_sites = evaluations
        .checked_mul(2)
        .and_then(|sites| sites.checked_add(first_warning_entries))
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
            "guard_evaluation_count",
            evaluations,
            state.guard_evaluation_count,
        ),
        (
            "first_warning_branch_entry_count",
            first_warning_entries,
            state.first_warning_branch_entry_count,
        ),
        (
            "guard_false_fallthrough_count",
            false_fallthroughs,
            state.guard_false_fallthrough_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.cp436_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp436_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.cp436_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp436_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.unchanged_supply_temperature_preservation_count,
            state.cp436_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp436_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "warning_counter_state_owner_count",
            evaluations,
            state.outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        ),
        (
            "warning_counter_read_count",
            evaluations,
            state.outdoor_air_flow_maximum_heating_output_error_count_read_count,
        ),
        (
            "warning_counter_comparison_count",
            evaluations,
            state
                .outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count,
        ),
        (
            "warning_counter_unchanged",
            0,
            state.outdoor_air_flow_maximum_heating_output_error_count,
        ),
        ("public_guard_evaluation_count", 0, evaluations),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_public_skip_matches(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    !predecessor.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed
        && !snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated
        && snapshot.cp436_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp436_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp436_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && !snapshot.outdoor_air_flow_maximum_heating_output_error_count_state_owned
        && !snapshot.outdoor_air_flow_maximum_heating_output_error_count_read
        && snapshot
            .outdoor_air_flow_maximum_heating_output_error_count_before
            .is_none()
        && !snapshot
            .outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated
        && snapshot
            .outdoor_air_flow_maximum_heating_output_error_count_less_than_one
            .is_none()
        && !snapshot.heating_outdoor_air_maximum_flow_first_warning_branch_entered
        && !snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
        && option_bits_equal(
            snapshot.predecessor_cp436_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.predecessor_cp436_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.predecessor_cp436_resulting_supply_temperature_c,
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
    Error::CalcHeatingOutdoorAirMaximumFlowFirstWarningGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn validator_uses_six_route_arrays_and_keeps_numerical_input_out() {
        let source =
            include_str!("heating_outdoor_air_maximum_flow_first_warning_guard_validation.rs");
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(production, _)| production);
        for required in [
            "predecessor_cp436_snapshot",
            "predecessor_volume_flow_assignment_route_counts",
            "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts",
            "heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts",
            "outdoor_air_flow_maximum_heating_output_error_count_read_count",
        ] {
            assert!(production.contains(required), "{required}");
        }
        assert!(!production.contains("private_characterization"));
        assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    }
}
