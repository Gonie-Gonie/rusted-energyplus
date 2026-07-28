//! Release-bound CP331 Cooling positive-supply `CpAir` assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, ZoneId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    advance_cooling_positive_supply_cp_air_assignment_state,
};
use crate::heat_balance::state::ZoneHeatBalanceState;
use crate::ideal_loads::calc::cooling_supply_mass_flow_positive_guard::completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    cp_air_assignment_humidity_links_to_mixed_air, cp_air_assignment_links_to_positive_guard,
    positive_guard_snapshots_match_bit_exact,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::next_cp_air_assignment_transition_fits as next_cp_air_assignment_transition_fits_for_test;
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use runtime_validation::pending_cp_air_assignment_state_is_consistent as pending_cp_air_assignment_state_is_consistent_for_test;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_cp_air_assignment_state_is_consistent, next_cp_air_assignment_transition_fits,
    pending_cp_air_assignment_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot>,
) -> bool {
    let Some(predecessor) = unit.calc_cooling_supply_mass_flow_positive_guard.latest else {
        return false;
    };
    let Some(mixed_air) = unit.calc_cooling_mixed_air_call.latest else {
        return false;
    };
    completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id),
    ) && cp_air_assignment_links_to_positive_guard(snapshot, predecessor)
        && cp_air_assignment_humidity_links_to_mixed_air(snapshot, mixed_air)
        && completed_cp_air_assignment_state_is_consistent(unit, snapshot, witness)
}

/// Active CP331 release input rejected before mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentInput {
    /// Controlled Zone humidity ratio.
    ZoneHumidityRatio,
    /// Canonical `PsyCpAirFnW` scalar result.
    PsychrometricCpAir,
}

/// Fail-closed CP331 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError {
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
    ZoneIdentityMismatch {
        expected: ZoneId,
        actual: ZoneId,
    },
    SystemOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    CoolingSupplyMassFlowPositiveGuardSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingMixedAirHumidityLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_supply_mass_flow_positive_guard_transition_count: usize,
        cooling_positive_supply_cp_air_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    InvalidActiveInput {
        system: IdealLoadsAirSystemId,
        input: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentInput,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP331 for the exact direct no-OA release route.
///
/// The source operand is the live controlled-Zone humidity ratio, not the
/// mixed-air humidity ratio. CP329's direct no-OA copies are used only as
/// bit-exact same-call lineage evidence. The source psychrometric last-call
/// cache, sentinel, cache-hit identity, and concurrency lifecycle remain
/// outside this scalar assignment.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp330: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    zone_state: &ZoneHeatBalanceState,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError,
> {
    let selected = predecessor_cp330.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness =
        runtime.cooling_supply_mass_flow_positive_guard_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_positive_supply_cp_air_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    if zone_state.zone_id != controlled_zone {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::ZoneIdentityMismatch {
                expected: controlled_zone,
                actual: zone_state.zone_id,
            },
        );
    }
    if predecessor_cp330.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_supply_mass_flow_positive_guard
            .latest
            .is_some_and(|latest| {
                positive_guard_snapshots_match_bit_exact(latest, predecessor_cp330)
            })
        || !predecessor_witness.is_some_and(|witness| {
            positive_guard_snapshots_match_bit_exact(witness, predecessor_cp330)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                CoolingSupplyMassFlowPositiveGuardSnapshotMismatch { system: selected },
        );
    }
    if !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(predecessor_cp330)
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_cp_air_assignment_state_is_consistent(
            unit,
            predecessor_cp330,
            assignment_witness,
        )
        || !next_cp_air_assignment_transition_fits(unit, predecessor_cp330)
        || !completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp330,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp330)
        || predecessor_cp330.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = if predecessor_cp330.positive_supply_mass_flow_body_entered {
        let mixed_air = unit.calc_cooling_mixed_air_call.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                CoolingMixedAirHumidityLineageMismatch { system: selected },
        )?;
        let zone_humidity_ratio = zone_state.air_humidity_ratio;
        if !zone_humidity_ratio.is_finite() || zone_humidity_ratio < 0.0 {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::InvalidActiveInput {
                    system: selected,
                    input:
                        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentInput::ZoneHumidityRatio,
                },
            );
        }
        if !mixed_air_humidity_matches_zone_bits(mixed_air, zone_humidity_ratio) {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::
                    CoolingMixedAirHumidityLineageMismatch { system: selected },
            );
        }
        if !energyplus_psy_cp_air_fn_w(zone_humidity_ratio).is_finite() {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::InvalidActiveInput {
                    system: selected,
                    input:
                        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentInput::PsychrometricCpAir,
                },
            );
        }
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentActiveInput {
                zone_humidity_ratio,
            },
        )
    } else {
        None
    };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_cp_air_assignment_state(
            &mut unit.calc_cooling_positive_supply_cp_air_assignment,
            predecessor_cp330,
            active_input,
        )
    };
    runtime.set_cooling_positive_supply_cp_air_assignment_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_positive_supply_cp_air_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn mixed_air_humidity_matches_zone_bits(
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    zone_humidity_ratio: f64,
) -> bool {
    [
        mixed_air.recirculation_humidity_ratio,
        mixed_air.mixed_air_humidity_ratio,
    ]
    .into_iter()
    .all(|value| value.is_some_and(|value| value.to_bits() == zone_humidity_ratio.to_bits()))
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError {
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_mass_flow_positive_guard_transition_count: unit
            .calc_cooling_supply_mass_flow_positive_guard
            .transition_count,
        cooling_positive_supply_cp_air_assignment_transition_count: unit
            .calc_cooling_positive_supply_cp_air_assignment
            .transition_count,
    }
}
