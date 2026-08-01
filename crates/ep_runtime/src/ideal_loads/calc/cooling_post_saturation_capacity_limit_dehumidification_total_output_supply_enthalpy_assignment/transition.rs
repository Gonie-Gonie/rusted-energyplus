//! Pure CP384-to-CP385 raw binary64 supply-enthalpy assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Predecessor;

mod accounting;
mod routes;

use accounting::{increment_counts, next_transition_fits};
pub(in crate::ideal_loads::calc) use routes::{
    PredecessorRoute, predecessor_route, predecessor_route_is_assignment,
};
use routes::{predecessor_route_is_guard_evaluated, retained_route};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentActiveOperands {
    pub mixed_air_enthalpy_j_per_kg: f64,
    pub cooling_total_output_w: f64,
    pub supply_mass_flow_rate_kg_per_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput {
    pub preexisting_supply_enthalpy_j_per_kg: f64,
    pub active_operands: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentActiveOperands>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    retained_input: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let assignment = predecessor_route_is_assignment(predecessor_route);
    let guard_evaluated = predecessor_route_is_guard_evaluated(predecessor_route);
    if guard_evaluated != retained_input.is_some()
        || assignment != retained_input.and_then(|input| input.active_operands).is_some()
    {
        return None;
    }

    let preexisting_supply_enthalpy_j_per_kg =
        retained_input.map(|input| input.preexisting_supply_enthalpy_j_per_kg);
    let calculated = retained_input
        .and_then(|input| input.active_operands)
        .map(|operands| {
            // Preserve the source grouping exactly: division first, subtraction second.
            let specific_cooling_output =
                operands.cooling_total_output_w / operands.supply_mass_flow_rate_kg_per_s;
            let supply_enthalpy =
                operands.mixed_air_enthalpy_j_per_kg - specific_cooling_output;
            (operands, specific_cooling_output, supply_enthalpy)
        });
    let mixed_air_enthalpy_j_per_kg =
        calculated.map(|(operands, _, _)| operands.mixed_air_enthalpy_j_per_kg);
    let cooling_total_output_w =
        calculated.map(|(operands, _, _)| operands.cooling_total_output_w);
    let supply_mass_flow_rate_kg_per_s =
        calculated.map(|(operands, _, _)| operands.supply_mass_flow_rate_kg_per_s);
    let specific_cooling_output_j_per_kg = calculated.map(|(_, value, _)| value);
    let calculated_supply_enthalpy_j_per_kg = calculated.map(|(_, _, value)| value);
    let assigned_supply_enthalpy_j_per_kg = calculated_supply_enthalpy_j_per_kg;
    let resulting_supply_enthalpy_j_per_kg = if assignment {
        assigned_supply_enthalpy_j_per_kg
    } else {
        preexisting_supply_enthalpy_j_per_kg
    };

    let route = retained_route(predecessor_route);
    if !next_transition_fits(state, predecessor_route, route, assignment) {
        return None;
    }
    state.transition_count += 1;
    increment_counts(state, predecessor_route, route, guard_evaluated, assignment);

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
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
        supply_enthalpy_assignment_executed: assignment,
        preexisting_supply_enthalpy_j_per_kg,
        cp379_retained_supply_enthalpy_owned_read: guard_evaluated,
        cp329_retained_mixed_air_enthalpy_owned_read: assignment,
        mixed_air_enthalpy_read: assignment,
        mixed_air_enthalpy_j_per_kg,
        cp384_retained_cooling_total_output_owned_read: assignment,
        cooling_total_output_read: assignment,
        cooling_total_output_w,
        cp330_retained_supply_mass_flow_rate_owned_read: assignment,
        supply_mass_flow_rate_read: assignment,
        supply_mass_flow_rate_kg_per_s,
        specific_cooling_output_calculated: assignment,
        specific_cooling_output_j_per_kg,
        supply_enthalpy_difference_calculated: assignment,
        calculated_supply_enthalpy_j_per_kg,
        supply_enthalpy_assigned: assignment,
        assigned_supply_enthalpy_j_per_kg,
        resulting_supply_enthalpy_j_per_kg,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
