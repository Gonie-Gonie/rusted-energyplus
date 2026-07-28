use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    capacity: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
    let evaluated = predecessor.capacity_limit_sensible_output_assignment_executed;
    let cooling_sensible_output_w = evaluated.then(|| {
        predecessor
            .cooling_sensible_output_w
            .expect("active CP340 fixture predecessor output")
    });
    let maximum_total_cooling_capacity_w = evaluated.then(|| {
        capacity
            .maximum_total_cooling_capacity_w
            .expect("active CP340 fixture CP321 capacity")
    });
    let comparison = cooling_sensible_output_w
        .zip(maximum_total_cooling_capacity_w)
        .map(|(output, maximum)| output >= maximum);

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
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
            .capacity_limit_sensible_output_assignment_executed,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_false_fallthrough_skipped: predecessor
            .capacity_limit_guard_false_fallthrough_skipped,
        capacity_limit_sensible_output_guard_evaluated: evaluated,
        cooling_sensible_output_read: evaluated,
        cooling_sensible_output_w,
        maximum_total_cooling_capacity_read: evaluated,
        maximum_total_cooling_capacity_w,
        cooling_sensible_output_maximum_capacity_comparison_evaluated: evaluated,
        cooling_sensible_output_at_or_above_maximum_capacity: comparison,
        capacity_limit_sensible_output_guard_false_fallthrough:
            comparison == Some(false),
        capacity_limit_sensible_output_adjustment_body_entered: comparison == Some(true),
    }
}
