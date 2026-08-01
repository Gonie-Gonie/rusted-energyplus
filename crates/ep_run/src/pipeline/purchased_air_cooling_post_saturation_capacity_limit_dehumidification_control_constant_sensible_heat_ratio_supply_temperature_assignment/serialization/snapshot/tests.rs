use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
};

use super::*;

#[test]
fn full_active_snapshot_projects_nonfinite_values_to_null_with_exact_bits() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0389);
    let value = snapshot_json(snapshot(Some(nan), true));
    for field in numeric_fields() {
        assert!(value[field].is_null(), "{field}");
        assert_eq!(
            value[format!("{field}_ieee_bits")],
            "0x7ff8000000000389",
            "{field}"
        );
    }
}

#[test]
fn inactive_early_and_later_prefix_temperature_retention_is_explicit() {
    let early = snapshot_json(snapshot(None, false));
    assert!(early["preexisting_supply_temperature_c"].is_null());
    assert!(early["resulting_supply_temperature_c"].is_null());

    let later = snapshot_json(snapshot(Some(-0.0), false));
    assert_eq!(
        later["preexisting_supply_temperature_c_ieee_bits"],
        "0x8000000000000000"
    );
    assert_eq!(
        later["resulting_supply_temperature_c_ieee_bits"],
        "0x8000000000000000"
    );
    for field in source_local_fields() {
        assert!(later[field].is_null(), "{field}");
        assert!(
            later[format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
}

fn numeric_fields() -> [&'static str; 18] {
    [
        "predecessor_mixed_air_humidity_ratio",
        "predecessor_psychrometric_cp_air_result_j_per_kg_k",
        "predecessor_cp_air_j_per_kg_k",
        "predecessor_cooling_total_output_w",
        "predecessor_cooling_sensible_heat_ratio",
        "predecessor_calculated_cooling_sensible_output_w",
        "predecessor_cooling_sensible_output_w",
        "resulting_supply_enthalpy_j_per_kg",
        "preexisting_supply_temperature_c",
        "mixed_air_temperature_c",
        "cooling_sensible_output_w",
        "cp_air_j_per_kg_k",
        "supply_mass_flow_rate_kg_per_s",
        "cp_air_times_supply_mass_flow_rate_w_per_k",
        "cooling_sensible_output_over_air_capacity_rate_k",
        "calculated_supply_temperature_c",
        "assigned_supply_temperature_c",
        "resulting_supply_temperature_c",
    ]
}

fn source_local_fields() -> [&'static str; 8] {
    [
        "mixed_air_temperature_c",
        "cooling_sensible_output_w",
        "cp_air_j_per_kg_k",
        "supply_mass_flow_rate_kg_per_s",
        "cp_air_times_supply_mass_flow_rate_w_per_k",
        "cooling_sensible_output_over_air_capacity_rate_k",
        "calculated_supply_temperature_c",
        "assigned_supply_temperature_c",
    ]
}

fn snapshot(
    value: Option<f64>,
    active: bool,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot{
    let local = active.then_some(value).flatten();
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(389),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(38),
        unit_off_skipped: false,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: false,
        humidification_control_guard_false_fallthrough: false,
        dehumidification_control_humidistat_maximum_assignment_executed: false,
        dehumidification_control_none_maximum_assignment_executed: !active,
        dehumidification_control_guard_false_fallthrough: false,
        predecessor_capacity_limit_guard_evaluated: true,
        predecessor_capacity_limit_body_entered: true,
        predecessor_active_capacity_limit_guard_false_fallthrough: false,
        predecessor_dehumidification_guard_evaluated: true,
        predecessor_dehumidification_body_entered: true,
        predecessor_dehumidification_guard_false_fallthrough: false,
        predecessor_dehumidification_total_output_assignment_executed: true,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: true,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: true,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_maximum_capacity_assignment_executed: true,
        predecessor_supply_enthalpy_assignment_executed: true,
        predecessor_dehumidification_control_type_read: true,
        predecessor_dehumidification_control_type: Some(if active { DehumidificationControlType::ConstantSensibleHeatRatio } else { DehumidificationControlType::None }),
        predecessor_dehumidification_control_switch_dispatched: true,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: active,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: active,
        predecessor_mixed_air_humidity_ratio_read: active,
        predecessor_mixed_air_humidity_ratio: value,
        predecessor_psychrometric_cp_air_evaluated: active,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: value,
        predecessor_cp_air_assigned: active,
        predecessor_cp_air_j_per_kg_k: value,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed: active,
        predecessor_cp384_retained_cooling_total_output_owned_read: active,
        predecessor_cp385_cooling_total_output_bit_corroborated: active,
        predecessor_cooling_total_output_read: active,
        predecessor_cooling_total_output_w: value,
        predecessor_cooling_sensible_heat_ratio_read: active,
        predecessor_cooling_sensible_heat_ratio: value,
        predecessor_cooling_sensible_output_calculated: active,
        predecessor_calculated_cooling_sensible_output_w: value,
        predecessor_cooling_sensible_output_assigned: active,
        predecessor_cooling_sensible_output_w: value,
        resulting_supply_enthalpy_j_per_kg: value,
        dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed: active,
        cp379_retained_supply_temperature_state_owned: value.is_some(),
        preexisting_supply_temperature_c: value,
        cp329_retained_mixed_air_temperature_owned_read: active,
        mixed_air_temperature_read: active,
        mixed_air_temperature_c: local,
        cp388_retained_cooling_sensible_output_owned_read: active,
        cooling_sensible_output_read: active,
        cooling_sensible_output_w: local,
        cp387_retained_cp_air_owned_read: active,
        cp_air_read: active,
        cp_air_j_per_kg_k: local,
        cp330_retained_supply_mass_flow_rate_owned_read: active,
        cp329_supply_mass_flow_rate_bit_corroborated: active,
        supply_mass_flow_rate_read: active,
        supply_mass_flow_rate_kg_per_s: local,
        cp_air_times_supply_mass_flow_rate_calculated: active,
        cp_air_times_supply_mass_flow_rate_w_per_k: local,
        cooling_sensible_output_over_air_capacity_rate_calculated: active,
        cooling_sensible_output_over_air_capacity_rate_k: local,
        supply_temperature_calculated: active,
        calculated_supply_temperature_c: local,
        supply_temperature_assigned: active,
        assigned_supply_temperature_c: local,
        resulting_supply_temperature_c: value,
    }
}
