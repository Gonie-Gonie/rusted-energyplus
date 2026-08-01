//! Persistent CP378 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_limit_assignment::transition::{
    predecessor_route, route_count, route_is_active,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as OriginalState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_saturation_assignment
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_supply_humidity_ratio_saturation_assignment
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
    let state = &unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let prior = &unit.calc_cooling_supply_humidity_ratio_saturation_assignment;
    let original = &unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_route_counts_match(state, prior, route)
        && pending_owner_counts_match(state, prior, original, route)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state(
        &mut next,
        predecessor,
    )?;
    Some((next, snapshot))
}

pub(super) fn prepared_completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
) -> bool {
    completed_state_parts(unit, state, snapshot, Some(snapshot))
}

#[allow(dead_code)]
pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    completed_state_parts(
        unit,
        &unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment,
        snapshot,
        witness,
    )
}

fn completed_state_parts(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let prior = &unit.calc_cooling_supply_humidity_ratio_saturation_assignment;
    let original = &unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && completed_route_counts_match(state, prior)
        && completed_owner_counts_match(state, prior, original)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_saturation_limit_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let prior = &unit.calc_cooling_supply_humidity_ratio_saturation_assignment;
    let original = &unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && completed_owner_counts_match(state, prior, original)
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
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]) else {
        return false;
    };
    let Some(active) = checked_sum(&[
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]) else {
        return false;
    };
    let Some(source_sites) = active.checked_mul(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER
            .len(),
    ) else {
        return false;
    };
    state.system == system
        && partition == state.transition_count
        && state.source_site_execution_count == source_sites
        && active_counters(state)
            .into_iter()
            .all(|count| count == active)
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

fn pending_route_counts_match(state: &State, prior: &PredecessorState, route: Route) -> bool {
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

fn route_count_pairs(state: &State, prior: &PredecessorState) -> [(usize, usize, Route); 8] {
    [
        (
            state.unit_off_skip_count,
            prior.unit_off_skip_count,
            Route::UnitOff,
        ),
        (
            state.non_cooling_skip_count,
            prior.non_cooling_skip_count,
            Route::NonCooling,
        ),
        (
            state.positive_guard_false_fallthrough_skip_count,
            prior.positive_guard_false_fallthrough_skip_count,
            Route::PositiveGuardFalseFallthrough,
        ),
        (
            state.heating_availability_guard_false_fallthrough_count,
            prior.heating_availability_guard_false_fallthrough_count,
            Route::HeatingAvailabilityGuardFalseFallthrough,
        ),
        (
            state.humidification_control_guard_false_fallthrough_count,
            prior.humidification_control_guard_false_fallthrough_count,
            Route::HumidificationControlGuardFalseFallthrough,
        ),
        (
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            prior
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
        ),
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            prior.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            prior.dehumidification_control_guard_false_fallthrough_count,
            Route::DehumidificationControlGuardFalseFallthrough,
        ),
    ]
}

fn pending_owner_counts_match(
    state: &State,
    prior: &PredecessorState,
    original: &OriginalState,
    route: Route,
) -> bool {
    let pending = usize::from(route_is_active(route));
    state
        .cp376_original_supply_humidity_ratio_owner_count
        .checked_add(pending)
        == Some(
            original.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count,
        )
        && state
            .cp377_saturation_supply_humidity_ratio_owner_count
            .checked_add(pending)
            == Some(prior.local_saturation_supply_humidity_ratio_assignment_count)
}

fn completed_owner_counts_match(
    state: &State,
    prior: &PredecessorState,
    original: &OriginalState,
) -> bool {
    state.cp376_original_supply_humidity_ratio_owner_count
        == original.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count
        && state.cp377_saturation_supply_humidity_ratio_owner_count
            == prior.local_saturation_supply_humidity_ratio_assignment_count
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

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
