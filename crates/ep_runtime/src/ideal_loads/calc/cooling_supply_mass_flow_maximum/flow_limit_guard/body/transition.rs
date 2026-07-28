//! Pure CP325-to-CP326 cooling supply-mass-flow limit-body transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyInput,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_mass_flow_limit_body_state(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    input: PurchasedAirCalcCoolingSupplyMassFlowLimitBodyInput,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
    let cooling = predecessor.cooling_body_entered;
    let body_entered = predecessor.supply_mass_flow_limit_body_entered;
    let body_skipped = !body_entered;
    let supply_before = if body_entered {
        input.supply_mass_flow_rate_before_limit_kg_per_s
    } else {
        None
    };
    let maximum = body_entered.then_some(input.maximum_cooling_air_mass_flow_rate_kg_per_s);
    let minimum = supply_before
        .zip(maximum)
        .map(|(supply, maximum)| source_min(supply, maximum));
    let resulting = if cooling {
        if body_entered {
            minimum
        } else {
            input.supply_mass_flow_rate_before_limit_kg_per_s
        }
    } else {
        None
    };

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        state.body_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        state.body_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute::NonCooling
    } else {
        state.cooling_body_entry_count += 1;
        if body_entered {
            state.supply_mass_flow_limit_body_entry_count += 1;
            state.supply_mass_flow_rate_for_minimum_read_count += 1;
            state.maximum_cooling_air_mass_flow_rate_for_minimum_read_count += 1;
            state.source_shaped_two_argument_minimum_evaluation_count += 1;
            state.supply_mass_flow_rate_assignment_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute::SupplyMassFlowLimitApplied
        } else {
            state.body_skip_count += 1;
            state.active_guard_false_fallthrough_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute::ActiveGuardFalseFallthrough
        }
    };

    let snapshot = PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
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
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        supply_mass_flow_limit_body_entered: body_entered,
        body_skipped,
        active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        supply_mass_flow_rate_for_minimum_read: body_entered,
        supply_mass_flow_rate_before_limit_kg_per_s: supply_before,
        maximum_cooling_air_mass_flow_rate_for_minimum_read: body_entered,
        maximum_cooling_air_mass_flow_rate_kg_per_s: maximum,
        source_shaped_two_argument_minimum_evaluated: body_entered,
        minimum_supply_mass_flow_rate_kg_per_s: minimum,
        supply_mass_flow_rate_assignment_performed: body_entered,
        assigned_supply_mass_flow_rate_kg_per_s: minimum,
        resulting_supply_mass_flow_rate_kg_per_s: resulting,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}

#[inline]
fn source_min(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
}
