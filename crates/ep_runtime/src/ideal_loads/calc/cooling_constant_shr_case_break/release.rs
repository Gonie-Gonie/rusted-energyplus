//! Release-bound CP357 constant-SHR case-break evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot as Snapshot,
    advance_cooling_constant_shr_case_break_state,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::{
    completed_direct_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_is_consistent,
    cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

#[cfg(test)]
pub(in crate::ideal_loads::calc) use prefix_validation::private_active_predecessor_links_to_direct_release;
use prefix_validation::{active_lineage_is_exact, case_break_links_to_predecessor};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_constant_shr_case_break_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_exact as snapshots_match_exact_for_test;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_constant_shr_case_break_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
                predecessor,
                predecessor_witness,
            )
        })
        && cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && case_break_links_to_predecessor(snapshot, predecessor)
        && active_lineage_is_exact(runtime, unit, system, predecessor, snapshot)
        && completed_state_is_consistent(
            unit,
            snapshot,
            witness,
            system.dehumidification_control_type,
        )
}

/// Fail-closed CP357 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingConstantShrCaseBreakError {
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
    CoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_transition_count: usize,
        cooling_constant_shr_case_break_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP357 for the exact direct no-OA release route.
///
/// CP356 latest, its private witness, and CP356's recursive proof are the sole
/// predecessor evidence owners. CP357 reads no CP355/CP329 state and carries
/// no numerical operand, result, arithmetic, comparison, finite, or range
/// gate. Direct `None` completes the switch before line 2227, so the break site
/// is a complete control-flow skip.
pub fn advance_direct_no_oa_calc_cooling_constant_shr_case_break(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp356: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingConstantShrCaseBreakError> {
    let selected = predecessor_cp356.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingConstantShrCaseBreakError::UnknownSystem { system: selected },
    )?;
    let retained_predecessor = unit
        .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit
        .latest;
    let predecessor_witness =
        runtime.cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_latest_witness(selected);
    let case_break_witness = runtime.cooling_constant_shr_case_break_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingConstantShrCaseBreakError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingConstantShrCaseBreakError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingConstantShrCaseBreakError::DehumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingConstantShrCaseBreakError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingConstantShrCaseBreakError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp356.controlled_zone != controlled_zone
        || !cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
            retained_predecessor,
            predecessor_cp356,
        )
        || !predecessor_witness.is_some_and(|witness| {
            cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
                witness,
                predecessor_cp356,
            )
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
        predecessor_cp356,
    ) {
        return Err(
            PurchasedAirCalcCoolingConstantShrCaseBreakError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            case_break_witness,
            system.dehumidification_control_type,
        )
        || !completed_direct_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingConstantShrCaseBreakError::RuntimeStateInvariantViolation {
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
        &unit.calc_cooling_constant_shr_case_break,
        retained_predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingConstantShrCaseBreakError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingConstantShrCaseBreakError::UnknownSystem { system: selected },
        )?;
        advance_cooling_constant_shr_case_break_state(
            &mut unit.calc_cooling_constant_shr_case_break,
            retained_predecessor,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingConstantShrCaseBreakError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime.set_cooling_constant_shr_case_break_latest_witness(selected, snapshot);
    debug_assert!(cooling_constant_shr_case_break_snapshot_is_exact_direct_release(snapshot));
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_constant_shr_case_break_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_constant_shr_case_break_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingConstantShrCaseBreakError {
    PurchasedAirCalcCoolingConstantShrCaseBreakError::
        CoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingConstantShrCaseBreakError {
    PurchasedAirCalcCoolingConstantShrCaseBreakError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_transition_count: unit
            .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit
            .transition_count,
        cooling_constant_shr_case_break_transition_count: unit
            .calc_cooling_constant_shr_case_break
            .transition_count,
    }
}
