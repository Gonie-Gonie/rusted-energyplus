//! Cheap coupled validation for CP419 not-dehumidifying `CpAir` evidence.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as OwnerLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryLifecycleSummary as PredecessorLifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_has_exact_cp418_prefix_and_local_assignment,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ASSIGNMENT_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];

// Inactive CP419 routes must not read CP329 owner evidence, so keep this lazy.
#[allow(clippy::unnecessary_lazy_evaluations)]
pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    let owner = output.calculation_cooling_mixed_air_call;
    let owner_operand = snapshot
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        .then(|| owner.mixed_air_humidity_ratio)
        .flatten();

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_has_exact_cp418_prefix_and_local_assignment(
            snapshot,
            predecessor,
            owner_operand,
        )
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp418: &PredecessorLifecycle,
    owner_cp329: &OwnerLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp418.state;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp418.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || predecessor_cp418.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || owner_cp329.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || owner_cp329.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER.len() != 3
        || [state.system, predecessor.system, owner_cp329.state.system]
            .into_iter()
            .any(|system| system != binding.ideal_loads_air_system)
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.predecessor_guard_body_entry_route_counts
        || state.predecessor_supply_temperature_saturation_assignment_route_counts
            != predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        || state.predecessor_supply_temperature_mixed_air_limit_route_counts
            != predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        || state.predecessor_supply_humidity_ratio_assignment_route_counts
            != predecessor.predecessor_supply_humidity_ratio_assignment_route_counts
        || state.predecessor_supply_enthalpy_assignment_route_counts
            != predecessor.predecessor_supply_enthalpy_assignment_route_counts
        || state.predecessor_dehumidification_guard_else_branch_entry_route_counts
            != predecessor.dehumidification_guard_else_branch_entry_route_counts
    {
        return Err(violation("source_owner_predecessor_route_and_system_identity", 1, 0));
    }

    for values in route_arrays(state) {
        ensure_public_routes_only(values)?;
    }
    for (index, (&route_count, &assignment_count)) in state
        .predecessor_route_counts
        .iter()
        .zip(&state.dehumidification_guard_else_branch_cp_air_assignment_route_counts)
        .enumerate()
    {
        let expected = usize::from(ASSIGNMENT_LOGICAL_INDICES.contains(&index)) * route_count;
        ensure_count(
            assignment_count,
            expected,
            "cp_air_assignment_route_partition",
        )?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route_partition_overflow")?;
    let assignments = checked_sum(
        &state.dehumidification_guard_else_branch_cp_air_assignment_route_counts,
        "assignment_partition_overflow",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| violation("transition_partition_underflow", assignments, transitions))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "owner_transition_count",
            owner_cp329.state.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "predecessor_else_entry_count",
            predecessor.dehumidification_guard_else_branch_entry_count,
            state.predecessor_dehumidification_guard_else_branch_entry_count,
        ),
        (
            "predecessor_supply_temperature_saturation_assignment_count",
            predecessor.predecessor_supply_temperature_saturation_assignment_count,
            state.predecessor_supply_temperature_saturation_assignment_count,
        ),
        (
            "predecessor_supply_temperature_saturation_mixed_air_limit_count",
            predecessor.predecessor_supply_temperature_saturation_mixed_air_limit_count,
            state.predecessor_supply_temperature_saturation_mixed_air_limit_count,
        ),
        (
            "predecessor_supply_humidity_ratio_assignment_count",
            predecessor.predecessor_supply_humidity_ratio_assignment_count,
            state.predecessor_supply_humidity_ratio_assignment_count,
        ),
        (
            "predecessor_supply_enthalpy_assignment_count",
            predecessor.predecessor_supply_enthalpy_assignment_count,
            state.predecessor_supply_enthalpy_assignment_count,
        ),
        (
            "cp_air_assignment_count",
            assignments,
            state.dehumidification_guard_else_branch_cp_air_assignment_count,
        ),
        (
            "source_site_execution_count",
            assignments
                .checked_mul(3)
                .ok_or_else(|| violation("site_count_overflow", 0, usize::MAX))?,
            state.source_site_execution_count,
        ),
        (
            "cp419_owner_count",
            assignments,
            state.cp419_psychrometric_cp_air_state_owner_count,
        ),
        (
            "cp329_owner_read_count",
            assignments,
            state.cp329_retained_mixed_air_humidity_ratio_owned_read_count,
        ),
        (
            "mixed_air_humidity_ratio_read_count",
            assignments,
            state.mixed_air_humidity_ratio_for_cp_air_read_count,
        ),
        (
            "psychrometric_cp_air_evaluation_count",
            assignments,
            state.psychrometric_cp_air_evaluation_count,
        ),
        (
            "cp_air_assignment_write_count",
            assignments,
            state.cp_air_assignment_write_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp417_supply_humidity_ratio_state_owner_count,
            state.cp418_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp417_supply_enthalpy_state_owner_count,
            state.cp418_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp417_supply_temperature_state_owner_count,
            state.cp418_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            predecessor.unchanged_supply_temperature_preservation_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let owner_operand = latest
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        .then(|| owner_cp329.state.latest.and_then(|owner| owner.mixed_air_humidity_ratio))
        .flatten();
    let output_latest = latest_output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_has_exact_cp418_prefix_and_local_assignment(
        latest,
        predecessor_latest,
        owner_operand,
    ) || !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact(
        latest,
        output_latest,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn route_arrays(
    state: &crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState,
) -> [&[usize; 36]; 9] {
    [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.predecessor_supply_temperature_saturation_assignment_route_counts,
        &state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        &state.predecessor_supply_humidity_ratio_assignment_route_counts,
        &state.predecessor_supply_enthalpy_assignment_route_counts,
        &state.predecessor_dehumidification_guard_else_branch_entry_route_counts,
        &state.dehumidification_guard_else_branch_cp_air_assignment_route_counts,
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn conceptual_cp419_contract_is_54_routes_49_inactive_5_assignments_and_15_sites() {
        assert_eq!((54 - 5, 5, 5 * 3, route_arrays_width()), (49, 5, 15, 9));
    }

    fn route_arrays_width() -> usize {
        9
    }
}
