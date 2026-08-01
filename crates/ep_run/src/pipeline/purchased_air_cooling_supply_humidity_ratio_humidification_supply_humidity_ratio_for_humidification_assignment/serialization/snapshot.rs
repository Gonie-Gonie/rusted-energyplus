//! JSON serialization for one CP373 humidity-ratio assignment snapshot.

use ep_model::{DehumidificationControlType, HumidificationControlType};
use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_no_outdoor_air_fallback_entered":
            snapshot.predecessor_no_outdoor_air_fallback_entered,
        "predecessor_positive_supply_mass_flow_body_entered":
            snapshot.predecessor_positive_supply_mass_flow_body_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "predecessor_dehumidification_control_type": snapshot
            .predecessor_dehumidification_control_type
            .map(dehumidification_control_name),
        "predecessor_dehumidification_control_none_case_completed_skip":
            snapshot.predecessor_dehumidification_control_none_case_completed_skip,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "predecessor_dehumidification_control_humidistat_case_completed_skip":
            snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip,
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        "predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break":
            snapshot.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "dehumidification_control_humidistat_case_completed_skip":
            snapshot.dehumidification_control_humidistat_case_completed_skip,
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        "predecessor_heating_on_read": snapshot.predecessor_heating_on_read,
        "predecessor_heating_on": snapshot.predecessor_heating_on,
        "predecessor_cooling_supply_humidity_ratio_humidification_body_entered":
            snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        "predecessor_heating_on_guard_false_fallthrough":
            snapshot.predecessor_heating_on_guard_false_fallthrough,
        "predecessor_humidification_control_type_read":
            snapshot.predecessor_humidification_control_type_read,
        "predecessor_humidification_control_type": snapshot
            .predecessor_humidification_control_type
            .map(humidification_control_name),
        "predecessor_humidification_control_type_humidistat":
            snapshot.predecessor_humidification_control_type_humidistat,
        "predecessor_humidification_control_body_entered":
            snapshot.predecessor_humidification_control_body_entered,
        "predecessor_humidification_control_guard_false_fallthrough":
            snapshot.predecessor_humidification_control_guard_false_fallthrough,
        "predecessor_dehumidification_control_type_first_read":
            snapshot.predecessor_dehumidification_control_type_first_read,
        "predecessor_first_dehumidification_control_type": snapshot
            .predecessor_first_dehumidification_control_type
            .map(dehumidification_control_name),
        "predecessor_dehumidification_control_type_humidistat":
            snapshot.predecessor_dehumidification_control_type_humidistat,
        "predecessor_dehumidification_control_type_second_read":
            snapshot.predecessor_dehumidification_control_type_second_read,
        "predecessor_second_dehumidification_control_type": snapshot
            .predecessor_second_dehumidification_control_type
            .map(dehumidification_control_name),
        "predecessor_dehumidification_control_type_none":
            snapshot.predecessor_dehumidification_control_type_none,
        "predecessor_dehumidification_control_body_entered":
            snapshot.predecessor_dehumidification_control_body_entered,
        "predecessor_dehumidification_control_guard_false_fallthrough":
            snapshot.predecessor_dehumidification_control_guard_false_fallthrough,
        "predecessor_humidification_moisture_demand_assignment_executed":
            snapshot.predecessor_humidification_moisture_demand_assignment_executed,
        "predecessor_zone_humidifying_setpoint_moisture_demand_read":
            snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_read,
        "predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s":
            json_number(snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s),
        "predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits":
            ieee_bits(snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s),
        "predecessor_zone_humidifying_setpoint_moisture_demand_assigned":
            snapshot.predecessor_zone_humidifying_setpoint_moisture_demand_assigned,
        "predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s":
            json_number(snapshot.predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s),
        "predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits":
            ieee_bits(snapshot.predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s),
        "predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s":
            json_number(snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s),
        "predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits":
            ieee_bits(snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s),
        "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed":
            snapshot.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed,
        "dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed":
            snapshot.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed,
        "zone_humidifying_setpoint_moisture_demand_read":
            snapshot.zone_humidifying_setpoint_moisture_demand_read,
        "zone_humidifying_setpoint_moisture_demand_kg_per_s":
            json_number(snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s),
        "zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits":
            ieee_bits(snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s),
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s":
            json_number(snapshot.supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "moisture_demand_derived_supply_humidity_ratio_calculated":
            snapshot.moisture_demand_derived_supply_humidity_ratio_calculated,
        "moisture_demand_derived_supply_humidity_ratio":
            json_number(snapshot.moisture_demand_derived_supply_humidity_ratio),
        "moisture_demand_derived_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.moisture_demand_derived_supply_humidity_ratio),
        "zone_node_humidity_ratio_read": snapshot.zone_node_humidity_ratio_read,
        "zone_node_humidity_ratio": json_number(snapshot.zone_node_humidity_ratio),
        "zone_node_humidity_ratio_ieee_bits": ieee_bits(snapshot.zone_node_humidity_ratio),
        "supply_humidity_ratio_for_humidification_calculated":
            snapshot.supply_humidity_ratio_for_humidification_calculated,
        "calculated_supply_humidity_ratio_for_humidification":
            json_number(snapshot.calculated_supply_humidity_ratio_for_humidification),
        "calculated_supply_humidity_ratio_for_humidification_ieee_bits":
            ieee_bits(snapshot.calculated_supply_humidity_ratio_for_humidification),
        "supply_humidity_ratio_for_humidification_assigned":
            snapshot.supply_humidity_ratio_for_humidification_assigned,
        "assigned_supply_humidity_ratio_for_humidification":
            json_number(snapshot.assigned_supply_humidity_ratio_for_humidification),
        "assigned_supply_humidity_ratio_for_humidification_ieee_bits":
            ieee_bits(snapshot.assigned_supply_humidity_ratio_for_humidification),
        "resulting_supply_humidity_ratio_for_humidification":
            json_number(snapshot.resulting_supply_humidity_ratio_for_humidification),
        "resulting_supply_humidity_ratio_for_humidification_ieee_bits":
            ieee_bits(snapshot.resulting_supply_humidity_ratio_for_humidification),
    })
}

fn dehumidification_control_name(control: DehumidificationControlType) -> &'static str {
    match control {
        DehumidificationControlType::None => "None",
        DehumidificationControlType::ConstantSensibleHeatRatio => "ConstantSensibleHeatRatio",
        DehumidificationControlType::Humidistat => "Humidistat",
        DehumidificationControlType::ConstantSupplyHumidityRatio => "ConstantSupplyHumidityRatio",
    }
}

fn humidification_control_name(control: HumidificationControlType) -> &'static str {
    match control {
        HumidificationControlType::None => "None",
        HumidificationControlType::ConstantSupplyHumidityRatio => "ConstantSupplyHumidityRatio",
        HumidificationControlType::Humidistat => "Humidistat",
    }
}

fn json_number(value: Option<f64>) -> Value {
    value
        .filter(|value| value.is_finite())
        .map_or(Value::Null, |value| json!(value))
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}

#[cfg(test)]
mod tests;
