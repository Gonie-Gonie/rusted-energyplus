use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
};

pub(super) fn calculation_cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot {
    let active = !(predecessor.unit_off_skipped
        || predecessor.non_cooling_skipped
        || predecessor.positive_guard_false_fallthrough_skipped);
    let original = predecessor.predecessor_resulting_supply_humidity_ratio_original;
    let saturation = predecessor.resulting_saturation_supply_humidity_ratio;
    let minimum = match (original, saturation) {
        (Some(left), Some(right)) if active => Some(if left < right { left } else { right }),
        _ => None,
    };
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough:
            predecessor.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough:
            predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed:
            predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed:
            predecessor.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough:
            predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type:
            predecessor.predecessor_dehumidification_control_type,
        predecessor_local_supply_humidity_ratio_original_assignment_performed:
            predecessor.predecessor_local_supply_humidity_ratio_original_assignment_performed,
        predecessor_resulting_supply_humidity_ratio_original: original,
        predecessor_local_saturation_supply_humidity_ratio_assignment_performed:
            predecessor.local_saturation_supply_humidity_ratio_assignment_performed,
        predecessor_resulting_saturation_supply_humidity_ratio: saturation,
        cp376_original_supply_humidity_ratio_owned_read: active,
        cp377_saturation_supply_humidity_ratio_owned_read: active,
        local_original_supply_humidity_ratio_for_saturation_limit_minimum_read: active,
        original_supply_humidity_ratio_before_saturation_limit: if active {
            original
        } else {
            None
        },
        local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read: active,
        saturation_supply_humidity_ratio_for_limit: if active { saturation } else { None },
        source_shaped_two_argument_minimum_evaluated: active,
        minimum_supply_humidity_ratio_after_saturation_limit: minimum,
        purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed: active,
        assigned_supply_humidity_ratio: minimum,
        resulting_supply_humidity_ratio: minimum,
    }
}
