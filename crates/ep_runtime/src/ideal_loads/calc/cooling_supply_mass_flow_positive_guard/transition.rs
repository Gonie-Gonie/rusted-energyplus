//! Pure CP329-to-CP330 Cooling positive supply-mass-flow guard transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingMixedAirCallSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_mass_flow_positive_guard_state(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
    let cooling = predecessor.cooling_call_executed;
    let supply_mass_flow_rate_kg_per_s = if cooling {
        predecessor.supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    debug_assert_eq!(supply_mass_flow_rate_kg_per_s.is_some(), cooling);
    let strictly_positive = supply_mass_flow_rate_kg_per_s.map(|supply| supply > 0.0);
    let body_entered = strictly_positive == Some(true);
    let false_fallthrough = strictly_positive == Some(false);

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::NonCooling
    } else {
        state.cooling_body_entry_count += 1;
        state.source_site_execution_count += 2 + usize::from(body_entered);
        state.supply_mass_flow_rate_read_count += 1;
        state.supply_mass_flow_rate_strictly_positive_comparison_count += 1;
        if body_entered {
            state.positive_supply_mass_flow_body_entry_count += 1;
            state.witnessed_positive_supply_mass_flow_body_entry_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::
                PositiveSupplyMassFlowBodyEntered
        } else {
            state.active_guard_false_fallthrough_count += 1;
            state.witnessed_active_guard_false_fallthrough_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::
                ActiveGuardFalseFallthrough
        }
    };

    let snapshot = PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_call_executed: predecessor.cooling_call_executed,
        predecessor_zero_flow_reset_body_entered: predecessor
            .predecessor_zero_flow_reset_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor
            .predecessor_active_guard_false_fallthrough,
        predecessor_no_outdoor_air_fallback_entered: predecessor.no_outdoor_air_fallback_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        supply_mass_flow_rate_read: cooling,
        supply_mass_flow_rate_kg_per_s,
        supply_mass_flow_rate_strictly_positive_comparison_evaluated: cooling,
        supply_mass_flow_rate_strictly_positive: strictly_positive,
        positive_supply_mass_flow_body_entered: body_entered,
        active_guard_false_fallthrough: false_fallthrough,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
