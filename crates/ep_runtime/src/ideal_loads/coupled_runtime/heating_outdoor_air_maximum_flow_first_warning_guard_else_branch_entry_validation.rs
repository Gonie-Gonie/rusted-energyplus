//! Cheap coupled validation for CP442 first-warning guard else-entry evidence.

#[rustfmt::skip]
use crate::ideal_loads::{DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_FIRST_EXCLUDED_SOURCE, PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryLifecycleSummary as Lifecycle, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRuntimeState as State, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot as Snapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallLifecycleSummary as PredecessorLifecycle, PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallRuntimeState as PredecessorState, PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot as PredecessorSnapshot, heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot, heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshot_is_exact, heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshots_match_bit_exact, heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call;
    let snapshot =
        output.calculation_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact(
            heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot(snapshot),
            predecessor,
        )
        && heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshot_is_exact(snapshot)
        && local_public_skip_matches(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp441: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp441.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor_cp441.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE
        || predecessor_cp441.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER.len()
            != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts
            != predecessor.predecessor_first_warning_guard_false_fallthrough_route_counts
    {
        return Err(violation(
            "source_predecessor_route_and_system_identity",
            1,
            0,
        ));
    }
    validate_counts(state, predecessor, timestep_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry,
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
        &state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        ensure_count(
            state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts[index],
            predecessor.predecessor_first_warning_guard_false_fallthrough_route_counts[index],
            "first_warning_guard_else_entry_route_alias",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "transition_overflow")?;
    let entries = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts,
        "else_entry_overflow",
    )?;
    let inactive = transitions
        .checked_sub(entries)
        .ok_or_else(|| violation("transition_partition_underflow", entries, transitions))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "else_entry_count",
            entries,
            state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_count,
        ),
        (
            "source_site_execution_count",
            entries,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.cp441_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp441_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.cp441_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp441_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.unchanged_supply_temperature_preservation_count,
            state.cp441_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp441_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        ("public_else_entry_count", 0, entries),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_public_skip_matches(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    !snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered
        && !predecessor
            .heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_site_reached
        && !predecessor.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
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
    Error::CalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_uses_two_routes_one_structural_site_and_keeps_services_out() {
        let source = include_str!(
            "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_validation.rs"
        );
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(production, _)| production);
        for required in [
            "predecessor_route_counts",
            "first_warning_guard_else_entry_route_alias",
            "predecessor_first_warning_guard_false_fallthrough_route_counts",
            "public_else_entry_count",
            "predecessor_cp441_snapshot",
        ] {
            assert!(production.contains(required), "{required}");
        }
        assert!(!production.contains("private_characterization"));
        assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
        for forbidden in [
            "message",
            "sink",
            "sqlite",
            "callback",
            "warning_counter_owner",
        ] {
            assert!(
                !production.to_ascii_lowercase().contains(forbidden),
                "{forbidden}"
            );
        }
    }
}
