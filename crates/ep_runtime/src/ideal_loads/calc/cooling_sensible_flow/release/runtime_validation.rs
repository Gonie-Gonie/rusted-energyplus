//! Completed CP317 and pending CP318 retained-state invariants.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_sensible_flow_snapshot_is_exact_direct_release, cooling_sensible_flow_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodySnapshot, PurchasedAirCalcCoolingSensibleFlowRuntimeState,
    PurchasedAirCalcCoolingSensibleFlowSnapshot, PurchasedAirUnitRuntimeState,
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
}

pub(super) fn call_order_is_pending_sensible_flow(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerBodySnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_entry.call_count == unit.calc_minimum_oa_prefix.transition_count
        && unit.calc_minimum_oa_prefix.transition_count
            == unit.calc_cooling_entry_gate.transition_count
        && unit.calc_cooling_entry_gate.transition_count
            == unit.calc_cooling_oa_max_flow_gate.transition_count
        && unit.calc_cooling_oa_max_flow_gate.transition_count
            == unit.calc_cooling_oa_max_flow_body.transition_count
        && unit.calc_cooling_oa_max_flow_body.transition_count
            == unit.calc_cooling_economizer_guard.transition_count
        && unit.calc_cooling_economizer_guard.transition_count
            == unit.calc_cooling_economizer_condition.transition_count
        && unit.calc_cooling_economizer_condition.transition_count
            == unit.calc_cooling_economizer_body.transition_count
        && unit
            .calc_cooling_sensible_flow
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_economizer_body.transition_count)
        && predecessor.parent_call_ordinal == unit.calc_cooling_economizer_body.transition_count
}

pub(super) fn pending_sensible_flow_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    latest_witness: Option<PurchasedAirCalcCoolingSensibleFlowSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_sensible_flow;
    transition_partition_is_consistent(state)
        && latest_is_valid(state, unit.controlled_zone, latest_witness)
        && histories_link_to_predecessor(unit, predecessor)
        && source_counter_relationships_are_consistent(state)
}

fn transition_partition_is_consistent(
    state: &PurchasedAirCalcCoolingSensibleFlowRuntimeState,
) -> bool {
    state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.cooling_body_entry_count))
        == Some(state.transition_count)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingSensibleFlowRuntimeState,
    controlled_zone: Option<ZoneId>,
    latest_witness: Option<PurchasedAirCalcCoolingSensibleFlowSnapshot>,
) -> bool {
    match (
        state.transition_count,
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        latest_witness,
    ) {
        (0, None, None, None, None) => true,
        (
            count,
            Some(latest),
            Some(retained_route),
            Some(latest_transition_ordinal),
            Some(consumer_witness),
        ) if count > 0 => {
            latest_transition_ordinal == count
                && consumer_witness == latest
                && latest.parent_call_ordinal == count
                && latest.system == state.system
                && controlled_zone == Some(latest.controlled_zone)
                && cooling_sensible_flow_snapshot_is_exact_direct_release(latest)
                && cooling_sensible_flow_snapshot_route(latest) == Some(retained_route)
        }
        _ => false,
    }
}

fn histories_link_to_predecessor(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerBodySnapshot,
) -> bool {
    let state = &unit.calc_cooling_sensible_flow;
    let body = &unit.calc_cooling_economizer_body;
    let Some(completed_cooling_count) = body
        .transition_count
        .checked_sub(body.unit_off_skip_count)
        .and_then(|count| count.checked_sub(body.non_cooling_skip_count))
    else {
        return false;
    };
    state.transition_count.checked_add(1) == Some(body.transition_count)
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
            .checked_add(usize::from(predecessor.predecessor_cooling_body_entered))
            == Some(completed_cooling_count)
}

fn source_counter_relationships_are_consistent(
    state: &PurchasedAirCalcCoolingSensibleFlowRuntimeState,
) -> bool {
    let cooling_on_partition = state
        .cooling_on_body_entry_count
        .checked_add(state.cooling_on_fallthrough_count);
    let delta_partition = state
        .delta_temperature_comparison_satisfied_count
        .checked_add(state.delta_temperature_fallthrough_count);
    state.supply_mass_flow_rate_for_cool_reset_assignment_count == state.cooling_body_entry_count
        && state.cooling_on_read_count == state.cooling_body_entry_count
        && cooling_on_partition == Some(state.cooling_on_read_count)
        && state.cooling_on_body_entry_count == state.cooling_body_entry_count
        && state.cooling_on_fallthrough_count == 0
        && state.zone_humidity_ratio_read_count == state.cooling_on_body_entry_count
        && state.psychrometric_cp_air_evaluation_count == state.cooling_on_body_entry_count
        && state.cp_air_assignment_count == state.cooling_on_body_entry_count
        && state.minimum_cooling_supply_air_temperature_read_count
            == state.cooling_on_body_entry_count
        && state.zone_temperature_read_count == state.cooling_on_body_entry_count
        && state.delta_temperature_calculation_count == state.cooling_on_body_entry_count
        && state.delta_temperature_assignment_count == state.cooling_on_body_entry_count
        && state.delta_temperature_for_gate_read_count == state.cooling_on_body_entry_count
        && state.delta_temperature_comparison_count == state.cooling_on_body_entry_count
        && delta_partition == Some(state.delta_temperature_comparison_count)
        && state.delta_temperature_body_entry_count
            == state.delta_temperature_comparison_satisfied_count
        && state.zone_cooling_setpoint_load_read_count == state.delta_temperature_body_entry_count
        && state.cp_air_for_first_division_read_count == state.delta_temperature_body_entry_count
        && state.zone_cooling_setpoint_load_over_cp_air_calculation_count
            == state.delta_temperature_body_entry_count
        && state.delta_temperature_for_second_division_read_count
            == state.delta_temperature_body_entry_count
        && state.supply_mass_flow_rate_for_cool_calculation_count
            == state.delta_temperature_body_entry_count
        && state.supply_mass_flow_rate_for_cool_assignment_count
            == state.delta_temperature_body_entry_count
}
