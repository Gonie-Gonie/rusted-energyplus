//! Persistent CP412 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::transition::routes::{predecessor_index_is_split, RetainedRoute};
use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER as ORDER,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment.system == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment.transition_count == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state(
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
    let predecessor = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, Some(snapshot), snapshot.system)
        && predecessor_counts_match(state, predecessor)
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
    let predecessor = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment;
    state.transition_count == predecessor.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && predecessor_counts_match(state, predecessor)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment,
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
        && state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count
            == predecessor.supply_humidity_ratio_pre_saturation_original_assignment_count
        && state
            .predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts
            == predecessor
                .supply_humidity_ratio_pre_saturation_original_assignment_route_counts
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
    let original_total = checked_sum(
        &state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts,
    );
    let saturation_total =
        checked_sum(&state.supply_humidity_ratio_saturation_assignment_route_counts);
    let inactive_total = checked_sum_where(&state.predecessor_route_counts, |index| index < 18);
    let humidity_total = checked_sum_where(&state.predecessor_route_counts, |index| {
        matches!(index, 18..=29)
    });
    let enthalpy_total = checked_sum_where(&state.predecessor_route_counts, |index| {
        matches!(index, 5 | 8 | 11 | 14 | 17..=29)
    });
    let temperature_total = checked_sum_where(&state.predecessor_route_counts, |index| index >= 3);
    let route_partition_exact = (0..30).all(|index| {
        let split = predecessor_index_is_split(index);
        let predecessor_split = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(state.predecessor_maximum_capacity_assignment_route_counts[index]);
        (!split
            && state.predecessor_guard_false_fallthrough_route_counts[index] == 0
            && state.predecessor_maximum_capacity_assignment_route_counts[index] == 0)
            || (split && predecessor_split == Some(state.predecessor_route_counts[index]))
    });
    let assignment_partition_exact = (0..30).all(|index| {
        let expected = if matches!(index, 18..=29) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts
            [index]
            == expected
            && state.supply_humidity_ratio_saturation_assignment_route_counts[index] == expected
    });
    let transition_partition = state
        .inactive_transition_count
        .checked_add(state.supply_humidity_ratio_saturation_assignment_count);
    predecessor_total == Some(state.transition_count)
        && guard_false_total == Some(state.predecessor_guard_false_fallthrough_count)
        && maximum_total == Some(state.predecessor_maximum_capacity_assignment_count)
        && original_total
            == Some(
                state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count,
            )
        && saturation_total == Some(state.supply_humidity_ratio_saturation_assignment_count)
        && state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count
            == state.supply_humidity_ratio_saturation_assignment_count
        && inactive_total == Some(state.inactive_transition_count)
        && transition_partition == Some(state.transition_count)
        && state
            .supply_humidity_ratio_saturation_assignment_count
            .checked_mul(ORDER.len())
            == Some(state.source_site_execution_count)
        && Some(state.cp411_supply_humidity_ratio_state_owner_count) == humidity_total
        && state.unchanged_supply_humidity_ratio_preservation_count
            == state.cp411_supply_humidity_ratio_state_owner_count
        && Some(state.cp411_supply_enthalpy_state_owner_count) == enthalpy_total
        && state.unchanged_supply_enthalpy_preservation_count
            == state.cp411_supply_enthalpy_state_owner_count
        && Some(state.cp411_supply_temperature_state_owner_count) == temperature_total
        && state.unchanged_supply_temperature_preservation_count
            == state.cp411_supply_temperature_state_owner_count
        && state.cp411_retained_supply_temperature_owned_read_count
            == state.supply_humidity_ratio_saturation_assignment_count
        && state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count
            == state.supply_humidity_ratio_saturation_assignment_count
        && state.environment_outdoor_barometric_pressure_owner_count
            == state.supply_humidity_ratio_saturation_assignment_count
        && state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count
            == state.supply_humidity_ratio_saturation_assignment_count
        && state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count
            == state.supply_humidity_ratio_saturation_assignment_count
        && state.local_saturation_supply_humidity_ratio_assignment_write_count
            == state.supply_humidity_ratio_saturation_assignment_count
        && route_partition_exact
        && assignment_partition_exact
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

fn checked_sum_where(values: &[usize; 30], selected: impl Fn(usize) -> bool) -> Option<usize> {
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
