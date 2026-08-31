//! Cheap coupled validation for CP434 heating operating-mode Deadband-assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    IdealLoadsSensibleMode,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcHeatingModeGuardElseBranchEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot as Snapshot,
    heating_mode_guard_else_branch_entry_snapshots_match_bit_exact,
    heating_operating_mode_deadband_assignment_predecessor_cp433_snapshot,
    heating_operating_mode_deadband_assignment_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_heating_mode_guard_else_branch_entry;
    let snapshot = output.calculation_heating_operating_mode_deadband_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_mode_guard_else_branch_entry_snapshots_match_bit_exact(
            heating_operating_mode_deadband_assignment_predecessor_cp433_snapshot(snapshot),
            predecessor,
        )
        && local_shape_is_exact(snapshot, predecessor, output)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp433: &PredecessorLifecycle,
    timestep_count: usize,
    numerical_deadband_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp433.state;
    if lifecycle.source != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp433.source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || predecessor_cp433.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE_ORDER.len() != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.heating_operating_mode_deadband_assignment_route_counts
            != predecessor.heating_mode_guard_else_branch_entry_route_counts
    {
        return Err(violation(
            "source_predecessor_route_and_system_identity",
            1,
            0,
        ));
    }
    validate_counts(state, predecessor, timestep_count, numerical_deadband_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_operating_mode_deadband_assignment,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcHeatingModeGuardElseBranchEntryRuntimeState,
    timestep_count: usize,
    numerical_deadband_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.heating_operating_mode_deadband_assignment_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        ensure_count(
            state.heating_operating_mode_deadband_assignment_route_counts[index],
            predecessor.heating_mode_guard_else_branch_entry_route_counts[index],
            "assignment_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let assignments = checked_sum(&state.heating_operating_mode_deadband_assignment_route_counts)?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| violation("inactive_partition_underflow", assignments, transitions))?;
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
            state.heating_operating_mode_deadband_assignment_count,
        ),
        (
            "numerical_deadband_reconciliation_count",
            numerical_deadband_count,
            assignments,
        ),
        (
            "source_site_execution_count",
            assignments,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp432_supply_humidity_ratio_state_owner_count,
            state.cp433_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp433_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp432_supply_enthalpy_state_owner_count,
            state.cp433_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp433_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp432_supply_temperature_state_owner_count,
            state.cp433_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp433_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "operating_mode_owner_count",
            assignments,
            state.cp434_heating_operating_mode_state_owner_count,
        ),
        (
            "operating_mode_write_count",
            assignments,
            state.heating_operating_mode_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
) -> bool {
    let executed = predecessor.heating_mode_guard_else_branch_entered;
    provenance_is_exact(
        snapshot.source,
        snapshot.first_excluded_source,
        snapshot.source_order,
    ) && same(
        snapshot.predecessor_cp433_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && same(
        snapshot.predecessor_cp433_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && same(
        snapshot.predecessor_cp433_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && snapshot.heating_mode_guard_else_branch_entered == executed
        && snapshot.heating_operating_mode_deadband_assignment_executed == executed
        && snapshot.cp433_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp433_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp433_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.heating_operating_mode_deadband_assignment_performed == executed
        && snapshot.assigned_heating_operating_mode_deadband
            == executed.then_some(IdealLoadsSensibleMode::Deadband)
        && !(executed && predecessor.heating_operating_mode_heat_assignment_executed)
        && (output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Deadband)
            == executed
        && same(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

fn provenance_is_exact(source: &str, first_excluded_source: &str, source_order: &[&str]) -> bool {
    source == PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE
        && first_excluded_source
            == PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && source_order
            == PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE_ORDER
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
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
    Error::CalcHeatingOperatingModeDeadbandAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use crate::ideal_loads::{
        IdealLoadsSensibleMode,
        coupled_output::tests::{scaled_output, test_system},
    };

    use super::local_shape_is_exact;

    #[test]
    fn assigned_enum_and_numerical_deadband_are_reconciliation_only() {
        let system = test_system();
        let mut output = scaled_output(&system, 2, 1.0);
        let predecessor = output.calculation_heating_mode_guard_else_branch_entry;
        let exact = output.calculation_heating_operating_mode_deadband_assignment;
        assert!(local_shape_is_exact(exact, predecessor, &output));
        let mut forged = exact;
        forged.assigned_heating_operating_mode_deadband =
            match exact.assigned_heating_operating_mode_deadband {
                Some(_) => None,
                None => Some(IdealLoadsSensibleMode::Deadband),
            };
        assert!(!local_shape_is_exact(forged, predecessor, &output));
        output.coupling.purchased_air.calculation.mode =
            if output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Deadband {
                IdealLoadsSensibleMode::Heating
            } else {
                IdealLoadsSensibleMode::Deadband
            };
        assert!(!local_shape_is_exact(exact, predecessor, &output));
    }

    #[test]
    fn production_validator_keeps_characterization_and_coupling_input_out() {
        let source = include_str!("heating_operating_mode_deadband_assignment_validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(
                include_str!("heating_operating_mode_deadband_assignment_validation.rs"),
                |(production, _)| production,
            );
        for required in [
            "heating_operating_mode_deadband_assignment_route_counts",
            "assigned_heating_operating_mode_deadband",
            "predecessor_cp433_snapshot",
            "numerical_deadband_reconciliation_count",
        ] {
            assert!(source.contains(required), "{required}");
        }
        assert!(!source.contains("private_characterization"));
        assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
        assert!(!source.lines().any(|line| {
            line.contains("calculation.mode =") && !line.contains("calculation.mode ==")
        }));
    }
}
