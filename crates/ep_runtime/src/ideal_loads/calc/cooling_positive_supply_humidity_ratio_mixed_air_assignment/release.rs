//! Release-bound CP335 Cooling positive-supply mixed-air humidity-ratio assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    advance_cooling_positive_supply_humidity_ratio_mixed_air_assignment_state,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use prefix_validation::active_operand_links_to_retained_prefix as active_operand_links_to_retained_prefix_for_test;
use prefix_validation::{
    active_operand_links_to_retained_prefix,
    humidity_assignment_links_to_temperature_mixed_air_limit,
    temperature_mixed_air_limit_snapshots_match_bit_exact,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_supply_humidity_ratio_mixed_air_assignment_state_is_consistent,
    next_supply_humidity_ratio_mixed_air_assignment_transition_fits,
    pending_supply_humidity_ratio_mixed_air_assignment_state_is_consistent,
};
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_supply_humidity_ratio_mixed_air_assignment_transition_fits as next_supply_humidity_ratio_mixed_air_assignment_transition_fits_for_test,
    pending_supply_humidity_ratio_mixed_air_assignment_state_is_consistent as pending_supply_humidity_ratio_mixed_air_assignment_state_is_consistent_for_test,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .latest
    else {
        return false;
    };
    let active_lineage_is_exact = if snapshot.supply_humidity_ratio_mixed_air_assignment_executed {
        let Some(mixed_air) = unit.calc_cooling_mixed_air_call.latest else {
            return false;
        };
        let Some(mixed_air_witness) = runtime.cooling_mixed_air_call_latest_witness(system.id)
        else {
            return false;
        };
        cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
            && completed_direct_cooling_mixed_air_call_is_consistent(
                runtime,
                unit,
                system,
                mixed_air,
                Some(mixed_air_witness),
            )
            && active_operand_links_to_retained_prefix(
                predecessor,
                mixed_air,
                mixed_air_witness,
                snapshot.mixed_air_humidity_ratio,
            )
    } else {
        true
    };

    completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id),
    ) && humidity_assignment_links_to_temperature_mixed_air_limit(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_supply_humidity_ratio_mixed_air_assignment_state_is_consistent(
            unit, snapshot, witness,
        )
}

/// Fail-closed CP335 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError {
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
    CoolingPositiveSupplyTemperatureMixedAirLimitSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingActiveOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidMixedAirHumidityRatio {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_temperature_mixed_air_limit_transition_count: usize,
        cooling_positive_supply_humidity_ratio_mixed_air_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP335 for the exact direct no-OA release route.
///
/// The right-hand side is the same-call completed CP329 mixed-air humidity
/// result. Skipped routes never read that operand, and active release admission
/// accepts only finite, nonnegative humidity ratios.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp334: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError,
> {
    let selected = predecessor_cp334.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness =
        runtime.cooling_positive_supply_temperature_mixed_air_limit_latest_witness(selected);
    let assignment_witness = runtime
        .cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
            InitializationNotReady { system: selected },
    )?;
    if predecessor_cp334.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest
            .is_some_and(|latest| {
                temperature_mixed_air_limit_snapshots_match_bit_exact(latest, predecessor_cp334)
            })
        || !predecessor_witness.is_some_and(|witness| {
            temperature_mixed_air_limit_snapshots_match_bit_exact(witness, predecessor_cp334)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                CoolingPositiveSupplyTemperatureMixedAirLimitSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
        predecessor_cp334,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_supply_humidity_ratio_mixed_air_assignment_state_is_consistent(
            unit,
            predecessor_cp334,
            assignment_witness,
        )
        || !next_supply_humidity_ratio_mixed_air_assignment_transition_fits(unit, predecessor_cp334)
        || !completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp334,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp334)
        || predecessor_cp334.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = if predecessor_cp334.supply_temperature_mixed_air_limit_executed {
        let mixed_air = unit.calc_cooling_mixed_air_call.latest.ok_or(
                PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let mixed_air_witness =
                runtime.cooling_mixed_air_call_latest_witness(selected).ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                )?;
        if !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
            || !completed_direct_cooling_mixed_air_call_is_consistent(
                runtime,
                unit,
                system,
                mixed_air,
                Some(mixed_air_witness),
            )
        {
            return Err(
                    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                );
        }
        let mixed_air_humidity_ratio = mixed_air.mixed_air_humidity_ratio.ok_or(
                PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        if !mixed_air_humidity_ratio.is_finite() || mixed_air_humidity_ratio < 0.0 {
            return Err(
                    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                        InvalidMixedAirHumidityRatio { system: selected },
                );
        }
        if !active_operand_links_to_retained_prefix(
            predecessor_cp334,
            mixed_air,
            mixed_air_witness,
            Some(mixed_air_humidity_ratio),
        ) {
            return Err(
                    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                );
        }
        Some(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentActiveInput {
                mixed_air_humidity_ratio,
            },
        )
    } else {
        None
    };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::
                UnknownSystem { system: selected },
        )?;
        advance_cooling_positive_supply_humidity_ratio_mixed_air_assignment_state(
            &mut unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment,
            predecessor_cp334,
            active_input,
        )
    };
    runtime.set_cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
        selected, snapshot,
    );
    debug_assert!(
        cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
                selected,
            ),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError {
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_positive_supply_temperature_mixed_air_limit_transition_count: unit
            .calc_cooling_positive_supply_temperature_mixed_air_limit
            .transition_count,
        cooling_positive_supply_humidity_ratio_mixed_air_assignment_transition_count: unit
            .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
            .transition_count,
    }
}
