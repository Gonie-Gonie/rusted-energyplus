//! Exact direct-release validation for CP319 snapshots.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
};

pub(in crate::ideal_loads) fn cooling_dehumidification_flow_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered;
    let non_cooling = snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered;
    let cooling = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered;
    let direct_predecessor_shape = snapshot.predecessor_cooling_on_body_entered == cooling
        && snapshot.predecessor_supply_mass_flow_rate_for_cool_assigned
            == snapshot.predecessor_delta_temperature_body_entered
        && (!snapshot.predecessor_delta_temperature_body_entered || cooling);

    provenance
        && direct_predecessor_shape
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(cooling) == 1
        && if cooling {
            active_direct_sites_are_exact(snapshot)
        } else {
            skipped_sites_are_exact(snapshot)
        }
}

pub(super) fn cooling_dehumidification_flow_snapshot_route(
    snapshot: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) -> Option<PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute> {
    if !cooling_dehumidification_flow_snapshot_is_exact_direct_release(snapshot) {
        return None;
    }
    if snapshot.unit_off_skipped {
        Some(PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::NonCooling)
    } else {
        Some(
            PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::
                DehumidificationControlInactive,
        )
    }
}

fn active_direct_sites_are_exact(
    snapshot: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) -> bool {
    snapshot.supply_mass_flow_rate_for_dehumidification_reset_assigned
        && option_f64_has_bits(
            snapshot.reset_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            0.0,
        )
        && snapshot.cooling_on_read
        && snapshot.cooling_on == Some(true)
        && snapshot.cooling_on_body_entered
        && snapshot.dehumidification_control_type_read
        && snapshot.dehumidification_control_type == Some(DehumidificationControlType::None)
        && snapshot.dehumidification_control_type_humidistat == Some(false)
        && !snapshot.dehumidification_control_body_entered
        && humidistat_sites_are_skipped(snapshot)
        && option_f64_has_bits(
            snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            0.0,
        )
}

fn skipped_sites_are_exact(snapshot: PurchasedAirCalcCoolingDehumidificationFlowSnapshot) -> bool {
    !snapshot.supply_mass_flow_rate_for_dehumidification_reset_assigned
        && snapshot
            .reset_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && !snapshot.cooling_on_read
        && snapshot.cooling_on.is_none()
        && !snapshot.cooling_on_body_entered
        && !snapshot.dehumidification_control_type_read
        && snapshot.dehumidification_control_type.is_none()
        && snapshot.dehumidification_control_type_humidistat.is_none()
        && !snapshot.dehumidification_control_body_entered
        && humidistat_sites_are_skipped(snapshot)
        && snapshot
            .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
}

fn humidistat_sites_are_skipped(
    snapshot: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) -> bool {
    !snapshot.zone_dehumidifying_setpoint_moisture_demand_read
        && snapshot
            .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
            .is_none()
        && !snapshot.zone_dehumidifying_setpoint_moisture_demand_assigned
        && snapshot
            .assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
            .is_none()
        && !snapshot.minimum_cooling_supply_air_humidity_ratio_read
        && snapshot
            .minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air
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
        && snapshot
            .delta_humidity_ratio_below_negative_small_delta
            .is_none()
        && !snapshot.zone_dehumidifying_setpoint_moisture_demand_for_gate_read
        && snapshot
            .zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s
            .is_none()
        && !snapshot.zone_dehumidifying_setpoint_moisture_demand_comparison_evaluated
        && snapshot
            .zone_dehumidifying_setpoint_moisture_demand_below_zero
            .is_none()
        && !snapshot.dehumidification_flow_body_entered
        && !snapshot.zone_dehumidifying_setpoint_moisture_demand_for_division_read
        && snapshot
            .zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s
            .is_none()
        && !snapshot.delta_humidity_ratio_for_division_read
        && snapshot
            .delta_humidity_ratio_for_division_kg_water_per_kg_dry_air
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_dehumidification_calculated
        && snapshot
            .calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_dehumidification_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
}

fn option_f64_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
