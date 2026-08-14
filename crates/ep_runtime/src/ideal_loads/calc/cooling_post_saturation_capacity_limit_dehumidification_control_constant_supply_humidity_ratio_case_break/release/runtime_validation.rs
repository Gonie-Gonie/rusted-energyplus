//! Persistent CP409 runtime-state validation.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::super::transition::routes::{RetainedRoute, predecessor_index_is_active};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break.system == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit.transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_state(
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
    let predecessor = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, Some(snapshot), snapshot.system)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break;
    let predecessor = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && predecessor_counts_match(state, predecessor)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit,
        )
}

/// Bounded committed snapshot/state proof for the immediate successor.
pub(in crate::ideal_loads::calc) fn committed_latest_snapshot_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    latest: Snapshot,
    witness: Snapshot,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break;
    let Some(calc_entry_latest) = unit.calc_entry.latest else {
        return false;
    };
    system.id == unit.system
        && state.system == unit.system
        && latest.system == system.id
        && witness.system == system.id
        && unit.calc_entry.system == system.id
        && calc_entry_latest.system == system.id
        && state.transition_count > 0
        && state.transition_count == unit.init_call_count
        && state.transition_count == unit.calc_entry.call_count
        && latest.parent_call_ordinal == state.transition_count
        && witness.parent_call_ordinal == state.transition_count
        && calc_entry_latest.call_ordinal == state.transition_count
        && unit.controlled_zone == Some(latest.controlled_zone)
        && witness.controlled_zone == latest.controlled_zone
        && calc_entry_latest.controlled_zone == latest.controlled_zone
        && snapshot_route(latest).is_some()
        && completed_state_is_consistent(unit, latest, Some(witness))
        && super::snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
            latest,
        )
}

fn predecessor_counts_match(state: &State, predecessor: &PredecessorState) -> bool {
    state.transition_count == predecessor.transition_count
        && state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_count
            == predecessor.predecessor_guard_false_fallthrough_count
        && state.predecessor_maximum_capacity_assignment_count
            == predecessor.predecessor_maximum_capacity_assignment_count
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_maximum_capacity_assignment_route_counts
            == predecessor.predecessor_maximum_capacity_assignment_route_counts
        && state
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count
            .checked_sub(state.predecessor_guard_false_fallthrough_count)
            == Some(state.predecessor_maximum_capacity_assignment_count)
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
                && retained_route_count(state, route) > 0
                && snapshot_route(latest) == Some(route)
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}

fn counts_are_exact(state: &State) -> bool {
    let predecessor_total = checked_sum(&state.predecessor_route_counts);
    let guard_false_total = checked_sum(&state.predecessor_guard_false_fallthrough_route_counts);
    let maximum_total = checked_sum(&state.predecessor_maximum_capacity_assignment_route_counts);
    let breaks = state
        .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count;
    let active_total = checked_selected_sum(&state.predecessor_route_counts);
    let partition_exact = (0..30).all(|index| {
        if predecessor_index_is_active(index) {
            state.predecessor_guard_false_fallthrough_route_counts[index]
                .checked_add(state.predecessor_maximum_capacity_assignment_route_counts[index])
                == Some(state.predecessor_route_counts[index])
        } else {
            state.predecessor_guard_false_fallthrough_route_counts[index] == 0
                && state.predecessor_maximum_capacity_assignment_route_counts[index] == 0
        }
    });
    predecessor_total == Some(state.transition_count)
        && guard_false_total == Some(state.predecessor_guard_false_fallthrough_count)
        && maximum_total == Some(state.predecessor_maximum_capacity_assignment_count)
        && state
            .predecessor_guard_false_fallthrough_count
            .checked_add(state.predecessor_maximum_capacity_assignment_count)
            == Some(breaks)
        && active_total == Some(breaks)
        && state.inactive_transition_count.checked_add(breaks) == Some(state.transition_count)
        && breaks.checked_mul(ORDER.len()) == Some(state.source_site_execution_count)
        && partition_exact
}

fn retained_route_count(state: &State, route: RetainedRoute) -> usize {
    if route.predecessor_guard_false_fallthrough {
        state.predecessor_guard_false_fallthrough_route_counts[route.predecessor_index]
    } else if route.predecessor_maximum_capacity_assignment_executed {
        state.predecessor_maximum_capacity_assignment_route_counts[route.predecessor_index]
    } else {
        state.predecessor_route_counts[route.predecessor_index]
    }
}

fn checked_sum(values: &[usize; 30]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}

fn checked_selected_sum(values: &[usize; 30]) -> Option<usize> {
    [20, 21, 24, 25, 27, 29]
        .into_iter()
        .try_fold(0usize, |sum, index| sum.checked_add(values[index]))
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_counts_are_exact(state: &State) -> bool {
    counts_are_exact(state)
}
