//! Persistent CP373 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment::transition::predecessor_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state = &unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_route_counts_match(state, prior, route)
}

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor) -> bool {
    let route = predecessor_route(predecessor);
    let active = matches!(
        route,
        Some(
            Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted
                | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted
        )
    );
    let operands = active.then_some(ActiveOperands {
        supply_mass_flow_rate_kg_per_s: 1.0,
        zone_node_humidity_ratio: 0.0,
    });
    let mut probe = state.clone();
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_state(
        &mut probe,
        predecessor,
        operands,
    )
    .is_some()
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && completed_route_counts_match(state, prior)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_metadata_is_consistent(
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
        state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]) else {
        return false;
    };
    let Some(active) = state
        .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count
        .checked_add(
            state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count,
        )
    else {
        return false;
    };
    let Some(source_sites) = active.checked_mul(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER.len(),
    ) else {
        return false;
    };
    state.system == system
        && partition == state.transition_count
        && state.source_site_execution_count == source_sites
        && site_counters_match_active(state, active)
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
    route: Route,
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
) -> [(usize, usize, Route); 8] {
    [
        (state.unit_off_skip_count, prior.unit_off_skip_count, Route::UnitOff),
        (state.non_cooling_skip_count, prior.non_cooling_skip_count, Route::NonCooling),
        (state.positive_guard_false_fallthrough_skip_count, prior.positive_guard_false_fallthrough_skip_count, Route::PositiveGuardFalseFallthrough),
        (state.heating_availability_guard_false_fallthrough_count, prior.heating_availability_guard_false_fallthrough_count, Route::HeatingAvailabilityGuardFalseFallthrough),
        (state.humidification_control_guard_false_fallthrough_count, prior.humidification_control_guard_false_fallthrough_count, Route::HumidificationControlGuardFalseFallthrough),
        (state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count, prior.dehumidification_control_humidistat_moisture_demand_assignment_count, Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted),
        (state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count, prior.dehumidification_control_none_moisture_demand_assignment_count, Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted),
        (state.dehumidification_control_guard_false_fallthrough_count, prior.dehumidification_control_guard_false_fallthrough_count, Route::DehumidificationControlGuardFalseFallthrough),
    ]
}

fn route_count(state: &State, route: Route) -> usize {
    match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::HeatingAvailabilityGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_count,
        Route::HumidificationControlGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_count,
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted => state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count,
        Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted => state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count,
        Route::DehumidificationControlGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_count,
    }
}

fn site_counters_match_active(state: &State, active: usize) -> bool {
    [
        state.zone_humidifying_setpoint_moisture_demand_read_count,
        state.supply_mass_flow_rate_read_count,
        state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
        state.zone_node_humidity_ratio_read_count,
        state.supply_humidity_ratio_for_humidification_calculation_count,
        state.supply_humidity_ratio_for_humidification_assignment_count,
    ]
    .into_iter()
    .all(|count| count == active)
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
