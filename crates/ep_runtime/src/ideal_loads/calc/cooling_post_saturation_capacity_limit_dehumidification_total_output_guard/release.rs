//! Release-bound CP383 total-output capacity guard.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{
    direct_predecessor_is_retained_and_complete, guard_links_to_predecessor,
    retained_active_input,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshots_match_bit_exact;

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment
        .latest
    else {
        return false;
    };
    let Some(active_input) = retained_active_input(runtime, unit, system, predecessor) else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && guard_links_to_predecessor(snapshot, predecessor)
        && active_input_matches_snapshot(active_input, snapshot)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP383 for the exact direct no-OA release route.
///
/// CP382 solely owns `CoolTotOutput`; retained CP321 solely owns
/// `MaxCoolTotCap`, with mandatory same-call CP340 bit corroboration. The
/// transition preserves raw binary64 `>` behavior and mutates no load value.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp382: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardError,
> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardError as Error;

    let selected = predecessor_cp382.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let guard_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witness(selected);

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
    if predecessor_cp382.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(predecessor_cp382)
    {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor_cp382) {
        return Err(predecessor_mismatch(selected));
    }
    let active_input = retained_active_input(runtime, unit, system, predecessor_cp382)
        .ok_or(Error::ActiveOperandOwnerLineageMismatch { system: selected })?;
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, predecessor_cp382, guard_witness)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, predecessor_cp382)
        || predecessor_cp382.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
        predecessor_cp382,
        active_input,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !guard_links_to_predecessor(snapshot, predecessor_cp382)
        || !active_input_matches_snapshot(active_input, snapshot)
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(Error::UnknownSystem { system: selected });
    };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =
        next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witness(
        selected,
        snapshot,
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn active_input_matches_snapshot(input: Option<ActiveInput>, snapshot: Snapshot) -> bool {
    match input {
        Some(input) => {
            snapshot.dehumidification_total_output_capacity_guard_evaluated
                && option_matches_bits(snapshot.cooling_total_output_w, input.cooling_total_output_w)
                && option_matches_bits(
                    snapshot.maximum_total_cooling_capacity_w,
                    input.maximum_total_cooling_capacity_w,
                )
        }
        None => {
            !snapshot.dehumidification_total_output_capacity_guard_evaluated
                && snapshot.cooling_total_output_w.is_none()
                && snapshot.maximum_total_cooling_capacity_w.is_none()
        }
    }
}

fn option_matches_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
