//! Cheap coupled validation for CP430 Heating-or-no-load case-entry evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_TOTAL_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_TOTAL_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOrNoLoadCaseEntryLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState as State,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Snapshot,
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshots_match_bit_exact,
    heating_or_no_load_case_entry_predecessor_cp429_snapshot,
    heating_or_no_load_case_entry_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment;
    let snapshot = output.calculation_heating_or_no_load_case_entry;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshots_match_bit_exact(
            heating_or_no_load_case_entry_predecessor_cp429_snapshot(snapshot),
            predecessor,
        )
        && local_shape_is_exact(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp429: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp429.state;
    if lifecycle.source != PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor_cp429.source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_TOTAL_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_SOURCE
        || predecessor_cp429.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_TOTAL_OUTPUT_POSITIVE_ZERO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE_ORDER.len() != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
    {
        return Err(violation("source_predecessor_route_and_system_identity", 1, 0));
    }
    validate_counts(state, predecessor, timestep_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !heating_or_no_load_case_entry_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_or_no_load_case_entry,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.heating_or_no_load_case_entry_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        let expected = if index == 1 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(
            state.heating_or_no_load_case_entry_route_counts[index],
            expected,
            "case_entry_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let entries = checked_sum(&state.heating_or_no_load_case_entry_route_counts)?;
    let inactive = transitions
        .checked_sub(entries)
        .ok_or_else(|| violation("inactive_partition_underflow", entries, transitions))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "case_entry_count",
            entries,
            state.heating_or_no_load_case_entry_count,
        ),
        (
            "source_site_execution_count",
            entries,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp428_supply_humidity_ratio_state_owner_count,
            state.cp429_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp429_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp428_supply_enthalpy_state_owner_count,
            state.cp429_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp429_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp428_supply_temperature_state_owner_count,
            state.cp429_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp429_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
) -> bool {
    provenance_is_exact(
        snapshot.source,
        snapshot.first_excluded_source,
        snapshot.source_order,
    ) && snapshot.heating_or_no_load_case_entered == predecessor.non_cooling_skipped
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
    source == PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE
        && first_excluded_source
            == PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        && source_order == PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE_ORDER
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
    Error::CalcHeatingOrNoLoadCaseEntryLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
        PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE as SOURCE,
        PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE_ORDER as ORDER,
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
    fn hot_validator_is_bounded_and_keeps_the_numerical_dto_out() {
        let source = include_str!("heating_or_no_load_case_entry_validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(
                include_str!("heating_or_no_load_case_entry_validation.rs"),
                |(production, _)| production,
            );
        for required in [
            "predecessor_route_counts",
            "heating_or_no_load_case_entry_route_counts",
            "predecessor_cp429_snapshot",
        ] {
            assert!(source.contains(required), "{required}");
        }
        for forbidden in [
            "snapshot_is_exact",
            "private_characterization",
            "DirectZonePurchasedAirCouplingInput",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
