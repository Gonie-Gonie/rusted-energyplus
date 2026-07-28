use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
};

pub(super) fn calculation_cooling_supply_mass_flow_ems_override_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
    let cooling = predecessor.cooling_body_entered;
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        ems_supply_mass_flow_override_flag_read: cooling,
        ems_supply_mass_flow_override_enabled: cooling.then_some(false),
        ems_supply_mass_flow_override_guard_evaluated: cooling,
        ems_supply_mass_flow_override_body_entered: false,
        ems_supply_mass_flow_override_guard_false_fallthrough: cooling,
    }
}
