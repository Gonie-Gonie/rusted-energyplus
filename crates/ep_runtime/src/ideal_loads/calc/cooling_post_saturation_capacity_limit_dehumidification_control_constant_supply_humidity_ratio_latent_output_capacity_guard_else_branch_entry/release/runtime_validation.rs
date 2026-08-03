//! Persistent CP406 runtime-state and latest-witness validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_state as advance,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance(&mut next, predecessor)?;
    Some((next, snapshot))
}

pub(super) fn prepared_completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
) -> bool {
    let predecessor = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, Some(snapshot), snapshot.system)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry;
    let predecessor = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && predecessor_counts_match(state, predecessor)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment,
        )
}

fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_maximum_capacity_assignment_route_counts
            == predecessor.cooling_latent_output_maximum_capacity_assignment_route_counts
        && state.else_branch_entry_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && predecessor
            .inactive_transition_count
            .checked_add(predecessor.cooling_latent_output_maximum_capacity_assignment_count)
            == Some(state.inactive_transition_count)
        && state.predecessor_guard_false_fallthrough_count
            == predecessor.predecessor_guard_false_fallthrough_count
        && state.predecessor_maximum_capacity_assignment_count
            == predecessor.cooling_latent_output_maximum_capacity_assignment_count
        && state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count
            == predecessor.predecessor_guard_false_fallthrough_count
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    if state.system != expected_system || !counts_are_exact(state) {
        return false;
    }
    match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) {
        (0, None, None, None, None) => true,
        (count, Some(latest), Some(route), Some(ordinal), Some(witness)) => {
            let retained_count = if route.guard_evaluated && !route.assignment_executed {
                state.else_branch_entry_route_counts[route.predecessor_index]
            } else if route.assignment_executed {
                state.predecessor_maximum_capacity_assignment_route_counts
                    [route.predecessor_index]
            } else {
                state.predecessor_route_counts[route.predecessor_index]
            };
            count > 0
                && ordinal == count
                && retained_count > 0
                && snapshot_route(latest) == Some(route)
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}

pub(in crate::ideal_loads::calc) fn counts_are_exact(state: &State) -> bool {
    let transition = checked_sum(state.predecessor_route_counts.iter().copied());
    let guard_false = checked_sum(
        state
            .predecessor_guard_false_fallthrough_route_counts
            .iter()
            .copied(),
    );
    let assignment = checked_sum(
        state
            .predecessor_maximum_capacity_assignment_route_counts
            .iter()
            .copied(),
    );
    let else_entry = checked_sum(state.else_branch_entry_route_counts.iter().copied());
    let partition_exact = (0..30).all(|index| {
        let active = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
        if active {
            state.predecessor_guard_false_fallthrough_route_counts[index].checked_add(
                state.predecessor_maximum_capacity_assignment_route_counts[index],
            ) == Some(state.predecessor_route_counts[index])
        } else {
            state.predecessor_guard_false_fallthrough_route_counts[index] == 0
                && state.predecessor_maximum_capacity_assignment_route_counts[index] == 0
        }
    });
    transition == Some(state.transition_count)
        && guard_false == Some(state.predecessor_guard_false_fallthrough_count)
        && assignment == Some(state.predecessor_maximum_capacity_assignment_count)
        && else_entry
            == Some(
                state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count,
            )
        && state.else_branch_entry_route_counts
            == state.predecessor_guard_false_fallthrough_route_counts
        && state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count
            == state.predecessor_guard_false_fallthrough_count
        && state
            .inactive_transition_count
            .checked_add(
                state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count,
            )
            == Some(state.transition_count)
        && partition_exact
        && state
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count
            .checked_mul(super::super::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER.len())
            == Some(state.source_site_execution_count)
}

fn checked_sum(values: impl IntoIterator<Item = usize>) -> Option<usize> {
    values
        .into_iter()
        .try_fold(0usize, |sum, value| sum.checked_add(value))
}
