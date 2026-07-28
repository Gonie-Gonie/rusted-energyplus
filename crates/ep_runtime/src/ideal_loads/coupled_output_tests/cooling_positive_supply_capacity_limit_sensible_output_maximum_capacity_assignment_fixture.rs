use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot
{
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment = predecessor.capacity_limit_sensible_output_adjustment_body_entered;
    let preexisting_cooling_sensible_output_w = (guard_false || assignment)
        .then_some(predecessor.cooling_sensible_output_w)
        .flatten();
    let maximum_total_cooling_capacity_w = assignment
        .then_some(predecessor.maximum_total_cooling_capacity_w)
        .flatten();
    let resulting_cooling_sensible_output_w = if assignment {
        maximum_total_cooling_capacity_w
    } else {
        preexisting_cooling_sensible_output_w
    };

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_capacity_limit_cp_air_assignment_executed: predecessor
            .predecessor_capacity_limit_cp_air_assignment_executed,
        predecessor_capacity_limit_sensible_output_assignment_executed: predecessor
            .predecessor_capacity_limit_sensible_output_assignment_executed,
        predecessor_capacity_limit_sensible_output_guard_evaluated: predecessor
            .capacity_limit_sensible_output_guard_evaluated,
        predecessor_capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        predecessor_capacity_limit_sensible_output_adjustment_body_entered: assignment,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_false_fallthrough_skipped: predecessor
            .capacity_limit_guard_false_fallthrough_skipped,
        capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        capacity_limit_sensible_output_maximum_capacity_assignment_executed: assignment,
        preexisting_cooling_sensible_output_w,
        maximum_total_cooling_capacity_read: assignment,
        maximum_total_cooling_capacity_w,
        cooling_sensible_output_assigned: assignment,
        assigned_cooling_sensible_output_w: maximum_total_cooling_capacity_w,
        resulting_cooling_sensible_output_w,
    }
}
