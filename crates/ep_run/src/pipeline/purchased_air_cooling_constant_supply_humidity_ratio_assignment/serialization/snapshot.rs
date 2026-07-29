//! JSON serialization for one CP365 constant-supply-humidity-ratio assignment.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
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
        "predecessor_dehumidification_control_humidistat_case_completed_skip":
            snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip,
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered,
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "dehumidification_control_humidistat_case_completed_skip":
            snapshot.dehumidification_control_humidistat_case_completed_skip,
        "dehumidification_control_constant_supply_humidity_ratio_assignment_executed":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_assignment_executed,
        "minimum_cooling_supply_air_humidity_ratio_read":
            snapshot.minimum_cooling_supply_air_humidity_ratio_read,
        "minimum_cooling_supply_air_humidity_ratio":
            json_number(snapshot.minimum_cooling_supply_air_humidity_ratio),
        "minimum_cooling_supply_air_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.minimum_cooling_supply_air_humidity_ratio),
        "supply_humidity_ratio_assigned":
            snapshot.supply_humidity_ratio_assigned,
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
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_none_release_serializes_complete_null_skip() {
        let value = snapshot_json(snapshot(None, false));
        assert_eq!(value["predecessor_dehumidification_control_type"], "None");
        assert_eq!(
            value["dehumidification_control_none_case_completed_skip"],
            true
        );
        assert_eq!(
            value["dehumidification_control_constant_supply_humidity_ratio_assignment_executed"],
            false
        );
        for field in [
            "minimum_cooling_supply_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
            "minimum_cooling_supply_air_humidity_ratio_ieee_bits",
            "assigned_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
        ] {
            assert!(value[field].is_null(), "{field}");
        }
    }

    #[test]
    fn active_assignment_serializes_finite_value_and_authoritative_bits() {
        let value = snapshot_json(snapshot(Some(-0.0), true));
        for field in [
            "minimum_cooling_supply_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ] {
            assert!(value[field].is_number(), "{field}");
            assert_eq!(value[format!("{field}_ieee_bits")], "0x8000000000000000");
        }
    }

    #[test]
    fn defensive_nonfinite_characterization_keeps_bits_but_projects_null_number() {
        let payload = f64::from_bits(0x7ff8_0000_0000_0365);
        let value = snapshot_json(snapshot(Some(payload), true));
        for field in [
            "minimum_cooling_supply_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ] {
            assert!(value[field].is_null(), "{field}");
            assert_eq!(value[format!("{field}_ieee_bits")], "0x7ff8000000000365");
        }
    }

    fn snapshot(
        value: Option<f64>,
        active: bool,
    ) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
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
                DehumidificationControlType::ConstantSupplyHumidityRatio
            } else {
                DehumidificationControlType::None
            }),
            predecessor_dehumidification_control_none_case_completed_skip: !active,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
                false,
            predecessor_dehumidification_control_humidistat_case_completed_skip: false,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered:
                active,
            dehumidification_control_none_case_completed_skip: !active,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
            dehumidification_control_humidistat_case_completed_skip: false,
            dehumidification_control_constant_supply_humidity_ratio_assignment_executed: active,
            minimum_cooling_supply_air_humidity_ratio_read: active,
            minimum_cooling_supply_air_humidity_ratio: value,
            supply_humidity_ratio_assigned: active,
            assigned_supply_humidity_ratio: value,
            resulting_supply_humidity_ratio: value,
        }
    }
}
