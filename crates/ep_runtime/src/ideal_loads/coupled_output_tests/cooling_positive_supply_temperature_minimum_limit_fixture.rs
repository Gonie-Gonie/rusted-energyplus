use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_temperature_minimum_limit_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    minimum_cooling_supply_air_temperature_c: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot {
    let minimum_limit_executed = predecessor.supply_temperature_assignment_executed;
    let supply_temperature_before_minimum_limit_c = minimum_limit_executed
        .then_some(predecessor.supply_temperature_c)
        .flatten();
    let minimum_cooling_supply_air_temperature_c =
        minimum_limit_executed.then_some(minimum_cooling_supply_air_temperature_c);
    let maximum_supply_temperature_c = supply_temperature_before_minimum_limit_c
        .zip(minimum_cooling_supply_air_temperature_c)
        .map(|(left, right)| if left < right { right } else { left });

    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER,
        system: predecessor.system,
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
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        supply_temperature_minimum_limit_executed: minimum_limit_executed,
        supply_temperature_for_maximum_read: minimum_limit_executed,
        supply_temperature_before_minimum_limit_c,
        minimum_cooling_supply_air_temperature_for_maximum_read: minimum_limit_executed,
        minimum_cooling_supply_air_temperature_c,
        source_shaped_two_argument_maximum_evaluated: minimum_limit_executed,
        maximum_supply_temperature_c,
        supply_temperature_assignment_performed: minimum_limit_executed,
        assigned_supply_temperature_c: maximum_supply_temperature_c,
    }
}
