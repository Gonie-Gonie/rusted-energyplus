//! Pure CP339-to-CP340 Cooling capacity-limit sensible-output guard.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardActiveInput
{
    pub cooling_sensible_output_w: f64,
    pub maximum_total_cooling_capacity_w: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_capacity_limit_sensible_output_guard_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    active_input:
        Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardActiveInput>,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
    let guard_evaluated =
        predecessor.capacity_limit_sensible_output_assignment_executed;
    debug_assert_eq!(guard_evaluated, active_input.is_some());
    let cooling_sensible_output_w =
        active_input.map(|input| input.cooling_sensible_output_w);
    let maximum_total_cooling_capacity_w =
        active_input.map(|input| input.maximum_total_cooling_capacity_w);
    let at_or_above_maximum = active_input.map(|input| {
        input.cooling_sensible_output_w >= input.maximum_total_cooling_capacity_w
    });
    let body_entered = at_or_above_maximum == Some(true);
    let false_fallthrough = at_or_above_maximum == Some(false);

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::
            PositiveGuardFalseFallthrough
    } else if predecessor.capacity_limit_guard_false_fallthrough_skipped {
        state.capacity_limit_guard_false_fallthrough_skip_count += 1;
        state.witnessed_capacity_limit_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::
            ActiveCapacityLimitGuardFalseFallthrough
    } else {
        state.capacity_limit_sensible_output_guard_evaluation_count += 1;
        state.source_site_execution_count += 3 + usize::from(body_entered);
        state.cooling_sensible_output_read_count += 1;
        state.maximum_total_cooling_capacity_read_count += 1;
        state.cooling_sensible_output_maximum_capacity_comparison_count += 1;
        if body_entered {
            state.capacity_limit_sensible_output_adjustment_body_entry_count += 1;
            state.witnessed_capacity_limit_sensible_output_adjustment_body_entry_count += 1;
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::
                CapacityLimitSensibleOutputAdjustmentBodyEntered
        } else {
            state.capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
            state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count += 1;
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::
                CapacityLimitSensibleOutputGuardFalseFallthrough
        }
    };

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
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
                .capacity_limit_sensible_output_assignment_executed,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_guard_evaluated: guard_evaluated,
            cooling_sensible_output_read: guard_evaluated,
            cooling_sensible_output_w,
            maximum_total_cooling_capacity_read: guard_evaluated,
            maximum_total_cooling_capacity_w,
            cooling_sensible_output_maximum_capacity_comparison_evaluated: guard_evaluated,
            cooling_sensible_output_at_or_above_maximum_capacity: at_or_above_maximum,
            capacity_limit_sensible_output_guard_false_fallthrough: false_fallthrough,
            capacity_limit_sensible_output_adjustment_body_entered: body_entered,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
