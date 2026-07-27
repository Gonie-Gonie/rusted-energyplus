//! Pure CP319-to-CP320 source-characterization transition.

use ep_model::{DehumidificationControlType, HumidificationControlType};

use super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidificationFlowInput,
    PurchasedAirCalcCoolingHumidificationFlowRetainedRoute,
    PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingDehumidificationFlowSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_humidification_flow_state(
    state: &mut PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    predecessor: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    input: PurchasedAirCalcCoolingHumidificationFlowInput,
) -> PurchasedAirCalcCoolingHumidificationFlowSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let reset = cooling.then_some(0.0_f64);
    let heating_on = if cooling {
        Some(input.heating_on)
    } else {
        None
    };
    let heating_body = heating_on == Some(true);

    let humid_control = if heating_body {
        Some(input.humidification_control_type)
    } else {
        None
    };
    let humidistat = humid_control.map(|value| value == HumidificationControlType::Humidistat);
    let humid_body = humidistat == Some(true);

    // Preserve the source's repeated reads and left-to-right short-circuit `||`.
    let first_dehumid_control = if humid_body {
        Some(input.dehumidification_control_type)
    } else {
        None
    };
    let dehumid_is_humidistat =
        first_dehumid_control.map(|value| value == DehumidificationControlType::Humidistat);
    let second_dehumid_control = if dehumid_is_humidistat == Some(false) {
        Some(input.dehumidification_control_type)
    } else {
        None
    };
    let dehumid_is_none =
        second_dehumid_control.map(|value| value == DehumidificationControlType::None);
    let controls_admitted = dehumid_is_humidistat == Some(true) || dehumid_is_none == Some(true);

    let demand = if controls_admitted {
        Some(input.zone_humidifying_setpoint_moisture_demand_kg_per_s)
    } else {
        None
    };
    let max_supply = if controls_admitted {
        Some(input.maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air)
    } else {
        None
    };
    let zone = if controls_admitted {
        Some(input.zone_humidity_ratio_kg_water_per_kg_dry_air)
    } else {
        None
    };
    let delta = max_supply.zip(zone).map(|(supply, zone)| supply - zone);
    let delta_gate = delta;
    let delta_above = delta_gate.map(|value| {
        value > PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO
    });
    // Preserve the source's left-to-right short-circuit `&&`.
    let demand_gate = if delta_above == Some(true) {
        demand
    } else {
        None
    };
    let demand_above = demand_gate.map(|value| value > 0.0);
    let flow_body = demand_above == Some(true);
    let division_demand = if flow_body { demand } else { None };
    let division_delta = if flow_body { delta } else { None };
    let calculated = division_demand
        .zip(division_delta)
        .map(|(demand, delta)| demand / delta);
    let resulting = if cooling {
        Some(calculated.unwrap_or(0.0))
    } else {
        None
    };

    state.transition_count += 1;
    if cooling {
        state.cooling_body_entry_count += 1;
        state.reset_assignment_count += 1;
        state.heating_on_read_count += 1;
        if heating_body {
            state.heating_on_body_entry_count += 1;
            state.humidification_control_type_read_count += 1;
            if humid_body {
                state.humidification_control_type_humidistat_count += 1;
                state.humidification_control_body_entry_count += 1;
                state.dehumidification_control_type_first_read_count += 1;
                if dehumid_is_humidistat == Some(true) {
                    state.dehumidification_control_type_humidistat_count += 1;
                } else {
                    state.dehumidification_control_type_second_read_count += 1;
                    if dehumid_is_none == Some(true) {
                        state.dehumidification_control_type_none_count += 1;
                    } else {
                        state.dehumidification_control_type_rejected_count += 1;
                    }
                }
                if controls_admitted {
                    state.admitted_control_body_entry_count += 1;
                    state.moisture_demand_read_count += 1;
                    state.moisture_demand_assignment_count += 1;
                    state.maximum_heating_supply_humidity_ratio_read_count += 1;
                    state.zone_humidity_ratio_read_count += 1;
                    state.delta_calculation_count += 1;
                    state.delta_assignment_count += 1;
                    state.delta_gate_read_count += 1;
                    state.delta_comparison_count += 1;
                    if delta_above == Some(true) {
                        state.delta_comparison_satisfied_count += 1;
                        state.moisture_demand_gate_read_count += 1;
                        state.moisture_demand_comparison_count += 1;
                        if flow_body {
                            state.moisture_demand_comparison_satisfied_count += 1;
                            state.humidification_flow_body_entry_count += 1;
                            state.moisture_demand_division_read_count += 1;
                            state.delta_division_read_count += 1;
                            state.calculation_count += 1;
                            state.assignment_count += 1;
                        } else {
                            state.moisture_demand_fallthrough_count += 1;
                        }
                    } else {
                        state.delta_fallthrough_count += 1;
                    }
                }
            } else {
                state.humidification_control_type_fallthrough_count += 1;
            }
        } else {
            state.heating_on_fallthrough_count += 1;
        }
    } else if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
    } else {
        state.non_cooling_skip_count += 1;
    }

    let route = if predecessor.unit_off_skipped {
        PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::NonCooling
    } else if !heating_body {
        PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::HeatingAvailabilityOff
    } else if !humid_body {
        PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::HumidificationControlInactive
    } else if !controls_admitted {
        PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::DehumidificationControlRejected
    } else if delta_above != Some(true) {
        PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::DeltaHumidityRatioFallthrough
    } else if !flow_body {
        PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::MoistureDemandFallthrough
    } else {
        PurchasedAirCalcCoolingHumidificationFlowRetainedRoute::CandidateAssigned
    };

    let snapshot = PurchasedAirCalcCoolingHumidificationFlowSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        supply_mass_flow_rate_for_humidification_reset_assigned: cooling,
        reset_supply_mass_flow_rate_for_humidification_kg_per_s: reset,
        heating_on_read: cooling,
        heating_on,
        heating_on_body_entered: heating_body,
        humidification_control_type_read: heating_body,
        humidification_control_type: humid_control,
        humidification_control_type_humidistat: humidistat,
        humidification_control_body_entered: humid_body,
        dehumidification_control_type_first_read: humid_body,
        first_dehumidification_control_type: first_dehumid_control,
        dehumidification_control_type_humidistat: dehumid_is_humidistat,
        dehumidification_control_type_second_read: dehumid_is_humidistat == Some(false),
        second_dehumidification_control_type: second_dehumid_control,
        dehumidification_control_type_none: dehumid_is_none,
        humidification_control_condition_admitted: controls_admitted,
        zone_humidifying_setpoint_moisture_demand_read: controls_admitted,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: demand,
        zone_humidifying_setpoint_moisture_demand_assigned: controls_admitted,
        assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: demand,
        maximum_heating_supply_air_humidity_ratio_read: controls_admitted,
        maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air: max_supply,
        zone_humidity_ratio_read: controls_admitted,
        zone_humidity_ratio_kg_water_per_kg_dry_air: zone,
        delta_humidity_ratio_calculated: controls_admitted,
        delta_humidity_ratio_kg_water_per_kg_dry_air: delta,
        delta_humidity_ratio_assigned: controls_admitted,
        assigned_delta_humidity_ratio_kg_water_per_kg_dry_air: delta,
        delta_humidity_ratio_for_gate_read: controls_admitted,
        delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air: delta_gate,
        delta_humidity_ratio_comparison_evaluated: controls_admitted,
        delta_humidity_ratio_above_small_delta: delta_above,
        zone_humidifying_setpoint_moisture_demand_for_gate_read: delta_above == Some(true),
        zone_humidifying_setpoint_moisture_demand_for_gate_kg_per_s: demand_gate,
        zone_humidifying_setpoint_moisture_demand_comparison_evaluated: delta_above == Some(true),
        zone_humidifying_setpoint_moisture_demand_above_zero: demand_above,
        humidification_flow_body_entered: flow_body,
        zone_humidifying_setpoint_moisture_demand_for_division_read: flow_body,
        zone_humidifying_setpoint_moisture_demand_for_division_kg_per_s: division_demand,
        delta_humidity_ratio_for_division_read: flow_body,
        delta_humidity_ratio_for_division_kg_water_per_kg_dry_air: division_delta,
        supply_mass_flow_rate_for_humidification_calculated: flow_body,
        calculated_supply_mass_flow_rate_for_humidification_kg_per_s: calculated,
        supply_mass_flow_rate_for_humidification_assigned: flow_body,
        assigned_supply_mass_flow_rate_for_humidification_kg_per_s: calculated,
        resulting_supply_mass_flow_rate_for_humidification_kg_per_s: resulting,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
