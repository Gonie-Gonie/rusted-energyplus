//! Release-bound CP343 Cooling capacity-limit supply-temperature assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_enthalpy_assignment::completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_humidity_ratio_mixed_air_assignment::completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    retained_input_from_prefix, retained_source_owner_lineage_is_exact,
    supply_enthalpy_assignment_snapshots_match_bit_exact,
    supply_temperature_assignment_links_to_predecessors,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_supply_temperature_assignment_state_is_consistent,
    next_supply_temperature_assignment_transition_fits,
    pending_supply_temperature_assignment_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release;

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn next_supply_temperature_assignment_transition_fits_for_test(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let active = predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let cp334 = active
        .then_some(
            unit.calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest,
        )
        .flatten();
    let cp335 = active
        .then_some(
            unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
                .latest,
        )
        .flatten();
    next_supply_temperature_assignment_transition_fits(
        unit,
        predecessor,
        retained_input_from_prefix(predecessor, cp334, cp335),
    )
}

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
            system.id,
        );
    let active = predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let cp334 = active
        .then_some(
            unit.calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest,
        )
        .flatten();
    let cp334_witness = active
        .then(|| {
            runtime.cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)
        })
        .flatten();
    let cp335 = active
        .then_some(
            unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
                .latest,
        )
        .flatten();
    let cp335_witness = active
        .then(|| {
            runtime.cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
                system.id,
            )
        })
        .flatten();
    let cp336 = active
        .then_some(unit.calc_cooling_positive_supply_enthalpy_assignment.latest)
        .flatten();
    let cp336_witness = active
        .then(|| runtime.cooling_positive_supply_enthalpy_assignment_latest_witness(system.id))
        .flatten();
    let owners_complete = if active {
        let (
            Some(cp334),
            Some(cp334_witness),
            Some(cp335),
            Some(cp335_witness),
            Some(cp336),
            Some(cp336_witness),
        ) = (
            cp334,
            cp334_witness,
            cp335,
            cp335_witness,
            cp336,
            cp336_witness,
        )
        else {
            return false;
        };
        completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            cp334,
            Some(cp334_witness),
        ) && completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            cp335,
            Some(cp335_witness),
        ) && completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            cp336,
            Some(cp336_witness),
        )
    } else {
        true
    };

    cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
        predecessor,
    ) && completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && retained_source_owner_lineage_is_exact(
        predecessor,
        cp334,
        cp334_witness,
        cp335,
        cp335_witness,
        cp336,
        cp336_witness,
    ) && owners_complete
        && supply_temperature_assignment_links_to_predecessors(
            snapshot,
            predecessor,
            cp334,
            cp335,
        )
        && completed_supply_temperature_assignment_state_is_consistent(unit, snapshot, witness)
}

/// Fail-closed CP343 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError
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
    CoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    RetainedSupplyTemperatureAndHumidityOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_transition_count:
            usize,
        cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP343 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp342:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError,
>{
    let selected = predecessor_cp342.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment
        .latest;
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness(
            selected,
        );
    let assignment_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
            InitializationNotReady { system: selected },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    };
    if predecessor_cp342.controlled_zone != controlled_zone
        || !supply_enthalpy_assignment_snapshots_match_bit_exact(
            retained_predecessor,
            predecessor_cp342,
        )
        || !predecessor_witness.is_some_and(|witness| {
            supply_enthalpy_assignment_snapshots_match_bit_exact(witness, predecessor_cp342)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
        predecessor_cp342,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }

    let active = retained_predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || retained_predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let cp334 = active
        .then_some(
            unit.calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest,
        )
        .flatten();
    let cp334_witness = active
        .then(|| {
            runtime.cooling_positive_supply_temperature_mixed_air_limit_latest_witness(selected)
        })
        .flatten();
    let cp335 = active
        .then_some(
            unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
                .latest,
        )
        .flatten();
    let cp335_witness = active
        .then(|| {
            runtime.cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
                selected,
            )
        })
        .flatten();
    let cp336 = active
        .then_some(unit.calc_cooling_positive_supply_enthalpy_assignment.latest)
        .flatten();
    let cp336_witness = active
        .then(|| runtime.cooling_positive_supply_enthalpy_assignment_latest_witness(selected))
        .flatten();
    if !retained_source_owner_lineage_is_exact(
        retained_predecessor,
        cp334,
        cp334_witness,
        cp335,
        cp335_witness,
        cp336,
        cp336_witness,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                RetainedSupplyTemperatureAndHumidityOperandLineageMismatch {
                    system: selected,
                },
        );
    }
    let owners_complete = if active {
        let (
            Some(cp334),
            Some(cp334_witness),
            Some(cp335),
            Some(cp335_witness),
            Some(cp336),
            Some(cp336_witness),
        ) = (
            cp334,
            cp334_witness,
            cp335,
            cp335_witness,
            cp336,
            cp336_witness,
        )
        else {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                    RetainedSupplyTemperatureAndHumidityOperandLineageMismatch {
                        system: selected,
                    },
            );
        };
        completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            cp334,
            Some(cp334_witness),
        ) && completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            cp335,
            Some(cp335_witness),
        ) && completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            cp336,
            Some(cp336_witness),
        )
    } else {
        true
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_supply_temperature_assignment_state_is_consistent(
            unit,
            retained_predecessor,
            assignment_witness,
        )
        || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
        || !owners_complete
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let retained_input = retained_input_from_prefix(retained_predecessor, cp334, cp335);
    if !next_supply_temperature_assignment_transition_fits(
        unit,
        retained_predecessor,
        retained_input,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
                UnknownSystem { system: selected },
        )?;
        advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_state(
            &mut unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment,
            retained_predecessor,
            retained_input,
        )
    };
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError
{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment
                    .transition_count,
            cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
                    .transition_count,
        }
}
