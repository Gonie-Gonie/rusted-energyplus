use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};

pub(super) fn calculation_cooling_supply_mass_flow_limit_body_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    supply_mass_flow_rate_before_limit_kg_per_s: Option<f64>,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
    let cooling = predecessor.cooling_body_entered;
    let body_entered = predecessor.supply_mass_flow_limit_body_entered;
    let supply_before = supply_mass_flow_rate_before_limit_kg_per_s.filter(|_| cooling);
    let maximum = body_entered.then_some(maximum_cooling_air_mass_flow_rate_kg_per_s);
    let minimum = supply_before
        .zip(maximum)
        .map(|(supply, maximum)| source_min(supply, maximum));
    let resulting = supply_before.map(|supply| minimum.unwrap_or(supply));

    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
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
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        supply_mass_flow_limit_body_entered: body_entered,
        body_skipped: !body_entered,
        active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        supply_mass_flow_rate_for_minimum_read: body_entered,
        supply_mass_flow_rate_before_limit_kg_per_s: if body_entered {
            supply_before
        } else {
            None
        },
        maximum_cooling_air_mass_flow_rate_for_minimum_read: body_entered,
        maximum_cooling_air_mass_flow_rate_kg_per_s: maximum,
        source_shaped_two_argument_minimum_evaluated: body_entered,
        minimum_supply_mass_flow_rate_kg_per_s: minimum,
        supply_mass_flow_rate_assignment_performed: body_entered,
        assigned_supply_mass_flow_rate_kg_per_s: minimum,
        resulting_supply_mass_flow_rate_kg_per_s: resulting,
    }
}

fn source_min(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
}
