//! Pure CP376-to-CP377 saturation-humidity-ratio assignment.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

/// Exact latest writer of `PurchAir.SupplyTemp` read by line 2259.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads) enum PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner
{
    Cp334MixedAirLimit,
    Cp344CapacityMixedAirLimit,
}

/// Owner-resolved operands needed on every active route.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput
{
    /// Exact latest purchased-air supply-temperature bits.
    pub supply_temperature_c: f64,
    /// Exact latest source writer of the temperature bits.
    pub temperature_owner:
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner,
    /// Active environment/coupling-input outdoor barometric pressure bits.
    pub outdoor_barometric_pressure_pa: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_saturation_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let evaluated = prepare_evaluation(route, input)?;
    if !next_transition_fits(state, route, input.map(|input| input.temperature_owner)) {
        return None;
    }

    state.transition_count += 1;
    increment_route_count(state, route);
    if let Some(input) = input {
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
                .len();
        state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count += 1;
        state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count += 1;
        state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count += 1;
        state.local_saturation_supply_humidity_ratio_assignment_count += 1;
        increment_temperature_owner_count(state, input.temperature_owner);
        state.environment_outdoor_barometric_pressure_owner_count += 1;
    }

    let active = route_is_active(route);
    let temperature_owner = input.map(|input| input.temperature_owner);
    let supply_temperature_c = input.map(|input| input.supply_temperature_c);
    let outdoor_barometric_pressure_pa = input.map(|input| input.outdoor_barometric_pressure_pa);
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
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
            .local_supply_humidity_ratio_original_assignment_performed,
        predecessor_resulting_supply_humidity_ratio_original: predecessor
            .resulting_supply_humidity_ratio_original,
        cp334_supply_temperature_mixed_air_limit_owned_read: temperature_owner
            == Some(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner::Cp334MixedAirLimit),
        cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: temperature_owner
            == Some(PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner::Cp344CapacityMixedAirLimit),
        environment_outdoor_barometric_pressure_owned_read: active,
        purchased_air_supply_temperature_for_saturation_humidity_ratio_read: active,
        supply_temperature_for_saturation_humidity_ratio_c: supply_temperature_c,
        environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: active,
        outdoor_barometric_pressure_pa,
        psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: active,
        saturation_supply_humidity_ratio: evaluated,
        local_saturation_supply_humidity_ratio_assignment_performed: active,
        assigned_saturation_supply_humidity_ratio: evaluated,
        resulting_saturation_supply_humidity_ratio: evaluated,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let flags = [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.heating_availability_guard_false_fallthrough,
        predecessor.humidification_control_guard_false_fallthrough,
        predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        predecessor.dehumidification_control_none_maximum_assignment_executed,
        predecessor.dehumidification_control_guard_false_fallthrough,
    ];
    if flags.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    let route = if predecessor.unit_off_skipped {
        Route::UnitOff
    } else if predecessor.non_cooling_skipped {
        Route::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        Route::PositiveGuardFalseFallthrough
    } else if predecessor.heating_availability_guard_false_fallthrough {
        Route::HeatingAvailabilityGuardFalseFallthrough
    } else if predecessor.humidification_control_guard_false_fallthrough {
        Route::HumidificationControlGuardFalseFallthrough
    } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
    } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
        Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
    } else {
        Route::DehumidificationControlGuardFalseFallthrough
    };
    predecessor_shape_matches_route(predecessor, route).then_some(route)
}

fn predecessor_shape_matches_route(predecessor: Predecessor, route: Route) -> bool {
    let active = route_is_active(route);
    let owner_count = [
        predecessor.cp375_maximum_assignment_owned_read,
        predecessor.cp347_none_case_owned_read,
        predecessor.cp356_constant_shr_owned_read,
        predecessor.cp362_humidistat_owned_read,
        predecessor.cp365_constant_supply_humidity_ratio_owned_read,
    ]
    .into_iter()
    .filter(|owner| *owner)
    .count();
    let values = [
        predecessor.purchased_air_supply_humidity_ratio_before_saturation_check,
        predecessor.assigned_supply_humidity_ratio_original,
        predecessor.resulting_supply_humidity_ratio_original,
    ];
    let local_shape = if active {
        predecessor.purchased_air_supply_humidity_ratio_read
            && predecessor.local_supply_humidity_ratio_original_assignment_performed
            && owner_count == 1
            && values.into_iter().all(|value| value.is_some())
            && option_bits_match(
                predecessor.purchased_air_supply_humidity_ratio_before_saturation_check,
                predecessor.assigned_supply_humidity_ratio_original,
            )
            && option_bits_match(
                predecessor.assigned_supply_humidity_ratio_original,
                predecessor.resulting_supply_humidity_ratio_original,
            )
    } else {
        !predecessor.purchased_air_supply_humidity_ratio_read
            && !predecessor.local_supply_humidity_ratio_original_assignment_performed
            && owner_count == 0
            && values.into_iter().all(|value| value.is_none())
    };
    let source_owner_shape = match route {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
        | Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            predecessor.predecessor_purchased_air_supply_humidity_ratio_assignment_performed
                && predecessor
                    .predecessor_resulting_supply_humidity_ratio
                    .is_some()
                && predecessor.cp375_maximum_assignment_owned_read
                && option_bits_match(
                    predecessor.predecessor_resulting_supply_humidity_ratio,
                    predecessor.purchased_air_supply_humidity_ratio_before_saturation_check,
                )
        }
        Route::HeatingAvailabilityGuardFalseFallthrough
        | Route::HumidificationControlGuardFalseFallthrough
        | Route::DehumidificationControlGuardFalseFallthrough => {
            !predecessor.predecessor_purchased_air_supply_humidity_ratio_assignment_performed
                && predecessor
                    .predecessor_resulting_supply_humidity_ratio
                    .is_none()
                && predecessor_owner_matches_selector(predecessor)
        }
        _ => {
            predecessor
                .predecessor_dehumidification_control_type
                .is_none()
                && !predecessor.predecessor_purchased_air_supply_humidity_ratio_assignment_performed
                && predecessor
                    .predecessor_resulting_supply_humidity_ratio
                    .is_none()
        }
    };
    local_shape && source_owner_shape
}

fn predecessor_owner_matches_selector(predecessor: Predecessor) -> bool {
    match predecessor.predecessor_dehumidification_control_type {
        Some(DehumidificationControlType::None) => predecessor.cp347_none_case_owned_read,
        Some(DehumidificationControlType::ConstantSensibleHeatRatio) => {
            predecessor.cp356_constant_shr_owned_read
        }
        Some(DehumidificationControlType::Humidistat) => predecessor.cp362_humidistat_owned_read,
        Some(DehumidificationControlType::ConstantSupplyHumidityRatio) => {
            predecessor.cp365_constant_supply_humidity_ratio_owned_read
        }
        None => false,
    }
}

fn prepare_evaluation(
    route: Route,
    input: Option<PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput>,
) -> Option<Option<f64>> {
    if !route_is_active(route) {
        return input.is_none().then_some(None);
    }
    let input = input?;
    Some(Some(energyplus_psy_w_fn_tdb_rh_pb(
        input.supply_temperature_c,
        1.0,
        input.outdoor_barometric_pressure_pa,
    )))
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(
    state: &State,
    route: Route,
    owner: Option<PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner>,
) -> bool {
    state.transition_count.checked_add(1).is_some()
        && route_count(state, route).checked_add(1).is_some()
        && (!route_is_active(route)
            || (state
                .source_site_execution_count
                .checked_add(
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
                        .len(),
                )
                .is_some()
                && state
                    .purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count
                    .checked_add(1)
                    .is_some()
                && state
                    .environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count
                    .checked_add(1)
                    .is_some()
                && state
                    .psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count
                    .checked_add(1)
                    .is_some()
                && state
                    .local_saturation_supply_humidity_ratio_assignment_count
                    .checked_add(1)
                    .is_some()
                && owner.is_some_and(|owner| {
                    temperature_owner_count(state, owner).checked_add(1).is_some()
                })
                && state
                    .environment_outdoor_barometric_pressure_owner_count
                    .checked_add(1)
                    .is_some()))
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
    owner: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner,
) -> usize {
    match owner {
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner::Cp334MixedAirLimit => {
            state.cp334_supply_temperature_mixed_air_limit_owner_count
        }
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner::Cp344CapacityMixedAirLimit => {
            state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count
        }
    }
}

fn increment_temperature_owner_count(
    state: &mut State,
    owner: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner,
) {
    match owner {
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner::Cp334MixedAirLimit => {
            state.cp334_supply_temperature_mixed_air_limit_owner_count += 1;
        }
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner::Cp344CapacityMixedAirLimit => {
            state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count += 1;
        }
    }
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
