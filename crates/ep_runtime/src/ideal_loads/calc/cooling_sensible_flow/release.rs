//! Release-bound validation for the CP318 cooling sensible-flow calculation.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, OutdoorAirEconomizerType, ZoneId};

use super::{
    PurchasedAirCalcCoolingSensibleFlowInput, PurchasedAirCalcCoolingSensibleFlowSnapshot,
    advance_cooling_sensible_flow_state,
};
use crate::heat_balance::state::ZoneHeatBalanceState;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodySnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
};

mod predecessor_validation;
mod runtime_validation;
mod snapshot_validation;

use predecessor_validation::economizer_body_links_to_condition;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_sensible_flow,
    pending_sensible_flow_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_sensible_flow_snapshot_is_exact_direct_release;

/// Active CP318 input rejected before mutation because it is not finite.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingSensibleFlowActiveInput {
    /// Controlled Zone humidity ratio.
    ZoneHumidityRatio,
    /// Controlled Zone mean air temperature.
    ZoneTemperature,
    /// Model minimum cooling supply-air temperature.
    MinimumCoolingSupplyAirTemperature,
    /// Retained CP310 cooling setpoint demand.
    CoolingSetpointDemand,
}

/// Fail-closed error before the bounded calculation mutates CP318 state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSensibleFlowError {
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
    /// The current model system disagrees with CP317.
    SystemIdentityMismatch {
        /// Runtime-selected system.
        expected: IdealLoadsAirSystemId,
        /// Caller-supplied model system.
        actual: IdealLoadsAirSystemId,
    },
    /// The supplied Zone state disagrees with retained initialization.
    ZoneIdentityMismatch {
        /// Retained controlled Zone.
        expected: ZoneId,
        /// Caller-supplied Zone.
        actual: ZoneId,
    },
    /// The current system lies outside the direct no-OA sensible subset.
    SystemOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The supplied CP317 snapshot is not the retained latest snapshot.
    CoolingEconomizerBodySnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP316 and CP317 do not describe one source invocation.
    PredecessorLinkMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310 through CP318 are not in one-for-one source order.
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
    },
    /// CP317 does not have an exact direct-release route.
    PredecessorOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// A source-active input is not finite.
    NonFiniteActiveInput {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
        /// Rejected input site.
        input: PurchasedAirCalcCoolingSensibleFlowActiveInput,
    },
    /// Retained CP310-through-CP318 state violates exact release partitions.
    RuntimeStateInvariantViolation {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP318 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_sensible_flow(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    zone_state: &ZoneHeatBalanceState,
) -> Result<PurchasedAirCalcCoolingSensibleFlowSnapshot, PurchasedAirCalcCoolingSensibleFlowError> {
    let selected = predecessor.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(PurchasedAirCalcCoolingSensibleFlowError::UnknownSystem { system: selected })?;
    let condition_consumer_latest_witness =
        runtime.cooling_economizer_condition_latest_witness(selected);
    let body_consumer_latest_witness = runtime.cooling_economizer_body_latest_witness(selected);
    let sensible_flow_latest_witness = runtime.cooling_sensible_flow_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !super::super::cooling_economizer_condition::exact_direct_initialization_is_consistent(
        runtime, unit, system,
    ) {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::InitializationNotReady { system: selected },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported()
        || system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
    {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_economizer_body.latest != Some(predecessor) {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::CoolingEconomizerBodySnapshotMismatch {
                system: selected,
            },
        );
    }
    let condition = unit.calc_cooling_economizer_condition.latest.ok_or(
        PurchasedAirCalcCoolingSensibleFlowError::PredecessorLinkMismatch { system: selected },
    )?;
    let guard = unit.calc_cooling_economizer_guard.latest.ok_or(
        PurchasedAirCalcCoolingSensibleFlowError::PredecessorLinkMismatch { system: selected },
    )?;
    let entry = unit.calc_entry.latest.ok_or(
        PurchasedAirCalcCoolingSensibleFlowError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    let expected_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSensibleFlowError::InitializationNotReady { system: selected },
    )?;
    if predecessor.controlled_zone != expected_zone
        || !economizer_body_links_to_condition(predecessor, condition)
    {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::PredecessorLinkMismatch { system: selected },
        );
    }
    if zone_state.zone_id != expected_zone {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::ZoneIdentityMismatch {
                expected: expected_zone,
                actual: zone_state.zone_id,
            },
        );
    }
    if !super::super::cooling_economizer_body::release::body_snapshot_is_exact_direct_release(
        predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_sensible_flow(unit, predecessor) {
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
                predecessor,
                body_consumer_latest_witness,
            )
        || !pending_sensible_flow_state_is_consistent(
            unit,
            predecessor,
            sensible_flow_latest_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingSensibleFlowError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let input = PurchasedAirCalcCoolingSensibleFlowInput {
        cooling_on: entry.cooling_on,
        zone_humidity_ratio: zone_state.air_humidity_ratio,
        minimum_cooling_supply_air_temperature_c: system.minimum_cooling_supply_air_temperature_c,
        zone_temperature_c: zone_state.mean_air_temperature_c,
        zone_cooling_setpoint_load_w: entry.demand.remaining_output_req_to_cool_sp_w,
    };
    if predecessor.predecessor_cooling_body_entered {
        validate_active_input(selected, input)?;
    }

    let snapshot = {
        let unit = runtime
            .units
            .get_mut(&selected)
            .ok_or(PurchasedAirCalcCoolingSensibleFlowError::UnknownSystem { system: selected })?;
        advance_cooling_sensible_flow_state(
            &mut unit.calc_cooling_sensible_flow,
            predecessor,
            input,
        )
    };
    runtime.set_cooling_sensible_flow_latest_witness(selected, snapshot);
    debug_assert!(cooling_sensible_flow_snapshot_is_exact_direct_release(
        snapshot
    ));
    Ok(snapshot)
}

fn validate_active_input(
    system: IdealLoadsAirSystemId,
    input: PurchasedAirCalcCoolingSensibleFlowInput,
) -> Result<(), PurchasedAirCalcCoolingSensibleFlowError> {
    for (site, value) in [
        (
            PurchasedAirCalcCoolingSensibleFlowActiveInput::ZoneHumidityRatio,
            input.zone_humidity_ratio,
        ),
        (
            PurchasedAirCalcCoolingSensibleFlowActiveInput::ZoneTemperature,
            input.zone_temperature_c,
        ),
        (
            PurchasedAirCalcCoolingSensibleFlowActiveInput::MinimumCoolingSupplyAirTemperature,
            input.minimum_cooling_supply_air_temperature_c,
        ),
        (
            PurchasedAirCalcCoolingSensibleFlowActiveInput::CoolingSetpointDemand,
            input.zone_cooling_setpoint_load_w,
        ),
    ] {
        if !value.is_finite() {
            return Err(
                PurchasedAirCalcCoolingSensibleFlowError::NonFiniteActiveInput {
                    system,
                    input: site,
                },
            );
        }
    }
    Ok(())
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSensibleFlowError {
    PurchasedAirCalcCoolingSensibleFlowError::PredecessorCallOrder {
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
    }
}
