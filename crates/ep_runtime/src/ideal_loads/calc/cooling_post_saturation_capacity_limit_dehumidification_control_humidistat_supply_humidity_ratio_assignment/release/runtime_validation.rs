//! Persistent CP395 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment.system == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry.transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(
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
    let predecessor = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, Some(snapshot), snapshot.system)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment;
    let predecessor = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && predecessor_counts_match(state, predecessor)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry,
        )
}

fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.transition_count == predecessor.transition_count
        && state.dehumidification_control_humidistat_supply_humidity_ratio_assignment_count
            == predecessor.dehumidification_control_humidistat_case_entry_count
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let predecessor_total = checked_sum(&state.predecessor_route_counts);
    let active_total = checked_selected_sum(&state.predecessor_route_counts, &[19, 23, 26]);
    let humidity_total = checked_selected_sum(&state.predecessor_route_counts, &[18, 22, 28]);
    let temperature_total = (3..30).try_fold(0usize, |sum, index| {
        sum.checked_add(state.predecessor_route_counts[index])
    });
    let enthalpy_total = [5, 8, 11, 14, 17]
        .into_iter()
        .chain(18..30)
        .try_fold(0usize, |sum, index| {
            sum.checked_add(state.predecessor_route_counts[index])
        });
    let assignments =
        state.dehumidification_control_humidistat_supply_humidity_ratio_assignment_count;
    let expected_sites = assignments.checked_mul(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER.len(),
    );
    let active_counts = [
        state.supply_temperature_owned_read_count,
        state.supply_temperature_for_humidity_ratio_inversion_read_count,
        state.supply_enthalpy_owned_read_count,
        state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        state.psychrometric_supply_humidity_ratio_evaluation_count,
        state.supply_humidity_ratio_assignment_write_count,
    ];
    if state.system != expected_system
        || predecessor_total != Some(state.transition_count)
        || state.inactive_transition_count.checked_add(assignments) != Some(state.transition_count)
        || active_total != Some(assignments)
        || humidity_total != Some(state.cp394_supply_humidity_ratio_state_owner_count)
        || humidity_total != Some(state.unchanged_supply_humidity_ratio_preservation_count)
        || temperature_total != Some(state.cp394_supply_temperature_state_owner_count)
        || temperature_total != Some(state.unchanged_supply_temperature_preservation_count)
        || enthalpy_total != Some(state.cp394_supply_enthalpy_state_owner_count)
        || enthalpy_total != Some(state.unchanged_supply_enthalpy_preservation_count)
        || expected_sites != Some(state.source_site_execution_count)
        || active_counts.into_iter().any(|count| count != assignments)
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
