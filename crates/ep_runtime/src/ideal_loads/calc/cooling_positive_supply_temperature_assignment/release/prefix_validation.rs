//! CP310/CP318/CP329/CP330/CP331-to-CP332 lineage validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirCalcEntrySnapshot,
};

pub(super) fn temperature_assignment_links_to_cp_air_assignment(
    assignment: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.supply_temperature_assignment_executed
            == predecessor.cp_air_assignment_executed
}

pub(super) fn cp_air_assignment_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(left.zone_humidity_ratio, right.zone_humidity_ratio)
        && option_bits_match(
            left.psychrometric_cp_air_result_j_per_kg_k,
            right.psychrometric_cp_air_result_j_per_kg_k,
        )
        && option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    for snapshot in [&mut left, &mut right] {
        snapshot.zone_humidity_ratio = None;
        snapshot.psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.cp_air_j_per_kg_k = None;
    }
    values_match && left == right
}

pub(in crate::ideal_loads::calc) fn active_operands_link_to_retained_prefix(
    entry: PurchasedAirCalcEntrySnapshot,
    sensible_flow: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    positive_guard: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    cp_air_assignment: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    zone_node_temperature_c: f64,
) -> bool {
    let ordinal = cp_air_assignment.parent_call_ordinal;
    let system = cp_air_assignment.system;
    let zone = cp_air_assignment.controlled_zone;
    let entry_load = entry.demand.remaining_output_req_to_cool_sp_w;
    let sensible_load_matches = match sensible_flow.zone_cooling_setpoint_load_w {
        Some(sensible_load) => {
            sensible_flow.zone_cooling_setpoint_load_read
                && entry_load.to_bits() == sensible_load.to_bits()
        }
        None => !sensible_flow.zone_cooling_setpoint_load_read,
    };
    let Some(sensible_cp_air) = sensible_flow.cp_air_j_per_kg_k else {
        return false;
    };
    let Some(sensible_zone_temperature) = sensible_flow.zone_temperature_c else {
        return false;
    };
    let Some(recirculation_temperature) = mixed_air.recirculation_temperature_c else {
        return false;
    };
    let Some(mixed_air_temperature) = mixed_air.mixed_air_temperature_c else {
        return false;
    };
    let Some(supply_flow) = positive_guard.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(cp_air) = cp_air_assignment.cp_air_j_per_kg_k else {
        return false;
    };

    entry.system == system
        && entry.call_ordinal == ordinal
        && entry.controlled_zone == zone
        && entry.demand.zone == zone
        && sensible_flow.system == system
        && sensible_flow.parent_call_ordinal == ordinal
        && sensible_flow.controlled_zone == zone
        && sensible_load_matches
        && sensible_flow.cp_air_assigned
        && sensible_flow.zone_temperature_read
        && mixed_air.system == system
        && mixed_air.parent_call_ordinal == ordinal
        && mixed_air.controlled_zone == zone
        && mixed_air.recirculation_temperature_read
        && mixed_air.mixed_air_temperature_assigned
        && positive_guard.system == system
        && positive_guard.parent_call_ordinal == ordinal
        && positive_guard.controlled_zone == zone
        && positive_guard.positive_supply_mass_flow_body_entered
        && cp_air_assignment.cp_air_assignment_executed
        && cp_air.to_bits() == sensible_cp_air.to_bits()
        && zone_node_temperature_c.to_bits() == sensible_zone_temperature.to_bits()
        && zone_node_temperature_c.to_bits() == recirculation_temperature.to_bits()
        && zone_node_temperature_c.to_bits() == mixed_air_temperature.to_bits()
        && supply_flow > 0.0
}

pub(super) fn assigned_operands_match_sources(
    assignment: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    entry: PurchasedAirCalcEntrySnapshot,
    positive_guard: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    cp_air_assignment: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    options_match_value_bits(
        assignment.zone_cooling_setpoint_load_w,
        entry.demand.remaining_output_req_to_cool_sp_w,
    ) && options_match_options_bits(
        assignment.cp_air_j_per_kg_k,
        cp_air_assignment.cp_air_j_per_kg_k,
    ) && options_match_options_bits(
        assignment.supply_mass_flow_rate_kg_per_s,
        positive_guard.supply_mass_flow_rate_kg_per_s,
    )
}

fn options_match_value_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn options_match_options_bits(left: Option<f64>, right: Option<f64>) -> bool {
    option_bits_match(left, right)
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
