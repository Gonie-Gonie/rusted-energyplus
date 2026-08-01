//! Persistent CP388 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment.system == system
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment.system == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment.transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
        &mut next,
        predecessor,
        active_input,
    )?;
    Some((next, snapshot))
}

pub(super) fn prepared_completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
) -> bool {
    state.transition_count
        == unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment.transition_count
        && state_is_consistent(state, Some(snapshot), snapshot.system)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_predecessor_counts_match_exact_direct_cp_air_assignment(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
        )
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
    state.transition_count
        == unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment.transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_predecessor_counts_match_exact_direct_cp_air_assignment(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
        )
        && state.latest.is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_predecessor_counts_match_exact_direct_cp_air_assignment(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
        )
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_predecessor_counts_match_exact_direct_cp_air_assignment(
    state: &State,
    predecessor: &PredecessorState,
) -> bool {
    state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.transition_count == predecessor.transition_count
        && state.inactive_transition_count == state.transition_count
        && state.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count == 0
        && state.source_site_execution_count == 0
        && state.cooling_total_output_owned_read_count == 0
        && state.cooling_total_output_bit_corroboration_count == 0
        && state.cooling_sensible_heat_ratio_read_count == 0
        && state.cooling_sensible_output_calculation_count == 0
        && state.cooling_sensible_output_assignment_write_count == 0
        && predecessor.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count == 0
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(predecessor_total) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let route_total = state.inactive_transition_count.checked_add(
        state.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
    );
    let active_total = [18, 22, 28]
        .into_iter()
        .try_fold(0usize, |sum, index| sum.checked_add(state.predecessor_route_counts[index]));
    let assignments = state
        .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count;
    let expected_sites = assignments.checked_mul(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
    );
    let site_counts = [
        state.cooling_total_output_owned_read_count,
        state.cooling_total_output_bit_corroboration_count,
        state.cooling_sensible_heat_ratio_read_count,
        state.cooling_sensible_output_calculation_count,
        state.cooling_sensible_output_assignment_write_count,
    ];
    if state.system != expected_system
        || predecessor_total != state.transition_count
        || route_total != Some(state.transition_count)
        || active_total != Some(assignments)
        || expected_sites != Some(state.source_site_execution_count)
        || site_counts.into_iter().any(|count| count != assignments)
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
                && route.active
                    == super::super::transition::routes::predecessor_index_is_active(
                        route.predecessor_index,
                    )
                && snapshot_route(latest) == Some(route)
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values.iter().try_fold(0usize, |sum, value| sum.checked_add(*value))
}
