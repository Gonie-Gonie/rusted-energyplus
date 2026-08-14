//! Release-bound CP425 zero-flow supply-enthalpy assignment.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot as Snapshot,
};
use super::transition::{
    advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_state as advance,
    advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_state_with_validated_route as advance_with_route,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_from_committed_predecessor,
};
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_mixed_air_enthalpy,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_committed_latest_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshots_match_bit_exact,
};

mod error;
mod committed;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentError;
pub(in crate::ideal_loads::calc) use committed::cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_committed_latest_route;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{
    prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact,
};

pub(in crate::ideal_loads::calc) fn cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_snapshot_route(
    snapshot: Snapshot,
) -> Option<super::transition::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRetainedRoute> {
    snapshot_validation::snapshot_route(snapshot)
}

pub(in crate::ideal_loads::calc) fn cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: super::transition::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentRetainedRoute,
) -> bool {
    snapshot_validation::retained_route_matches_snapshot_bounded(snapshot, route)
}

/// Losslessly reconstructs the CP424 predecessor retained in CP425.
#[must_use]
pub fn cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_predecessor_cp424_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp424_snapshot(snapshot)
}

/// Executes CP425 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp424: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentError,
> {
    use PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentError as Error;

    let selected = predecessor_cp424.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_latest_witness(
            selected,
        );
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
    if unit.controlled_zone != Some(predecessor_cp424.controlled_zone)
        || unit
            .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
            .latest
            .is_none_or(|latest| {
                !cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshots_match_bit_exact(
                    latest,
                    predecessor_cp424,
                )
            })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .cooling_supply_mass_flow_positive_guard_else_branch_entry_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshots_match_bit_exact(
        predecessor_cp424,
        predecessor_witness,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_route =
        cooling_supply_mass_flow_positive_guard_else_branch_entry_committed_latest_route(
            unit,
            predecessor_witness,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route =
        cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_route_from_committed_predecessor(
            predecessor_cp424,
            predecessor_route,
        )
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    let mixed_air_enthalpy_j_per_kg = if route.assignment_executed {
        let mixed_air_witness = runtime
            .cooling_mixed_air_call_latest_witness(selected)
            .ok_or(Error::MixedAirEnthalpyOwnerUnavailableOrInconsistent { system: selected })?;
        Some(
            cooling_mixed_air_call_committed_latest_mixed_air_enthalpy(
                unit,
                mixed_air_witness,
            )
            .ok_or(Error::MixedAirEnthalpyOwnerUnavailableOrInconsistent { system: selected })?,
        )
    } else {
        None
    };
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp424.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }

    let state = &unit.calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment;
    let mut next_state = state.clone();
    let snapshot = advance_with_route(
        &mut next_state,
        predecessor_cp424,
        route,
        mixed_air_enthalpy_j_per_kg,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp424, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            post_route,
            &unit.calc_cooling_supply_mass_flow_positive_guard_else_branch_entry,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment = next_state;
    runtime
        .set_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_latest_witness(
            selected,
            Some(snapshot),
        );
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_characterization(
    predecessor: Predecessor,
    mixed_air_enthalpy_j_per_kg: Option<f64>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(
        &mut state,
        predecessor,
        mixed_air_enthalpy_j_per_kg,
    )
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentError {
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentError::
        CoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentError {
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            predecessor_transition_count: unit
                .calc_cooling_supply_mass_flow_positive_guard_else_branch_entry
                .transition_count,
            transition_count: unit
                .calc_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment
                .transition_count,
        }
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
