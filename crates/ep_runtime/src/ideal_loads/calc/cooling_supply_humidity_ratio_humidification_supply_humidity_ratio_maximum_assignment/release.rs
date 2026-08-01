//! Release-bound CP375 humidification supply-humidity-ratio maximum assignment.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
};

use super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_state,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshots_match_bit_exact as cp374_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release,
};

mod error;
mod prefix_validation;
#[allow(dead_code)]
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{assignment_links_to_predecessor, direct_predecessor_is_retained_and_complete};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use prefix_validation::{
    active_humidistat_operands_from_cp362_counterfactual,
    active_none_operands_from_retained_cp345_for_test,
};
pub(in crate::ideal_loads) use private_counterfactual::{
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_witness(
            system.id,
        );
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && predecessor_witness
            .is_some_and(|witness| cp374_snapshots_match_bit_exact(predecessor, witness))
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release(predecessor)
        && completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && assignment_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP375 for the exact direct no-OA release route.
///
/// Direct release remains on the humidification-control false path. It never
/// assembles, reads, or validates either active branch owner.
pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp374: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError,
> {
    let selected = predecessor_cp374.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::UnknownSystem { system: selected },
    )?;
    let retained_predecessor = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit
        .latest;
    let predecessor_witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_witness(
            selected,
        );
    let assignment_witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::SystemOutsideDirectSubset {
            system: selected,
        });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::InitializationNotReady {
            system: selected,
        });
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp374.controlled_zone != controlled_zone
        || !cp374_snapshots_match_bit_exact(retained_predecessor, predecessor_cp374)
        || !predecessor_witness
            .is_some_and(|witness| cp374_snapshots_match_bit_exact(witness, predecessor_cp374))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release(
        predecessor_cp374,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::PredecessorOutsideDirectSubset {
            system: selected,
        });
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, retained_predecessor, assignment_witness)
        || !direct_predecessor_is_retained_and_complete(
            runtime,
            unit,
            system,
            retained_predecessor,
        )
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    if !next_transition_fits(
        &unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
        retained_predecessor,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::UnknownSystem { system: selected },
        )?;
        advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_state(
            &mut unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
            retained_predecessor,
            None,
        )
    }
    .ok_or(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    runtime.set_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_witness(
        selected,
        snapshot,
    );
    debug_assert!(
        cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
