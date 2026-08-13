//! Release-bound CP421 sensible-output maximum-capacity guard.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Snapshot,
};
use super::transition::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state as advance,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state_with_validated_route as advance_with_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor,
};
use crate::ideal_loads::calc::{
    cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_latest_route_and_cooling_sensible_output,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact,
};

mod error;
mod committed;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub(in crate::ideal_loads::calc) use committed::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_committed_latest_route_and_assignment_values;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshot_route as cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot_route;
pub(in crate::ideal_loads::calc) use snapshot_validation::retained_route_matches_snapshot_bounded as cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_retained_route_matches_snapshot_bounded;
pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError;
use runtime_validation::{
    pending_state_is_consistent, post_transition_state_is_consistent,
    predecessor_counts_match,
};
#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(
    state: &super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState,
) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
use snapshot_validation::{prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact};

/// Losslessly reconstructs the CP420 predecessor retained in a CP421 snapshot.
#[must_use]
pub fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp420_snapshot(snapshot)
}

/// Executes CP421 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp420: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError as Error;

    let selected = predecessor_cp420.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witness(selected);
    if system.id != selected {
        return Err(Error::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(Error::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(Error::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    if unit.controlled_zone != Some(predecessor_cp420.controlled_zone)
        || unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
            .latest
            .is_none_or(|latest| {
                !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact(
                    latest,
                    predecessor_cp420,
                )
            })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact(
        predecessor_cp420,
        predecessor_witness,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let (committed_route, committed_output) =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_latest_route_and_cooling_sensible_output(
            unit,
            predecessor_witness,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor(
        predecessor_cp420,
        committed_route,
    )
    .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if !pending_state_is_consistent(unit, witness)
        || !pending_predecessor_counts_are_aligned(unit, route)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp420.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }

    let input = if route.active {
        let cooling_sensible_output_w = committed_output
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let cp321_witness = runtime
            .cooling_capacity_zero_flow_reset_latest_witness(selected)
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let cp340_witness = runtime
            .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(selected)
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let maximum_total_cooling_capacity_w =
            cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity(
                unit,
                cp321_witness,
                cp340_witness,
            )
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        Some(ActiveInput {
            cooling_sensible_output_w,
            maximum_total_cooling_capacity_w,
            cp420_cooling_sensible_output_owned_read: true,
            cp321_maximum_total_cooling_capacity_owned_read: true,
            cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
        })
    } else {
        if committed_output.is_some() {
            return Err(Error::RuntimeStateInvariantViolation { system: selected });
        }
        None
    };

    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard;
    let mut next_state = state.clone();
    let snapshot = advance_with_route(&mut next_state, predecessor_cp420, route, input)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_transition_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp420, post_transition_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            post_transition_route,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_latest_witness(
        selected,
        Some(snapshot),
    );
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_characterization(
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let mut state = super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState::new(predecessor.system);
    advance(&mut state, predecessor, active_input)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn pending_predecessor_counts_are_aligned(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    route: super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard;
    let mut next = state.clone();
    let Some(count) = next.predecessor_route_counts[route.logical_index].checked_add(1) else {
        return false;
    };
    let Some(transition_count) = next.transition_count.checked_add(1) else {
        return false;
    };
    next.transition_count = transition_count;
    next.predecessor_route_counts[route.logical_index] = count;
    if route.active {
        let Some(evaluation_count) = next
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count
            .checked_add(1)
        else {
            return false;
        };
        next.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count =
            evaluation_count;
    }
    predecessor_counts_match(
        &next,
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment,
    )
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError::CoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
            .transition_count,
        transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard
            .transition_count,
    }
}
