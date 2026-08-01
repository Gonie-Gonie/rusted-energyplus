//! Release-bound CP387 case-entry and `CpAir` assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{assignment_links_to_predecessor, direct_predecessor_is_retained_and_complete};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use prefix_validation::active_input_from_owner_for_test;
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_predecessor_counts_match_exact_direct_switch,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch
        .latest
    else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && assignment_links_to_predecessor(snapshot, predecessor, None)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP387 for the exact direct no-OA release route.
///
/// Exact direct execution selects `None`, so the constant-SHR case entry and
/// all three assignment sites are complete-skipped. Private characterization
/// owns the three source-valid SHR routes and canonical scalar calculation.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp386: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentError>
{
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentError as Error;

    let selected = predecessor_cp386.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(Error::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(Error::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(Error::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    let controlled_zone = unit
        .controlled_zone
        .ok_or(Error::InitializationNotReady { system: selected })?;
    if predecessor_cp386.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
        predecessor_cp386,
    ) {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor_cp386) {
        return Err(predecessor_mismatch(selected));
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, predecessor_cp386, witness)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, predecessor_cp386)
        || predecessor_cp386.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
        predecessor_cp386,
        None,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !assignment_links_to_predecessor(snapshot, predecessor_cp386, None)
        || !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(Error::UnknownSystem { system: selected });
    };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(
        selected,
        snapshot,
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
