use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot, PurchasedAirUnitRuntimeState,
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
        && unit.calc_cooling_supply_mass_flow_very_small_guard.system == selected
        && unit
            .calc_cooling_supply_mass_flow_very_small_guard_body
            .system
            == selected
}

pub(super) fn call_order_is_pending_body(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_very_small_guard
            .transition_count
            == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_very_small_guard_body
            .transition_count
            .checked_add(1)
            == Some(
                unit.calc_cooling_supply_mass_flow_very_small_guard
                    .transition_count,
            )
        && predecessor.parent_call_ordinal
            == unit
                .calc_cooling_supply_mass_flow_very_small_guard
                .transition_count
}

pub(super) fn pending_body_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_very_small_guard_body;
    let guard = &unit.calc_cooling_supply_mass_flow_very_small_guard;
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
            .zero_flow_reset_body_entry_count
            .checked_add(usize::from(predecessor.zero_flow_reset_body_entered))
            == Some(guard.zero_flow_reset_body_entry_count)
        && state
            .active_guard_false_fallthrough_count
            .checked_add(usize::from(predecessor.active_guard_false_fallthrough))
            == Some(guard.active_guard_false_fallthrough_count)
        && source_counters_are_consistent(state)
}

pub(super) fn completed_body_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    body: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_very_small_guard_body;
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
        && cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(body)
        && completed_body_history_links_to_guard(
            state,
            &unit.calc_cooling_supply_mass_flow_very_small_guard,
        )
        && source_counters_are_consistent(state)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    controlled_zone: Option<ZoneId>,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot>,
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
                && cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshot_route(latest) == Some(route)
        }
        _ => false,
    }
}

fn completed_body_history_links_to_guard(
    body: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    guard: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
) -> bool {
    body.transition_count == guard.transition_count
        && body.unit_off_skip_count == guard.unit_off_skip_count
        && body.non_cooling_skip_count == guard.non_cooling_skip_count
        && body.cooling_body_entry_count == guard.cooling_body_entry_count
        && body.zero_flow_reset_body_entry_count == guard.zero_flow_reset_body_entry_count
        && body.active_guard_false_fallthrough_count == guard.active_guard_false_fallthrough_count
}

fn source_counters_are_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
) -> bool {
    state.supply_mass_flow_rate_positive_zero_assignment_count
        == state.zero_flow_reset_body_entry_count
        && state.zero_flow_reset_body_entry_count
            == state.witnessed_zero_flow_reset_body_entry_count
        && state.active_guard_false_fallthrough_count
            == state.witnessed_active_guard_false_fallthrough_count
        && state
            .zero_flow_reset_body_entry_count
            .checked_add(state.active_guard_false_fallthrough_count)
            == Some(state.cooling_body_entry_count)
        && state
            .body_skip_count
            .checked_add(state.zero_flow_reset_body_entry_count)
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
