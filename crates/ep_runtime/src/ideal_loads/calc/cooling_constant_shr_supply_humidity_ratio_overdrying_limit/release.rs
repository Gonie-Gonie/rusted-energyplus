//! Release-bound CP354 constant-SHR supply-humidity-ratio overdrying limit.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::transition::predecessor_snapshots_match_bit_exact;
use super::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot as Snapshot,
    advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit::completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{active_lineage_is_exact, assignment_links_to_predecessor};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release;

#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use prefix_validation::active_operands_from_retained_owners as active_operands_from_retained_owners_for_test;
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use runtime_validation::next_transition_fits as next_transition_fits_for_test;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as snapshots_match_bit_exact_for_test;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_latest_witness(
            system.id,
        );
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            predecessor_snapshots_match_bit_exact(predecessor, predecessor_witness)
        })
        && cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && assignment_links_to_predecessor(snapshot, predecessor)
        && active_lineage_is_exact(runtime, unit, system, predecessor, snapshot)
        && completed_state_is_consistent(
            unit,
            snapshot,
            witness,
            system.dehumidification_control_type,
        )
}

/// Retained owner whose CP354 private active operand failed validation.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperandOwner {
    HumidityRatioBeforeOverdryingLimit,
    Temperature,
    Enthalpy,
}

/// Fail-closed CP354 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError {
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
    DehumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: DehumidificationControlType,
    },
    CoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshotMismatch
    {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_transition_count:
            usize,
        cooling_constant_shr_supply_humidity_ratio_overdrying_limit_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
    CoolingActiveOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidActiveInput {
        system: IdealLoadsAirSystemId,
        owner:
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperandOwner,
    },
}

/// Executes CP354 for the exact direct no-OA release route.
///
/// CP353 is the immediate predecessor and sole private temperature/enthalpy
/// owner. Exact direct release selects `None`, so this API performs none of
/// CP354's six reads/evaluations/writes.
pub fn advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp353: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError> {
    let selected = predecessor_cp353.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
        .latest;
    let predecessor_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_latest_witness(
            selected,
        );
    let assignment_witness = runtime
        .cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::DehumidificationControlTypeOutsideDirectSubset {
                system: selected,
                actual: system.dehumidification_control_type,
            },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp353.controlled_zone != controlled_zone
        || !predecessor_snapshots_match_bit_exact(retained_predecessor, predecessor_cp353)
        || !predecessor_witness.is_some_and(|witness| {
            predecessor_snapshots_match_bit_exact(witness, predecessor_cp353)
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
        predecessor_cp353,
    ) {
        return Err(
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            assignment_witness,
            system.dehumidification_control_type,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    if !next_transition_fits(
        &unit.calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit,
        retained_predecessor,
        None,
    ) {
        return Err(
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_state(
            &mut unit
                .calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit,
            retained_predecessor,
            None,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime.set_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witness(
        selected, snapshot,
    );
    debug_assert!(
        cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witness(
                selected,
            ),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError {
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::
        CoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshotMismatch {
            system,
        }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError {
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_transition_count:
                unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
                    .transition_count,
            cooling_constant_shr_supply_humidity_ratio_overdrying_limit_transition_count:
                unit.calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit
                    .transition_count,
        }
}
