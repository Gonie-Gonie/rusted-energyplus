//! Pure CP385-to-CP386 symbolic dehumidification-control dispatch.

use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Predecessor;

mod accounting;
pub(in crate::ideal_loads::calc) mod routes;

use accounting::{increment_counts, next_transition_fits};
use routes::{RetainedRoute, predecessor_route, selector_is_allowed};

/// Typed owner input read only on a CP385 assignment route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput
{
    pub dehumidification_control_type: DehumidificationControlType,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
    state: &mut State,
    predecessor: Predecessor,
    active_input: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let active = predecessor_route.is_assignment();
    if active != active_input.is_some() {
        return None;
    }
    let selected_case = active_input.map(|input| input.dehumidification_control_type);
    if !selector_is_allowed(predecessor_route, selected_case) {
        return None;
    }
    let route = RetainedRoute {
        predecessor: predecessor_route,
        selected_case,
    };
    if !next_transition_fits(state, route) {
        return None;
    }

    state.transition_count += 1;
    increment_counts(state, route);

    // CP386 is symbolic control flow only. The CP385 enthalpy is carried as
    // bit-exact evidence and is neither read as a dispatch operand nor changed.
    let resulting_supply_enthalpy_j_per_kg = predecessor.resulting_supply_enthalpy_j_per_kg;
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
        first_excluded_lexical_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: predecessor.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: predecessor.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: predecessor.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor.supply_enthalpy_assignment_executed,
        predecessor_resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_type_read: active,
        dehumidification_control_type: selected_case,
        dehumidification_control_switch_dispatched: active,
        resulting_supply_enthalpy_j_per_kg,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
