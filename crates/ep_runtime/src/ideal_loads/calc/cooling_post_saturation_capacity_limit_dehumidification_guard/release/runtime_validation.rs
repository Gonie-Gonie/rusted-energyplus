//! Persistent CP381 runtime-state validation.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state,
};
use super::snapshot_validation::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard::transition::route_count;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_mixed_air_call.system == system
        && unit
            .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
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
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_post_saturation_capacity_limit_guard
            .transition_count
            == ordinal
        && unit
            .calc_cooling_supply_enthalpy_post_saturation_assignment
            .transition_count
            == ordinal
        && unit
            .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
            .transition_count
            == ordinal
        && unit.calc_cooling_mixed_air_call.transition_count == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state(
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
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard,
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
    let prior = &unit.calc_cooling_post_saturation_capacity_limit_guard;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && predecessor_counts_match(state, prior)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard;
    let prior = &unit.calc_cooling_post_saturation_capacity_limit_guard;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(state, prior)
}

/// Bounded committed snapshot/state proof for the immediate successor.
pub(in crate::ideal_loads::calc) fn committed_latest_snapshot_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    witness: Snapshot,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard;
    system.id == unit.system
        && unit.calc_entry.system == unit.system
        && state.system == unit.system
        && witness.system == system.id
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && state.transition_count > 0
        && state.transition_count == unit.init_call_count
        && state.transition_count == unit.calc_entry.call_count
        && witness.parent_call_ordinal == state.transition_count
        && unit.controlled_zone == Some(witness.controlled_zone)
        && completed_state_is_consistent(unit, witness, Some(witness))
        && cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(
            witness,
        )
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    system: IdealLoadsAirSystemId,
) -> bool {
    let inherited = inherited_counts(state);
    let Some(inherited_partition) = checked_sum(&inherited) else {
        return false;
    };
    let predecessor_body = predecessor_body_counts(state);
    let predecessor_false = predecessor_false_counts(state);
    let dehumidification_body = dehumidification_body_counts(state);
    let dehumidification_false = dehumidification_false_counts(state);
    if inherited[3..]
        .iter()
        .copied()
        .zip(
            predecessor_body
                .iter()
                .copied()
                .zip(predecessor_false.iter().copied()),
        )
        .any(|(expected, (body, guard_false))| body.checked_add(guard_false) != Some(expected))
        || predecessor_body
            .iter()
            .copied()
            .zip(
                dehumidification_body
                    .iter()
                    .copied()
                    .zip(dehumidification_false.iter().copied()),
            )
            .any(|(expected, (body, guard_false))| body.checked_add(guard_false) != Some(expected))
    {
        return false;
    }
    let Some(evaluations) = checked_sum(&predecessor_body) else {
        return false;
    };
    let Some(outer_false) = checked_sum(&predecessor_false) else {
        return false;
    };
    let Some(body_entries) = checked_sum(&dehumidification_body) else {
        return false;
    };
    let Some(guard_false) = checked_sum(&dehumidification_false) else {
        return false;
    };
    let Some(conceptual_partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        outer_false,
        body_entries,
        guard_false,
    ]) else {
        return false;
    };
    let Some(expected_sites) = evaluations
        .checked_mul(3)
        .and_then(|count| count.checked_add(body_entries))
    else {
        return false;
    };
    let counters_match = state.system == system
        && state.transition_count == inherited_partition
        && state.transition_count == conceptual_partition
        && state.dehumidification_guard_evaluation_count == evaluations
        && state.source_site_execution_count == expected_sites
        && state.cp378_supply_humidity_ratio_saturation_limit_owned_read_count == evaluations
        && state.cp379_same_call_supply_humidity_ratio_bit_corroboration_count == evaluations
        && state.purchased_air_supply_humidity_ratio_read_count == evaluations
        && state.cp329_mixed_air_humidity_ratio_owned_read_count == evaluations
        && state.purchased_air_mixed_air_humidity_ratio_read_count == evaluations
        && state.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count == evaluations
        && state.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count
            == body_entries
        && state.dehumidification_body_entry_count == body_entries
        && state.dehumidification_guard_false_fallthrough_count == guard_false
        && body_entries.checked_add(guard_false) == Some(evaluations);
    counters_match && latest_metadata_is_consistent(state, witness)
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
        && cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(latest)
        && snapshots_match_bit_exact(latest, witness)
        && route_count(state, route) > 0
}

fn predecessor_counts_match(state: &State, prior: &PredecessorState) -> bool {
    inherited_counts(state) == predecessor_inherited_counts(prior)
        && predecessor_body_counts(state) == prior_body_counts(prior)
        && predecessor_false_counts(state) == prior_false_counts(prior)
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

fn predecessor_inherited_counts(state: &PredecessorState) -> [usize; 8] {
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

fn predecessor_body_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_body_entry_count,
        state.humidification_control_guard_false_fallthrough_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_body_entry_count,
    ]
}

fn prior_body_counts(state: &PredecessorState) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_body_entry_count,
        state.humidification_control_guard_false_fallthrough_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_body_entry_count,
    ]
}

fn predecessor_false_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
    ]
}

fn prior_false_counts(state: &PredecessorState) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
    ]
}

fn dehumidification_body_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
    ]
}

fn dehumidification_false_counts(state: &State) -> [usize; 5] {
    [
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
    ]
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
