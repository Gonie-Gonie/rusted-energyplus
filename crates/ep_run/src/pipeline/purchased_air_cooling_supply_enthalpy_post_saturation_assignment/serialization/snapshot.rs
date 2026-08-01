//! JSON serialization for one CP379 snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
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
        "predecessor_supply_humidity_ratio_saturation_limit_assignment_performed":
            snapshot.predecessor_supply_humidity_ratio_saturation_limit_assignment_performed,
        "predecessor_resulting_supply_humidity_ratio":
            json_number(snapshot.predecessor_resulting_supply_humidity_ratio),
        "predecessor_resulting_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.predecessor_resulting_supply_humidity_ratio),
        "cp377_supply_temperature_owned_read": snapshot.cp377_supply_temperature_owned_read,
        "cp334_supply_temperature_mixed_air_limit_owned_read":
            snapshot.cp334_supply_temperature_mixed_air_limit_owned_read,
        "cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read":
            snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
        "cp378_supply_humidity_ratio_saturation_limit_owned_read":
            snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read,
        "purchased_air_supply_temperature_for_post_saturation_enthalpy_read":
            snapshot.purchased_air_supply_temperature_for_post_saturation_enthalpy_read,
        "supply_temperature_c": json_number(snapshot.supply_temperature_c),
        "supply_temperature_c_ieee_bits": ieee_bits(snapshot.supply_temperature_c),
        "purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read":
            snapshot.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read,
        "supply_humidity_ratio": json_number(snapshot.supply_humidity_ratio),
        "supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.supply_humidity_ratio),
        "psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated":
            snapshot.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated,
        "psychrometric_supply_enthalpy_j_per_kg":
            json_number(snapshot.psychrometric_supply_enthalpy_j_per_kg),
        "psychrometric_supply_enthalpy_j_per_kg_ieee_bits":
            ieee_bits(snapshot.psychrometric_supply_enthalpy_j_per_kg),
        "local_supply_enthalpy_after_saturation_limit_assignment_performed":
            snapshot.local_supply_enthalpy_after_saturation_limit_assignment_performed,
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
        PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn non_finite_values_are_null_with_exact_ieee_sidecars() {
        let bits = 0x7ff8_0000_0000_0379;
        let value = f64::from_bits(bits);
        let snapshot = PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
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
            predecessor_supply_humidity_ratio_saturation_limit_assignment_performed: true,
            predecessor_resulting_supply_humidity_ratio: Some(value),
            cp377_supply_temperature_owned_read: true,
            cp334_supply_temperature_mixed_air_limit_owned_read: true,
            cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: false,
            cp378_supply_humidity_ratio_saturation_limit_owned_read: true,
            purchased_air_supply_temperature_for_post_saturation_enthalpy_read: true,
            supply_temperature_c: Some(value),
            purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read: true,
            supply_humidity_ratio: Some(value),
            psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated: true,
            psychrometric_supply_enthalpy_j_per_kg: Some(value),
            local_supply_enthalpy_after_saturation_limit_assignment_performed: true,
            assigned_supply_enthalpy_j_per_kg: Some(value),
            resulting_supply_enthalpy_j_per_kg: Some(value),
        };
        let json = snapshot_json(snapshot);
        let expected_bits = format!("0x{bits:016x}");
        for field in [
            "predecessor_resulting_supply_humidity_ratio",
            "supply_temperature_c",
            "supply_humidity_ratio",
            "psychrometric_supply_enthalpy_j_per_kg",
            "assigned_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ] {
            assert!(json[field].is_null(), "{field}");
            assert_eq!(json[format!("{field}_ieee_bits")], expected_bits, "{field}");
        }
    }
}
