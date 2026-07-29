//! JSON serialization for one CP362 Humidistat mixed-air-limit snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
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
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed":
            snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed,
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification":
            json_number(snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification),
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification_ieee_bits":
            ieee_bits(snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification),
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed":
            snapshot.dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed,
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "mixed_air_humidity_ratio_for_minimum_read":
            snapshot.mixed_air_humidity_ratio_for_minimum_read,
        "mixed_air_humidity_ratio": json_number(snapshot.mixed_air_humidity_ratio),
        "mixed_air_humidity_ratio_ieee_bits": ieee_bits(snapshot.mixed_air_humidity_ratio),
        "supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read":
            snapshot.supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read,
        "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit":
            json_number(snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit),
        "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit_ieee_bits":
            ieee_bits(snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit),
        "source_shaped_two_argument_minimum_evaluated":
            snapshot.source_shaped_two_argument_minimum_evaluated,
        "minimum_supply_humidity_ratio":
            json_number(snapshot.minimum_supply_humidity_ratio),
        "minimum_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.minimum_supply_humidity_ratio),
        "supply_humidity_ratio_assignment_performed":
            snapshot.supply_humidity_ratio_assignment_performed,
        "assigned_supply_humidity_ratio":
            json_number(snapshot.assigned_supply_humidity_ratio),
        "assigned_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.assigned_supply_humidity_ratio),
        "resulting_supply_humidity_ratio":
            json_number(snapshot.resulting_supply_humidity_ratio),
        "resulting_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.resulting_supply_humidity_ratio),
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
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_none_release_serializes_six_null_values_and_bits() {
        let value = snapshot_json(snapshot(None, false));
        assert_eq!(value["predecessor_dehumidification_control_type"], "None");
        assert_eq!(
            value["dehumidification_control_none_case_completed_skip"],
            true
        );
        for field in numeric_fields() {
            assert!(value[field].is_null(), "{field} value");
            assert!(
                value[format!("{field}_ieee_bits")].is_null(),
                "{field} bits"
            );
        }
    }

    #[test]
    fn finite_and_nonfinite_characterization_preserves_exact_bits() {
        for scalar in [-0.0, f64::from_bits(0x7ff8_0000_0000_0062), f64::INFINITY] {
            let value = snapshot_json(snapshot(Some(scalar), true));
            let expected_bits = format!("0x{:016x}", scalar.to_bits());
            for field in numeric_fields() {
                assert_eq!(value[format!("{field}_ieee_bits")], expected_bits);
                if scalar.is_finite() {
                    assert_eq!(value[field], json!(scalar));
                } else {
                    assert!(value[field].is_null(), "{field} JSON number");
                }
            }
        }
    }

    fn numeric_fields() -> [&'static str; 6] {
        [
            "predecessor_resulting_supply_humidity_ratio_for_dehumidification",
            "mixed_air_humidity_ratio",
            "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit",
            "minimum_supply_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ]
    }

    fn snapshot(
        scalar: Option<f64>,
        active: bool,
    ) -> PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot {
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
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
                DehumidificationControlType::Humidistat
            } else {
                DehumidificationControlType::None
            }),
            predecessor_dehumidification_control_none_case_completed_skip: !active,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
                false,
            predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed:
                active,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
                false,
            predecessor_resulting_supply_humidity_ratio_for_dehumidification: scalar,
            dehumidification_control_none_case_completed_skip: !active,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
            dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed:
                active,
            dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: false,
            mixed_air_humidity_ratio_for_minimum_read: active,
            mixed_air_humidity_ratio: scalar,
            supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read: active,
            supply_humidity_ratio_for_dehumidification_before_mixed_air_limit: scalar,
            source_shaped_two_argument_minimum_evaluated: active,
            minimum_supply_humidity_ratio: scalar,
            supply_humidity_ratio_assignment_performed: active,
            assigned_supply_humidity_ratio: scalar,
            resulting_supply_humidity_ratio: scalar,
        }
    }
}
