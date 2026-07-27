//! JSON serialization for one CP320 source-site snapshot.

use ep_model::{DehumidificationControlType, HumidificationControlType};
use ep_runtime::PurchasedAirCalcCoolingHumidificationFlowSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(snapshot: PurchasedAirCalcCoolingHumidificationFlowSnapshot) -> Value {
    let mut value = json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "supply_mass_flow_rate_for_humidification_reset_assigned":
            snapshot.supply_mass_flow_rate_for_humidification_reset_assigned,
        "reset_supply_mass_flow_rate_for_humidification_kg_per_s":
            snapshot.reset_supply_mass_flow_rate_for_humidification_kg_per_s,
        "heating_on_read": snapshot.heating_on_read,
        "heating_on": snapshot.heating_on,
        "heating_on_body_entered": snapshot.heating_on_body_entered,
        "humidification_control_type_read": snapshot.humidification_control_type_read,
        "humidification_control_type":
            snapshot.humidification_control_type.map(humidification_control_type_name),
        "humidification_control_type_humidistat":
            snapshot.humidification_control_type_humidistat,
        "humidification_control_body_entered":
            snapshot.humidification_control_body_entered,
        "dehumidification_control_type_first_read":
            snapshot.dehumidification_control_type_first_read,
        "first_dehumidification_control_type":
            snapshot.first_dehumidification_control_type.map(dehumidification_control_type_name),
        "dehumidification_control_type_humidistat":
            snapshot.dehumidification_control_type_humidistat,
    });
    extend_object(
        &mut value,
        json!({
            "dehumidification_control_type_second_read":
                snapshot.dehumidification_control_type_second_read,
            "second_dehumidification_control_type":
                snapshot.second_dehumidification_control_type
                    .map(dehumidification_control_type_name),
            "dehumidification_control_type_none":
                snapshot.dehumidification_control_type_none,
            "humidification_control_condition_admitted":
                snapshot.humidification_control_condition_admitted,
            "zone_humidifying_setpoint_moisture_demand_read":
                snapshot.zone_humidifying_setpoint_moisture_demand_read,
            "zone_humidifying_setpoint_moisture_demand_kg_per_s":
                snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
            "zone_humidifying_setpoint_moisture_demand_assigned":
                snapshot.zone_humidifying_setpoint_moisture_demand_assigned,
            "assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s":
                snapshot.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            "maximum_heating_supply_air_humidity_ratio_read":
                snapshot.maximum_heating_supply_air_humidity_ratio_read,
            "maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air":
                snapshot.maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air,
            "zone_humidity_ratio_read": snapshot.zone_humidity_ratio_read,
            "zone_humidity_ratio_kg_water_per_kg_dry_air":
                snapshot.zone_humidity_ratio_kg_water_per_kg_dry_air,
            "delta_humidity_ratio_calculated": snapshot.delta_humidity_ratio_calculated,
            "delta_humidity_ratio_kg_water_per_kg_dry_air":
                snapshot.delta_humidity_ratio_kg_water_per_kg_dry_air,
            "delta_humidity_ratio_assigned": snapshot.delta_humidity_ratio_assigned,
            "assigned_delta_humidity_ratio_kg_water_per_kg_dry_air":
                snapshot.assigned_delta_humidity_ratio_kg_water_per_kg_dry_air,
            "delta_humidity_ratio_for_gate_read":
                snapshot.delta_humidity_ratio_for_gate_read,
            "delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air":
                snapshot.delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air,
        }),
    );
    extend_object(
        &mut value,
        json!({
            "delta_humidity_ratio_comparison_evaluated":
                snapshot.delta_humidity_ratio_comparison_evaluated,
            "delta_humidity_ratio_above_small_delta":
                snapshot.delta_humidity_ratio_above_small_delta,
            "zone_humidifying_setpoint_moisture_demand_for_gate_read":
                snapshot.zone_humidifying_setpoint_moisture_demand_for_gate_read,
            "zone_humidifying_setpoint_moisture_demand_for_gate_kg_per_s":
                snapshot.zone_humidifying_setpoint_moisture_demand_for_gate_kg_per_s,
            "zone_humidifying_setpoint_moisture_demand_comparison_evaluated":
                snapshot.zone_humidifying_setpoint_moisture_demand_comparison_evaluated,
            "zone_humidifying_setpoint_moisture_demand_above_zero":
                snapshot.zone_humidifying_setpoint_moisture_demand_above_zero,
            "humidification_flow_body_entered": snapshot.humidification_flow_body_entered,
            "zone_humidifying_setpoint_moisture_demand_for_division_read":
                snapshot.zone_humidifying_setpoint_moisture_demand_for_division_read,
            "zone_humidifying_setpoint_moisture_demand_for_division_kg_per_s":
                snapshot.zone_humidifying_setpoint_moisture_demand_for_division_kg_per_s,
            "delta_humidity_ratio_for_division_read":
                snapshot.delta_humidity_ratio_for_division_read,
            "delta_humidity_ratio_for_division_kg_water_per_kg_dry_air":
                snapshot.delta_humidity_ratio_for_division_kg_water_per_kg_dry_air,
            "supply_mass_flow_rate_for_humidification_calculated":
                snapshot.supply_mass_flow_rate_for_humidification_calculated,
            "calculated_supply_mass_flow_rate_for_humidification_kg_per_s":
                snapshot.calculated_supply_mass_flow_rate_for_humidification_kg_per_s,
            "supply_mass_flow_rate_for_humidification_assigned":
                snapshot.supply_mass_flow_rate_for_humidification_assigned,
            "assigned_supply_mass_flow_rate_for_humidification_kg_per_s":
                snapshot.assigned_supply_mass_flow_rate_for_humidification_kg_per_s,
            "resulting_supply_mass_flow_rate_for_humidification_kg_per_s":
                snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        }),
    );
    value
}

fn humidification_control_type_name(control_type: HumidificationControlType) -> &'static str {
    match control_type {
        HumidificationControlType::None => "None",
        HumidificationControlType::ConstantSupplyHumidityRatio => "ConstantSupplyHumidityRatio",
        HumidificationControlType::Humidistat => "Humidistat",
    }
}

fn dehumidification_control_type_name(control_type: DehumidificationControlType) -> &'static str {
    match control_type {
        DehumidificationControlType::None => "None",
        DehumidificationControlType::ConstantSensibleHeatRatio => "ConstantSensibleHeatRatio",
        DehumidificationControlType::ConstantSupplyHumidityRatio => "ConstantSupplyHumidityRatio",
        DehumidificationControlType::Humidistat => "Humidistat",
    }
}

fn extend_object(target: &mut Value, extension: Value) {
    let Value::Object(extension) = extension else {
        return;
    };
    if let Value::Object(target) = target {
        target.extend(extension);
    }
}
