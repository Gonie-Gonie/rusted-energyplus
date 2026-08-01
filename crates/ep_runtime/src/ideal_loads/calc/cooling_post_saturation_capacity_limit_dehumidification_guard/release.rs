//! Release-bound CP381 post-saturation capacity-limit dehumidification guard.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{
    direct_predecessor_is_retained_and_complete, guard_links_to_predecessor, retained_active_input,
    retained_mixed_air_owner_is_valid,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_guard_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_guard_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_post_saturation_capacity_limit_dehumidification_guard_snapshots_match_bit_exact;

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_guard
        .latest
    else {
        return false;
    };
    let active_input = if snapshot.dehumidification_guard_evaluated {
        let Some(input) = retained_active_input(runtime, unit, system, predecessor) else {
            return false;
        };
        Some(input)
    } else {
        None
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(
            predecessor,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && guard_links_to_predecessor(snapshot, predecessor)
        && active_input_matches_snapshot(active_input, snapshot)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(
            snapshot,
        )
}

/// Executes CP381 for the exact direct no-OA release route.
///
/// CP380 is the sole immediate route predecessor. Active values are extracted
/// only from retained same-call CP378/CP379 and CP329 owner evidence. The pure
/// transition evaluates raw built-in `f64 < f64`; it performs no line-2267
/// calculation and does not consume or feed a numerical coupling DTO.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp380: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError>
{
    let selected = predecessor_cp380.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
            UnknownSystem { system: selected },
    )?;
    let guard_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
            InitializationNotReady { system: selected },
    )?;
    if predecessor_cp380.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(
        predecessor_cp380,
    ) {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
            PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if predecessor_cp380.capacity_limit_body_entered
        && !retained_mixed_air_owner_is_valid(runtime, unit, system, predecessor_cp380)
    {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
                MixedAirHumidityRatioOwnerLineageMismatch { system: selected },
        );
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor_cp380) {
        return Err(predecessor_mismatch(selected));
    }
    let active_input = if predecessor_cp380.capacity_limit_body_entered {
        Some(retained_active_input(runtime, unit, system, predecessor_cp380).ok_or(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
                SupplyHumidityRatioOwnerLineageMismatch { system: selected },
        )?)
    } else {
        None
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, predecessor_cp380, guard_witness)
    {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp380)
        || predecessor_cp380.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard,
        predecessor_cp380,
        active_input,
    )
    .ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
            RuntimeStateInvariantViolation { system: selected },
    )?;
    if !guard_links_to_predecessor(snapshot, predecessor_cp380)
        || !active_input_matches_snapshot(active_input, snapshot)
        || !cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(
            snapshot,
        )
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
                UnknownSystem { system: selected },
        );
    };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_guard_latest_witness(
        selected, snapshot,
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_post_saturation_capacity_limit_dehumidification_guard_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn active_input_matches_snapshot(input: Option<ActiveInput>, snapshot: Snapshot) -> bool {
    match input {
        Some(input) => {
            snapshot.dehumidification_guard_evaluated
                && snapshot
                    .supply_humidity_ratio
                    .is_some_and(|value| value.to_bits() == input.supply_humidity_ratio.to_bits())
                && snapshot.mixed_air_humidity_ratio.is_some_and(|value| {
                    value.to_bits() == input.mixed_air_humidity_ratio.to_bits()
                })
        }
        None => {
            !snapshot.dehumidification_guard_evaluated
                && snapshot.supply_humidity_ratio.is_none()
                && snapshot.mixed_air_humidity_ratio.is_none()
        }
    }
}
