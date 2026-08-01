//! Persistent CP370 runtime-state validation.

use ep_model::{HumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Snapshot,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_control_humidistat_guard::transition::{
    PredecessorRoute, SelectorRoute, next_transition_fits as pure_next_transition_fits,
    predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_humidification_flow.system == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
    control: HumidificationControlType,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state = &unit
        .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_carried_counts_match(state, prior, route)
        && pending_control_counts_match(state, prior, route, control)
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    control: HumidificationControlType,
) -> bool {
    predecessor_route(predecessor).is_some_and(|predecessor_route| {
        let route = local_route(predecessor_route, control);
        pure_next_transition_fits(state, predecessor_route, route)
    })
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && carried_counts_match(state, prior)
        && state.humidification_control_type_read_count == prior.heating_on_body_entry_count
        && state.latest.is_some_and(|latest| snapshots_match_exact(latest, snapshot))
        && state.latest_transition_ordinal == Some(state.transition_count)
}

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_metadata_is_consistent(
    state: &State,
    expected_transition_count: usize,
) -> bool {
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    system: IdealLoadsAirSystemId,
) -> bool {
    let Some(route_partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ]) else {
        return false;
    };
    let Some(active) = checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ]) else {
        return false;
    };
    let Some(heating_partition) = state
        .heating_on_body_entry_count
        .checked_add(state.heating_on_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(control_partition) = state
        .humidification_control_body_entry_count
        .checked_add(state.humidification_control_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(source_count) = state
        .humidification_control_type_read_count
        .checked_mul(2)
        .and_then(|count| count.checked_add(state.humidification_control_body_entry_count))
    else {
        return false;
    };
    state.system == system
        && route_partition == state.transition_count
        && state.heating_on_read_count == active
        && heating_partition == active
        && state.humidification_control_type_read_count == state.heating_on_body_entry_count
        && state.humidification_control_type_humidistat_comparison_count
            == state.humidification_control_type_read_count
        && control_partition == state.humidification_control_type_read_count
        && state.source_site_execution_count == source_count
        && witnessed_counts_match(state)
        && latest_metadata_is_consistent(state, witness)
}

fn witnessed_counts_match(state: &State) -> bool {
    state.witnessed_positive_guard_false_fallthrough_skip_count
        == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_dehumidification_control_none_case_completed_skip_count
            == state.dehumidification_control_none_case_completed_skip_count
        && state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        && state.witnessed_dehumidification_control_humidistat_case_completed_skip_count
            == state.dehumidification_control_humidistat_case_completed_skip_count
        && state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
            == state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
        && state.witnessed_heating_on_body_entry_count == state.heating_on_body_entry_count
        && state.witnessed_heating_on_guard_false_fallthrough_count
            == state.heating_on_guard_false_fallthrough_count
        && state.witnessed_humidification_control_body_entry_count
            == state.humidification_control_body_entry_count
        && state.witnessed_humidification_control_guard_false_fallthrough_count
            == state.humidification_control_guard_false_fallthrough_count
}

fn latest_metadata_is_consistent(state: &State, witness: Option<Snapshot>) -> bool {
    if state.transition_count == 0 {
        return state.latest.is_none()
            && state.latest_route.is_none()
            && state.latest_transition_ordinal.is_none()
            && witness.is_none();
    }
    let (Some(latest), Some(route), Some(ordinal), Some(witness)) = (
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) else {
        return false;
    };
    ordinal == state.transition_count
        && latest.system == state.system
        && snapshot_route(latest) == Some(route)
        && snapshots_match_exact(latest, witness)
        && latest_route_is_counted(state, latest, route)
}

fn latest_route_is_counted(state: &State, latest: Snapshot, route: Route) -> bool {
    let selector_count = if latest.dehumidification_control_none_case_completed_skip {
        state.dehumidification_control_none_case_completed_skip_count
    } else if latest.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip {
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
    } else if latest.dehumidification_control_humidistat_case_completed_skip {
        state.dehumidification_control_humidistat_case_completed_skip_count
    } else if latest.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip {
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
    } else {
        0
    };
    match route {
        Route::UnitOff => state.unit_off_skip_count > 0,
        Route::NonCooling => state.non_cooling_skip_count > 0,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count > 0
        }
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            selector_count > 0 && state.heating_on_guard_false_fallthrough_count > 0
        }
        Route::HumidificationControlBodyEntered => {
            selector_count > 0 && state.humidification_control_body_entry_count > 0
        }
        Route::HumidificationControlGuardFalseFallthrough => {
            selector_count > 0
                && state.humidification_control_guard_false_fallthrough_count > 0
        }
    }
}

fn pending_carried_counts_match(
    state: &State,
    prior: &PredecessorState,
    route: PredecessorRoute,
) -> bool {
    pending_count(state.unit_off_skip_count, route == PredecessorRoute::UnitOff)
        == Some(prior.unit_off_skip_count)
        && pending_count(
            state.non_cooling_skip_count,
            route == PredecessorRoute::NonCooling,
        ) == Some(prior.non_cooling_skip_count)
        && pending_count(
            state.positive_guard_false_fallthrough_skip_count,
            route == PredecessorRoute::PositiveGuardFalseFallthrough,
        ) == Some(prior.positive_guard_false_fallthrough_skip_count)
        && pending_count(
            state.dehumidification_control_none_case_completed_skip_count,
            selector_is(route, SelectorRoute::None),
        ) == Some(prior.dehumidification_control_none_case_completed_skip_count)
        && pending_count(
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            selector_is(route, SelectorRoute::ConstantSensibleHeatRatio),
        ) == Some(
            prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        )
        && pending_count(
            state.dehumidification_control_humidistat_case_completed_skip_count,
            selector_is(route, SelectorRoute::Humidistat),
        ) == Some(prior.dehumidification_control_humidistat_case_completed_skip_count)
        && pending_count(
            state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            selector_is(route, SelectorRoute::ConstantSupplyHumidityRatio),
        ) == Some(
            prior.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        )
        && pending_count(state.heating_on_read_count, is_active(route))
            == Some(prior.heating_on_read_count)
        && pending_count(state.heating_on_body_entry_count, evaluates_control(route))
            == Some(prior.heating_on_body_entry_count)
        && pending_count(
            state.heating_on_guard_false_fallthrough_count,
            is_active(route) && !evaluates_control(route),
        ) == Some(prior.heating_on_guard_false_fallthrough_count)
}

fn pending_control_counts_match(
    state: &State,
    prior: &PredecessorState,
    route: PredecessorRoute,
    control: HumidificationControlType,
) -> bool {
    let evaluate = evaluates_control(route);
    let body = evaluate && control == HumidificationControlType::Humidistat;
    let fallthrough = evaluate && !body;
    let Some(read) = pending_count(state.humidification_control_type_read_count, evaluate)
    else {
        return false;
    };
    let Some(comparison) = pending_count(
        state.humidification_control_type_humidistat_comparison_count,
        evaluate,
    ) else {
        return false;
    };
    let Some(body_count) = pending_count(state.humidification_control_body_entry_count, body)
    else {
        return false;
    };
    let Some(false_count) = pending_count(
        state.humidification_control_guard_false_fallthrough_count,
        fallthrough,
    ) else {
        return false;
    };
    let Some(source) = state.source_site_execution_count.checked_add(
        usize::from(evaluate) * 2 + usize::from(body),
    ) else {
        return false;
    };
    read == prior.heating_on_body_entry_count
        && comparison == read
        && body_count.checked_add(false_count) == Some(read)
        && read
            .checked_mul(2)
            .and_then(|count| count.checked_add(body_count))
            == Some(source)
}

fn carried_counts_match(state: &State, prior: &PredecessorState) -> bool {
    state.unit_off_skip_count == prior.unit_off_skip_count
        && state.non_cooling_skip_count == prior.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == prior.positive_guard_false_fallthrough_skip_count
        && state.dehumidification_control_none_case_completed_skip_count
            == prior.dehumidification_control_none_case_completed_skip_count
        && state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == prior
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        && state.dehumidification_control_humidistat_case_completed_skip_count
            == prior.dehumidification_control_humidistat_case_completed_skip_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
            == prior
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
        && state.heating_on_read_count == prior.heating_on_read_count
        && state.heating_on_body_entry_count == prior.heating_on_body_entry_count
        && state.heating_on_guard_false_fallthrough_count
            == prior.heating_on_guard_false_fallthrough_count
}

fn local_route(
    predecessor: PredecessorRoute,
    control: HumidificationControlType,
) -> Route {
    match predecessor {
        PredecessorRoute::UnitOff => Route::UnitOff,
        PredecessorRoute::NonCooling => Route::NonCooling,
        PredecessorRoute::PositiveGuardFalseFallthrough => {
            Route::PositiveGuardFalseFallthrough
        }
        PredecessorRoute::Active {
            heating_on: false,
            ..
        } => Route::HeatingAvailabilityGuardFalseFallthrough,
        _ if control == HumidificationControlType::Humidistat => {
            Route::HumidificationControlBodyEntered
        }
        _ => Route::HumidificationControlGuardFalseFallthrough,
    }
}

fn selector_is(route: PredecessorRoute, expected: SelectorRoute) -> bool {
    matches!(route, PredecessorRoute::Active { selector, .. } if selector == expected)
}

fn is_active(route: PredecessorRoute) -> bool {
    matches!(route, PredecessorRoute::Active { .. })
}

fn evaluates_control(route: PredecessorRoute) -> bool {
    matches!(
        route,
        PredecessorRoute::Active {
            heating_on: true,
            ..
        }
    )
}

fn pending_count(count: usize, current_route: bool) -> Option<usize> {
    count.checked_add(usize::from(current_route))
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
