//! Pure CP322-to-CP323 EMS supply-mass-flow override guard transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_supply_mass_flow_ems_override_guard_state(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    ems_supply_mass_flow_override_enabled: bool,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let enabled = cooling.then_some(ems_supply_mass_flow_override_enabled);
    let body_entered = enabled == Some(true);
    let false_fallthrough = enabled == Some(false);

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute::NonCooling
    } else {
        state.cooling_body_entry_count += 1;
        state.ems_supply_mass_flow_override_flag_read_count += 1;
        state.ems_supply_mass_flow_override_guard_evaluation_count += 1;
        if body_entered {
            state.ems_supply_mass_flow_override_body_entry_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute::OverrideBodyEntered
        } else {
            state.ems_supply_mass_flow_override_guard_false_fallthrough_count += 1;
            PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute::
                OverrideGuardFalseFallthrough
        }
    };

    let snapshot = PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        ems_supply_mass_flow_override_flag_read: cooling,
        ems_supply_mass_flow_override_enabled: enabled,
        ems_supply_mass_flow_override_guard_evaluated: cooling,
        ems_supply_mass_flow_override_body_entered: body_entered,
        ems_supply_mass_flow_override_guard_false_fallthrough: false_fallthrough,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
