use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};

use super::snapshot_json;

fn snapshot(value: Option<f64>) -> Snapshot {
    Snapshot {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_off_skipped: false,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        heating_availability_guard_false_fallthrough: false,
        humidification_control_guard_false_fallthrough: false,
        dehumidification_control_humidistat_maximum_assignment_executed: false,
        dehumidification_control_none_maximum_assignment_executed: false,
        dehumidification_control_guard_false_fallthrough: false,
        predecessor_capacity_limit_guard_evaluated: false,
        predecessor_capacity_limit_body_entered: false,
        predecessor_active_capacity_limit_guard_false_fallthrough: false,
        predecessor_dehumidification_guard_evaluated: false,
        predecessor_dehumidification_body_entered: false,
        predecessor_dehumidification_guard_false_fallthrough: false,
        predecessor_dehumidification_total_output_assignment_executed: false,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: false,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: false,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_capacity_guard_false_fallthrough: false,
        dehumidification_total_output_maximum_capacity_assignment_executed: false,
        predecessor_supply_enthalpy_assignment_executed: false,
        predecessor_dehumidification_control_type_read: false,
        predecessor_dehumidification_control_type: Some(DehumidificationControlType::None),
        predecessor_dehumidification_control_switch_dispatched: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: false,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: false,
        predecessor_dehumidification_control_humidistat_case_entered: false,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: false,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: false,
        predecessor_cp397_resulting_supply_humidity_ratio: value,
        predecessor_cp397_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp397_resulting_supply_temperature_c: value,
        predecessor_dehumidification_control_none_case_entered: false,
        predecessor_cp398_resulting_supply_humidity_ratio: value,
        predecessor_cp398_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp398_resulting_supply_temperature_c: value,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed: false,
        predecessor_mixed_air_humidity_ratio_read: false,
        predecessor_mixed_air_humidity_ratio: value,
        predecessor_psychrometric_cp_air_evaluated: false,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: value,
        predecessor_cp_air_assigned: false,
        predecessor_cp_air_j_per_kg_k: value,
        predecessor_cp399_resulting_supply_humidity_ratio: value,
        predecessor_cp399_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp399_resulting_supply_temperature_c: value,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed: false,
        predecessor_cp399_retained_supply_humidity_ratio_state_owned: false,
        predecessor_cp399_retained_supply_enthalpy_state_owned: false,
        predecessor_cp399_retained_supply_temperature_state_owned: false,
        predecessor_cp330_retained_supply_mass_flow_rate_owned_read: false,
        predecessor_cp329_supply_mass_flow_rate_bit_corroborated: false,
        predecessor_supply_mass_flow_rate_read: false,
        predecessor_supply_mass_flow_rate_kg_per_s: value,
        predecessor_cp399_retained_cp_air_owned_read: false,
        predecessor_cp_air_read: false,
        predecessor_cp400_cp_air_j_per_kg_k: value,
        predecessor_supply_mass_flow_rate_times_cp_air_calculated: false,
        predecessor_supply_mass_flow_rate_times_cp_air_w_per_k: value,
        predecessor_cp329_retained_mixed_air_temperature_owned_read: false,
        predecessor_mixed_air_temperature_read: false,
        predecessor_mixed_air_temperature_c: value,
        predecessor_cp399_retained_supply_temperature_owned_read: false,
        predecessor_supply_temperature_read: false,
        predecessor_supply_temperature_c: value,
        predecessor_mixed_air_minus_supply_temperature_calculated: false,
        predecessor_mixed_air_minus_supply_temperature_k: value,
        predecessor_cooling_sensible_output_calculated: false,
        predecessor_calculated_cooling_sensible_output_w: value,
        predecessor_cooling_sensible_output_assigned: false,
        predecessor_cooling_sensible_output_w: value,
        predecessor_cp400_resulting_supply_humidity_ratio: value,
        predecessor_cp400_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp400_resulting_supply_temperature_c: value,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed: false,
        predecessor_cp400_retained_supply_humidity_ratio_state_owned: false,
        predecessor_cp400_retained_supply_enthalpy_state_owned: false,
        predecessor_cp400_retained_supply_temperature_state_owned: false,
        predecessor_cp384_retained_cooling_total_output_owned_read: false,
        predecessor_cp385_cooling_total_output_bit_corroborated: false,
        predecessor_cooling_total_output_read: false,
        predecessor_cooling_total_output_w: value,
        predecessor_cp400_retained_cooling_sensible_output_owned_read: false,
        predecessor_cp401_cooling_sensible_output_read: false,
        predecessor_cp401_cooling_sensible_output_w: value,
        predecessor_cooling_latent_output_calculated: false,
        predecessor_calculated_cooling_latent_output_w: value,
        predecessor_cooling_latent_output_assigned: false,
        predecessor_cooling_latent_output_w: value,
        predecessor_cp401_resulting_supply_humidity_ratio: value,
        predecessor_cp401_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp401_resulting_supply_temperature_c: value,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated: false,
        predecessor_cp401_retained_cooling_latent_output_owned_read: false,
        predecessor_cooling_latent_output_read: false,
        predecessor_cp402_cooling_latent_output_w: value,
        predecessor_cp321_maximum_total_cooling_capacity_owned_read: false,
        predecessor_cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: false,
        predecessor_maximum_total_cooling_capacity_read: false,
        predecessor_maximum_total_cooling_capacity_w: value,
        predecessor_cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated: false,
        predecessor_cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity: Some(false),
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered: false,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: false,
        predecessor_cp401_retained_supply_humidity_ratio_state_owned: false,
        predecessor_cp401_retained_supply_enthalpy_state_owned: false,
        predecessor_cp401_retained_supply_temperature_state_owned: false,
        predecessor_cp402_resulting_supply_humidity_ratio: value,
        predecessor_cp402_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp402_resulting_supply_temperature_c: value,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed: false,
        predecessor_cp403_cp329_retained_mixed_air_temperature_owned_read: false,
        predecessor_cp402_same_call_mixed_air_temperature_bit_corroborated: false,
        predecessor_cp403_mixed_air_temperature_read: false,
        predecessor_cp403_mixed_air_temperature_c: value,
        predecessor_supply_temperature_assigned: false,
        predecessor_cp403_assigned_supply_temperature_c: value,
        predecessor_cp402_retained_supply_humidity_ratio_state_owned: false,
        predecessor_cp402_retained_supply_enthalpy_state_owned: false,
        predecessor_cp402_retained_supply_temperature_state_owned: false,
        predecessor_cp403_resulting_supply_humidity_ratio: value,
        predecessor_cp403_resulting_supply_enthalpy_j_per_kg: value,
        predecessor_cp403_resulting_supply_temperature_c: value,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed: false,
        cp403_retained_supply_humidity_ratio_state_owned: false,
        cp403_retained_supply_temperature_state_owned: false,
        cp403_retained_supply_enthalpy_state_owned: false,
        cp403_retained_supply_temperature_owned_read: false,
        supply_temperature_for_humidity_ratio_inversion_read: false,
        supply_temperature_c: value,
        cp403_retained_supply_enthalpy_owned_read: false,
        supply_enthalpy_for_humidity_ratio_inversion_read: false,
        supply_enthalpy_j_per_kg: value,
        psychrometric_supply_humidity_ratio_evaluated: false,
        psychrometric_supply_humidity_ratio: value,
        supply_humidity_ratio_assignment_performed: false,
        assigned_supply_humidity_ratio: value,
        resulting_supply_humidity_ratio: value,
        resulting_supply_enthalpy_j_per_kg: value,
        resulting_supply_temperature_c: value,
    }
}

#[test]
fn snapshot_serializes_all_forty_seven_numeric_values_with_adjacent_ieee_sidecars() {
    let encoded = snapshot_json(snapshot(Some(-0.0)));
    let Some(object) = encoded.as_object() else {
        assert!(encoded.is_object(), "CP404 snapshot must be an object");
        return;
    };
    assert_eq!(object.len(), 194);
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        47
    );
    assert_eq!(encoded["supply_temperature_c"], 0.0);
    assert_eq!(
        encoded["supply_temperature_c_ieee_bits"],
        "0x8000000000000000"
    );
    assert_eq!(
        encoded["psychrometric_supply_humidity_ratio_ieee_bits"],
        "0x8000000000000000"
    );
}

#[test]
fn non_finite_projection_is_null_while_ieee_payload_remains_authoritative() {
    let encoded = snapshot_json(snapshot(Some(f64::from_bits(0x7ff8_0000_0000_0042))));
    assert!(encoded["assigned_supply_humidity_ratio"].is_null());
    assert_eq!(
        encoded["assigned_supply_humidity_ratio_ieee_bits"],
        "0x7ff8000000000042"
    );
}

#[test]
fn absent_numeric_values_have_null_projection_and_sidecar() {
    let encoded = snapshot_json(snapshot(None));
    assert!(encoded["predecessor_cp403_resulting_supply_enthalpy_j_per_kg"].is_null());
    assert!(encoded["predecessor_cp403_resulting_supply_enthalpy_j_per_kg_ieee_bits"].is_null());
}
