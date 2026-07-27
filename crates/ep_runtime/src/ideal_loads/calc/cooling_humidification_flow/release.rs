//! Release-bound CP320 cooling humidification-flow calculation.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcCoolingHumidificationFlowInput,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, advance_cooling_humidification_flow_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
};

mod predecessor_validation;
mod runtime_validation;
mod snapshot_validation;

use predecessor_validation::humidification_flow_links_to_dehumidification_flow;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_humidification_flow,
    pending_humidification_flow_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_humidification_flow_snapshot_is_exact_direct_release;

/// Fail-closed CP320 release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingHumidificationFlowError {
    UnknownSystem {
        system: IdealLoadsAirSystemId,
    },
    InitializationNotReady {
        system: IdealLoadsAirSystemId,
    },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    SystemOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    CoolingDehumidificationFlowSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorLinkMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_dehumidification_flow_transition_count: usize,
        cooling_humidification_flow_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP320 without requesting live moisture or Zone-node humidity services.
pub fn advance_direct_no_oa_calc_cooling_humidification_flow(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp319: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) -> Result<
    PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowError,
> {
    let selected = predecessor_cp319.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingHumidificationFlowError::UnknownSystem { system: selected },
    )?;
    let predecessor_witness = runtime.cooling_dehumidification_flow_latest_witness(selected);
    let latest_witness = runtime.cooling_humidification_flow_latest_witness(selected);
    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingHumidificationFlowError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingHumidificationFlowError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported()
        || system.humidification_control_type != HumidificationControlType::None
        || system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(
            PurchasedAirCalcCoolingHumidificationFlowError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingHumidificationFlowError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_dehumidification_flow.latest != Some(predecessor_cp319) {
        return Err(
            PurchasedAirCalcCoolingHumidificationFlowError::
                CoolingDehumidificationFlowSnapshotMismatch { system: selected },
        );
    }
    if !humidification_flow_links_to_dehumidification_flow(predecessor_cp319)
        || predecessor_witness != Some(predecessor_cp319)
    {
        return Err(
            PurchasedAirCalcCoolingHumidificationFlowError::PredecessorLinkMismatch {
                system: selected,
            },
        );
    }
    if !super::super::cooling_dehumidification_flow::
        cooling_dehumidification_flow_snapshot_is_exact_direct_release(predecessor_cp319)
    {
        return Err(
            PurchasedAirCalcCoolingHumidificationFlowError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_humidification_flow(unit, predecessor_cp319) {
        return Err(call_order_error(unit, selected));
    }
    let sensible = unit.calc_cooling_sensible_flow.latest.ok_or(
        PurchasedAirCalcCoolingHumidificationFlowError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    if !super::super::cooling_dehumidification_flow::
        completed_direct_cooling_dehumidification_flow_is_consistent(
            unit,
            sensible,
            predecessor_cp319,
            predecessor_witness,
        )
        || !pending_humidification_flow_state_is_consistent(
            unit,
            predecessor_cp319,
            latest_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingHumidificationFlowError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    let heating_on = unit
        .calc_entry
        .latest
        .ok_or(
            PurchasedAirCalcCoolingHumidificationFlowError::InitializationNotReady {
                system: selected,
            },
        )?
        .heating_on;
    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingHumidificationFlowError::UnknownSystem { system: selected },
        )?;
        advance_cooling_humidification_flow_state(
            &mut unit.calc_cooling_humidification_flow,
            predecessor_cp319,
            PurchasedAirCalcCoolingHumidificationFlowInput {
                heating_on,
                humidification_control_type: system.humidification_control_type,
                dehumidification_control_type: system.dehumidification_control_type,
                zone_humidifying_setpoint_moisture_demand_kg_per_s: f64::NAN,
                maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN,
                zone_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN,
            },
        )
    };
    runtime.set_cooling_humidification_flow_latest_witness(selected, snapshot);
    debug_assert!(cooling_humidification_flow_snapshot_is_exact_direct_release(snapshot));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingHumidificationFlowError {
    PurchasedAirCalcCoolingHumidificationFlowError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_dehumidification_flow_transition_count: unit
            .calc_cooling_dehumidification_flow
            .transition_count,
        cooling_humidification_flow_transition_count: unit
            .calc_cooling_humidification_flow
            .transition_count,
    }
}
