//! JSON serialization for one CP377 saturation-assignment snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
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
        "cp334_supply_temperature_mixed_air_limit_owned_read":
            snapshot.cp334_supply_temperature_mixed_air_limit_owned_read,
        "cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read":
            snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
        "environment_outdoor_barometric_pressure_owned_read":
            snapshot.environment_outdoor_barometric_pressure_owned_read,
        "purchased_air_supply_temperature_for_saturation_humidity_ratio_read":
            snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
        "supply_temperature_for_saturation_humidity_ratio_c":
            json_number(snapshot.supply_temperature_for_saturation_humidity_ratio_c),
        "supply_temperature_for_saturation_humidity_ratio_c_ieee_bits":
            ieee_bits(snapshot.supply_temperature_for_saturation_humidity_ratio_c),
        "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read":
            snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
        "outdoor_barometric_pressure_pa": json_number(snapshot.outdoor_barometric_pressure_pa),
        "outdoor_barometric_pressure_pa_ieee_bits":
            ieee_bits(snapshot.outdoor_barometric_pressure_pa),
        "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated":
            snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
        "saturation_supply_humidity_ratio":
            json_number(snapshot.saturation_supply_humidity_ratio),
        "saturation_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.saturation_supply_humidity_ratio),
        "local_saturation_supply_humidity_ratio_assignment_performed":
            snapshot.local_saturation_supply_humidity_ratio_assignment_performed,
        "assigned_saturation_supply_humidity_ratio":
            json_number(snapshot.assigned_saturation_supply_humidity_ratio),
        "assigned_saturation_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.assigned_saturation_supply_humidity_ratio),
        "resulting_saturation_supply_humidity_ratio":
            json_number(snapshot.resulting_saturation_supply_humidity_ratio),
        "resulting_saturation_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.resulting_saturation_supply_humidity_ratio),
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
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn non_finite_private_values_are_null_with_exact_ieee_sidecars() {
        let bits = 0x7ff8_0000_0000_0377;
        let value = f64::from_bits(bits);
        let snapshot = PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
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
            predecessor_local_supply_humidity_ratio_original_assignment_performed: true,
            predecessor_resulting_supply_humidity_ratio_original: Some(value),
            cp334_supply_temperature_mixed_air_limit_owned_read: true,
            cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: false,
            environment_outdoor_barometric_pressure_owned_read: true,
            purchased_air_supply_temperature_for_saturation_humidity_ratio_read: true,
            supply_temperature_for_saturation_humidity_ratio_c: Some(value),
            environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: true,
            outdoor_barometric_pressure_pa: Some(value),
            psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: true,
            saturation_supply_humidity_ratio: Some(value),
            local_saturation_supply_humidity_ratio_assignment_performed: true,
            assigned_saturation_supply_humidity_ratio: Some(value),
            resulting_saturation_supply_humidity_ratio: Some(value),
        };
        let json = snapshot_json(snapshot);
        let expected_bits = format!("0x{bits:016x}");
        for field in [
            "predecessor_resulting_supply_humidity_ratio_original",
            "supply_temperature_for_saturation_humidity_ratio_c",
            "outdoor_barometric_pressure_pa",
            "saturation_supply_humidity_ratio",
            "assigned_saturation_supply_humidity_ratio",
            "resulting_saturation_supply_humidity_ratio",
        ] {
            assert!(json[field].is_null(), "{field}");
            assert_eq!(json[format!("{field}_ieee_bits")], expected_bits, "{field}");
        }
    }
}
