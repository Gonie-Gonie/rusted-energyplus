use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot,
};

pub(super) fn calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot {
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
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
            predecessor.predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip:
            predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            predecessor
                .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip:
            predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: predecessor.predecessor_heating_on_read,
        predecessor_heating_on: predecessor.predecessor_heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered:
            predecessor.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough:
            predecessor.predecessor_heating_on_guard_false_fallthrough,
        predecessor_humidification_control_type_read:
            predecessor.predecessor_humidification_control_type_read,
        predecessor_humidification_control_type:
            predecessor.predecessor_humidification_control_type,
        predecessor_humidification_control_type_humidistat:
            predecessor.predecessor_humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered:
            predecessor.predecessor_humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough:
            predecessor.predecessor_humidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type_first_read:
            predecessor.dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type:
            predecessor.first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat:
            predecessor.dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read:
            predecessor.dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type:
            predecessor.second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none:
            predecessor.dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered:
            predecessor.dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough:
            predecessor.dehumidification_control_guard_false_fallthrough,
        humidification_moisture_demand_assignment_executed: false,
        zone_humidifying_setpoint_moisture_demand_read: false,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
        zone_humidifying_setpoint_moisture_demand_assigned: false,
        assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
        resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
    }
}
