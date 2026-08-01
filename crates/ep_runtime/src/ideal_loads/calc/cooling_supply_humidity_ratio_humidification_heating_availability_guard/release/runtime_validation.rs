//! Persistent CP369 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Snapshot,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_heating_availability_guard::transition::{
    PredecessorRoute, next_transition_fits as pure_next_transition_fits, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_default_supply_humidity_ratio_case_break.system == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_default_supply_humidity_ratio_case_break
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
    heating_on: bool,
) -> bool {
    let Some(predecessor_route) = predecessor_route(predecessor) else {
        return false;
    };
    let state =
        &unit.calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard;
    let prior = &unit.calc_cooling_default_supply_humidity_ratio_case_break;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_route_counts_match(state, prior, predecessor_route)
        && pending_guard_counts_match(state, prior, predecessor_route, heating_on)
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    heating_on: bool,
) -> bool {
    predecessor_route(predecessor).is_some_and(|predecessor_route| {
        let active = is_active(predecessor_route);
        let route = match predecessor_route {
            PredecessorRoute::UnitOff => Route::UnitOff,
            PredecessorRoute::NonCooling => Route::NonCooling,
            PredecessorRoute::PositiveGuardFalseFallthrough => {
                Route::PositiveGuardFalseFallthrough
            }
            _ if heating_on => Route::HeatingAvailabilityBodyEntered,
            _ => Route::HeatingAvailabilityGuardFalseFallthrough,
        };
        (!active || matches!(route, Route::HeatingAvailabilityBodyEntered | Route::HeatingAvailabilityGuardFalseFallthrough))
            && pure_next_transition_fits(state, predecessor_route, route)
    })
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state =
        &unit.calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard;
    let prior = &unit.calc_cooling_default_supply_humidity_ratio_case_break;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && route_counts_match(state, prior)
        && state.latest.is_some_and(|latest| snapshots_match_exact(latest, snapshot))
        && state.latest_transition_ordinal == Some(state.transition_count)
}

pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_metadata_is_consistent(
    state: &State,
    expected_transition_count: usize,
) -> bool {
    state.transition_count == expected_transition_count
        && state_is_consistent(
            state,
            state.latest,
            state.system,
        )
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
    let Some(guard_partition) = state
        .heating_on_body_entry_count
        .checked_add(state.heating_on_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(source_count) = state
        .heating_on_read_count
        .checked_add(state.heating_on_body_entry_count)
    else {
        return false;
    };
    state.system == system
        && route_partition == state.transition_count
        && state.heating_on_read_count == active
        && guard_partition == active
        && state.source_site_execution_count == source_count
        && state.witnessed_positive_guard_false_fallthrough_skip_count
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
        && latest_metadata_is_consistent(state, witness)
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
        Route::HeatingAvailabilityBodyEntered => {
            selector_count > 0 && state.heating_on_body_entry_count > 0
        }
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            selector_count > 0 && state.heating_on_guard_false_fallthrough_count > 0
        }
    }
}

fn pending_route_counts_match(
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
            route == PredecessorRoute::NoneCaseCompleted,
        ) == Some(prior.dehumidification_control_none_case_completed_skip_count)
        && pending_count(
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            route == PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted,
        ) == Some(prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count)
        && pending_count(
            state.dehumidification_control_humidistat_case_completed_skip_count,
            route == PredecessorRoute::HumidistatCaseCompleted,
        ) == Some(prior.dehumidification_control_humidistat_case_completed_skip_count)
        && pending_count(
            state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            route == PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted,
        ) == Some(prior.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count)
}

fn pending_guard_counts_match(
    state: &State,
    prior: &PredecessorState,
    route: PredecessorRoute,
    heating_on: bool,
) -> bool {
    let Some(active) = checked_sum(&[
        prior.dehumidification_control_none_case_completed_skip_count,
        prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        prior.dehumidification_control_humidistat_case_completed_skip_count,
        prior.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ]) else {
        return false;
    };
    let entered = is_active(route) && heating_on;
    let fell_through = is_active(route) && !heating_on;
    pending_count(state.heating_on_read_count, is_active(route)) == Some(active)
        && pending_count(state.heating_on_body_entry_count, entered)
            .zip(pending_count(
                state.heating_on_guard_false_fallthrough_count,
                fell_through,
            ))
            .is_some_and(|(body, fallthrough)| body.checked_add(fallthrough) == Some(active))
        && state
            .source_site_execution_count
            .checked_add(usize::from(is_active(route)) + usize::from(entered))
            == state
                .heating_on_read_count
                .checked_add(usize::from(is_active(route)))
                .and_then(|read| {
                    state
                        .heating_on_body_entry_count
                        .checked_add(usize::from(entered))
                        .and_then(|body| read.checked_add(body))
                })
}

fn route_counts_match(state: &State, prior: &PredecessorState) -> bool {
    state.unit_off_skip_count == prior.unit_off_skip_count
        && state.non_cooling_skip_count == prior.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == prior.positive_guard_false_fallthrough_skip_count
        && state.dehumidification_control_none_case_completed_skip_count
            == prior.dehumidification_control_none_case_completed_skip_count
        && state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        && state.dehumidification_control_humidistat_case_completed_skip_count
            == prior.dehumidification_control_humidistat_case_completed_skip_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
            == prior.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
}

fn is_active(route: PredecessorRoute) -> bool {
    matches!(
        route,
        PredecessorRoute::NoneCaseCompleted
            | PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted
            | PredecessorRoute::HumidistatCaseCompleted
            | PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted
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