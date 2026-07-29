//! JSON serialization for one CP353 snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
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
        "predecessor_dehumidification_control_none_case_completed_skip":
            snapshot.predecessor_dehumidification_control_none_case_completed_skip,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed":
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed,
        "predecessor_dehumidification_control_humidistat_case_selected_skip":
            snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip,
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed,
        "dehumidification_control_humidistat_case_selected_skip":
            snapshot.dehumidification_control_humidistat_case_selected_skip,
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "supply_enthalpy_for_overdrying_limit_maximum_read":
            snapshot.supply_enthalpy_for_overdrying_limit_maximum_read,
        "supply_enthalpy_before_overdrying_limit_j_per_kg":
            json_number(snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg),
        "supply_enthalpy_before_overdrying_limit_j_per_kg_ieee_bits":
            ieee_bits(snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg),
        "supply_temperature_for_minimum_humidity_ratio_enthalpy_read":
            snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read,
        "supply_temperature_c": json_number(snapshot.supply_temperature_c),
        "supply_temperature_c_ieee_bits": ieee_bits(snapshot.supply_temperature_c),
        "psychrometric_minimum_supply_enthalpy_evaluated":
            snapshot.psychrometric_minimum_supply_enthalpy_evaluated,
        "psychrometric_minimum_supply_enthalpy_j_per_kg":
            json_number(snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg),
        "psychrometric_minimum_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg),
        "source_shaped_two_argument_maximum_evaluated":
            snapshot.source_shaped_two_argument_maximum_evaluated,
        "maximum_supply_enthalpy_j_per_kg":
            json_number(snapshot.maximum_supply_enthalpy_j_per_kg),
        "maximum_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.maximum_supply_enthalpy_j_per_kg),
        "supply_enthalpy_assignment_performed":
            snapshot.supply_enthalpy_assignment_performed,
        "assigned_supply_enthalpy_j_per_kg":
            json_number(snapshot.assigned_supply_enthalpy_j_per_kg),
        "assigned_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.assigned_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg":
            json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_none_release_serializes_six_null_values_and_bits() {
        let value = snapshot_json(snapshot(None, false));
        assert_eq!(value["predecessor_dehumidification_control_type"], "None");
        for field in numeric_fields() {
            assert!(value[field].is_null(), "{field} value");
            assert!(
                value[format!("{field}_ieee_bits")].is_null(),
                "{field} bits"
            );
        }
    }

    #[test]
    fn nonfinite_numeric_evidence_keeps_authoritative_ieee_bits() {
        let value = snapshot_json(snapshot(Some(f64::NAN), true));
        let expected_bits = format!("0x{:016x}", f64::NAN.to_bits());
        for field in numeric_fields() {
            assert!(value[field].is_null(), "{field} JSON number");
            assert_eq!(value[format!("{field}_ieee_bits")], expected_bits);
        }
    }

    fn numeric_fields() -> [&'static str; 6] {
        [
            "supply_enthalpy_before_overdrying_limit_j_per_kg",
            "supply_temperature_c",
            "psychrometric_minimum_supply_enthalpy_j_per_kg",
            "maximum_supply_enthalpy_j_per_kg",
            "assigned_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ]
    }

    fn snapshot(
        numeric: Option<f64>,
        active: bool,
    ) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot
    {
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
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
            predecessor_dehumidification_control_type: Some(if active {
                DehumidificationControlType::ConstantSensibleHeatRatio
            } else {
                DehumidificationControlType::None
            }),
            predecessor_dehumidification_control_none_case_completed_skip: !active,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed: active,
            predecessor_dehumidification_control_humidistat_case_selected_skip: false,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: false,
            dehumidification_control_none_case_completed_skip: !active,
            dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed: active,
            dehumidification_control_humidistat_case_selected_skip: false,
            dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: false,
            supply_enthalpy_for_overdrying_limit_maximum_read: active,
            supply_enthalpy_before_overdrying_limit_j_per_kg: numeric,
            supply_temperature_for_minimum_humidity_ratio_enthalpy_read: active,
            supply_temperature_c: numeric,
            psychrometric_minimum_supply_enthalpy_evaluated: active,
            psychrometric_minimum_supply_enthalpy_j_per_kg: numeric,
            source_shaped_two_argument_maximum_evaluated: active,
            maximum_supply_enthalpy_j_per_kg: numeric,
            supply_enthalpy_assignment_performed: active,
            assigned_supply_enthalpy_j_per_kg: numeric,
            resulting_supply_enthalpy_j_per_kg: numeric,
        }
    }
}
