use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

use super::snapshot_validation::{
    cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release, snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
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
}

pub(super) fn call_order_is_pending_guard(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_ems_override_body
            .transition_count
            == unit.calc_entry.call_count
        && unit
            .calc_cooling_supply_mass_flow_limit_guard
            .transition_count
            .checked_add(1)
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_body
                    .transition_count,
            )
        && predecessor.parent_call_ordinal
            == unit
                .calc_cooling_supply_mass_flow_ems_override_body
                .transition_count
}

pub(in crate::ideal_loads::calc) fn pending_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_limit_guard;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && latest_is_valid(
        state,
        unit.controlled_zone,
        system.cooling_limit,
        maximum_cooling_air_mass_flow_rate_kg_per_s,
        witness,
    ) && state.transition_count.checked_add(1)
        == Some(
            unit.calc_cooling_supply_mass_flow_ems_override_body
                .transition_count,
        )
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_body
                    .unit_off_skip_count,
            )
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_body
                    .non_cooling_skip_count,
            )
        && state
            .cooling_body_entry_count
            .checked_add(usize::from(predecessor.cooling_body_entered))
            == Some(
                unit.calc_cooling_supply_mass_flow_ems_override_body
                    .cooling_body_entry_count,
            )
        && source_counters_are_consistent(
            state,
            system.cooling_limit,
            maximum_cooling_air_mass_flow_rate_kg_per_s,
        )
}

pub(super) fn completed_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_limit_guard;
    partition_is_consistent(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.cooling_body_entry_count,
    ) && state.transition_count == unit.calc_entry.call_count
        && state.latest == Some(predecessor)
        && witness == Some(predecessor)
        && state.latest_transition_ordinal == Some(state.transition_count)
        && snapshot_route(predecessor) == state.latest_route
        && predecessor.parent_call_ordinal == state.transition_count
        && predecessor.system == state.system
        && unit.controlled_zone == Some(predecessor.controlled_zone)
        && cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(predecessor)
        && latest_inputs_match(
            predecessor,
            system.cooling_limit,
            maximum_cooling_air_mass_flow_rate_kg_per_s,
        )
        && completed_guard_history_links_to_body(
            state,
            &unit.calc_cooling_supply_mass_flow_ems_override_body,
        )
        && source_counters_are_consistent(
            state,
            system.cooling_limit,
            maximum_cooling_air_mass_flow_rate_kg_per_s,
        )
}

fn completed_guard_history_links_to_body(
    guard: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    body: &crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
) -> bool {
    guard.transition_count == body.transition_count
        && guard.unit_off_skip_count == body.unit_off_skip_count
        && guard.non_cooling_skip_count == body.non_cooling_skip_count
        && guard.cooling_body_entry_count == body.cooling_body_entry_count
        && guard
            .supply_mass_flow_limit_body_entry_count
            .checked_add(guard.active_guard_false_fallthrough_count)
            == Some(guard.cooling_body_entry_count)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    controlled_zone: Option<ZoneId>,
    cooling_limit: IdealLoadsLimit,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot>,
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
            latest == witness
                && ordinal == count
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && controlled_zone == Some(latest.controlled_zone)
                && cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(latest)
                && snapshot_route(latest) == Some(route)
                && latest_inputs_match(
                    latest,
                    cooling_limit,
                    maximum_cooling_air_mass_flow_rate_kg_per_s,
                )
        }
        _ => false,
    }
}

fn latest_inputs_match(
    latest: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    cooling_limit: IdealLoadsLimit,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> bool {
    if !latest.cooling_body_entered {
        return true;
    }

    latest.first_cooling_limit == Some(cooling_limit)
        && (!latest.second_cooling_limit_read || latest.second_cooling_limit == Some(cooling_limit))
        && (!latest.maximum_cooling_air_mass_flow_rate_read
            || latest
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_some_and(|observed| {
                    observed.to_bits() == maximum_cooling_air_mass_flow_rate_kg_per_s.to_bits()
                }))
}

fn source_counters_are_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    cooling_limit: IdealLoadsLimit,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> bool {
    let cooling = state.cooling_body_entry_count;
    let flow_rate = usize::from(cooling_limit == IdealLoadsLimit::LimitFlowRate) * cooling;
    let second = cooling - flow_rate;
    let flow_rate_and_capacity =
        usize::from(cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity) * cooling;
    let selected = flow_rate + flow_rate_and_capacity;
    let positive = usize::from(maximum_cooling_air_mass_flow_rate_kg_per_s > 0.0) * selected;

    state.first_cooling_limit_read_count == cooling
        && state.cooling_limit_flow_rate_comparison_count == cooling
        && state.cooling_limit_flow_rate_match_count == flow_rate
        && state.second_cooling_limit_read_count == second
        && state.cooling_limit_flow_rate_and_capacity_comparison_count == second
        && state.cooling_limit_flow_rate_and_capacity_match_count == flow_rate_and_capacity
        && state.cooling_limit_rejected_count == cooling - selected
        && state.maximum_cooling_air_mass_flow_rate_read_count == selected
        && state.maximum_cooling_air_mass_flow_rate_positive_comparison_count == selected
        && state.maximum_cooling_air_mass_flow_rate_strictly_positive_count == positive
        && state.maximum_cooling_air_mass_flow_rate_not_positive_count == selected - positive
        && state.supply_mass_flow_limit_body_entry_count == positive
        && state.active_guard_false_fallthrough_count == cooling - positive
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
