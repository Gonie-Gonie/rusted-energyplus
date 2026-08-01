//! Persistent CP383 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state,
};
use super::snapshot_validation::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::transition::{
    PredecessorRoute, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

mod counters;

use counters::*;

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_capacity_zero_flow_reset.system == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment
            .transition_count
            == ordinal
        && unit.calc_cooling_capacity_zero_flow_reset.transition_count == ordinal
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard;
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && state_is_one_transition_behind_predecessor(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment,
            route,
        )
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot =
        advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state(
            &mut next,
            predecessor,
            input,
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

pub(in crate::ideal_loads::calc) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    completed_state_parts(
        unit,
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
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
    let prior = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && predecessor_counts_match(state, prior)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment,
        )
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    system: IdealLoadsAirSystemId,
) -> bool {
    let inherited = inherited_counts(state);
    let partitions = predecessor_partitions(state);
    let (false_routes, body_routes) = successor_partitions(state);
    let Some(inherited_total) = checked_sum(&inherited) else {
        return false;
    };
    for lineage in 0..5 {
        let offset = lineage * 3;
        if checked_sum(&partitions[offset..offset + 3]) != Some(inherited[lineage + 3])
            || false_routes[lineage].checked_add(body_routes[lineage])
                != Some(partitions[offset + 2])
        {
            return false;
        }
    }
    let assignments = [partitions[2], partitions[5], partitions[8], partitions[11], partitions[14]];
    let (Some(active), Some(false_total), Some(body_total)) = (
        checked_sum(&assignments),
        checked_sum(&false_routes),
        checked_sum(&body_routes),
    ) else {
        return false;
    };
    let Some(expected_sites) = active
        .checked_mul(3)
        .and_then(|sites| sites.checked_add(body_total))
    else {
        return false;
    };
    state.system == system
        && state.transition_count == inherited_total
        && state.source_site_execution_count == expected_sites
        && active_counters(state).into_iter().all(|count| count == active)
        && state.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count
            == body_total
        && state.dehumidification_total_output_capacity_adjustment_body_entry_count
            == body_total
        && state.dehumidification_total_output_capacity_guard_false_fallthrough_count
            == false_total
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
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == state.system
        && snapshot_route(latest) == Some(route)
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(latest)
        && snapshots_match_bit_exact(latest, witness)
        && route_count(state, route) > 0
}

fn predecessor_counts_match(state: &State, prior: &PredecessorState) -> bool {
    inherited_counts(state) == prior_inherited_counts(prior)
        && predecessor_partitions(state) == prior_partitions(prior)
}

fn state_is_one_transition_behind_predecessor(
    state: &State,
    prior: &PredecessorState,
    route: PredecessorRoute,
) -> bool {
    let ours_inherited = inherited_counts(state);
    let theirs_inherited = prior_inherited_counts(prior);
    let ours_partitions = predecessor_partitions(state);
    let theirs_partitions = prior_partitions(prior);
    ours_inherited
        .into_iter()
        .enumerate()
        .all(|(index, count)| {
            count.checked_add(usize::from(inherited_index(route) == index))
                == Some(theirs_inherited[index])
        })
        && ours_partitions
            .into_iter()
            .enumerate()
            .all(|(index, count)| {
                count.checked_add(usize::from(partition_index(route) == Some(index)))
                    == Some(theirs_partitions[index])
            })
}
