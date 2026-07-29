use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
};

pub(super) fn calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit_snapshot(
    predecessor: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
) -> PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot {
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type:
            predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed,
        predecessor_dehumidification_control_humidistat_case_selected_skip:
            predecessor.dehumidification_control_humidistat_case_selected_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed:
            false,
        dehumidification_control_humidistat_case_selected_skip:
            predecessor.dehumidification_control_humidistat_case_selected_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        supply_humidity_ratio_for_minimum_limit_maximum_read: false,
        supply_humidity_ratio_before_minimum_limit: None,
        minimum_cooling_supply_air_humidity_ratio_for_maximum_read: false,
        minimum_cooling_supply_air_humidity_ratio: None,
        source_shaped_two_argument_maximum_evaluated: false,
        maximum_supply_humidity_ratio: None,
        supply_humidity_ratio_assignment_performed: false,
        assigned_supply_humidity_ratio: None,
        resulting_supply_humidity_ratio: None,
    }
}
