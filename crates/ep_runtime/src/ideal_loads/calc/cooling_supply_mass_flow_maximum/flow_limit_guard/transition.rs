//! Pure CP324-to-CP325 cooling supply-mass-flow limit-guard transition.

use ep_model::IdealLoadsLimit;

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_mass_flow_limit_guard_state(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    input: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let first_limit = if cooling {
        Some(input.cooling_limit)
    } else {
        None
    };
    let is_flow_rate = first_limit.map(|limit| limit == IdealLoadsLimit::LimitFlowRate);
    let second_limit = if is_flow_rate == Some(false) {
        Some(input.cooling_limit)
    } else {
        None
    };
    let is_flow_rate_and_capacity =
        second_limit.map(|limit| limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    let limit_satisfied =
        is_flow_rate.map(|first| first || is_flow_rate_and_capacity == Some(true));
    let maximum_cooling_air_mass_flow_rate_kg_per_s = if limit_satisfied == Some(true) {
        Some(input.maximum_cooling_air_mass_flow_rate_kg_per_s)
    } else {
        None
    };
    let maximum_is_strictly_positive =
        maximum_cooling_air_mass_flow_rate_kg_per_s.map(|maximum| maximum > 0.0);
    let body_entered = maximum_is_strictly_positive == Some(true);
    let active_guard_false_fallthrough = cooling && !body_entered;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute::NonCooling
    } else {
        state.cooling_body_entry_count += 1;
        state.first_cooling_limit_read_count += 1;
        state.cooling_limit_flow_rate_comparison_count += 1;
        if is_flow_rate == Some(true) {
            state.cooling_limit_flow_rate_match_count += 1;
        } else {
            state.second_cooling_limit_read_count += 1;
            state.cooling_limit_flow_rate_and_capacity_comparison_count += 1;
        }
        if is_flow_rate_and_capacity == Some(true) {
            state.cooling_limit_flow_rate_and_capacity_match_count += 1;
        }
        if limit_satisfied == Some(true) {
            state.maximum_cooling_air_mass_flow_rate_read_count += 1;
            state.maximum_cooling_air_mass_flow_rate_positive_comparison_count += 1;
            if body_entered {
                state.maximum_cooling_air_mass_flow_rate_strictly_positive_count += 1;
                state.supply_mass_flow_limit_body_entry_count += 1;
            } else {
                state.maximum_cooling_air_mass_flow_rate_not_positive_count += 1;
            }
        } else {
            state.cooling_limit_rejected_count += 1;
        }
        if active_guard_false_fallthrough {
            state.active_guard_false_fallthrough_count += 1;
        }

        if limit_satisfied != Some(true) {
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute::CoolingLimitRejected
        } else if body_entered {
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute::FlowLimitBodyEntered
        } else {
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute::
                MaximumCoolingMassFlowNotPositive
        }
    };

    let snapshot = PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_ems_supply_mass_flow_override_body_entered: predecessor
            .predecessor_ems_supply_mass_flow_override_body_entered,
        predecessor_ems_supply_mass_flow_override_body_skipped: predecessor.body_skipped,
        predecessor_ems_disabled_fallthrough: predecessor.ems_disabled_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        first_cooling_limit_read: cooling,
        first_cooling_limit: first_limit,
        cooling_limit_flow_rate_comparison_evaluated: cooling,
        cooling_limit_flow_rate: is_flow_rate,
        second_cooling_limit_read: is_flow_rate == Some(false),
        second_cooling_limit: second_limit,
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: is_flow_rate == Some(false),
        cooling_limit_flow_rate_and_capacity: is_flow_rate_and_capacity,
        cooling_limit_condition_satisfied: limit_satisfied,
        maximum_cooling_air_mass_flow_rate_read: limit_satisfied == Some(true),
        maximum_cooling_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: limit_satisfied
            == Some(true),
        maximum_cooling_air_mass_flow_rate_strictly_positive: maximum_is_strictly_positive,
        supply_mass_flow_limit_body_entered: body_entered,
        active_guard_false_fallthrough,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
