use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot
{
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let execution =
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    let active_prefix = guard_false || execution;
    let preexisting = active_prefix
        .then_some(predecessor.resulting_supply_temperature_c)
        .flatten();
    let right = execution
        .then_some(mixed_air.mixed_air_temperature_c)
        .flatten();
    let minimum = preexisting
        .zip(right)
        .map(|(left, right)| if left < right { left } else { right });

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough:
            predecessor.predecessor_active_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated:
            predecessor.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered:
            predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough:
            predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_capacity_limit_cp_air_assignment_executed:
            predecessor.predecessor_capacity_limit_cp_air_assignment_executed,
        predecessor_capacity_limit_sensible_output_assignment_executed:
            predecessor.predecessor_capacity_limit_sensible_output_assignment_executed,
        predecessor_capacity_limit_sensible_output_guard_evaluated:
            predecessor.predecessor_capacity_limit_sensible_output_guard_evaluated,
        predecessor_capacity_limit_sensible_output_guard_false_fallthrough:
            predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
        predecessor_capacity_limit_sensible_output_adjustment_body_entered:
            predecessor.predecessor_capacity_limit_sensible_output_adjustment_body_entered,
        predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed:
            predecessor
                .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed,
        predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed:
            predecessor
                .predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed,
        predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed:
            execution,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_false_fallthrough_skipped:
            predecessor.capacity_limit_guard_false_fallthrough_skipped,
        capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed: execution,
        preexisting_supply_temperature_c: preexisting,
        supply_temperature_for_minimum_read: execution,
        supply_temperature_before_mixed_air_limit_c: execution.then_some(preexisting).flatten(),
        mixed_air_temperature_for_minimum_read: execution,
        mixed_air_temperature_c: right,
        source_shaped_two_argument_minimum_evaluated: execution,
        minimum_supply_temperature_c: minimum,
        supply_temperature_assignment_performed: execution,
        assigned_supply_temperature_c: minimum,
        resulting_supply_temperature_c: if execution { minimum } else { preexisting },
    }
}
