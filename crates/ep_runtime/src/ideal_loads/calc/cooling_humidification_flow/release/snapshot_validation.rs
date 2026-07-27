use ep_model::HumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidificationFlowRetainedRoute,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot,
};

pub(in crate::ideal_loads) fn cooling_humidification_flow_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER;
    let unit_off =
        snapshot.unit_off_skipped && !snapshot.unit_body_entered && !snapshot.cooling_body_entered;
    let non_cooling = snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.cooling_body_entered;
    let cooling = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.cooling_body_entered;
    provenance
        && snapshot.predecessor_cooling_body_entered == snapshot.cooling_body_entered
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(cooling) == 1
        && if cooling {
            active_direct_sites_are_exact(snapshot)
        } else {
            all_sites_are_skipped(snapshot)
        }
}

pub(super) fn cooling_humidification_flow_snapshot_route(
    snapshot: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> Option<PurchasedAirCalcCoolingHumidificationFlowRetainedRoute> {
    if !cooling_humidification_flow_snapshot_is_exact_direct_release(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::NonCooling)
    } else {
        Some(PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::HumidificationControlInactive)
    }
}

fn active_direct_sites_are_exact(
    snapshot: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> bool {
    snapshot.supply_mass_flow_rate_for_humidification_reset_assigned
        && has_bits(
            snapshot.reset_supply_mass_flow_rate_for_humidification_kg_per_s,
            0.0,
        )
        && snapshot.heating_on_read
        && snapshot.heating_on == Some(true)
        && snapshot.heating_on_body_entered
        && snapshot.humidification_control_type_read
        && snapshot.humidification_control_type == Some(HumidificationControlType::None)
        && snapshot.humidification_control_type_humidistat == Some(false)
        && !snapshot.humidification_control_body_entered
        && downstream_sites_are_skipped(snapshot)
        && has_bits(
            snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
            0.0,
        )
}

fn all_sites_are_skipped(snapshot: PurchasedAirCalcCoolingHumidificationFlowSnapshot) -> bool {
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
        && downstream_sites_are_skipped(snapshot)
        && snapshot
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
}

fn downstream_sites_are_skipped(
    snapshot: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> bool {
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

fn has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
