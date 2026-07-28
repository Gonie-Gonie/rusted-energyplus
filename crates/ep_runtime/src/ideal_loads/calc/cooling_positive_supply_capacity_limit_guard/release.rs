//! Release-bound CP337 Cooling positive-supply capacity-limit guard.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    advance_cooling_positive_supply_capacity_limit_guard_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_enthalpy_assignment::completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
    cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release,
    cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_cooling_limit_links_to_retained_prefix,
    capacity_limit_guard_links_to_enthalpy_assignment,
    enthalpy_assignment_snapshots_match_bit_exact,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_capacity_limit_guard_state_is_consistent,
    next_capacity_limit_guard_transition_fits,
    pending_capacity_limit_guard_state_is_consistent,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_capacity_limit_guard_transition_fits as next_capacity_limit_guard_transition_fits_for_test,
    pending_capacity_limit_guard_state_is_consistent as pending_capacity_limit_guard_state_is_consistent_for_test,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_enthalpy_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness =
        runtime.cooling_positive_supply_enthalpy_assignment_latest_witness(system.id);
    let active_lineage_is_exact = if snapshot.capacity_limit_guard_evaluated {
        let Some(capacity_reset) = unit.calc_cooling_capacity_zero_flow_reset.latest else {
            return false;
        };
        let Some(flow_limit_guard) = unit.calc_cooling_supply_mass_flow_limit_guard.latest else {
            return false;
        };
        cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(capacity_reset)
            && cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(
                flow_limit_guard,
            )
            && active_cooling_limit_links_to_retained_prefix(
                predecessor,
                capacity_reset,
                flow_limit_guard,
                system.cooling_limit,
            )
    } else {
        true
    };

    completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && capacity_limit_guard_links_to_enthalpy_assignment(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_capacity_limit_guard_state_is_consistent(
            unit, system, snapshot, witness,
        )
}

/// Fail-closed CP337 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError {
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
    CoolingPositiveSupplyEnthalpyAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingLimitLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_enthalpy_assignment_transition_count: usize,
        cooling_positive_supply_capacity_limit_guard_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP337 for the exact direct no-OA release route.
///
/// The active selector comes only from the selected typed system. Skipped
/// routes do not project either source occurrence. This checkpoint reads no
/// capacity value and executes no statement from the body beginning at line
/// 2196.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp336: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError,
> {
    let selected = predecessor_cp336.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness =
        runtime.cooling_positive_supply_enthalpy_assignment_latest_witness(selected);
    let guard_witness =
        runtime.cooling_positive_supply_capacity_limit_guard_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::InitializationNotReady {
            system: selected,
        },
    )?;
    if predecessor_cp336.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_enthalpy_assignment
            .latest
            .is_some_and(|latest| {
                enthalpy_assignment_snapshots_match_bit_exact(latest, predecessor_cp336)
            })
        || !predecessor_witness.is_some_and(|witness| {
            enthalpy_assignment_snapshots_match_bit_exact(witness, predecessor_cp336)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::
                CoolingPositiveSupplyEnthalpyAssignmentSnapshotMismatch { system: selected },
        );
    }
    if !cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
        predecessor_cp336,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_capacity_limit_guard_state_is_consistent(
            unit,
            system,
            predecessor_cp336,
            guard_witness,
        )
        || !next_capacity_limit_guard_transition_fits(
            unit,
            predecessor_cp336,
            system.cooling_limit,
        )
        || !completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp336,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp336)
        || predecessor_cp336.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = if predecessor_cp336.supply_enthalpy_assignment_executed {
        let capacity_reset = unit.calc_cooling_capacity_zero_flow_reset.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::
                CoolingLimitLineageMismatch { system: selected },
        )?;
        let flow_limit_guard = unit.calc_cooling_supply_mass_flow_limit_guard.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::
                CoolingLimitLineageMismatch { system: selected },
        )?;
        if !cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(capacity_reset)
            || !cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(
                flow_limit_guard,
            )
            || !active_cooling_limit_links_to_retained_prefix(
                predecessor_cp336,
                capacity_reset,
                flow_limit_guard,
                system.cooling_limit,
            )
        {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::
                    CoolingLimitLineageMismatch { system: selected },
            );
        }
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardActiveInput {
                cooling_limit: system.cooling_limit,
            },
        )
    } else {
        None
    };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_capacity_limit_guard_state(
            &mut unit.calc_cooling_positive_supply_capacity_limit_guard,
            predecessor_cp336,
            active_input,
        )
    };
    runtime.set_cooling_positive_supply_capacity_limit_guard_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_positive_supply_capacity_limit_guard_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError {
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_positive_supply_enthalpy_assignment_transition_count: unit
            .calc_cooling_positive_supply_enthalpy_assignment
            .transition_count,
        cooling_positive_supply_capacity_limit_guard_transition_count: unit
            .calc_cooling_positive_supply_capacity_limit_guard
            .transition_count,
    }
}
