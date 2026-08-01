//! Exact CP370 humidification-control Humidistat-guard snapshot validation.

use ep_model::HumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_control_humidistat_guard::transition::{
    PredecessorRoute, predecessor_route,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Predecessor,
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release,
};

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    let predecessor = predecessor_snapshot(snapshot);
    if !cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release(
        predecessor,
    ) {
        return false;
    }
    match snapshot_route(snapshot) {
        Some(Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough) => true,
        Some(Route::HumidificationControlGuardFalseFallthrough) => {
            snapshot.humidification_control_type == Some(HumidificationControlType::None)
        }
        Some(
            Route::HeatingAvailabilityGuardFalseFallthrough
            | Route::HumidificationControlBodyEntered,
        )
        | None => false,
    }
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER
    {
        return None;
    }
    match predecessor_route(predecessor_snapshot(snapshot))? {
        PredecessorRoute::UnitOff if control_is_skipped(snapshot) => Some(Route::UnitOff),
        PredecessorRoute::NonCooling if control_is_skipped(snapshot) => Some(Route::NonCooling),
        PredecessorRoute::PositiveGuardFalseFallthrough if control_is_skipped(snapshot) => {
            Some(Route::PositiveGuardFalseFallthrough)
        }
        PredecessorRoute::Active {
            heating_on: false,
            ..
        } if control_is_skipped(snapshot) => {
            Some(Route::HeatingAvailabilityGuardFalseFallthrough)
        }
        PredecessorRoute::Active {
            heating_on: true, ..
        } => active_route(snapshot),
        _ => None,
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    left == right
}

pub(super) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_body_entered: snapshot.unit_body_entered,
        predecessor_cooling_body_entered: snapshot.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: snapshot
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: snapshot
            .predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot
            .positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: snapshot
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: snapshot
            .predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            snapshot
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: snapshot
            .predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            snapshot
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            snapshot
                .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: snapshot
            .dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: snapshot
            .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip: snapshot
            .dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: snapshot
            .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        heating_on_read: snapshot.predecessor_heating_on_read,
        heating_on: snapshot.predecessor_heating_on,
        cooling_supply_humidity_ratio_humidification_body_entered: snapshot
            .predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        heating_on_guard_false_fallthrough: snapshot
            .predecessor_heating_on_guard_false_fallthrough,
    }
}

fn active_route(snapshot: Snapshot) -> Option<Route> {
    if !snapshot.humidification_control_type_read {
        return None;
    }
    let control = snapshot.humidification_control_type?;
    let humidistat = control == HumidificationControlType::Humidistat;
    if snapshot.humidification_control_type_humidistat != Some(humidistat) {
        return None;
    }
    if humidistat
        && snapshot.humidification_control_body_entered
        && !snapshot.humidification_control_guard_false_fallthrough
    {
        Some(Route::HumidificationControlBodyEntered)
    } else if !humidistat
        && !snapshot.humidification_control_body_entered
        && snapshot.humidification_control_guard_false_fallthrough
    {
        Some(Route::HumidificationControlGuardFalseFallthrough)
    } else {
        None
    }
}

fn control_is_skipped(snapshot: Snapshot) -> bool {
    !snapshot.humidification_control_type_read
        && snapshot.humidification_control_type.is_none()
        && snapshot
            .humidification_control_type_humidistat
            .is_none()
        && !snapshot.humidification_control_body_entered
        && !snapshot.humidification_control_guard_false_fallthrough
}
