//! JSON serialization for one CP319 source-site snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingDehumidificationFlowSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) -> Value {
    let mut value = json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "source_order": snapshot.source_order,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_cooling_on_body_entered":
            snapshot.predecessor_cooling_on_body_entered,
        "predecessor_delta_temperature_body_entered":
            snapshot.predecessor_delta_temperature_body_entered,
        "predecessor_supply_mass_flow_rate_for_cool_assigned":
            snapshot.predecessor_supply_mass_flow_rate_for_cool_assigned,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "supply_mass_flow_rate_for_dehumidification_reset_assigned":
            snapshot.supply_mass_flow_rate_for_dehumidification_reset_assigned,
        "reset_supply_mass_flow_rate_for_dehumidification_kg_per_s":
            snapshot.reset_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        "cooling_on_read": snapshot.cooling_on_read,
        "cooling_on": snapshot.cooling_on,
        "cooling_on_body_entered": snapshot.cooling_on_body_entered,
        "dehumidification_control_type_read":
            snapshot.dehumidification_control_type_read,
        "dehumidification_control_type":
            snapshot.dehumidification_control_type.map(control_type_name),
        "dehumidification_control_type_humidistat":
            snapshot.dehumidification_control_type_humidistat,
        "dehumidification_control_body_entered":
            snapshot.dehumidification_control_body_entered,
    });
    extend_object(
        &mut value,
        json!({
            "zone_dehumidifying_setpoint_moisture_demand_read":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_read,
            "zone_dehumidifying_setpoint_moisture_demand_kg_per_s":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            "zone_dehumidifying_setpoint_moisture_demand_assigned":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_assigned,
            "assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s":
                snapshot.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            "minimum_cooling_supply_air_humidity_ratio_read":
                snapshot.minimum_cooling_supply_air_humidity_ratio_read,
            "minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air":
                snapshot.minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air,
            "zone_humidity_ratio_read": snapshot.zone_humidity_ratio_read,
            "zone_humidity_ratio_kg_water_per_kg_dry_air":
                snapshot.zone_humidity_ratio_kg_water_per_kg_dry_air,
            "delta_humidity_ratio_calculated":
                snapshot.delta_humidity_ratio_calculated,
            "delta_humidity_ratio_kg_water_per_kg_dry_air":
                snapshot.delta_humidity_ratio_kg_water_per_kg_dry_air,
            "delta_humidity_ratio_assigned": snapshot.delta_humidity_ratio_assigned,
            "assigned_delta_humidity_ratio_kg_water_per_kg_dry_air":
                snapshot.assigned_delta_humidity_ratio_kg_water_per_kg_dry_air,
            "delta_humidity_ratio_for_gate_read":
                snapshot.delta_humidity_ratio_for_gate_read,
            "delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air":
                snapshot.delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air,
            "delta_humidity_ratio_comparison_evaluated":
                snapshot.delta_humidity_ratio_comparison_evaluated,
            "delta_humidity_ratio_below_negative_small_delta":
                snapshot.delta_humidity_ratio_below_negative_small_delta,
        }),
    );
    extend_object(
        &mut value,
        json!({
            "zone_dehumidifying_setpoint_moisture_demand_for_gate_read":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_for_gate_read,
            "zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s,
            "zone_dehumidifying_setpoint_moisture_demand_comparison_evaluated":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_comparison_evaluated,
            "zone_dehumidifying_setpoint_moisture_demand_below_zero":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_below_zero,
            "dehumidification_flow_body_entered":
                snapshot.dehumidification_flow_body_entered,
            "zone_dehumidifying_setpoint_moisture_demand_for_division_read":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_for_division_read,
            "zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s":
                snapshot.zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s,
            "delta_humidity_ratio_for_division_read":
                snapshot.delta_humidity_ratio_for_division_read,
            "delta_humidity_ratio_for_division_kg_water_per_kg_dry_air":
                snapshot.delta_humidity_ratio_for_division_kg_water_per_kg_dry_air,
            "supply_mass_flow_rate_for_dehumidification_calculated":
                snapshot.supply_mass_flow_rate_for_dehumidification_calculated,
            "calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s":
                snapshot.calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            "supply_mass_flow_rate_for_dehumidification_assigned":
                snapshot.supply_mass_flow_rate_for_dehumidification_assigned,
            "assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s":
                snapshot.assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            "resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s":
                snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        }),
    );
    value
}

fn control_type_name(control_type: DehumidificationControlType) -> &'static str {
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
