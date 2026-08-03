//! Release-bound CP412 saturation-humidity-ratio assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::transition::routes::{predecessor_route, route_is_active};
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

mod error;
mod prefix_validation;
mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentError;
use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{direct_prefix_is_retained_and_complete, snapshot_links_to_prefix};
pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_latest_metadata_is_consistent;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::test_counts_are_exact;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    snapshot_is_exact as cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact,
    snapshot_route as cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_route,
    snapshots_match_bit_exact as cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact,
};

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment
        .latest
    else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && direct_prefix_is_retained_and_complete(runtime, unit, system, predecessor)
        && snapshot_links_to_prefix(snapshot, predecessor)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(snapshot)
}

/// Executes CP412 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp411: Predecessor,
    barometric_pressure_pa: f64,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentError> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentError as Error;

    let selected = predecessor_cp411.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_latest_witness(selected);
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
    if predecessor_cp411.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    if !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(predecessor_cp411) {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !direct_prefix_is_retained_and_complete(runtime, unit, system, predecessor_cp411) {
        return Err(predecessor_mismatch(selected));
    }
    let route = predecessor_route(predecessor_cp411)
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    let input = if route_is_active(route) {
        let temperature = predecessor_cp411
            .resulting_supply_temperature_c
            .ok_or(predecessor_mismatch(selected))?;
        if !temperature.is_finite() {
            return Err(Error::SupplyTemperatureOutsideDirectSubset {
                system: selected,
                bits: temperature.to_bits(),
            });
        }
        if !barometric_pressure_pa.is_finite() || barometric_pressure_pa <= 0.0 {
            return Err(Error::BarometricPressureOutsideDirectSubset {
                system: selected,
                bits: barometric_pressure_pa.to_bits(),
            });
        }
        let result =
            energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, barometric_pressure_pa);
        if !result.is_finite() {
            return Err(Error::SaturationHumidityRatioOutsideDirectSubset {
                system: selected,
                bits: result.to_bits(),
            });
        }
        Some(ActiveInput {
            outdoor_barometric_pressure_pa: barometric_pressure_pa,
        })
    } else {
        None
    };
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, predecessor_cp411, witness)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, predecessor_cp411)
        || predecessor_cp411.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let (next_state, snapshot) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment,
        predecessor_cp411,
        input,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !snapshot_links_to_prefix(snapshot, predecessor_cp411)
        || !cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(snapshot)
        || !prepared_completed_state_is_consistent(unit, &next_state, snapshot)
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let Some(unit) = runtime.units.get_mut(&selected) else {
        return Err(Error::UnknownSystem { system: selected });
    };
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_latest_witness(selected, snapshot);
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
