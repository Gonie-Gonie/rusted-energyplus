use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
};

pub(super) fn calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
) -> PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot {
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_moisture_demand_assignment_executed:
            predecessor
                .dehumidification_control_humidistat_moisture_demand_assignment_executed,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: predecessor
            .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor
            .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed:
            predecessor
                .dehumidification_control_humidistat_moisture_demand_assignment_executed,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: predecessor
            .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        zone_dehumidifying_setpoint_moisture_demand_read: false,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: None,
        supply_mass_flow_rate_read: false,
        supply_mass_flow_rate_kg_per_s: None,
        moisture_demand_derived_supply_humidity_ratio_calculated: false,
        moisture_demand_derived_supply_humidity_ratio: None,
        zone_node_humidity_ratio_read: false,
        zone_node_humidity_ratio: None,
        supply_humidity_ratio_for_dehumidification_calculated: false,
        calculated_supply_humidity_ratio_for_dehumidification: None,
        supply_humidity_ratio_for_dehumidification_assigned: false,
        assigned_supply_humidity_ratio_for_dehumidification: None,
        resulting_supply_humidity_ratio_for_dehumidification: None,
    }
}
