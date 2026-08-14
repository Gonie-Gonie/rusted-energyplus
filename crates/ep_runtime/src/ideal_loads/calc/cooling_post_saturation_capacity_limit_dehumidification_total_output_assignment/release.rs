//! Release-bound CP382 total-output assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentInput,
};
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{
    ActiveInputValidationError, assignment_links_to_predecessor,
    direct_predecessor_is_retained_and_complete, retained_active_input,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_metadata_is_consistent;
pub(in crate::ideal_loads::calc) use runtime_validation::committed_latest_snapshot_is_consistent as cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_committed_latest_snapshot_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshots_match_bit_exact;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshot_route;

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard
        .latest
    else {
        return false;
    };
    let Ok(active_input) = retained_active_input(runtime, unit, system, predecessor) else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && assignment_links_to_predecessor(snapshot, predecessor)
        && active_input_matches_snapshot(active_input, snapshot)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP382 for the exact direct no-OA release route.
///
/// CP381 is the sole immediate route predecessor. Active values are obtained
/// only from retained same-call CP330, CP329, CP339, and CP379 evidence. The
/// pure transition preserves `flow * (mixed - supply)` grouping and does not
/// consume or feed a numerical coupling DTO.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp381: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError,
> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError as Error;

    let selected = predecessor_cp381.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let assignment_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_witness(selected);

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
    if predecessor_cp381.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(predecessor_cp381)
    {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor_cp381) {
        return Err(predecessor_mismatch(selected));
    }
    let active_input = match retained_active_input(runtime, unit, system, predecessor_cp381) {
        Ok(input) => input,
        Err(ActiveInputValidationError::Lineage) => {
            return Err(Error::ActiveOperandOwnerLineageMismatch { system: selected });
        }
        Err(ActiveInputValidationError::Invalid(input)) => {
            return Err(Error::InvalidActiveInput {
                system: selected,
                input,
            });
        }
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, predecessor_cp381, assignment_witness)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, predecessor_cp381)
        || predecessor_cp381.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment,
        predecessor_cp381,
        active_input,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !assignment_links_to_predecessor(snapshot, predecessor_cp381)
        || !active_input_matches_snapshot(active_input, snapshot)
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(Error::UnknownSystem { system: selected });
    };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =
        next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_witness(
        selected,
        snapshot,
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn active_input_matches_snapshot(input: Option<ActiveInput>, snapshot: Snapshot) -> bool {
    match input {
        Some(input) => {
            snapshot.dehumidification_total_output_assignment_executed
                && option_matches_bits(
                    snapshot.supply_mass_flow_rate_kg_per_s,
                    input.supply_mass_flow_rate_kg_per_s,
                )
                && option_matches_bits(
                    snapshot.mixed_air_enthalpy_j_per_kg,
                    input.mixed_air_enthalpy_j_per_kg,
                )
                && option_matches_bits(
                    snapshot.supply_enthalpy_j_per_kg,
                    input.supply_enthalpy_j_per_kg,
                )
        }
        None => {
            !snapshot.dehumidification_total_output_assignment_executed
                && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
                && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
                && snapshot.supply_enthalpy_j_per_kg.is_none()
        }
    }
}

fn option_matches_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
