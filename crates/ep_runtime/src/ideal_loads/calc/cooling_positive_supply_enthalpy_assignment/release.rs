//! Release-bound CP336 Cooling positive-supply enthalpy assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    advance_cooling_positive_supply_enthalpy_assignment_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_humidity_ratio_mixed_air_assignment::completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_operands_link_to_retained_prefix,
    humidity_assignment_snapshots_match_bit_exact,
    supply_enthalpy_assignment_links_to_humidity_assignment,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use prefix_validation::active_operands_link_to_retained_prefix as active_operands_link_to_retained_prefix_for_test;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_supply_enthalpy_assignment_state_is_consistent,
    next_supply_enthalpy_assignment_transition_fits,
    pending_supply_enthalpy_assignment_state_is_consistent,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_supply_enthalpy_assignment_transition_fits as next_supply_enthalpy_assignment_transition_fits_for_test,
    pending_supply_enthalpy_assignment_state_is_consistent as pending_supply_enthalpy_assignment_state_is_consistent_for_test,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .latest
    else {
        return false;
    };
    let active_lineage_is_exact = if snapshot.supply_enthalpy_assignment_executed {
        let Some(temperature_assignment) = unit
            .calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest
        else {
            return false;
        };
        let Some(temperature_witness) = runtime
            .cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)
        else {
            return false;
        };
        cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            temperature_assignment,
        ) && completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            temperature_assignment,
            Some(temperature_witness),
        ) && active_operands_link_to_retained_prefix(
            predecessor,
            temperature_assignment,
            temperature_witness,
            snapshot.supply_temperature_c,
            snapshot.supply_humidity_ratio,
        )
    } else {
        true
    };

    completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime
            .cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(system.id),
    ) && supply_enthalpy_assignment_links_to_humidity_assignment(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_supply_enthalpy_assignment_state_is_consistent(unit, snapshot, witness)
}

/// Fail-closed CP336 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError {
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
    CoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingActiveOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidSupplyTemperature {
        system: IdealLoadsAirSystemId,
    },
    InvalidSupplyHumidityRatio {
        system: IdealLoadsAirSystemId,
    },
    InvalidPsychrometricSupplyEnthalpy {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_humidity_ratio_mixed_air_assignment_transition_count: usize,
        cooling_positive_supply_enthalpy_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP336 for the exact direct no-OA release route.
///
/// Active temperature and humidity operands come only from the same-call
/// completed CP334 and CP335 retained witnesses. Skipped routes do not read
/// either operand.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp335: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError,
> {
    let selected = predecessor_cp335.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness = runtime
        .cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_positive_supply_enthalpy_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    if predecessor_cp335.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
            .latest
            .is_some_and(|latest| {
                humidity_assignment_snapshots_match_bit_exact(latest, predecessor_cp335)
            })
        || !predecessor_witness.is_some_and(|witness| {
            humidity_assignment_snapshots_match_bit_exact(witness, predecessor_cp335)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                CoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
        predecessor_cp335,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_supply_enthalpy_assignment_state_is_consistent(
            unit,
            predecessor_cp335,
            assignment_witness,
        )
        || !next_supply_enthalpy_assignment_transition_fits(unit, predecessor_cp335)
        || !completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp335,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp335)
        || predecessor_cp335.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input =
        if predecessor_cp335.supply_humidity_ratio_mixed_air_assignment_executed {
            let temperature_assignment = unit
                .calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest
                .ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                )?;
            let temperature_witness = runtime
                .cooling_positive_supply_temperature_mixed_air_limit_latest_witness(selected)
                .ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                )?;
            if !cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                temperature_assignment,
            ) || !completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
                runtime,
                unit,
                system,
                temperature_assignment,
                Some(temperature_witness),
            ) {
                return Err(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                );
            }
            let supply_temperature_c = temperature_assignment
                .assigned_supply_temperature_c
                .ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                )?;
            let supply_humidity_ratio = predecessor_cp335
                .assigned_supply_humidity_ratio
                .ok_or(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                )?;
            if !supply_temperature_c.is_finite() {
                return Err(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        InvalidSupplyTemperature { system: selected },
                );
            }
            if !supply_humidity_ratio.is_finite() || supply_humidity_ratio < 0.0 {
                return Err(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        InvalidSupplyHumidityRatio { system: selected },
                );
            }
            let supply_enthalpy_j_per_kg =
                energyplus_psy_h_fn_tdb_w(supply_temperature_c, supply_humidity_ratio);
            if !supply_enthalpy_j_per_kg.is_finite() {
                return Err(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        InvalidPsychrometricSupplyEnthalpy { system: selected },
                );
            }
            if !active_operands_link_to_retained_prefix(
                predecessor_cp335,
                temperature_assignment,
                temperature_witness,
                Some(supply_temperature_c),
                Some(supply_humidity_ratio),
            ) {
                return Err(
                    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::
                        CoolingActiveOperandLineageMismatch { system: selected },
                );
            }
            Some(
                PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentActiveInput {
                    supply_temperature_c,
                    supply_humidity_ratio,
                },
            )
        } else {
            None
        };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_enthalpy_assignment_state(
            &mut unit.calc_cooling_positive_supply_enthalpy_assignment,
            predecessor_cp335,
            active_input,
        )
    };
    runtime.set_cooling_positive_supply_enthalpy_assignment_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_positive_supply_enthalpy_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError {
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_positive_supply_humidity_ratio_mixed_air_assignment_transition_count: unit
            .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
            .transition_count,
        cooling_positive_supply_enthalpy_assignment_transition_count: unit
            .calc_cooling_positive_supply_enthalpy_assignment
            .transition_count,
    }
}
