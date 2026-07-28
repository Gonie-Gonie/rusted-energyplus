//! Release-bound CP344 Cooling capacity-limit supply-temperature mixed-air limit.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_state,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    mixed_air_limit_links_to_predecessor, retained_input_from_prefix,
    retained_source_owner_lineage_is_exact,
    supply_temperature_assignment_snapshots_match_bit_exact,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_supply_temperature_mixed_air_limit_state_is_consistent,
    next_supply_temperature_mixed_air_limit_transition_fits,
    pending_supply_temperature_mixed_air_limit_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release;

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn next_supply_temperature_mixed_air_limit_transition_fits_for_test(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let executed =
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    let mixed_air = executed
        .then_some(unit.calc_cooling_mixed_air_call.latest)
        .flatten();
    next_supply_temperature_mixed_air_limit_transition_fits(
        unit,
        predecessor,
        retained_input_from_prefix(predecessor, mixed_air),
    )
}

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
            system.id,
        );
    let executed =
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    let mixed_air = executed
        .then_some(unit.calc_cooling_mixed_air_call.latest)
        .flatten();
    let mixed_air_witness = executed
        .then(|| runtime.cooling_mixed_air_call_latest_witness(system.id))
        .flatten();
    let owner_complete = if executed {
        let (Some(mixed_air), Some(mixed_air_witness)) = (mixed_air, mixed_air_witness) else {
            return false;
        };
        completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            mixed_air,
            Some(mixed_air_witness),
        )
    } else {
        true
    };

    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
        predecessor,
    ) && completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && retained_source_owner_lineage_is_exact(
        predecessor,
        mixed_air,
        mixed_air_witness,
    ) && owner_complete
        && mixed_air_limit_links_to_predecessor(snapshot, predecessor)
        && completed_supply_temperature_mixed_air_limit_state_is_consistent(
            unit, snapshot, witness,
        )
}

/// Fail-closed CP344 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError
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
    CoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    RetainedMixedAirTemperatureOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidMixedAirTemperature {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_transition_count:
            usize,
        cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP344 for the exact direct no-OA release route.
///
/// CP343's exact resulting `SupplyTemp` owns the left operand. The same-call
/// CP329 latest/private `MixedAirTemp` owns the right operand. The
/// source-shaped two-argument minimum intentionally selects the right operand
/// on equality and unordered comparisons. CP334 is checked only through
/// recursive CP343 completion; its copied temperature never substitutes for
/// either CP344 source operand.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp343:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError,
> {
    let selected = predecessor_cp343.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
        .latest;
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness(
            selected,
        );
    let limit_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
            InitializationNotReady { system: selected },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    };
    if predecessor_cp343.controlled_zone != controlled_zone
        || !supply_temperature_assignment_snapshots_match_bit_exact(
            retained_predecessor,
            predecessor_cp343,
        )
        || !predecessor_witness.is_some_and(|witness| {
            supply_temperature_assignment_snapshots_match_bit_exact(witness, predecessor_cp343)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                CoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
        predecessor_cp343,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }

    let executed = retained_predecessor
        .capacity_limit_sensible_output_supply_temperature_assignment_executed;
    let mixed_air = executed
        .then_some(unit.calc_cooling_mixed_air_call.latest)
        .flatten();
    let mixed_air_witness = executed
        .then(|| runtime.cooling_mixed_air_call_latest_witness(selected))
        .flatten();
    if executed
        && mixed_air
            .and_then(|snapshot| snapshot.mixed_air_temperature_c)
            .is_some_and(|temperature| !temperature.is_finite())
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                InvalidMixedAirTemperature { system: selected },
        );
    }
    if !retained_source_owner_lineage_is_exact(
        retained_predecessor,
        mixed_air,
        mixed_air_witness,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                RetainedMixedAirTemperatureOperandLineageMismatch {
                    system: selected,
                },
        );
    }
    let owner_complete = if executed {
        let (Some(mixed_air), Some(mixed_air_witness)) = (mixed_air, mixed_air_witness) else {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                    RetainedMixedAirTemperatureOperandLineageMismatch {
                        system: selected,
                    },
            );
        };
        completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            mixed_air,
            Some(mixed_air_witness),
        )
    } else {
        true
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_supply_temperature_mixed_air_limit_state_is_consistent(
            unit,
            retained_predecessor,
            limit_witness,
        )
        || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
        || !owner_complete
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let retained_input = retained_input_from_prefix(retained_predecessor, mixed_air);
    if !next_supply_temperature_mixed_air_limit_transition_fits(
        unit,
        retained_predecessor,
        retained_input,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
                UnknownSystem { system: selected },
        )?;
        advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_state(
            &mut unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,
            retained_predecessor,
            retained_input,
        )
    };
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError
{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment
                    .transition_count,
            cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
                    .transition_count,
        }
}
