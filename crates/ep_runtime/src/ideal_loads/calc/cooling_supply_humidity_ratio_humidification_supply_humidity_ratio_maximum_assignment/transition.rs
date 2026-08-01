//! Pure CP374-to-CP375 humidification supply-humidity-ratio maximum assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;

mod predecessor;
pub(in crate::ideal_loads::calc) use predecessor::predecessor_route;

/// Same-call branch-owned purchased-air result-store value needed on active routes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentActiveOperands {
    /// Exact branch-specific owner of `PurchAir.SupplyHumRat` before line 2251.
    pub purchased_air_supply_humidity_ratio: f64,
}

struct PreparedValues {
    left: Option<f64>,
    right: Option<f64>,
    maximum: Option<f64>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    active_operands: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentActiveOperands>,
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
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count += 1;
            increment_active_counters(state);
        }
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count += 1;
            increment_active_counters(state);
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count += 1;
        }
    }

    let humidistat_active = route
        == Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted;
    let none_active = route
        == Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted;
    let active = humidistat_active || none_active;
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed: predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed,
        predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed: predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed,
        predecessor_resulting_supply_humidity_ratio_for_humidification: predecessor.resulting_supply_humidity_ratio_for_humidification,
        dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed: humidistat_active,
        dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed: none_active,
        purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read: active,
        purchased_air_supply_humidity_ratio_before_humidification_supply_maximum: values.left,
        supply_humidity_ratio_for_humidification_for_supply_maximum_read: active,
        supply_humidity_ratio_for_humidification_for_supply_maximum: values.right,
        source_shaped_two_argument_maximum_evaluated: active,
        maximum_supply_humidity_ratio: values.maximum,
        purchased_air_supply_humidity_ratio_assignment_performed: active,
        assigned_supply_humidity_ratio: values.maximum,
        resulting_supply_humidity_ratio: values.maximum,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(state: &State, route: Route) -> bool {
    state.transition_count.checked_add(1).is_some()
        && route_count(state, route).checked_add(1).is_some()
        && (!route_is_active(route)
            || (state
                .source_site_execution_count
                .checked_add(PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER.len())
                .is_some()
                && all_site_counters_fit(state)))
}

fn prepare_values(
    route: Route,
    predecessor: Predecessor,
    operands: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentActiveOperands>,
) -> Option<PreparedValues> {
    if !route_is_active(route) {
        return operands.is_none().then_some(PreparedValues {
            left: None,
            right: None,
            maximum: None,
        });
    }
    let left = operands?.purchased_air_supply_humidity_ratio;
    let right = predecessor.resulting_supply_humidity_ratio_for_humidification?;
    let maximum = source_shaped_two_argument_maximum(left, right);
    Some(PreparedValues {
        left: Some(left),
        right: Some(right),
        maximum: Some(maximum),
    })
}

fn route_is_active(route: Route) -> bool {
    matches!(
        route,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
            | Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
    )
}

fn route_count(state: &State, route: Route) -> usize {
    match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::HeatingAvailabilityGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_count,
        Route::HumidificationControlGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_count,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        Route::DehumidificationControlGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_count,
    }
}

fn increment_active_counters(state: &mut State) {
    state.source_site_execution_count +=
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER.len();
    state.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count += 1;
    state.supply_humidity_ratio_for_humidification_for_supply_maximum_read_count += 1;
    state.source_shaped_two_argument_maximum_evaluation_count += 1;
    state.purchased_air_supply_humidity_ratio_assignment_count += 1;
}

fn all_site_counters_fit(state: &State) -> bool {
    [
        state.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count,
        state.supply_humidity_ratio_for_humidification_for_supply_maximum_read_count,
        state.source_shaped_two_argument_maximum_evaluation_count,
        state.purchased_air_supply_humidity_ratio_assignment_count,
    ]
    .into_iter()
    .all(|count| count.checked_add(1).is_some())
}
