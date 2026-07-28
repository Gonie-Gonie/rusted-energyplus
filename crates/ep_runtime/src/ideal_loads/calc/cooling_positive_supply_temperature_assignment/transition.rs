//! Pure CP331-to-CP332 Cooling positive-supply temperature assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput
{
    pub zone_cooling_setpoint_load_w: f64,
    pub cp_air_j_per_kg_k: f64,
    pub supply_mass_flow_rate_kg_per_s: f64,
    pub zone_node_temperature_c: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_temperature_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    active_input: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput>,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
    let assignment_executed = predecessor.cp_air_assignment_executed;
    debug_assert_eq!(assignment_executed, active_input.is_some());

    let zone_cooling_setpoint_load_w = active_input.map(|input| input.zone_cooling_setpoint_load_w);
    let cp_air_j_per_kg_k = active_input.map(|input| input.cp_air_j_per_kg_k);
    let supply_mass_flow_rate_kg_per_s =
        active_input.map(|input| input.supply_mass_flow_rate_kg_per_s);
    let cp_air_times_supply_mass_flow_rate_w_per_k =
        active_input.map(|input| input.cp_air_j_per_kg_k * input.supply_mass_flow_rate_kg_per_s);
    let zone_cooling_setpoint_load_over_denominator_c = active_input
        .zip(cp_air_times_supply_mass_flow_rate_w_per_k)
        .map(|(input, denominator)| input.zone_cooling_setpoint_load_w / denominator);
    let zone_node_temperature_c = active_input.map(|input| input.zone_node_temperature_c);
    let calculated_supply_temperature_c = zone_cooling_setpoint_load_over_denominator_c
        .zip(zone_node_temperature_c)
        .map(|(quotient, zone_node_temperature)| quotient + zone_node_temperature);
    let supply_temperature_c = calculated_supply_temperature_c;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute::
            PositiveGuardFalseFallthrough
    } else {
        state.supply_temperature_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len();
        state.zone_cooling_setpoint_load_read_count += 1;
        state.cp_air_read_count += 1;
        state.supply_mass_flow_rate_read_count += 1;
        state.cp_air_times_supply_mass_flow_rate_calculation_count += 1;
        state.zone_cooling_setpoint_load_over_denominator_calculation_count += 1;
        state.zone_node_temperature_read_count += 1;
        state.supply_temperature_calculation_count += 1;
        state.supply_temperature_assignment_write_count += 1;
        state.witnessed_supply_temperature_assignment_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute::
            SupplyTemperatureAssigned
    };

    let snapshot = PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
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
        supply_temperature_c,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
