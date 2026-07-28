//! Pure CP342-to-CP343 Cooling capacity-limit supply-temperature assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot;
use crate::psychrometrics::energyplus_psy_tdb_fn_h_w;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands
{
    pub supply_enthalpy_j_per_kg: f64,
    pub supply_humidity_ratio: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedInput
{
    pub preexisting_supply_temperature_c: f64,
    pub active_operands: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands,
    >,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    retained_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedInput,
    >,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot
{
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment_executed =
        predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    debug_assert_eq!(guard_false || assignment_executed, retained_input.is_some());
    debug_assert_eq!(
        assignment_executed,
        retained_input
            .and_then(|input| input.active_operands)
            .is_some()
    );

    let preexisting_supply_temperature_c =
        retained_input.map(|input| input.preexisting_supply_temperature_c);
    let evaluated = retained_input
        .and_then(|input| input.active_operands)
        .map(|operands| {
            let result = energyplus_psy_tdb_fn_h_w(
                operands.supply_enthalpy_j_per_kg,
                operands.supply_humidity_ratio,
            );
            (operands, result)
        });
    let supply_enthalpy_j_per_kg = evaluated.map(|(operands, _)| operands.supply_enthalpy_j_per_kg);
    let supply_humidity_ratio = evaluated.map(|(operands, _)| operands.supply_humidity_ratio);
    let psychrometric_supply_temperature_result_c = evaluated.map(|(_, result)| result);
    let assigned_supply_temperature_c = psychrometric_supply_temperature_result_c;
    let resulting_supply_temperature_c = if assignment_executed {
        assigned_supply_temperature_c
    } else {
        preexisting_supply_temperature_c
    };

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute::PositiveGuardFalseFallthrough
    } else if predecessor.capacity_limit_guard_false_fallthrough_skipped {
        state.capacity_limit_guard_false_fallthrough_skip_count += 1;
        state.witnessed_capacity_limit_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute::ActiveCapacityLimitGuardFalseFallthrough
    } else if guard_false {
        state.capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
        state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute::CapacityLimitSensibleOutputGuardFalseFallthrough
    } else {
        debug_assert!(assignment_executed);
        state.capacity_limit_sensible_output_supply_temperature_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len();
        state.supply_enthalpy_for_dry_bulb_inversion_read_count += 1;
        state.supply_humidity_ratio_for_dry_bulb_inversion_read_count += 1;
        state.psychrometric_supply_temperature_evaluation_count += 1;
        state.supply_temperature_assignment_write_count += 1;
        state.witnessed_capacity_limit_sensible_output_supply_temperature_assignment_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute::CapacityLimitSensibleOutputSupplyTemperatureAssigned
    };

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
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
                predecessor.predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed,
            predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed:
                assignment_executed,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_supply_temperature_assignment_executed:
                assignment_executed,
            preexisting_supply_temperature_c,
            supply_enthalpy_for_dry_bulb_inversion_read: assignment_executed,
            supply_enthalpy_j_per_kg,
            supply_humidity_ratio_for_dry_bulb_inversion_read: assignment_executed,
            supply_humidity_ratio,
            psychrometric_supply_temperature_evaluated: assignment_executed,
            psychrometric_supply_temperature_result_c,
            supply_temperature_assigned: assignment_executed,
            assigned_supply_temperature_c,
            resulting_supply_temperature_c,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
