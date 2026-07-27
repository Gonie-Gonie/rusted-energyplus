use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, PurchasedAirCalcCoolingSensibleFlowSnapshot,
};

pub(super) fn calculation_cooling_capacity_zero_flow_reset_snapshot(
    sensible: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    dehumidification: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    humidification: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot {
    let cooling = humidification.cooling_body_entered;
    let prior_cool = sensible.resulting_supply_mass_flow_rate_for_cool_kg_per_s;
    let prior_dehumidification =
        dehumidification.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s;
    let prior_humidification =
        humidification.resulting_supply_mass_flow_rate_for_humidification_kg_per_s;
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
        system: humidification.system,
        parent_call_ordinal: humidification.parent_call_ordinal,
        controlled_zone: humidification.controlled_zone,
        unit_body_entered: humidification.unit_body_entered,
        predecessor_cooling_body_entered: cooling,
        unit_off_skipped: humidification.unit_off_skipped,
        non_cooling_skipped: humidification.non_cooling_skipped,
        cooling_body_entered: cooling,
        first_cooling_limit_read: cooling,
        first_cooling_limit: cooling.then_some(IdealLoadsLimit::NoLimit),
        cooling_limit_capacity: cooling.then_some(false),
        second_cooling_limit_read: cooling,
        second_cooling_limit: cooling.then_some(IdealLoadsLimit::NoLimit),
        cooling_limit_flow_rate_and_capacity: cooling.then_some(false),
        cooling_limit_condition_satisfied: cooling.then_some(false),
        maximum_total_cooling_capacity_read: false,
        maximum_total_cooling_capacity_w: None,
        maximum_total_cooling_capacity_comparison_evaluated: false,
        maximum_total_cooling_capacity_equal_to_zero: None,
        zero_cooling_capacity_body_entered: false,
        predecessor_supply_mass_flow_rate_for_cool_kg_per_s: prior_cool,
        predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s: prior_dehumidification,
        predecessor_supply_mass_flow_rate_for_humidification_kg_per_s: prior_humidification,
        supply_mass_flow_rate_for_cool_zero_assigned: false,
        assigned_supply_mass_flow_rate_for_cool_kg_per_s: None,
        supply_mass_flow_rate_for_dehumidification_zero_assigned: false,
        assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s: None,
        supply_mass_flow_rate_for_humidification_zero_assigned: false,
        assigned_supply_mass_flow_rate_for_humidification_kg_per_s: None,
        resulting_supply_mass_flow_rate_for_cool_kg_per_s: prior_cool,
        resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s: prior_dehumidification,
        resulting_supply_mass_flow_rate_for_humidification_kg_per_s: prior_humidification,
    }
}
