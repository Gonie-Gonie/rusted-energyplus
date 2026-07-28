//! Release-bound CP324 EMS supply-mass-flow override body.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    advance_cooling_supply_mass_flow_ems_override_body_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot, PurchasedAirRuntimeState,
    PurchasedAirSizedLimits, classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::completed_direct_prefix_through_ems_override_guard_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_body, completed_body_state_is_consistent,
    pending_body_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_mass_flow_ems_override_body_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    body: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    body_witness: Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot>,
) -> bool {
    let Some(guard) = unit.calc_cooling_supply_mass_flow_ems_override_guard.latest else {
        return false;
    };

    completed_direct_prefix_through_ems_override_guard_is_consistent(
        runtime,
        unit,
        system,
        guard,
        runtime.cooling_supply_mass_flow_ems_override_guard_latest_witness(system.id),
    ) && body_links_to_guard(body, guard)
        && completed_body_state_is_consistent(unit, body, body_witness)
}

fn body_links_to_guard(
    body: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    guard: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> bool {
    body.system == guard.system
        && body.parent_call_ordinal == guard.parent_call_ordinal
        && body.controlled_zone == guard.controlled_zone
        && body.unit_body_entered == guard.unit_body_entered
        && body.predecessor_cooling_body_entered == guard.cooling_body_entered
        && body.predecessor_ems_supply_mass_flow_override_body_entered
            == guard.ems_supply_mass_flow_override_body_entered
        && body.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough
            == guard.ems_supply_mass_flow_override_guard_false_fallthrough
        && body.unit_off_skipped == guard.unit_off_skipped
        && body.non_cooling_skipped == guard.non_cooling_skipped
        && body.cooling_body_entered == guard.cooling_body_entered
}

/// Fail-closed CP324 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError {
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
    CoolingSupplyMassFlowEmsOverrideGuardSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    SizedLimitsMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_supply_mass_flow_ems_override_guard_transition_count: usize,
        cooling_supply_mass_flow_ems_override_body_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes the CP324 complete skip for the exact direct no-OA/no-EMS route.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp323: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError,
> {
    let selected = predecessor_cp323.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::UnknownSystem {
            system: selected,
        },
    )?;
    let guard_witness =
        runtime.cooling_supply_mass_flow_ems_override_guard_latest_witness(selected);
    let body_witness = runtime.cooling_supply_mass_flow_ems_override_body_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let sized_limits = unit.sized_limits.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::InitializationNotReady {
            system: selected,
        },
    )?;
    if sized_limits != PurchasedAirSizedLimits::from_system(system)
        || unit
            .sizing_outcome
            .is_none_or(|outcome| outcome.sized_limits != sized_limits)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::SizedLimitsMismatch {
                system: selected,
            },
        );
    }
    if !crate::ideal_loads::calc::cooling_economizer_condition::
        exact_direct_initialization_is_consistent(runtime, unit, system)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if unit.calc_cooling_supply_mass_flow_ems_override_guard.latest != Some(predecessor_cp323)
        || guard_witness != Some(predecessor_cp323)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::
                CoolingSupplyMassFlowEmsOverrideGuardSnapshotMismatch { system: selected },
        );
    }
    if !cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(
        predecessor_cp323,
    ) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !call_order_is_pending_body(unit, predecessor_cp323) {
        return Err(call_order_error(unit, selected));
    }
    if !completed_direct_prefix_through_ems_override_guard_is_consistent(
        runtime,
        unit,
        system,
        predecessor_cp323,
        guard_witness,
    ) || !pending_body_state_is_consistent(unit, predecessor_cp323, body_witness)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_mass_flow_ems_override_body_state(
            &mut unit.calc_cooling_supply_mass_flow_ems_override_body,
            predecessor_cp323,
            None,
        )
    };
    runtime.set_cooling_supply_mass_flow_ems_override_body_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(snapshot)
    );
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError {
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_mass_flow_ems_override_guard_transition_count: unit
            .calc_cooling_supply_mass_flow_ems_override_guard
            .transition_count,
        cooling_supply_mass_flow_ems_override_body_transition_count: unit
            .calc_cooling_supply_mass_flow_ems_override_body
            .transition_count,
    }
}
