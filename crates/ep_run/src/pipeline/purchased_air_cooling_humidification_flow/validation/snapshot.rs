//! Exact direct-lane shape checks for one CP320 source-site snapshot.

use ep_model::HumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingHumidificationFlowSnapshot;

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> bool {
    if !snapshot.cooling_body_entered {
        return usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
            == 1
            && skipped_source_shape(snapshot);
    }
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || !snapshot.supply_mass_flow_rate_for_humidification_reset_assigned
        || !same_option(
            snapshot.reset_supply_mass_flow_rate_for_humidification_kg_per_s,
            Some(0.0),
        )
        || !snapshot.heating_on_read
        || snapshot.heating_on != Some(true)
        || !snapshot.heating_on_body_entered
        || !snapshot.humidification_control_type_read
        || snapshot.humidification_control_type != Some(HumidificationControlType::None)
        || snapshot.humidification_control_type_humidistat != Some(false)
        || snapshot.humidification_control_body_entered
        || !downstream_absent(snapshot)
    {
        return false;
    }
    same_option(
        snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        Some(0.0),
    )
}

fn skipped_source_shape(snapshot: &PurchasedAirCalcCoolingHumidificationFlowSnapshot) -> bool {
    !snapshot.supply_mass_flow_rate_for_humidification_reset_assigned
        && snapshot
            .reset_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
        && !snapshot.heating_on_read
        && snapshot.heating_on.is_none()
        && !snapshot.heating_on_body_entered
        && !snapshot.humidification_control_type_read
        && snapshot.humidification_control_type.is_none()
        && snapshot.humidification_control_type_humidistat.is_none()
        && !snapshot.humidification_control_body_entered
        && downstream_absent(snapshot)
        && snapshot
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
}

fn downstream_absent(snapshot: &PurchasedAirCalcCoolingHumidificationFlowSnapshot) -> bool {
    !snapshot.dehumidification_control_type_first_read
        && snapshot.first_dehumidification_control_type.is_none()
        && snapshot.dehumidification_control_type_humidistat.is_none()
        && !snapshot.dehumidification_control_type_second_read
        && snapshot.second_dehumidification_control_type.is_none()
        && snapshot.dehumidification_control_type_none.is_none()
        && !snapshot.humidification_control_condition_admitted
        && !snapshot.zone_humidifying_setpoint_moisture_demand_read
        && snapshot
            .zone_humidifying_setpoint_moisture_demand_kg_per_s
            .is_none()
        && !snapshot.zone_humidifying_setpoint_moisture_demand_assigned
        && snapshot
            .assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s
            .is_none()
        && !snapshot.maximum_heating_supply_air_humidity_ratio_read
        && snapshot
            .maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air
            .is_none()
        && !snapshot.zone_humidity_ratio_read
        && snapshot
            .zone_humidity_ratio_kg_water_per_kg_dry_air
            .is_none()
        && !snapshot.delta_humidity_ratio_calculated
        && snapshot
            .delta_humidity_ratio_kg_water_per_kg_dry_air
            .is_none()
        && !snapshot.delta_humidity_ratio_assigned
        && snapshot
            .assigned_delta_humidity_ratio_kg_water_per_kg_dry_air
            .is_none()
        && !snapshot.delta_humidity_ratio_for_gate_read
        && snapshot
            .delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air
            .is_none()
        && !snapshot.delta_humidity_ratio_comparison_evaluated
        && snapshot.delta_humidity_ratio_above_small_delta.is_none()
        && !snapshot.zone_humidifying_setpoint_moisture_demand_for_gate_read
        && snapshot
            .zone_humidifying_setpoint_moisture_demand_for_gate_kg_per_s
            .is_none()
        && !snapshot.zone_humidifying_setpoint_moisture_demand_comparison_evaluated
        && snapshot
            .zone_humidifying_setpoint_moisture_demand_above_zero
            .is_none()
        && !snapshot.humidification_flow_body_entered
        && !snapshot.zone_humidifying_setpoint_moisture_demand_for_division_read
        && snapshot
            .zone_humidifying_setpoint_moisture_demand_for_division_kg_per_s
            .is_none()
        && !snapshot.delta_humidity_ratio_for_division_read
        && snapshot
            .delta_humidity_ratio_for_division_kg_water_per_kg_dry_air
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_humidification_calculated
        && snapshot
            .calculated_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_humidification_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
}

fn same_option(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
