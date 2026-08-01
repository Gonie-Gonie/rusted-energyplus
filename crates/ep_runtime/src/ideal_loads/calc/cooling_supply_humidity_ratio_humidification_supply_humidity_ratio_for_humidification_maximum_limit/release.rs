//! Release-bound CP374 humidification humidity-ratio maximum-limit evidence.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
};

use super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_state,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshots_match_bit_exact as cp373_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release,
};

mod error;
mod operand_validation;
mod prefix_validation;
#[allow(dead_code)]
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{
    active_lineage_is_exact, assignment_links_to_predecessor,
    direct_predecessor_is_retained_and_complete,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use prefix_validation::active_operands_from_selected_typed_owner_for_test;
pub(in crate::ideal_loads) use private_counterfactual::{
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    snapshot_route as cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_route,
    snapshots_match_bit_exact as cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshots_match_bit_exact,
};

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witness(
            system.id,
        );
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && predecessor_witness
            .is_some_and(|witness| cp373_snapshots_match_bit_exact(predecessor, witness))
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release(predecessor)
        && completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && assignment_links_to_predecessor(snapshot, predecessor)
        && active_lineage_is_exact(runtime, unit, system, predecessor, snapshot)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP374 for the exact direct no-OA release route.
///
/// Direct release remains on CP373's humidification-control false path. The
/// four CP374 sites and all numeric evidence therefore remain complete skips.
pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp373: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError,
> {
    let selected = predecessor_cp373.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::UnknownSystem { system: selected },
    )?;
    let retained_predecessor = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment
        .latest;
    let predecessor_witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witness(
            selected,
        );
    let limit_witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::SystemOutsideDirectSubset {
            system: selected,
        });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::InitializationNotReady {
            system: selected,
        });
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp373.controlled_zone != controlled_zone
        || !cp373_snapshots_match_bit_exact(retained_predecessor, predecessor_cp373)
        || !predecessor_witness
            .is_some_and(|witness| cp373_snapshots_match_bit_exact(witness, predecessor_cp373))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release(
        predecessor_cp373,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::PredecessorOutsideDirectSubset {
            system: selected,
        });
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, retained_predecessor, limit_witness)
        || !direct_predecessor_is_retained_and_complete(
            runtime,
            unit,
            system,
            retained_predecessor,
        )
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    if !next_transition_fits(
        &unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
        retained_predecessor,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::UnknownSystem { system: selected },
        )?;
        advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_state(
            &mut unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
            retained_predecessor,
            None,
        )
    }
    .ok_or(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    runtime.set_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_witness(
        selected,
        snapshot,
    );
    debug_assert!(
        cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
