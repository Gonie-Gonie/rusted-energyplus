use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release, snapshot_route,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
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
}

pub(super) fn call_order_is_pending_guard(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_limit_body
            .transition_count
            == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_very_small_guard
            .transition_count
            .checked_add(1)
            == Some(
                unit.calc_cooling_supply_mass_flow_limit_body
                    .transition_count,
            )
        && predecessor.parent_call_ordinal
            == unit
                .calc_cooling_supply_mass_flow_limit_body
                .transition_count
}

pub(super) fn pending_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_very_small_guard;
    let body = &unit.calc_cooling_supply_mass_flow_limit_body;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && latest_is_valid(state, unit.controlled_zone, witness)
        && state.transition_count.checked_add(1) == Some(body.transition_count)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(body.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(body.non_cooling_skip_count)
        && state
            .cooling_body_entry_count
            .checked_add(usize::from(predecessor.cooling_body_entered))
            == Some(body.cooling_body_entry_count)
        && source_counters_are_consistent(state)
}

pub(super) fn completed_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    guard: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_very_small_guard;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && state.transition_count == unit.calc_entry.call_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, guard))
        && witness.is_some_and(|witness| snapshots_match_bit_exact(witness, guard))
        && state.latest_transition_ordinal == Some(state.transition_count)
        && snapshot_route(guard) == state.latest_route
        && guard.parent_call_ordinal == state.transition_count
        && guard.system == state.system
        && unit.controlled_zone == Some(guard.controlled_zone)
        && cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(guard)
        && completed_guard_history_links_to_limit_body(
            state,
            &unit.calc_cooling_supply_mass_flow_limit_body,
        )
        && source_counters_are_consistent(state)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    controlled_zone: Option<ZoneId>,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot>,
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
                && cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshot_route(latest) == Some(route)
        }
        _ => false,
    }
}

fn completed_guard_history_links_to_limit_body(
    guard: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    body: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
) -> bool {
    guard.transition_count == body.transition_count
        && guard.unit_off_skip_count == body.unit_off_skip_count
        && guard.non_cooling_skip_count == body.non_cooling_skip_count
        && guard.cooling_body_entry_count == body.cooling_body_entry_count
}

fn source_counters_are_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
) -> bool {
    let cooling = state.cooling_body_entry_count;
    state.supply_mass_flow_rate_read_count == cooling
        && state.hvac_very_small_mass_flow_read_count == cooling
        && state.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count == cooling
        && state.zero_flow_reset_body_entry_count
            == state.witnessed_zero_flow_reset_body_entry_count
        && state.active_guard_false_fallthrough_count
            == state.witnessed_active_guard_false_fallthrough_count
        && state
            .zero_flow_reset_body_entry_count
            .checked_add(state.active_guard_false_fallthrough_count)
            == Some(cooling)
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
