use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Snapshot,
};

pub(in crate::ideal_loads::coupled_runtime) fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
) -> Snapshot {
    Snapshot {
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

pub(in crate::ideal_loads::coupled_runtime) fn snapshots_match_bit_exact(
    left: &Snapshot,
    right: &Snapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.moisture_demand_derived_supply_humidity_ratio,
            right.moisture_demand_derived_supply_humidity_ratio,
        ),
        (
            left.zone_node_humidity_ratio,
            right.zone_node_humidity_ratio,
        ),
        (
            left.calculated_supply_humidity_ratio_for_dehumidification,
            right.calculated_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.assigned_supply_humidity_ratio_for_dehumidification,
            right.assigned_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.resulting_supply_humidity_ratio_for_dehumidification,
            right.resulting_supply_humidity_ratio_for_dehumidification,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_eq(left, right));
    let mut left = *left;
    let mut right = *right;
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.moisture_demand_derived_supply_humidity_ratio = None;
        snapshot.zone_node_humidity_ratio = None;
        snapshot.calculated_supply_humidity_ratio_for_dehumidification = None;
        snapshot.assigned_supply_humidity_ratio_for_dehumidification = None;
        snapshot.resulting_supply_humidity_ratio_for_dehumidification = None;
    }
    values_match && left == right
}

fn option_bits_eq(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
