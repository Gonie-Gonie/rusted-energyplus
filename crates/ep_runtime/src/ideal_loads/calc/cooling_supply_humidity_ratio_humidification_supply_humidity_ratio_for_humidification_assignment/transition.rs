//! Pure CP372-to-CP373 humidification humidity-ratio assignment transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Predecessor;

mod predecessor;
pub(in crate::ideal_loads::calc) use predecessor::predecessor_route;

/// Same-call active values needed after CP372 retains the numerator.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentActiveOperands {
    /// Same-call CP330-owned `SupplyMassFlowRate`.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Explicit pre-sampled `Node(ZoneNodeNum).HumRat`; this is not owner proof.
    pub zone_node_humidity_ratio: f64,
}

struct PreparedValues {
    demand: Option<f64>,
    flow: Option<f64>,
    quotient: Option<f64>,
    zone_humidity: Option<f64>,
    calculated: Option<f64>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    active_operands: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentActiveOperands>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let values = prepare_values(route, predecessor, active_operands)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    state.transition_count += 1;
    match route {
        Route::UnitOff => state.unit_off_skip_count += 1,
        Route::NonCooling => state.non_cooling_skip_count += 1,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
        }
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count += 1;
        }
        Route::HumidificationControlGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count += 1;
        }
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count += 1;
            increment_active_counters(state);
        }
        Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted => {
            state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count += 1;
            increment_active_counters(state);
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count += 1;
        }
    }

    let humidistat_active = route
        == Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted;
    let none_active = route
        == Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted;
    let active = humidistat_active || none_active;
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor.predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: predecessor.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip: predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: predecessor.predecessor_heating_on_read,
        predecessor_heating_on: predecessor.predecessor_heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: predecessor.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough: predecessor.predecessor_heating_on_guard_false_fallthrough,
        predecessor_humidification_control_type_read: predecessor.predecessor_humidification_control_type_read,
        predecessor_humidification_control_type: predecessor.predecessor_humidification_control_type,
        predecessor_humidification_control_type_humidistat: predecessor.predecessor_humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered: predecessor.predecessor_humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough: predecessor.predecessor_humidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type_first_read: predecessor.predecessor_dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type: predecessor.predecessor_first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat: predecessor.predecessor_dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read: predecessor.predecessor_dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type: predecessor.predecessor_second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none: predecessor.predecessor_dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered: predecessor.predecessor_dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough: predecessor.predecessor_dehumidification_control_guard_false_fallthrough,
        predecessor_humidification_moisture_demand_assignment_executed: predecessor.humidification_moisture_demand_assignment_executed,
        predecessor_zone_humidifying_setpoint_moisture_demand_read: predecessor.zone_humidifying_setpoint_moisture_demand_read,
        predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s: predecessor.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        predecessor_zone_humidifying_setpoint_moisture_demand_assigned: predecessor.zone_humidifying_setpoint_moisture_demand_assigned,
        predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: predecessor.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: predecessor.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed: humidistat_active,
        dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed: none_active,
        zone_humidifying_setpoint_moisture_demand_read: active,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: values.demand,
        supply_mass_flow_rate_read: active,
        supply_mass_flow_rate_kg_per_s: values.flow,
        moisture_demand_derived_supply_humidity_ratio_calculated: active,
        moisture_demand_derived_supply_humidity_ratio: values.quotient,
        zone_node_humidity_ratio_read: active,
        zone_node_humidity_ratio: values.zone_humidity,
        supply_humidity_ratio_for_humidification_calculated: active,
        calculated_supply_humidity_ratio_for_humidification: values.calculated,
        supply_humidity_ratio_for_humidification_assigned: active,
        assigned_supply_humidity_ratio_for_humidification: values.calculated,
        resulting_supply_humidity_ratio_for_humidification: values.calculated,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(state: &State, route: Route) -> bool {
    let route_count = route_count(state, route);
    state.transition_count.checked_add(1).is_some()
        && route_count.checked_add(1).is_some()
        && (!route_is_active(route)
            || (state
                .source_site_execution_count
                .checked_add(PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER.len())
                .is_some()
                && all_site_counters_fit(state)))
}

fn prepare_values(
    route: Route,
    predecessor: Predecessor,
    operands: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentActiveOperands>,
) -> Option<PreparedValues> {
    if !route_is_active(route) {
        return operands.is_none().then_some(PreparedValues {
            demand: None,
            flow: None,
            quotient: None,
            zone_humidity: None,
            calculated: None,
        });
    }
    let demand = predecessor.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s?;
    let operands = operands?;
    let quotient = demand / operands.supply_mass_flow_rate_kg_per_s;
    let calculated = quotient + operands.zone_node_humidity_ratio;
    Some(PreparedValues {
        demand: Some(demand),
        flow: Some(operands.supply_mass_flow_rate_kg_per_s),
        quotient: Some(quotient),
        zone_humidity: Some(operands.zone_node_humidity_ratio),
        calculated: Some(calculated),
    })
}

fn route_is_active(route: Route) -> bool {
    matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted
    )
}

fn route_count(state: &State, route: Route) -> usize {
    match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::HeatingAvailabilityGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_count,
        Route::HumidificationControlGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_count,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted => state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count,
        Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted => state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count,
        Route::DehumidificationControlGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_count,
    }
}

fn increment_active_counters(state: &mut State) {
    state.source_site_execution_count +=
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER.len();
    state.zone_humidifying_setpoint_moisture_demand_read_count += 1;
    state.supply_mass_flow_rate_read_count += 1;
    state.moisture_demand_derived_supply_humidity_ratio_calculation_count += 1;
    state.zone_node_humidity_ratio_read_count += 1;
    state.supply_humidity_ratio_for_humidification_calculation_count += 1;
    state.supply_humidity_ratio_for_humidification_assignment_count += 1;
}

fn all_site_counters_fit(state: &State) -> bool {
    [
        state.zone_humidifying_setpoint_moisture_demand_read_count,
        state.supply_mass_flow_rate_read_count,
        state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
        state.zone_node_humidity_ratio_read_count,
        state.supply_humidity_ratio_for_humidification_calculation_count,
        state.supply_humidity_ratio_for_humidification_assignment_count,
    ]
    .into_iter()
    .all(|count| count.checked_add(1).is_some())
}
