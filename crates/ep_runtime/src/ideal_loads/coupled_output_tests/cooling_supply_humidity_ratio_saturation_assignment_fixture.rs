use crate::{
    ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
    },
    psychrometrics::energyplus_psy_w_fn_tdb_rh_pb,
};

pub(super) fn calculation_cooling_supply_humidity_ratio_saturation_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
    cp334: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    cp344: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    pressure_pa: f64,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot {
    let active = !(predecessor.unit_off_skipped
        || predecessor.non_cooling_skipped
        || predecessor.positive_guard_false_fallthrough_skipped);
    let cp344_owned =
        active && cp344.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let cp334_owned = active && !cp344_owned;
    let temperature = if cp344_owned {
        cp344.resulting_supply_temperature_c
    } else if cp334_owned {
        cp334.assigned_supply_temperature_c
    } else {
        None
    };
    let saturation =
        temperature.map(|temperature| energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure_pa));

    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type: predecessor.predecessor_dehumidification_control_type,
        predecessor_local_supply_humidity_ratio_original_assignment_performed: predecessor.local_supply_humidity_ratio_original_assignment_performed,
        predecessor_resulting_supply_humidity_ratio_original: predecessor.resulting_supply_humidity_ratio_original,
        cp334_supply_temperature_mixed_air_limit_owned_read: cp334_owned,
        cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: cp344_owned,
        environment_outdoor_barometric_pressure_owned_read: active,
        purchased_air_supply_temperature_for_saturation_humidity_ratio_read: active,
        supply_temperature_for_saturation_humidity_ratio_c: temperature,
        environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: active,
        outdoor_barometric_pressure_pa: active.then_some(pressure_pa),
        psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: active,
        saturation_supply_humidity_ratio: saturation,
        local_saturation_supply_humidity_ratio_assignment_performed: active,
        assigned_saturation_supply_humidity_ratio: saturation,
        resulting_saturation_supply_humidity_ratio: saturation,
    }
}
