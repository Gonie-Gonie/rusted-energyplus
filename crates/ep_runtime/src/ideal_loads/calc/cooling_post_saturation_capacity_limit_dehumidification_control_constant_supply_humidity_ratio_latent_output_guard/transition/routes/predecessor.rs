//! Lossless CP402-to-CP401 predecessor reconstruction.

use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
};

pub(in crate::ideal_loads::calc) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot
            .predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot
            .predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot
            .predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: snapshot
            .predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: snapshot
            .predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_cp397_resulting_supply_humidity_ratio: snapshot
            .predecessor_cp397_resulting_supply_humidity_ratio,
        predecessor_cp397_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp397_resulting_supply_temperature_c: snapshot
            .predecessor_cp397_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_case_entered: snapshot
            .predecessor_dehumidification_control_none_case_entered,
        predecessor_cp398_resulting_supply_humidity_ratio: snapshot
            .predecessor_cp398_resulting_supply_humidity_ratio,
        predecessor_cp398_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp398_resulting_supply_temperature_c: snapshot
            .predecessor_cp398_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        predecessor_mixed_air_humidity_ratio_read: snapshot
            .predecessor_mixed_air_humidity_ratio_read,
        predecessor_mixed_air_humidity_ratio: snapshot.predecessor_mixed_air_humidity_ratio,
        predecessor_psychrometric_cp_air_evaluated: snapshot
            .predecessor_psychrometric_cp_air_evaluated,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: snapshot
            .predecessor_psychrometric_cp_air_result_j_per_kg_k,
        predecessor_cp_air_assigned: snapshot.predecessor_cp_air_assigned,
        predecessor_cp_air_j_per_kg_k: snapshot.predecessor_cp_air_j_per_kg_k,
        predecessor_cp399_resulting_supply_humidity_ratio: snapshot
            .predecessor_cp399_resulting_supply_humidity_ratio,
        predecessor_cp399_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp399_resulting_supply_temperature_c: snapshot
            .predecessor_cp399_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
        predecessor_cp399_retained_supply_humidity_ratio_state_owned: snapshot
            .predecessor_cp399_retained_supply_humidity_ratio_state_owned,
        predecessor_cp399_retained_supply_enthalpy_state_owned: snapshot
            .predecessor_cp399_retained_supply_enthalpy_state_owned,
        predecessor_cp399_retained_supply_temperature_state_owned: snapshot
            .predecessor_cp399_retained_supply_temperature_state_owned,
        predecessor_cp330_retained_supply_mass_flow_rate_owned_read: snapshot
            .predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        predecessor_cp329_supply_mass_flow_rate_bit_corroborated: snapshot
            .predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        predecessor_supply_mass_flow_rate_read: snapshot.predecessor_supply_mass_flow_rate_read,
        predecessor_supply_mass_flow_rate_kg_per_s: snapshot
            .predecessor_supply_mass_flow_rate_kg_per_s,
        predecessor_cp399_retained_cp_air_owned_read: snapshot
            .predecessor_cp399_retained_cp_air_owned_read,
        predecessor_cp_air_read: snapshot.predecessor_cp_air_read,
        predecessor_cp400_cp_air_j_per_kg_k: snapshot.predecessor_cp400_cp_air_j_per_kg_k,
        predecessor_supply_mass_flow_rate_times_cp_air_calculated: snapshot
            .predecessor_supply_mass_flow_rate_times_cp_air_calculated,
        predecessor_supply_mass_flow_rate_times_cp_air_w_per_k: snapshot
            .predecessor_supply_mass_flow_rate_times_cp_air_w_per_k,
        predecessor_cp329_retained_mixed_air_temperature_owned_read: snapshot
            .predecessor_cp329_retained_mixed_air_temperature_owned_read,
        predecessor_mixed_air_temperature_read: snapshot.predecessor_mixed_air_temperature_read,
        predecessor_mixed_air_temperature_c: snapshot.predecessor_mixed_air_temperature_c,
        predecessor_cp399_retained_supply_temperature_owned_read: snapshot
            .predecessor_cp399_retained_supply_temperature_owned_read,
        predecessor_supply_temperature_read: snapshot.predecessor_supply_temperature_read,
        predecessor_supply_temperature_c: snapshot.predecessor_supply_temperature_c,
        predecessor_mixed_air_minus_supply_temperature_calculated: snapshot
            .predecessor_mixed_air_minus_supply_temperature_calculated,
        predecessor_mixed_air_minus_supply_temperature_k: snapshot
            .predecessor_mixed_air_minus_supply_temperature_k,
        predecessor_cooling_sensible_output_calculated: snapshot
            .predecessor_cooling_sensible_output_calculated,
        predecessor_calculated_cooling_sensible_output_w: snapshot
            .predecessor_calculated_cooling_sensible_output_w,
        predecessor_cooling_sensible_output_assigned: snapshot
            .predecessor_cooling_sensible_output_assigned,
        predecessor_cooling_sensible_output_w: snapshot.predecessor_cooling_sensible_output_w,
        predecessor_cp400_resulting_supply_humidity_ratio: snapshot
            .predecessor_cp400_resulting_supply_humidity_ratio,
        predecessor_cp400_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp400_resulting_supply_temperature_c: snapshot
            .predecessor_cp400_resulting_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
        cp400_retained_supply_humidity_ratio_state_owned: snapshot
            .predecessor_cp400_retained_supply_humidity_ratio_state_owned,
        cp400_retained_supply_enthalpy_state_owned: snapshot
            .predecessor_cp400_retained_supply_enthalpy_state_owned,
        cp400_retained_supply_temperature_state_owned: snapshot
            .predecessor_cp400_retained_supply_temperature_state_owned,
        cp384_retained_cooling_total_output_owned_read: snapshot
            .predecessor_cp384_retained_cooling_total_output_owned_read,
        cp385_cooling_total_output_bit_corroborated: snapshot
            .predecessor_cp385_cooling_total_output_bit_corroborated,
        cooling_total_output_read: snapshot.predecessor_cooling_total_output_read,
        cooling_total_output_w: snapshot.predecessor_cooling_total_output_w,
        cp400_retained_cooling_sensible_output_owned_read: snapshot
            .predecessor_cp400_retained_cooling_sensible_output_owned_read,
        cooling_sensible_output_read: snapshot.predecessor_cp401_cooling_sensible_output_read,
        cooling_sensible_output_w: snapshot.predecessor_cp401_cooling_sensible_output_w,
        cooling_latent_output_calculated: snapshot.predecessor_cooling_latent_output_calculated,
        calculated_cooling_latent_output_w: snapshot
            .predecessor_calculated_cooling_latent_output_w,
        cooling_latent_output_assigned: snapshot.predecessor_cooling_latent_output_assigned,
        cooling_latent_output_w: snapshot.predecessor_cooling_latent_output_w,
        resulting_supply_humidity_ratio: snapshot
            .predecessor_cp401_resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp401_resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: snapshot
            .predecessor_cp401_resulting_supply_temperature_c,
    }
}
