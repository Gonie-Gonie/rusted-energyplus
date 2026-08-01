//! Pure CP377-to-CP378 final saturation-limit assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::cooling_supply_humidity_ratio_saturation_assignment_snapshot_route;

struct PreparedValues {
    original: Option<f64>,
    saturation: Option<f64>,
    minimum: Option<f64>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let values = prepare_values(route, predecessor)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    state.transition_count += 1;
    increment_route_count(state, route);
    let active = route_is_active(route);
    if active {
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER
                .len();
        state.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count += 1;
        state.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count += 1;
        state.source_shaped_two_argument_minimum_evaluation_count += 1;
        state.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count += 1;
        state.cp376_original_supply_humidity_ratio_owner_count += 1;
        state.cp377_saturation_supply_humidity_ratio_owner_count += 1;
    }

    let snapshot = Snapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: route == Route::UnitOff,
        non_cooling_skipped: route == Route::NonCooling,
        positive_guard_false_fallthrough_skipped: route
            == Route::PositiveGuardFalseFallthrough,
        heating_availability_guard_false_fallthrough: route
            == Route::HeatingAvailabilityGuardFalseFallthrough,
        humidification_control_guard_false_fallthrough: route
            == Route::HumidificationControlGuardFalseFallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: route
            == Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
        dehumidification_control_none_maximum_assignment_executed: route
            == Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
        dehumidification_control_guard_false_fallthrough: route
            == Route::DehumidificationControlGuardFalseFallthrough,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_local_supply_humidity_ratio_original_assignment_performed: predecessor
            .predecessor_local_supply_humidity_ratio_original_assignment_performed,
        predecessor_resulting_supply_humidity_ratio_original: predecessor
            .predecessor_resulting_supply_humidity_ratio_original,
        predecessor_local_saturation_supply_humidity_ratio_assignment_performed: predecessor
            .local_saturation_supply_humidity_ratio_assignment_performed,
        predecessor_resulting_saturation_supply_humidity_ratio: predecessor
            .resulting_saturation_supply_humidity_ratio,
        cp376_original_supply_humidity_ratio_owned_read: active,
        cp377_saturation_supply_humidity_ratio_owned_read: active,
        local_original_supply_humidity_ratio_for_saturation_limit_minimum_read: active,
        original_supply_humidity_ratio_before_saturation_limit: values.original,
        local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read: active,
        saturation_supply_humidity_ratio_for_limit: values.saturation,
        source_shaped_two_argument_minimum_evaluated: active,
        minimum_supply_humidity_ratio_after_saturation_limit: values.minimum,
        purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed: active,
        assigned_supply_humidity_ratio: values.minimum,
        resulting_supply_humidity_ratio: values.minimum,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_route(predecessor)?;
    if predecessor.unit_off_skipped {
        Some(Route::UnitOff)
    } else if predecessor.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        Some(Route::PositiveGuardFalseFallthrough)
    } else if predecessor.heating_availability_guard_false_fallthrough {
        Some(Route::HeatingAvailabilityGuardFalseFallthrough)
    } else if predecessor.humidification_control_guard_false_fallthrough {
        Some(Route::HumidificationControlGuardFalseFallthrough)
    } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
        Some(Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted)
    } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
        Some(Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted)
    } else if predecessor.dehumidification_control_guard_false_fallthrough {
        Some(Route::DehumidificationControlGuardFalseFallthrough)
    } else {
        None
    }
}

fn prepare_values(route: Route, predecessor: Predecessor) -> Option<PreparedValues> {
    if !route_is_active(route) {
        return Some(PreparedValues {
            original: None,
            saturation: None,
            minimum: None,
        });
    }
    let original = predecessor.predecessor_resulting_supply_humidity_ratio_original?;
    let saturation = predecessor.resulting_saturation_supply_humidity_ratio?;
    Some(PreparedValues {
        original: Some(original),
        saturation: Some(saturation),
        minimum: Some(source_shaped_two_argument_minimum(original, saturation)),
    })
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(state: &State, route: Route) -> bool {
    state.transition_count.checked_add(1).is_some()
        && route_count(state, route).checked_add(1).is_some()
        && (!route_is_active(route)
            || state
                .source_site_execution_count
                .checked_add(
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER
                        .len(),
                )
                .is_some()
                && active_counters(state)
                    .into_iter()
                    .all(|count| count.checked_add(1).is_some()))
}

fn active_counters(state: &State) -> [usize; 6] {
    [
        state.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count,
        state.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count,
        state.source_shaped_two_argument_minimum_evaluation_count,
        state.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count,
        state.cp376_original_supply_humidity_ratio_owner_count,
        state.cp377_saturation_supply_humidity_ratio_owner_count,
    ]
}

pub(in crate::ideal_loads::calc) fn route_is_active(route: Route) -> bool {
    !matches!(
        route,
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough
    )
}

pub(in crate::ideal_loads::calc) fn route_count(state: &State, route: Route) -> usize {
    match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count
        }
        Route::HumidificationControlGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count
        }
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count
        }
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count
        }
    }
}

fn increment_route_count(state: &mut State, route: Route) {
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
        }
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count += 1;
        }
    }
}
