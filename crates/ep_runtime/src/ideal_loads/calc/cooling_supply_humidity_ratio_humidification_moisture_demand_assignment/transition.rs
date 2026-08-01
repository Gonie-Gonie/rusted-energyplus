//! Pure CP371-to-CP372 humidifying-demand assignment transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute as PredecessorRoute,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Predecessor;

/// Same-call value read only on the admitted nested-control routes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentActiveInput {
    pub zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    active_input: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route =
        cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route(
            predecessor,
        )?;
    let route = retained_route(predecessor_route);
    let active = matches!(
        route,
        Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted
            | Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted
    );
    let value = match (active, active_input) {
        (true, Some(input)) => Some(input.zone_humidifying_setpoint_moisture_demand_kg_per_s),
        (false, None) => None,
        _ => return None,
    };
    if !next_transition_fits(state, route, active) {
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
        Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted => {
            state.dehumidification_control_humidistat_moisture_demand_assignment_count += 1;
        }
        Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted => {
            state.dehumidification_control_none_moisture_demand_assignment_count += 1;
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count += 1;
        }
    }
    if active {
        state.humidification_moisture_demand_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len();
        state.zone_humidifying_setpoint_moisture_demand_read_count += 1;
        state.zone_humidifying_setpoint_moisture_demand_assignment_count += 1;
    }

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type_first_read: predecessor.dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type: predecessor.first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat: predecessor.dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read: predecessor.dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type: predecessor.second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none: predecessor.dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered: predecessor.dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough: predecessor.dehumidification_control_guard_false_fallthrough,
        humidification_moisture_demand_assignment_executed: active,
        zone_humidifying_setpoint_moisture_demand_read: active,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: value,
        zone_humidifying_setpoint_moisture_demand_assigned: active,
        assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: value,
        resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: value,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn retained_route(predecessor: PredecessorRoute) -> Route {
    match predecessor {
        PredecessorRoute::UnitOff => Route::UnitOff,
        PredecessorRoute::NonCooling => Route::NonCooling,
        PredecessorRoute::PositiveGuardFalseFallthrough => Route::PositiveGuardFalseFallthrough,
        PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough => {
            Route::HeatingAvailabilityGuardFalseFallthrough
        }
        PredecessorRoute::HumidificationControlGuardFalseFallthrough => {
            Route::HumidificationControlGuardFalseFallthrough
        }
        PredecessorRoute::DehumidificationControlHumidistatBodyEntered => {
            Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted
        }
        PredecessorRoute::DehumidificationControlNoneBodyEntered => {
            Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted
        }
        PredecessorRoute::DehumidificationControlGuardFalseFallthrough => {
            Route::DehumidificationControlGuardFalseFallthrough
        }
    }
}

fn next_transition_fits(state: &State, route: Route, active: bool) -> bool {
    let route_count = match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count
        }
        Route::HumidificationControlGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count
        }
        Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted => {
            state.dehumidification_control_humidistat_moisture_demand_assignment_count
        }
        Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted => {
            state.dehumidification_control_none_moisture_demand_assignment_count
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count
        }
    };
    state.transition_count.checked_add(1).is_some()
        && route_count.checked_add(1).is_some()
        && (!active
            || (state
                .humidification_moisture_demand_assignment_count
                .checked_add(1)
                .is_some()
                && state
                    .source_site_execution_count
                    .checked_add(
                        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len(),
                    )
                    .is_some()
                && state
                    .zone_humidifying_setpoint_moisture_demand_read_count
                    .checked_add(1)
                    .is_some()
                && state
                    .zone_humidifying_setpoint_moisture_demand_assignment_count
                    .checked_add(1)
                    .is_some()))
}
