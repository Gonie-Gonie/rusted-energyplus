//! Release-bound CP423 sensible-output supply-temperature assignment.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
};
use super::transition::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_state as advance,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_state_with_validated_route as advance_with_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor,
};
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_sensible_output_inputs,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_committed_latest_route_and_assigned_cooling_sensible_output,
    cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact,
};

mod error;
mod committed;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentError;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact};
pub(in crate::ideal_loads::calc) use committed::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_committed_latest_route;

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshot_route(
    snapshot: Snapshot,
) -> Option<super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute> {
    snapshot_validation::snapshot_route(snapshot)
}

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute,
) -> bool {
    snapshot_validation::retained_route_matches_snapshot_bounded(snapshot, route)
}

/// Losslessly reconstructs the CP422 predecessor retained in CP423.
#[must_use]
pub fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_predecessor_cp422_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp422_snapshot(snapshot)
}

/// Executes CP423 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp422: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentError> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentError as Error;

    let selected = predecessor_cp422.system;
    let unit = runtime.units.get(&selected).ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_latest_witness(selected);
    if system.id != selected {
        return Err(Error::SystemIdentityMismatch { expected: selected, actual: system.id });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(Error::DehumidificationControlTypeOutsideDirectSubset { system: selected, actual: system.dehumidification_control_type });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(Error::HumidificationControlTypeOutsideDirectSubset { system: selected, actual: system.humidification_control_type });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    if unit.controlled_zone != Some(predecessor_cp422.controlled_zone)
        || unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment
            .latest
            .is_none_or(|latest| !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact(latest, predecessor_cp422))
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact(predecessor_cp422, predecessor_witness) {
        return Err(predecessor_mismatch(selected));
    }
    let (committed_route, committed_output) =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_committed_latest_route_and_assigned_cooling_sensible_output(
            unit,
            predecessor_witness,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor(
        predecessor_cp422,
        committed_route,
    )
    .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    let input = if route.assignment_executed {
        let mixed_witness = runtime
            .cooling_mixed_air_call_latest_witness(selected)
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let cp329 = cooling_mixed_air_call_committed_latest_sensible_output_inputs(unit, mixed_witness)
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let flow_witness = runtime
            .cooling_supply_mass_flow_positive_guard_latest_witness(selected)
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let supply_mass_flow_rate_kg_per_s =
            cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate(unit, flow_witness, cp329)
                .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let cp419_witness = runtime
            .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_latest_witness(selected)
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let (cp419_route, cp_air) =
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air(unit, cp419_witness)
                .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        if !cp419_route.active || cp419_route.logical_index != route.logical_index {
            return Err(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected });
        }
        Some(ActiveInput {
            mixed_air_temperature_c: cp329.mixed_air_temperature_c,
            cooling_sensible_output_w: committed_output.ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?,
            supply_mass_flow_rate_kg_per_s,
            cp_air_j_per_kg_k: cp_air.ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?,
        })
    } else {
        if committed_output.is_some() {
            return Err(Error::RuntimeStateInvariantViolation { system: selected });
        }
        None
    };
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp422.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment;
    let mut next_state = state.clone();
    let snapshot = advance_with_route(&mut next_state, predecessor_cp422, route, input)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state.latest_route.ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp422, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            post_route,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let unit = runtime.units.get_mut(&selected).ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_latest_witness(selected, Some(snapshot));
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_characterization(
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, input)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshots_match_bit_exact(left: Snapshot, right: Snapshot) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(system: IdealLoadsAirSystemId) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentError::CoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment.transition_count,
        transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment.transition_count,
    }
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
