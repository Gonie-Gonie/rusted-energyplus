//! Release-bound CP399 shared-case `CpAir` assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Snapshot;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{
    active_input_from_retained_owner, assignment_links_to_prefix,
    direct_prefix_is_retained_and_complete, snapshot_operand_links_to_owner,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    snapshot_is_exact as cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact,
    snapshot_route as cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_route,
    snapshots_match_bit_exact as cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshots_match_bit_exact,
};

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry.latest else {
        return false;
    };
    let Some(route) = super::transition::routes::predecessor_route(predecessor) else {
        return false;
    };
    let active_input = if route.active {
        let Some(input) = active_input_from_retained_owner(runtime, unit, system, predecessor) else {
            return false;
        };
        Some(input)
    } else {
        None
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && direct_prefix_is_retained_and_complete(runtime, unit, system, predecessor)
        && assignment_links_to_prefix(snapshot, predecessor, active_input)
        && snapshot_operand_links_to_owner(snapshot, active_input)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP399 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp398: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentError> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentError as Error;

    let selected = predecessor_cp398.system;
    let unit = runtime.units.get(&selected).ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_latest_witness(selected);
    if system.id != selected {
        return Err(Error::SystemIdentityMismatch { expected: selected, actual: system.id });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(Error::DehumidificationControlTypeOutsideDirectSubset { system: selected, actual: system.dehumidification_control_type });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(Error::HumidificationControlTypeOutsideDirectSubset { system: selected, actual: system.humidification_control_type });
    }
    if !unit.topology_completed || unit.topology_failure.is_some() || unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(Error::InitializationNotReady { system: selected });
    }
    let controlled_zone = unit.controlled_zone.ok_or(Error::InitializationNotReady { system: selected })?;
    if predecessor_cp398.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(predecessor_cp398) {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !direct_prefix_is_retained_and_complete(runtime, unit, system, predecessor_cp398) {
        return Err(predecessor_mismatch(selected));
    }
    if !calc_state_identities_match(unit, selected) || !pending_state_is_consistent(unit, predecessor_cp398, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, predecessor_cp398) || predecessor_cp398.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }
    let route = super::transition::routes::predecessor_route(predecessor_cp398)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let active_input = if route.active {
        Some(active_input_from_retained_owner(runtime, unit, system, predecessor_cp398).ok_or(
            Error::MixedAirHumidityRatioOwnerUnavailableOrInconsistent { system: selected },
        )?)
    } else {
        None
    };
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment,
        predecessor_cp398,
        active_input,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !assignment_links_to_prefix(snapshot, predecessor_cp398, active_input)
        || !snapshot_operand_links_to_owner(snapshot, active_input)
        || !cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else { return Err(Error::UnknownSystem { system: selected }); };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_latest_witness(selected, snapshot);
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_state_is_consistent(
            unit,
            snapshot,
            runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
