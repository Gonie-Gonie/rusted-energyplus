//! Cheap coupled validation for CP420 not-dehumidifying sensible-output evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary as SupplyFlowLifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_has_exact_cp419_prefix_and_local_assignment,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ASSIGNMENT_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment;
    let owner_input = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        .then(|| {
            Some(ActiveInput {
                supply_mass_flow_rate_kg_per_s: output
                    .calculation_cooling_supply_mass_flow_positive_guard
                    .supply_mass_flow_rate_kg_per_s?,
                mixed_air_temperature_c: output
                    .calculation_cooling_mixed_air_call
                    .mixed_air_temperature_c?,
            })
        })
        .flatten();

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_has_exact_cp419_prefix_and_local_assignment(
            snapshot,
            predecessor,
            owner_input,
        )
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp419: &PredecessorLifecycle,
    supply_flow_cp330: &SupplyFlowLifecycle,
    mixed_air_cp329: &MixedAirLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp419.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp419.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor_cp419.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || supply_flow_cp330.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || supply_flow_cp330.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || mixed_air_cp329.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_cp329.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len() != 8
        || [state.system, predecessor.system, supply_flow_cp330.state.system, mixed_air_cp329.state.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || supply_flow_cp330.state.transition_count != state.transition_count
        || mixed_air_cp329.state.transition_count != state.transition_count
        || !route_prefix_matches(state, predecessor)
    {
        return Err(violation("source_owner_predecessor_route_and_system_identity", 1, 0));
    }
    for values in route_arrays(state) {
        ensure_public_routes_only(values)?;
    }
    for (index, (&route_count, &assignment_count)) in state
        .predecessor_route_counts
        .iter()
        .zip(&state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts)
        .enumerate()
    {
        ensure_count(
            assignment_count,
            usize::from(ASSIGNMENT_LOGICAL_INDICES.contains(&index)) * route_count,
            "sensible_output_assignment_route_partition",
        )?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let assignments = checked_sum(
        &state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts,
        "assignment_partition_overflow",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| violation("transition_partition_underflow", assignments, transitions))?;
    let sites = assignments
        .checked_mul(8)
        .ok_or_else(|| violation("site_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in base_counts(
        state,
        predecessor,
        timestep_count,
        transitions,
        inactive,
        assignments,
        sites,
    ) {
        ensure_count(actual, expected, field)?;
    }
    for (field, actual) in active_counters(state) {
        ensure_count(actual, assignments, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let owner_input = predecessor_latest
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        .then(|| {
            Some(ActiveInput {
                supply_mass_flow_rate_kg_per_s: supply_flow_cp330
                    .state
                    .latest?
                    .supply_mass_flow_rate_kg_per_s?,
                mixed_air_temperature_c: mixed_air_cp329
                    .state
                    .latest?
                    .mixed_air_temperature_c?,
            })
        })
        .flatten();
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_has_exact_cp419_prefix_and_local_assignment(
        latest,
        predecessor_latest,
        owner_input,
    ) || !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment,
    )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn route_prefix_matches(
    state: &crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState,
    predecessor: &crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState,
) -> bool {
    state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == predecessor.predecessor_guard_body_entry_route_counts
        && state.predecessor_supply_temperature_saturation_assignment_route_counts
            == predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        && state.predecessor_supply_temperature_mixed_air_limit_route_counts
            == predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        && state.predecessor_supply_humidity_ratio_assignment_route_counts
            == predecessor.predecessor_supply_humidity_ratio_assignment_route_counts
        && state.predecessor_supply_enthalpy_assignment_route_counts
            == predecessor.predecessor_supply_enthalpy_assignment_route_counts
        && state.predecessor_dehumidification_guard_else_branch_entry_route_counts
            == predecessor.predecessor_dehumidification_guard_else_branch_entry_route_counts
        && state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts
            == predecessor.dehumidification_guard_else_branch_cp_air_assignment_route_counts
}

fn route_arrays(
    state: &crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState,
) -> [&[usize; 36]; 10] {
    [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.predecessor_supply_temperature_saturation_assignment_route_counts,
        &state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        &state.predecessor_supply_humidity_ratio_assignment_route_counts,
        &state.predecessor_supply_enthalpy_assignment_route_counts,
        &state.predecessor_dehumidification_guard_else_branch_entry_route_counts,
        &state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts,
        &state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts,
    ]
}

fn ensure_public_routes_only(values: &[usize; 36]) -> Result<(), Error> {
    for (index, count) in values.iter().enumerate() {
        if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
            return Err(violation("non_direct_route_count", 0, *count));
        }
    }
    Ok(())
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentLifecycleInvariant { field, expected, actual }
}

include!(
    "cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_validation/counts.rs"
);

#[cfg(test)]
mod tests {
    #[test]
    fn conceptual_cp420_contract_is_54_routes_49_inactive_5_assignments_and_40_sites() {
        assert_eq!((54 - 5, 5, 5 * 8, 10), (49, 5, 40, 10));
    }

    #[test]
    fn hot_validator_uses_only_bounded_cp419_prefix_helper() {
        let source = include_str!(
            "cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_validation.rs"
        );
        let production_source = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(production, _)| production);
        assert!(production_source.contains("snapshot_has_exact_cp419_prefix_and_local_assignment"));
        for forbidden in [
            "private_characterization",
            "snapshot_is_exact(",
            "predecessor_route(",
        ] {
            assert!(!production_source.contains(forbidden), "{forbidden}");
        }
    }
}
