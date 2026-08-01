//! Release-bound CP380 post-saturation capacity-limit guard.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact as cp379_snapshots_match_bit_exact;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError;
use error::{call_order_error, predecessor_mismatch, selector_lineage_mismatch};
use prefix_validation::{
    direct_predecessor_is_retained_and_complete, direct_selector_lineage_is_retained_and_complete,
    guard_links_to_predecessor,
};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_guard_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_guard_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release;
use snapshot_validation::predecessor_route_is_active;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_exact as cooling_post_saturation_capacity_limit_guard_snapshots_match_exact;

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_enthalpy_post_saturation_assignment
        .latest
    else {
        return false;
    };
    let Some(predecessor_witness) =
        runtime.cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && cp379_snapshots_match_bit_exact(predecessor, predecessor_witness)
        && cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && direct_selector_lineage_is_retained_and_complete(runtime, unit, system, predecessor)
        && guard_links_to_predecessor(snapshot, predecessor, system.cooling_limit)
        && completed_state_is_consistent(unit, snapshot, witness, system.cooling_limit)
        && cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP380 for the exact direct no-OA release route.
///
/// The typed configured cooling-limit selector is the only source operand.
/// CP337 corroborates that selector on the same call, while CP379 supplies only
/// the immediate route/lifecycle predecessor. No numerical value is read, fed,
/// reconciled, or mutated by this checkpoint.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp379: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError> {
    let selected = predecessor_cp379.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_supply_enthalpy_post_saturation_assignment
        .latest;
    let predecessor_witness =
        runtime.cooling_supply_enthalpy_post_saturation_assignment_latest_witness(selected);
    let guard_witness =
        runtime.cooling_post_saturation_capacity_limit_guard_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::HumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp379.controlled_zone != controlled_zone
        || !cp379_snapshots_match_bit_exact(retained_predecessor, predecessor_cp379)
        || !predecessor_witness
            .is_some_and(|value| cp379_snapshots_match_bit_exact(value, predecessor_cp379))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
        predecessor_cp379,
    ) {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !direct_selector_lineage_is_retained_and_complete(runtime, unit, system, predecessor_cp379) {
        return Err(selector_lineage_mismatch(selected));
    }
    if !direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor_cp379) {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    let active = predecessor_route_is_active(predecessor_cp379).ok_or({
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::RuntimeStateInvariantViolation {
            system: selected,
        }
    })?;
    let active_input = active.then_some(ActiveInput {
        cooling_limit: system.cooling_limit,
        cp337_same_call_selector_lineage_corroborated: true,
    });
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            predecessor_cp379,
            guard_witness,
            system.cooling_limit,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp379)
        || predecessor_cp379.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_guard,
        predecessor_cp379,
        active_input,
    )
    .ok_or(PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    if !guard_links_to_predecessor(snapshot, predecessor_cp379, system.cooling_limit)
        || !cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(
            unit,
            &next_state,
            snapshot,
            system.cooling_limit,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(
            PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::UnknownSystem {
                system: selected,
            },
        );
    };
    unit.calc_cooling_post_saturation_capacity_limit_guard = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_guard_latest_witness(selected, snapshot);
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_post_saturation_capacity_limit_guard_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
