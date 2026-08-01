use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
};

pub(super) fn calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot,
    owner: PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot {
    let active = !(predecessor.unit_off_skipped
        || predecessor.non_cooling_skipped
        || predecessor.positive_guard_false_fallthrough_skipped);
    let owner_value = active
        .then_some(owner.resulting_supply_humidity_ratio)
        .flatten();
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor.predecessor_heating_on_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor.predecessor_humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor.predecessor_dehumidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type: predecessor.predecessor_dehumidification_control_type,
        predecessor_purchased_air_supply_humidity_ratio_assignment_performed: predecessor.purchased_air_supply_humidity_ratio_assignment_performed,
        predecessor_resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        cp375_maximum_assignment_owned_read: false,
        cp347_none_case_owned_read: active,
        cp356_constant_shr_owned_read: false,
        cp362_humidistat_owned_read: false,
        cp365_constant_supply_humidity_ratio_owned_read: false,
        purchased_air_supply_humidity_ratio_read: active,
        purchased_air_supply_humidity_ratio_before_saturation_check: owner_value,
        local_supply_humidity_ratio_original_assignment_performed: active,
        assigned_supply_humidity_ratio_original: owner_value,
        resulting_supply_humidity_ratio_original: owner_value,
    }
}
