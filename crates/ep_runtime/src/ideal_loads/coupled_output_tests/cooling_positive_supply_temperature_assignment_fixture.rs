use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_temperature_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    zone_cooling_setpoint_load_w: f64,
    supply_mass_flow_rate_kg_per_s: Option<f64>,
    zone_node_temperature_c: Option<f64>,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
    let assignment_executed = predecessor.cp_air_assignment_executed;
    let zone_cooling_setpoint_load_w = assignment_executed.then_some(zone_cooling_setpoint_load_w);
    let cp_air_j_per_kg_k = assignment_executed
        .then_some(predecessor.cp_air_j_per_kg_k)
        .flatten();
    let supply_mass_flow_rate_kg_per_s = assignment_executed
        .then_some(supply_mass_flow_rate_kg_per_s)
        .flatten();
    let cp_air_times_supply_mass_flow_rate_w_per_k = cp_air_j_per_kg_k
        .zip(supply_mass_flow_rate_kg_per_s)
        .map(|(cp_air, mass_flow)| cp_air * mass_flow);
    let zone_cooling_setpoint_load_over_denominator_c = zone_cooling_setpoint_load_w
        .zip(cp_air_times_supply_mass_flow_rate_w_per_k)
        .map(|(load, denominator)| load / denominator);
    let zone_node_temperature_c = assignment_executed
        .then_some(zone_node_temperature_c)
        .flatten();
    let calculated_supply_temperature_c = zone_cooling_setpoint_load_over_denominator_c
        .zip(zone_node_temperature_c)
        .map(|(load_temperature, zone_temperature)| load_temperature + zone_temperature);

    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor
            .predecessor_active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        supply_temperature_assignment_executed: assignment_executed,
        zone_cooling_setpoint_load_read: assignment_executed,
        zone_cooling_setpoint_load_w,
        cp_air_read: assignment_executed,
        cp_air_j_per_kg_k,
        supply_mass_flow_rate_read: assignment_executed,
        supply_mass_flow_rate_kg_per_s,
        cp_air_times_supply_mass_flow_rate_calculated: assignment_executed,
        cp_air_times_supply_mass_flow_rate_w_per_k,
        zone_cooling_setpoint_load_over_denominator_calculated: assignment_executed,
        zone_cooling_setpoint_load_over_denominator_c,
        zone_node_temperature_read: assignment_executed,
        zone_node_temperature_c,
        supply_temperature_calculated: assignment_executed,
        calculated_supply_temperature_c,
        supply_temperature_assigned: assignment_executed,
        supply_temperature_c: calculated_supply_temperature_c,
    }
}
