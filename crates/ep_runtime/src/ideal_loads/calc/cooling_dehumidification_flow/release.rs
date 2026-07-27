//! Release-bound validation for the CP319 cooling dehumidification-flow calculation.

use ep_model::{
    DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    OutdoorAirEconomizerType,
};

use super::{
    PurchasedAirCalcCoolingDehumidificationFlowInput,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    advance_cooling_dehumidification_flow_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSensibleFlowSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
};

mod predecessor_validation;
mod runtime_validation;
mod snapshot_validation;

use predecessor_validation::sensible_flow_links_to_economizer_body;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_dehumidification_flow,
    pending_dehumidification_flow_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_dehumidification_flow_snapshot_is_exact_direct_release;

/// Fail-closed error before the bounded calculation mutates CP319 state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingDehumidificationFlowError {
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
    /// The current model system disagrees with CP318.
    SystemIdentityMismatch {
        /// Runtime-selected system.
        expected: IdealLoadsAirSystemId,
        /// Caller-supplied model system.
        actual: IdealLoadsAirSystemId,
    },
    /// The current system lies outside the direct no-OA sensible-only subset.
    SystemOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The supplied CP318 snapshot is not the retained latest snapshot.
    CoolingSensibleFlowSnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP317 and CP318 do not describe one source invocation.
    PredecessorLinkMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310 through CP319 are not in one-for-one source order.
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
        /// Completed CP318 transitions.
        cooling_sensible_flow_transition_count: usize,
        /// Completed CP319 transitions.
        cooling_dehumidification_flow_transition_count: usize,
    },
    /// CP318 does not have an exact direct-release route.
    PredecessorOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Retained CP310-through-CP319 state violates exact release partitions.
    RuntimeStateInvariantViolation {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP319 for the exact direct no-OA sensible-only release route.
///
/// The exact route proves `DehumidificationControlType::None`; consequently
/// this API accepts no moisture-demand or Zone humidity service input.
pub fn advance_direct_no_oa_calc_cooling_dehumidification_flow(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingSensibleFlowSnapshot,
) -> Result<
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowError,
> {
    let selected = predecessor.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingDehumidificationFlowError::UnknownSystem { system: selected },
    )?;
    let condition_consumer_latest_witness =
        runtime.cooling_economizer_condition_latest_witness(selected);
    let body_consumer_latest_witness = runtime.cooling_economizer_body_latest_witness(selected);
    let sensible_flow_consumer_latest_witness =
        runtime.cooling_sensible_flow_latest_witness(selected);
    let dehumidification_flow_latest_witness =
        runtime.cooling_dehumidification_flow_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !super::super::cooling_economizer_condition::exact_direct_initialization_is_consistent(
        runtime, unit, system,
    ) {
        return Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported()
        || system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
        || system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_sensible_flow.latest != Some(predecessor) {
        return Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::CoolingSensibleFlowSnapshotMismatch {
                system: selected,
            },
        );
    }
    let body = unit.calc_cooling_economizer_body.latest.ok_or(
        PurchasedAirCalcCoolingDehumidificationFlowError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let condition = unit.calc_cooling_economizer_condition.latest.ok_or(
        PurchasedAirCalcCoolingDehumidificationFlowError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let guard = unit.calc_cooling_economizer_guard.latest.ok_or(
        PurchasedAirCalcCoolingDehumidificationFlowError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let entry = unit.calc_entry.latest.ok_or(
        PurchasedAirCalcCoolingDehumidificationFlowError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    if unit.controlled_zone != Some(predecessor.controlled_zone)
        || !sensible_flow_links_to_economizer_body(predecessor, body)
    {
        return Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::PredecessorLinkMismatch {
                system: selected,
            },
        );
    }
    if !super::super::cooling_sensible_flow::cooling_sensible_flow_snapshot_is_exact_direct_release(
        predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_dehumidification_flow(unit, predecessor) {
        return Err(call_order_error(unit, selected));
    }
    if !super::super::cooling_economizer_condition::
        completed_direct_prefix_through_economizer_guard_is_consistent(unit, system, guard)
        || !super::super::cooling_economizer_condition::
            completed_direct_economizer_condition_is_consistent(
                unit,
                condition,
                condition_consumer_latest_witness,
            )
        || !super::super::cooling_economizer_body::
            completed_direct_cooling_economizer_body_is_consistent(
                unit,
                condition,
                body,
                body_consumer_latest_witness,
            )
        || !super::super::cooling_sensible_flow::
            completed_direct_cooling_sensible_flow_is_consistent(
                unit,
                body,
                predecessor,
                sensible_flow_consumer_latest_witness,
            )
        || !pending_dehumidification_flow_state_is_consistent(
            unit,
            predecessor,
            dehumidification_flow_latest_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingDehumidificationFlowError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingDehumidificationFlowError::UnknownSystem { system: selected },
        )?;
        advance_cooling_dehumidification_flow_state(
            &mut unit.calc_cooling_dehumidification_flow,
            predecessor,
            PurchasedAirCalcCoolingDehumidificationFlowInput {
                cooling_on: entry.cooling_on,
                dehumidification_control_type: system.dehumidification_control_type,
                zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64::NAN,
                minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN,
                zone_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN,
            },
        )
    };
    runtime.set_cooling_dehumidification_flow_latest_witness(selected, snapshot);
    debug_assert!(cooling_dehumidification_flow_snapshot_is_exact_direct_release(snapshot));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingDehumidificationFlowError {
    PurchasedAirCalcCoolingDehumidificationFlowError::PredecessorCallOrder {
        system,
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
        cooling_sensible_flow_transition_count: unit.calc_cooling_sensible_flow.transition_count,
        cooling_dehumidification_flow_transition_count: unit
            .calc_cooling_dehumidification_flow
            .transition_count,
    }
}
