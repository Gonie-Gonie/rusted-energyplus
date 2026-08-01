//! JSON serialization for one CP378 snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
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
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "heating_availability_guard_false_fallthrough":
            snapshot.heating_availability_guard_false_fallthrough,
        "humidification_control_guard_false_fallthrough":
            snapshot.humidification_control_guard_false_fallthrough,
        "dehumidification_control_humidistat_maximum_assignment_executed":
            snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        "dehumidification_control_none_maximum_assignment_executed":
            snapshot.dehumidification_control_none_maximum_assignment_executed,
        "dehumidification_control_guard_false_fallthrough":
            snapshot.dehumidification_control_guard_false_fallthrough,
        "predecessor_dehumidification_control_type": snapshot
            .predecessor_dehumidification_control_type
            .map(dehumidification_control_name),
        "predecessor_local_supply_humidity_ratio_original_assignment_performed":
            snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed,
        "predecessor_resulting_supply_humidity_ratio_original":
            json_number(snapshot.predecessor_resulting_supply_humidity_ratio_original),
        "predecessor_resulting_supply_humidity_ratio_original_ieee_bits":
            ieee_bits(snapshot.predecessor_resulting_supply_humidity_ratio_original),
        "predecessor_local_saturation_supply_humidity_ratio_assignment_performed":
            snapshot.predecessor_local_saturation_supply_humidity_ratio_assignment_performed,
        "predecessor_resulting_saturation_supply_humidity_ratio":
            json_number(snapshot.predecessor_resulting_saturation_supply_humidity_ratio),
        "predecessor_resulting_saturation_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.predecessor_resulting_saturation_supply_humidity_ratio),
        "cp376_original_supply_humidity_ratio_owned_read":
            snapshot.cp376_original_supply_humidity_ratio_owned_read,
        "cp377_saturation_supply_humidity_ratio_owned_read":
            snapshot.cp377_saturation_supply_humidity_ratio_owned_read,
        "local_original_supply_humidity_ratio_for_saturation_limit_minimum_read":
            snapshot.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read,
        "original_supply_humidity_ratio_before_saturation_limit":
            json_number(snapshot.original_supply_humidity_ratio_before_saturation_limit),
        "original_supply_humidity_ratio_before_saturation_limit_ieee_bits":
            ieee_bits(snapshot.original_supply_humidity_ratio_before_saturation_limit),
        "local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read":
            snapshot.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read,
        "saturation_supply_humidity_ratio_for_limit":
            json_number(snapshot.saturation_supply_humidity_ratio_for_limit),
        "saturation_supply_humidity_ratio_for_limit_ieee_bits":
            ieee_bits(snapshot.saturation_supply_humidity_ratio_for_limit),
        "source_shaped_two_argument_minimum_evaluated":
            snapshot.source_shaped_two_argument_minimum_evaluated,
        "minimum_supply_humidity_ratio_after_saturation_limit":
            json_number(snapshot.minimum_supply_humidity_ratio_after_saturation_limit),
        "minimum_supply_humidity_ratio_after_saturation_limit_ieee_bits":
            ieee_bits(snapshot.minimum_supply_humidity_ratio_after_saturation_limit),
        "purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed":
            snapshot.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed,
        "assigned_supply_humidity_ratio": json_number(snapshot.assigned_supply_humidity_ratio),
        "assigned_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.assigned_supply_humidity_ratio),
        "resulting_supply_humidity_ratio": json_number(snapshot.resulting_supply_humidity_ratio),
        "resulting_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.resulting_supply_humidity_ratio),
    })
}

fn dehumidification_control_name(control: DehumidificationControlType) -> &'static str {
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
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn non_finite_values_are_null_with_exact_ieee_sidecars() {
        let bits = 0x7ff8_0000_0000_0378;
        let value = f64::from_bits(bits);
        let snapshot =
            PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot {
                source:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
                first_excluded_source:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
                source_order:
                    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
                system: IdealLoadsAirSystemId(0),
                parent_call_ordinal: 1,
                controlled_zone: ZoneId(0),
                unit_off_skipped: false,
                non_cooling_skipped: false,
                positive_guard_false_fallthrough_skipped: false,
                heating_availability_guard_false_fallthrough: true,
                humidification_control_guard_false_fallthrough: false,
                dehumidification_control_humidistat_maximum_assignment_executed: false,
                dehumidification_control_none_maximum_assignment_executed: false,
                dehumidification_control_guard_false_fallthrough: false,
                predecessor_dehumidification_control_type:
                    Some(DehumidificationControlType::None),
                predecessor_local_supply_humidity_ratio_original_assignment_performed: true,
                predecessor_resulting_supply_humidity_ratio_original: Some(value),
                predecessor_local_saturation_supply_humidity_ratio_assignment_performed: true,
                predecessor_resulting_saturation_supply_humidity_ratio: Some(value),
                cp376_original_supply_humidity_ratio_owned_read: true,
                cp377_saturation_supply_humidity_ratio_owned_read: true,
                local_original_supply_humidity_ratio_for_saturation_limit_minimum_read: true,
                original_supply_humidity_ratio_before_saturation_limit: Some(value),
                local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read: true,
                saturation_supply_humidity_ratio_for_limit: Some(value),
                source_shaped_two_argument_minimum_evaluated: true,
                minimum_supply_humidity_ratio_after_saturation_limit: Some(value),
                purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed: true,
                assigned_supply_humidity_ratio: Some(value),
                resulting_supply_humidity_ratio: Some(value),
            };
        let json = snapshot_json(snapshot);
        let expected_bits = format!("0x{bits:016x}");
        for field in [
            "predecessor_resulting_supply_humidity_ratio_original",
            "predecessor_resulting_saturation_supply_humidity_ratio",
            "original_supply_humidity_ratio_before_saturation_limit",
            "saturation_supply_humidity_ratio_for_limit",
            "minimum_supply_humidity_ratio_after_saturation_limit",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ] {
            assert!(json[field].is_null(), "{field}");
            assert_eq!(json[format!("{field}_ieee_bits")], expected_bits, "{field}");
        }
    }
}
