//! Completed CP318 and pending CP319 retained-state invariants.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_dehumidification_flow_snapshot_is_exact_direct_release,
    cooling_dehumidification_flow_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
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
        && unit.calc_cooling_dehumidification_flow.system == selected
}

pub(super) fn call_order_is_pending_dehumidification_flow(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSensibleFlowSnapshot,
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
        && unit.calc_cooling_economizer_body.transition_count
            == unit.calc_cooling_sensible_flow.transition_count
        && unit
            .calc_cooling_dehumidification_flow
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_sensible_flow.transition_count)
        && predecessor.parent_call_ordinal == unit.calc_cooling_sensible_flow.transition_count
}

pub(super) fn pending_dehumidification_flow_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    latest_witness: Option<PurchasedAirCalcCoolingDehumidificationFlowSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_dehumidification_flow;
    transition_partition_is_consistent(state)
        && latest_is_valid(state, unit.controlled_zone, latest_witness)
        && histories_link_to_predecessor(unit, predecessor)
        && source_counter_relationships_are_consistent(state)
}

pub(super) fn transition_partition_is_consistent(
    state: &PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
) -> bool {
    state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.cooling_body_entry_count))
        == Some(state.transition_count)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
    controlled_zone: Option<ZoneId>,
    latest_witness: Option<PurchasedAirCalcCoolingDehumidificationFlowSnapshot>,
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
                && cooling_dehumidification_flow_snapshot_is_exact_direct_release(latest)
                && cooling_dehumidification_flow_snapshot_route(latest) == Some(retained_route)
        }
        _ => false,
    }
}

fn histories_link_to_predecessor(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSensibleFlowSnapshot,
) -> bool {
    let state = &unit.calc_cooling_dehumidification_flow;
    let sensible = &unit.calc_cooling_sensible_flow;
    let Some(completed_cooling_count) = sensible
        .transition_count
        .checked_sub(sensible.unit_off_skip_count)
        .and_then(|count| count.checked_sub(sensible.non_cooling_skip_count))
    else {
        return false;
    };
    state.transition_count.checked_add(1) == Some(sensible.transition_count)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(sensible.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(sensible.non_cooling_skip_count)
        && state
            .cooling_body_entry_count
            .checked_add(usize::from(predecessor.cooling_body_entered))
            == Some(completed_cooling_count)
}

pub(super) fn source_counter_relationships_are_consistent(
    state: &PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
) -> bool {
    let cooling_on_partition = state
        .cooling_on_body_entry_count
        .checked_add(state.cooling_on_fallthrough_count);
    let control_partition = state
        .dehumidification_control_type_humidistat_count
        .checked_add(state.dehumidification_control_type_fallthrough_count);
    state.supply_mass_flow_rate_for_dehumidification_reset_assignment_count
        == state.cooling_body_entry_count
        && state.cooling_on_read_count == state.cooling_body_entry_count
        && cooling_on_partition == Some(state.cooling_on_read_count)
        && state.cooling_on_body_entry_count == state.cooling_body_entry_count
        && state.cooling_on_fallthrough_count == 0
        && state.dehumidification_control_type_read_count == state.cooling_on_body_entry_count
        && control_partition == Some(state.dehumidification_control_type_read_count)
        && state.dehumidification_control_type_humidistat_count == 0
        && state.dehumidification_control_type_fallthrough_count
            == state.dehumidification_control_type_read_count
        && state.dehumidification_control_body_entry_count == 0
        && downstream_source_counters_are_zero(state)
}

fn downstream_source_counters_are_zero(
    state: &PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
) -> bool {
    state.zone_dehumidifying_setpoint_moisture_demand_read_count == 0
        && state.zone_dehumidifying_setpoint_moisture_demand_assignment_count == 0
        && state.minimum_cooling_supply_air_humidity_ratio_read_count == 0
        && state.zone_humidity_ratio_read_count == 0
        && state.delta_humidity_ratio_calculation_count == 0
        && state.delta_humidity_ratio_assignment_count == 0
        && state.delta_humidity_ratio_for_gate_read_count == 0
        && state.delta_humidity_ratio_comparison_count == 0
        && state.delta_humidity_ratio_comparison_satisfied_count == 0
        && state.delta_humidity_ratio_fallthrough_count == 0
        && state.zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count == 0
        && state.zone_dehumidifying_setpoint_moisture_demand_comparison_count == 0
        && state.zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count == 0
        && state.zone_dehumidifying_setpoint_moisture_demand_fallthrough_count == 0
        && state.dehumidification_flow_body_entry_count == 0
        && state.zone_dehumidifying_setpoint_moisture_demand_for_division_read_count == 0
        && state.delta_humidity_ratio_for_division_read_count == 0
        && state.supply_mass_flow_rate_for_dehumidification_calculation_count == 0
        && state.supply_mass_flow_rate_for_dehumidification_assignment_count == 0
}
