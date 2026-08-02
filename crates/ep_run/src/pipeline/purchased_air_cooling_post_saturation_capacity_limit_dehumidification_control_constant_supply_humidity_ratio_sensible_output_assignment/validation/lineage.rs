//! Exact CP399-to-CP400 lineage and CP330/CP329 operand ownership.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as FlowOwner,
};

pub(super) fn links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    predecessor.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && control_flags(snapshot) == predecessor_control_flags(predecessor)
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_dehumidification_control_none_case_entered
            == predecessor.predecessor_dehumidification_control_none_case_entered
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
            == predecessor
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed
            == predecessor
                .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed
        && snapshot.predecessor_mixed_air_humidity_ratio_read
            == predecessor.mixed_air_humidity_ratio_read
        && snapshot.predecessor_psychrometric_cp_air_evaluated
            == predecessor.psychrometric_cp_air_evaluated
        && snapshot.predecessor_cp_air_assigned == predecessor.cp_air_assigned
        && recursive_values(snapshot)
            .into_iter()
            .zip(predecessor_values(predecessor))
            .all(|(left, right)| option_bits_equal(left, right))
}

pub(super) fn operation_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    flow_owner: FlowOwner,
    mixed_owner: MixedOwner,
) -> bool {
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed;
    if local_bools(snapshot)
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
    let (Some(flow), Some(cp_air), Some(mixed_temperature), Some(supply_temperature)) = (
        flow_owner.supply_mass_flow_rate_kg_per_s,
        predecessor.cp_air_j_per_kg_k,
        mixed_owner.mixed_air_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) else {
        return false;
    };
    let first_product = flow * cp_air;
    let difference = mixed_temperature - supply_temperature;
    let output = first_product * difference;
    flow_owner_links(snapshot, flow_owner)
        && mixed_owner_links(snapshot, mixed_owner)
        && option_has_bits(mixed_owner.supply_mass_flow_rate_kg_per_s, flow)
        && option_has_bits(mixed_owner.child_supply_mass_flow_rate_kg_per_s, flow)
        && flow > 0.0
        && !flow.is_nan()
        && cp_air.is_finite()
        && option_has_bits(snapshot.supply_mass_flow_rate_kg_per_s, flow)
        && option_has_bits(snapshot.cp_air_j_per_kg_k, cp_air)
        && option_has_bits(
            snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
            first_product,
        )
        && option_has_bits(snapshot.mixed_air_temperature_c, mixed_temperature)
        && option_has_bits(snapshot.supply_temperature_c, supply_temperature)
        && option_has_bits(snapshot.mixed_air_minus_supply_temperature_k, difference)
        && option_has_bits(snapshot.calculated_cooling_sensible_output_w, output)
        && option_has_bits(snapshot.cooling_sensible_output_w, output)
}

pub(super) fn carriers_are_preserved(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    snapshot.cp399_retained_supply_humidity_ratio_state_owned
        == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp399_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp399_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
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

fn flow_owner_links(snapshot: Snapshot, owner: FlowOwner) -> bool {
    owner.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        && owner.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        && owner.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER
        && owner.system == snapshot.system
        && owner.parent_call_ordinal == snapshot.parent_call_ordinal
        && owner.controlled_zone == snapshot.controlled_zone
        && owner.cooling_body_entered
        && owner.supply_mass_flow_rate_read
        && owner.supply_mass_flow_rate_strictly_positive_comparison_evaluated
        && owner.supply_mass_flow_rate_strictly_positive == Some(true)
        && owner.positive_supply_mass_flow_body_entered
}

fn mixed_owner_links(snapshot: Snapshot, owner: MixedOwner) -> bool {
    owner.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && owner.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && owner.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && owner.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && owner.no_oa_child_source_order
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER
        && owner.system == snapshot.system
        && owner.parent_call_ordinal == snapshot.parent_call_ordinal
        && owner.controlled_zone == snapshot.controlled_zone
        && owner.cooling_call_executed
        && owner.no_outdoor_air_fallback_entered
        && owner.mixed_air_temperature_assigned
}

fn local_bools(snapshot: Snapshot) -> [bool; 14] {
    [
        snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
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

fn local_values(snapshot: Snapshot) -> [Option<f64>; 8] {
    [
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.cp_air_j_per_kg_k,
        snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
        snapshot.mixed_air_temperature_c,
        snapshot.supply_temperature_c,
        snapshot.mixed_air_minus_supply_temperature_k,
        snapshot.calculated_cooling_sensible_output_w,
        snapshot.cooling_sensible_output_w,
    ]
}

fn recursive_values(snapshot: Snapshot) -> [Option<f64>; 12] {
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
    ]
}

fn predecessor_values(snapshot: Predecessor) -> [Option<f64>; 12] {
    [
        snapshot.predecessor_cp397_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp397_resulting_supply_temperature_c,
        snapshot.predecessor_cp398_resulting_supply_humidity_ratio,
        snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        snapshot.predecessor_cp398_resulting_supply_temperature_c,
        snapshot.mixed_air_humidity_ratio,
        snapshot.psychrometric_cp_air_result_j_per_kg_k,
        snapshot.cp_air_j_per_kg_k,
        snapshot.resulting_supply_humidity_ratio,
        snapshot.resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_temperature_c,
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
