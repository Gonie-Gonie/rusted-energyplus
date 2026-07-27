//! Pure CP318-to-CP319 source-characterization transition.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::PurchasedAirCalcCoolingSensibleFlowSnapshot;

use super::{
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingDehumidificationFlowInput,
    PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute,
    PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
};

pub(in crate::ideal_loads::calc) fn advance_cooling_dehumidification_flow_state(
    state: &mut PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
    predecessor: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    input: PurchasedAirCalcCoolingDehumidificationFlowInput,
) -> PurchasedAirCalcCoolingDehumidificationFlowSnapshot {
    let unit_off_skipped = predecessor.unit_off_skipped;
    let non_cooling_skipped = predecessor.non_cooling_skipped;
    let cooling_body_entered = predecessor.cooling_body_entered;
    let reset_supply_mass_flow_rate_for_dehumidification_kg_per_s =
        cooling_body_entered.then_some(0.0_f64);
    let cooling_on = if cooling_body_entered {
        Some(input.cooling_on)
    } else {
        None
    };
    let cooling_on_body_entered = cooling_on == Some(true);

    let dehumidification_control_type = if cooling_on_body_entered {
        Some(input.dehumidification_control_type)
    } else {
        None
    };
    let dehumidification_control_type_humidistat = dehumidification_control_type
        .map(|control| control == DehumidificationControlType::Humidistat);
    let dehumidification_control_body_entered =
        dehumidification_control_type_humidistat == Some(true);

    let zone_dehumidifying_setpoint_moisture_demand_kg_per_s =
        if dehumidification_control_body_entered {
            Some(input.zone_dehumidifying_setpoint_moisture_demand_kg_per_s)
        } else {
            None
        };
    let assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s =
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s;
    let minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air =
        if dehumidification_control_body_entered {
            Some(input.minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air)
        } else {
            None
        };
    let zone_humidity_ratio_kg_water_per_kg_dry_air = if dehumidification_control_body_entered {
        Some(input.zone_humidity_ratio_kg_water_per_kg_dry_air)
    } else {
        None
    };
    let delta_humidity_ratio_kg_water_per_kg_dry_air =
        minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air
            .zip(zone_humidity_ratio_kg_water_per_kg_dry_air)
            .map(|(minimum_supply, zone)| minimum_supply - zone);
    let assigned_delta_humidity_ratio_kg_water_per_kg_dry_air =
        delta_humidity_ratio_kg_water_per_kg_dry_air;
    let delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air =
        assigned_delta_humidity_ratio_kg_water_per_kg_dry_air;
    let delta_humidity_ratio_below_negative_small_delta =
        delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air.map(|delta| {
            delta < -PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO
        });
    let delta_humidity_ratio_comparison_satisfied =
        delta_humidity_ratio_below_negative_small_delta == Some(true);

    let zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s =
        if delta_humidity_ratio_comparison_satisfied {
            assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
        } else {
            None
        };
    let zone_dehumidifying_setpoint_moisture_demand_below_zero =
        zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s.map(|demand| demand < 0.0);
    let dehumidification_flow_body_entered =
        zone_dehumidifying_setpoint_moisture_demand_below_zero == Some(true);

    let zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s =
        if dehumidification_flow_body_entered {
            assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
        } else {
            None
        };
    let delta_humidity_ratio_for_division_kg_water_per_kg_dry_air =
        if dehumidification_flow_body_entered {
            assigned_delta_humidity_ratio_kg_water_per_kg_dry_air
        } else {
            None
        };
    let calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s =
        zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s
            .zip(delta_humidity_ratio_for_division_kg_water_per_kg_dry_air)
            .map(|(demand, delta)| demand / delta);
    let assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s =
        calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s;
    let resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s = if cooling_body_entered {
        Some(
            assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s.unwrap_or(
                reset_supply_mass_flow_rate_for_dehumidification_kg_per_s.unwrap_or(0.0),
            ),
        )
    } else {
        None
    };

    state.transition_count += 1;
    if cooling_body_entered {
        state.cooling_body_entry_count += 1;
        state.supply_mass_flow_rate_for_dehumidification_reset_assignment_count += 1;
        state.cooling_on_read_count += 1;
        if cooling_on_body_entered {
            state.cooling_on_body_entry_count += 1;
            state.dehumidification_control_type_read_count += 1;
            if dehumidification_control_body_entered {
                state.dehumidification_control_type_humidistat_count += 1;
                state.dehumidification_control_body_entry_count += 1;
                state.zone_dehumidifying_setpoint_moisture_demand_read_count += 1;
                state.zone_dehumidifying_setpoint_moisture_demand_assignment_count += 1;
                state.minimum_cooling_supply_air_humidity_ratio_read_count += 1;
                state.zone_humidity_ratio_read_count += 1;
                state.delta_humidity_ratio_calculation_count += 1;
                state.delta_humidity_ratio_assignment_count += 1;
                state.delta_humidity_ratio_for_gate_read_count += 1;
                state.delta_humidity_ratio_comparison_count += 1;
                if delta_humidity_ratio_comparison_satisfied {
                    state.delta_humidity_ratio_comparison_satisfied_count += 1;
                    state.zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count += 1;
                    state.zone_dehumidifying_setpoint_moisture_demand_comparison_count += 1;
                    if dehumidification_flow_body_entered {
                        state
                            .zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count +=
                            1;
                        state.dehumidification_flow_body_entry_count += 1;
                        state
                            .zone_dehumidifying_setpoint_moisture_demand_for_division_read_count +=
                            1;
                        state.delta_humidity_ratio_for_division_read_count += 1;
                        state.supply_mass_flow_rate_for_dehumidification_calculation_count += 1;
                        state.supply_mass_flow_rate_for_dehumidification_assignment_count += 1;
                    } else {
                        state.zone_dehumidifying_setpoint_moisture_demand_fallthrough_count += 1;
                    }
                } else {
                    state.delta_humidity_ratio_fallthrough_count += 1;
                }
            } else {
                state.dehumidification_control_type_fallthrough_count += 1;
            }
        } else {
            state.cooling_on_fallthrough_count += 1;
        }
    } else if unit_off_skipped {
        state.unit_off_skip_count += 1;
    } else if non_cooling_skipped {
        state.non_cooling_skip_count += 1;
    }

    let retained_route = if unit_off_skipped {
        PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::UnitOff
    } else if non_cooling_skipped {
        PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::NonCooling
    } else if !cooling_on_body_entered {
        PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::CoolingAvailabilityOff
    } else if !dehumidification_control_body_entered {
        PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::DehumidificationControlInactive
    } else if !delta_humidity_ratio_comparison_satisfied {
        PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::DeltaHumidityRatioFallthrough
    } else if !dehumidification_flow_body_entered {
        PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::MoistureDemandFallthrough
    } else {
        PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute::CandidateAssigned
    };

    let snapshot = PurchasedAirCalcCoolingDehumidificationFlowSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_cooling_on_body_entered: predecessor.cooling_on_body_entered,
        predecessor_delta_temperature_body_entered: predecessor.delta_temperature_body_entered,
        predecessor_supply_mass_flow_rate_for_cool_assigned: predecessor
            .supply_mass_flow_rate_for_cool_assigned,
        unit_off_skipped,
        non_cooling_skipped,
        cooling_body_entered,
        supply_mass_flow_rate_for_dehumidification_reset_assigned: cooling_body_entered,
        reset_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        cooling_on_read: cooling_body_entered,
        cooling_on,
        cooling_on_body_entered,
        dehumidification_control_type_read: cooling_on_body_entered,
        dehumidification_control_type,
        dehumidification_control_type_humidistat,
        dehumidification_control_body_entered,
        zone_dehumidifying_setpoint_moisture_demand_read: dehumidification_control_body_entered,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        zone_dehumidifying_setpoint_moisture_demand_assigned: dehumidification_control_body_entered,
        assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        minimum_cooling_supply_air_humidity_ratio_read: dehumidification_control_body_entered,
        minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air,
        zone_humidity_ratio_read: dehumidification_control_body_entered,
        zone_humidity_ratio_kg_water_per_kg_dry_air,
        delta_humidity_ratio_calculated: dehumidification_control_body_entered,
        delta_humidity_ratio_kg_water_per_kg_dry_air,
        delta_humidity_ratio_assigned: dehumidification_control_body_entered,
        assigned_delta_humidity_ratio_kg_water_per_kg_dry_air,
        delta_humidity_ratio_for_gate_read: dehumidification_control_body_entered,
        delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air,
        delta_humidity_ratio_comparison_evaluated: dehumidification_control_body_entered,
        delta_humidity_ratio_below_negative_small_delta,
        zone_dehumidifying_setpoint_moisture_demand_for_gate_read:
            delta_humidity_ratio_comparison_satisfied,
        zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s,
        zone_dehumidifying_setpoint_moisture_demand_comparison_evaluated:
            delta_humidity_ratio_comparison_satisfied,
        zone_dehumidifying_setpoint_moisture_demand_below_zero,
        dehumidification_flow_body_entered,
        zone_dehumidifying_setpoint_moisture_demand_for_division_read:
            dehumidification_flow_body_entered,
        zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s,
        delta_humidity_ratio_for_division_read: dehumidification_flow_body_entered,
        delta_humidity_ratio_for_division_kg_water_per_kg_dry_air,
        supply_mass_flow_rate_for_dehumidification_calculated: dehumidification_flow_body_entered,
        calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        supply_mass_flow_rate_for_dehumidification_assigned: dehumidification_flow_body_entered,
        assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(retained_route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
