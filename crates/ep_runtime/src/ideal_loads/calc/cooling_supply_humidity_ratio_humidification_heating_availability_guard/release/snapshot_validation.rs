//! Exact CP369 heating-availability-guard snapshot validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    match snapshot_route(snapshot) {
        Some(Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough) => true,
        Some(Route::HeatingAvailabilityBodyEntered) => {
            snapshot.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::None)
                && snapshot.dehumidification_control_none_case_completed_skip
        }
        Some(Route::HeatingAvailabilityGuardFalseFallthrough) | None => false,
    }
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER
        || snapshot
            .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
    {
        return None;
    }
    let predecessor_count = predecessor_active_count(snapshot);
    let local_count = local_active_count(snapshot);
    if snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0
        && guard_skipped(snapshot)
    {
        return Some(Route::UnitOff);
    }
    if !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0
        && guard_skipped(snapshot)
    {
        return Some(Route::NonCooling);
    }
    if positive_guard_fallthrough(snapshot)
        && predecessor_count == 0
        && local_count == 0
        && guard_skipped(snapshot)
    {
        return Some(Route::PositiveGuardFalseFallthrough);
    }
    if !active_prefix(snapshot)
        || predecessor_count != 1
        || local_count != 1
        || !selector_flags_match(snapshot)
        || !snapshot.heating_on_read
    {
        return None;
    }
    match snapshot.heating_on {
        Some(true)
            if snapshot.cooling_supply_humidity_ratio_humidification_body_entered
                && !snapshot.heating_on_guard_false_fallthrough =>
        {
            Some(Route::HeatingAvailabilityBodyEntered)
        }
        Some(false)
            if !snapshot.cooling_supply_humidity_ratio_humidification_body_entered
                && snapshot.heating_on_guard_false_fallthrough =>
        {
            Some(Route::HeatingAvailabilityGuardFalseFallthrough)
        }
        _ => None,
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_exact(left: Snapshot, right: Snapshot) -> bool {
    left == right
}

fn selector_flags_match(snapshot: Snapshot) -> bool {
    match snapshot.predecessor_dehumidification_control_type {
        Some(DehumidificationControlType::None) => {
            snapshot.predecessor_dehumidification_control_none_case_completed_skip
                && snapshot.dehumidification_control_none_case_completed_skip
        }
        Some(DehumidificationControlType::ConstantSensibleHeatRatio) => {
            snapshot
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
                && snapshot
                    .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        }
        Some(DehumidificationControlType::Humidistat) => {
            snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip
                && snapshot.dehumidification_control_humidistat_case_completed_skip
        }
        Some(DehumidificationControlType::ConstantSupplyHumidityRatio) => {
            snapshot
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
                && snapshot
                    .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
        }
        None => false,
    }
}

fn predecessor_active_count(snapshot: Snapshot) -> usize {
    usize::from(snapshot.predecessor_dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            snapshot
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        )
}

fn local_active_count(snapshot: Snapshot) -> usize {
    usize::from(snapshot.dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(snapshot.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        )
}

fn guard_skipped(snapshot: Snapshot) -> bool {
    !snapshot.heating_on_read
        && snapshot.heating_on.is_none()
        && !snapshot.cooling_supply_humidity_ratio_humidification_body_entered
        && !snapshot.heating_on_guard_false_fallthrough
}

fn positive_guard_fallthrough(snapshot: Snapshot) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
}

fn active_prefix(snapshot: Snapshot) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_some()
}

fn inactive_prefix(snapshot: Snapshot) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
}