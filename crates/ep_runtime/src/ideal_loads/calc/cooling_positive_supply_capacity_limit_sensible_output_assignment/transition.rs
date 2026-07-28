//! Pure CP338-to-CP339 Cooling capacity-limit sensible-output assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentActiveInput
{
    pub supply_mass_flow_rate_kg_per_s: f64,
    pub mixed_air_enthalpy_j_per_kg: f64,
    pub supply_enthalpy_j_per_kg: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_capacity_limit_sensible_output_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentActiveInput,
    >,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
    let assignment_executed = predecessor.capacity_limit_cp_air_assignment_executed;
    debug_assert_eq!(assignment_executed, active_input.is_some());

    let calculated = active_input.map(|input| {
        let enthalpy_difference =
            input.mixed_air_enthalpy_j_per_kg - input.supply_enthalpy_j_per_kg;
        let cooling_sensible_output =
            input.supply_mass_flow_rate_kg_per_s * enthalpy_difference;
        (input, enthalpy_difference, cooling_sensible_output)
    });
    let supply_mass_flow_rate_kg_per_s =
        calculated.map(|(input, _, _)| input.supply_mass_flow_rate_kg_per_s);
    let mixed_air_enthalpy_j_per_kg =
        calculated.map(|(input, _, _)| input.mixed_air_enthalpy_j_per_kg);
    let supply_enthalpy_j_per_kg =
        calculated.map(|(input, _, _)| input.supply_enthalpy_j_per_kg);
    let mixed_air_minus_supply_enthalpy_j_per_kg =
        calculated.map(|(_, enthalpy_difference, _)| enthalpy_difference);
    let calculated_cooling_sensible_output_w =
        calculated.map(|(_, _, cooling_sensible_output)| cooling_sensible_output);
    let cooling_sensible_output_w = calculated_cooling_sensible_output_w;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute::
            PositiveGuardFalseFallthrough
    } else if predecessor.capacity_limit_guard_false_fallthrough_skipped {
        state.capacity_limit_guard_false_fallthrough_skip_count += 1;
        state.witnessed_capacity_limit_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute::
            ActiveCapacityLimitGuardFalseFallthrough
    } else {
        state.capacity_limit_sensible_output_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
                .len();
        state.supply_mass_flow_rate_read_count += 1;
        state.mixed_air_enthalpy_read_count += 1;
        state.supply_enthalpy_read_count += 1;
        state.enthalpy_difference_calculation_count += 1;
        state.cooling_sensible_output_calculation_count += 1;
        state.cooling_sensible_output_assignment_write_count += 1;
        state.witnessed_capacity_limit_sensible_output_assignment_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute::
            CapacityLimitSensibleOutputAssigned
    };

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
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
                .capacity_limit_cp_air_assignment_executed,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_assignment_executed: assignment_executed,
            supply_mass_flow_rate_read: assignment_executed,
            supply_mass_flow_rate_kg_per_s,
            mixed_air_enthalpy_read: assignment_executed,
            mixed_air_enthalpy_j_per_kg,
            supply_enthalpy_read: assignment_executed,
            supply_enthalpy_j_per_kg,
            enthalpy_difference_calculated: assignment_executed,
            mixed_air_minus_supply_enthalpy_j_per_kg,
            cooling_sensible_output_calculated: assignment_executed,
            calculated_cooling_sensible_output_w,
            cooling_sensible_output_assigned: assignment_executed,
            cooling_sensible_output_w,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
