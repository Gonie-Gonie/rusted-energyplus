//! Exact CP400-to-CP401 lineage and CP384/CP385 total-output ownership.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Owner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Corroborator,
};

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && control_flags(snapshot) == predecessor_control_flags(predecessor)
        && inherited_flags(snapshot) == predecessor_inherited_flags(predecessor)
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
    if !owner_links(snapshot, owner)
        || !corroborator_links(snapshot, corroborator)
        || owner_control_flags(owner) != operation_control_flags(snapshot)
        || corroborator_control_flags(corroborator) != operation_control_flags(snapshot)
        || snapshot.predecessor_supply_enthalpy_assignment_executed
            != corroborator.supply_enthalpy_assignment_executed
    {
        return false;
    }
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed;
    if local_flags(snapshot)
        .into_iter()
        .any(|value| value != active)
    {
        return false;
    }
    if !active {
        return local_values(snapshot)
            .into_iter()
            .all(|value| value.is_none());
    }
    if !owner.dehumidification_total_output_maximum_capacity_assignment_executed
        || !owner.cp383_retained_maximum_total_cooling_capacity_owned_read
        || !owner.cooling_total_output_assigned
        || !corroborator.supply_enthalpy_assignment_executed
        || !corroborator.cp384_retained_cooling_total_output_owned_read
        || !corroborator.cooling_total_output_read
        || !predecessor.cooling_sensible_output_calculated
        || !predecessor.cooling_sensible_output_assigned
    {
        return false;
    }
    let (Some(total), Some(sensible)) = (
        owner.resulting_cooling_total_output_w,
        predecessor.cooling_sensible_output_w,
    ) else {
        return false;
    };
    let latent = total - sensible;
    option_has_bits(owner.assigned_cooling_total_output_w, total)
        && option_has_bits(corroborator.cooling_total_output_w, total)
        && option_has_bits(snapshot.cooling_total_output_w, total)
        && option_has_bits(predecessor.calculated_cooling_sensible_output_w, sensible)
        && option_has_bits(snapshot.cooling_sensible_output_w, sensible)
        && option_has_bits(snapshot.calculated_cooling_latent_output_w, latent)
        && option_has_bits(snapshot.cooling_latent_output_w, latent)
        && option_bits_equal(
            corroborator.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
}

pub(super) fn carriers_are_preserved(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    snapshot.cp400_retained_supply_humidity_ratio_state_owned
        == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp400_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp400_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && carrier_matches(
            snapshot.predecessor_cp400_resulting_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && carrier_matches(
            snapshot.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && carrier_matches(
            snapshot.predecessor_cp400_resulting_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

fn owner_links(snapshot: Snapshot, owner: Owner) -> bool {
    owner.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        && owner.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && owner.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER
        && owner.system == snapshot.system
        && owner.parent_call_ordinal == snapshot.parent_call_ordinal
        && owner.controlled_zone == snapshot.controlled_zone
}

fn corroborator_links(snapshot: Snapshot, corroborator: Corroborator) -> bool {
    corroborator.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        && corroborator.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && corroborator.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
        && corroborator.system == snapshot.system
        && corroborator.parent_call_ordinal == snapshot.parent_call_ordinal
        && corroborator.controlled_zone == snapshot.controlled_zone
}

fn local_flags(snapshot: Snapshot) -> [bool; 8] {
    [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
        snapshot.cp384_retained_cooling_total_output_owned_read,
        snapshot.cp385_cooling_total_output_bit_corroborated,
        snapshot.cooling_total_output_read,
        snapshot.cp400_retained_cooling_sensible_output_owned_read,
        snapshot.cooling_sensible_output_read,
        snapshot.cooling_latent_output_calculated,
        snapshot.cooling_latent_output_assigned,
    ]
}

fn local_values(snapshot: Snapshot) -> [Option<f64>; 4] {
    [
        snapshot.cooling_total_output_w,
        snapshot.cooling_sensible_output_w,
        snapshot.calculated_cooling_latent_output_w,
        snapshot.cooling_latent_output_w,
    ]
}

fn inherited_values(snapshot: Snapshot) -> [Option<f64>; 23] {
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
    ]
}

fn predecessor_values(snapshot: Predecessor) -> [Option<f64>; 23] {
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
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.cp_air_j_per_kg_k,
        snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
        snapshot.mixed_air_temperature_c,
        snapshot.supply_temperature_c,
        snapshot.mixed_air_minus_supply_temperature_k,
        snapshot.calculated_cooling_sensible_output_w,
        snapshot.cooling_sensible_output_w,
        snapshot.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
    ]
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 22] {
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
    ]
}

fn predecessor_inherited_flags(snapshot: Predecessor) -> [bool; 22] {
    [
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        snapshot.predecessor_mixed_air_humidity_ratio_read,
        snapshot.predecessor_psychrometric_cp_air_evaluated,
        snapshot.predecessor_cp_air_assigned,
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
        snapshot.cp399_retained_supply_humidity_ratio_state_owned,
        snapshot.cp399_retained_supply_enthalpy_state_owned,
        snapshot.cp399_retained_supply_temperature_state_owned,
        snapshot.cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.cp329_supply_mass_flow_rate_bit_corroborated,
        snapshot.supply_mass_flow_rate_read,
        snapshot.cp399_retained_cp_air_owned_read,
        snapshot.cp_air_read,
        snapshot.supply_mass_flow_rate_times_cp_air_calculated,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.mixed_air_temperature_read,
        snapshot.cp399_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_read,
        snapshot.mixed_air_minus_supply_temperature_calculated,
        snapshot.cooling_sensible_output_calculated,
        snapshot.cooling_sensible_output_assigned,
    ]
}

fn control_flags(snapshot: Snapshot) -> [bool; 29] {
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

fn operation_control_flags(snapshot: Snapshot) -> [bool; 20] {
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

fn owner_control_flags(snapshot: Owner) -> [bool; 20] {
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

fn corroborator_control_flags(snapshot: Corroborator) -> [bool; 20] {
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

fn carrier_matches(first: Option<f64>, second: Option<f64>, third: Option<f64>) -> bool {
    option_bits_equal(first, third) && option_bits_equal(second, third)
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
