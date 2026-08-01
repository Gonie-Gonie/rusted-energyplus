//! Release-bound CP379 post-saturation enthalpy assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Snapshot;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_limit_assignment::cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact as cp378_snapshots_match_bit_exact;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError;
use error::{call_order_error, predecessor_mismatch, temperature_owner_mismatch};
use prefix_validation::{
    assignment_links_to_prefix, direct_predecessor_is_retained_and_complete,
    direct_temperature_prefix_and_input,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_supply_enthalpy_post_saturation_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_supply_enthalpy_post_saturation_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    snapshot_is_exact as cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact,
    snapshots_match_bit_exact as cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact,
};

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_enthalpy_post_saturation_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
        .latest
    else {
        return false;
    };
    let Some(predecessor_witness) =
        runtime.cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(system.id)
    else {
        return false;
    };
    let Some((temperature_prefix, _)) =
        direct_temperature_prefix_and_input(runtime, unit, system, predecessor)
    else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && cp378_snapshots_match_bit_exact(predecessor_witness, predecessor)
        && cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && assignment_links_to_prefix(snapshot, predecessor, temperature_prefix)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
}

/// Executes CP379 for the exact direct no-OA release route.
///
/// Active temperature bits come only from the same-call recursively complete
/// CP377 witness, and humidity bits come only from the same-call recursively
/// complete CP378 witness. Skipped routes read neither operand. This checkpoint
/// records source evidence only and does not feed or reconcile numerical output.
pub fn advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp378: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError> {
    let selected = predecessor_cp378.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
        .latest;
    let predecessor_witness =
        runtime.cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_supply_enthalpy_post_saturation_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp378.controlled_zone != controlled_zone
        || !cp378_snapshots_match_bit_exact(retained_predecessor, predecessor_cp378)
        || !predecessor_witness
            .is_some_and(|value| cp378_snapshots_match_bit_exact(value, predecessor_cp378))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
        predecessor_cp378,
    ) {
        return Err(
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, retained_predecessor) {
        return Err(
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    let Some((temperature_prefix, active_input)) =
        direct_temperature_prefix_and_input(runtime, unit, system, retained_predecessor)
    else {
        return Err(temperature_owner_mismatch(selected));
    };
    if let Some(input) = active_input {
        if !input.supply_temperature_c.is_finite() {
            return Err(
                PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::InvalidSupplyTemperature {
                    system: selected,
                },
            );
        }
        let supply_humidity_ratio = predecessor_cp378
            .resulting_supply_humidity_ratio
            .ok_or_else(|| temperature_owner_mismatch(selected))?;
        if !supply_humidity_ratio.is_finite() || supply_humidity_ratio < 0.0 {
            return Err(
                PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::InvalidSupplyHumidityRatio {
                    system: selected,
                },
            );
        }
        if !energyplus_psy_h_fn_tdb_w(input.supply_temperature_c, supply_humidity_ratio).is_finite()
        {
            return Err(PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::InvalidPsychrometricSupplyEnthalpy {
                system: selected,
            });
        }
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            assignment_witness,
            active_input,
        )
    {
        return Err(
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_supply_enthalpy_post_saturation_assignment,
        retained_predecessor,
        active_input,
    )
    .ok_or(PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    if !assignment_links_to_prefix(snapshot, retained_predecessor, temperature_prefix)
        || !cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::UnknownSystem {
                system: selected,
            },
        );
    };
    unit.calc_cooling_supply_enthalpy_post_saturation_assignment = next_state;
    runtime
        .set_cooling_supply_enthalpy_post_saturation_assignment_latest_witness(selected, snapshot);
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_supply_enthalpy_post_saturation_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_enthalpy_post_saturation_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
