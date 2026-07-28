//! Release-bound CP330 Cooling positive supply-mass-flow guard.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    advance_cooling_supply_mass_flow_positive_guard_state,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset, cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::positive_guard_links_to_mixed_air_call;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::next_positive_guard_transition_fits as next_positive_guard_transition_fits_for_test;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::pending_positive_guard_state_is_consistent as pending_positive_guard_state_is_consistent_for_test;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_positive_guard_state_is_consistent, next_positive_guard_transition_fits,
    pending_positive_guard_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot>,
) -> bool {
    let Some(predecessor) = unit.calc_cooling_mixed_air_call.latest else {
        return false;
    };
    completed_direct_cooling_mixed_air_call_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_mixed_air_call_latest_witness(system.id),
    ) && positive_guard_links_to_mixed_air_call(snapshot, predecessor)
        && completed_positive_guard_state_is_consistent(unit, snapshot, witness)
}

/// Fail-closed CP330 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError {
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
    CoolingMixedAirCallSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_mixed_air_call_transition_count: usize,
        cooling_supply_mass_flow_positive_guard_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP330 for the exact direct no-OA release route.
///
/// This wrapper preserves the source built-in `double > 0.0` behavior. It
/// does not add a finite check, normalization, clamp, numerical-DTO input, or
/// any statement from the body beginning at line 2185.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp329: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError,
> {
    let selected = predecessor_cp329.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::UnknownSystem { system: selected },
    )?;
    let predecessor_witness = runtime.cooling_mixed_air_call_latest_witness(selected);
    let guard_witness = runtime.cooling_supply_mass_flow_positive_guard_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::InitializationNotReady {
            system: selected,
        },
    )?;
    if predecessor_cp329.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_mixed_air_call
            .latest
            .is_some_and(|latest| {
                cooling_mixed_air_call_snapshots_match_bit_exact(latest, predecessor_cp329)
            })
        || !predecessor_witness.is_some_and(|witness| {
            cooling_mixed_air_call_snapshots_match_bit_exact(witness, predecessor_cp329)
        })
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                CoolingMixedAirCallSnapshotMismatch { system: selected },
        );
    }
    if !cooling_mixed_air_call_snapshot_is_exact_direct_release(predecessor_cp329) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_positive_guard_state_is_consistent(unit, predecessor_cp329, guard_witness)
        || !next_positive_guard_transition_fits(unit, predecessor_cp329)
        || !completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp329,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp329)
        || predecessor_cp329.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_mass_flow_positive_guard_state(
            &mut unit.calc_cooling_supply_mass_flow_positive_guard,
            predecessor_cp329,
        )
    };
    runtime.set_cooling_supply_mass_flow_positive_guard_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(positive_guard_links_to_mixed_air_call(
        snapshot,
        predecessor_cp329
    ));
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        super::completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_mass_flow_positive_guard_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError {
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_mixed_air_call_transition_count: unit.calc_cooling_mixed_air_call.transition_count,
        cooling_supply_mass_flow_positive_guard_transition_count: unit
            .calc_cooling_supply_mass_flow_positive_guard
            .transition_count,
    }
}
