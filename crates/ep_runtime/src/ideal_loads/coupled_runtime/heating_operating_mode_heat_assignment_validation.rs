//! Cheap coupled validation for CP432 heating operating-mode Heat-assignment evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    IdealLoadsSensibleMode, PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcHeatingModeGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Snapshot,
    heating_mode_guard_snapshots_match_bit_exact,
    heating_operating_mode_heat_assignment_predecessor_cp431_snapshot,
    heating_operating_mode_heat_assignment_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_heating_mode_guard;
    let snapshot = output.calculation_heating_operating_mode_heat_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_mode_guard_snapshots_match_bit_exact(
            heating_operating_mode_heat_assignment_predecessor_cp431_snapshot(snapshot),
            predecessor,
        )
        && local_shape_is_exact(snapshot, predecessor, output)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp431: &PredecessorLifecycle,
    timestep_count: usize,
    numerical_heating_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp431.state;
    if lifecycle.source != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp431.source != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE
        || predecessor_cp431.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE_ORDER.len() != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_heating_mode_guard_evaluation_route_counts
            != predecessor.heating_mode_guard_evaluation_route_counts
        || state.predecessor_heating_mode_guard_false_fallthrough_route_counts
            != predecessor.heating_mode_guard_false_fallthrough_route_counts
        || state.heating_operating_mode_heat_assignment_route_counts
            != predecessor.heating_operating_mode_body_entry_route_counts
    {
        return Err(violation(
            "source_predecessor_route_and_system_identity",
            1,
            0,
        ));
    }
    validate_counts(state, predecessor, timestep_count, numerical_heating_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !heating_operating_mode_heat_assignment_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_operating_mode_heat_assignment,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcHeatingModeGuardRuntimeState,
    timestep_count: usize,
    numerical_heating_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_heating_mode_guard_evaluation_route_counts,
        &state.predecessor_heating_mode_guard_false_fallthrough_route_counts,
        &state.heating_operating_mode_heat_assignment_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        let evaluations = state.predecessor_heating_mode_guard_evaluation_route_counts[index];
        let false_fallthroughs =
            state.predecessor_heating_mode_guard_false_fallthrough_route_counts[index];
        let assignments = state.heating_operating_mode_heat_assignment_route_counts[index];
        let terminal = false_fallthroughs
            .checked_add(assignments)
            .ok_or_else(|| violation("active_route_partition_overflow", 0, usize::MAX))?;
        ensure_count(terminal, evaluations, "active_route_partition")?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let evaluations = checked_sum(&state.predecessor_heating_mode_guard_evaluation_route_counts)?;
    let false_fallthroughs =
        checked_sum(&state.predecessor_heating_mode_guard_false_fallthrough_route_counts)?;
    let assignments = checked_sum(&state.heating_operating_mode_heat_assignment_route_counts)?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| violation("inactive_partition_underflow", evaluations, transitions))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "predecessor_evaluation_count",
            evaluations,
            state.predecessor_heating_mode_guard_evaluation_count,
        ),
        (
            "predecessor_false_fallthrough_count",
            false_fallthroughs,
            state.predecessor_heating_mode_guard_false_fallthrough_count,
        ),
        (
            "assignment_count",
            assignments,
            state.heating_operating_mode_heat_assignment_count,
        ),
        (
            "numerical_heating_reconciliation_count",
            numerical_heating_count,
            assignments,
        ),
        (
            "source_site_execution_count",
            assignments,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp430_supply_humidity_ratio_state_owner_count,
            state.cp431_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp431_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp430_supply_enthalpy_state_owner_count,
            state.cp431_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp431_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp430_supply_temperature_state_owner_count,
            state.cp431_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp431_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "operating_mode_owner_count",
            assignments,
            state.cp432_heating_operating_mode_state_owner_count,
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
    predecessor: crate::ideal_loads::PurchasedAirCalcHeatingModeGuardSnapshot,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
) -> bool {
    let executed = predecessor.heating_operating_mode_body_entered;
    provenance_is_exact(
        snapshot.source,
        snapshot.first_excluded_source,
        snapshot.source_order,
    ) && same(
        snapshot.predecessor_cp431_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && same(
        snapshot.predecessor_cp431_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && same(
        snapshot.predecessor_cp431_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && snapshot.heating_operating_mode_heat_assignment_executed == executed
        && snapshot.cp431_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp431_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp431_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.heating_operating_mode_heat_assignment_performed == executed
        && snapshot.assigned_heating_operating_mode
            == executed.then_some(IdealLoadsSensibleMode::Heating)
        && (output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Heating)
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
    source == PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE
        && first_excluded_source
            == PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && source_order == PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE_ORDER
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
    Error::CalcHeatingOperatingModeHeatAssignmentLifecycleInvariant {
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
    fn assigned_enum_and_numerical_heating_are_reconciliation_only() {
        let system = test_system();
        let mut output = scaled_output(&system, 0, 1.0);
        let predecessor = output.calculation_heating_mode_guard;
        let exact = output.calculation_heating_operating_mode_heat_assignment;
        assert!(local_shape_is_exact(exact, predecessor, &output));
        let mut forged = exact;
        forged.assigned_heating_operating_mode = match exact.assigned_heating_operating_mode {
            Some(_) => None,
            None => Some(IdealLoadsSensibleMode::Heating),
        };
        assert!(!local_shape_is_exact(forged, predecessor, &output));
        output.coupling.purchased_air.calculation.mode =
            if output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Heating {
                IdealLoadsSensibleMode::Deadband
            } else {
                IdealLoadsSensibleMode::Heating
            };
        assert!(!local_shape_is_exact(exact, predecessor, &output));
    }

    #[test]
    fn hot_validator_keeps_characterization_and_numerical_mutation_out() {
        let source = include_str!("heating_operating_mode_heat_assignment_validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(
                include_str!("heating_operating_mode_heat_assignment_validation.rs"),
                |(production, _)| production,
            );
        for required in [
            "predecessor_route_counts",
            "predecessor_heating_mode_guard_evaluation_route_counts",
            "predecessor_heating_mode_guard_false_fallthrough_route_counts",
            "heating_operating_mode_heat_assignment_route_counts",
            "assigned_heating_operating_mode",
        ] {
            assert!(source.contains(required), "{required}");
        }
        assert!(!source.contains("private_characterization"));
        assert!(!source.lines().any(|line| {
            line.contains("calculation.mode =") && !line.contains("calculation.mode ==")
        }));
    }
}
