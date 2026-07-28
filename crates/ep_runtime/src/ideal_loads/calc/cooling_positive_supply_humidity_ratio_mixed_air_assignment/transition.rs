//! Pure CP334-to-CP335 Cooling positive-supply mixed-air humidity-ratio assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentActiveInput
{
    pub mixed_air_humidity_ratio: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_humidity_ratio_mixed_air_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    active_input: Option<
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentActiveInput,
    >,
) -> PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot {
    let assignment_executed = predecessor.supply_temperature_mixed_air_limit_executed;
    debug_assert_eq!(assignment_executed, active_input.is_some());

    let mixed_air_humidity_ratio = active_input.map(|input| input.mixed_air_humidity_ratio);
    let assigned_supply_humidity_ratio = mixed_air_humidity_ratio;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRetainedRoute::
            PositiveGuardFalseFallthrough
    } else {
        state.supply_humidity_ratio_mixed_air_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
                .len();
        state.mixed_air_humidity_ratio_read_count += 1;
        state.supply_humidity_ratio_assignment_count += 1;
        state.witnessed_supply_humidity_ratio_mixed_air_assignment_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRetainedRoute::
            SupplyHumidityRatioMixedAirAssigned
    };

    let snapshot = PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
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
        supply_humidity_ratio_mixed_air_assignment_executed: assignment_executed,
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
