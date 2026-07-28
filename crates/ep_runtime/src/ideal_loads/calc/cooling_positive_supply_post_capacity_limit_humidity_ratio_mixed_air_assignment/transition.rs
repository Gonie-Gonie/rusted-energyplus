//! Pure CP344-to-CP345 post-capacity-limit humidity-ratio assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput
{
    pub mixed_air_humidity_ratio: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput,
    >,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot {
    let assignment_after_capacity_guard =
        predecessor.capacity_limit_guard_false_fallthrough_skipped;
    let assignment_after_sensible_guard =
        predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment_after_temperature_limit =
        predecessor.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let assignment_executed = assignment_after_capacity_guard
        || assignment_after_sensible_guard
        || assignment_after_temperature_limit;
    debug_assert_eq!(
        usize::from(assignment_after_capacity_guard)
            + usize::from(assignment_after_sensible_guard)
            + usize::from(assignment_after_temperature_limit),
        usize::from(assignment_executed),
    );
    debug_assert_eq!(assignment_executed, active_input.is_some());

    let mixed_air_humidity_ratio = active_input.map(|input| input.mixed_air_humidity_ratio);
    let assigned_supply_humidity_ratio = mixed_air_humidity_ratio;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute::PositiveGuardFalseFallthrough
    } else {
        debug_assert!(assignment_executed);
        state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
                .len();
        state.mixed_air_humidity_ratio_read_count += 1;
        state.supply_humidity_ratio_assignment_count += 1;
        state.witnessed_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count += 1;
        if assignment_after_capacity_guard {
            state.assignment_after_capacity_limit_guard_false_fallthrough_count += 1;
            state.witnessed_assignment_after_capacity_limit_guard_false_fallthrough_count += 1;
        } else if assignment_after_sensible_guard {
            state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count +=
                1;
            state
                .witnessed_assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count +=
                1;
        } else {
            debug_assert!(assignment_after_temperature_limit);
            state
                .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count +=
                1;
            state
                .witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count +=
                1;
        }
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute::SupplyHumidityRatioMixedAirAssigned
    };

    let snapshot =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
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
                .predecessor_capacity_limit_sensible_output_guard_evaluated,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: predecessor
                .predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
            predecessor_capacity_limit_sensible_output_adjustment_body_entered: predecessor
                .predecessor_capacity_limit_sensible_output_adjustment_body_entered,
            predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed:
                predecessor.predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed,
            predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed:
                predecessor.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed,
            predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed:
                predecessor.predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_guard_false_fallthrough: predecessor
                .capacity_limit_sensible_output_guard_false_fallthrough,
            capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
                predecessor
                    .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed:
                assignment_executed,
            mixed_air_humidity_ratio_read: assignment_executed,
            mixed_air_humidity_ratio,
            supply_humidity_ratio_assignment_performed: assignment_executed,
            assigned_supply_humidity_ratio,
        };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
