use crate::ideal_loads::{
    ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

pub(super) fn calculation_cooling_supply_mass_flow_very_small_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let supply = if cooling {
        predecessor.resulting_supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let threshold = cooling.then_some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S);
    let comparison = supply
        .zip(threshold)
        .map(|(supply, threshold)| supply <= threshold);

    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
        system: predecessor.system,
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
        supply_mass_flow_rate_kg_per_s: supply,
        hvac_very_small_mass_flow_read: cooling,
        hvac_very_small_mass_flow_source: cooling
            .then_some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE),
        hvac_very_small_mass_flow_kg_per_s: threshold,
        supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated: cooling,
        supply_mass_flow_rate_at_or_below_very_small_mass_flow: comparison,
        zero_flow_reset_body_entered: comparison == Some(true),
        active_guard_false_fallthrough: comparison == Some(false),
    }
}
