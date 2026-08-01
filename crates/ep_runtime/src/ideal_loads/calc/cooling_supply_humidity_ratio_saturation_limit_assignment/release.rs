//! Release-bound CP378 final saturation-limit assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Snapshot;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::{
    completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent,
    cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact as cp377_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError;
use error::{call_order_error, original_owner_mismatch, predecessor_mismatch};
use prefix_validation::{
    assignment_links_to_predecessor, direct_original_owner_is_retained_and_complete,
    direct_predecessor_is_retained_and_complete,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_supply_humidity_ratio_saturation_limit_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_supply_humidity_ratio_saturation_limit_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact;

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_humidity_ratio_saturation_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness =
        runtime.cooling_supply_humidity_ratio_saturation_assignment_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && predecessor_witness
            .is_some_and(|value| cp377_snapshots_match_bit_exact(value, predecessor))
        && cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && direct_original_owner_is_retained_and_complete(runtime, unit, system, predecessor)
        && assignment_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
}

/// Executes CP378 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp377: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError> {
    let selected = predecessor_cp377.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_supply_humidity_ratio_saturation_assignment
        .latest;
    let predecessor_witness =
        runtime.cooling_supply_humidity_ratio_saturation_assignment_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::SystemOutsideDirectSubset {
            system: selected,
        });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::InitializationNotReady {
            system: selected,
        });
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp377.controlled_zone != controlled_zone
        || !cp377_snapshots_match_bit_exact(retained_predecessor, predecessor_cp377)
        || !predecessor_witness
            .is_some_and(|value| cp377_snapshots_match_bit_exact(value, predecessor_cp377))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
        predecessor_cp377,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::PredecessorOutsideDirectSubset {
            system: selected,
        });
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, retained_predecessor) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    if !direct_original_owner_is_retained_and_complete(runtime, unit, system, retained_predecessor)
    {
        return Err(original_owner_mismatch(selected));
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, retained_predecessor, assignment_witness)
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment,
        retained_predecessor,
    )
    .ok_or(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    if !assignment_links_to_predecessor(snapshot, retained_predecessor)
        || !cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(
            PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::UnknownSystem {
                system: selected,
            },
        );
    };
    unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment = next_state;
    runtime.set_cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(
        selected, snapshot,
    );
    Ok(snapshot)
}
