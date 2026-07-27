//! Pure CP317 cooling economizer true-body transition.

use ep_model::IdealLoadsLimit;

use super::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SMALL_TEMP_DIFF_C,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerBodyInput, PurchasedAirCalcCoolingEconomizerBodyRetainedRoute,
    PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingEconomizerConditionSnapshot;
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(in crate::ideal_loads::calc) fn advance_cooling_economizer_body_state(
    state: &mut PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    input: PurchasedAirCalcCoolingEconomizerBodyInput,
) -> PurchasedAirCalcCoolingEconomizerBodySnapshot {
    state.transition_count += 1;
    let economizer_calculation_body_executed = predecessor.economizer_calculation_body_entered;
    let unit_off_skipped = !economizer_calculation_body_executed && predecessor.unit_off_skipped;
    let non_cooling_skipped =
        !economizer_calculation_body_executed && predecessor.non_cooling_skipped;
    let maximum_cooling_flow_body_sibling_skipped = !economizer_calculation_body_executed
        && predecessor.maximum_cooling_flow_body_sibling_skipped;
    let no_economizer_outer_guard_fallthrough_skipped = !economizer_calculation_body_executed
        && predecessor.no_economizer_outer_guard_fallthrough_skipped;
    let economizer_condition_fallthrough_skipped =
        !economizer_calculation_body_executed && predecessor.economizer_condition_fallthrough;
    let retained_route = if economizer_calculation_body_executed {
        PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::Executed
    } else if unit_off_skipped {
        PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::UnitOff
    } else if non_cooling_skipped {
        PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::NonCooling
    } else if maximum_cooling_flow_body_sibling_skipped {
        PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::MaximumCoolingFlowBodySibling
    } else if no_economizer_outer_guard_fallthrough_skipped {
        PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::NoEconomizerOuterGuardFallthrough
    } else {
        PurchasedAirCalcCoolingEconomizerBodyRetainedRoute::EconomizerConditionFallthrough
    };

    let zone_humidity_ratio = if economizer_calculation_body_executed {
        Some(input.zone_humidity_ratio)
    } else {
        None
    };
    // PsyCp source static cache identity/sentinel is outside this pure scalar characterization.
    let psychrometric_cp_air_result_j_per_kg_k =
        zone_humidity_ratio.map(energyplus_psy_cp_air_fn_w);
    let cp_air_j_per_kg_k = psychrometric_cp_air_result_j_per_kg_k;
    let outdoor_air_temperature_c = if economizer_calculation_body_executed {
        Some(input.outdoor_air_temperature_c)
    } else {
        None
    };
    let zone_temperature_c = if economizer_calculation_body_executed {
        Some(input.zone_temperature_c)
    } else {
        None
    };
    let delta_temperature_c = outdoor_air_temperature_c
        .zip(zone_temperature_c)
        .map(|(outdoor_air, zone)| outdoor_air - zone);
    let assigned_delta_temperature_c = delta_temperature_c;
    let delta_temperature_for_gate_c = assigned_delta_temperature_c;
    let delta_temperature_below_negative_small_temp_diff = delta_temperature_for_gate_c
        .map(|delta| delta < -PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SMALL_TEMP_DIFF_C);
    let delta_temperature_condition_satisfied =
        delta_temperature_below_negative_small_temp_diff == Some(true);

    let zone_cooling_setpoint_load_w = if delta_temperature_condition_satisfied {
        Some(input.zone_cooling_setpoint_load_w)
    } else {
        None
    };
    let cp_air_for_first_division_j_per_kg_k = if delta_temperature_condition_satisfied {
        cp_air_j_per_kg_k
    } else {
        None
    };
    let zone_cooling_setpoint_load_over_cp_air_kg_k_per_s = zone_cooling_setpoint_load_w
        .zip(cp_air_for_first_division_j_per_kg_k)
        .map(|(load, cp_air)| load / cp_air);
    let delta_temperature_for_second_division_c = if delta_temperature_condition_satisfied {
        assigned_delta_temperature_c
    } else {
        None
    };
    let calculated_supply_mass_flow_rate_kg_per_s =
        zone_cooling_setpoint_load_over_cp_air_kg_k_per_s
            .zip(delta_temperature_for_second_division_c)
            .map(|(load_over_cp, delta_temperature)| load_over_cp / delta_temperature);
    let initial_supply_mass_flow_rate_kg_per_s = calculated_supply_mass_flow_rate_kg_per_s;

    let cooling_limit_flow_rate_value = if delta_temperature_condition_satisfied {
        Some(input.cooling_limit)
    } else {
        None
    };
    let cooling_limit_flow_rate_comparison_satisfied =
        cooling_limit_flow_rate_value.map(|limit| limit == IdealLoadsLimit::LimitFlowRate);
    let cooling_limit_flow_rate_and_capacity_comparison_evaluated =
        cooling_limit_flow_rate_comparison_satisfied == Some(false);
    let cooling_limit_flow_rate_and_capacity_value =
        if cooling_limit_flow_rate_and_capacity_comparison_evaluated {
            Some(input.cooling_limit)
        } else {
            None
        };
    let cooling_limit_flow_rate_and_capacity_comparison_satisfied =
        cooling_limit_flow_rate_and_capacity_value
            .map(|limit| limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    let cooling_flow_limit_active = delta_temperature_condition_satisfied.then_some(
        cooling_limit_flow_rate_comparison_satisfied == Some(true)
            || cooling_limit_flow_rate_and_capacity_comparison_satisfied == Some(true),
    );

    let maximum_cooling_air_mass_flow_rate_kg_per_s = if cooling_flow_limit_active == Some(true) {
        Some(input.maximum_cooling_air_mass_flow_rate_kg_per_s)
    } else {
        None
    };
    let maximum_cooling_air_mass_flow_rate_positive =
        maximum_cooling_air_mass_flow_rate_kg_per_s.map(|maximum| maximum > 0.0);
    let maximum_flow_clamp_body_entered = maximum_cooling_air_mass_flow_rate_positive == Some(true);
    let supply_mass_flow_rate_clamped = maximum_flow_clamp_body_entered;
    let (
        supply_mass_flow_rate_for_clamp_kg_per_s,
        nonnegative_supply_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s,
        clamped_supply_mass_flow_rate_kg_per_s,
    ) = match (
        supply_mass_flow_rate_clamped,
        initial_supply_mass_flow_rate_kg_per_s,
    ) {
        (true, Some(supply_for_clamp)) => {
            let nonnegative = source_max(supply_for_clamp, 0.0);
            let clamp_upper_bound = input.maximum_cooling_air_mass_flow_rate_kg_per_s;
            let clamped = source_min(nonnegative, clamp_upper_bound);
            (
                Some(supply_for_clamp),
                Some(nonnegative),
                Some(clamp_upper_bound),
                Some(clamped),
            )
        }
        _ => (None, None, None, None),
    };
    let current_supply_mass_flow_rate_kg_per_s = initial_supply_mass_flow_rate_kg_per_s
        .map(|initial| clamped_supply_mass_flow_rate_kg_per_s.unwrap_or(initial));

    let resulting_supply_mass_flow_rate_kg_per_s = if delta_temperature_condition_satisfied {
        current_supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let outdoor_air_mass_flow_rate_kg_per_s = if delta_temperature_condition_satisfied {
        Some(input.outdoor_air_mass_flow_rate_kg_per_s)
    } else {
        None
    };
    let supply_mass_flow_above_outdoor_air_mass_flow = resulting_supply_mass_flow_rate_kg_per_s
        .zip(outdoor_air_mass_flow_rate_kg_per_s)
        .map(|(supply, outdoor_air)| supply > outdoor_air);
    let economizer_activation_body_entered =
        supply_mass_flow_above_outdoor_air_mass_flow == Some(true);
    let economizer_assignments_executed = economizer_activation_body_entered;
    let economizer_on = economizer_assignments_executed.then_some(true);
    let supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s =
        if economizer_assignments_executed {
            resulting_supply_mass_flow_rate_kg_per_s
        } else {
            None
        };
    let assigned_outdoor_air_mass_flow_rate_kg_per_s =
        supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s;
    let system_time_step_hours = if economizer_assignments_executed {
        Some(input.system_time_step_hours)
    } else {
        None
    };
    let assigned_economizer_active_time_hours = system_time_step_hours;

    if economizer_calculation_body_executed {
        state.body_execution_count += 1;
        state.zone_humidity_ratio_read_count += 1;
        state.psychrometric_cp_air_evaluation_count += 1;
        state.cp_air_assignment_count += 1;
        state.outdoor_air_temperature_read_count += 1;
        state.zone_temperature_read_count += 1;
        state.delta_temperature_calculation_count += 1;
        state.delta_temperature_assignment_count += 1;
        state.delta_temperature_for_gate_read_count += 1;
        state.delta_temperature_comparison_count += 1;
        if delta_temperature_condition_satisfied {
            state.delta_temperature_comparison_satisfied_count += 1;
            state.delta_temperature_body_entry_count += 1;
            state.zone_cooling_setpoint_load_read_count += 1;
            state.cp_air_for_first_division_read_count += 1;
            state.zone_cooling_setpoint_load_over_cp_air_calculation_count += 1;
            state.delta_temperature_for_second_division_read_count += 1;
            state.supply_mass_flow_rate_calculation_count += 1;
            state.initial_supply_mass_flow_rate_assignment_count += 1;
            state.cooling_limit_flow_rate_read_count += 1;
            state.cooling_limit_flow_rate_comparison_count += 1;
            if cooling_limit_flow_rate_comparison_satisfied == Some(true) {
                state.cooling_limit_flow_rate_match_count += 1;
            }
            if cooling_limit_flow_rate_and_capacity_comparison_evaluated {
                state.cooling_limit_flow_rate_and_capacity_read_count += 1;
                state.cooling_limit_flow_rate_and_capacity_comparison_count += 1;
            }
            if cooling_limit_flow_rate_and_capacity_comparison_satisfied == Some(true) {
                state.cooling_limit_flow_rate_and_capacity_match_count += 1;
            }
            if cooling_flow_limit_active == Some(true) {
                state.maximum_cooling_air_mass_flow_rate_read_count += 1;
                state.maximum_cooling_air_mass_flow_rate_positive_comparison_count += 1;
            }
            if supply_mass_flow_rate_clamped {
                state.maximum_cooling_air_mass_flow_rate_positive_count += 1;
                state.maximum_flow_clamp_body_entry_count += 1;
                state.supply_mass_flow_rate_for_clamp_read_count += 1;
                state.inner_max_evaluation_count += 1;
                state.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count += 1;
                state.outer_min_evaluation_count += 1;
                state.supply_mass_flow_rate_clamp_count += 1;
                state.clamped_supply_mass_flow_rate_assignment_count += 1;
            }
            state.resulting_supply_mass_flow_rate_read_count += 1;
            state.outdoor_air_mass_flow_rate_read_count += 1;
            state.supply_above_outdoor_air_mass_flow_comparison_count += 1;
            if economizer_assignments_executed {
                state.supply_above_outdoor_air_mass_flow_comparison_satisfied_count += 1;
                state.economizer_activation_body_entry_count += 1;
                state.economizer_on_assignment_count += 1;
                state.supply_mass_flow_rate_for_outdoor_air_assignment_read_count += 1;
                state.outdoor_air_mass_flow_rate_assignment_count += 1;
                state.system_time_step_read_count += 1;
                state.economizer_active_time_assignment_count += 1;
            } else {
                state.outdoor_air_mass_flow_comparison_fallthrough_count += 1;
            }
        } else {
            state.delta_temperature_fallthrough_count += 1;
        }
    } else if unit_off_skipped {
        state.unit_off_skip_count += 1;
    } else if non_cooling_skipped {
        state.non_cooling_skip_count += 1;
    } else if maximum_cooling_flow_body_sibling_skipped {
        state.maximum_cooling_flow_body_sibling_skip_count += 1;
    } else if no_economizer_outer_guard_fallthrough_skipped {
        state.no_economizer_outer_guard_fallthrough_skip_count += 1;
    } else if economizer_condition_fallthrough_skipped {
        state.economizer_condition_fallthrough_skip_count += 1;
    }

    let snapshot = PurchasedAirCalcCoolingEconomizerBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_maximum_cooling_flow_body_entered: predecessor
            .predecessor_maximum_cooling_flow_body_entered,
        predecessor_active_guard_false_economizer_fallthrough: predecessor
            .predecessor_active_guard_false_economizer_fallthrough,
        predecessor_economizer_guard_evaluated: predecessor.predecessor_economizer_guard_evaluated,
        predecessor_economizer_body_entered: predecessor.predecessor_economizer_body_entered,
        predecessor_no_economizer_fallthrough: predecessor.predecessor_no_economizer_fallthrough,
        predecessor_economizer_condition_evaluated: predecessor.economizer_condition_evaluated,
        predecessor_economizer_condition_satisfied: predecessor.economizer_condition_satisfied,
        predecessor_economizer_calculation_body_entered: predecessor
            .economizer_calculation_body_entered,
        unit_off_skipped,
        non_cooling_skipped,
        maximum_cooling_flow_body_sibling_skipped,
        no_economizer_outer_guard_fallthrough_skipped,
        economizer_condition_fallthrough_skipped,
        economizer_calculation_body_executed,
        zone_humidity_ratio_read: economizer_calculation_body_executed,
        zone_humidity_ratio,
        psychrometric_cp_air_evaluated: economizer_calculation_body_executed,
        psychrometric_cp_air_result_j_per_kg_k,
        cp_air_assigned: economizer_calculation_body_executed,
        cp_air_j_per_kg_k,
        outdoor_air_temperature_read: economizer_calculation_body_executed,
        outdoor_air_temperature_c,
        zone_temperature_read: economizer_calculation_body_executed,
        zone_temperature_c,
        delta_temperature_calculated: economizer_calculation_body_executed,
        delta_temperature_c,
        delta_temperature_assigned: economizer_calculation_body_executed,
        assigned_delta_temperature_c,
        delta_temperature_for_gate_read: economizer_calculation_body_executed,
        delta_temperature_for_gate_c,
        delta_temperature_comparison_evaluated: economizer_calculation_body_executed,
        delta_temperature_below_negative_small_temp_diff,
        delta_temperature_body_entered: delta_temperature_condition_satisfied,
        zone_cooling_setpoint_load_read: delta_temperature_condition_satisfied,
        zone_cooling_setpoint_load_w,
        cp_air_for_first_division_read: delta_temperature_condition_satisfied,
        cp_air_for_first_division_j_per_kg_k,
        zone_cooling_setpoint_load_over_cp_air_calculated: delta_temperature_condition_satisfied,
        zone_cooling_setpoint_load_over_cp_air_kg_k_per_s,
        delta_temperature_for_second_division_read: delta_temperature_condition_satisfied,
        delta_temperature_for_second_division_c,
        supply_mass_flow_rate_calculated: delta_temperature_condition_satisfied,
        calculated_supply_mass_flow_rate_kg_per_s,
        initial_supply_mass_flow_rate_assigned: delta_temperature_condition_satisfied,
        initial_supply_mass_flow_rate_kg_per_s,
        cooling_limit_flow_rate_comparison_evaluated: delta_temperature_condition_satisfied,
        cooling_limit_flow_rate_read: delta_temperature_condition_satisfied,
        cooling_limit_flow_rate_value,
        cooling_limit_flow_rate_comparison_satisfied,
        cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        cooling_limit_flow_rate_and_capacity_read:
            cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        cooling_limit_flow_rate_and_capacity_value,
        cooling_limit_flow_rate_and_capacity_comparison_satisfied,
        cooling_flow_limit_active,
        maximum_cooling_air_mass_flow_rate_read: cooling_flow_limit_active == Some(true),
        maximum_cooling_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: cooling_flow_limit_active
            == Some(true),
        maximum_cooling_air_mass_flow_rate_positive,
        maximum_flow_clamp_body_entered,
        supply_mass_flow_rate_clamped,
        supply_mass_flow_rate_for_clamp_read: supply_mass_flow_rate_clamped,
        supply_mass_flow_rate_for_clamp_kg_per_s,
        inner_max_evaluated: supply_mass_flow_rate_clamped,
        nonnegative_supply_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read: supply_mass_flow_rate_clamped,
        maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s,
        outer_min_evaluated: supply_mass_flow_rate_clamped,
        clamped_supply_mass_flow_rate_kg_per_s,
        clamped_supply_mass_flow_rate_assigned: supply_mass_flow_rate_clamped,
        resulting_supply_mass_flow_rate_kg_per_s,
        resulting_supply_mass_flow_rate_read: delta_temperature_condition_satisfied,
        outdoor_air_mass_flow_rate_read: delta_temperature_condition_satisfied,
        outdoor_air_mass_flow_rate_kg_per_s,
        supply_above_outdoor_air_mass_flow_comparison_evaluated:
            delta_temperature_condition_satisfied,
        supply_mass_flow_above_outdoor_air_mass_flow,
        economizer_activation_body_entered,
        economizer_on_assigned: economizer_assignments_executed,
        economizer_on,
        supply_mass_flow_rate_for_outdoor_air_assignment_read: economizer_assignments_executed,
        supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s,
        outdoor_air_mass_flow_rate_assigned: economizer_assignments_executed,
        assigned_outdoor_air_mass_flow_rate_kg_per_s,
        system_time_step_read: economizer_assignments_executed,
        system_time_step_hours,
        economizer_active_time_assigned: economizer_assignments_executed,
        assigned_economizer_active_time_hours,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(retained_route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}

#[inline]
fn source_max(left: f64, right: f64) -> f64 {
    if left < right { right } else { left }
}

#[inline]
fn source_min(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
}
