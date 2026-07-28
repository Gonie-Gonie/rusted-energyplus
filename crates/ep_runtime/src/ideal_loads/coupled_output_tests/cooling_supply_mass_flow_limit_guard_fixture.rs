use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};

pub(super) fn calculation_cooling_supply_mass_flow_limit_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    cooling_limit: IdealLoadsLimit,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let flow_rate = cooling && cooling_limit == IdealLoadsLimit::LimitFlowRate;
    let read_second = cooling && !flow_rate;
    let flow_rate_and_capacity =
        read_second && cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let selected = flow_rate || flow_rate_and_capacity;
    let positive = selected && maximum_cooling_air_mass_flow_rate_kg_per_s > 0.0;

    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_ems_supply_mass_flow_override_body_entered: predecessor
            .predecessor_ems_supply_mass_flow_override_body_entered,
        predecessor_ems_supply_mass_flow_override_body_skipped: predecessor.body_skipped,
        predecessor_ems_disabled_fallthrough: predecessor.ems_disabled_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        first_cooling_limit_read: cooling,
        first_cooling_limit: cooling.then_some(cooling_limit),
        cooling_limit_flow_rate_comparison_evaluated: cooling,
        cooling_limit_flow_rate: cooling.then_some(flow_rate),
        second_cooling_limit_read: read_second,
        second_cooling_limit: read_second.then_some(cooling_limit),
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: read_second,
        cooling_limit_flow_rate_and_capacity: read_second.then_some(flow_rate_and_capacity),
        cooling_limit_condition_satisfied: cooling.then_some(selected),
        maximum_cooling_air_mass_flow_rate_read: selected,
        maximum_cooling_air_mass_flow_rate_kg_per_s: selected
            .then_some(maximum_cooling_air_mass_flow_rate_kg_per_s),
        maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: selected,
        maximum_cooling_air_mass_flow_rate_strictly_positive: selected.then_some(positive),
        supply_mass_flow_limit_body_entered: positive,
        active_guard_false_fallthrough: cooling && !positive,
    }
}
