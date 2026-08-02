//! Lossless CP402 snapshot construction.

use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
};

pub(super) fn build_snapshot(
    predecessor: Predecessor,
    route: RetainedRoute,
    cooling_latent_output_w: Option<f64>,
    maximum_total_cooling_capacity_w: Option<f64>,
    comparison: Option<bool>,
) -> Snapshot {
    let active = route.active;
    Snapshot {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: predecessor
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor
            .predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: predecessor
            .predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: predecessor
            .predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: predecessor
            .predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: predecessor
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_cp397_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp397_resulting_supply_humidity_ratio,
        predecessor_cp397_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp397_resulting_supply_temperature_c: predecessor
            .predecessor_cp397_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_case_entered: predecessor
            .predecessor_dehumidification_control_none_case_entered,
        predecessor_cp398_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp398_resulting_supply_humidity_ratio,
        predecessor_cp398_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp398_resulting_supply_temperature_c: predecessor
            .predecessor_cp398_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        predecessor_mixed_air_humidity_ratio_read: predecessor
            .predecessor_mixed_air_humidity_ratio_read,
        predecessor_mixed_air_humidity_ratio: predecessor.predecessor_mixed_air_humidity_ratio,
        predecessor_psychrometric_cp_air_evaluated: predecessor
            .predecessor_psychrometric_cp_air_evaluated,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: predecessor
            .predecessor_psychrometric_cp_air_result_j_per_kg_k,
        predecessor_cp_air_assigned: predecessor.predecessor_cp_air_assigned,
        predecessor_cp_air_j_per_kg_k: predecessor.predecessor_cp_air_j_per_kg_k,
        predecessor_cp399_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp399_resulting_supply_humidity_ratio,
        predecessor_cp399_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp399_resulting_supply_temperature_c: predecessor
            .predecessor_cp399_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
        predecessor_cp399_retained_supply_humidity_ratio_state_owned: predecessor
            .predecessor_cp399_retained_supply_humidity_ratio_state_owned,
        predecessor_cp399_retained_supply_enthalpy_state_owned: predecessor
            .predecessor_cp399_retained_supply_enthalpy_state_owned,
        predecessor_cp399_retained_supply_temperature_state_owned: predecessor
            .predecessor_cp399_retained_supply_temperature_state_owned,
        predecessor_cp330_retained_supply_mass_flow_rate_owned_read: predecessor
            .predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        predecessor_cp329_supply_mass_flow_rate_bit_corroborated: predecessor
            .predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        predecessor_supply_mass_flow_rate_read: predecessor.predecessor_supply_mass_flow_rate_read,
        predecessor_supply_mass_flow_rate_kg_per_s: predecessor
            .predecessor_supply_mass_flow_rate_kg_per_s,
        predecessor_cp399_retained_cp_air_owned_read: predecessor
            .predecessor_cp399_retained_cp_air_owned_read,
        predecessor_cp_air_read: predecessor.predecessor_cp_air_read,
        predecessor_cp400_cp_air_j_per_kg_k: predecessor.predecessor_cp400_cp_air_j_per_kg_k,
        predecessor_supply_mass_flow_rate_times_cp_air_calculated: predecessor
            .predecessor_supply_mass_flow_rate_times_cp_air_calculated,
        predecessor_supply_mass_flow_rate_times_cp_air_w_per_k: predecessor
            .predecessor_supply_mass_flow_rate_times_cp_air_w_per_k,
        predecessor_cp329_retained_mixed_air_temperature_owned_read: predecessor
            .predecessor_cp329_retained_mixed_air_temperature_owned_read,
        predecessor_mixed_air_temperature_read: predecessor.predecessor_mixed_air_temperature_read,
        predecessor_mixed_air_temperature_c: predecessor.predecessor_mixed_air_temperature_c,
        predecessor_cp399_retained_supply_temperature_owned_read: predecessor
            .predecessor_cp399_retained_supply_temperature_owned_read,
        predecessor_supply_temperature_read: predecessor.predecessor_supply_temperature_read,
        predecessor_supply_temperature_c: predecessor.predecessor_supply_temperature_c,
        predecessor_mixed_air_minus_supply_temperature_calculated: predecessor
            .predecessor_mixed_air_minus_supply_temperature_calculated,
        predecessor_mixed_air_minus_supply_temperature_k: predecessor
            .predecessor_mixed_air_minus_supply_temperature_k,
        predecessor_cooling_sensible_output_calculated: predecessor
            .predecessor_cooling_sensible_output_calculated,
        predecessor_calculated_cooling_sensible_output_w: predecessor
            .predecessor_calculated_cooling_sensible_output_w,
        predecessor_cooling_sensible_output_assigned: predecessor
            .predecessor_cooling_sensible_output_assigned,
        predecessor_cooling_sensible_output_w: predecessor.predecessor_cooling_sensible_output_w,
        predecessor_cp400_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp400_resulting_supply_humidity_ratio,
        predecessor_cp400_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp400_resulting_supply_temperature_c: predecessor
            .predecessor_cp400_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed: predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
        predecessor_cp400_retained_supply_humidity_ratio_state_owned: predecessor
            .cp400_retained_supply_humidity_ratio_state_owned,
        predecessor_cp400_retained_supply_enthalpy_state_owned: predecessor
            .cp400_retained_supply_enthalpy_state_owned,
        predecessor_cp400_retained_supply_temperature_state_owned: predecessor
            .cp400_retained_supply_temperature_state_owned,
        predecessor_cp384_retained_cooling_total_output_owned_read: predecessor
            .cp384_retained_cooling_total_output_owned_read,
        predecessor_cp385_cooling_total_output_bit_corroborated: predecessor
            .cp385_cooling_total_output_bit_corroborated,
        predecessor_cooling_total_output_read: predecessor.cooling_total_output_read,
        predecessor_cooling_total_output_w: predecessor.cooling_total_output_w,
        predecessor_cp400_retained_cooling_sensible_output_owned_read: predecessor
            .cp400_retained_cooling_sensible_output_owned_read,
        predecessor_cp401_cooling_sensible_output_read: predecessor.cooling_sensible_output_read,
        predecessor_cp401_cooling_sensible_output_w: predecessor.cooling_sensible_output_w,
        predecessor_cooling_latent_output_calculated: predecessor.cooling_latent_output_calculated,
        predecessor_calculated_cooling_latent_output_w: predecessor
            .calculated_cooling_latent_output_w,
        predecessor_cooling_latent_output_assigned: predecessor.cooling_latent_output_assigned,
        predecessor_cooling_latent_output_w: predecessor.cooling_latent_output_w,
        predecessor_cp401_resulting_supply_humidity_ratio: predecessor
            .resulting_supply_humidity_ratio,
        predecessor_cp401_resulting_supply_enthalpy_j_per_kg: predecessor
            .resulting_supply_enthalpy_j_per_kg,
        predecessor_cp401_resulting_supply_temperature_c: predecessor
            .resulting_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated: active,
        cp401_retained_cooling_latent_output_owned_read: active,
        cooling_latent_output_read: active,
        cooling_latent_output_w,
        cp321_maximum_total_cooling_capacity_owned_read: active,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: active,
        maximum_total_cooling_capacity_read: active,
        maximum_total_cooling_capacity_w,
        cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated: active,
        cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity: comparison,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered: comparison == Some(true),
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: comparison == Some(false),
        cp401_retained_supply_humidity_ratio_state_owned: predecessor_has_supply_humidity_ratio(
            route.predecessor_index,
        ),
        cp401_retained_supply_enthalpy_state_owned: predecessor_has_supply_enthalpy(
            route.predecessor_index,
        ),
        cp401_retained_supply_temperature_state_owned: predecessor_has_supply_temperature(
            route.predecessor_index,
        ),
        resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: predecessor.resulting_supply_temperature_c,
    }
}
