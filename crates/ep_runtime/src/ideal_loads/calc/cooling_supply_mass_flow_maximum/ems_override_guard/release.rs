//! Release-bound CP323 EMS supply-mass-flow override guard.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    advance_cooling_supply_mass_flow_ems_override_guard_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot, PurchasedAirRuntimeState,
    PurchasedAirSizedLimits, classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::completed_direct_prefix_through_supply_maximum_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_guard, pending_guard_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release;

/// Fail-closed CP323 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError {
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
    CoolingSupplyMassFlowMaximumSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    SizedLimitsMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_supply_mass_flow_maximum_transition_count: usize,
        cooling_supply_mass_flow_ems_override_guard_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes the CP323 guard for the exact direct no-OA/no-EMS release route.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp322: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError,
> {
    let selected = predecessor_cp322.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::UnknownSystem {
            system: selected,
        },
    )?;
    let maximum_witness = runtime.cooling_supply_mass_flow_maximum_latest_witness(selected);
    let guard_witness =
        runtime.cooling_supply_mass_flow_ems_override_guard_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let sized_limits = unit.sized_limits.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::InitializationNotReady {
            system: selected,
        },
    )?;
    if sized_limits != PurchasedAirSizedLimits::from_system(system)
        || unit
            .sizing_outcome
            .is_none_or(|outcome| outcome.sized_limits != sized_limits)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::SizedLimitsMismatch {
                system: selected,
            },
        );
    }
    if !crate::ideal_loads::calc::cooling_economizer_condition::
        exact_direct_initialization_is_consistent(runtime, unit, system)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if unit.calc_cooling_supply_mass_flow_maximum.latest != Some(predecessor_cp322)
        || maximum_witness != Some(predecessor_cp322)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::
                CoolingSupplyMassFlowMaximumSnapshotMismatch { system: selected },
        );
    }
    if !cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(predecessor_cp322) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !call_order_is_pending_guard(unit, predecessor_cp322) {
        return Err(call_order_error(unit, selected));
    }
    if !completed_direct_prefix_through_supply_maximum_is_consistent(
        runtime,
        unit,
        system,
        predecessor_cp322,
        maximum_witness,
    ) || !pending_guard_state_is_consistent(unit, predecessor_cp322, guard_witness)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_mass_flow_ems_override_guard_state(
            &mut unit.calc_cooling_supply_mass_flow_ems_override_guard,
            predecessor_cp322,
            false,
        )
    };
    runtime.set_cooling_supply_mass_flow_ems_override_guard_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(snapshot)
    );
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError {
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_mass_flow_maximum_transition_count: unit
            .calc_cooling_supply_mass_flow_maximum
            .transition_count,
        cooling_supply_mass_flow_ems_override_guard_transition_count: unit
            .calc_cooling_supply_mass_flow_ems_override_guard
            .transition_count,
    }
}
