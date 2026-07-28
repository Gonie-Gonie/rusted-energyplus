use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot {
    let assignment_executed = predecessor.capacity_limit_guard_false_fallthrough_skipped
        || predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let mixed_air_humidity_ratio = assignment_executed
        .then_some(mixed_air.mixed_air_humidity_ratio)
        .flatten();

    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
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
            predecessor
                .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_false_fallthrough_skipped:
            predecessor.capacity_limit_guard_false_fallthrough_skipped,
        capacity_limit_sensible_output_guard_false_fallthrough:
            predecessor.capacity_limit_sensible_output_guard_false_fallthrough,
        capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
            predecessor
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
        post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed:
            assignment_executed,
        mixed_air_humidity_ratio_read: assignment_executed,
        mixed_air_humidity_ratio,
        supply_humidity_ratio_assignment_performed: assignment_executed,
        assigned_supply_humidity_ratio: mixed_air_humidity_ratio,
    }
}
