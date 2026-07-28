//! Release-bound CP338 Cooling positive-supply capacity-limit `CpAir` assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    advance_cooling_positive_supply_capacity_limit_cp_air_assignment_state,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_guard::completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset, cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_operand_links_to_retained_prefix, capacity_limit_guard_snapshots_match_exact,
    cp_air_assignment_links_to_capacity_limit_guard,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_capacity_limit_cp_air_assignment_state_is_consistent,
    next_capacity_limit_cp_air_assignment_transition_fits,
    pending_capacity_limit_cp_air_assignment_state_is_consistent,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_capacity_limit_cp_air_assignment_transition_fits as next_capacity_limit_cp_air_assignment_transition_fits_for_test,
    pending_capacity_limit_cp_air_assignment_state_is_consistent as pending_capacity_limit_cp_air_assignment_state_is_consistent_for_test,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_capacity_limit_cp_air_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    >,
) -> bool {
    #[rustfmt::skip]
    let predecessor_latest = unit.calc_cooling_positive_supply_capacity_limit_guard.latest;
    let Some(predecessor) = predecessor_latest else {
        return false;
    };
    let predecessor_witness =
        runtime.cooling_positive_supply_capacity_limit_guard_latest_witness(system.id);
    let active_lineage_is_exact = if snapshot.capacity_limit_cp_air_assignment_executed {
        let Some(mixed_air) = unit.calc_cooling_mixed_air_call.latest else {
            return false;
        };
        let Some(mixed_air_witness) =
            runtime.cooling_mixed_air_call_latest_witness(system.id)
        else {
            return false;
        };
        cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
            && active_operand_links_to_retained_prefix(
                predecessor,
                mixed_air,
                mixed_air_witness,
                snapshot.mixed_air_humidity_ratio,
            )
            && completed_direct_cooling_mixed_air_call_is_consistent(
                runtime,
                unit,
                system,
                mixed_air,
                Some(mixed_air_witness),
            )
    } else {
        true
    };

    completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && cp_air_assignment_links_to_capacity_limit_guard(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_capacity_limit_cp_air_assignment_state_is_consistent(
            unit, snapshot, witness,
        )
}

/// Active CP338 release input rejected before mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentInput {
    /// Retained CP329 mixed-air humidity ratio.
    MixedAirHumidityRatio,
    /// Canonical `PsyCpAirFnW` scalar result.
    PsychrometricCpAir,
}

/// Fail-closed CP338 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError {
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
    CoolingPositiveSupplyCapacityLimitGuardSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingActiveOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_capacity_limit_guard_transition_count: usize,
        cooling_positive_supply_capacity_limit_cp_air_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    InvalidActiveInput {
        system: IdealLoadsAirSystemId,
        input: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentInput,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP338 for the exact direct no-OA release route.
///
/// The active right-hand side comes only from the retained same-call CP329
/// mixed-air humidity ratio. The source psychrometric last-call cache,
/// sentinel, cache-hit identity, and concurrency lifecycle remain outside
/// this pure scalar assignment.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp337: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError,
> {
    let selected = predecessor_cp337.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness =
        runtime.cooling_positive_supply_capacity_limit_guard_latest_witness(selected);
    let assignment_witness = runtime
        .cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
            InitializationNotReady { system: selected },
    )?;
    if predecessor_cp337.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_capacity_limit_guard
            .latest
            .is_some_and(|latest| {
                capacity_limit_guard_snapshots_match_exact(latest, predecessor_cp337)
            })
        || !predecessor_witness.is_some_and(|witness| {
            capacity_limit_guard_snapshots_match_exact(witness, predecessor_cp337)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                CoolingPositiveSupplyCapacityLimitGuardSnapshotMismatch { system: selected },
        );
    }
    if !cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
        predecessor_cp337,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_capacity_limit_cp_air_assignment_state_is_consistent(
            unit,
            predecessor_cp337,
            assignment_witness,
        )
        || !next_capacity_limit_cp_air_assignment_transition_fits(unit, predecessor_cp337)
        || !completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp337,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp337)
        || predecessor_cp337.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = if predecessor_cp337.capacity_limit_body_entered {
        let mixed_air = unit.calc_cooling_mixed_air_call.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        let mixed_air_witness = runtime.cooling_mixed_air_call_latest_witness(selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        let mixed_air_humidity_ratio = mixed_air.mixed_air_humidity_ratio.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        if !mixed_air_humidity_ratio.is_finite() || mixed_air_humidity_ratio < 0.0 {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                    InvalidActiveInput {
                        system: selected,
                        input:
                            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentInput::
                                MixedAirHumidityRatio,
                    },
            );
        }
        if !active_operand_links_to_retained_prefix(
            predecessor_cp337,
            mixed_air,
            mixed_air_witness,
            Some(mixed_air_humidity_ratio),
        ) || !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
            || !completed_direct_cooling_mixed_air_call_is_consistent(
                runtime,
                unit,
                system,
                mixed_air,
                Some(mixed_air_witness),
            )
        {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            );
        }
        if !energyplus_psy_cp_air_fn_w(mixed_air_humidity_ratio).is_finite() {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                    InvalidActiveInput {
                        system: selected,
                        input:
                            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentInput::
                                PsychrometricCpAir,
                    },
            );
        }
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentActiveInput {
                mixed_air_humidity_ratio,
            },
        )
    } else {
        None
    };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::
                UnknownSystem { system: selected },
        )?;
        advance_cooling_positive_supply_capacity_limit_cp_air_assignment_state(
            &mut unit.calc_cooling_positive_supply_capacity_limit_cp_air_assignment,
            predecessor_cp337,
            active_input,
        )
    };
    runtime
        .set_cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(
            selected, snapshot,
        );
    debug_assert!(
        cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_capacity_limit_cp_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError {
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_positive_supply_capacity_limit_guard_transition_count: unit
            .calc_cooling_positive_supply_capacity_limit_guard
            .transition_count,
        cooling_positive_supply_capacity_limit_cp_air_assignment_transition_count: unit
            .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
            .transition_count,
    }
}
