//! CP372 predecessor provenance, route, and numeric-shape validation.

use super::Route;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute as Cp371Route,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Cp371Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Predecessor,
};

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let cp371_route = cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route(
        cp371_snapshot(predecessor),
    )?;
    let route = match cp371_route {
        Cp371Route::UnitOff => Route::UnitOff,
        Cp371Route::NonCooling => Route::NonCooling,
        Cp371Route::PositiveGuardFalseFallthrough => Route::PositiveGuardFalseFallthrough,
        Cp371Route::HeatingAvailabilityGuardFalseFallthrough => Route::HeatingAvailabilityGuardFalseFallthrough,
        Cp371Route::HumidificationControlGuardFalseFallthrough => Route::HumidificationControlGuardFalseFallthrough,
        Cp371Route::DehumidificationControlHumidistatBodyEntered => Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted,
        Cp371Route::DehumidificationControlNoneBodyEntered => Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted,
        Cp371Route::DehumidificationControlGuardFalseFallthrough => Route::DehumidificationControlGuardFalseFallthrough,
    };
    values_fit_route(predecessor, route).then_some(route)
}

fn values_fit_route(predecessor: Predecessor, route: Route) -> bool {
    let active = matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted
    );
    if !active {
        return !predecessor.humidification_moisture_demand_assignment_executed
            && !predecessor.zone_humidifying_setpoint_moisture_demand_read
            && predecessor.zone_humidifying_setpoint_moisture_demand_kg_per_s.is_none()
            && !predecessor.zone_humidifying_setpoint_moisture_demand_assigned
            && predecessor.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s.is_none()
            && predecessor.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s.is_none();
    }
    let (Some(read), Some(assigned), Some(resulting)) = (
        predecessor.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
    ) else {
        return false;
    };
    predecessor.humidification_moisture_demand_assignment_executed
        && predecessor.zone_humidifying_setpoint_moisture_demand_read
        && predecessor.zone_humidifying_setpoint_moisture_demand_assigned
        && read.to_bits() == assigned.to_bits()
        && read.to_bits() == resulting.to_bits()
}

fn cp371_snapshot(snapshot: Predecessor) -> Cp371Snapshot {
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
