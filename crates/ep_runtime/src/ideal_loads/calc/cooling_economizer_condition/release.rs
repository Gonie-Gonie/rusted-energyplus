//! Release-bound validation for the CP316 cooling economizer condition.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, OutdoorAirEconomizerType};

use super::{
    PurchasedAirCalcCoolingEconomizerConditionInput,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot, advance_cooling_economizer_condition_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
};

mod entry_prefix_validation;
mod initialization_validation;
mod predecessor_validation;
mod runtime_validation;

use entry_prefix_validation::completed_cp310_through_cp313_prefix_is_consistent;
use initialization_validation::initialization_state_is_exact_direct_release;
use predecessor_validation::*;
use runtime_validation::*;

/// Fail-closed error before the bounded condition mutates CP316 state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingEconomizerConditionError {
    /// The selected unit is absent from the persistent arena.
    UnknownSystem {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The selected unit has not completed bounded initialization.
    InitializationNotReady {
        /// Unready typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The current model system disagrees with CP315.
    SystemIdentityMismatch {
        /// Runtime-selected system.
        expected: IdealLoadsAirSystemId,
        /// Caller-supplied model system.
        actual: IdealLoadsAirSystemId,
    },
    /// The current system lies outside the direct no-OA sensible subset.
    SystemOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The supplied CP315 snapshot is not the retained latest snapshot.
    CoolingEconomizerGuardSnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP314 and CP315 do not describe one source invocation.
    PredecessorLinkMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310 through CP316 are not in one-for-one source order.
    PredecessorCallOrder {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
        /// Completed initialization calls.
        init_call_count: usize,
        /// Completed CP310 calls.
        calculation_entry_call_count: usize,
        /// Completed CP311 transitions.
        minimum_oa_prefix_transition_count: usize,
        /// Completed CP312 transitions.
        cooling_entry_gate_transition_count: usize,
        /// Completed CP313 transitions.
        cooling_oa_max_flow_gate_transition_count: usize,
        /// Completed CP314 transitions.
        cooling_oa_max_flow_body_transition_count: usize,
        /// Completed CP315 transitions.
        cooling_economizer_guard_transition_count: usize,
        /// Completed CP316 transitions.
        cooling_economizer_condition_transition_count: usize,
    },
    /// CP315 does not have an exact direct-release skip or fallthrough shape.
    PredecessorOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Retained CP310-through-CP316 state violates exact release partitions.
    RuntimeStateInvariantViolation {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP316 for the exact direct no-OA release route.
///
/// Node values are deliberately absent: CP315 can only reach this wrapper with
/// `NoEconomizer`, so every public CP316 transition is a complete source skip.
pub fn advance_direct_no_oa_calc_cooling_economizer_condition(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> Result<
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingEconomizerConditionError,
> {
    let selected = predecessor.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingEconomizerConditionError::UnknownSystem { system: selected },
    )?;
    let condition_consumer_latest_witness =
        runtime.cooling_economizer_condition_latest_witness(selected);
    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !initialization_state_is_exact_direct_release(runtime, unit, system) {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported()
        || system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
    {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_economizer_guard.latest != Some(predecessor) {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::CoolingEconomizerGuardSnapshotMismatch {
                system: selected,
            },
        );
    }
    let body = unit.calc_cooling_oa_max_flow_body.latest.ok_or(
        PurchasedAirCalcCoolingEconomizerConditionError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    if unit.controlled_zone != Some(predecessor.controlled_zone)
        || !economizer_guard_links_to_body(predecessor, body)
    {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::PredecessorLinkMismatch {
                system: selected,
            },
        );
    }
    if !economizer_guard_snapshot_is_exact_direct_release(predecessor) {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_condition(unit, predecessor) {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::PredecessorCallOrder {
                system: selected,
                init_call_count: unit.init_call_count,
                calculation_entry_call_count: unit.calc_entry.call_count,
                minimum_oa_prefix_transition_count: unit.calc_minimum_oa_prefix.transition_count,
                cooling_entry_gate_transition_count: unit.calc_cooling_entry_gate.transition_count,
                cooling_oa_max_flow_gate_transition_count: unit
                    .calc_cooling_oa_max_flow_gate
                    .transition_count,
                cooling_oa_max_flow_body_transition_count: unit
                    .calc_cooling_oa_max_flow_body
                    .transition_count,
                cooling_economizer_guard_transition_count: unit
                    .calc_cooling_economizer_guard
                    .transition_count,
                cooling_economizer_condition_transition_count: unit
                    .calc_cooling_economizer_condition
                    .transition_count,
            },
        );
    }
    if !completed_cp310_through_cp313_prefix_is_consistent(unit, system)
        || !completed_cp313_through_cp315_prefix_is_consistent(unit, system, predecessor)
        || !pending_condition_state_is_consistent(
            unit,
            predecessor,
            condition_consumer_latest_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingEconomizerConditionError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingEconomizerConditionError::UnknownSystem { system: selected },
        )?;
        advance_cooling_economizer_condition_state(
            &mut unit.calc_cooling_economizer_condition,
            predecessor,
            PurchasedAirCalcCoolingEconomizerConditionInput {
                economizer_type: system.outdoor_air_economizer_type,
                outdoor_air_temperature_c: f64::NAN,
                recirculation_air_temperature_c: f64::NAN,
                outdoor_air_enthalpy_j_per_kg: f64::NAN,
                recirculation_air_enthalpy_j_per_kg: f64::NAN,
            },
        )
    };
    runtime.set_cooling_economizer_condition_latest_witness(selected, snapshot);
    debug_assert!(!snapshot.economizer_condition_evaluated);
    debug_assert!(!snapshot.differential_dry_bulb_economizer_type_read);
    debug_assert!(!snapshot.differential_enthalpy_economizer_type_read);
    debug_assert!(!snapshot.outdoor_air_temperature_read);
    debug_assert!(!snapshot.outdoor_air_enthalpy_read);
    debug_assert!(!snapshot.economizer_calculation_body_entered);
    Ok(snapshot)
}
