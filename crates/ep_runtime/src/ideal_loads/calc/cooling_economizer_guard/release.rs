//! Release-bound validation for the CP315 cooling economizer outer guard.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, advance_cooling_economizer_guard_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingOaMaxFlowBodySnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
};

mod validation;

use validation::*;

/// Fail-closed error before the bounded guard mutates CP315 state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingEconomizerGuardError {
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
    /// The current model system disagrees with CP314.
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
    /// The supplied CP314 snapshot is not the retained latest snapshot.
    CoolingOaMaxFlowBodySnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP313 and CP314 do not describe one source invocation.
    PredecessorLinkMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310 through CP315 are not in one-for-one source order.
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
    },
    /// CP314 does not have an exact direct-release skip or fallthrough shape.
    PredecessorOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Retained CP313-through-CP315 state violates exact release partitions.
    RuntimeStateInvariantViolation {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP315 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_economizer_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
) -> Result<
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    PurchasedAirCalcCoolingEconomizerGuardError,
> {
    let selected = predecessor.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(PurchasedAirCalcCoolingEconomizerGuardError::UnknownSystem { system: selected })?;
    let initialization_ready = unit.one_time_latched
        && unit.topology_completed
        && unit.topology_failure.is_none()
        && !unit.sizing_needed
        && unit.sizing_outcome.is_some()
        && unit.environment_initialization_count > 0
        && runtime.equipment_list_checked;
    if !initialization_ready {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    let state_identities_match = unit.system == selected
        && unit.calc_entry.system == selected
        && unit.calc_minimum_oa_prefix.system == selected
        && unit.calc_cooling_entry_gate.system == selected
        && unit.calc_cooling_oa_max_flow_gate.system == selected
        && unit.calc_cooling_oa_max_flow_body.system == selected
        && unit.calc_cooling_economizer_guard.system == selected;
    if !state_identities_match {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if !unit
        .calc_cooling_oa_max_flow_body
        .latest
        .is_some_and(|latest| cooling_oa_max_flow_body_snapshots_bitwise_equal(latest, predecessor))
    {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::CoolingOaMaxFlowBodySnapshotMismatch {
                system: selected,
            },
        );
    }
    let gate = unit.calc_cooling_oa_max_flow_gate.latest.ok_or(
        PurchasedAirCalcCoolingEconomizerGuardError::PredecessorLinkMismatch { system: selected },
    )?;
    if unit.controlled_zone != Some(predecessor.controlled_zone)
        || !predecessor_links_to_gate(predecessor, gate)
    {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::PredecessorLinkMismatch {
                system: selected,
            },
        );
    }

    let ordered = unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_entry.call_count == unit.calc_minimum_oa_prefix.transition_count
        && unit.calc_minimum_oa_prefix.transition_count
            == unit.calc_cooling_entry_gate.transition_count
        && unit.calc_cooling_entry_gate.transition_count
            == unit.calc_cooling_oa_max_flow_gate.transition_count
        && unit.calc_cooling_oa_max_flow_gate.transition_count
            == unit.calc_cooling_oa_max_flow_body.transition_count
        && unit
            .calc_cooling_economizer_guard
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_oa_max_flow_body.transition_count)
        && predecessor.parent_call_ordinal == unit.calc_cooling_oa_max_flow_body.transition_count;
    if !ordered {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::PredecessorCallOrder {
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
            },
        );
    }
    if !predecessor_is_exact_direct_release(predecessor) {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !direct_runtime_states_are_consistent(
        &unit.calc_cooling_economizer_guard,
        &unit.calc_cooling_oa_max_flow_body,
        &unit.calc_cooling_oa_max_flow_gate,
        predecessor,
        gate,
        system.cooling_limit,
        unit.maximum_cooling_air_mass_flow_rate_kg_per_s,
        unit.controlled_zone,
    ) {
        return Err(
            PurchasedAirCalcCoolingEconomizerGuardError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(PurchasedAirCalcCoolingEconomizerGuardError::UnknownSystem { system: selected })?;
    let snapshot = advance_cooling_economizer_guard_state(
        &mut unit.calc_cooling_economizer_guard,
        predecessor,
        system.outdoor_air_economizer_type,
    );
    debug_assert!(!snapshot.economizer_body_entered);
    if snapshot.economizer_guard_evaluated {
        debug_assert_eq!(snapshot.economizer_not_no_economizer, Some(false));
        debug_assert!(snapshot.no_economizer_fallthrough);
    }
    Ok(snapshot)
}
