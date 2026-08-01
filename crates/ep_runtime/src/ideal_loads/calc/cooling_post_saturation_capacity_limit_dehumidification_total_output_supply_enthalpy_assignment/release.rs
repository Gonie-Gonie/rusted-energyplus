//! Release-bound CP385 supply-enthalpy assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Snapshot;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{
    assignment_links_to_predecessors, direct_predecessor_is_retained_and_complete,
    retained_cp382_lineage_is_exact, retained_input_from_prefix,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshots_match_bit_exact;

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment
        .latest
    else {
        return false;
    };
    let active = predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated;
    let cp382 = active
        .then_some(unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment.latest)
        .flatten();
    let cp382_witness = active
        .then(|| runtime.cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_witness(system.id))
        .flatten();
    let cp382_complete = if active {
        let (Some(cp382), Some(cp382_witness)) = (cp382, cp382_witness) else {
            return false;
        };
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent(
            runtime, unit, system, cp382, Some(cp382_witness),
        )
    } else {
        true
    };
    let retained_input = retained_input_from_prefix(predecessor, cp382);

    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && retained_cp382_lineage_is_exact(predecessor, cp382, cp382_witness)
        && cp382_complete
        && assignment_links_to_predecessors(snapshot, predecessor, retained_input)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP385 for the exact direct no-OA release route.
///
/// CP382 recursively owns and corroborates the CP379 preexisting enthalpy,
/// CP329 mixed-air enthalpy, and CP330 mass-flow operands. The numerator is
/// strictly CP384's resulting cooling total output. Arithmetic is raw binary64.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp384: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentError> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentError as Error;

    let selected = predecessor_cp384.system;
    let unit = runtime.units.get(&selected).ok_or(Error::UnknownSystem { system: selected })?;
    let assignment_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(Error::SystemIdentityMismatch { expected: selected, actual: system.id });
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
    let controlled_zone = unit.controlled_zone.ok_or(Error::InitializationNotReady { system: selected })?;
    if predecessor_cp384.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(predecessor_cp384)
    {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor_cp384) {
        return Err(predecessor_mismatch(selected));
    }

    let retained_cp384 = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment
        .latest
        .ok_or_else(|| predecessor_mismatch(selected))?;
    let active = retained_cp384.predecessor_dehumidification_total_output_capacity_guard_evaluated;
    let cp382 = active
        .then_some(unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment.latest)
        .flatten();
    let cp382_witness = active
        .then(|| runtime.cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_witness(selected))
        .flatten();
    if !retained_cp382_lineage_is_exact(retained_cp384, cp382, cp382_witness) {
        return Err(Error::RetainedPostSaturationDehumidificationOperandBundleLineageMismatch { system: selected });
    }
    let cp382_complete = if active {
        let (Some(cp382), Some(cp382_witness)) = (cp382, cp382_witness) else {
            return Err(Error::RetainedPostSaturationDehumidificationOperandBundleLineageMismatch { system: selected });
        };
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent(
            runtime, unit, system, cp382, Some(cp382_witness),
        )
    } else {
        true
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, retained_cp384, assignment_witness)
        || !cp382_complete
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, retained_cp384)
        || retained_cp384.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let retained_input = retained_input_from_prefix(retained_cp384, cp382);
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment,
        retained_cp384,
        retained_input,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !assignment_links_to_predecessors(snapshot, retained_cp384, retained_input)
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(Error::UnknownSystem { system: selected });
    };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_witness(selected, snapshot);
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
