//! Cheap coupled validation for CP433 heating-mode guard else-entry evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE,
    PurchasedAirCalcHeatingModeGuardElseBranchEntryLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingModeGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot as Snapshot,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentLifecycleSummary as PredecessorLifecycle,
    heating_mode_guard_else_branch_entry_predecessor_cp432_snapshot,
    heating_mode_guard_else_branch_entry_snapshots_match_bit_exact,
    heating_operating_mode_heat_assignment_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_heating_operating_mode_heat_assignment;
    let snapshot = output.calculation_heating_mode_guard_else_branch_entry;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_operating_mode_heat_assignment_snapshots_match_bit_exact(
            heating_mode_guard_else_branch_entry_predecessor_cp432_snapshot(snapshot),
            predecessor,
        )
        && local_shape_is_exact(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp432: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp432.state;
    if lifecycle.source != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor_cp432.source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE
        || predecessor_cp432.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER.len() != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.heating_mode_guard_else_branch_entry_route_counts
            != predecessor.predecessor_heating_mode_guard_false_fallthrough_route_counts
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
    if !heating_mode_guard_else_branch_entry_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_mode_guard_else_branch_entry,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.heating_mode_guard_else_branch_entry_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        ensure_count(
            state.heating_mode_guard_else_branch_entry_route_counts[index],
            predecessor.predecessor_heating_mode_guard_false_fallthrough_route_counts[index],
            "else_entry_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let entries = checked_sum(&state.heating_mode_guard_else_branch_entry_route_counts)?;
    let inactive = predecessor
        .inactive_transition_count
        .checked_add(predecessor.heating_operating_mode_heat_assignment_count)
        .ok_or_else(|| violation("inactive_partition_overflow", 0, usize::MAX))?;
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
            state.heating_mode_guard_else_branch_entry_count,
        ),
        (
            "source_site_execution_count",
            entries,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp431_supply_humidity_ratio_state_owner_count,
            state.cp432_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp432_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp431_supply_enthalpy_state_owner_count,
            state.cp432_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp432_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp431_supply_temperature_state_owner_count,
            state.cp432_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp432_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot,
) -> bool {
    provenance_is_exact(
        snapshot.source,
        snapshot.first_excluded_source,
        snapshot.source_order,
    ) && snapshot.heating_mode_guard_else_branch_entered
        == predecessor.heating_mode_guard_false_fallthrough
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        && first_excluded_source
            == PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        && source_order == PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("count_overflow", 0, usize::MAX))
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
    Error::CalcHeatingModeGuardElseBranchEntryLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
        PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE as SOURCE,
        PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as ORDER,
        provenance_is_exact,
    };

    #[test]
    fn snapshot_provenance_rejects_each_coordinated_field_forgery() {
        assert!(provenance_is_exact(SOURCE, EXCLUDED, ORDER));
        assert!(!provenance_is_exact("forged source", EXCLUDED, ORDER));
        assert!(!provenance_is_exact(SOURCE, "forged exclusion", ORDER));
        assert!(!provenance_is_exact(SOURCE, EXCLUDED, &["forged order"]));
    }

    #[test]
    fn hot_validator_is_structural_and_keeps_deadband_and_numerical_dto_out() {
        let source = include_str!("heating_mode_guard_else_branch_entry_validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(
                include_str!("heating_mode_guard_else_branch_entry_validation.rs"),
                |(production, _)| production,
            );
        for required in [
            "predecessor_route_counts",
            "heating_mode_guard_else_branch_entry_route_counts",
            "predecessor_heating_mode_guard_false_fallthrough_route_counts",
            "predecessor_cp432_snapshot",
        ] {
            assert!(source.contains(required), "{required}");
        }
        for forbidden in [
            "IdealLoadsSensibleMode::Deadband",
            "calculation.mode",
            "DirectZonePurchasedAirCouplingInput",
            "private_characterization",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
