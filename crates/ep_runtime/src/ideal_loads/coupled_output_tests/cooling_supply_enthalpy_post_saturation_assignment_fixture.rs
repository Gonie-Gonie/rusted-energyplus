use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

pub(super) fn calculation_cooling_supply_enthalpy_post_saturation_assignment_snapshot(
    humidity: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    temperature: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
) -> PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot {
    let active = !(humidity.unit_off_skipped
        || humidity.non_cooling_skipped
        || humidity.positive_guard_false_fallthrough_skipped);
    let supply_temperature_c = if active {
        temperature.supply_temperature_for_saturation_humidity_ratio_c
    } else {
        None
    };
    let supply_humidity_ratio = if active {
        humidity.resulting_supply_humidity_ratio
    } else {
        None
    };
    let enthalpy = match (supply_temperature_c, supply_humidity_ratio) {
        (Some(temperature), Some(humidity_ratio)) if active => {
            Some(energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio))
        }
        _ => None,
    };
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
        system: humidity.system,
        parent_call_ordinal: humidity.parent_call_ordinal,
        controlled_zone: humidity.controlled_zone,
        unit_off_skipped: humidity.unit_off_skipped,
        non_cooling_skipped: humidity.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            humidity.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough:
            humidity.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough:
            humidity.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed:
            humidity.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed:
            humidity.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough:
            humidity.dehumidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type:
            humidity.predecessor_dehumidification_control_type,
        predecessor_supply_humidity_ratio_saturation_limit_assignment_performed:
            humidity.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed,
        predecessor_resulting_supply_humidity_ratio: humidity.resulting_supply_humidity_ratio,
        cp377_supply_temperature_owned_read: active,
        cp334_supply_temperature_mixed_air_limit_owned_read:
            temperature.cp334_supply_temperature_mixed_air_limit_owned_read,
        cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read:
            temperature.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
        cp378_supply_humidity_ratio_saturation_limit_owned_read: active,
        purchased_air_supply_temperature_for_post_saturation_enthalpy_read: active,
        supply_temperature_c,
        purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read: active,
        supply_humidity_ratio,
        psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated: active,
        psychrometric_supply_enthalpy_j_per_kg: enthalpy,
        local_supply_enthalpy_after_saturation_limit_assignment_performed: active,
        assigned_supply_enthalpy_j_per_kg: enthalpy,
        resulting_supply_enthalpy_j_per_kg: enthalpy,
    }
}
