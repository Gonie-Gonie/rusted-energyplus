//! Persistent CP386 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
            .system
            == system
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
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
        == unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
            .transition_count
        && state_is_consistent(state, Some(snapshot), snapshot.system)
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch;
    state.transition_count
        == unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
            .transition_count
        && state_is_consistent(state, witness, snapshot.system)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_switch_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(predecessor_total) = checked_sum(&state.predecessor_route_counts) else {
        return false;
    };
    let Some(route_total) = state
        .inactive_transition_count
        .checked_add(state.dehumidification_control_switch_count)
    else {
        return false;
    };
    let case_counts = [
        state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        state.dehumidification_control_humidistat_case_selection_count,
        state.dehumidification_control_none_case_selection_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
    ];
    let Some(case_total) = checked_sum(&case_counts) else {
        return false;
    };
    let Some(expected_source_sites) = state
        .dehumidification_control_switch_count
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER.len(),
        )
    else {
        return false;
    };
    if state.system != expected_system
        || predecessor_total != state.transition_count
        || route_total != state.transition_count
        || case_total != state.dehumidification_control_switch_count
        || state.source_site_execution_count != expected_source_sites
        || state.dehumidification_control_type_read_count
            != state.dehumidification_control_switch_count
        || state.dehumidification_control_switch_dispatch_count
            != state.dehumidification_control_switch_count
    {
        return false;
    }
    match state.transition_count {
        0 => {
            state.latest.is_none()
                && state.latest_route.is_none()
                && state.latest_transition_ordinal.is_none()
                && witness.is_none()
        }
        count => {
            let (Some(latest), Some(route), Some(ordinal), Some(witness)) = (
                state.latest,
                state.latest_route,
                state.latest_transition_ordinal,
                witness,
            ) else {
                return false;
            };
            ordinal == count
                && snapshot_route(latest) == Some(route)
                && snapshots_match_bit_exact(latest, witness)
        }
    }
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
