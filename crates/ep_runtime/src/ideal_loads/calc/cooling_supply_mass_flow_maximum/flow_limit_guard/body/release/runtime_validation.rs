use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release, snapshot_route,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot, PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    selected: IdealLoadsAirSystemId,
) -> bool {
    unit.system == selected
        && unit.calc_entry.system == selected
        && unit.calc_minimum_oa_prefix.system == selected
        && unit.calc_cooling_entry_gate.system == selected
        && unit.calc_cooling_oa_max_flow_gate.system == selected
        && unit.calc_cooling_oa_max_flow_body.system == selected
        && unit.calc_cooling_economizer_guard.system == selected
        && unit.calc_cooling_economizer_condition.system == selected
        && unit.calc_cooling_economizer_body.system == selected
        && unit.calc_cooling_sensible_flow.system == selected
        && unit.calc_cooling_dehumidification_flow.system == selected
        && unit.calc_cooling_humidification_flow.system == selected
        && unit.calc_cooling_capacity_zero_flow_reset.system == selected
        && unit.calc_cooling_supply_mass_flow_maximum.system == selected
        && unit.calc_cooling_supply_mass_flow_ems_override_guard.system == selected
        && unit.calc_cooling_supply_mass_flow_ems_override_body.system == selected
        && unit.calc_cooling_supply_mass_flow_limit_guard.system == selected
        && unit.calc_cooling_supply_mass_flow_limit_body.system == selected
}

pub(super) fn call_order_is_pending_body(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_limit_guard
            .transition_count
            == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_limit_body
            .transition_count
            .checked_add(1)
            == Some(
                unit.calc_cooling_supply_mass_flow_limit_guard
                    .transition_count,
            )
        && predecessor.parent_call_ordinal
            == unit
                .calc_cooling_supply_mass_flow_limit_guard
                .transition_count
}

pub(super) fn pending_body_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_limit_body;
    let guard = &unit.calc_cooling_supply_mass_flow_limit_guard;
    let Some(expected_guard_body_skips) = guard
        .transition_count
        .checked_sub(guard.supply_mass_flow_limit_body_entry_count)
    else {
        return false;
    };
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && latest_is_valid(state, unit.controlled_zone, witness)
        && state.transition_count.checked_add(1) == Some(guard.transition_count)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(guard.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(guard.non_cooling_skip_count)
        && state
            .cooling_body_entry_count
            .checked_add(usize::from(predecessor.cooling_body_entered))
            == Some(guard.cooling_body_entry_count)
        && state
            .supply_mass_flow_limit_body_entry_count
            .checked_add(usize::from(predecessor.supply_mass_flow_limit_body_entered))
            == Some(guard.supply_mass_flow_limit_body_entry_count)
        && state
            .active_guard_false_fallthrough_count
            .checked_add(usize::from(predecessor.active_guard_false_fallthrough))
            == Some(guard.active_guard_false_fallthrough_count)
        && state.body_skip_count.checked_add(usize::from(
            !predecessor.supply_mass_flow_limit_body_entered,
        )) == Some(expected_guard_body_skips)
        && source_counters_are_consistent(state)
}

pub(super) fn completed_body_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    body: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_limit_body;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && state.transition_count == unit.calc_entry.call_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, body))
        && witness.is_some_and(|witness| snapshots_match_bit_exact(witness, body))
        && state.latest_transition_ordinal == Some(state.transition_count)
        && snapshot_route(body) == state.latest_route
        && body.parent_call_ordinal == state.transition_count
        && body.system == state.system
        && unit.controlled_zone == Some(body.controlled_zone)
        && cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release(body)
        && completed_body_history_links_to_guard(
            state,
            &unit.calc_cooling_supply_mass_flow_limit_guard,
        )
        && source_counters_are_consistent(state)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    controlled_zone: Option<ZoneId>,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot>,
) -> bool {
    match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) {
        (0, None, None, None, None) => true,
        (count, Some(latest), Some(route), Some(ordinal), Some(witness)) if count > 0 => {
            snapshots_match_bit_exact(latest, witness)
                && ordinal == count
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && controlled_zone == Some(latest.controlled_zone)
                && cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release(latest)
                && snapshot_route(latest) == Some(route)
        }
        _ => false,
    }
}

fn completed_body_history_links_to_guard(
    body: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    guard: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
) -> bool {
    let Some(expected_body_skips) = guard
        .transition_count
        .checked_sub(guard.supply_mass_flow_limit_body_entry_count)
    else {
        return false;
    };
    body.transition_count == guard.transition_count
        && body.unit_off_skip_count == guard.unit_off_skip_count
        && body.non_cooling_skip_count == guard.non_cooling_skip_count
        && body.cooling_body_entry_count == guard.cooling_body_entry_count
        && body.supply_mass_flow_limit_body_entry_count
            == guard.supply_mass_flow_limit_body_entry_count
        && body.active_guard_false_fallthrough_count == guard.active_guard_false_fallthrough_count
        && body.body_skip_count == expected_body_skips
}

fn source_counters_are_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
) -> bool {
    let entered = state.supply_mass_flow_limit_body_entry_count;
    state.supply_mass_flow_rate_for_minimum_read_count == entered
        && state.maximum_cooling_air_mass_flow_rate_for_minimum_read_count == entered
        && state.source_shaped_two_argument_minimum_evaluation_count == entered
        && state.supply_mass_flow_rate_assignment_count == entered
        && state
            .supply_mass_flow_limit_body_entry_count
            .checked_add(state.active_guard_false_fallthrough_count)
            == Some(state.cooling_body_entry_count)
        && state
            .body_skip_count
            .checked_add(state.supply_mass_flow_limit_body_entry_count)
            == Some(state.transition_count)
}

fn partition_is_consistent(
    transitions: usize,
    unit_off: usize,
    non_cooling: usize,
    cooling: usize,
) -> bool {
    unit_off
        .checked_add(non_cooling)
        .and_then(|count| count.checked_add(cooling))
        == Some(transitions)
}
