//! Release-bound CP340 Cooling capacity-limit sensible-output guard.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    advance_cooling_positive_supply_capacity_limit_sensible_output_guard_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_assignment::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_operands_link_to_retained_prefix,
    sensible_output_assignment_snapshots_match_bit_exact,
    sensible_output_guard_links_to_assignment,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_capacity_limit_sensible_output_guard_state_is_consistent,
    next_capacity_limit_sensible_output_guard_transition_fits,
    pending_capacity_limit_sensible_output_guard_state_is_consistent,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_capacity_limit_sensible_output_guard_transition_fits as next_capacity_limit_sensible_output_guard_transition_fits_for_test,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            system.id,
        );
    let active_lineage_is_exact =
        if snapshot.capacity_limit_sensible_output_guard_evaluated {
            let Some(capacity_reset) = unit.calc_cooling_capacity_zero_flow_reset.latest else {
                return false;
            };
            let Some(capacity_reset_witness) =
                runtime.cooling_capacity_zero_flow_reset_latest_witness(system.id)
            else {
                return false;
            };
            let (Some(cooling_sensible_output_w), Some(maximum_total_cooling_capacity_w)) = (
                snapshot.cooling_sensible_output_w,
                snapshot.maximum_total_cooling_capacity_w,
            ) else {
                return false;
            };
            active_operands_link_to_retained_prefix(
                predecessor,
                capacity_reset,
                capacity_reset_witness,
                cooling_sensible_output_w,
                maximum_total_cooling_capacity_w,
            )
        } else {
            true
        };

    cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
        predecessor,
    ) && completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && sensible_output_guard_links_to_assignment(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_capacity_limit_sensible_output_guard_state_is_consistent(
            unit, snapshot, witness,
        )
}

/// Fail-closed CP340 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError {
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
    CoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingCapacityZeroFlowResetOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_capacity_limit_sensible_output_assignment_transition_count:
            usize,
        cooling_positive_supply_capacity_limit_sensible_output_guard_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP340 for the exact direct no-OA release route.
///
/// The active operands come only from the same-call retained CP339 cooling
/// sensible output and CP321 maximum total cooling capacity latest/private
/// witnesses. The wrapper preserves raw built-in `double >= double` behavior.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp339:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError,
> {
    let selected = predecessor_cp339.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            selected,
        );
    let guard_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
            InitializationNotReady { system: selected },
    )?;
    if predecessor_cp339.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
            .latest
            .is_some_and(|latest| {
                sensible_output_assignment_snapshots_match_bit_exact(
                    latest,
                    predecessor_cp339,
                )
            })
        || !predecessor_witness.is_some_and(|witness| {
            sensible_output_assignment_snapshots_match_bit_exact(
                witness,
                predecessor_cp339,
            )
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
        predecessor_cp339,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_capacity_limit_sensible_output_guard_state_is_consistent(
            unit,
            predecessor_cp339,
            guard_witness,
        )
        || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp339,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp339)
        || predecessor_cp339.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input =
        if predecessor_cp339.capacity_limit_sensible_output_assignment_executed {
            let retained_assignment = unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
                .latest
                .ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                        CoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshotMismatch {
                            system: selected,
                        },
                )?;
            let capacity_reset =
                unit.calc_cooling_capacity_zero_flow_reset.latest.ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                        CoolingCapacityZeroFlowResetOperandLineageMismatch {
                            system: selected,
                        },
                )?;
            let capacity_reset_witness = runtime
                .cooling_capacity_zero_flow_reset_latest_witness(selected)
                .ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                        CoolingCapacityZeroFlowResetOperandLineageMismatch {
                            system: selected,
                        },
                )?;
            let cooling_sensible_output_w =
                retained_assignment.cooling_sensible_output_w.ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                        CoolingCapacityZeroFlowResetOperandLineageMismatch {
                            system: selected,
                        },
                )?;
            let maximum_total_cooling_capacity_w = capacity_reset
                .maximum_total_cooling_capacity_w
                .ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                        CoolingCapacityZeroFlowResetOperandLineageMismatch {
                            system: selected,
                        },
                )?;
            if !active_operands_link_to_retained_prefix(
                retained_assignment,
                capacity_reset,
                capacity_reset_witness,
                cooling_sensible_output_w,
                maximum_total_cooling_capacity_w,
            ) {
                return Err(
                    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                        CoolingCapacityZeroFlowResetOperandLineageMismatch {
                            system: selected,
                        },
                );
            }
            Some(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardActiveInput {
                    cooling_sensible_output_w,
                    maximum_total_cooling_capacity_w,
                },
            )
        } else {
            None
        };

    if !next_capacity_limit_sensible_output_guard_transition_fits(
        unit,
        predecessor_cp339,
        active_input,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
                UnknownSystem { system: selected },
        )?;
        advance_cooling_positive_supply_capacity_limit_sensible_output_guard_state(
            &mut unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_guard,
            predecessor_cp339,
            active_input,
        )
    };
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(
            selected, snapshot,
        );
    debug_assert!(
        cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError {
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_capacity_limit_sensible_output_assignment_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
                    .transition_count,
            cooling_positive_supply_capacity_limit_sensible_output_guard_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_guard
                    .transition_count,
        }
}
