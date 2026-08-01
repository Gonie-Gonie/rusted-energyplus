//! Persistent CP372 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute as PredecessorRoute,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let Some(route) = cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route(predecessor) else {
        return false;
    };
    let state = &unit
        .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_route_counts_match(state, prior, route)
}

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor) -> bool {
    let route = cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route(predecessor);
    let active = matches!(
        route,
        Some(
            PredecessorRoute::DehumidificationControlHumidistatBodyEntered
                | PredecessorRoute::DehumidificationControlNoneBodyEntered
        )
    );
    let input = active.then_some(ActiveInput {
        zone_humidifying_setpoint_moisture_demand_kg_per_s: 0.0,
    });
    let mut probe = state.clone();
    advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state(
        &mut probe,
        predecessor,
        input,
    )
    .is_some()
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && completed_route_counts_match(state, prior)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_latest_metadata_is_consistent(
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
    let Some(partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        state.dehumidification_control_none_moisture_demand_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]) else {
        return false;
    };
    let Some(active) = state
        .dehumidification_control_humidistat_moisture_demand_assignment_count
        .checked_add(state.dehumidification_control_none_moisture_demand_assignment_count)
    else {
        return false;
    };
    let Some(source_sites) = active.checked_mul(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len(),
    ) else {
        return false;
    };
    state.system == system
        && partition == state.transition_count
        && state.humidification_moisture_demand_assignment_count == active
        && state.source_site_execution_count == source_sites
        && state.zone_humidifying_setpoint_moisture_demand_read_count == active
        && state.zone_humidifying_setpoint_moisture_demand_assignment_count == active
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
        && snapshots_match_bit_exact(latest, witness)
        && route_count(state, route) > 0
}

fn pending_route_counts_match(
    state: &State,
    prior: &PredecessorState,
    route: PredecessorRoute,
) -> bool {
    route_count_pairs(state, prior)
        .into_iter()
        .all(|(current, expected, applies)| {
            current.checked_add(usize::from(applies == route)) == Some(expected)
        })
}

fn completed_route_counts_match(state: &State, prior: &PredecessorState) -> bool {
    route_count_pairs(state, prior)
        .into_iter()
        .all(|(current, expected, _)| current == expected)
}

fn route_count_pairs(
    state: &State,
    prior: &PredecessorState,
) -> [(usize, usize, PredecessorRoute); 8] {
    [
        (state.unit_off_skip_count, prior.unit_off_skip_count, PredecessorRoute::UnitOff),
        (state.non_cooling_skip_count, prior.non_cooling_skip_count, PredecessorRoute::NonCooling),
        (state.positive_guard_false_fallthrough_skip_count, prior.positive_guard_false_fallthrough_skip_count, PredecessorRoute::PositiveGuardFalseFallthrough),
        (state.heating_availability_guard_false_fallthrough_count, prior.heating_on_guard_false_fallthrough_count, PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough),
        (state.humidification_control_guard_false_fallthrough_count, prior.humidification_control_guard_false_fallthrough_count, PredecessorRoute::HumidificationControlGuardFalseFallthrough),
        (state.dehumidification_control_humidistat_moisture_demand_assignment_count, prior.dehumidification_control_type_humidistat_match_count, PredecessorRoute::DehumidificationControlHumidistatBodyEntered),
        (state.dehumidification_control_none_moisture_demand_assignment_count, prior.dehumidification_control_type_none_match_count, PredecessorRoute::DehumidificationControlNoneBodyEntered),
        (state.dehumidification_control_guard_false_fallthrough_count, prior.dehumidification_control_guard_false_fallthrough_count, PredecessorRoute::DehumidificationControlGuardFalseFallthrough),
    ]
}

fn route_count(state: &State, route: Route) -> usize {
    match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::HeatingAvailabilityGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_count,
        Route::HumidificationControlGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_count,
        Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted => state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted => state.dehumidification_control_none_moisture_demand_assignment_count,
        Route::DehumidificationControlGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_count,
    }
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
