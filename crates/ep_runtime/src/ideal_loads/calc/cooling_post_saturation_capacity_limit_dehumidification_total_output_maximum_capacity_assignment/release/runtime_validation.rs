//! Persistent CP384 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state,
};
use super::snapshot_validation::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::transition::{
    PredecessorRoute, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Predecessor,
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
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment;
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && state_is_one_transition_behind_predecessor(
            state,
            &unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
            route,
        )
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state(
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

pub(in crate::ideal_loads::calc) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    completed_state_parts(
        unit,
        &unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment,
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
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && predecessor_counts_match(state, prior)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
        )
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    system: IdealLoadsAirSystemId,
) -> bool {
    let inherited = inherited_counts(state);
    let partitions = predecessor_partitions(state);
    let (false_routes, assignments) = successor_partitions(state);
    let Some(inherited_total) = checked_sum(&inherited) else {
        return false;
    };
    for lineage in 0..5 {
        let offset = lineage * 3;
        if checked_sum(&partitions[offset..offset + 3]) != Some(inherited[lineage + 3])
            || false_routes[lineage].checked_add(assignments[lineage])
                != Some(partitions[offset + 2])
        {
            return false;
        }
    }
    let active_routes = [
        partitions[2],
        partitions[5],
        partitions[8],
        partitions[11],
        partitions[14],
    ];
    let (Some(active), Some(false_total), Some(assignment_total)) = (
        checked_sum(&active_routes),
        checked_sum(&false_routes),
        checked_sum(&assignments),
    ) else {
        return false;
    };
    let Some(expected_sites) = assignment_total.checked_mul(2) else {
        return false;
    };
    state.system == system
        && state.transition_count == inherited_total
        && state.dehumidification_total_output_capacity_guard_evaluation_count == active
        && state.dehumidification_total_output_capacity_guard_false_fallthrough_count
            == false_total
        && state.dehumidification_total_output_maximum_capacity_assignment_count
            == assignment_total
        && state.source_site_execution_count == expected_sites
        && [
            state.cp383_retained_maximum_total_cooling_capacity_owned_read_count,
            state.maximum_total_cooling_capacity_read_count,
            state.cooling_total_output_assignment_write_count,
        ]
        .into_iter()
        .all(|count| count == assignment_total)
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
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(latest)
        && snapshots_match_bit_exact(latest, witness)
        && route_count(state, route) > 0
}

fn predecessor_counts_match(state: &State, prior: &PredecessorState) -> bool {
    inherited_counts(state) == prior_inherited_counts(prior)
        && predecessor_partitions(state) == prior_partitions(prior)
        && successor_partitions(state) == prior_successor_partitions(prior)
}

fn state_is_one_transition_behind_predecessor(
    state: &State,
    prior: &PredecessorState,
    route: PredecessorRoute,
) -> bool {
    let inherited_ok = one_behind(
        &inherited_counts(state),
        &prior_inherited_counts(prior),
        Some(inherited_index(route)),
    );
    let partitions_ok = one_behind(
        &predecessor_partitions(state),
        &prior_partitions(prior),
        partition_index(route),
    );
    let (our_false, our_assignments) = successor_partitions(state);
    let (their_false, their_assignments) = prior_successor_partitions(prior);
    let successor = successor_index(route);
    inherited_ok
        && partitions_ok
        && one_behind(
            &our_false,
            &their_false,
            successor.and_then(|(assignment, index)| (!assignment).then_some(index)),
        )
        && one_behind(
            &our_assignments,
            &their_assignments,
            successor.and_then(|(assignment, index)| assignment.then_some(index)),
        )
}

fn one_behind<const N: usize>(
    ours: &[usize; N],
    theirs: &[usize; N],
    changed: Option<usize>,
) -> bool {
    ours.iter().enumerate().all(|(index, count)| {
        count.checked_add(usize::from(changed == Some(index))) == Some(theirs[index])
    })
}
