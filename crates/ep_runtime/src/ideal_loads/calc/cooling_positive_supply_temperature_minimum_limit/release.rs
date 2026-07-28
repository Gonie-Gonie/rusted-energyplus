//! Release-bound CP333 Cooling positive-supply temperature minimum limit.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    advance_cooling_positive_supply_temperature_minimum_limit_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_assignment::
    completed_direct_cooling_positive_supply_temperature_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_operands_link_to_retained_prefix, minimum_limit_links_to_temperature_assignment,
    temperature_assignment_snapshots_match_bit_exact,
};
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use prefix_validation::active_operands_link_to_retained_prefix as active_operands_link_to_retained_prefix_for_test;
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_supply_temperature_minimum_limit_transition_fits as next_supply_temperature_minimum_limit_transition_fits_for_test,
    pending_supply_temperature_minimum_limit_state_is_consistent as pending_supply_temperature_minimum_limit_state_is_consistent_for_test,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_supply_temperature_minimum_limit_state_is_consistent,
    next_supply_temperature_minimum_limit_transition_fits,
    pending_supply_temperature_minimum_limit_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release;
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_temperature_minimum_limit_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_temperature_assignment
        .latest
    else {
        return false;
    };
    let active_lineage_is_exact = if snapshot.supply_temperature_minimum_limit_executed {
        let Some(sensible_flow) = unit.calc_cooling_sensible_flow.latest else {
            return false;
        };
        active_operands_link_to_retained_prefix(
            system,
            sensible_flow,
            predecessor,
            snapshot.supply_temperature_before_minimum_limit_c,
            snapshot.minimum_cooling_supply_air_temperature_c,
        )
    } else {
        true
    };

    completed_direct_cooling_positive_supply_temperature_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_positive_supply_temperature_assignment_latest_witness(system.id),
    ) && minimum_limit_links_to_temperature_assignment(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_supply_temperature_minimum_limit_state_is_consistent(
            unit, snapshot, witness,
        )
}

/// Fail-closed CP333 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError {
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
    CoolingPositiveSupplyTemperatureAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingActiveOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidMinimumCoolingSupplyAirTemperature {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_temperature_assignment_transition_count: usize,
        cooling_positive_supply_temperature_minimum_limit_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP333 for the exact direct no-OA release route.
///
/// The left maximum operand is the exact CP332 assignment. The selected typed
/// system owns the right operand; CP318's retained value is lineage evidence
/// only. The source-shaped maximum intentionally preserves its left operand on
/// equality and unordered comparisons.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp332: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError,
> {
    let selected = predecessor_cp332.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness =
        runtime.cooling_positive_supply_temperature_assignment_latest_witness(selected);
    let limit_witness =
        runtime.cooling_positive_supply_temperature_minimum_limit_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
            InitializationNotReady { system: selected },
    )?;
    if predecessor_cp332.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_temperature_assignment
            .latest
            .is_some_and(|latest| {
                temperature_assignment_snapshots_match_bit_exact(latest, predecessor_cp332)
            })
        || !predecessor_witness.is_some_and(|witness| {
            temperature_assignment_snapshots_match_bit_exact(witness, predecessor_cp332)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                CoolingPositiveSupplyTemperatureAssignmentSnapshotMismatch { system: selected },
        );
    }
    if !cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(
        predecessor_cp332,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_supply_temperature_minimum_limit_state_is_consistent(
            unit,
            predecessor_cp332,
            limit_witness,
        )
        || !next_supply_temperature_minimum_limit_transition_fits(unit, predecessor_cp332)
        || !completed_direct_cooling_positive_supply_temperature_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp332,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp332)
        || predecessor_cp332.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = if predecessor_cp332.supply_temperature_assignment_executed {
        let supply_temperature_before_minimum_limit_c =
            predecessor_cp332.supply_temperature_c.ok_or(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let minimum_cooling_supply_air_temperature_c =
            system.minimum_cooling_supply_air_temperature_c;
        if !minimum_cooling_supply_air_temperature_c.is_finite() {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                    InvalidMinimumCoolingSupplyAirTemperature { system: selected },
            );
        }
        let sensible_flow = unit.calc_cooling_sensible_flow.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        if !active_operands_link_to_retained_prefix(
            system,
            sensible_flow,
            predecessor_cp332,
            Some(supply_temperature_before_minimum_limit_c),
            Some(minimum_cooling_supply_air_temperature_c),
        ) {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            );
        }
        Some(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitActiveInput {
                supply_temperature_before_minimum_limit_c,
                minimum_cooling_supply_air_temperature_c,
            },
        )
    } else {
        None
    };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_temperature_minimum_limit_state(
            &mut unit.calc_cooling_positive_supply_temperature_minimum_limit,
            predecessor_cp332,
            active_input,
        )
    };
    runtime.set_cooling_positive_supply_temperature_minimum_limit_latest_witness(
        selected, snapshot,
    );
    debug_assert!(
        cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_temperature_minimum_limit_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_positive_supply_temperature_minimum_limit_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError {
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_positive_supply_temperature_assignment_transition_count: unit
            .calc_cooling_positive_supply_temperature_assignment
            .transition_count,
        cooling_positive_supply_temperature_minimum_limit_transition_count: unit
            .calc_cooling_positive_supply_temperature_minimum_limit
            .transition_count,
    }
}
