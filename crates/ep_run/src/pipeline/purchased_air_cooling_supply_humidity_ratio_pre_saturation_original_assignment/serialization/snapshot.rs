//! JSON serialization for one CP376 pre-saturation original-assignment snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
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
        "predecessor_purchased_air_supply_humidity_ratio_assignment_performed":
            snapshot.predecessor_purchased_air_supply_humidity_ratio_assignment_performed,
        "predecessor_resulting_supply_humidity_ratio":
            json_number(snapshot.predecessor_resulting_supply_humidity_ratio),
        "predecessor_resulting_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.predecessor_resulting_supply_humidity_ratio),
        "cp375_maximum_assignment_owned_read": snapshot.cp375_maximum_assignment_owned_read,
        "cp347_none_case_owned_read": snapshot.cp347_none_case_owned_read,
        "cp356_constant_shr_owned_read": snapshot.cp356_constant_shr_owned_read,
        "cp362_humidistat_owned_read": snapshot.cp362_humidistat_owned_read,
        "cp365_constant_supply_humidity_ratio_owned_read":
            snapshot.cp365_constant_supply_humidity_ratio_owned_read,
        "purchased_air_supply_humidity_ratio_read":
            snapshot.purchased_air_supply_humidity_ratio_read,
        "purchased_air_supply_humidity_ratio_before_saturation_check":
            json_number(snapshot.purchased_air_supply_humidity_ratio_before_saturation_check),
        "purchased_air_supply_humidity_ratio_before_saturation_check_ieee_bits":
            ieee_bits(snapshot.purchased_air_supply_humidity_ratio_before_saturation_check),
        "local_supply_humidity_ratio_original_assignment_performed":
            snapshot.local_supply_humidity_ratio_original_assignment_performed,
        "assigned_supply_humidity_ratio_original":
            json_number(snapshot.assigned_supply_humidity_ratio_original),
        "assigned_supply_humidity_ratio_original_ieee_bits":
            ieee_bits(snapshot.assigned_supply_humidity_ratio_original),
        "resulting_supply_humidity_ratio_original":
            json_number(snapshot.resulting_supply_humidity_ratio_original),
        "resulting_supply_humidity_ratio_original_ieee_bits":
            ieee_bits(snapshot.resulting_supply_humidity_ratio_original),
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
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn non_finite_copy_is_null_with_exact_ieee_sidecars() {
        let bits = 0x7ff8_0000_0000_0376;
        let value = f64::from_bits(bits);
        let snapshot = PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
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
            predecessor_dehumidification_control_type: Some(DehumidificationControlType::None),
            predecessor_purchased_air_supply_humidity_ratio_assignment_performed: false,
            predecessor_resulting_supply_humidity_ratio: None,
            cp375_maximum_assignment_owned_read: false,
            cp347_none_case_owned_read: true,
            cp356_constant_shr_owned_read: false,
            cp362_humidistat_owned_read: false,
            cp365_constant_supply_humidity_ratio_owned_read: false,
            purchased_air_supply_humidity_ratio_read: true,
            purchased_air_supply_humidity_ratio_before_saturation_check: Some(value),
            local_supply_humidity_ratio_original_assignment_performed: true,
            assigned_supply_humidity_ratio_original: Some(value),
            resulting_supply_humidity_ratio_original: Some(value),
        };
        let json = snapshot_json(snapshot);
        let expected_bits = format!("0x{bits:016x}");

        for field in [
            "purchased_air_supply_humidity_ratio_before_saturation_check",
            "assigned_supply_humidity_ratio_original",
            "resulting_supply_humidity_ratio_original",
        ] {
            assert!(json[field].is_null(), "{field}");
            assert_eq!(json[format!("{field}_ieee_bits")], expected_bits, "{field}");
        }
        assert!(json["predecessor_resulting_supply_humidity_ratio"].is_null());
        assert!(json["predecessor_resulting_supply_humidity_ratio_ieee_bits"].is_null());
    }
}
