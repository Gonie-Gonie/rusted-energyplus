//! Exact CP401-to-CP402 lineage and CP321/CP340 maximum-capacity ownership.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot as Owner,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot as Corroborator,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
};

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && inherited_control_flags(snapshot) == predecessor_control_flags(predecessor)
        && inherited_operation_flags(snapshot) == predecessor_operation_flags(predecessor)
        && inherited_values(snapshot)
            .into_iter()
            .zip(predecessor_values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
}

pub(super) fn operation_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    owner: Owner,
    corroborator: Corroborator,
) -> bool {
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed;
    if local_common_flags(snapshot)
        .into_iter()
        .any(|flag| flag != active)
    {
        return false;
    }
    if !active {
        return snapshot.cooling_latent_output_w.is_none()
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && snapshot
                .cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity
                .is_none()
            && !snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered
            && !snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough;
    }
    if !predecessor.cooling_latent_output_assigned
        || !same_call(snapshot, owner, corroborator)
        || owner.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || owner.source_order != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER
        || corroborator.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        || corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || corroborator.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER
        || !owner.cooling_body_entered
        || owner.cooling_limit_condition_satisfied != Some(true)
        || !owner.maximum_total_cooling_capacity_read
        || !owner.maximum_total_cooling_capacity_comparison_evaluated
        || owner.maximum_total_cooling_capacity_equal_to_zero != Some(false)
        || owner.zero_cooling_capacity_body_entered
        || !corroborator.capacity_limit_sensible_output_guard_evaluated
        || !corroborator.maximum_total_cooling_capacity_read
    {
        return false;
    }
    let (Some(cooling_latent_output_w), Some(maximum_total_cooling_capacity_w)) = (
        predecessor.cooling_latent_output_w,
        owner.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    if !maximum_total_cooling_capacity_w.is_finite()
        || maximum_total_cooling_capacity_w < 0.0
        || !option_has_bits(snapshot.cooling_latent_output_w, cooling_latent_output_w)
        || !option_has_bits(
            snapshot.maximum_total_cooling_capacity_w,
            maximum_total_cooling_capacity_w,
        )
        || !option_has_bits(
            corroborator.maximum_total_cooling_capacity_w,
            maximum_total_cooling_capacity_w,
        )
    {
        return false;
    }
    let comparison = cooling_latent_output_w >= maximum_total_cooling_capacity_w;
    snapshot.cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity
        == Some(comparison)
        && snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered
            == comparison
        && snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough
            != comparison
}

pub(super) fn carriers_are_preserved(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    snapshot.cp401_retained_supply_humidity_ratio_state_owned
        == predecessor.cp400_retained_supply_humidity_ratio_state_owned
        && snapshot.cp401_retained_supply_enthalpy_state_owned
            == predecessor.cp400_retained_supply_enthalpy_state_owned
        && snapshot.cp401_retained_supply_temperature_state_owned
            == predecessor.cp400_retained_supply_temperature_state_owned
        && option_bits_equal(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

fn same_call(snapshot: Snapshot, owner: Owner, corroborator: Corroborator) -> bool {
    [owner.system, corroborator.system]
        .into_iter()
        .all(|system| system == snapshot.system)
        && [owner.parent_call_ordinal, corroborator.parent_call_ordinal]
            .into_iter()
            .all(|ordinal| ordinal == snapshot.parent_call_ordinal)
        && [owner.controlled_zone, corroborator.controlled_zone]
            .into_iter()
            .all(|zone| zone == snapshot.controlled_zone)
}

fn local_common_flags(snapshot: Snapshot) -> [bool; 7] {
    [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated,
        snapshot.cp401_retained_cooling_latent_output_owned_read,
        snapshot.cooling_latent_output_read,
        snapshot.cp321_maximum_total_cooling_capacity_owned_read,
        snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
        snapshot.maximum_total_cooling_capacity_read,
        snapshot.cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated,
    ]
}

fn inherited_values(snapshot: Snapshot) -> [Option<f64>; 30] {
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
    ]
}

fn predecessor_values(snapshot: Predecessor) -> [Option<f64>; 30] {
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
        snapshot.cooling_total_output_w,
        snapshot.cooling_sensible_output_w,
        snapshot.calculated_cooling_latent_output_w,
        snapshot.cooling_latent_output_w,
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
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
        snapshot.cp400_retained_supply_humidity_ratio_state_owned,
        snapshot.cp400_retained_supply_enthalpy_state_owned,
        snapshot.cp400_retained_supply_temperature_state_owned,
        snapshot.cp384_retained_cooling_total_output_owned_read,
        snapshot.cp385_cooling_total_output_bit_corroborated,
        snapshot.cooling_total_output_read,
        snapshot.cooling_latent_output_calculated,
        snapshot.cooling_latent_output_assigned,
        snapshot.cp400_retained_cooling_sensible_output_owned_read,
        snapshot.cooling_sensible_output_read,
    ]
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
