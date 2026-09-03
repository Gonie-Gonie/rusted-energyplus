//! Cheap coupled validation for CP439 heating first-warning call-site evidence.

#[rustfmt::skip]
use crate::ideal_loads::{DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_FIRST_EXCLUDED_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_SOURCE_ORDER, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_FIRST_EXCLUDED_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallLifecycleSummary as Lifecycle, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallRuntimeState as State, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot as Snapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementLifecycleSummary as PredecessorLifecycle, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState as PredecessorState, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot as PredecessorSnapshot, heating_outdoor_air_maximum_flow_first_warning_call_predecessor_cp438_snapshot, heating_outdoor_air_maximum_flow_first_warning_call_snapshot_is_exact, heating_outdoor_air_maximum_flow_first_warning_call_snapshots_match_bit_exact, heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment;
    let snapshot = output.calculation_heating_outdoor_air_maximum_flow_first_warning_call;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact(
            heating_outdoor_air_maximum_flow_first_warning_call_predecessor_cp438_snapshot(snapshot),
            predecessor,
        )
        && heating_outdoor_air_maximum_flow_first_warning_call_snapshot_is_exact(snapshot)
        && local_public_skip_matches(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp438: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp438.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_FIRST_EXCLUDED_SOURCE
        || predecessor_cp438.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE
        || predecessor_cp438.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_SOURCE_ORDER.len()
            != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.predecessor_guard_body_entry_route_counts
        || state.predecessor_volume_flow_assignment_route_counts
            != predecessor.predecessor_volume_flow_assignment_route_counts
        || state.predecessor_first_warning_guard_false_fallthrough_route_counts
            != predecessor.predecessor_first_warning_guard_false_fallthrough_route_counts
        || state.predecessor_first_warning_branch_entry_route_counts
            != predecessor.predecessor_first_warning_branch_entry_route_counts
        || state.predecessor_first_warning_counter_increment_route_counts
            != predecessor
                .heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts
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
    if !heating_outdoor_air_maximum_flow_first_warning_call_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_outdoor_air_maximum_flow_first_warning_call,
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
        &state.predecessor_first_warning_guard_false_fallthrough_route_counts,
        &state.predecessor_first_warning_branch_entry_route_counts,
        &state.predecessor_first_warning_counter_increment_route_counts,
        &state.heating_outdoor_air_maximum_flow_first_warning_call_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        ensure_count(
            state.heating_outdoor_air_maximum_flow_first_warning_call_route_counts[index],
            state.predecessor_first_warning_counter_increment_route_counts[index],
            "first_warning_call_route_alias",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "transition_overflow")?;
    let calls = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_call_route_counts,
        "first_warning_call_overflow",
    )?;
    let inactive = transitions
        .checked_sub(calls)
        .ok_or_else(|| violation("transition_partition_underflow", calls, transitions))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "first_warning_call_site_count",
            calls,
            state.heating_outdoor_air_maximum_flow_first_warning_call_site_count,
        ),
        (
            "source_site_execution_count",
            calls,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.cp438_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp438_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.cp438_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp438_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.unchanged_supply_temperature_preservation_count,
            state.cp438_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp438_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "warning_counter_owner_count",
            calls,
            state.cp438_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        ),
        (
            "warning_counter_preservation_count",
            state.cp438_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
            state.unchanged_outdoor_air_flow_maximum_heating_output_error_count_preservation_count,
        ),
        (
            "predecessor_warning_counter_increment_count",
            predecessor.outdoor_air_flow_maximum_heating_output_error_count_increment_count,
            calls,
        ),
        ("public_first_warning_call_count", 0, calls),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_public_skip_matches(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    !snapshot.heating_outdoor_air_maximum_flow_first_warning_call_site_reached
        && !predecessor.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed
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
    Error::CalcHeatingOutdoorAirMaximumFlowFirstWarningCallLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_uses_eight_routes_one_marker_site_and_keeps_services_out() {
        let source =
            include_str!("heating_outdoor_air_maximum_flow_first_warning_call_validation.rs");
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(production, _)| production);
        for required in [
            "predecessor_cp438_snapshot",
            "first_warning_call_route_alias",
            "public_first_warning_call_count",
            "warning_counter_preservation_count",
        ] {
            assert!(production.contains(required), "{required}");
        }
        assert!(!production.contains("private_characterization"));
        assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
        for forbidden in ["message", "sink", "sqlite", "callback"] {
            assert!(
                !production.to_ascii_lowercase().contains(forbidden),
                "{forbidden}"
            );
        }
    }
}
