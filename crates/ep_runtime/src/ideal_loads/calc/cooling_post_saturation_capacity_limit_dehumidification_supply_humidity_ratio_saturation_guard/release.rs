//! Release-bound CP413 saturation-humidity-ratio guard.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::transition::routes::{predecessor_route, route_is_active};
use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Snapshot;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{direct_prefix_is_retained_and_complete, snapshot_links_to_prefix};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_metadata_is_consistent;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::test_counts_are_exact;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    snapshot_is_exact as cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact,
    snapshot_route as cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_route,
    snapshots_match_bit_exact as cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshots_match_bit_exact,
};

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment
        .latest
    else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && direct_prefix_is_retained_and_complete(runtime, unit, system, predecessor)
        && snapshot_links_to_prefix(snapshot, predecessor)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP413 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp412: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardError> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardError as Error;

    let selected = predecessor_cp412.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_witness(selected);
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
    if predecessor_cp412.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if predecessor_cp412
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed
    {
        let saturation = predecessor_cp412
            .resulting_saturation_supply_humidity_ratio
            .ok_or(predecessor_mismatch(selected))?;
        let original = predecessor_cp412
            .resulting_supply_humidity_ratio_original
            .ok_or(predecessor_mismatch(selected))?;
        let cp411_terminal = predecessor_cp412
            .predecessor_cp411_resulting_supply_humidity_ratio
            .ok_or(predecessor_mismatch(selected))?;
        if original.to_bits() != cp411_terminal.to_bits() {
            return Err(predecessor_mismatch(selected));
        }
        if !saturation.is_finite() {
            return Err(Error::SaturationHumidityRatioOutsideDirectSubset {
                system: selected,
                bits: saturation.to_bits(),
            });
        }
        if !original.is_finite() {
            return Err(Error::OriginalHumidityRatioOutsideDirectSubset {
                system: selected,
                bits: original.to_bits(),
            });
        }
    }
    if !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(predecessor_cp412) {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !direct_prefix_is_retained_and_complete(runtime, unit, system, predecessor_cp412) {
        return Err(predecessor_mismatch(selected));
    }
    let route = predecessor_route(predecessor_cp412)
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    debug_assert_eq!(route_is_active(route), predecessor_cp412.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed);
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, predecessor_cp412, witness)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, predecessor_cp412)
        || predecessor_cp412.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard,
        predecessor_cp412,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !snapshot_links_to_prefix(snapshot, predecessor_cp412)
        || !cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(Error::UnknownSystem { system: selected });
    };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_witness(selected, snapshot);
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
