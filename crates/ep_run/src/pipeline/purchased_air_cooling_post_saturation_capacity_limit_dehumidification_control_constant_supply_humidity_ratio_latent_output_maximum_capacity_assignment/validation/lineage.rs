//! Exact CP404-to-CP405 lineage and source-operation validation.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Predecessor,
};

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && inherited_control_flags(snapshot) == predecessor_control_flags(predecessor)
        && inherited_operation_flags(snapshot) == predecessor_operation_flags(predecessor)
        && inherited_guard_flags(snapshot) == predecessor_guard_flags(predecessor)
        && inherited_cp403_flags(snapshot) == predecessor_cp403_flags(predecessor)
        && inherited_cp404_flags(snapshot) == predecessor_cp404_flags(predecessor)
        && snapshot
            .predecessor_cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity
            == predecessor
                .predecessor_cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity
        && inherited_values(snapshot)
            .into_iter()
            .zip(predecessor_values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
}

pub(super) fn operation_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let assignment = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed;
    let guard_evaluated = predecessor
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated;
    if local_flags(snapshot)
        .into_iter()
        .any(|flag| flag != assignment)
        || snapshot.cp404_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp404_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp404_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !option_bits_equal(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        || !option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_equal(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
    {
        return false;
    }
    if !guard_evaluated {
        return [
            snapshot.preexisting_cooling_latent_output_w,
            snapshot.maximum_total_cooling_capacity_w,
            snapshot.assigned_cooling_latent_output_w,
            snapshot.resulting_cooling_latent_output_w,
        ]
        .into_iter()
        .all(|value| value.is_none());
    }
    let Some(preexisting) = predecessor.predecessor_cp402_cooling_latent_output_w else {
        return false;
    };
    if !option_bits_equal(
        snapshot.preexisting_cooling_latent_output_w,
        Some(preexisting),
    ) {
        return false;
    }
    if !assignment {
        return snapshot.maximum_total_cooling_capacity_w.is_none()
            && snapshot.assigned_cooling_latent_output_w.is_none()
            && option_bits_equal(
                snapshot.resulting_cooling_latent_output_w,
                Some(preexisting),
            );
    }
    let Some(maximum) = predecessor.predecessor_maximum_total_cooling_capacity_w else {
        return false;
    };
    option_bits_equal(snapshot.maximum_total_cooling_capacity_w, Some(maximum))
        && option_bits_equal(snapshot.assigned_cooling_latent_output_w, Some(maximum))
        && option_bits_equal(snapshot.resulting_cooling_latent_output_w, Some(maximum))
}

fn local_flags(snapshot: Snapshot) -> [bool; 4] {
    [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        snapshot.cp404_retained_maximum_total_cooling_capacity_owned_read,
        snapshot.maximum_total_cooling_capacity_read,
        snapshot.cooling_latent_output_assigned,
    ]
}

fn inherited_cp403_flags(snapshot: Snapshot) -> [bool; 8] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed,
        snapshot.predecessor_cp403_cp329_retained_mixed_air_temperature_owned_read,
        snapshot.predecessor_cp402_same_call_mixed_air_temperature_bit_corroborated,
        snapshot.predecessor_cp403_mixed_air_temperature_read,
        snapshot.predecessor_supply_temperature_assigned,
        snapshot.predecessor_cp402_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp402_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp402_retained_supply_temperature_state_owned,
    ]
}

fn predecessor_cp403_flags(snapshot: Predecessor) -> [bool; 8] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed,
        snapshot.predecessor_cp403_cp329_retained_mixed_air_temperature_owned_read,
        snapshot.predecessor_cp402_same_call_mixed_air_temperature_bit_corroborated,
        snapshot.predecessor_cp403_mixed_air_temperature_read,
        snapshot.predecessor_supply_temperature_assigned,
        snapshot.predecessor_cp402_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp402_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp402_retained_supply_temperature_state_owned,
    ]
}

fn inherited_cp404_flags(snapshot: Snapshot) -> [bool; 10] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_cp403_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp403_retained_supply_temperature_state_owned,
        snapshot.predecessor_cp403_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp404_cp403_retained_supply_temperature_owned_read,
        snapshot.predecessor_supply_temperature_for_humidity_ratio_inversion_read,
        snapshot.predecessor_cp404_cp403_retained_supply_enthalpy_owned_read,
        snapshot.predecessor_supply_enthalpy_for_humidity_ratio_inversion_read,
        snapshot.predecessor_psychrometric_supply_humidity_ratio_evaluated,
        snapshot.predecessor_supply_humidity_ratio_assignment_performed,
    ]
}

fn predecessor_cp404_flags(snapshot: Predecessor) -> [bool; 10] {
    [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed,
        snapshot.cp403_retained_supply_humidity_ratio_state_owned,
        snapshot.cp403_retained_supply_temperature_state_owned,
        snapshot.cp403_retained_supply_enthalpy_state_owned,
        snapshot.cp403_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_humidity_ratio_inversion_read,
        snapshot.cp403_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_humidity_ratio_inversion_read,
        snapshot.psychrometric_supply_humidity_ratio_evaluated,
        snapshot.supply_humidity_ratio_assignment_performed,
    ]
}

fn inherited_values(snapshot: Snapshot) -> [Option<f64>; 47] {
    [
        snapshot.predecessor_cp397_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp397_resulting_supply_temperature_c,
        snapshot.predecessor_cp398_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp398_resulting_supply_temperature_c,
        snapshot.predecessor_mixed_air_humidity_ratio,
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        snapshot.predecessor_cp_air_j_per_kg_k,
        snapshot.predecessor_cp399_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp399_resulting_supply_temperature_c,
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s,
        snapshot.predecessor_cp400_cp_air_j_per_kg_k,
        snapshot.predecessor_supply_mass_flow_rate_times_cp_air_w_per_k,
        snapshot.predecessor_mixed_air_temperature_c,
        snapshot.predecessor_supply_temperature_c,
        snapshot.predecessor_mixed_air_minus_supply_temperature_k,
        snapshot.predecessor_calculated_cooling_sensible_output_w,
        snapshot.predecessor_cooling_sensible_output_w,
        snapshot.predecessor_cp400_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp400_resulting_supply_temperature_c,
        snapshot.predecessor_cooling_total_output_w,
        snapshot.predecessor_cp401_cooling_sensible_output_w,
        snapshot.predecessor_calculated_cooling_latent_output_w,
        snapshot.predecessor_cooling_latent_output_w,
        snapshot.predecessor_cp401_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp401_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp401_resulting_supply_temperature_c,
        snapshot.predecessor_cp402_cooling_latent_output_w,
        snapshot.predecessor_maximum_total_cooling_capacity_w,
        snapshot.predecessor_cp402_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp402_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp402_resulting_supply_temperature_c,
        snapshot.predecessor_cp403_mixed_air_temperature_c,
        snapshot.predecessor_cp403_assigned_supply_temperature_c,
        snapshot.predecessor_cp403_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp403_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp403_resulting_supply_temperature_c,
        snapshot.predecessor_cp404_supply_temperature_c,
        snapshot.predecessor_cp404_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp404_psychrometric_supply_humidity_ratio,
        snapshot.predecessor_cp404_assigned_supply_humidity_ratio,
        snapshot.predecessor_cp404_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp404_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp404_resulting_supply_temperature_c,
    ]
}

fn predecessor_values(snapshot: Predecessor) -> [Option<f64>; 47] {
    [
        snapshot.predecessor_cp397_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp397_resulting_supply_temperature_c,
        snapshot.predecessor_cp398_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp398_resulting_supply_temperature_c,
        snapshot.predecessor_mixed_air_humidity_ratio,
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        snapshot.predecessor_cp_air_j_per_kg_k,
        snapshot.predecessor_cp399_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp399_resulting_supply_temperature_c,
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s,
        snapshot.predecessor_cp400_cp_air_j_per_kg_k,
        snapshot.predecessor_supply_mass_flow_rate_times_cp_air_w_per_k,
        snapshot.predecessor_mixed_air_temperature_c,
        snapshot.predecessor_supply_temperature_c,
        snapshot.predecessor_mixed_air_minus_supply_temperature_k,
        snapshot.predecessor_calculated_cooling_sensible_output_w,
        snapshot.predecessor_cooling_sensible_output_w,
        snapshot.predecessor_cp400_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp400_resulting_supply_temperature_c,
        snapshot.predecessor_cooling_total_output_w,
        snapshot.predecessor_cp401_cooling_sensible_output_w,
        snapshot.predecessor_calculated_cooling_latent_output_w,
        snapshot.predecessor_cooling_latent_output_w,
        snapshot.predecessor_cp401_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp401_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp401_resulting_supply_temperature_c,
        snapshot.predecessor_cp402_cooling_latent_output_w,
        snapshot.predecessor_maximum_total_cooling_capacity_w,
        snapshot.predecessor_cp402_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp402_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp402_resulting_supply_temperature_c,
        snapshot.predecessor_cp403_mixed_air_temperature_c,
        snapshot.predecessor_cp403_assigned_supply_temperature_c,
        snapshot.predecessor_cp403_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp403_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp403_resulting_supply_temperature_c,
        snapshot.supply_temperature_c,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.psychrometric_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn inherited_control_flags(snapshot: Snapshot) -> [bool; 29] {
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
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_none_case_entered,
    ]
}

fn predecessor_control_flags(snapshot: Predecessor) -> [bool; 29] {
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
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_none_case_entered,
    ]
}

fn inherited_operation_flags(snapshot: Snapshot) -> [bool; 33] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        snapshot.predecessor_mixed_air_humidity_ratio_read,
        snapshot.predecessor_psychrometric_cp_air_evaluated,
        snapshot.predecessor_cp_air_assigned,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
        snapshot.predecessor_cp399_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp399_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp399_retained_supply_temperature_state_owned,
        snapshot.predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        snapshot.predecessor_supply_mass_flow_rate_read,
        snapshot.predecessor_cp399_retained_cp_air_owned_read,
        snapshot.predecessor_cp_air_read,
        snapshot.predecessor_supply_mass_flow_rate_times_cp_air_calculated,
        snapshot.predecessor_cp329_retained_mixed_air_temperature_owned_read,
        snapshot.predecessor_mixed_air_temperature_read,
        snapshot.predecessor_cp399_retained_supply_temperature_owned_read,
        snapshot.predecessor_supply_temperature_read,
        snapshot.predecessor_mixed_air_minus_supply_temperature_calculated,
        snapshot.predecessor_cooling_sensible_output_calculated,
        snapshot.predecessor_cooling_sensible_output_assigned,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
        snapshot.predecessor_cp400_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp400_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp400_retained_supply_temperature_state_owned,
        snapshot.predecessor_cp384_retained_cooling_total_output_owned_read,
        snapshot.predecessor_cp385_cooling_total_output_bit_corroborated,
        snapshot.predecessor_cooling_total_output_read,
        snapshot.predecessor_cooling_latent_output_calculated,
        snapshot.predecessor_cooling_latent_output_assigned,
        snapshot.predecessor_cp400_retained_cooling_sensible_output_owned_read,
        snapshot.predecessor_cp401_cooling_sensible_output_read,
    ]
}

fn predecessor_operation_flags(snapshot: Predecessor) -> [bool; 33] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        snapshot.predecessor_mixed_air_humidity_ratio_read,
        snapshot.predecessor_psychrometric_cp_air_evaluated,
        snapshot.predecessor_cp_air_assigned,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
        snapshot.predecessor_cp399_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp399_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp399_retained_supply_temperature_state_owned,
        snapshot.predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        snapshot.predecessor_supply_mass_flow_rate_read,
        snapshot.predecessor_cp399_retained_cp_air_owned_read,
        snapshot.predecessor_cp_air_read,
        snapshot.predecessor_supply_mass_flow_rate_times_cp_air_calculated,
        snapshot.predecessor_cp329_retained_mixed_air_temperature_owned_read,
        snapshot.predecessor_mixed_air_temperature_read,
        snapshot.predecessor_cp399_retained_supply_temperature_owned_read,
        snapshot.predecessor_supply_temperature_read,
        snapshot.predecessor_mixed_air_minus_supply_temperature_calculated,
        snapshot.predecessor_cooling_sensible_output_calculated,
        snapshot.predecessor_cooling_sensible_output_assigned,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
        snapshot.predecessor_cp400_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp400_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp400_retained_supply_temperature_state_owned,
        snapshot.predecessor_cp384_retained_cooling_total_output_owned_read,
        snapshot.predecessor_cp385_cooling_total_output_bit_corroborated,
        snapshot.predecessor_cooling_total_output_read,
        snapshot.predecessor_cooling_latent_output_calculated,
        snapshot.predecessor_cooling_latent_output_assigned,
        snapshot.predecessor_cp400_retained_cooling_sensible_output_owned_read,
        snapshot.predecessor_cp401_cooling_sensible_output_read,
    ]
}

fn inherited_guard_flags(snapshot: Snapshot) -> [bool; 12] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated,
        snapshot.predecessor_cp401_retained_cooling_latent_output_owned_read,
        snapshot.predecessor_cooling_latent_output_read,
        snapshot.predecessor_cp321_maximum_total_cooling_capacity_owned_read,
        snapshot.predecessor_cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
        snapshot.predecessor_maximum_total_cooling_capacity_read,
        snapshot.predecessor_cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        snapshot.predecessor_cp401_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp401_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp401_retained_supply_temperature_state_owned,
    ]
}

fn predecessor_guard_flags(snapshot: Predecessor) -> [bool; 12] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated,
        snapshot.predecessor_cp401_retained_cooling_latent_output_owned_read,
        snapshot.predecessor_cooling_latent_output_read,
        snapshot.predecessor_cp321_maximum_total_cooling_capacity_owned_read,
        snapshot.predecessor_cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
        snapshot.predecessor_maximum_total_cooling_capacity_read,
        snapshot.predecessor_cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        snapshot.predecessor_cp401_retained_supply_humidity_ratio_state_owned,
        snapshot.predecessor_cp401_retained_supply_enthalpy_state_owned,
        snapshot.predecessor_cp401_retained_supply_temperature_state_owned,
    ]
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
