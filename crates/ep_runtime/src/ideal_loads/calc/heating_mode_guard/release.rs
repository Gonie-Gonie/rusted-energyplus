//! Release-bound CP431 heating-mode guard.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcHeatingModeGuardRuntimeState as State,
    PurchasedAirCalcHeatingModeGuardSnapshot as Snapshot,
};
use super::transition::{
    PurchasedAirCalcHeatingModeGuardActiveInput as ActiveInput,
    advance_heating_mode_guard_state as advance,
    advance_heating_mode_guard_state_with_validated_route as advance_with_route,
    heating_mode_guard_route_from_committed_predecessor,
};
use crate::ideal_loads::calc::{
    cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands,
    cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type,
    heating_or_no_load_case_entry_committed_latest_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    heating_or_no_load_case_entry_snapshots_match_bit_exact,
};

mod error;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::{
    PurchasedAirCalcHeatingModeGuardError,
    PurchasedAirCalcHeatingModeGuardPredicateInput,
};
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{
    prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact,
};

/// Losslessly reconstructs the CP430 predecessor retained in CP431.
#[must_use]
pub fn heating_mode_guard_predecessor_cp430_snapshot(snapshot: Snapshot) -> Predecessor {
    prefix::predecessor_cp430_snapshot(snapshot)
}

/// Executes CP431 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_heating_mode_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp430: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcHeatingModeGuardError> {
    use PurchasedAirCalcHeatingModeGuardError as Error;
    use PurchasedAirCalcHeatingModeGuardPredicateInput as Predicate;

    let selected = predecessor_cp430.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.heating_mode_guard_latest_witness(selected);
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
    if unit.controlled_zone != Some(predecessor_cp430.controlled_zone)
        || unit.calc_heating_or_no_load_case_entry.latest.is_none_or(|latest| {
            !heating_or_no_load_case_entry_snapshots_match_bit_exact(latest, predecessor_cp430)
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .heating_or_no_load_case_entry_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !heating_or_no_load_case_entry_snapshots_match_bit_exact(
        predecessor_cp430,
        predecessor_witness,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let mixed_air_witness = if predecessor_cp430
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
    let predecessor_route = heating_or_no_load_case_entry_committed_latest_route(
        unit,
        predecessor_witness,
        mixed_air_witness,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let active = predecessor_route.entered;
    let numeric = if active {
        Some(
            cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands(unit)
                .ok_or(Error::HeatingModeGuardInputsUnavailableOrInconsistent {
                    system: selected,
                })?,
        )
    } else {
        None
    };
    if let Some(numeric) = numeric {
        if !numeric.minimum_outdoor_air_sensible_output_w.is_finite() {
            return Err(Error::NonFinitePredicateInput {
                input: Predicate::MinimumOutdoorAirSensibleOutput,
            });
        }
        if !numeric.heating_setpoint_demand_w.is_finite() {
            return Err(Error::NonFinitePredicateInput {
                input: Predicate::HeatingSetpointDemand,
            });
        }
    }
    let first_satisfied = numeric.is_some_and(|numeric| {
        numeric.minimum_outdoor_air_sensible_output_w < numeric.heating_setpoint_demand_w
    });
    let temperature_control_type = if first_satisfied {
        Some(
            cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type(unit)
                .ok_or(Error::HeatingModeGuardInputsUnavailableOrInconsistent {
                    system: selected,
                })?,
        )
    } else {
        None
    };
    let input = numeric.map(|numeric| ActiveInput {
        numeric,
        temperature_control_type,
    });
    let route = heating_mode_guard_route_from_committed_predecessor(
        predecessor_cp430,
        predecessor_route,
        input,
    )
    .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp430.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }
    let state = &unit.calc_heating_mode_guard;
    let mut next_state = state.clone();
    let snapshot = advance_with_route(
        &mut next_state,
        predecessor_cp430,
        predecessor_route,
        input,
        route,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp430, input, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            predecessor_route,
            post_route,
            &unit.calc_heating_or_no_load_case_entry,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_heating_mode_guard = next_state;
    runtime.set_heating_mode_guard_latest_witness(selected, Some(snapshot));
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_heating_mode_guard_characterization(
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, input)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_mode_guard_snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_mode_guard_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(system: IdealLoadsAirSystemId) -> PurchasedAirCalcHeatingModeGuardError {
    PurchasedAirCalcHeatingModeGuardError::HeatingOrNoLoadCaseEntrySnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingModeGuardError {
    PurchasedAirCalcHeatingModeGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit.calc_heating_or_no_load_case_entry.transition_count,
        transition_count: unit.calc_heating_mode_guard.transition_count,
    }
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
#[cfg(test)]
pub(super) fn retained_prior_route_matches_for_test(
    snapshot: Snapshot,
    route: super::transition::PurchasedAirCalcHeatingModeGuardRetainedRoute,
) -> bool {
    snapshot_validation::retained_route_matches_prior_snapshot_bounded(snapshot, route)
}
