//! JSON serialization for one CP347 snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
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
        "predecessor_capacity_limit_guard_false_fallthrough":
            snapshot.predecessor_capacity_limit_guard_false_fallthrough,
        "predecessor_capacity_limit_sensible_output_guard_false_fallthrough":
            snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
        "predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed":
            snapshot
                .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
        "predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed":
            snapshot
                .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed,
        "predecessor_assigned_supply_humidity_ratio":
            json_number(snapshot.predecessor_assigned_supply_humidity_ratio),
        "predecessor_assigned_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.predecessor_assigned_supply_humidity_ratio),
        "predecessor_dehumidification_control_type_read":
            snapshot.predecessor_dehumidification_control_type_read,
        "predecessor_dehumidification_control_type":
            snapshot
                .predecessor_dehumidification_control_type
                .map(control_type_name),
        "predecessor_dehumidification_control_switch_dispatched":
            snapshot.predecessor_dehumidification_control_switch_dispatched,
        "dehumidification_control_none_case_entered":
            snapshot.dehumidification_control_none_case_entered,
        "mixed_air_humidity_ratio_read": snapshot.mixed_air_humidity_ratio_read,
        "mixed_air_humidity_ratio": json_number(snapshot.mixed_air_humidity_ratio),
        "mixed_air_humidity_ratio_ieee_bits": ieee_bits(snapshot.mixed_air_humidity_ratio),
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
        "dehumidification_control_none_case_exited_via_break":
            snapshot.dehumidification_control_none_case_exited_via_break,
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
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
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
            let value = snapshot_json(snapshot_for_control(control, Some(0.008)));
            assert_eq!(value["predecessor_dehumidification_control_type"], expected);
            assert!(
                value["predecessor_dehumidification_control_type"]
                    .as_u64()
                    .is_none()
            );
        }
    }

    #[test]
    fn humidity_values_keep_authoritative_exact_bits_even_when_json_number_is_null() {
        let humidity = f64::from_bits(0x7ff8_0000_0000_0042);
        let value = snapshot_json(snapshot_for_control(
            DehumidificationControlType::None,
            Some(humidity),
        ));
        for field in [
            "predecessor_assigned_supply_humidity_ratio",
            "mixed_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ] {
            assert!(value[field].is_null());
            assert_eq!(value[format!("{field}_ieee_bits")], "0x7ff8000000000042");
        }
    }

    #[test]
    fn absent_humidity_values_serialize_as_null_values_and_null_bits() {
        let value = snapshot_json(snapshot_for_control(
            DehumidificationControlType::None,
            None,
        ));
        for field in [
            "predecessor_assigned_supply_humidity_ratio",
            "mixed_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ] {
            assert!(value[field].is_null());
            assert!(value[format!("{field}_ieee_bits")].is_null());
        }
    }

    fn snapshot_for_control(
        control: DehumidificationControlType,
        humidity: Option<f64>,
    ) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot
    {
        let none_case = control == DehumidificationControlType::None;
        let none_case_humidity = none_case.then_some(humidity).flatten();
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
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
            predecessor_capacity_limit_guard_false_fallthrough: true,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: false,
            predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
                false,
            predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed:
                true,
            predecessor_assigned_supply_humidity_ratio: humidity,
            predecessor_dehumidification_control_type_read: true,
            predecessor_dehumidification_control_type: Some(control),
            predecessor_dehumidification_control_switch_dispatched: true,
            dehumidification_control_none_case_entered: none_case,
            mixed_air_humidity_ratio_read: none_case,
            mixed_air_humidity_ratio: none_case_humidity,
            supply_humidity_ratio_assignment_performed: none_case,
            assigned_supply_humidity_ratio: none_case_humidity,
            resulting_supply_humidity_ratio: none_case_humidity,
            dehumidification_control_none_case_exited_via_break: none_case,
        }
    }
}
