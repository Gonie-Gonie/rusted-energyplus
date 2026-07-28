//! Pure CP341-to-CP342 Cooling capacity-limit supply-enthalpy assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands
{
    pub mixed_air_enthalpy_j_per_kg: f64,
    pub cooling_sensible_output_w: f64,
    pub supply_mass_flow_rate_kg_per_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedInput
{
    pub preexisting_supply_enthalpy_j_per_kg: f64,
    pub active_operands: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentActiveOperands,
    >,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    retained_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedInput,
    >,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot
{
    let guard_false =
        predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment_executed = predecessor
        .capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    debug_assert_eq!(
        guard_false || assignment_executed,
        retained_input.is_some()
    );
    debug_assert_eq!(
        assignment_executed,
        retained_input
            .and_then(|input| input.active_operands)
            .is_some()
    );

    let preexisting_supply_enthalpy_j_per_kg =
        retained_input.map(|input| input.preexisting_supply_enthalpy_j_per_kg);
    let calculated = retained_input
        .and_then(|input| input.active_operands)
        .map(|operands| {
            let specific_cooling_output = operands.cooling_sensible_output_w
                / operands.supply_mass_flow_rate_kg_per_s;
            let supply_enthalpy =
                operands.mixed_air_enthalpy_j_per_kg - specific_cooling_output;
            (operands, specific_cooling_output, supply_enthalpy)
        });
    let mixed_air_enthalpy_j_per_kg =
        calculated.map(|(operands, _, _)| operands.mixed_air_enthalpy_j_per_kg);
    let cooling_sensible_output_w =
        calculated.map(|(operands, _, _)| operands.cooling_sensible_output_w);
    let supply_mass_flow_rate_kg_per_s =
        calculated.map(|(operands, _, _)| operands.supply_mass_flow_rate_kg_per_s);
    let specific_cooling_output_j_per_kg =
        calculated.map(|(_, value, _)| value);
    let calculated_supply_enthalpy_j_per_kg =
        calculated.map(|(_, _, value)| value);
    let assigned_supply_enthalpy_j_per_kg =
        calculated_supply_enthalpy_j_per_kg;
    let resulting_supply_enthalpy_j_per_kg = if assignment_executed {
        assigned_supply_enthalpy_j_per_kg
    } else {
        preexisting_supply_enthalpy_j_per_kg
    };

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute::PositiveGuardFalseFallthrough
    } else if predecessor.capacity_limit_guard_false_fallthrough_skipped {
        state.capacity_limit_guard_false_fallthrough_skip_count += 1;
        state.witnessed_capacity_limit_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute::ActiveCapacityLimitGuardFalseFallthrough
    } else if guard_false {
        state.capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
        state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute::CapacityLimitSensibleOutputGuardFalseFallthrough
    } else {
        debug_assert!(assignment_executed);
        state.capacity_limit_sensible_output_supply_enthalpy_assignment_count += 1;
        state.source_site_execution_count += 6;
        state.mixed_air_enthalpy_read_count += 1;
        state.cooling_sensible_output_read_count += 1;
        state.supply_mass_flow_rate_read_count += 1;
        state.specific_cooling_output_calculation_count += 1;
        state.supply_enthalpy_calculation_count += 1;
        state.supply_enthalpy_assignment_write_count += 1;
        state.witnessed_capacity_limit_sensible_output_supply_enthalpy_assignment_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute::CapacityLimitSensibleOutputSupplyEnthalpyAssigned
    };

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
            system: state.system,
            parent_call_ordinal: predecessor.parent_call_ordinal,
            controlled_zone: predecessor.controlled_zone,
            unit_body_entered: predecessor.unit_body_entered,
            predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
            predecessor_no_outdoor_air_fallback_entered: predecessor
                .predecessor_no_outdoor_air_fallback_entered,
            predecessor_positive_supply_mass_flow_body_entered: predecessor
                .predecessor_positive_supply_mass_flow_body_entered,
            predecessor_active_guard_false_fallthrough: predecessor
                .predecessor_active_guard_false_fallthrough,
            predecessor_capacity_limit_guard_evaluated: predecessor
                .predecessor_capacity_limit_guard_evaluated,
            predecessor_capacity_limit_body_entered: predecessor
                .predecessor_capacity_limit_body_entered,
            predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
                .predecessor_active_capacity_limit_guard_false_fallthrough,
            predecessor_capacity_limit_cp_air_assignment_executed: predecessor
                .predecessor_capacity_limit_cp_air_assignment_executed,
            predecessor_capacity_limit_sensible_output_assignment_executed: predecessor
                .predecessor_capacity_limit_sensible_output_assignment_executed,
            predecessor_capacity_limit_sensible_output_guard_evaluated: predecessor
                .predecessor_capacity_limit_sensible_output_guard_evaluated,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: predecessor
                .predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
            predecessor_capacity_limit_sensible_output_adjustment_body_entered: predecessor
                .predecessor_capacity_limit_sensible_output_adjustment_body_entered,
            predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed:
                assignment_executed,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_supply_enthalpy_assignment_executed:
                assignment_executed,
            preexisting_supply_enthalpy_j_per_kg,
            mixed_air_enthalpy_read: assignment_executed,
            mixed_air_enthalpy_j_per_kg,
            cooling_sensible_output_read: assignment_executed,
            cooling_sensible_output_w,
            supply_mass_flow_rate_read: assignment_executed,
            supply_mass_flow_rate_kg_per_s,
            specific_cooling_output_calculated: assignment_executed,
            specific_cooling_output_j_per_kg,
            supply_enthalpy_calculated: assignment_executed,
            calculated_supply_enthalpy_j_per_kg,
            supply_enthalpy_assigned: assignment_executed,
            assigned_supply_enthalpy_j_per_kg,
            resulting_supply_enthalpy_j_per_kg,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
