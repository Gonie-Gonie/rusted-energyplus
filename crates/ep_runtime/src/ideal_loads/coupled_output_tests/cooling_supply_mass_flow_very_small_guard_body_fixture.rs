use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

pub(super) fn calculation_cooling_supply_mass_flow_very_small_guard_body_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
    let cooling = predecessor.cooling_body_entered;
    let body_entered = predecessor.zero_flow_reset_body_entered;
    let supply_before = if cooling {
        predecessor.supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let assigned = body_entered.then_some(0.0_f64);
    let resulting = supply_before.map(|supply| assigned.unwrap_or(supply));

    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
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
            .predecessor_supply_mass_flow_limit_body_entered,
        predecessor_supply_mass_flow_limit_body_skipped: predecessor
            .predecessor_supply_mass_flow_limit_body_skipped,
        predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: predecessor
            .predecessor_supply_mass_flow_limit_active_guard_false_fallthrough,
        predecessor_zero_flow_reset_body_entered: predecessor.zero_flow_reset_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        zero_flow_reset_body_entered: body_entered,
        body_skipped: !body_entered,
        active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        predecessor_supply_mass_flow_rate_kg_per_s: supply_before,
        supply_mass_flow_rate_positive_zero_assignment_performed: body_entered,
        assigned_supply_mass_flow_rate_kg_per_s: assigned,
        resulting_supply_mass_flow_rate_kg_per_s: resulting,
    }
}
