//! Release-bound CP341 Cooling sensible-output maximum-capacity assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    advance_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    maximum_capacity_assignment_links_to_guard,
    retained_guard_active_values_are_release_reachable,
    sensible_output_guard_snapshots_match_bit_exact,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_maximum_capacity_assignment_state_is_consistent,
    next_maximum_capacity_assignment_transition_fits,
    pending_maximum_capacity_assignment_state_is_consistent,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::next_maximum_capacity_assignment_transition_fits as next_maximum_capacity_assignment_transition_fits_for_test;
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(system.id);

    cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
        predecessor,
    ) && completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && retained_guard_active_values_are_release_reachable(predecessor)
        && maximum_capacity_assignment_links_to_guard(snapshot, predecessor)
        && completed_maximum_capacity_assignment_state_is_consistent(unit, snapshot, witness)
}

/// Fail-closed CP341 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError
{
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
    CoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    RetainedCoolingSensibleOutputMaximumCapacityLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_capacity_limit_sensible_output_guard_transition_count: usize,
        cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP341 for the exact direct no-OA release route.
///
/// The supplied CP340 snapshot is admission evidence only. The actual
/// predecessor, preserved output, and conditionally read right-hand-side value
/// come only from the same-call retained CP340 latest/private witness pair.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp340:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError,
> {
    let selected = predecessor_cp340.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest;
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(selected);
    let assignment_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
            InitializationNotReady { system: selected },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshotMismatch {
                    system: selected,
                },
        );
    };
    if predecessor_cp340.controlled_zone != controlled_zone
        || !sensible_output_guard_snapshots_match_bit_exact(
            retained_predecessor,
            predecessor_cp340,
        )
        || !predecessor_witness.is_some_and(|witness| {
            sensible_output_guard_snapshots_match_bit_exact(
                witness,
                predecessor_cp340,
            )
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
        predecessor_cp340,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !retained_guard_active_values_are_release_reachable(retained_predecessor) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                RetainedCoolingSensibleOutputMaximumCapacityLineageMismatch {
                    system: selected,
                },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_maximum_capacity_assignment_state_is_consistent(
            unit,
            retained_predecessor,
            assignment_witness,
        )
        || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    if !next_maximum_capacity_assignment_transition_fits(unit, retained_predecessor) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
                UnknownSystem { system: selected },
        )?;
        advance_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_state(
            &mut unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment,
            retained_predecessor,
        )
    };
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError
{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_capacity_limit_sensible_output_guard_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_guard
                    .transition_count,
            cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
                    .transition_count,
        }
}
