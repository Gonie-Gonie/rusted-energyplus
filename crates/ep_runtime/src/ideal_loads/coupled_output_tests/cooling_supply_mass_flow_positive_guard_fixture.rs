use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

pub(super) fn calculation_cooling_supply_mass_flow_positive_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
    let cooling = predecessor.cooling_call_executed;
    let supply = if cooling {
        predecessor.supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let strictly_positive = supply.map(|value| value > 0.0);

    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
        system: predecessor.system,
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
        supply_mass_flow_rate_kg_per_s: supply,
        supply_mass_flow_rate_strictly_positive_comparison_evaluated: cooling,
        supply_mass_flow_rate_strictly_positive: strictly_positive,
        positive_supply_mass_flow_body_entered: strictly_positive == Some(true),
        active_guard_false_fallthrough: strictly_positive == Some(false),
    }
}
