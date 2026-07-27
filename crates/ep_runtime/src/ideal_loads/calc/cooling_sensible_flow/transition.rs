//! Pure CP317-to-CP318 source-characterization transition.

use crate::ideal_loads::PurchasedAirCalcCoolingEconomizerBodySnapshot;
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

use super::{
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingSensibleFlowInput, PurchasedAirCalcCoolingSensibleFlowRetainedRoute,
    PurchasedAirCalcCoolingSensibleFlowRuntimeState, PurchasedAirCalcCoolingSensibleFlowSnapshot,
};

pub(in crate::ideal_loads::calc) fn advance_cooling_sensible_flow_state(
    state: &mut PurchasedAirCalcCoolingSensibleFlowRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    input: PurchasedAirCalcCoolingSensibleFlowInput,
) -> PurchasedAirCalcCoolingSensibleFlowSnapshot {
    let unit_off_skipped = predecessor.unit_off_skipped;
    let non_cooling_skipped = predecessor.non_cooling_skipped;
    let cooling_body_entered = predecessor.predecessor_cooling_body_entered;
    let reset_supply_mass_flow_rate_for_cool_kg_per_s = cooling_body_entered.then_some(0.0_f64);
    let cooling_on = if cooling_body_entered {
        Some(input.cooling_on)
    } else {
        None
    };
    let cooling_on_body_entered = cooling_on == Some(true);

    let zone_humidity_ratio = if cooling_on_body_entered {
        Some(input.zone_humidity_ratio)
    } else {
        None
    };
    let psychrometric_cp_air_result_j_per_kg_k =
        zone_humidity_ratio.map(energyplus_psy_cp_air_fn_w);
    let cp_air_j_per_kg_k = psychrometric_cp_air_result_j_per_kg_k;
    let minimum_cooling_supply_air_temperature_c = if cooling_on_body_entered {
        Some(input.minimum_cooling_supply_air_temperature_c)
    } else {
        None
    };
    let zone_temperature_c = if cooling_on_body_entered {
        Some(input.zone_temperature_c)
    } else {
        None
    };
    let delta_temperature_c = minimum_cooling_supply_air_temperature_c
        .zip(zone_temperature_c)
        .map(|(minimum_supply, zone)| minimum_supply - zone);
    let assigned_delta_temperature_c = delta_temperature_c;
    let delta_temperature_for_gate_c = assigned_delta_temperature_c;
    let delta_temperature_below_negative_small_temp_diff = delta_temperature_for_gate_c
        .map(|delta| delta < -PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C);
    let delta_temperature_body_entered =
        delta_temperature_below_negative_small_temp_diff == Some(true);

    let zone_cooling_setpoint_load_w = if delta_temperature_body_entered {
        Some(input.zone_cooling_setpoint_load_w)
    } else {
        None
    };
    let cp_air_for_first_division_j_per_kg_k = if delta_temperature_body_entered {
        cp_air_j_per_kg_k
    } else {
        None
    };
    let zone_cooling_setpoint_load_over_cp_air_kg_k_per_s = zone_cooling_setpoint_load_w
        .zip(cp_air_for_first_division_j_per_kg_k)
        .map(|(load, cp_air)| load / cp_air);
    let delta_temperature_for_second_division_c = if delta_temperature_body_entered {
        assigned_delta_temperature_c
    } else {
        None
    };
    let calculated_supply_mass_flow_rate_for_cool_kg_per_s =
        zone_cooling_setpoint_load_over_cp_air_kg_k_per_s
            .zip(delta_temperature_for_second_division_c)
            .map(|(first_division, delta)| first_division / delta);
    let assigned_supply_mass_flow_rate_for_cool_kg_per_s =
        calculated_supply_mass_flow_rate_for_cool_kg_per_s;
    let resulting_supply_mass_flow_rate_for_cool_kg_per_s = if cooling_body_entered {
        Some(
            assigned_supply_mass_flow_rate_for_cool_kg_per_s
                .unwrap_or(reset_supply_mass_flow_rate_for_cool_kg_per_s.unwrap_or(0.0)),
        )
    } else {
        None
    };

    state.transition_count += 1;
    if cooling_body_entered {
        state.cooling_body_entry_count += 1;
        state.supply_mass_flow_rate_for_cool_reset_assignment_count += 1;
        state.cooling_on_read_count += 1;
        if cooling_on_body_entered {
            state.cooling_on_body_entry_count += 1;
            state.zone_humidity_ratio_read_count += 1;
            state.psychrometric_cp_air_evaluation_count += 1;
            state.cp_air_assignment_count += 1;
            state.minimum_cooling_supply_air_temperature_read_count += 1;
            state.zone_temperature_read_count += 1;
            state.delta_temperature_calculation_count += 1;
            state.delta_temperature_assignment_count += 1;
            state.delta_temperature_for_gate_read_count += 1;
            state.delta_temperature_comparison_count += 1;
            if delta_temperature_body_entered {
                state.delta_temperature_comparison_satisfied_count += 1;
                state.delta_temperature_body_entry_count += 1;
                state.zone_cooling_setpoint_load_read_count += 1;
                state.cp_air_for_first_division_read_count += 1;
                state.zone_cooling_setpoint_load_over_cp_air_calculation_count += 1;
                state.delta_temperature_for_second_division_read_count += 1;
                state.supply_mass_flow_rate_for_cool_calculation_count += 1;
                state.supply_mass_flow_rate_for_cool_assignment_count += 1;
            } else {
                state.delta_temperature_fallthrough_count += 1;
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
        PurchasedAirCalcCoolingSensibleFlowRetainedRoute::UnitOff
    } else if non_cooling_skipped {
        PurchasedAirCalcCoolingSensibleFlowRetainedRoute::NonCooling
    } else if !cooling_on_body_entered {
        PurchasedAirCalcCoolingSensibleFlowRetainedRoute::CoolingAvailabilityOff
    } else if !delta_temperature_body_entered {
        PurchasedAirCalcCoolingSensibleFlowRetainedRoute::DeltaTemperatureFallthrough
    } else {
        PurchasedAirCalcCoolingSensibleFlowRetainedRoute::CandidateAssigned
    };

    let snapshot = PurchasedAirCalcCoolingSensibleFlowSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_maximum_cooling_flow_body_sibling_skipped: predecessor
            .maximum_cooling_flow_body_sibling_skipped,
        predecessor_no_economizer_outer_guard_fallthrough_skipped: predecessor
            .no_economizer_outer_guard_fallthrough_skipped,
        predecessor_economizer_condition_fallthrough_skipped: predecessor
            .economizer_condition_fallthrough_skipped,
        predecessor_economizer_calculation_body_executed: predecessor
            .economizer_calculation_body_executed,
        unit_off_skipped,
        non_cooling_skipped,
        cooling_body_entered,
        supply_mass_flow_rate_for_cool_reset_assigned: cooling_body_entered,
        reset_supply_mass_flow_rate_for_cool_kg_per_s,
        cooling_on_read: cooling_body_entered,
        cooling_on,
        cooling_on_body_entered,
        zone_humidity_ratio_read: cooling_on_body_entered,
        zone_humidity_ratio,
        psychrometric_cp_air_evaluated: cooling_on_body_entered,
        psychrometric_cp_air_result_j_per_kg_k,
        cp_air_assigned: cooling_on_body_entered,
        cp_air_j_per_kg_k,
        minimum_cooling_supply_air_temperature_read: cooling_on_body_entered,
        minimum_cooling_supply_air_temperature_c,
        zone_temperature_read: cooling_on_body_entered,
        zone_temperature_c,
        delta_temperature_calculated: cooling_on_body_entered,
        delta_temperature_c,
        delta_temperature_assigned: cooling_on_body_entered,
        assigned_delta_temperature_c,
        delta_temperature_for_gate_read: cooling_on_body_entered,
        delta_temperature_for_gate_c,
        delta_temperature_comparison_evaluated: cooling_on_body_entered,
        delta_temperature_below_negative_small_temp_diff,
        delta_temperature_body_entered,
        zone_cooling_setpoint_load_read: delta_temperature_body_entered,
        zone_cooling_setpoint_load_w,
        cp_air_for_first_division_read: delta_temperature_body_entered,
        cp_air_for_first_division_j_per_kg_k,
        zone_cooling_setpoint_load_over_cp_air_calculated: delta_temperature_body_entered,
        zone_cooling_setpoint_load_over_cp_air_kg_k_per_s,
        delta_temperature_for_second_division_read: delta_temperature_body_entered,
        delta_temperature_for_second_division_c,
        supply_mass_flow_rate_for_cool_calculated: delta_temperature_body_entered,
        calculated_supply_mass_flow_rate_for_cool_kg_per_s,
        supply_mass_flow_rate_for_cool_assigned: delta_temperature_body_entered,
        assigned_supply_mass_flow_rate_for_cool_kg_per_s,
        resulting_supply_mass_flow_rate_for_cool_kg_per_s,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(retained_route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
