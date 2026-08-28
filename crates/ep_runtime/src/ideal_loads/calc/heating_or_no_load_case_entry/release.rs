//! Release-bound CP430 heating-or-no-load case entry.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState as State,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Snapshot,
};
use super::transition::{
    advance_heating_or_no_load_case_entry_state as advance,
    advance_heating_or_no_load_case_entry_state_with_validated_route as advance_with_route,
    heating_or_no_load_case_entry_route_from_committed_predecessor,
};
use crate::ideal_loads::calc::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshots_match_bit_exact,
};

mod error;
mod committed;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcHeatingOrNoLoadCaseEntryError;
pub(in crate::ideal_loads::calc) use committed::heating_or_no_load_case_entry_committed_latest_route;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{
    prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact,
};

/// Losslessly reconstructs the CP429 predecessor retained in CP430.
#[must_use]
pub fn heating_or_no_load_case_entry_predecessor_cp429_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp429_snapshot(snapshot)
}

/// Executes CP430 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_heating_or_no_load_case_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp429: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcHeatingOrNoLoadCaseEntryError> {
    use PurchasedAirCalcHeatingOrNoLoadCaseEntryError as Error;

    let selected = predecessor_cp429.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.heating_or_no_load_case_entry_latest_witness(selected);
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
    if unit.controlled_zone != Some(predecessor_cp429.controlled_zone)
        || unit
            .calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment
            .latest
            .is_none_or(|latest| {
                !cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshots_match_bit_exact(
                    latest,
                    predecessor_cp429,
                )
            })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_latest_witness(
            selected,
        )
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshots_match_bit_exact(
        predecessor_cp429,
        predecessor_witness,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let mixed_air_witness = if predecessor_cp429
        .cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_executed
    {
        Some(
            runtime
                .cooling_mixed_air_call_latest_witness(selected)
                .ok_or(
                    Error::PredecessorMixedAirTemperatureWitnessUnavailableOrInconsistent {
                        system: selected,
                    },
                )?,
        )
    } else {
        None
    };
    let predecessor_route =
        cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_committed_latest_route(
            unit,
            predecessor_witness,
            mixed_air_witness,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route = heating_or_no_load_case_entry_route_from_committed_predecessor(
        predecessor_cp429,
        predecessor_route,
    )
    .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp429.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }

    let state = &unit.calc_heating_or_no_load_case_entry;
    let mut next_state = state.clone();
    let snapshot = advance_with_route(&mut next_state, predecessor_cp429, route)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp429, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            post_route,
            &unit.calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_heating_or_no_load_case_entry = next_state;
    runtime.set_heating_or_no_load_case_entry_latest_witness(selected, Some(snapshot));
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_heating_or_no_load_case_entry_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_or_no_load_case_entry_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_or_no_load_case_entry_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOrNoLoadCaseEntryError {
    PurchasedAirCalcHeatingOrNoLoadCaseEntryError::
        CoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOrNoLoadCaseEntryError {
    PurchasedAirCalcHeatingOrNoLoadCaseEntryError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment
            .transition_count,
        transition_count: unit.calc_heating_or_no_load_case_entry.transition_count,
    }
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
