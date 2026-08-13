//! Release-bound CP424 positive-supply guard else-branch entry.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Snapshot,
};
use super::transition::{
    advance_cooling_supply_mass_flow_positive_guard_else_branch_entry_state as advance,
    advance_cooling_supply_mass_flow_positive_guard_else_branch_entry_state_with_validated_route as advance_with_route,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_route_from_committed_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshots_match_bit_exact,
};

mod error;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryError;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact};

/// Losslessly reconstructs the CP423 predecessor retained in CP424.
#[must_use]
pub fn cooling_supply_mass_flow_positive_guard_else_branch_entry_predecessor_cp423_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp423_snapshot(snapshot)
}

/// Executes CP424 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard_else_branch_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp423: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryError> {
    use PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryError as Error;

    let selected = predecessor_cp423.system;
    let unit = runtime.units.get(&selected).ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.cooling_supply_mass_flow_positive_guard_else_branch_entry_latest_witness(selected);
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
    if unit.controlled_zone != Some(predecessor_cp423.controlled_zone)
        || unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .latest
            .is_none_or(|latest| !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshots_match_bit_exact(latest, predecessor_cp423))
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshots_match_bit_exact(predecessor_cp423, predecessor_witness) {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_route =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_committed_latest_route(
            unit,
            predecessor_witness,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route = cooling_supply_mass_flow_positive_guard_else_branch_entry_route_from_committed_predecessor(
        predecessor_cp423,
        predecessor_route,
    )
    .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp423.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }
    let state = &unit.calc_cooling_supply_mass_flow_positive_guard_else_branch_entry;
    let mut next_state = state.clone();
    let snapshot = advance_with_route(&mut next_state, predecessor_cp423, route)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state.latest_route.ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp423, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            post_route,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let unit = runtime.units.get_mut(&selected).ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_supply_mass_flow_positive_guard_else_branch_entry = next_state;
    runtime.set_cooling_supply_mass_flow_positive_guard_else_branch_entry_latest_witness(selected, Some(snapshot));
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_supply_mass_flow_positive_guard_else_branch_entry_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(system: IdealLoadsAirSystemId) -> PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryError {
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryError::CoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryError {
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment.transition_count,
        transition_count: unit.calc_cooling_supply_mass_flow_positive_guard_else_branch_entry.transition_count,
    }
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
