//! Pure CP323-to-CP324 EMS supply-mass-flow override body transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_mass_flow_ems_override_body_state(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    input: Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput>,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
    let cooling = predecessor.cooling_body_entered;
    let body_entered = predecessor.ems_supply_mass_flow_override_body_entered;
    let body_skipped = !body_entered;
    let ems_disabled_fallthrough =
        predecessor.ems_supply_mass_flow_override_guard_false_fallthrough;
    let characterized = if body_entered { input } else { None };
    let ems_value = characterized.map(|input| input.ems_supply_mass_flow_override_value_kg_per_s);
    let outdoor_air_before =
        characterized.map(|input| input.outdoor_air_mass_flow_rate_before_override_kg_per_s);
    let outdoor_air_after = outdoor_air_before
        .zip(ems_value)
        .map(|(outdoor_air, supply)| source_min(outdoor_air, supply));

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        state.body_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        state.body_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute::NonCooling
    } else {
        state.cooling_body_entry_count += 1;
        if body_entered {
            state.body_entry_count += 1;
            state.ems_supply_mass_flow_override_value_read_count += 1;
            state.supply_mass_flow_rate_override_assignment_count += 1;
            state.outdoor_air_mass_flow_rate_for_minimum_read_count += 1;
            state.supply_mass_flow_rate_for_minimum_read_count += 1;
            state.source_shaped_two_argument_minimum_evaluation_count += 1;
            state.outdoor_air_mass_flow_rate_assignment_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute::OverrideApplied
        } else {
            state.body_skip_count += 1;
            state.ems_disabled_fallthrough_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute::
                EmsDisabledFallthrough
        }
    };

    let snapshot = PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_ems_supply_mass_flow_override_body_entered: predecessor
            .ems_supply_mass_flow_override_body_entered,
        predecessor_ems_supply_mass_flow_override_guard_false_fallthrough: predecessor
            .ems_supply_mass_flow_override_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        body_skipped,
        ems_disabled_fallthrough,
        ems_supply_mass_flow_override_value_read: body_entered,
        ems_supply_mass_flow_override_value_kg_per_s: ems_value,
        supply_mass_flow_rate_override_assignment_performed: body_entered,
        assigned_supply_mass_flow_rate_kg_per_s: ems_value,
        outdoor_air_mass_flow_rate_for_minimum_read: body_entered,
        outdoor_air_mass_flow_rate_before_override_kg_per_s: outdoor_air_before,
        supply_mass_flow_rate_for_minimum_read: body_entered,
        supply_mass_flow_rate_for_minimum_kg_per_s: ems_value,
        source_shaped_two_argument_minimum_evaluated: body_entered,
        minimum_outdoor_air_mass_flow_rate_kg_per_s: outdoor_air_after,
        outdoor_air_mass_flow_rate_assignment_performed: body_entered,
        assigned_outdoor_air_mass_flow_rate_kg_per_s: outdoor_air_after,
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
