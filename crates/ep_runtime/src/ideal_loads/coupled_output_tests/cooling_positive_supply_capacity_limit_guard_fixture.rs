use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_capacity_limit_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
    let active = predecessor.supply_enthalpy_assignment_executed;
    let capacity_match = active && cooling_limit == IdealLoadsLimit::LimitCapacity;
    let second_comparison = active && !capacity_match;
    let combined_match =
        second_comparison && cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let condition_satisfied = capacity_match || combined_match;

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        system: predecessor.system,
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
        capacity_limit_guard_evaluated: active,
        first_cooling_limit_read: active,
        first_cooling_limit: active.then_some(cooling_limit),
        cooling_limit_capacity_comparison_evaluated: active,
        cooling_limit_capacity: active.then_some(capacity_match),
        second_cooling_limit_read: second_comparison,
        second_cooling_limit: second_comparison.then_some(cooling_limit),
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: second_comparison,
        cooling_limit_flow_rate_and_capacity: second_comparison.then_some(combined_match),
        cooling_limit_condition_satisfied: active.then_some(condition_satisfied),
        cooling_limit_rejected: active && !condition_satisfied,
        capacity_limit_body_entered: active && condition_satisfied,
        active_guard_false_fallthrough: active && !condition_satisfied,
    }
}
