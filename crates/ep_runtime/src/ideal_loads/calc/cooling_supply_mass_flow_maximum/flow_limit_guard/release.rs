//! Release-bound CP325 cooling supply-mass-flow limit guard.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    advance_cooling_supply_mass_flow_limit_guard_state,
};
use crate::ideal_loads::calc::cooling_supply_mass_flow_maximum::ems_override_guard::completed_direct_cooling_supply_mass_flow_ems_override_body_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot, PurchasedAirRuntimeState,
    PurchasedAirSizedLimits, classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::flow_limit_guard_links_to_ems_override_body;
pub(in crate::ideal_loads::calc) use runtime_validation::pending_guard_state_is_consistent;
use runtime_validation::{calc_state_identities_match, call_order_is_pending_guard};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release;

/// Fail-closed CP325 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError {
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
    CoolingSupplyMassFlowEmsOverrideBodySnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    SizedLimitsMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidMaximumCoolingMassFlowCache {
        system: IdealLoadsAirSystemId,
        value_kg_per_s: f64,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_supply_mass_flow_ems_override_body_transition_count: usize,
        cooling_supply_mass_flow_limit_guard_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP325 for the exact direct no-OA/no-EMS route.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp324: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError,
> {
    let selected = predecessor_cp324.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::UnknownSystem { system: selected },
    )?;
    let body_witness = runtime.cooling_supply_mass_flow_ems_override_body_latest_witness(selected);
    let guard_witness = runtime.cooling_supply_mass_flow_limit_guard_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let sized_limits = unit.sized_limits.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::InitializationNotReady {
            system: selected,
        },
    )?;
    if sized_limits != PurchasedAirSizedLimits::from_system(system)
        || unit
            .sizing_outcome
            .is_none_or(|outcome| outcome.sized_limits != sized_limits)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::SizedLimitsMismatch {
                system: selected,
            },
        );
    }
    let maximum_cooling_air_mass_flow_rate_kg_per_s =
        unit.maximum_cooling_air_mass_flow_rate_kg_per_s;
    if !maximum_cooling_air_mass_flow_rate_kg_per_s.is_finite()
        || maximum_cooling_air_mass_flow_rate_kg_per_s < 0.0
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::
                InvalidMaximumCoolingMassFlowCache {
                    system: selected,
                    value_kg_per_s: maximum_cooling_air_mass_flow_rate_kg_per_s,
                },
        );
    }
    if !crate::ideal_loads::calc::cooling_economizer_condition::
        exact_direct_initialization_is_consistent(runtime, unit, system)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_supply_mass_flow_ems_override_body.latest != Some(predecessor_cp324)
        || body_witness != Some(predecessor_cp324)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::
                CoolingSupplyMassFlowEmsOverrideBodySnapshotMismatch { system: selected },
        );
    }
    if !cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(
        predecessor_cp324,
    ) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_guard(unit, predecessor_cp324) {
        return Err(call_order_error(unit, selected));
    }

    if !completed_direct_cooling_supply_mass_flow_ems_override_body_is_consistent(
        runtime,
        unit,
        system,
        predecessor_cp324,
        body_witness,
    ) || !pending_guard_state_is_consistent(
        unit,
        system,
        maximum_cooling_air_mass_flow_rate_kg_per_s,
        predecessor_cp324,
        guard_witness,
    ) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_mass_flow_limit_guard_state(
            &mut unit.calc_cooling_supply_mass_flow_limit_guard,
            predecessor_cp324,
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput {
                cooling_limit: system.cooling_limit,
                maximum_cooling_air_mass_flow_rate_kg_per_s,
            },
        )
    };
    runtime.set_cooling_supply_mass_flow_limit_guard_latest_witness(selected, snapshot);
    debug_assert!(cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(snapshot));
    debug_assert!(flow_limit_guard_links_to_ems_override_body(
        snapshot,
        predecessor_cp324
    ));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError {
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_mass_flow_ems_override_body_transition_count: unit
            .calc_cooling_supply_mass_flow_ems_override_body
            .transition_count,
        cooling_supply_mass_flow_limit_guard_transition_count: unit
            .calc_cooling_supply_mass_flow_limit_guard
            .transition_count,
    }
}
