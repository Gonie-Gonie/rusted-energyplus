//! CP318/CP332-to-CP333 lineage validation.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
};

pub(super) fn minimum_limit_links_to_temperature_assignment(
    limit: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    limit.system == predecessor.system
        && limit.parent_call_ordinal == predecessor.parent_call_ordinal
        && limit.controlled_zone == predecessor.controlled_zone
        && limit.unit_body_entered == predecessor.unit_body_entered
        && limit.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && limit.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && limit.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && limit.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && limit.unit_off_skipped == predecessor.unit_off_skipped
        && limit.non_cooling_skipped == predecessor.non_cooling_skipped
        && limit.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && limit.supply_temperature_minimum_limit_executed
            == predecessor.supply_temperature_assignment_executed
}

pub(in crate::ideal_loads::calc) fn active_operands_link_to_retained_prefix(
    system: &IdealLoadsAirSystem,
    sensible_flow: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    supply_temperature_before_minimum_limit_c: Option<f64>,
    minimum_cooling_supply_air_temperature_c: Option<f64>,
) -> bool {
    let minimum = system.minimum_cooling_supply_air_temperature_c;
    predecessor.supply_temperature_assignment_executed
        && predecessor.supply_temperature_assigned
        && system.id == predecessor.system
        && sensible_flow.system == predecessor.system
        && sensible_flow.parent_call_ordinal == predecessor.parent_call_ordinal
        && sensible_flow.controlled_zone == predecessor.controlled_zone
        && sensible_flow.minimum_cooling_supply_air_temperature_read
        && option_matches_value_bits(
            sensible_flow.minimum_cooling_supply_air_temperature_c,
            minimum,
        )
        && options_match_bits(
            supply_temperature_before_minimum_limit_c,
            predecessor.supply_temperature_c,
        )
        && option_matches_value_bits(
            minimum_cooling_supply_air_temperature_c,
            minimum,
        )
}

pub(super) fn temperature_assignment_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let values_match = options_match_bits(
        left.zone_cooling_setpoint_load_w,
        right.zone_cooling_setpoint_load_w,
    ) && options_match_bits(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k)
        && options_match_bits(
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        )
        && options_match_bits(
            left.cp_air_times_supply_mass_flow_rate_w_per_k,
            right.cp_air_times_supply_mass_flow_rate_w_per_k,
        )
        && options_match_bits(
            left.zone_cooling_setpoint_load_over_denominator_c,
            right.zone_cooling_setpoint_load_over_denominator_c,
        )
        && options_match_bits(left.zone_node_temperature_c, right.zone_node_temperature_c)
        && options_match_bits(
            left.calculated_supply_temperature_c,
            right.calculated_supply_temperature_c,
        )
        && options_match_bits(left.supply_temperature_c, right.supply_temperature_c);
    for snapshot in [&mut left, &mut right] {
        snapshot.zone_cooling_setpoint_load_w = None;
        snapshot.cp_air_j_per_kg_k = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.cp_air_times_supply_mass_flow_rate_w_per_k = None;
        snapshot.zone_cooling_setpoint_load_over_denominator_c = None;
        snapshot.zone_node_temperature_c = None;
        snapshot.calculated_supply_temperature_c = None;
        snapshot.supply_temperature_c = None;
    }
    values_match && left == right
}

fn option_matches_value_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn options_match_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
