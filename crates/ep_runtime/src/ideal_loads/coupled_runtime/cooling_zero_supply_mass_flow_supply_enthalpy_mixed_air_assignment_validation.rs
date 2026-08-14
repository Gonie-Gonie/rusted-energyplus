//! Cheap coupled validation for CP425 zero-flow supply-enthalpy evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot as Snapshot,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshots_match_bit_exact,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_predecessor_cp424_snapshot,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry;
    let snapshot =
        output.calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshots_match_bit_exact(
            cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_predecessor_cp424_snapshot(snapshot),
            predecessor,
        )
        && local_shape_is_exact(snapshot, predecessor)
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp424: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp424.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp424.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || predecessor_cp424.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER.len() != 2
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
    if !cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_snapshots_match_bit_exact(
        latest,
        latest_output
            .calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        let expected = if index == 2 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(
            state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts[index],
            expected,
            "assignment_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let assignments = checked_sum(
        &state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts,
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| violation("inactive_partition_underflow", assignments, transitions))?;
    let sites = assignments
        .checked_mul(2)
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
            state.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp423_supply_humidity_ratio_state_owner_count,
            state.cp424_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp424_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp423_supply_enthalpy_state_owner_count,
            state.cp424_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp424_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp423_supply_temperature_state_owner_count,
            state.cp424_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp424_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp425_enthalpy_owner_count",
            assignments,
            state.cp425_supply_enthalpy_state_owner_count,
        ),
        (
            "mixed_air_owner_read_count",
            assignments,
            state.cp329_retained_mixed_air_enthalpy_owned_read_count,
        ),
        (
            "mixed_air_read_count",
            assignments,
            state.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read_count,
        ),
        (
            "assignment_write_count",
            assignments,
            state.supply_enthalpy_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot,
) -> bool {
    let assignment = predecessor.cooling_supply_mass_flow_positive_guard_else_branch_entered;
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_ENTHALPY_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
        && snapshot.cooling_supply_mass_flow_positive_guard_else_branch_entered == assignment
        && snapshot.cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_executed == assignment
        && snapshot.cp424_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp424_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp424_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.cp329_retained_mixed_air_enthalpy_owned_read == assignment
        && snapshot.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read == assignment
        && snapshot.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_performed
            == assignment
        && same(snapshot.predecessor_cp424_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        && same(snapshot.predecessor_cp424_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        && same(snapshot.predecessor_cp424_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        && same(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        && same(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        && if assignment {
            active_enthalpy_chain_is_exact(
                snapshot
                    .mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg,
                snapshot.assigned_supply_enthalpy_from_mixed_air_j_per_kg,
                snapshot.resulting_supply_enthalpy_j_per_kg,
            )
        } else {
            snapshot.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg.is_none()
                && snapshot.assigned_supply_enthalpy_from_mixed_air_j_per_kg.is_none()
                && same(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        }
}

fn active_enthalpy_chain_is_exact(
    mixed_air_rhs: Option<f64>,
    assigned: Option<f64>,
    resulting: Option<f64>,
) -> bool {
    match (mixed_air_rhs, assigned, resulting) {
        (Some(mixed_air_rhs), Some(assigned), Some(resulting)) => {
            mixed_air_rhs.to_bits() == assigned.to_bits()
                && assigned.to_bits() == resulting.to_bits()
        }
        _ => false,
    }
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
    Error::CalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::active_enthalpy_chain_is_exact;

    #[test]
    fn active_enthalpy_chain_rejects_each_none_forgery() {
        let value = f64::from_bits(0x7ff8_0000_0000_0042);
        assert!(active_enthalpy_chain_is_exact(
            Some(value),
            Some(value),
            Some(value)
        ));
        for forged in [
            (None, Some(value), Some(value)),
            (Some(value), None, Some(value)),
            (Some(value), Some(value), None),
        ] {
            assert!(!active_enthalpy_chain_is_exact(
                forged.0, forged.1, forged.2
            ));
        }
    }

    #[test]
    fn hot_validator_is_bounded_and_keeps_the_numerical_dto_out() {
        let source = include_str!(
            "cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_validation.rs"
        )
        .split_once("#[cfg(test)]")
        .map_or(
            include_str!(
                "cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_validation.rs"
            ),
            |(production, _)| production,
        );
        for required in [
            "predecessor_route_counts",
            "zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_counts",
            "predecessor_cp424_snapshot",
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
