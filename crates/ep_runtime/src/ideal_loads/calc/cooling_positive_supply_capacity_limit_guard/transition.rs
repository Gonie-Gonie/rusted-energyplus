//! Pure CP336-to-CP337 Cooling positive-supply capacity-limit guard transition.

use ep_model::IdealLoadsLimit;

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardActiveInput
{
    pub cooling_limit: IdealLoadsLimit,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_capacity_limit_guard_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    active_input: Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardActiveInput>,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
    let guard_evaluated = predecessor.supply_enthalpy_assignment_executed;
    debug_assert_eq!(guard_evaluated, active_input.is_some());

    let first_cooling_limit = active_input.map(|input| input.cooling_limit);
    let cooling_limit_capacity =
        first_cooling_limit.map(|limit| limit == IdealLoadsLimit::LimitCapacity);
    let second_cooling_limit = if cooling_limit_capacity == Some(false) {
        active_input.map(|input| input.cooling_limit)
    } else {
        None
    };
    let cooling_limit_flow_rate_and_capacity = second_cooling_limit
        .map(|limit| limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    let cooling_limit_condition_satisfied = cooling_limit_capacity.map(|capacity| {
        capacity || cooling_limit_flow_rate_and_capacity == Some(true)
    });
    let capacity_limit_body_entered = cooling_limit_condition_satisfied == Some(true);
    let cooling_limit_rejected = cooling_limit_condition_satisfied == Some(false);
    let active_guard_false_fallthrough = cooling_limit_rejected;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute::
            PositiveGuardFalseFallthrough
    } else {
        state.capacity_limit_guard_evaluation_count += 1;
        state.first_cooling_limit_read_count += 1;
        state.cooling_limit_capacity_comparison_count += 1;
        state.source_site_execution_count += 2;
        if cooling_limit_capacity == Some(true) {
            state.cooling_limit_capacity_match_count += 1;
        } else {
            state.second_cooling_limit_read_count += 1;
            state.cooling_limit_flow_rate_and_capacity_comparison_count += 1;
            state.source_site_execution_count += 2;
            if cooling_limit_flow_rate_and_capacity == Some(true) {
                state.cooling_limit_flow_rate_and_capacity_match_count += 1;
            }
        }

        if capacity_limit_body_entered {
            state.capacity_limit_body_entry_count += 1;
            state.witnessed_capacity_limit_body_entry_count += 1;
            state.source_site_execution_count += 1;
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute::
                CapacityLimitBodyEntered
        } else {
            state.cooling_limit_rejected_count += 1;
            state.active_guard_false_fallthrough_count += 1;
            state.witnessed_active_guard_false_fallthrough_count += 1;
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute::
                ActiveCapacityLimitGuardFalseFallthrough
        }
    };

    let snapshot = PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor
            .predecessor_active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_evaluated: guard_evaluated,
        first_cooling_limit_read: guard_evaluated,
        first_cooling_limit,
        cooling_limit_capacity_comparison_evaluated: guard_evaluated,
        cooling_limit_capacity,
        second_cooling_limit_read: cooling_limit_capacity == Some(false),
        second_cooling_limit,
        cooling_limit_flow_rate_and_capacity_comparison_evaluated:
            cooling_limit_capacity == Some(false),
        cooling_limit_flow_rate_and_capacity,
        cooling_limit_condition_satisfied,
        cooling_limit_rejected,
        capacity_limit_body_entered,
        active_guard_false_fallthrough,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
