use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::snapshot_validation::{
    cooling_humidification_flow_snapshot_is_exact_direct_release,
    cooling_humidification_flow_snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    selected: IdealLoadsAirSystemId,
) -> bool {
    unit.system == selected
        && unit.calc_entry.system == selected
        && unit.calc_cooling_dehumidification_flow.system == selected
        && unit.calc_cooling_humidification_flow.system == selected
}

pub(super) fn call_order_is_pending_humidification_flow(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) -> bool {
    unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_cooling_dehumidification_flow.transition_count == unit.calc_entry.call_count
        && unit
            .calc_cooling_humidification_flow
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_dehumidification_flow.transition_count)
        && predecessor.parent_call_ordinal
            == unit.calc_cooling_dehumidification_flow.transition_count
}

pub(super) fn pending_humidification_flow_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    witness: Option<PurchasedAirCalcCoolingHumidificationFlowSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_humidification_flow;
    partition_is_consistent(state)
        && latest_is_valid(state, unit.controlled_zone, witness)
        && state.transition_count.checked_add(1)
            == Some(unit.calc_cooling_dehumidification_flow.transition_count)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(unit.calc_cooling_dehumidification_flow.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(
                unit.calc_cooling_dehumidification_flow
                    .non_cooling_skip_count,
            )
        && direct_counter_relationships_are_consistent(state)
}

fn partition_is_consistent(state: &PurchasedAirCalcCoolingHumidificationFlowRuntimeState) -> bool {
    state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.cooling_body_entry_count))
        == Some(state.transition_count)
}

fn latest_is_valid(
    state: &PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    controlled_zone: Option<ZoneId>,
    witness: Option<PurchasedAirCalcCoolingHumidificationFlowSnapshot>,
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
                && cooling_humidification_flow_snapshot_is_exact_direct_release(latest)
                && cooling_humidification_flow_snapshot_route(latest) == Some(route)
        }
        _ => false,
    }
}

fn direct_counter_relationships_are_consistent(
    state: &PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
) -> bool {
    state.reset_assignment_count == state.cooling_body_entry_count
        && state.heating_on_read_count == state.cooling_body_entry_count
        && state.heating_on_body_entry_count == state.cooling_body_entry_count
        && state.heating_on_fallthrough_count == 0
        && state.humidification_control_type_read_count == state.cooling_body_entry_count
        && state.humidification_control_type_humidistat_count == 0
        && state.humidification_control_type_fallthrough_count == state.cooling_body_entry_count
        && state.humidification_control_body_entry_count == 0
        && downstream_counts_are_zero(state)
}

fn downstream_counts_are_zero(
    state: &PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
) -> bool {
    state.dehumidification_control_type_first_read_count == 0
        && state.dehumidification_control_type_humidistat_count == 0
        && state.dehumidification_control_type_second_read_count == 0
        && state.dehumidification_control_type_none_count == 0
        && state.dehumidification_control_type_rejected_count == 0
        && state.admitted_control_body_entry_count == 0
        && state.moisture_demand_read_count == 0
        && state.moisture_demand_assignment_count == 0
        && state.maximum_heating_supply_humidity_ratio_read_count == 0
        && state.zone_humidity_ratio_read_count == 0
        && state.delta_calculation_count == 0
        && state.delta_assignment_count == 0
        && state.delta_gate_read_count == 0
        && state.delta_comparison_count == 0
        && state.delta_comparison_satisfied_count == 0
        && state.delta_fallthrough_count == 0
        && state.moisture_demand_gate_read_count == 0
        && state.moisture_demand_comparison_count == 0
        && state.moisture_demand_comparison_satisfied_count == 0
        && state.moisture_demand_fallthrough_count == 0
        && state.humidification_flow_body_entry_count == 0
        && state.moisture_demand_division_read_count == 0
        && state.delta_division_read_count == 0
        && state.calculation_count == 0
        && state.assignment_count == 0
}
