//! Release-bound validation for the CP317 cooling economizer body.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, OutdoorAirEconomizerType};

use super::{
    PurchasedAirCalcCoolingEconomizerBodyInput, PurchasedAirCalcCoolingEconomizerBodySnapshot,
    advance_cooling_economizer_body_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerConditionSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
};

mod completed_body_validation;
mod entry_prefix_validation;
mod initialization_validation;
mod predecessor_validation;
mod runtime_validation;

use completed_body_validation::completed_body_state_is_consistent;
use entry_prefix_validation::completed_direct_prefix_through_economizer_guard_is_consistent;
use initialization_validation::initialization_state_is_exact_direct_release;
use predecessor_validation::economizer_condition_links_to_guard;
pub(in crate::ideal_loads) use runtime_validation::body_snapshot_is_exact_direct_release;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_body,
    economizer_condition_snapshot_is_exact_direct_release, pending_body_state_is_consistent,
};

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_economizer_body_is_consistent(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    predecessor_condition: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    predecessor_body: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    body_consumer_latest_witness: Option<PurchasedAirCalcCoolingEconomizerBodySnapshot>,
) -> bool {
    completed_body_state_is_consistent(
        unit,
        predecessor_condition,
        predecessor_body,
        body_consumer_latest_witness,
    )
}

/// Fail-closed error before the bounded body mutates CP317 state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingEconomizerBodyError {
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
    /// The current model system disagrees with CP316.
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
    /// The supplied CP316 snapshot is not the retained latest snapshot.
    CoolingEconomizerConditionSnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP315 and CP316 do not describe one source invocation.
    PredecessorLinkMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310 through CP317 are not in one-for-one source order.
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
        /// Completed CP317 transitions.
        cooling_economizer_body_transition_count: usize,
    },
    /// CP316 does not have an exact direct-release skip shape.
    PredecessorOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Retained CP310-through-CP317 state violates exact release partitions.
    RuntimeStateInvariantViolation {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP317 for the exact direct no-OA release route.
///
/// Every public CP316 predecessor is a complete CP317 skip, so this API does
/// not accept Node, psychrometric, load, flow, or timestep values.
pub fn advance_direct_no_oa_calc_cooling_economizer_body(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) -> Result<PurchasedAirCalcCoolingEconomizerBodySnapshot, PurchasedAirCalcCoolingEconomizerBodyError>
{
    let selected = predecessor.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(PurchasedAirCalcCoolingEconomizerBodyError::UnknownSystem { system: selected })?;
    let condition_consumer_latest_witness =
        runtime.cooling_economizer_condition_latest_witness(selected);
    let body_consumer_latest_witness = runtime.cooling_economizer_body_latest_witness(selected);
    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !initialization_state_is_exact_direct_release(runtime, unit, system) {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::InitializationNotReady { system: selected },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported()
        || system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
    {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_economizer_condition.latest != Some(predecessor) {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::
                CoolingEconomizerConditionSnapshotMismatch { system: selected },
        );
    }
    let guard = unit.calc_cooling_economizer_guard.latest.ok_or(
        PurchasedAirCalcCoolingEconomizerBodyError::PredecessorLinkMismatch { system: selected },
    )?;
    if unit.controlled_zone != Some(predecessor.controlled_zone)
        || !economizer_condition_links_to_guard(predecessor, guard)
    {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::PredecessorLinkMismatch {
                system: selected,
            },
        );
    }
    if !economizer_condition_snapshot_is_exact_direct_release(predecessor) {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_body(unit, predecessor) {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::PredecessorCallOrder {
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
                cooling_economizer_body_transition_count: unit
                    .calc_cooling_economizer_body
                    .transition_count,
            },
        );
    }
    if !completed_direct_prefix_through_economizer_guard_is_consistent(unit, system, guard)
        || !super::super::cooling_economizer_condition::
            completed_direct_economizer_condition_is_consistent(
                unit,
                predecessor,
                condition_consumer_latest_witness,
            )
        || !pending_body_state_is_consistent(unit, predecessor, body_consumer_latest_witness)
    {
        return Err(
            PurchasedAirCalcCoolingEconomizerBodyError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingEconomizerBodyError::UnknownSystem { system: selected },
        )?;
        advance_cooling_economizer_body_state(
            &mut unit.calc_cooling_economizer_body,
            predecessor,
            PurchasedAirCalcCoolingEconomizerBodyInput {
                zone_humidity_ratio: f64::NAN,
                outdoor_air_temperature_c: f64::NAN,
                zone_temperature_c: f64::NAN,
                zone_cooling_setpoint_load_w: f64::NAN,
                cooling_limit: system.cooling_limit,
                maximum_cooling_air_mass_flow_rate_kg_per_s: f64::NAN,
                outdoor_air_mass_flow_rate_kg_per_s: f64::NAN,
                system_time_step_hours: f64::NAN,
            },
        )
    };
    runtime.set_cooling_economizer_body_latest_witness(selected, snapshot);
    debug_assert!(!snapshot.economizer_calculation_body_executed);
    debug_assert!(!snapshot.zone_humidity_ratio_read);
    debug_assert!(!snapshot.psychrometric_cp_air_evaluated);
    debug_assert!(!snapshot.outdoor_air_temperature_read);
    debug_assert!(!snapshot.zone_temperature_read);
    debug_assert!(!snapshot.zone_cooling_setpoint_load_read);
    debug_assert!(!snapshot.resulting_supply_mass_flow_rate_read);
    debug_assert!(!snapshot.outdoor_air_mass_flow_rate_read);
    debug_assert!(!snapshot.system_time_step_read);
    debug_assert!(!snapshot.economizer_on_assigned);
    debug_assert!(!snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read);
    debug_assert!(!snapshot.supply_mass_flow_rate_for_outdoor_air_assignment_read);
    Ok(snapshot)
}
