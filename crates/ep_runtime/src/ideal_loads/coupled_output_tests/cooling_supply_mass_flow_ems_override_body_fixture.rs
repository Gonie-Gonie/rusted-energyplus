use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
};

pub(super) fn calculation_cooling_supply_mass_flow_ems_override_body_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
    let cooling = predecessor.cooling_body_entered;
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
        system: predecessor.system,
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
        body_skipped: true,
        ems_disabled_fallthrough: cooling,
        ems_supply_mass_flow_override_value_read: false,
        ems_supply_mass_flow_override_value_kg_per_s: None,
        supply_mass_flow_rate_override_assignment_performed: false,
        assigned_supply_mass_flow_rate_kg_per_s: None,
        outdoor_air_mass_flow_rate_for_minimum_read: false,
        outdoor_air_mass_flow_rate_before_override_kg_per_s: None,
        supply_mass_flow_rate_for_minimum_read: false,
        supply_mass_flow_rate_for_minimum_kg_per_s: None,
        source_shaped_two_argument_minimum_evaluated: false,
        minimum_outdoor_air_mass_flow_rate_kg_per_s: None,
        outdoor_air_mass_flow_rate_assignment_performed: false,
        assigned_outdoor_air_mass_flow_rate_kg_per_s: None,
    }
}
