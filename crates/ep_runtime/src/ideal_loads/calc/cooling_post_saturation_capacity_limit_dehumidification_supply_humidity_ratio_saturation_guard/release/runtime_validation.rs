//! Persistent CP413 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::transition::routes::{logical_route_index, predecessor_index_is_split};
use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard.system == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment.transition_count == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state(
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
    let predecessor = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, Some(snapshot), snapshot.system)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
    let predecessor = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && predecessor_counts_match(state, predecessor)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment,
        )
}

fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    if state.transition_count != predecessor.transition_count {
        return false;
    }
    let mut predecessor_index = 0;
    let mut logical_index = 0;
    while predecessor_index < 30 {
        if predecessor_index_is_split(predecessor_index) {
            if state.predecessor_route_counts[logical_index]
                != predecessor.predecessor_guard_false_fallthrough_route_counts[predecessor_index]
                || state.predecessor_route_counts[logical_index + 1]
                    != predecessor.predecessor_maximum_capacity_assignment_route_counts
                        [predecessor_index]
            {
                return false;
            }
            logical_index += 2;
        } else {
            if state.predecessor_route_counts[logical_index]
                != predecessor.predecessor_route_counts[predecessor_index]
            {
                return false;
            }
            logical_index += 1;
        }
        predecessor_index += 1;
    }
    logical_index == 36
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
            count > 0
                && ordinal == count
                && state.predecessor_route_counts[logical_route_index(route)] > 0
                && snapshot_route(latest) == Some(route)
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}

fn counts_are_exact(state: &State) -> bool {
    let predecessor_total = checked_sum(&state.predecessor_route_counts);
    let guard_false_total = checked_sum(&state.guard_false_fallthrough_route_counts);
    let body_total = checked_sum(&state.guard_body_entry_route_counts);
    let inactive_total = checked_sum_where(&state.predecessor_route_counts, |logical_index| {
        predecessor_index_for_logical(logical_index) < 18
    });
    let humidity_total = checked_sum_where(&state.predecessor_route_counts, |logical_index| {
        matches!(predecessor_index_for_logical(logical_index), 18..=29)
    });
    let enthalpy_total = checked_sum_where(&state.predecessor_route_counts, |logical_index| {
        matches!(predecessor_index_for_logical(logical_index), 5 | 8 | 11 | 14 | 17..=29)
    });
    let temperature_total = checked_sum_where(&state.predecessor_route_counts, |logical_index| {
        predecessor_index_for_logical(logical_index) >= 3
    });
    let route_partition_exact = (0..36).all(|logical_index| {
        let active = predecessor_index_for_logical(logical_index) >= 18;
        let branch_total = state.guard_false_fallthrough_route_counts[logical_index]
            .checked_add(state.guard_body_entry_route_counts[logical_index]);
        if active {
            branch_total == Some(state.predecessor_route_counts[logical_index])
        } else {
            state.guard_false_fallthrough_route_counts[logical_index] == 0
                && state.guard_body_entry_route_counts[logical_index] == 0
        }
    });
    let transition_partition = state
        .inactive_transition_count
        .checked_add(state.saturation_supply_humidity_ratio_guard_evaluation_count);
    predecessor_total == Some(state.transition_count)
        && inactive_total == Some(state.inactive_transition_count)
        && transition_partition == Some(state.transition_count)
        && guard_false_total
            == Some(state.saturation_supply_humidity_ratio_guard_false_fallthrough_count)
        && body_total == Some(state.saturation_supply_humidity_ratio_guard_body_entry_count)
        && body_total
            == Some(
                state.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count,
            )
        && guard_false_total.and_then(|false_count| {
            body_total.and_then(|body_count| false_count.checked_add(body_count))
        }) == Some(state.saturation_supply_humidity_ratio_guard_evaluation_count)
        && state
            .saturation_supply_humidity_ratio_guard_evaluation_count
            .checked_mul(3)
            .and_then(|sites| {
                sites.checked_add(state.saturation_supply_humidity_ratio_guard_body_entry_count)
            })
            == Some(state.source_site_execution_count)
        && Some(state.cp412_supply_humidity_ratio_state_owner_count) == humidity_total
        && state.unchanged_supply_humidity_ratio_preservation_count
            == state.cp412_supply_humidity_ratio_state_owner_count
        && Some(state.cp412_supply_enthalpy_state_owner_count) == enthalpy_total
        && state.unchanged_supply_enthalpy_preservation_count
            == state.cp412_supply_enthalpy_state_owner_count
        && Some(state.cp412_supply_temperature_state_owner_count) == temperature_total
        && state.unchanged_supply_temperature_preservation_count
            == state.cp412_supply_temperature_state_owner_count
        && state.cp412_saturation_supply_humidity_ratio_owned_read_count
            == state.saturation_supply_humidity_ratio_guard_evaluation_count
        && state.saturation_supply_humidity_ratio_for_guard_read_count
            == state.saturation_supply_humidity_ratio_guard_evaluation_count
        && state.cp411_original_supply_humidity_ratio_owned_read_count
            == state.saturation_supply_humidity_ratio_guard_evaluation_count
        && state.cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count
            == state.saturation_supply_humidity_ratio_guard_evaluation_count
        && state.original_supply_humidity_ratio_for_guard_read_count
            == state.saturation_supply_humidity_ratio_guard_evaluation_count
        && state.saturation_original_supply_humidity_ratio_comparison_count
            == state.saturation_supply_humidity_ratio_guard_evaluation_count
        && route_partition_exact
}

fn predecessor_index_for_logical(logical_index: usize) -> usize {
    let mut predecessor_index = 0;
    let mut current_logical = 0;
    while predecessor_index < 30 {
        let width = if predecessor_index_is_split(predecessor_index) {
            2
        } else {
            1
        };
        if logical_index < current_logical + width {
            return predecessor_index;
        }
        current_logical += width;
        predecessor_index += 1;
    }
    30
}

fn checked_sum(values: &[usize; 36]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}

fn checked_sum_where(values: &[usize; 36], selected: impl Fn(usize) -> bool) -> Option<usize> {
    values
        .iter()
        .enumerate()
        .filter(|(index, _)| selected(*index))
        .try_fold(0usize, |sum, (_, value)| sum.checked_add(*value))
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_counts_are_exact(state: &State) -> bool {
    counts_are_exact(state)
}
