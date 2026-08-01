//! Pure CP370-to-CP371 nested dehumidification-control guard transition.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_control_humidistat_guard::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRetainedRoute as PredecessorRoute,
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_route,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Predecessor;

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    dehumidification_control_type: DehumidificationControlType,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route =
        cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_route(
            predecessor,
        )?;
    let evaluate = predecessor_route == PredecessorRoute::HumidificationControlBodyEntered;
    let first_control = evaluate.then_some(dehumidification_control_type);
    let first_is_humidistat =
        first_control.map(|control| control == DehumidificationControlType::Humidistat);
    let second_control = if first_is_humidistat == Some(false) {
        Some(dehumidification_control_type)
    } else {
        None
    };
    let second_is_none = second_control.map(|control| control == DehumidificationControlType::None);
    let body_entered = first_is_humidistat == Some(true) || second_is_none == Some(true);
    let false_fallthrough = evaluate && !body_entered;
    let route = route(predecessor_route, first_is_humidistat, second_is_none);
    let source_sites = if !evaluate {
        0
    } else if first_is_humidistat == Some(true) {
        3
    } else if second_is_none == Some(true) {
        5
    } else {
        4
    };
    if !next_transition_fits(
        state,
        predecessor,
        evaluate,
        first_is_humidistat,
        second_is_none,
        source_sites,
    ) {
        return None;
    }

    state.transition_count += 1;
    state.unit_off_skip_count += usize::from(predecessor.unit_off_skipped);
    state.non_cooling_skip_count += usize::from(predecessor.non_cooling_skipped);
    state.positive_guard_false_fallthrough_skip_count +=
        usize::from(predecessor.positive_guard_false_fallthrough_skipped);
    state.dehumidification_control_none_case_completed_skip_count +=
        usize::from(predecessor.dehumidification_control_none_case_completed_skip);
    state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count +=
        usize::from(
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        );
    state.dehumidification_control_humidistat_case_completed_skip_count +=
        usize::from(predecessor.dehumidification_control_humidistat_case_completed_skip);
    state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count +=
        usize::from(
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        );
    state.heating_on_read_count += usize::from(predecessor.predecessor_heating_on_read);
    state.heating_on_body_entry_count += usize::from(
        predecessor.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
    );
    state.heating_on_guard_false_fallthrough_count +=
        usize::from(predecessor.predecessor_heating_on_guard_false_fallthrough);
    state.humidification_control_type_read_count +=
        usize::from(predecessor.humidification_control_type_read);
    state.humidification_control_type_humidistat_comparison_count +=
        usize::from(predecessor.humidification_control_type_humidistat.is_some());
    state.humidification_control_body_entry_count +=
        usize::from(predecessor.humidification_control_body_entered);
    state.humidification_control_guard_false_fallthrough_count +=
        usize::from(predecessor.humidification_control_guard_false_fallthrough);
    state.dehumidification_control_type_first_read_count += usize::from(evaluate);
    state.dehumidification_control_type_humidistat_comparison_count += usize::from(evaluate);
    state.dehumidification_control_type_humidistat_match_count +=
        usize::from(first_is_humidistat == Some(true));
    state.dehumidification_control_type_second_read_count +=
        usize::from(first_is_humidistat == Some(false));
    state.dehumidification_control_type_none_comparison_count +=
        usize::from(first_is_humidistat == Some(false));
    state.dehumidification_control_type_none_match_count +=
        usize::from(second_is_none == Some(true));
    state.dehumidification_control_body_entry_count += usize::from(body_entered);
    state.dehumidification_control_guard_false_fallthrough_count += usize::from(false_fallthrough);
    state.source_site_execution_count += source_sites;

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
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
        predecessor_humidification_control_type_read: predecessor.humidification_control_type_read,
        predecessor_humidification_control_type: predecessor.humidification_control_type,
        predecessor_humidification_control_type_humidistat: predecessor.humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered: predecessor.humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_type_first_read: evaluate,
        first_dehumidification_control_type: first_control,
        dehumidification_control_type_humidistat: first_is_humidistat,
        dehumidification_control_type_second_read: first_is_humidistat == Some(false),
        second_dehumidification_control_type: second_control,
        dehumidification_control_type_none: second_is_none,
        dehumidification_control_body_entered: body_entered,
        dehumidification_control_guard_false_fallthrough: false_fallthrough,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn route(
    predecessor: PredecessorRoute,
    first_is_humidistat: Option<bool>,
    second_is_none: Option<bool>,
) -> Route {
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
        PredecessorRoute::HumidificationControlBodyEntered if first_is_humidistat == Some(true) => {
            Route::DehumidificationControlHumidistatBodyEntered
        }
        PredecessorRoute::HumidificationControlBodyEntered if second_is_none == Some(true) => {
            Route::DehumidificationControlNoneBodyEntered
        }
        PredecessorRoute::HumidificationControlBodyEntered => {
            Route::DehumidificationControlGuardFalseFallthrough
        }
    }
}

fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    evaluate: bool,
    first_is_humidistat: Option<bool>,
    second_is_none: Option<bool>,
    source_sites: usize,
) -> bool {
    let increments = [
        (state.transition_count, 1),
        (
            state.unit_off_skip_count,
            usize::from(predecessor.unit_off_skipped),
        ),
        (
            state.non_cooling_skip_count,
            usize::from(predecessor.non_cooling_skipped),
        ),
        (
            state.positive_guard_false_fallthrough_skip_count,
            usize::from(predecessor.positive_guard_false_fallthrough_skipped),
        ),
        (
            state.dehumidification_control_none_case_completed_skip_count,
            usize::from(predecessor.dehumidification_control_none_case_completed_skip),
        ),
        (
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            usize::from(
                predecessor
                    .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
            ),
        ),
        (
            state.dehumidification_control_humidistat_case_completed_skip_count,
            usize::from(predecessor.dehumidification_control_humidistat_case_completed_skip),
        ),
        (
            state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            usize::from(
                predecessor
                    .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
            ),
        ),
        (
            state.heating_on_read_count,
            usize::from(predecessor.predecessor_heating_on_read),
        ),
        (
            state.heating_on_body_entry_count,
            usize::from(
                predecessor.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
            ),
        ),
        (
            state.heating_on_guard_false_fallthrough_count,
            usize::from(predecessor.predecessor_heating_on_guard_false_fallthrough),
        ),
        (
            state.humidification_control_type_read_count,
            usize::from(predecessor.humidification_control_type_read),
        ),
        (
            state.humidification_control_type_humidistat_comparison_count,
            usize::from(predecessor.humidification_control_type_humidistat.is_some()),
        ),
        (
            state.humidification_control_body_entry_count,
            usize::from(predecessor.humidification_control_body_entered),
        ),
        (
            state.humidification_control_guard_false_fallthrough_count,
            usize::from(predecessor.humidification_control_guard_false_fallthrough),
        ),
        (
            state.dehumidification_control_type_first_read_count,
            usize::from(evaluate),
        ),
        (
            state.dehumidification_control_type_humidistat_comparison_count,
            usize::from(evaluate),
        ),
        (
            state.dehumidification_control_type_humidistat_match_count,
            usize::from(first_is_humidistat == Some(true)),
        ),
        (
            state.dehumidification_control_type_second_read_count,
            usize::from(first_is_humidistat == Some(false)),
        ),
        (
            state.dehumidification_control_type_none_comparison_count,
            usize::from(first_is_humidistat == Some(false)),
        ),
        (
            state.dehumidification_control_type_none_match_count,
            usize::from(second_is_none == Some(true)),
        ),
        (
            state.dehumidification_control_body_entry_count,
            usize::from(first_is_humidistat == Some(true) || second_is_none == Some(true)),
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            usize::from(
                evaluate && first_is_humidistat != Some(true) && second_is_none != Some(true),
            ),
        ),
        (state.source_site_execution_count, source_sites),
    ];
    increments
        .into_iter()
        .all(|(count, increment)| count.checked_add(increment).is_some())
}
