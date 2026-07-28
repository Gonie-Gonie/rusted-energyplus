//! JSON serialization for one CP349 snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_no_outdoor_air_fallback_entered":
            snapshot.predecessor_no_outdoor_air_fallback_entered,
        "predecessor_positive_supply_mass_flow_body_entered":
            snapshot.predecessor_positive_supply_mass_flow_body_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "predecessor_dehumidification_control_type":
            snapshot.predecessor_dehumidification_control_type.map(control_type_name),
        "predecessor_dehumidification_control_none_case_completed":
            snapshot.predecessor_dehumidification_control_none_case_completed,
        "predecessor_dehumidification_control_none_case_completed_skip":
            snapshot.predecessor_dehumidification_control_none_case_completed_skip,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered":
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        "predecessor_dehumidification_control_humidistat_case_selected_skip":
            snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip,
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        "dehumidification_control_humidistat_case_selected_skip":
            snapshot.dehumidification_control_humidistat_case_selected_skip,
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "mixed_air_humidity_ratio_read": snapshot.mixed_air_humidity_ratio_read,
        "mixed_air_humidity_ratio": json_number(snapshot.mixed_air_humidity_ratio),
        "mixed_air_humidity_ratio_ieee_bits": ieee_bits(snapshot.mixed_air_humidity_ratio),
        "psychrometric_cp_air_evaluated": snapshot.psychrometric_cp_air_evaluated,
        "psychrometric_cp_air_result_j_per_kg_k":
            json_number(snapshot.psychrometric_cp_air_result_j_per_kg_k),
        "psychrometric_cp_air_result_j_per_kg_k_ieee_bits":
            ieee_bits(snapshot.psychrometric_cp_air_result_j_per_kg_k),
        "cp_air_assigned": snapshot.cp_air_assigned,
        "cp_air_j_per_kg_k": json_number(snapshot.cp_air_j_per_kg_k),
        "cp_air_j_per_kg_k_ieee_bits": ieee_bits(snapshot.cp_air_j_per_kg_k),
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn selector_variants_serialize_as_symbolic_names() {
        for (control, expected) in [
            (DehumidificationControlType::None, "None"),
            (
                DehumidificationControlType::ConstantSensibleHeatRatio,
                "ConstantSensibleHeatRatio",
            ),
            (DehumidificationControlType::Humidistat, "Humidistat"),
            (
                DehumidificationControlType::ConstantSupplyHumidityRatio,
                "ConstantSupplyHumidityRatio",
            ),
        ] {
            let value = snapshot_json(snapshot(Some(control), None));
            assert_eq!(value["predecessor_dehumidification_control_type"], expected);
            assert!(
                value["predecessor_dehumidification_control_type"]
                    .as_u64()
                    .is_none()
            );
        }
    }

    #[test]
    fn direct_none_skip_serializes_null_numeric_values_and_bits() {
        let value = snapshot_json(snapshot(Some(DehumidificationControlType::None), None));
        for field in [
            "mixed_air_humidity_ratio",
            "mixed_air_humidity_ratio_ieee_bits",
            "psychrometric_cp_air_result_j_per_kg_k",
            "psychrometric_cp_air_result_j_per_kg_k_ieee_bits",
            "cp_air_j_per_kg_k",
            "cp_air_j_per_kg_k_ieee_bits",
        ] {
            assert!(value[field].is_null(), "{field}");
        }
        assert_eq!(
            value["source_order"],
            json!([
                "read-purchased-air-mixed-air-humidity-ratio-for-constant-sensible-heat-ratio-cp-air",
                "evaluate-psy-cp-air-fn-w-for-constant-sensible-heat-ratio-cp-air",
                "assign-local-cp-air-for-constant-sensible-heat-ratio-case",
            ])
        );
    }

    #[test]
    fn finite_and_nonfinite_numeric_serialization_preserves_authoritative_bits() {
        let finite = 0.008_765_432_109_876_543_f64;
        let value = snapshot_json(snapshot(
            Some(DehumidificationControlType::ConstantSensibleHeatRatio),
            Some(finite),
        ));
        assert_eq!(value["mixed_air_humidity_ratio"], json!(finite));
        assert_eq!(
            value["mixed_air_humidity_ratio_ieee_bits"],
            format!("0x{:016x}", finite.to_bits())
        );

        let nonfinite = f64::from_bits(0x7ff8_0000_0000_0042);
        let value = snapshot_json(snapshot(
            Some(DehumidificationControlType::ConstantSensibleHeatRatio),
            Some(nonfinite),
        ));
        assert!(value["mixed_air_humidity_ratio"].is_null());
        assert_eq!(
            value["mixed_air_humidity_ratio_ieee_bits"],
            "0x7ff8000000000042"
        );
    }

    fn snapshot(
        control: Option<DehumidificationControlType>,
        value: Option<f64>,
    ) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot
    {
        let none = control == Some(DehumidificationControlType::None);
        let active = control == Some(DehumidificationControlType::ConstantSensibleHeatRatio);
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            predecessor_positive_supply_mass_flow_body_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            predecessor_dehumidification_control_type: control,
            predecessor_dehumidification_control_none_case_completed: none,
            predecessor_dehumidification_control_none_case_completed_skip: none,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered:
                active,
            predecessor_dehumidification_control_humidistat_case_selected_skip:
                control == Some(DehumidificationControlType::Humidistat),
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
                control == Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
            dehumidification_control_none_case_completed_skip: none,
            dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed:
                active,
            dehumidification_control_humidistat_case_selected_skip:
                control == Some(DehumidificationControlType::Humidistat),
            dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
                control == Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
            mixed_air_humidity_ratio_read: active,
            mixed_air_humidity_ratio: value,
            psychrometric_cp_air_evaluated: active,
            psychrometric_cp_air_result_j_per_kg_k: value,
            cp_air_assigned: active,
            cp_air_j_per_kg_k: value,
        }
    }
}
