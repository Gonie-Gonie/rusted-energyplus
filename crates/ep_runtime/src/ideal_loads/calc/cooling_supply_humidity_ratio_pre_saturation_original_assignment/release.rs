//! Release-bound CP376 pre-saturation original assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner as Owner,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state,
};
use super::transition::{predecessor_route, route_is_active};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshots_match_bit_exact as cp375_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError;
use error::{call_order_error, owner_mismatch, predecessor_mismatch};
use prefix_validation::{
    assignment_links_to_predecessor, direct_cp347_owner,
    direct_predecessor_is_retained_and_complete,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_supply_humidity_ratio_pre_saturation_original_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshots_match_bit_exact;

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_witness(
            system.id,
        );
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && predecessor_witness.is_some_and(|witness| {
            cp375_snapshots_match_bit_exact(predecessor, witness)
        })
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent(
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
        && cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
}

/// Executes CP376 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp375: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError>
{
    let selected = predecessor_cp375.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment
        .latest;
    let predecessor_witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_witness(
            selected,
        );
    let assignment_witness = runtime
        .cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::SystemOutsideDirectSubset {
            system: selected,
        });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::InitializationNotReady {
            system: selected,
        });
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp375.controlled_zone != controlled_zone
        || !cp375_snapshots_match_bit_exact(retained_predecessor, predecessor_cp375)
        || !predecessor_witness
            .is_some_and(|witness| cp375_snapshots_match_bit_exact(witness, predecessor_cp375))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
        predecessor_cp375,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::PredecessorOutsideDirectSubset {
            system: selected,
        });
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, retained_predecessor) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    let route = predecessor_route(retained_predecessor).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::PredecessorOutsideDirectSubset {
            system: selected,
        },
    )?;
    let input = if route_is_active(route) {
        let owner = direct_cp347_owner(runtime, unit, system, retained_predecessor)
            .ok_or(owner_mismatch(selected))?;
        Some(ActiveInput {
            purchased_air_supply_humidity_ratio: owner,
            owner: Owner::Cp347NoneCase,
        })
    } else {
        None
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, retained_predecessor, assignment_witness)
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    if !next_transition_fits(
        &unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
        retained_predecessor,
        input,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state(
            &mut unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
            retained_predecessor,
            input,
        )
    }
    .ok_or(PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    runtime.set_cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(
        selected, snapshot,
    );
    debug_assert!(
        cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
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
                .purchased_air_supply_humidity_ratio_before_saturation_check
                .is_none();
        }
        direct_cp347_owner(runtime, unit, system, predecessor).is_some_and(|owner| {
            snapshot
                .purchased_air_supply_humidity_ratio_before_saturation_check
                .is_some_and(|value| value.to_bits() == owner.to_bits())
        })
    })
}
