//! Pure CP333-to-CP334 Cooling positive-supply mixed-air temperature limit.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitActiveInput
{
    pub supply_temperature_before_mixed_air_limit_c: f64,
    pub mixed_air_temperature_c: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_temperature_mixed_air_limit_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    active_input: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitActiveInput>,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot {
    let limit_executed = predecessor.supply_temperature_minimum_limit_executed;
    debug_assert_eq!(limit_executed, active_input.is_some());

    let supply_temperature_before_mixed_air_limit_c =
        active_input.map(|input| input.supply_temperature_before_mixed_air_limit_c);
    let mixed_air_temperature_c = active_input.map(|input| input.mixed_air_temperature_c);
    let minimum_supply_temperature_c = active_input.map(|input| {
        source_shaped_two_argument_minimum(
            input.supply_temperature_before_mixed_air_limit_c,
            input.mixed_air_temperature_c,
        )
    });
    let assigned_supply_temperature_c = minimum_supply_temperature_c;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRetainedRoute::
            PositiveGuardFalseFallthrough
    } else {
        state.supply_temperature_mixed_air_limit_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
                .len();
        state.supply_temperature_for_minimum_read_count += 1;
        state.mixed_air_temperature_for_minimum_read_count += 1;
        state.source_shaped_two_argument_minimum_evaluation_count += 1;
        state.supply_temperature_assignment_count += 1;
        state.witnessed_supply_temperature_mixed_air_limit_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRetainedRoute::
            SupplyTemperatureMixedAirLimitExecuted
    };

    let snapshot = PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
        supply_temperature_mixed_air_limit_executed: limit_executed,
        supply_temperature_for_minimum_read: limit_executed,
        supply_temperature_before_mixed_air_limit_c,
        mixed_air_temperature_for_minimum_read: limit_executed,
        mixed_air_temperature_c,
        source_shaped_two_argument_minimum_evaluated: limit_executed,
        minimum_supply_temperature_c,
        supply_temperature_assignment_performed: limit_executed,
        assigned_supply_temperature_c,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}

#[inline]
pub(in crate::ideal_loads::calc) fn source_shaped_two_argument_minimum(
    left: f64,
    right: f64,
) -> f64 {
    if left < right { left } else { right }
}
