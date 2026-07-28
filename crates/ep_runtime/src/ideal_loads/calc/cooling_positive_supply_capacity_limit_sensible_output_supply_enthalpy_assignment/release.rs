//! Release-bound CP342 Cooling capacity-limit supply-enthalpy assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    advance_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_assignment::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    maximum_capacity_assignment_snapshots_match_bit_exact,
    retained_cp339_lineage_is_exact, retained_input_from_prefix,
    supply_enthalpy_assignment_links_to_predecessors,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_supply_enthalpy_assignment_state_is_consistent,
    next_supply_enthalpy_assignment_transition_fits,
    pending_supply_enthalpy_assignment_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release;

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn next_supply_enthalpy_assignment_transition_fits_for_test(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    let active = predecessor
        .capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor
            .capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    let cp339 = active
        .then_some(
            unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
                .latest,
        )
        .flatten();
    next_supply_enthalpy_assignment_transition_fits(
        unit,
        predecessor,
        retained_input_from_prefix(predecessor, cp339),
    )
}

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
            system.id,
        );
    let active = predecessor
        .capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor
            .capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    let cp339 = active
        .then_some(
            unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
                .latest,
        )
        .flatten();
    let cp339_witness = active
        .then(|| {
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
                    system.id,
                )
        })
        .flatten();
    let cp339_complete = if active {
        let (Some(cp339), Some(cp339_witness)) = (cp339, cp339_witness) else {
            return false;
        };
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            cp339,
            Some(cp339_witness),
        )
    } else {
        true
    };

    cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
        predecessor,
    ) && completed_direct_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && retained_cp339_lineage_is_exact(predecessor, cp339, cp339_witness)
        && cp339_complete
        && supply_enthalpy_assignment_links_to_predecessors(snapshot, predecessor, cp339)
        && completed_supply_enthalpy_assignment_state_is_consistent(unit, snapshot, witness)
}

/// Fail-closed CP342 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError
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
    CoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    RetainedSensibleOutputAssignmentOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_transition_count:
            usize,
        cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP342 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp341:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError,
> {
    let selected = predecessor_cp341.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
        .latest;
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness(
            selected,
        );
    let assignment_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
            InitializationNotReady { system: selected },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    };
    if predecessor_cp341.controlled_zone != controlled_zone
        || !maximum_capacity_assignment_snapshots_match_bit_exact(
            retained_predecessor,
            predecessor_cp341,
        )
        || !predecessor_witness.is_some_and(|witness| {
            maximum_capacity_assignment_snapshots_match_bit_exact(
                witness,
                predecessor_cp341,
            )
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
        predecessor_cp341,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    let active = retained_predecessor
        .capacity_limit_sensible_output_guard_false_fallthrough
        || retained_predecessor
            .capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    let cp339 = active
        .then_some(
            unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
                .latest,
        )
        .flatten();
    let cp339_witness = active
        .then(|| {
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
                    selected,
                )
        })
        .flatten();
    if !retained_cp339_lineage_is_exact(
        retained_predecessor,
        cp339,
        cp339_witness,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                RetainedSensibleOutputAssignmentOperandLineageMismatch {
                    system: selected,
                },
        );
    }
    let cp339_complete = if active {
        let (Some(cp339), Some(cp339_witness)) = (cp339, cp339_witness) else {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                    RetainedSensibleOutputAssignmentOperandLineageMismatch {
                        system: selected,
                    },
            );
        };
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            cp339,
            Some(cp339_witness),
        )
    } else {
        true
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_supply_enthalpy_assignment_state_is_consistent(
            unit,
            retained_predecessor,
            assignment_witness,
        )
        || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
        || !cp339_complete
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let retained_input = retained_input_from_prefix(retained_predecessor, cp339);
    if !next_supply_enthalpy_assignment_transition_fits(
        unit,
        retained_predecessor,
        retained_input,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
                UnknownSystem { system: selected },
        )?;
        advance_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_state(
            &mut unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment,
            retained_predecessor,
            retained_input,
        )
    };
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError
{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
                    .transition_count,
            cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment
                    .transition_count,
        }
}
