//! Lossless JSON serialization for one CP400 sensible-output snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped": snapshot.positive_guard_false_fallthrough_skipped,
        "heating_availability_guard_false_fallthrough": snapshot.heating_availability_guard_false_fallthrough,
        "humidification_control_guard_false_fallthrough": snapshot.humidification_control_guard_false_fallthrough,
        "dehumidification_control_humidistat_maximum_assignment_executed": snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        "dehumidification_control_none_maximum_assignment_executed": snapshot.dehumidification_control_none_maximum_assignment_executed,
        "dehumidification_control_guard_false_fallthrough": snapshot.dehumidification_control_guard_false_fallthrough,
        "predecessor_capacity_limit_guard_evaluated": snapshot.predecessor_capacity_limit_guard_evaluated,
        "predecessor_capacity_limit_body_entered": snapshot.predecessor_capacity_limit_body_entered,
        "predecessor_active_capacity_limit_guard_false_fallthrough": snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        "predecessor_dehumidification_guard_evaluated": snapshot.predecessor_dehumidification_guard_evaluated,
        "predecessor_dehumidification_body_entered": snapshot.predecessor_dehumidification_body_entered,
        "predecessor_dehumidification_guard_false_fallthrough": snapshot.predecessor_dehumidification_guard_false_fallthrough,
        "predecessor_dehumidification_total_output_assignment_executed": snapshot.predecessor_dehumidification_total_output_assignment_executed,
        "predecessor_dehumidification_total_output_capacity_guard_evaluated": snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        "predecessor_dehumidification_total_output_capacity_adjustment_body_entered": snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough": snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        "dehumidification_total_output_capacity_guard_false_fallthrough": snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        "dehumidification_total_output_maximum_capacity_assignment_executed": snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        "predecessor_supply_enthalpy_assignment_executed": snapshot.predecessor_supply_enthalpy_assignment_executed,
        "predecessor_dehumidification_control_type_read": snapshot.predecessor_dehumidification_control_type_read,
        "predecessor_dehumidification_control_type": snapshot.predecessor_dehumidification_control_type.map(control_type_name),
        "predecessor_dehumidification_control_switch_dispatched": snapshot.predecessor_dehumidification_control_switch_dispatched,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered": snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break": snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        "predecessor_dehumidification_control_humidistat_case_entered": snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed": snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        "predecessor_dehumidification_control_humidistat_case_exited_via_break": snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        "predecessor_dehumidification_control_none_case_entered": snapshot.predecessor_dehumidification_control_none_case_entered,
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered": snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,

        "predecessor_cp397_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp397_resulting_supply_humidity_ratio),
        "predecessor_cp397_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp397_resulting_supply_humidity_ratio),
        "predecessor_cp397_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg),
        "predecessor_cp397_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg),
        "predecessor_cp397_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp397_resulting_supply_temperature_c),
        "predecessor_cp397_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp397_resulting_supply_temperature_c),
        "predecessor_cp398_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp398_resulting_supply_humidity_ratio),
        "predecessor_cp398_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp398_resulting_supply_humidity_ratio),
        "predecessor_cp398_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg),
        "predecessor_cp398_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg),
        "predecessor_cp398_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp398_resulting_supply_temperature_c),
        "predecessor_cp398_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp398_resulting_supply_temperature_c),

        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed": snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        "predecessor_mixed_air_humidity_ratio_read": snapshot.predecessor_mixed_air_humidity_ratio_read,
        "predecessor_mixed_air_humidity_ratio": json_number(snapshot.predecessor_mixed_air_humidity_ratio),
        "predecessor_mixed_air_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_mixed_air_humidity_ratio),
        "predecessor_psychrometric_cp_air_evaluated": snapshot.predecessor_psychrometric_cp_air_evaluated,
        "predecessor_psychrometric_cp_air_result_j_per_kg_k": json_number(snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k),
        "predecessor_psychrometric_cp_air_result_j_per_kg_k_ieee_bits": ieee_bits(snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k),
        "predecessor_cp_air_assigned": snapshot.predecessor_cp_air_assigned,
        "predecessor_cp_air_j_per_kg_k": json_number(snapshot.predecessor_cp_air_j_per_kg_k),
        "predecessor_cp_air_j_per_kg_k_ieee_bits": ieee_bits(snapshot.predecessor_cp_air_j_per_kg_k),
        "predecessor_cp399_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp399_resulting_supply_humidity_ratio),
        "predecessor_cp399_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp399_resulting_supply_humidity_ratio),
        "predecessor_cp399_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp399_resulting_supply_enthalpy_j_per_kg),
        "predecessor_cp399_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp399_resulting_supply_enthalpy_j_per_kg),
        "predecessor_cp399_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp399_resulting_supply_temperature_c),
        "predecessor_cp399_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp399_resulting_supply_temperature_c),

        "dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed": snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
        "cp399_retained_supply_humidity_ratio_state_owned": snapshot.cp399_retained_supply_humidity_ratio_state_owned,
        "cp399_retained_supply_enthalpy_state_owned": snapshot.cp399_retained_supply_enthalpy_state_owned,
        "cp399_retained_supply_temperature_state_owned": snapshot.cp399_retained_supply_temperature_state_owned,
        "cp330_retained_supply_mass_flow_rate_owned_read": snapshot.cp330_retained_supply_mass_flow_rate_owned_read,
        "cp329_supply_mass_flow_rate_bit_corroborated": snapshot.cp329_supply_mass_flow_rate_bit_corroborated,
        "supply_mass_flow_rate_read": snapshot.supply_mass_flow_rate_read,
        "supply_mass_flow_rate_kg_per_s": json_number(snapshot.supply_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_kg_per_s_ieee_bits": ieee_bits(snapshot.supply_mass_flow_rate_kg_per_s),
        "cp399_retained_cp_air_owned_read": snapshot.cp399_retained_cp_air_owned_read,
        "cp_air_read": snapshot.cp_air_read,
        "cp_air_j_per_kg_k": json_number(snapshot.cp_air_j_per_kg_k),
        "cp_air_j_per_kg_k_ieee_bits": ieee_bits(snapshot.cp_air_j_per_kg_k),
        "supply_mass_flow_rate_times_cp_air_calculated": snapshot.supply_mass_flow_rate_times_cp_air_calculated,
        "supply_mass_flow_rate_times_cp_air_w_per_k": json_number(snapshot.supply_mass_flow_rate_times_cp_air_w_per_k),
        "supply_mass_flow_rate_times_cp_air_w_per_k_ieee_bits": ieee_bits(snapshot.supply_mass_flow_rate_times_cp_air_w_per_k),
        "cp329_retained_mixed_air_temperature_owned_read": snapshot.cp329_retained_mixed_air_temperature_owned_read,
        "mixed_air_temperature_read": snapshot.mixed_air_temperature_read,
        "mixed_air_temperature_c": json_number(snapshot.mixed_air_temperature_c),
        "mixed_air_temperature_c_ieee_bits": ieee_bits(snapshot.mixed_air_temperature_c),
        "cp399_retained_supply_temperature_owned_read": snapshot.cp399_retained_supply_temperature_owned_read,
        "supply_temperature_read": snapshot.supply_temperature_read,
        "supply_temperature_c": json_number(snapshot.supply_temperature_c),
        "supply_temperature_c_ieee_bits": ieee_bits(snapshot.supply_temperature_c),
        "mixed_air_minus_supply_temperature_calculated": snapshot.mixed_air_minus_supply_temperature_calculated,
        "mixed_air_minus_supply_temperature_k": json_number(snapshot.mixed_air_minus_supply_temperature_k),
        "mixed_air_minus_supply_temperature_k_ieee_bits": ieee_bits(snapshot.mixed_air_minus_supply_temperature_k),
        "cooling_sensible_output_calculated": snapshot.cooling_sensible_output_calculated,
        "calculated_cooling_sensible_output_w": json_number(snapshot.calculated_cooling_sensible_output_w),
        "calculated_cooling_sensible_output_w_ieee_bits": ieee_bits(snapshot.calculated_cooling_sensible_output_w),
        "cooling_sensible_output_assigned": snapshot.cooling_sensible_output_assigned,
        "cooling_sensible_output_w": json_number(snapshot.cooling_sensible_output_w),
        "cooling_sensible_output_w_ieee_bits": ieee_bits(snapshot.cooling_sensible_output_w),
        "resulting_supply_humidity_ratio": json_number(snapshot.resulting_supply_humidity_ratio),
        "resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.resulting_supply_humidity_ratio),
        "resulting_supply_enthalpy_j_per_kg": json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_temperature_c": json_number(snapshot.resulting_supply_temperature_c),
        "resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.resulting_supply_temperature_c),
    })
}

fn control_type_name(control: DehumidificationControlType) -> &'static str {
    match control {
        DehumidificationControlType::None => "None",
        DehumidificationControlType::ConstantSensibleHeatRatio => "ConstantSensibleHeatRatio",
        DehumidificationControlType::Humidistat => "Humidistat",
        DehumidificationControlType::ConstantSupplyHumidityRatio => "ConstantSupplyHumidityRatio",
    }
}

fn json_number(value: Option<f64>) -> Value {
    value
        .filter(|value| value.is_finite())
        .map_or(Value::Null, |value| json!(value))
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(in crate::pipeline) use tests::snapshot as test_snapshot;
