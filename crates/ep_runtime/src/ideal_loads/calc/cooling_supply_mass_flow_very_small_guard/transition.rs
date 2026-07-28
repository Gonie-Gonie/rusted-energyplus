//! Pure CP326-to-CP327 cooling supply-mass-flow very-small-guard transition.

use super::{
    ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardInput,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_mass_flow_very_small_guard_state(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    input: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardInput,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let supply_mass_flow_rate_kg_per_s = if cooling {
        input.supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let hvac_very_small_mass_flow_kg_per_s = if cooling {
        Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S)
    } else {
        None
    };
    let at_or_below = supply_mass_flow_rate_kg_per_s
        .zip(hvac_very_small_mass_flow_kg_per_s)
        .map(|(supply, threshold)| supply <= threshold);
    let body_entered = at_or_below == Some(true);
    let false_fallthrough = at_or_below == Some(false);

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute::NonCooling
    } else {
        state.cooling_body_entry_count += 1;
        state.supply_mass_flow_rate_read_count += 1;
        state.hvac_very_small_mass_flow_read_count += 1;
        state.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count += 1;
        if body_entered {
            state.zero_flow_reset_body_entry_count += 1;
            state.witnessed_zero_flow_reset_body_entry_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute::
                ZeroFlowResetBodyEntered
        } else {
            state.active_guard_false_fallthrough_count += 1;
            state.witnessed_active_guard_false_fallthrough_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute::
                ActiveGuardFalseFallthrough
        }
    };

    let snapshot = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_ems_supply_mass_flow_override_body_entered: predecessor
            .predecessor_ems_supply_mass_flow_override_body_entered,
        predecessor_ems_supply_mass_flow_override_body_skipped: predecessor
            .predecessor_ems_supply_mass_flow_override_body_skipped,
        predecessor_ems_disabled_fallthrough: predecessor.predecessor_ems_disabled_fallthrough,
        predecessor_supply_mass_flow_limit_body_entered: predecessor
            .supply_mass_flow_limit_body_entered,
        predecessor_supply_mass_flow_limit_body_skipped: predecessor.body_skipped,
        predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: predecessor
            .active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        supply_mass_flow_rate_read: cooling,
        supply_mass_flow_rate_kg_per_s,
        hvac_very_small_mass_flow_read: cooling,
        hvac_very_small_mass_flow_source: if cooling {
            Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE)
        } else {
            None
        },
        hvac_very_small_mass_flow_kg_per_s,
        supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated: cooling,
        supply_mass_flow_rate_at_or_below_very_small_mass_flow: at_or_below,
        zero_flow_reset_body_entered: body_entered,
        active_guard_false_fallthrough: false_fallthrough,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
