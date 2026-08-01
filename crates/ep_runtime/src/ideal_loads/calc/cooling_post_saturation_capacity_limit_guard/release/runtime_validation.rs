//! Persistent CP380 runtime-state validation.

use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit};

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_guard_state,
};
use super::snapshot_validation::{
    cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release, snapshot_route,
    snapshots_match_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_guard::transition::{
    predecessor_route, predecessor_route_is_active, route_count,
};
use crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRetainedRoute as PredecessorRoute,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as PredecessorState,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_guard
            .system
            == system
        && unit
            .calc_cooling_supply_enthalpy_post_saturation_assignment
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_guard
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_supply_enthalpy_post_saturation_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state = &unit.calc_cooling_post_saturation_capacity_limit_guard;
    let prior = &unit.calc_cooling_supply_enthalpy_post_saturation_assignment;
    state_is_consistent(state, witness, predecessor.system, Some(cooling_limit))
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_route_counts_match(state, prior, route)
        && state
            .capacity_limit_guard_evaluation_count
            .checked_add(usize::from(predecessor_route_is_active(route)))
            == Some(prior.local_supply_enthalpy_after_saturation_limit_assignment_count)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot =
        advance_cooling_post_saturation_capacity_limit_guard_state(&mut next, predecessor, input)?;
    Some((next, snapshot))
}

pub(super) fn prepared_completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    completed_state_parts(unit, state, snapshot, Some(snapshot), cooling_limit)
}

pub(in crate::ideal_loads::calc) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    completed_state_parts(
        unit,
        &unit.calc_cooling_post_saturation_capacity_limit_guard,
        snapshot,
        witness,
        cooling_limit,
    )
}

fn completed_state_parts(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let prior = &unit.calc_cooling_supply_enthalpy_post_saturation_assignment;
    state_is_consistent(state, witness, snapshot.system, Some(cooling_limit))
        && state.transition_count == prior.transition_count
        && completed_route_counts_match(state, prior)
        && state.capacity_limit_guard_evaluation_count
            == prior.local_supply_enthalpy_after_saturation_limit_assignment_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_guard_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_guard;
    let prior = &unit.calc_cooling_supply_enthalpy_post_saturation_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system, None)
        && completed_route_counts_match(state, prior)
        && state.capacity_limit_guard_evaluation_count
            == prior.local_supply_enthalpy_after_saturation_limit_assignment_count
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    system: IdealLoadsAirSystemId,
    expected_cooling_limit: Option<IdealLoadsLimit>,
) -> bool {
    let inherited = inherited_counts(state);
    let Some(inherited_partition) = checked_sum(&inherited) else {
        return false;
    };
    let active_inherited = [
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ];
    let Some(active) = checked_sum(&active_inherited) else {
        return false;
    };
    let body_routes = body_route_counts(state);
    let false_routes = false_route_counts(state);
    let Some(body) = checked_sum(&body_routes) else {
        return false;
    };
    let Some(guard_false) = checked_sum(&false_routes) else {
        return false;
    };
    let Some(conceptual_partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        body,
        guard_false,
    ]) else {
        return false;
    };
    if active_inherited
        .into_iter()
        .zip(body_routes.into_iter().zip(false_routes))
        .any(|(expected, (body, guard_false))| body.checked_add(guard_false) != Some(expected))
    {
        return false;
    }
    let Some(active_partition) = body.checked_add(guard_false) else {
        return false;
    };
    let Some(selector_partition) = checked_sum(&[
        state.cooling_limit_capacity_match_count,
        state.cooling_limit_flow_rate_and_capacity_match_count,
        state.cooling_limit_rejected_count,
    ]) else {
        return false;
    };
    let Some(second_expected_from_algebra) =
        active.checked_sub(state.cooling_limit_capacity_match_count)
    else {
        return false;
    };
    let Some(expected_source_sites) = active
        .checked_mul(2)
        .and_then(|count| {
            state
                .second_cooling_limit_read_count
                .checked_mul(2)
                .and_then(|second| count.checked_add(second))
        })
        .and_then(|count| count.checked_add(body))
    else {
        return false;
    };
    let counters_match = state.system == system
        && inherited_partition == state.transition_count
        && conceptual_partition == state.transition_count
        && active_partition == active
        && selector_partition == active
        && state.capacity_limit_guard_evaluation_count == active
        && state.source_site_execution_count == expected_source_sites
        && state.configured_cooling_limit_owned_read_count == active
        && state.cp337_same_call_selector_lineage_corroboration_count == active
        && state.first_cooling_limit_read_count == active
        && state.cooling_limit_capacity_comparison_count == active
        && state.second_cooling_limit_read_count == second_expected_from_algebra
        && state.cooling_limit_flow_rate_and_capacity_comparison_count
            == second_expected_from_algebra
        && state.cooling_limit_rejected_count == guard_false
        && state.capacity_limit_body_entry_count == body
        && state.active_guard_false_fallthrough_count == guard_false;
    if !counters_match
        || expected_cooling_limit.is_some_and(|limit| {
            !fixed_selector_counts_match(state, active, body, guard_false, limit)
        })
    {
        return false;
    }
    latest_metadata_is_consistent(state, witness, expected_cooling_limit)
}

fn fixed_selector_counts_match(
    state: &State,
    active: usize,
    body: usize,
    guard_false: usize,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let expected_capacity = usize::from(cooling_limit == IdealLoadsLimit::LimitCapacity) * active;
    let expected_second = active - expected_capacity;
    let expected_combined =
        usize::from(cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity) * active;
    let expected_body = expected_capacity + expected_combined;
    let expected_false = active - expected_body;
    state.cooling_limit_capacity_match_count == expected_capacity
        && state.second_cooling_limit_read_count == expected_second
        && state.cooling_limit_flow_rate_and_capacity_match_count == expected_combined
        && body == expected_body
        && guard_false == expected_false
}

fn latest_metadata_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_cooling_limit: Option<IdealLoadsLimit>,
) -> bool {
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
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == state.system
        && snapshot_route(latest) == Some(route)
        && cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(latest)
        && snapshots_match_exact(latest, witness)
        && route_count(state, route) > 0
        && expected_cooling_limit.is_none_or(|limit| latest_input_matches(latest, limit))
}

fn latest_input_matches(latest: Snapshot, cooling_limit: IdealLoadsLimit) -> bool {
    if !latest.capacity_limit_guard_evaluated {
        return true;
    }
    latest.first_cooling_limit == Some(cooling_limit)
        && (!latest.second_cooling_limit_read || latest.second_cooling_limit == Some(cooling_limit))
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
        (
            state.unit_off_skip_count,
            prior.unit_off_skip_count,
            PredecessorRoute::UnitOff,
        ),
        (
            state.non_cooling_skip_count,
            prior.non_cooling_skip_count,
            PredecessorRoute::NonCooling,
        ),
        (
            state.positive_guard_false_fallthrough_skip_count,
            prior.positive_guard_false_fallthrough_skip_count,
            PredecessorRoute::PositiveGuardFalseFallthrough,
        ),
        (
            state.heating_availability_guard_false_fallthrough_count,
            prior.heating_availability_guard_false_fallthrough_count,
            PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough,
        ),
        (
            state.humidification_control_guard_false_fallthrough_count,
            prior.humidification_control_guard_false_fallthrough_count,
            PredecessorRoute::HumidificationControlGuardFalseFallthrough,
        ),
        (
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            prior
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            PredecessorRoute::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
        ),
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            prior.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            PredecessorRoute::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            prior.dehumidification_control_guard_false_fallthrough_count,
            PredecessorRoute::DehumidificationControlGuardFalseFallthrough,
        ),
    ]
}

fn inherited_counts(state: &State) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn body_route_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_body_entry_count,
        state.humidification_control_guard_false_fallthrough_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_body_entry_count,
    ]
}

fn false_route_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
    ]
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
