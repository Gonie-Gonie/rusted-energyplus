use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
    capacity_owner: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    capacity_corroborator: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot {
    let evaluated = predecessor.dehumidification_total_output_assignment_executed;
    let cooling_total_output_w = evaluated.then(|| {
        predecessor
            .cooling_total_output_w
            .expect("active CP383 fixture CP382 total-output owner")
    });
    let maximum_total_cooling_capacity_w = evaluated.then(|| {
        let value = capacity_owner
            .maximum_total_cooling_capacity_w
            .expect("active CP383 fixture CP321 capacity owner");
        debug_assert_eq!(
            value.to_bits(),
            capacity_corroborator
                .maximum_total_cooling_capacity_w
                .expect("active CP383 fixture CP340 capacity corroborator")
                .to_bits(),
        );
        value
    });
    let comparison = cooling_total_output_w
        .zip(maximum_total_cooling_capacity_w)
        .map(|(output, maximum)| output > maximum);

    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: evaluated,
        dehumidification_total_output_capacity_guard_evaluated: evaluated,
        cp382_cooling_total_output_owned_read: evaluated,
        cooling_total_output_read: evaluated,
        cooling_total_output_w,
        cp321_maximum_total_cooling_capacity_owned_read: evaluated,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: evaluated,
        maximum_total_cooling_capacity_read: evaluated,
        maximum_total_cooling_capacity_w,
        cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated: evaluated,
        cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity: comparison,
        dehumidification_total_output_capacity_adjustment_body_entered: comparison == Some(true),
        dehumidification_total_output_capacity_guard_false_fallthrough: comparison == Some(false),
    }
}
