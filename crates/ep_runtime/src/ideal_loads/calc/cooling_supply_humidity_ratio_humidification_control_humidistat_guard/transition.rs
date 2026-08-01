//! Pure CP369-to-CP370 Cooling humidification-control Humidistat-guard transition.

use ep_model::{DehumidificationControlType, HumidificationControlType};

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Predecessor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SelectorRoute {
    None,
    ConstantSensibleHeatRatio,
    Humidistat,
    ConstantSupplyHumidityRatio,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PredecessorRoute {
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    Active {
        selector: SelectorRoute,
        heating_on: bool,
    },
}

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    humidification_control_type: HumidificationControlType,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let evaluate = matches!(
        predecessor_route,
        PredecessorRoute::Active {
            heating_on: true,
            ..
        }
    );
    let humidistat = evaluate
        .then_some(humidification_control_type == HumidificationControlType::Humidistat);
    let body_entered = humidistat == Some(true);
    let false_fallthrough = humidistat == Some(false);
    let route = match predecessor_route {
        PredecessorRoute::UnitOff => Route::UnitOff,
        PredecessorRoute::NonCooling => Route::NonCooling,
        PredecessorRoute::PositiveGuardFalseFallthrough => {
            Route::PositiveGuardFalseFallthrough
        }
        PredecessorRoute::Active {
            heating_on: false,
            ..
        } => Route::HeatingAvailabilityGuardFalseFallthrough,
        _ if body_entered => Route::HumidificationControlBodyEntered,
        _ => Route::HumidificationControlGuardFalseFallthrough,
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
        PredecessorRoute::Active {
            selector,
            heating_on,
        } => {
            increment_selector(state, selector);
            state.heating_on_read_count += 1;
            if heating_on {
                state.heating_on_body_entry_count += 1;
                state.witnessed_heating_on_body_entry_count += 1;
                state.humidification_control_type_read_count += 1;
                state.humidification_control_type_humidistat_comparison_count += 1;
                if body_entered {
                    state.humidification_control_body_entry_count += 1;
                    state.witnessed_humidification_control_body_entry_count += 1;
                    state.source_site_execution_count += 3;
                } else {
                    state.humidification_control_guard_false_fallthrough_count += 1;
                    state.witnessed_humidification_control_guard_false_fallthrough_count += 1;
                    state.source_site_execution_count += 2;
                }
            } else {
                state.heating_on_guard_false_fallthrough_count += 1;
                state.witnessed_heating_on_guard_false_fallthrough_count += 1;
            }
        }
    }

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
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
            .predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: predecessor
            .predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            predecessor
                .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor
            .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip: predecessor
            .dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor
            .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: predecessor.heating_on_read,
        predecessor_heating_on: predecessor.heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: predecessor
            .cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough: predecessor
            .heating_on_guard_false_fallthrough,
        humidification_control_type_read: evaluate,
        humidification_control_type: evaluate.then_some(humidification_control_type),
        humidification_control_type_humidistat: humidistat,
        humidification_control_body_entered: body_entered,
        humidification_control_guard_false_fallthrough: false_fallthrough,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(super) fn predecessor_route(predecessor: Predecessor) -> Option<PredecessorRoute> {
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER
        || predecessor
            .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
    {
        return None;
    }
    let selector_count = selector_count(predecessor);
    if predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && selector_count == 0
        && guard_skipped(predecessor)
    {
        return Some(PredecessorRoute::UnitOff);
    }
    if !predecessor.unit_off_skipped
        && predecessor.non_cooling_skipped
        && predecessor.unit_body_entered
        && inactive_prefix(predecessor)
        && selector_count == 0
        && guard_skipped(predecessor)
    {
        return Some(PredecessorRoute::NonCooling);
    }
    if positive_guard_fallthrough(predecessor)
        && selector_count == 0
        && guard_skipped(predecessor)
    {
        return Some(PredecessorRoute::PositiveGuardFalseFallthrough);
    }
    if !active_prefix(predecessor)
        || selector_count != 1
        || !predecessor.heating_on_read
        || predecessor.heating_on.is_none()
        || predecessor.cooling_supply_humidity_ratio_humidification_body_entered
            == predecessor.heating_on_guard_false_fallthrough
    {
        return None;
    }
    let heating_on = predecessor.heating_on?;
    if predecessor.cooling_supply_humidity_ratio_humidification_body_entered != heating_on
        || predecessor.heating_on_guard_false_fallthrough == heating_on
    {
        return None;
    }
    Some(PredecessorRoute::Active {
        selector: selector_route(predecessor)?,
        heating_on,
    })
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: PredecessorRoute,
    route: Route,
) -> bool {
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    match predecessor {
        PredecessorRoute::UnitOff => state.unit_off_skip_count.checked_add(1).is_some(),
        PredecessorRoute::NonCooling => state.non_cooling_skip_count.checked_add(1).is_some(),
        PredecessorRoute::PositiveGuardFalseFallthrough => checked_pair(
            state.positive_guard_false_fallthrough_skip_count,
            state.witnessed_positive_guard_false_fallthrough_skip_count,
        ),
        PredecessorRoute::Active {
            selector,
            heating_on,
        } => {
            selector_fits(state, selector)
                && state.heating_on_read_count.checked_add(1).is_some()
                && if heating_on {
                    checked_pair(
                        state.heating_on_body_entry_count,
                        state.witnessed_heating_on_body_entry_count,
                    ) && state
                        .humidification_control_type_read_count
                        .checked_add(1)
                        .is_some()
                        && state
                            .humidification_control_type_humidistat_comparison_count
                            .checked_add(1)
                            .is_some()
                        && match route {
                            Route::HumidificationControlBodyEntered => {
                                checked_pair(
                                    state.humidification_control_body_entry_count,
                                    state.witnessed_humidification_control_body_entry_count,
                                ) && state.source_site_execution_count.checked_add(3).is_some()
                            }
                            Route::HumidificationControlGuardFalseFallthrough => {
                                checked_pair(
                                    state.humidification_control_guard_false_fallthrough_count,
                                    state.witnessed_humidification_control_guard_false_fallthrough_count,
                                ) && state.source_site_execution_count.checked_add(2).is_some()
                            }
                            _ => false,
                        }
                } else {
                    route == Route::HeatingAvailabilityGuardFalseFallthrough
                        && checked_pair(
                            state.heating_on_guard_false_fallthrough_count,
                            state.witnessed_heating_on_guard_false_fallthrough_count,
                        )
                }
        }
    }
}

fn increment_selector(state: &mut State, selector: SelectorRoute) {
    match selector {
        SelectorRoute::None => {
            state.dehumidification_control_none_case_completed_skip_count += 1;
            state.witnessed_dehumidification_control_none_case_completed_skip_count += 1;
        }
        SelectorRoute::ConstantSensibleHeatRatio => {
            state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count += 1;
        }
        SelectorRoute::Humidistat => {
            state.dehumidification_control_humidistat_case_completed_skip_count += 1;
            state.witnessed_dehumidification_control_humidistat_case_completed_skip_count += 1;
        }
        SelectorRoute::ConstantSupplyHumidityRatio => {
            state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count += 1;
        }
    }
}

fn selector_fits(state: &State, selector: SelectorRoute) -> bool {
    match selector {
        SelectorRoute::None => checked_pair(
            state.dehumidification_control_none_case_completed_skip_count,
            state.witnessed_dehumidification_control_none_case_completed_skip_count,
        ),
        SelectorRoute::ConstantSensibleHeatRatio => checked_pair(
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state.witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        SelectorRoute::Humidistat => checked_pair(
            state.dehumidification_control_humidistat_case_completed_skip_count,
            state.witnessed_dehumidification_control_humidistat_case_completed_skip_count,
        ),
        SelectorRoute::ConstantSupplyHumidityRatio => checked_pair(
            state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            state.witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        ),
    }
}

fn selector_route(predecessor: Predecessor) -> Option<SelectorRoute> {
    match predecessor.predecessor_dehumidification_control_type? {
        DehumidificationControlType::None
            if predecessor.predecessor_dehumidification_control_none_case_completed_skip
                && predecessor.dehumidification_control_none_case_completed_skip =>
        {
            Some(SelectorRoute::None)
        }
        DehumidificationControlType::ConstantSensibleHeatRatio
            if predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
                && predecessor
                    .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =>
        {
            Some(SelectorRoute::ConstantSensibleHeatRatio)
        }
        DehumidificationControlType::Humidistat
            if predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip
                && predecessor.dehumidification_control_humidistat_case_completed_skip =>
        {
            Some(SelectorRoute::Humidistat)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if predecessor
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
                && predecessor
                    .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip =>
        {
            Some(SelectorRoute::ConstantSupplyHumidityRatio)
        }
        _ => None,
    }
}

fn selector_count(predecessor: Predecessor) -> usize {
    usize::from(predecessor.dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(predecessor.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        )
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
        && predecessor_selector_count(predecessor) == 1
}

fn inactive_prefix(predecessor: Predecessor) -> bool {
    !predecessor.predecessor_cooling_body_entered
        && !predecessor.predecessor_no_outdoor_air_fallback_entered
        && !predecessor.predecessor_positive_supply_mass_flow_body_entered
        && !predecessor.positive_guard_false_fallthrough_skipped
        && predecessor.predecessor_dehumidification_control_type.is_none()
        && predecessor_selector_count(predecessor) == 0
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
        && predecessor_selector_count(predecessor) == 0
}

fn predecessor_selector_count(predecessor: Predecessor) -> usize {
    usize::from(predecessor.predecessor_dehumidification_control_none_case_completed_skip)
        + usize::from(
            predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(
            predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip,
        )
        + usize::from(
            predecessor
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        )
}

fn guard_skipped(predecessor: Predecessor) -> bool {
    !predecessor.heating_on_read
        && predecessor.heating_on.is_none()
        && !predecessor.cooling_supply_humidity_ratio_humidification_body_entered
        && !predecessor.heating_on_guard_false_fallthrough
}

fn checked_pair(left: usize, right: usize) -> bool {
    left.checked_add(1).is_some() && right.checked_add(1).is_some()
}
