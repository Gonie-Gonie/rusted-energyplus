//! Release-bound CP386 post-saturation switch dispatch.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{
    direct_predecessor_is_retained_and_complete, switch_links_to_predecessor,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_control_switch_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_control_switch_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact;
pub(in crate::ideal_loads::calc) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_switch_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
        .latest
    else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && switch_links_to_predecessor(
            snapshot,
            predecessor,
            predecessor.supply_enthalpy_assignment_executed.then_some(ActiveInput {
                dehumidification_control_type: system.dehumidification_control_type,
            }),
        )
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP386 for the exact direct no-OA release route.
///
/// The typed dehumidification-control owner is read only when CP385 completed
/// its supply-enthalpy assignment. Dispatch is symbolic and never consumes the
/// retained CP385 enthalpy as a numerical operand.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp385: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchError>
{
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchError as Error;

    let selected = predecessor_cp385.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_switch_latest_witness(
            selected,
        );

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
    if predecessor_cp385.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
        predecessor_cp385,
    ) {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !direct_predecessor_is_retained_and_complete(
        runtime,
        unit,
        system,
        predecessor_cp385,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, predecessor_cp385, witness)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, predecessor_cp385)
        || predecessor_cp385.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = predecessor_cp385
        .supply_enthalpy_assignment_executed
        .then_some(ActiveInput {
            dehumidification_control_type: system.dehumidification_control_type,
        });
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch,
        predecessor_cp385,
        active_input,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !switch_links_to_predecessor(snapshot, predecessor_cp385, active_input)
        || !cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(Error::UnknownSystem { system: selected });
    };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_switch_latest_witness(
        selected, snapshot,
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_switch_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_post_saturation_capacity_limit_dehumidification_control_switch_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
