//! Release-bound CP432 heating operating-mode Heat assignment.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Snapshot,
};
use super::transition::{
    advance_heating_operating_mode_heat_assignment_state as advance,
    advance_heating_operating_mode_heat_assignment_state_with_validated_route as advance_with_route,
    heating_operating_mode_heat_assignment_route_from_committed_predecessor,
};
use crate::ideal_loads::calc::heating_mode_guard_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
    heating_mode_guard_snapshots_match_bit_exact,
};

mod error;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcHeatingOperatingModeHeatAssignmentError;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{
    prefix_and_local_shape_match,
    snapshot_is_exact,
    snapshots_match_bit_exact,
};

/// Losslessly reconstructs the CP431 predecessor retained in CP432.
#[must_use]
pub fn heating_operating_mode_heat_assignment_predecessor_cp431_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp431_snapshot(snapshot)
}

/// Executes CP432 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_heating_operating_mode_heat_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp431: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcHeatingOperatingModeHeatAssignmentError> {
    use PurchasedAirCalcHeatingOperatingModeHeatAssignmentError as Error;

    let selected = predecessor_cp431.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.heating_operating_mode_heat_assignment_latest_witness(selected);
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
    if unit.controlled_zone != Some(predecessor_cp431.controlled_zone)
        || unit.calc_heating_mode_guard.latest.is_none_or(|latest| {
            !heating_mode_guard_snapshots_match_bit_exact(latest, predecessor_cp431)
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .heating_mode_guard_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !heating_mode_guard_snapshots_match_bit_exact(predecessor_cp431, predecessor_witness) {
        return Err(predecessor_mismatch(selected));
    }
    let mixed_air_witness = if predecessor_cp431
        .cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_executed
    {
        Some(runtime.cooling_mixed_air_call_latest_witness(selected).ok_or(
            Error::PredecessorMixedAirTemperatureWitnessUnavailableOrInconsistent {
                system: selected,
            },
        )?)
    } else {
        None
    };
    let predecessor_route = heating_mode_guard_committed_latest_route(
        unit,
        predecessor_witness,
        mixed_air_witness,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route = heating_operating_mode_heat_assignment_route_from_committed_predecessor(
        predecessor_cp431,
        predecessor_route,
    )
    .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp431.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = unit.calc_heating_operating_mode_heat_assignment.clone();
    let snapshot = advance_with_route(
        &mut next_state,
        predecessor_cp431,
        predecessor_route,
        route,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp431, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            predecessor_route,
            post_route,
            &unit.calc_heating_mode_guard,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_heating_operating_mode_heat_assignment = next_state;
    runtime.set_heating_operating_mode_heat_assignment_latest_witness(selected, Some(snapshot));
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_heating_operating_mode_heat_assignment_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_operating_mode_heat_assignment_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_operating_mode_heat_assignment_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOperatingModeHeatAssignmentError {
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentError::HeatingModeGuardSnapshotMismatch {
        system,
    }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOperatingModeHeatAssignmentError {
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit.calc_heating_mode_guard.transition_count,
        transition_count: unit
            .calc_heating_operating_mode_heat_assignment
            .transition_count,
    }
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
