//! Pure CP340-to-CP341 Cooling sensible-output maximum-capacity assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot
{
    let guard_false =
        predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment_executed =
        predecessor.capacity_limit_sensible_output_adjustment_body_entered;
    debug_assert!(!(guard_false && assignment_executed));
    debug_assert_eq!(
        predecessor.capacity_limit_sensible_output_guard_evaluated,
        guard_false || assignment_executed
    );

    let preexisting_cooling_sensible_output_w = if guard_false || assignment_executed {
        predecessor.cooling_sensible_output_w
    } else {
        None
    };
    let maximum_total_cooling_capacity_w = if assignment_executed {
        predecessor.maximum_total_cooling_capacity_w
    } else {
        None
    };
    let assigned_cooling_sensible_output_w = if assignment_executed {
        maximum_total_cooling_capacity_w
    } else {
        None
    };
    let resulting_cooling_sensible_output_w = if assignment_executed {
        assigned_cooling_sensible_output_w
    } else {
        preexisting_cooling_sensible_output_w
    };

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute::PositiveGuardFalseFallthrough
    } else if predecessor.capacity_limit_guard_false_fallthrough_skipped {
        state.capacity_limit_guard_false_fallthrough_skip_count += 1;
        state.witnessed_capacity_limit_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute::ActiveCapacityLimitGuardFalseFallthrough
    } else if guard_false {
        state.capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
        state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute::CapacityLimitSensibleOutputGuardFalseFallthrough
    } else {
        debug_assert!(assignment_executed);
        state.capacity_limit_sensible_output_maximum_capacity_assignment_count += 1;
        state.source_site_execution_count += 2;
        state.maximum_total_cooling_capacity_read_count += 1;
        state.cooling_sensible_output_assignment_write_count += 1;
        state.witnessed_capacity_limit_sensible_output_maximum_capacity_assignment_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute::CapacityLimitSensibleOutputMaximumCapacityAssigned
    };

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
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
            predecessor_capacity_limit_sensible_output_adjustment_body_entered:
                assignment_executed,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_maximum_capacity_assignment_executed:
                assignment_executed,
            preexisting_cooling_sensible_output_w,
            maximum_total_cooling_capacity_read: assignment_executed,
            maximum_total_cooling_capacity_w,
            cooling_sensible_output_assigned: assignment_executed,
            assigned_cooling_sensible_output_w,
            resulting_cooling_sensible_output_w,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
