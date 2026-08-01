//! Pure CP368-to-CP369 Cooling humidification heating-availability guard transition.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PredecessorRoute {
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    NoneCaseCompleted,
    ConstantSensibleHeatRatioCaseCompleted,
    HumidistatCaseCompleted,
    ConstantSupplyHumidityRatioCaseCompleted,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    heating_on: bool,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let active = matches!(
        predecessor_route,
        PredecessorRoute::NoneCaseCompleted
            | PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted
            | PredecessorRoute::HumidistatCaseCompleted
            | PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted
    );
    let route = match predecessor_route {
        PredecessorRoute::UnitOff => Route::UnitOff,
        PredecessorRoute::NonCooling => Route::NonCooling,
        PredecessorRoute::PositiveGuardFalseFallthrough => {
            Route::PositiveGuardFalseFallthrough
        }
        _ if heating_on => Route::HeatingAvailabilityBodyEntered,
        _ => Route::HeatingAvailabilityGuardFalseFallthrough,
    };
    if !next_transition_fits(state, predecessor_route, route) {
        return None;
    }

    state.transition_count += 1;
    match predecessor_route {
        PredecessorRoute::UnitOff => state.unit_off_skip_count += 1,
        PredecessorRoute::NonCooling => state.non_cooling_skip_count += 1,
        PredecessorRoute::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
            state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        }
        PredecessorRoute::NoneCaseCompleted => {
            state.dehumidification_control_none_case_completed_skip_count += 1;
            state.witnessed_dehumidification_control_none_case_completed_skip_count += 1;
        }
        PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted => {
            state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count += 1;
        }
        PredecessorRoute::HumidistatCaseCompleted => {
            state.dehumidification_control_humidistat_case_completed_skip_count += 1;
            state.witnessed_dehumidification_control_humidistat_case_completed_skip_count += 1;
        }
        PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted => {
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count += 1;
        }
    }
    if active {
        state.heating_on_read_count += 1;
        state.source_site_execution_count += 1;
        if heating_on {
            state.heating_on_body_entry_count += 1;
            state.witnessed_heating_on_body_entry_count += 1;
            state.source_site_execution_count += 1;
        } else {
            state.heating_on_guard_false_fallthrough_count += 1;
            state.witnessed_heating_on_guard_false_fallthrough_count += 1;
        }
    }

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: predecessor
            .dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            predecessor
                .dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: matches!(
            predecessor_route,
            PredecessorRoute::NoneCaseCompleted
        ),
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: matches!(
            predecessor_route,
            PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted
        ),
        dehumidification_control_humidistat_case_completed_skip: matches!(
            predecessor_route,
            PredecessorRoute::HumidistatCaseCompleted
        ),
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: matches!(
            predecessor_route,
            PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted
        ),
        heating_on_read: active,
        heating_on: active.then_some(heating_on),
        cooling_supply_humidity_ratio_humidification_body_entered: active && heating_on,
        heating_on_guard_false_fallthrough: active && !heating_on,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(super) fn predecessor_route(predecessor: Predecessor) -> Option<PredecessorRoute> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER
        || predecessor
            .dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
    {
        return None;
    }
    let active_count = usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(predecessor.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        );
    if predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && active_count == 0
    {
        return Some(PredecessorRoute::UnitOff);
    }
    if !predecessor.unit_off_skipped
        && predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && active_count == 0
    {
        return Some(PredecessorRoute::NonCooling);
    }
    if positive_guard_fallthrough(predecessor) && active_count == 0 {
        return Some(PredecessorRoute::PositiveGuardFalseFallthrough);
    }
    if !active_prefix(predecessor) || active_count != 1 {
        return None;
    }
    match predecessor.predecessor_dehumidification_control_type? {
        DehumidificationControlType::None
            if predecessor.dehumidification_control_none_case_completed_skip
                && predecessor.predecessor_dehumidification_control_none_case_completed_skip =>
        {
            Some(PredecessorRoute::NoneCaseCompleted)
        }
        DehumidificationControlType::ConstantSensibleHeatRatio
            if predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
                && predecessor
                    .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =>
        {
            Some(PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted)
        }
        DehumidificationControlType::Humidistat
            if predecessor.dehumidification_control_humidistat_case_completed_skip
                && predecessor
                    .predecessor_dehumidification_control_humidistat_case_completed_skip =>
        {
            Some(PredecessorRoute::HumidistatCaseCompleted)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
                && predecessor
                    .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip =>
        {
            Some(PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted)
        }
        _ => None,
    }
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> bool {
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    let predecessor_fits = match predecessor_route {
        PredecessorRoute::UnitOff => state.unit_off_skip_count.checked_add(1).is_some(),
        PredecessorRoute::NonCooling => state.non_cooling_skip_count.checked_add(1).is_some(),
        PredecessorRoute::PositiveGuardFalseFallthrough => checked_pair(
            state.positive_guard_false_fallthrough_skip_count,
            state.witnessed_positive_guard_false_fallthrough_skip_count,
        ),
        PredecessorRoute::NoneCaseCompleted => checked_pair(
            state.dehumidification_control_none_case_completed_skip_count,
            state.witnessed_dehumidification_control_none_case_completed_skip_count,
        ),
        PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted => checked_pair(
            state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        PredecessorRoute::HumidistatCaseCompleted => checked_pair(
            state.dehumidification_control_humidistat_case_completed_skip_count,
            state.witnessed_dehumidification_control_humidistat_case_completed_skip_count,
        ),
        PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted => checked_pair(
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        ),
    };
    if !predecessor_fits {
        return false;
    }
    match route {
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough => true,
        Route::HeatingAvailabilityBodyEntered => {
            state.heating_on_read_count.checked_add(1).is_some()
                && checked_pair(
                    state.heating_on_body_entry_count,
                    state.witnessed_heating_on_body_entry_count,
                )
                && state.source_site_execution_count.checked_add(2).is_some()
        }
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_on_read_count.checked_add(1).is_some()
                && checked_pair(
                    state.heating_on_guard_false_fallthrough_count,
                    state.witnessed_heating_on_guard_false_fallthrough_count,
                )
                && state.source_site_execution_count.checked_add(1).is_some()
        }
    }
}

fn positive_guard_fallthrough(predecessor: Predecessor) -> bool {
    !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && predecessor.positive_guard_false_fallthrough_skipped
        && predecessor.predecessor_dehumidification_control_type.is_none()
}

fn active_prefix(predecessor: Predecessor) -> bool {
    !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && predecessor.predecessor_cooling_body_entered
        && predecessor.predecessor_no_outdoor_air_fallback_entered
        && predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && predecessor.predecessor_dehumidification_control_type.is_some()
}

fn inactive_prefix(predecessor: Predecessor) -> bool {
    !predecessor.predecessor_cooling_body_entered
        && !predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && predecessor.predecessor_dehumidification_control_type.is_none()
}

fn checked_pair(left: usize, right: usize) -> bool {
    left.checked_add(1).is_some() && right.checked_add(1).is_some()
}