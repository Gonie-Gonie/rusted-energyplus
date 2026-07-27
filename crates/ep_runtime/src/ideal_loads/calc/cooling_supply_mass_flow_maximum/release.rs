//! Release-bound CP322 cooling supply-mass-flow maximum.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowMaximumInput,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    advance_cooling_supply_mass_flow_maximum_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot, PurchasedAirRuntimeState,
    PurchasedAirSizedLimits, classify_no_oa_sensible_subset,
};

mod runtime_validation;
mod snapshot_validation;

use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_supply_maximum,
    completed_capacity_zero_reset_is_consistent, pending_supply_maximum_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release;

/// Fail-closed CP322 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowMaximumError {
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
    CoolingCapacityZeroFlowResetSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorLinkMismatch {
        system: IdealLoadsAirSystemId,
    },
    SizedLimitsMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_capacity_zero_flow_reset_transition_count: usize,
        cooling_supply_mass_flow_maximum_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP322 without live service arguments.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp321: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumError,
> {
    let selected = predecessor_cp321.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::UnknownSystem { system: selected },
    )?;
    let condition_witness = runtime.cooling_economizer_condition_latest_witness(selected);
    let body_witness = runtime.cooling_economizer_body_latest_witness(selected);
    let sensible_witness = runtime.cooling_sensible_flow_latest_witness(selected);
    let dehumidification_witness = runtime.cooling_dehumidification_flow_latest_witness(selected);
    let humidification_witness = runtime.cooling_humidification_flow_latest_witness(selected);
    let reset_witness = runtime.cooling_capacity_zero_flow_reset_latest_witness(selected);
    let maximum_witness = runtime.cooling_supply_mass_flow_maximum_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let sized_limits = unit.sized_limits.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::InitializationNotReady {
            system: selected,
        },
    )?;
    if sized_limits != PurchasedAirSizedLimits::from_system(system)
        || unit
            .sizing_outcome
            .is_none_or(|outcome| outcome.sized_limits != sized_limits)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::SizedLimitsMismatch {
                system: selected,
            },
        );
    }
    if !super::super::cooling_economizer_condition::exact_direct_initialization_is_consistent(
        runtime, unit, system,
    ) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_capacity_zero_flow_reset.latest != Some(predecessor_cp321)
        || reset_witness != Some(predecessor_cp321)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::
                CoolingCapacityZeroFlowResetSnapshotMismatch { system: selected },
        );
    }

    let minimum_oa = unit.calc_minimum_oa_prefix.latest.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let guard = unit.calc_cooling_economizer_guard.latest.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let condition = unit.calc_cooling_economizer_condition.latest.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let body = unit.calc_cooling_economizer_body.latest.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let sensible = unit.calc_cooling_sensible_flow.latest.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let dehumidification = unit.calc_cooling_dehumidification_flow.latest.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let humidification = unit.calc_cooling_humidification_flow.latest.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    if !super::super::cooling_capacity_zero_flow_reset::
        cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(predecessor_cp321)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_supply_maximum(unit, predecessor_cp321) {
        return Err(call_order_error(unit, selected));
    }
    if !super::super::cooling_economizer_condition::
        completed_direct_prefix_through_economizer_guard_is_consistent(unit, system, guard)
        || !super::super::cooling_economizer_condition::
            completed_direct_economizer_condition_is_consistent(
                unit,
                condition,
                condition_witness,
            )
        || !super::super::cooling_economizer_body::
            completed_direct_cooling_economizer_body_is_consistent(
                unit,
                condition,
                body,
                body_witness,
            )
        || !super::super::cooling_sensible_flow::
            completed_direct_cooling_sensible_flow_is_consistent(
                unit,
                body,
                sensible,
                sensible_witness,
            )
        || !super::super::cooling_dehumidification_flow::
            completed_direct_cooling_dehumidification_flow_is_consistent(
                unit,
                sensible,
                dehumidification,
                dehumidification_witness,
            )
        || !super::super::cooling_humidification_flow::
            completed_direct_cooling_humidification_flow_is_consistent(
                unit,
                dehumidification,
                humidification,
                humidification_witness,
            )
        || !completed_capacity_zero_reset_is_consistent(
            unit,
            system,
            predecessor_cp321,
            reset_witness,
        )
        || !pending_supply_maximum_state_is_consistent(
            unit,
            predecessor_cp321,
            maximum_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let outdoor_air = minimum_oa
        .working_outdoor_air_mass_flow_rate_kg_per_s
        .filter(|_| predecessor_cp321.cooling_body_entered)
        .unwrap_or(f64::NAN);
    if predecessor_cp321.cooling_body_entered && outdoor_air.to_bits() != 0.0_f64.to_bits() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorLinkMismatch {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyMassFlowMaximumError::UnknownSystem { system: selected },
        )?;
        advance_cooling_supply_mass_flow_maximum_state(
            &mut unit.calc_cooling_supply_mass_flow_maximum,
            predecessor_cp321,
            PurchasedAirCalcCoolingSupplyMassFlowMaximumInput {
                outdoor_air_mass_flow_rate_kg_per_s: outdoor_air,
            },
        )
    };
    runtime.set_cooling_supply_mass_flow_maximum_latest_witness(selected, snapshot);
    debug_assert!(cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(snapshot));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowMaximumError {
    PurchasedAirCalcCoolingSupplyMassFlowMaximumError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_capacity_zero_flow_reset_transition_count: unit
            .calc_cooling_capacity_zero_flow_reset
            .transition_count,
        cooling_supply_mass_flow_maximum_transition_count: unit
            .calc_cooling_supply_mass_flow_maximum
            .transition_count,
    }
}
