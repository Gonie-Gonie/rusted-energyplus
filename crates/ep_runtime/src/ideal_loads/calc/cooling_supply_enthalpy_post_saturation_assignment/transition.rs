//! Pure CP378-to-CP379 post-saturation enthalpy assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as TemperatureOwner;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_limit_assignment::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRetainedRoute as PredecessorRoute,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_route,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

/// Owner-resolved temperature input needed on every active route.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput
{
    /// Exact latest purchased-air supply-temperature bits retained by CP377.
    pub supply_temperature_c: f64,
    /// Exact transitive CP334-or-CP344 writer of those bits.
    pub temperature_owner: TemperatureOwner,
}

struct PreparedValues {
    supply_temperature_c: Option<f64>,
    supply_humidity_ratio: Option<f64>,
    supply_enthalpy_j_per_kg: Option<f64>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_enthalpy_post_saturation_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let values = prepare_values(route, predecessor, input)?;
    let active = route_is_active(route);
    let temperature_owner = input.map(|input| input.temperature_owner);
    let active_temperature_owner = if active {
        Some(temperature_owner?)
    } else {
        None
    };
    if !next_transition_fits(state, route, temperature_owner) {
        return None;
    }

    state.transition_count += 1;
    increment_route_count(state, route);
    if active {
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER
                .len();
        state.purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count += 1;
        state.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count += 1;
        state.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count += 1;
        state.local_supply_enthalpy_after_saturation_limit_assignment_count += 1;
        if let Some(owner) = active_temperature_owner {
            increment_temperature_owner_count(state, owner);
        }
        state.cp378_supply_humidity_ratio_saturation_limit_owner_count += 1;
    }

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_supply_humidity_ratio_saturation_limit_assignment_performed: predecessor
            .purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed,
        predecessor_resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        cp377_supply_temperature_owned_read: active,
        cp334_supply_temperature_mixed_air_limit_owned_read: temperature_owner
            == Some(TemperatureOwner::Cp334MixedAirLimit),
        cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: temperature_owner
            == Some(TemperatureOwner::Cp344CapacityMixedAirLimit),
        cp378_supply_humidity_ratio_saturation_limit_owned_read: active,
        purchased_air_supply_temperature_for_post_saturation_enthalpy_read: active,
        supply_temperature_c: values.supply_temperature_c,
        purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read: active,
        supply_humidity_ratio: values.supply_humidity_ratio,
        psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated: active,
        psychrometric_supply_enthalpy_j_per_kg: values.supply_enthalpy_j_per_kg,
        local_supply_enthalpy_after_saturation_limit_assignment_performed: active,
        assigned_supply_enthalpy_j_per_kg: values.supply_enthalpy_j_per_kg,
        resulting_supply_enthalpy_j_per_kg: values.supply_enthalpy_j_per_kg,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    Some(
        match cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_route(
            predecessor,
        )? {
            PredecessorRoute::UnitOff => Route::UnitOff,
            PredecessorRoute::NonCooling => Route::NonCooling,
            PredecessorRoute::PositiveGuardFalseFallthrough => {
                Route::PositiveGuardFalseFallthrough
            }
            PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough => {
                Route::HeatingAvailabilityGuardFalseFallthrough
            }
            PredecessorRoute::HumidificationControlGuardFalseFallthrough => {
                Route::HumidificationControlGuardFalseFallthrough
            }
            PredecessorRoute::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
                Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
            }
            PredecessorRoute::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
                Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
            }
            PredecessorRoute::DehumidificationControlGuardFalseFallthrough => {
                Route::DehumidificationControlGuardFalseFallthrough
            }
        },
    )
}

fn prepare_values(
    route: Route,
    predecessor: Predecessor,
    input: Option<PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput>,
) -> Option<PreparedValues> {
    if !route_is_active(route) {
        return input.is_none().then_some(PreparedValues {
            supply_temperature_c: None,
            supply_humidity_ratio: None,
            supply_enthalpy_j_per_kg: None,
        });
    }
    let input = input?;
    if !predecessor.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed {
        return None;
    }
    let supply_humidity_ratio = predecessor.resulting_supply_humidity_ratio?;
    Some(PreparedValues {
        supply_temperature_c: Some(input.supply_temperature_c),
        supply_humidity_ratio: Some(supply_humidity_ratio),
        supply_enthalpy_j_per_kg: Some(energyplus_psy_h_fn_tdb_w(
            input.supply_temperature_c,
            supply_humidity_ratio,
        )),
    })
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    state: &State,
    route: Route,
    owner: Option<TemperatureOwner>,
) -> bool {
    state.transition_count.checked_add(1).is_some()
        && route_count(state, route).checked_add(1).is_some()
        && (!route_is_active(route)
            || state
                .source_site_execution_count
                .checked_add(
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER
                        .len(),
                )
                .is_some()
                && active_counters(state)
                    .into_iter()
                    .all(|count| count.checked_add(1).is_some())
                && owner.is_some_and(|owner| {
                    temperature_owner_count(state, owner)
                        .checked_add(1)
                        .is_some()
                }))
}

fn active_counters(state: &State) -> [usize; 5] {
    [
        state.purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count,
        state.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count,
        state.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count,
        state.local_supply_enthalpy_after_saturation_limit_assignment_count,
        state.cp378_supply_humidity_ratio_saturation_limit_owner_count,
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

pub(in crate::ideal_loads::calc) fn temperature_owner_count(
    state: &State,
    owner: TemperatureOwner,
) -> usize {
    match owner {
        TemperatureOwner::Cp334MixedAirLimit => {
            state.cp334_supply_temperature_mixed_air_limit_owner_count
        }
        TemperatureOwner::Cp344CapacityMixedAirLimit => {
            state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count
        }
    }
}

fn increment_temperature_owner_count(state: &mut State, owner: TemperatureOwner) {
    match owner {
        TemperatureOwner::Cp334MixedAirLimit => {
            state.cp334_supply_temperature_mixed_air_limit_owner_count += 1;
        }
        TemperatureOwner::Cp344CapacityMixedAirLimit => {
            state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count += 1;
        }
    }
}
