//! Exact bit-level CP391-to-CP392 snapshot lineage helpers.

use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    predecessor.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_flags(snapshot) == inherited_predecessor_flags(predecessor)
        && cp389_flags(snapshot) == predecessor_cp389_flags(predecessor)
        && cp390_flags(snapshot) == predecessor_cp390_flags(predecessor)
        && cp391_flags(snapshot) == predecessor_cp391_flags(predecessor)
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && predecessor_values(snapshot)
            .into_iter()
            .zip(predecessor_snapshot_values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn inherited_predecessor_flags(snapshot: PredecessorSnapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn cp389_flags(snapshot: Snapshot) -> [bool; 30] {
    [
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        snapshot.predecessor_mixed_air_humidity_ratio_read,
        snapshot.predecessor_psychrometric_cp_air_evaluated,
        snapshot.predecessor_cp_air_assigned,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        snapshot.predecessor_cp384_retained_cooling_total_output_owned_read,
        snapshot.predecessor_cp385_cooling_total_output_bit_corroborated,
        snapshot.predecessor_cooling_total_output_read,
        snapshot.predecessor_cooling_sensible_heat_ratio_read,
        snapshot.predecessor_cooling_sensible_output_calculated,
        snapshot.predecessor_cooling_sensible_output_assigned,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed,
        snapshot.predecessor_cp379_retained_supply_temperature_state_owned,
        snapshot.predecessor_cp329_retained_mixed_air_temperature_owned_read,
        snapshot.predecessor_mixed_air_temperature_read,
        snapshot.predecessor_cp388_retained_cooling_sensible_output_owned_read,
        snapshot.predecessor_cooling_sensible_output_read,
        snapshot.predecessor_cp387_retained_cp_air_owned_read,
        snapshot.predecessor_cp_air_read,
        snapshot.predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        snapshot.predecessor_supply_mass_flow_rate_read,
        snapshot.predecessor_cp_air_times_supply_mass_flow_rate_calculated,
        snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_calculated,
        snapshot.predecessor_supply_temperature_calculated,
        snapshot.predecessor_supply_temperature_assigned,
    ]
}

fn predecessor_cp389_flags(snapshot: PredecessorSnapshot) -> [bool; 30] {
    [
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        snapshot.predecessor_mixed_air_humidity_ratio_read,
        snapshot.predecessor_psychrometric_cp_air_evaluated,
        snapshot.predecessor_cp_air_assigned,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        snapshot.predecessor_cp384_retained_cooling_total_output_owned_read,
        snapshot.predecessor_cp385_cooling_total_output_bit_corroborated,
        snapshot.predecessor_cooling_total_output_read,
        snapshot.predecessor_cooling_sensible_heat_ratio_read,
        snapshot.predecessor_cooling_sensible_output_calculated,
        snapshot.predecessor_cooling_sensible_output_assigned,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed,
        snapshot.predecessor_cp379_retained_supply_temperature_state_owned,
        snapshot.predecessor_cp329_retained_mixed_air_temperature_owned_read,
        snapshot.predecessor_mixed_air_temperature_read,
        snapshot.predecessor_cp388_retained_cooling_sensible_output_owned_read,
        snapshot.predecessor_cooling_sensible_output_read,
        snapshot.predecessor_cp387_retained_cp_air_owned_read,
        snapshot.predecessor_cp_air_read,
        snapshot.predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        snapshot.predecessor_supply_mass_flow_rate_read,
        snapshot.predecessor_cp_air_times_supply_mass_flow_rate_calculated,
        snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_calculated,
        snapshot.predecessor_supply_temperature_calculated,
        snapshot.predecessor_supply_temperature_assigned,
    ]
}

fn cp390_flags(snapshot: Snapshot) -> [bool; 9] {
    [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed,
        snapshot.cp389_retained_supply_temperature_state_owned,
        snapshot.cp389_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_read,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.cp389_mixed_air_temperature_bit_corroborated,
        snapshot.mixed_air_temperature_for_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_temperature_assignment_performed,
    ]
}

fn predecessor_cp390_flags(snapshot: PredecessorSnapshot) -> [bool; 9] {
    [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed,
        snapshot.cp389_retained_supply_temperature_state_owned,
        snapshot.cp389_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_read,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.cp389_mixed_air_temperature_bit_corroborated,
        snapshot.mixed_air_temperature_for_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_temperature_assignment_performed,
    ]
}

fn cp391_flags(snapshot: Snapshot) -> [bool; 9] {
    [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed,
        snapshot.cp390_retained_supply_enthalpy_state_owned,
        snapshot.cp390_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_overdrying_limit_maximum_read,
        snapshot.cp390_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read,
        snapshot.psychrometric_minimum_supply_enthalpy_evaluated,
        snapshot.source_shaped_two_argument_maximum_evaluated,
        snapshot.supply_enthalpy_assignment_performed,
    ]
}

fn predecessor_cp391_flags(snapshot: PredecessorSnapshot) -> [bool; 9] {
    [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed,
        snapshot.cp390_retained_supply_enthalpy_state_owned,
        snapshot.cp390_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_overdrying_limit_maximum_read,
        snapshot.cp390_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read,
        snapshot.psychrometric_minimum_supply_enthalpy_evaluated,
        snapshot.source_shaped_two_argument_maximum_evaluated,
        snapshot.supply_enthalpy_assignment_performed,
    ]
}

fn predecessor_values(snapshot: Snapshot) -> [Option<f64>; 33] {
    [
        snapshot.predecessor_mixed_air_humidity_ratio,
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        snapshot.predecessor_cp_air_j_per_kg_k,
        snapshot.predecessor_cooling_total_output_w,
        snapshot.predecessor_cooling_sensible_heat_ratio,
        snapshot.predecessor_calculated_cooling_sensible_output_w,
        snapshot.predecessor_cooling_sensible_output_w,
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_preexisting_supply_temperature_c,
        snapshot.predecessor_mixed_air_temperature_c,
        snapshot.predecessor_cp389_cooling_sensible_output_w,
        snapshot.predecessor_cp389_cp_air_j_per_kg_k,
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s,
        snapshot.predecessor_cp_air_times_supply_mass_flow_rate_w_per_k,
        snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_k,
        snapshot.predecessor_calculated_supply_temperature_c,
        snapshot.predecessor_assigned_supply_temperature_c,
        snapshot.predecessor_resulting_supply_temperature_c,
        snapshot.predecessor_cp390_resulting_supply_enthalpy_j_per_kg,
        snapshot.preexisting_supply_temperature_c,
        snapshot.supply_temperature_before_mixed_air_limit_c,
        snapshot.mixed_air_temperature_c,
        snapshot.minimum_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.predecessor_cp390_resulting_supply_temperature_c,
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg,
        snapshot.predecessor_cp391_supply_temperature_c,
        snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg,
        snapshot.maximum_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp391_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp391_resulting_supply_temperature_c,
    ]
}

fn predecessor_snapshot_values(snapshot: PredecessorSnapshot) -> [Option<f64>; 33] {
    [
        snapshot.predecessor_mixed_air_humidity_ratio,
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        snapshot.predecessor_cp_air_j_per_kg_k,
        snapshot.predecessor_cooling_total_output_w,
        snapshot.predecessor_cooling_sensible_heat_ratio,
        snapshot.predecessor_calculated_cooling_sensible_output_w,
        snapshot.predecessor_cooling_sensible_output_w,
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_preexisting_supply_temperature_c,
        snapshot.predecessor_mixed_air_temperature_c,
        snapshot.predecessor_cp389_cooling_sensible_output_w,
        snapshot.predecessor_cp389_cp_air_j_per_kg_k,
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s,
        snapshot.predecessor_cp_air_times_supply_mass_flow_rate_w_per_k,
        snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_k,
        snapshot.predecessor_calculated_supply_temperature_c,
        snapshot.predecessor_assigned_supply_temperature_c,
        snapshot.predecessor_resulting_supply_temperature_c,
        snapshot.predecessor_cp390_resulting_supply_enthalpy_j_per_kg,
        snapshot.preexisting_supply_temperature_c,
        snapshot.supply_temperature_before_mixed_air_limit_c,
        snapshot.mixed_air_temperature_c,
        snapshot.minimum_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
        snapshot.predecessor_cp390_resulting_supply_temperature_c,
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg,
        snapshot.supply_temperature_c,
        snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg,
        snapshot.maximum_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
    ]
}

pub(super) fn same_snapshot(mut left: Snapshot, mut right: Snapshot) -> bool {
    let values_match = snapshot_values(left)
        .into_iter()
        .zip(snapshot_values(right))
        .all(|(left, right)| option_bits_equal(left, right));
    clear_values(&mut left);
    clear_values(&mut right);
    values_match && left == right
}

fn snapshot_values(snapshot: Snapshot) -> [Option<f64>; 40] {
    let predecessor = predecessor_values(snapshot);
    [
        predecessor[0],
        predecessor[1],
        predecessor[2],
        predecessor[3],
        predecessor[4],
        predecessor[5],
        predecessor[6],
        predecessor[7],
        predecessor[8],
        predecessor[9],
        predecessor[10],
        predecessor[11],
        predecessor[12],
        predecessor[13],
        predecessor[14],
        predecessor[15],
        predecessor[16],
        predecessor[17],
        predecessor[18],
        predecessor[19],
        predecessor[20],
        predecessor[21],
        predecessor[22],
        predecessor[23],
        predecessor[24],
        predecessor[25],
        predecessor[26],
        predecessor[27],
        predecessor[28],
        predecessor[29],
        predecessor[30],
        predecessor[31],
        predecessor[32],
        snapshot.supply_temperature_c,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.psychrometric_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn clear_values(snapshot: &mut Snapshot) {
    snapshot.predecessor_mixed_air_humidity_ratio = None;
    snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k = None;
    snapshot.predecessor_cp_air_j_per_kg_k = None;
    snapshot.predecessor_cooling_total_output_w = None;
    snapshot.predecessor_cooling_sensible_heat_ratio = None;
    snapshot.predecessor_calculated_cooling_sensible_output_w = None;
    snapshot.predecessor_cooling_sensible_output_w = None;
    snapshot.predecessor_resulting_supply_enthalpy_j_per_kg = None;
    snapshot.predecessor_preexisting_supply_temperature_c = None;
    snapshot.predecessor_mixed_air_temperature_c = None;
    snapshot.predecessor_cp389_cooling_sensible_output_w = None;
    snapshot.predecessor_cp389_cp_air_j_per_kg_k = None;
    snapshot.predecessor_supply_mass_flow_rate_kg_per_s = None;
    snapshot.predecessor_cp_air_times_supply_mass_flow_rate_w_per_k = None;
    snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_k = None;
    snapshot.predecessor_calculated_supply_temperature_c = None;
    snapshot.predecessor_assigned_supply_temperature_c = None;
    snapshot.predecessor_resulting_supply_temperature_c = None;
    snapshot.predecessor_cp390_resulting_supply_enthalpy_j_per_kg = None;
    snapshot.preexisting_supply_temperature_c = None;
    snapshot.supply_temperature_before_mixed_air_limit_c = None;
    snapshot.mixed_air_temperature_c = None;
    snapshot.minimum_supply_temperature_c = None;
    snapshot.assigned_supply_temperature_c = None;
    snapshot.predecessor_cp390_resulting_supply_temperature_c = None;
    snapshot.preexisting_supply_enthalpy_j_per_kg = None;
    snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg = None;
    snapshot.predecessor_cp391_supply_temperature_c = None;
    snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg = None;
    snapshot.maximum_supply_enthalpy_j_per_kg = None;
    snapshot.assigned_supply_enthalpy_j_per_kg = None;
    snapshot.predecessor_cp391_resulting_supply_enthalpy_j_per_kg = None;
    snapshot.predecessor_cp391_resulting_supply_temperature_c = None;
    snapshot.supply_temperature_c = None;
    snapshot.supply_enthalpy_j_per_kg = None;
    snapshot.psychrometric_supply_humidity_ratio = None;
    snapshot.assigned_supply_humidity_ratio = None;
    snapshot.resulting_supply_humidity_ratio = None;
    snapshot.resulting_supply_enthalpy_j_per_kg = None;
    snapshot.resulting_supply_temperature_c = None;
}

pub(super) fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
