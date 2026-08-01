//! CP373 predecessor provenance, eight-route, and binary64 validation.

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
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Cp371Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Cp372Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Predecessor,
};

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let route = cp372_route(cp372_snapshot(predecessor))?;
    let h = predecessor
        .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed;
    let n = predecessor
        .dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed;
    let structural_match = match route {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted => h && !n,
        Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted => !h && n,
        _ => !h && !n,
    };
    (structural_match && cp373_values_fit_route(predecessor, route)).then_some(route)
}

fn cp372_route(snapshot: Cp372Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER
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
    cp372_values_fit_route(snapshot, route).then_some(route)
}

fn cp372_values_fit_route(snapshot: Cp372Snapshot, route: Route) -> bool {
    let active = route_is_active(route);
    if !active {
        return !snapshot.humidification_moisture_demand_assignment_executed
            && !snapshot.zone_humidifying_setpoint_moisture_demand_read
            && snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s.is_none()
            && !snapshot.zone_humidifying_setpoint_moisture_demand_assigned
            && snapshot.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s.is_none()
            && snapshot.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s.is_none();
    }
    let (Some(read), Some(assigned), Some(resulting)) = (
        snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
    ) else {
        return false;
    };
    snapshot.humidification_moisture_demand_assignment_executed
        && snapshot.zone_humidifying_setpoint_moisture_demand_read
        && snapshot.zone_humidifying_setpoint_moisture_demand_assigned
        && read.to_bits() == assigned.to_bits()
        && read.to_bits() == resulting.to_bits()
}

fn cp373_values_fit_route(snapshot: Predecessor, route: Route) -> bool {
    let active = route_is_active(route);
    let flags = [
        snapshot.zone_humidifying_setpoint_moisture_demand_read,
        snapshot.supply_mass_flow_rate_read,
        snapshot.moisture_demand_derived_supply_humidity_ratio_calculated,
        snapshot.zone_node_humidity_ratio_read,
        snapshot.supply_humidity_ratio_for_humidification_calculated,
        snapshot.supply_humidity_ratio_for_humidification_assigned,
    ];
    let values = [
        snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.moisture_demand_derived_supply_humidity_ratio,
        snapshot.zone_node_humidity_ratio,
        snapshot.calculated_supply_humidity_ratio_for_humidification,
        snapshot.assigned_supply_humidity_ratio_for_humidification,
        snapshot.resulting_supply_humidity_ratio_for_humidification,
    ];
    if !active {
        return flags.into_iter().all(|flag| !flag)
            && values.into_iter().all(|value| value.is_none());
    }
    if !flags.into_iter().all(|flag| flag) {
        return false;
    }
    let (
        Some(predecessor_demand),
        Some(demand),
        Some(flow),
        Some(quotient),
        Some(zone_humidity),
        Some(calculated),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.moisture_demand_derived_supply_humidity_ratio,
        snapshot.zone_node_humidity_ratio,
        snapshot.calculated_supply_humidity_ratio_for_humidification,
        snapshot.assigned_supply_humidity_ratio_for_humidification,
        snapshot.resulting_supply_humidity_ratio_for_humidification,
    ) else {
        return false;
    };
    let expected_quotient = demand / flow;
    let expected_calculated = expected_quotient + zone_humidity;
    predecessor_demand.to_bits() == demand.to_bits()
        && quotient.to_bits() == expected_quotient.to_bits()
        && calculated.to_bits() == expected_calculated.to_bits()
        && assigned.to_bits() == calculated.to_bits()
        && resulting.to_bits() == calculated.to_bits()
}

fn route_is_active(route: Route) -> bool {
    matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted
    )
}

fn cp372_snapshot(snapshot: Predecessor) -> Cp372Snapshot {
    Cp372Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type_first_read: snapshot.predecessor_dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type: snapshot.predecessor_first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat: snapshot.predecessor_dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read: snapshot.predecessor_dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type: snapshot.predecessor_second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none: snapshot.predecessor_dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered: snapshot.predecessor_dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough: snapshot.predecessor_dehumidification_control_guard_false_fallthrough,
        humidification_moisture_demand_assignment_executed: snapshot.predecessor_humidification_moisture_demand_assignment_executed,
        zone_humidifying_setpoint_moisture_demand_read: snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_read,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        zone_humidifying_setpoint_moisture_demand_assigned: snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_assigned,
        assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: snapshot.predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
    }
}

fn cp371_snapshot(snapshot: Cp372Snapshot) -> Cp371Snapshot {
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
