//! Persistent CP394 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry.system == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break.transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(
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
    let predecessor = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break;
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
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry;
    let predecessor = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && predecessor_counts_match(state, predecessor)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break,
        )
}

fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.transition_count == predecessor.transition_count
        && checked_selected_sum(&predecessor.predecessor_route_counts, &[19, 23, 26])
            == Some(state.dehumidification_control_humidistat_case_entry_count)
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let predecessor_total = checked_sum(&state.predecessor_route_counts);
    let active_total = checked_selected_sum(&state.predecessor_route_counts, &[19, 23, 26]);
    let entries = state.dehumidification_control_humidistat_case_entry_count;
    let expected_sites = entries.checked_mul(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER.len(),
    );
    if state.system != expected_system
        || predecessor_total != Some(state.transition_count)
        || state.inactive_transition_count.checked_add(entries) != Some(state.transition_count)
        || active_total != Some(entries)
        || expected_sites != Some(state.source_site_execution_count)
    {
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
                && state.predecessor_route_counts[route.predecessor_index] > 0
                && snapshot_route(latest) == Some(route)
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}

fn checked_selected_sum(route_counts: &[usize; 30], indices: &[usize]) -> Option<usize> {
    indices
        .iter()
        .try_fold(0usize, |sum, index| sum.checked_add(route_counts[*index]))
}
