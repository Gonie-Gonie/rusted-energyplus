//! CP374 source provenance and eight-route structural validation.

use super::{Route, Snapshot};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute as Cp371Route,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Cp371Snapshot,
};

pub(super) fn structural_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER
    {
        return None;
    }
    let route = match cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route(
        cp371_snapshot(snapshot),
    )? {
        Cp371Route::UnitOff => Route::UnitOff,
        Cp371Route::NonCooling => Route::NonCooling,
        Cp371Route::PositiveGuardFalseFallthrough => Route::PositiveGuardFalseFallthrough,
        Cp371Route::HeatingAvailabilityGuardFalseFallthrough => Route::HeatingAvailabilityGuardFalseFallthrough,
        Cp371Route::HumidificationControlGuardFalseFallthrough => Route::HumidificationControlGuardFalseFallthrough,
        Cp371Route::DehumidificationControlHumidistatBodyEntered => Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted,
        Cp371Route::DehumidificationControlNoneBodyEntered => Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted,
        Cp371Route::DehumidificationControlGuardFalseFallthrough => Route::DehumidificationControlGuardFalseFallthrough,
    };
    let predecessor_h = snapshot
        .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed;
    let predecessor_n = snapshot
        .predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed;
    let local_h = snapshot
        .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed;
    let local_n = snapshot
        .dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed;
    let flags_match = match route {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted => predecessor_h && !predecessor_n && local_h && !local_n,
        Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted => !predecessor_h && predecessor_n && !local_h && local_n,
        _ => !predecessor_h && !predecessor_n && !local_h && !local_n,
    };
    flags_match.then_some(route)
}

fn cp371_snapshot(snapshot: Snapshot) -> Cp371Snapshot {
    Cp371Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_body_entered: snapshot.unit_body_entered,
        predecessor_cooling_body_entered: snapshot.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: snapshot.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: snapshot.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: snapshot.predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: snapshot.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: snapshot.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip: snapshot.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: snapshot.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: snapshot.predecessor_heating_on_read,
        predecessor_heating_on: snapshot.predecessor_heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough: snapshot.predecessor_heating_on_guard_false_fallthrough,
        predecessor_humidification_control_type_read: snapshot.predecessor_humidification_control_type_read,
        predecessor_humidification_control_type: snapshot.predecessor_humidification_control_type,
        predecessor_humidification_control_type_humidistat: snapshot.predecessor_humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered: snapshot.predecessor_humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough: snapshot.predecessor_humidification_control_guard_false_fallthrough,
        dehumidification_control_type_first_read: snapshot.predecessor_dehumidification_control_type_first_read,
        first_dehumidification_control_type: snapshot.predecessor_first_dehumidification_control_type,
        dehumidification_control_type_humidistat: snapshot.predecessor_dehumidification_control_type_humidistat,
        dehumidification_control_type_second_read: snapshot.predecessor_dehumidification_control_type_second_read,
        second_dehumidification_control_type: snapshot.predecessor_second_dehumidification_control_type,
        dehumidification_control_type_none: snapshot.predecessor_dehumidification_control_type_none,
        dehumidification_control_body_entered: snapshot.predecessor_dehumidification_control_body_entered,
        dehumidification_control_guard_false_fallthrough: snapshot.predecessor_dehumidification_control_guard_false_fallthrough,
    }
}
