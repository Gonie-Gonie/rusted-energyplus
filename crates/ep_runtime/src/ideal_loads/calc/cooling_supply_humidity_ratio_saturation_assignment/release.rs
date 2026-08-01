//! Release-bound CP377 saturation-humidity-ratio assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::transition::{predecessor_route, route_is_active};
use super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_pre_saturation_original_assignment::{
    completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshots_match_bit_exact as cp376_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError;
use error::{call_order_error, owner_mismatch, predecessor_mismatch};
use prefix_validation::{
    assignment_links_to_predecessor, direct_predecessor_is_retained_and_complete,
    direct_temperature_owner,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_supply_humidity_ratio_saturation_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_supply_humidity_ratio_saturation_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshot_route as cooling_supply_humidity_ratio_saturation_assignment_snapshot_route;
use snapshot_validation::snapshot_temperature_owner;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact;

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && predecessor_witness
            .is_some_and(|witness| cp376_snapshots_match_bit_exact(predecessor, witness))
        && cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && direct_owner_matches_route(runtime, unit, system, predecessor, snapshot)
        && assignment_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
}

/// Executes CP377 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp376: Predecessor,
    barometric_pressure_pa: f64,
) -> Result<Snapshot, PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError> {
    let selected = predecessor_cp376.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
        .latest;
    let predecessor_witness = runtime
        .cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_supply_humidity_ratio_saturation_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::HumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp376.controlled_zone != controlled_zone
        || !cp376_snapshots_match_bit_exact(retained_predecessor, predecessor_cp376)
        || !predecessor_witness
            .is_some_and(|witness| cp376_snapshots_match_bit_exact(witness, predecessor_cp376))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
        predecessor_cp376,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::PredecessorOutsideDirectSubset {
            system: selected,
        });
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, retained_predecessor) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    let route = predecessor_route(retained_predecessor).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::PredecessorOutsideDirectSubset {
            system: selected,
        },
    )?;
    let input = if route_is_active(route) {
        let (supply_temperature_c, temperature_owner) =
            direct_temperature_owner(runtime, unit, system, retained_predecessor)
                .ok_or(owner_mismatch(selected))?;
        if !supply_temperature_c.is_finite() {
            return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::SupplyTemperatureOutsideDirectSubset {
                system: selected,
                bits: supply_temperature_c.to_bits(),
            });
        }
        if !barometric_pressure_pa.is_finite() || barometric_pressure_pa <= 0.0 {
            return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::BarometricPressureOutsideDirectSubset {
                system: selected,
                bits: barometric_pressure_pa.to_bits(),
            });
        }
        let saturation_humidity_ratio =
            energyplus_psy_w_fn_tdb_rh_pb(supply_temperature_c, 1.0, barometric_pressure_pa);
        if !saturation_humidity_ratio.is_finite() {
            return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::SaturationHumidityRatioOutsideDirectSubset {
                system: selected,
                bits: saturation_humidity_ratio.to_bits(),
            });
        }
        Some(ActiveInput {
            supply_temperature_c,
            temperature_owner,
            outdoor_barometric_pressure_pa: barometric_pressure_pa,
        })
    } else {
        None
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, retained_predecessor, assignment_witness, input)
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_supply_humidity_ratio_saturation_assignment,
        retained_predecessor,
        input,
    )
    .ok_or(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    if !assignment_links_to_predecessor(snapshot, retained_predecessor)
        || !cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(
            PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::UnknownSystem {
                system: selected,
            },
        );
    };
    unit.calc_cooling_supply_humidity_ratio_saturation_assignment = next_state;
    runtime
        .set_cooling_supply_humidity_ratio_saturation_assignment_latest_witness(selected, snapshot);
    Ok(snapshot)
}

fn direct_owner_matches_route(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    snapshot: Snapshot,
) -> bool {
    predecessor_route(predecessor).is_some_and(|route| {
        if !route_is_active(route) {
            return snapshot
                .supply_temperature_for_saturation_humidity_ratio_c
                .is_none()
                && snapshot.outdoor_barometric_pressure_pa.is_none();
        }
        direct_temperature_owner(runtime, unit, system, predecessor).is_some_and(
            |(temperature, owner)| {
                snapshot_temperature_owner(snapshot) == Some(owner)
                    && snapshot
                        .supply_temperature_for_saturation_humidity_ratio_c
                        .is_some_and(|value| value.to_bits() == temperature.to_bits())
            },
        )
    })
}
