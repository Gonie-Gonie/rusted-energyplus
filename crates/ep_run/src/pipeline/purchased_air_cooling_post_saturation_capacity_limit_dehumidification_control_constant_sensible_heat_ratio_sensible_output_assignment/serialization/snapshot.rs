//! JSON serialization for one CP388 sensible-output assignment snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
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
        "predecessor_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_resulting_supply_enthalpy_j_per_kg),
        "predecessor_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_resulting_supply_enthalpy_j_per_kg),
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered": snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed": snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        "predecessor_mixed_air_humidity_ratio_read": snapshot.predecessor_mixed_air_humidity_ratio_read,
        "predecessor_mixed_air_humidity_ratio": json_number(snapshot.predecessor_mixed_air_humidity_ratio),
        "predecessor_mixed_air_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_mixed_air_humidity_ratio),
        "predecessor_psychrometric_cp_air_evaluated": snapshot.predecessor_psychrometric_cp_air_evaluated,
        "predecessor_psychrometric_cp_air_result_j_per_kg_k": json_number(snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k),
        "predecessor_psychrometric_cp_air_result_j_per_kg_k_ieee_bits": ieee_bits(snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k),
        "predecessor_cp_air_assigned": snapshot.predecessor_cp_air_assigned,
        "predecessor_cp_air_j_per_kg_k": json_number(snapshot.predecessor_cp_air_j_per_kg_k),
        "predecessor_cp_air_j_per_kg_k_ieee_bits": ieee_bits(snapshot.predecessor_cp_air_j_per_kg_k),
        "dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed": snapshot.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        "cp384_retained_cooling_total_output_owned_read": snapshot.cp384_retained_cooling_total_output_owned_read,
        "cp385_cooling_total_output_bit_corroborated": snapshot.cp385_cooling_total_output_bit_corroborated,
        "cooling_total_output_read": snapshot.cooling_total_output_read,
        "cooling_total_output_w": json_number(snapshot.cooling_total_output_w),
        "cooling_total_output_w_ieee_bits": ieee_bits(snapshot.cooling_total_output_w),
        "cooling_sensible_heat_ratio_read": snapshot.cooling_sensible_heat_ratio_read,
        "cooling_sensible_heat_ratio": json_number(snapshot.cooling_sensible_heat_ratio),
        "cooling_sensible_heat_ratio_ieee_bits": ieee_bits(snapshot.cooling_sensible_heat_ratio),
        "cooling_sensible_output_calculated": snapshot.cooling_sensible_output_calculated,
        "calculated_cooling_sensible_output_w": json_number(snapshot.calculated_cooling_sensible_output_w),
        "calculated_cooling_sensible_output_w_ieee_bits": ieee_bits(snapshot.calculated_cooling_sensible_output_w),
        "cooling_sensible_output_assigned": snapshot.cooling_sensible_output_assigned,
        "cooling_sensible_output_w": json_number(snapshot.cooling_sensible_output_w),
        "cooling_sensible_output_w_ieee_bits": ieee_bits(snapshot.cooling_sensible_output_w),
        "resulting_supply_enthalpy_j_per_kg": json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
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
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn symbolic_selector_and_nonfinite_bit_sidecars_are_stable() {
        assert_eq!(
            control_type_name(DehumidificationControlType::ConstantSensibleHeatRatio),
            "ConstantSensibleHeatRatio"
        );
        let value = Some(f64::from_bits(0x7ff8_0000_0000_0388));
        assert!(json_number(value).is_null());
        assert_eq!(ieee_bits(value), Some("0x7ff8000000000388".to_string()));
        assert_eq!(
            ieee_bits(Some(-0.0)),
            Some("0x8000000000000000".to_string())
        );
    }

    #[test]
    fn full_active_snapshot_json_preserves_each_nonfinite_bit_sidecar() {
        let values = std::array::from_fn(|index| {
            f64::from_bits(0x7ff8_0000_0000_0380 + u64::try_from(index).expect("small index"))
        });
        let json = snapshot_json(active_snapshot(values));
        for (index, field) in [
            "predecessor_resulting_supply_enthalpy_j_per_kg",
            "predecessor_mixed_air_humidity_ratio",
            "predecessor_psychrometric_cp_air_result_j_per_kg_k",
            "predecessor_cp_air_j_per_kg_k",
            "cooling_total_output_w",
            "cooling_sensible_heat_ratio",
            "calculated_cooling_sensible_output_w",
            "cooling_sensible_output_w",
            "resulting_supply_enthalpy_j_per_kg",
        ]
        .into_iter()
        .enumerate()
        {
            assert!(json[field].is_null(), "{field}");
            assert_eq!(
                json[format!("{field}_ieee_bits")],
                format!("0x{:016x}", values[index].to_bits()),
                "{field}"
            );
        }
        assert_eq!(
            json["predecessor_dehumidification_control_type"],
            "ConstantSensibleHeatRatio"
        );
    }

    fn active_snapshot(
        values: [f64; 9],
    ) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot
    {
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(388),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(38),
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            heating_availability_guard_false_fallthrough: false,
            humidification_control_guard_false_fallthrough: false,
            dehumidification_control_humidistat_maximum_assignment_executed: false,
            dehumidification_control_none_maximum_assignment_executed: false,
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
            predecessor_dehumidification_control_type: Some(
                DehumidificationControlType::ConstantSensibleHeatRatio,
            ),
            predecessor_dehumidification_control_switch_dispatched: true,
            predecessor_resulting_supply_enthalpy_j_per_kg: Some(values[0]),
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: true,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: true,
            predecessor_mixed_air_humidity_ratio_read: true,
            predecessor_mixed_air_humidity_ratio: Some(values[1]),
            predecessor_psychrometric_cp_air_evaluated: true,
            predecessor_psychrometric_cp_air_result_j_per_kg_k: Some(values[2]),
            predecessor_cp_air_assigned: true,
            predecessor_cp_air_j_per_kg_k: Some(values[3]),
            dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed: true,
            cp384_retained_cooling_total_output_owned_read: true,
            cp385_cooling_total_output_bit_corroborated: true,
            cooling_total_output_read: true,
            cooling_total_output_w: Some(values[4]),
            cooling_sensible_heat_ratio_read: true,
            cooling_sensible_heat_ratio: Some(values[5]),
            cooling_sensible_output_calculated: true,
            calculated_cooling_sensible_output_w: Some(values[6]),
            cooling_sensible_output_assigned: true,
            cooling_sensible_output_w: Some(values[7]),
            resulting_supply_enthalpy_j_per_kg: Some(values[8]),
        }
    }
}
