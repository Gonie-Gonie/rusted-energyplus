//! Pure CP343-to-CP344 Cooling capacity-limit supply-temperature mixed-air limit.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitActiveOperands
{
    pub mixed_air_temperature_c: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput
{
    pub preexisting_supply_temperature_c: f64,
    pub active_operands: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitActiveOperands,
    >,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    retained_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput,
    >,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot
{
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let limit_executed =
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    debug_assert_eq!(guard_false || limit_executed, retained_input.is_some());
    debug_assert_eq!(
        limit_executed,
        retained_input
            .and_then(|input| input.active_operands)
            .is_some()
    );

    let preexisting_supply_temperature_c =
        retained_input.map(|input| input.preexisting_supply_temperature_c);
    let active_operands = retained_input.and_then(|input| input.active_operands);
    let supply_temperature_before_mixed_air_limit_c =
        active_operands.and(preexisting_supply_temperature_c);
    let mixed_air_temperature_c =
        active_operands.map(|operands| operands.mixed_air_temperature_c);
    let minimum_supply_temperature_c =
        active_operands.zip(preexisting_supply_temperature_c).map(
            |(operands, supply_temperature)| {
                source_shaped_two_argument_minimum(
                    supply_temperature,
                    operands.mixed_air_temperature_c,
                )
            },
        );
    let assigned_supply_temperature_c = minimum_supply_temperature_c;
    let resulting_supply_temperature_c = if limit_executed {
        assigned_supply_temperature_c
    } else {
        preexisting_supply_temperature_c
    };

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute::PositiveGuardFalseFallthrough
    } else if predecessor.capacity_limit_guard_false_fallthrough_skipped {
        state.capacity_limit_guard_false_fallthrough_skip_count += 1;
        state.witnessed_capacity_limit_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute::ActiveCapacityLimitGuardFalseFallthrough
    } else if guard_false {
        state.capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
        state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute::CapacityLimitSensibleOutputGuardFalseFallthrough
    } else {
        debug_assert!(limit_executed);
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
                .len();
        state.supply_temperature_for_minimum_read_count += 1;
        state.mixed_air_temperature_for_minimum_read_count += 1;
        state.source_shaped_two_argument_minimum_evaluation_count += 1;
        state.supply_temperature_assignment_write_count += 1;
        state
            .witnessed_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count +=
            1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute::CapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitExecuted
    };

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
                predecessor.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed,
            predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed:
                limit_executed,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
                limit_executed,
            preexisting_supply_temperature_c,
            supply_temperature_for_minimum_read: limit_executed,
            supply_temperature_before_mixed_air_limit_c,
            mixed_air_temperature_for_minimum_read: limit_executed,
            mixed_air_temperature_c,
            source_shaped_two_argument_minimum_evaluated: limit_executed,
            minimum_supply_temperature_c,
            supply_temperature_assignment_performed: limit_executed,
            assigned_supply_temperature_c,
            resulting_supply_temperature_c,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
