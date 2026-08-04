//! CP415 predecessor reconstruction and bit-exact snapshot comparison.

use super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot as Snapshot;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot as Predecessor;

pub(super) fn cp414_shape(snapshot: Snapshot) -> Predecessor {
    use crate::ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as PREDECESSOR_EXCLUDED,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE as PREDECESSOR_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER as PREDECESSOR_ORDER,
    };
    Predecessor {
        source: PREDECESSOR_SOURCE,
        first_excluded_source: PREDECESSOR_EXCLUDED,
        source_order: PREDECESSOR_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot.predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot.predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot.predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        predecessor_cp409_resulting_supply_humidity_ratio: snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp409_resulting_supply_temperature_c: snapshot.predecessor_cp409_resulting_supply_temperature_c,
        predecessor_dehumidification_control_default_case_exited_via_break: snapshot.predecessor_dehumidification_control_default_case_exited_via_break,
        predecessor_cp410_resulting_supply_humidity_ratio: snapshot.predecessor_cp410_resulting_supply_humidity_ratio,
        predecessor_cp410_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp410_resulting_supply_temperature_c: snapshot.predecessor_cp410_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed: snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        cp410_retained_supply_humidity_ratio_state_owned: snapshot.cp410_retained_supply_humidity_ratio_state_owned,
        cp410_retained_supply_enthalpy_state_owned: snapshot.cp410_retained_supply_enthalpy_state_owned,
        cp410_retained_supply_temperature_state_owned: snapshot.cp410_retained_supply_temperature_state_owned,
        cp410_retained_supply_humidity_ratio_owned_read: snapshot.cp410_retained_supply_humidity_ratio_owned_read,
        purchased_air_supply_humidity_ratio_read: snapshot.purchased_air_supply_humidity_ratio_read,
        purchased_air_supply_humidity_ratio_before_saturation_check: snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
        local_supply_humidity_ratio_original_assignment_performed: snapshot.local_supply_humidity_ratio_original_assignment_performed,
        assigned_supply_humidity_ratio_original: snapshot.assigned_supply_humidity_ratio_original,
        resulting_supply_humidity_ratio_original: snapshot.resulting_supply_humidity_ratio_original,
        predecessor_cp411_resulting_supply_humidity_ratio: snapshot.predecessor_cp411_resulting_supply_humidity_ratio,
        predecessor_cp411_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp411_resulting_supply_temperature_c: snapshot.predecessor_cp411_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed: snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed,
        cp411_retained_supply_humidity_ratio_state_owned: snapshot.cp411_retained_supply_humidity_ratio_state_owned,
        cp411_retained_supply_enthalpy_state_owned: snapshot.cp411_retained_supply_enthalpy_state_owned,
        cp411_retained_supply_temperature_state_owned: snapshot.cp411_retained_supply_temperature_state_owned,
        cp411_retained_supply_temperature_owned_read: snapshot.cp411_retained_supply_temperature_owned_read,
        purchased_air_supply_temperature_for_saturation_humidity_ratio_read: snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
        supply_temperature_for_saturation_humidity_ratio_c: snapshot.supply_temperature_for_saturation_humidity_ratio_c,
        environment_outdoor_barometric_pressure_owned_read: snapshot.environment_outdoor_barometric_pressure_owned_read,
        environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
        outdoor_barometric_pressure_pa: snapshot.outdoor_barometric_pressure_pa,
        psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
        saturation_supply_humidity_ratio: snapshot.saturation_supply_humidity_ratio,
        local_saturation_supply_humidity_ratio_assignment_performed: snapshot.local_saturation_supply_humidity_ratio_assignment_performed,
        assigned_saturation_supply_humidity_ratio: snapshot.assigned_saturation_supply_humidity_ratio,
        resulting_saturation_supply_humidity_ratio: snapshot.resulting_saturation_supply_humidity_ratio,
        predecessor_cp412_resulting_supply_humidity_ratio: snapshot.predecessor_cp412_resulting_supply_humidity_ratio,
        predecessor_cp412_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp412_resulting_supply_temperature_c: snapshot.predecessor_cp412_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated: snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated,
        cp412_saturation_supply_humidity_ratio_owned_read: snapshot.cp412_saturation_supply_humidity_ratio_owned_read,
        saturation_supply_humidity_ratio_for_guard_read: snapshot.saturation_supply_humidity_ratio_for_guard_read,
        saturation_supply_humidity_ratio_for_guard: snapshot.saturation_supply_humidity_ratio_for_guard,
        cp411_original_supply_humidity_ratio_owned_read: snapshot.cp411_original_supply_humidity_ratio_owned_read,
        cp412_same_call_original_supply_humidity_ratio_bit_corroborated: snapshot.cp412_same_call_original_supply_humidity_ratio_bit_corroborated,
        original_supply_humidity_ratio_for_guard_read: snapshot.original_supply_humidity_ratio_for_guard_read,
        original_supply_humidity_ratio_for_guard: snapshot.original_supply_humidity_ratio_for_guard,
        saturation_original_supply_humidity_ratio_comparison_evaluated: snapshot.saturation_original_supply_humidity_ratio_comparison_evaluated,
        saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio: snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
        saturation_supply_humidity_ratio_guard_body_entered: snapshot.saturation_supply_humidity_ratio_guard_body_entered,
        saturation_supply_humidity_ratio_guard_false_fallthrough: snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough,
        cp412_retained_supply_humidity_ratio_state_owned: snapshot.cp412_retained_supply_humidity_ratio_state_owned,
        cp412_retained_supply_enthalpy_state_owned: snapshot.cp412_retained_supply_enthalpy_state_owned,
        cp412_retained_supply_temperature_state_owned: snapshot.cp412_retained_supply_temperature_state_owned,
        predecessor_cp413_resulting_supply_humidity_ratio: snapshot.predecessor_cp413_resulting_supply_humidity_ratio,
        predecessor_cp413_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp413_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp413_resulting_supply_temperature_c: snapshot.predecessor_cp413_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed: snapshot.post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed,
        cp413_retained_supply_humidity_ratio_state_owned: snapshot.cp413_retained_supply_humidity_ratio_state_owned,
        cp413_retained_supply_enthalpy_state_owned: snapshot.cp413_retained_supply_enthalpy_state_owned,
        cp413_retained_supply_temperature_state_owned: snapshot.cp413_retained_supply_temperature_state_owned,
        cp413_retained_supply_enthalpy_owned_read: snapshot.cp413_retained_supply_enthalpy_owned_read,
        supply_enthalpy_for_saturation_temperature_read: snapshot.supply_enthalpy_for_saturation_temperature_read,
        supply_enthalpy_for_saturation_temperature_j_per_kg: snapshot.supply_enthalpy_for_saturation_temperature_j_per_kg,
        environment_outdoor_barometric_pressure_for_saturation_temperature_owned_read: snapshot.environment_outdoor_barometric_pressure_for_saturation_temperature_owned_read,
        environment_outdoor_barometric_pressure_for_saturation_temperature_read: snapshot.environment_outdoor_barometric_pressure_for_saturation_temperature_read,
        outdoor_barometric_pressure_for_saturation_temperature_pa: snapshot.outdoor_barometric_pressure_for_saturation_temperature_pa,
        psy_tsat_fn_h_pb_evaluated: snapshot.psy_tsat_fn_h_pb_evaluated,
        psychrometric_saturation_supply_temperature_result_c: snapshot.psychrometric_saturation_supply_temperature_result_c,
        purchased_air_supply_temperature_saturation_assignment_performed: snapshot.purchased_air_supply_temperature_saturation_assignment_performed,
        assigned_saturation_supply_temperature_c: snapshot.assigned_saturation_supply_temperature_c,
        resulting_supply_humidity_ratio: snapshot.predecessor_cp414_resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp414_resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: snapshot.predecessor_cp414_resulting_supply_temperature_c,
    }
}

pub(super) fn snapshots_match_bit_exact(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! compare_clear {
        ($field:ident) => {{
            let matches = option_bits_match(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }
    let values_match = compare_clear!(predecessor_cp409_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp409_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp409_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp410_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp410_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp410_resulting_supply_temperature_c)
        && compare_clear!(purchased_air_supply_humidity_ratio_before_saturation_check)
        && compare_clear!(assigned_supply_humidity_ratio_original)
        && compare_clear!(resulting_supply_humidity_ratio_original)
        && compare_clear!(predecessor_cp411_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp411_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp411_resulting_supply_temperature_c)
        && compare_clear!(supply_temperature_for_saturation_humidity_ratio_c)
        && compare_clear!(outdoor_barometric_pressure_pa)
        && compare_clear!(saturation_supply_humidity_ratio)
        && compare_clear!(assigned_saturation_supply_humidity_ratio)
        && compare_clear!(resulting_saturation_supply_humidity_ratio)
        && compare_clear!(predecessor_cp412_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp412_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp412_resulting_supply_temperature_c)
        && compare_clear!(saturation_supply_humidity_ratio_for_guard)
        && compare_clear!(original_supply_humidity_ratio_for_guard)
        && compare_clear!(predecessor_cp413_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp413_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp413_resulting_supply_temperature_c)
        && compare_clear!(supply_enthalpy_for_saturation_temperature_j_per_kg)
        && compare_clear!(outdoor_barometric_pressure_for_saturation_temperature_pa)
        && compare_clear!(psychrometric_saturation_supply_temperature_result_c)
        && compare_clear!(assigned_saturation_supply_temperature_c)
        && compare_clear!(predecessor_cp414_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp414_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp414_resulting_supply_temperature_c)
        && compare_clear!(preexisting_supply_temperature_c)
        && compare_clear!(supply_temperature_before_mixed_air_limit_c)
        && compare_clear!(mixed_air_temperature_c)
        && compare_clear!(minimum_supply_temperature_c)
        && compare_clear!(assigned_supply_temperature_c)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
