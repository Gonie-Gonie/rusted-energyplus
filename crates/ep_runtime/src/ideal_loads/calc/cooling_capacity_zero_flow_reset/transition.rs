//! Pure CP320-to-CP321 source-characterization transition.

use ep_model::IdealLoadsLimit;

use super::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetInput,
    PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute,
    PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingHumidificationFlowSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_capacity_zero_flow_reset_state(
    state: &mut PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    predecessor: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    input: PurchasedAirCalcCoolingCapacityZeroFlowResetInput,
) -> PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let first_limit = if cooling {
        Some(input.cooling_limit)
    } else {
        None
    };
    let is_capacity = first_limit.map(|limit| limit == IdealLoadsLimit::LimitCapacity);
    let second_limit = if is_capacity == Some(false) {
        Some(input.cooling_limit)
    } else {
        None
    };
    let is_flow_and_capacity =
        second_limit.map(|limit| limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    let limit_satisfied = is_capacity.map(|first| first || is_flow_and_capacity == Some(true));
    let maximum_capacity = if limit_satisfied == Some(true) {
        Some(input.maximum_total_cooling_capacity_w)
    } else {
        None
    };
    let capacity_is_zero = maximum_capacity.map(|capacity| capacity == 0.0);
    let zero_body = capacity_is_zero == Some(true);

    let prior_cool = if cooling {
        Some(input.supply_mass_flow_rate_for_cool_kg_per_s)
    } else {
        None
    };
    let prior_dehumidification = if cooling {
        Some(input.supply_mass_flow_rate_for_dehumidification_kg_per_s)
    } else {
        None
    };
    let prior_humidification =
        predecessor.resulting_supply_mass_flow_rate_for_humidification_kg_per_s;
    // Preserve the three assignment sites and their exact source order.
    let assigned_cool = zero_body.then_some(0.0_f64);
    let resulting_cool = prior_cool.map(|prior| assigned_cool.unwrap_or(prior));
    let assigned_dehumidification = zero_body.then_some(0.0_f64);
    let resulting_dehumidification =
        prior_dehumidification.map(|prior| assigned_dehumidification.unwrap_or(prior));
    let assigned_humidification = zero_body.then_some(0.0_f64);
    let resulting_humidification =
        prior_humidification.map(|prior| assigned_humidification.unwrap_or(prior));

    state.transition_count += 1;
    if cooling {
        state.cooling_body_entry_count += 1;
        state.first_cooling_limit_read_count += 1;
        if is_capacity == Some(true) {
            state.cooling_limit_capacity_count += 1;
        } else {
            state.second_cooling_limit_read_count += 1;
            if is_flow_and_capacity == Some(true) {
                state.cooling_limit_flow_rate_and_capacity_count += 1;
            } else {
                state.cooling_limit_rejected_count += 1;
            }
        }
        if limit_satisfied == Some(true) {
            state.maximum_total_cooling_capacity_read_count += 1;
            state.maximum_total_cooling_capacity_comparison_count += 1;
            if zero_body {
                state.maximum_total_cooling_capacity_zero_count += 1;
                state.zero_cooling_capacity_body_entry_count += 1;
                state.supply_mass_flow_rate_for_cool_zero_assignment_count += 1;
                state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count += 1;
                state.supply_mass_flow_rate_for_humidification_zero_assignment_count += 1;
            } else {
                state.maximum_total_cooling_capacity_nonzero_count += 1;
            }
        }
    } else if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
    } else {
        state.non_cooling_skip_count += 1;
    }

    let route = if predecessor.unit_off_skipped {
        PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::NonCooling
    } else if limit_satisfied != Some(true) {
        PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::CoolingLimitRejected
    } else if !zero_body {
        PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::MaximumCoolingCapacityNonZero
    } else {
        PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::CandidatesZeroed
    };

    let snapshot = PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        first_cooling_limit_read: cooling,
        first_cooling_limit: first_limit,
        cooling_limit_capacity: is_capacity,
        second_cooling_limit_read: is_capacity == Some(false),
        second_cooling_limit: second_limit,
        cooling_limit_flow_rate_and_capacity: is_flow_and_capacity,
        cooling_limit_condition_satisfied: limit_satisfied,
        maximum_total_cooling_capacity_read: limit_satisfied == Some(true),
        maximum_total_cooling_capacity_w: maximum_capacity,
        maximum_total_cooling_capacity_comparison_evaluated: limit_satisfied == Some(true),
        maximum_total_cooling_capacity_equal_to_zero: capacity_is_zero,
        zero_cooling_capacity_body_entered: zero_body,
        predecessor_supply_mass_flow_rate_for_cool_kg_per_s: prior_cool,
        predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s: prior_dehumidification,
        predecessor_supply_mass_flow_rate_for_humidification_kg_per_s: prior_humidification,
        supply_mass_flow_rate_for_cool_zero_assigned: zero_body,
        assigned_supply_mass_flow_rate_for_cool_kg_per_s: assigned_cool,
        supply_mass_flow_rate_for_dehumidification_zero_assigned: zero_body,
        assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s: assigned_dehumidification,
        supply_mass_flow_rate_for_humidification_zero_assigned: zero_body,
        assigned_supply_mass_flow_rate_for_humidification_kg_per_s: assigned_humidification,
        resulting_supply_mass_flow_rate_for_cool_kg_per_s: resulting_cool,
        resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s: resulting_dehumidification,
        resulting_supply_mass_flow_rate_for_humidification_kg_per_s: resulting_humidification,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
