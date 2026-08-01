//! JSON serialization for one CP371 nested dehumidification-control guard snapshot.

use ep_model::{DehumidificationControlType, HumidificationControlType};
use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot,
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
        "predecessor_dehumidification_control_type": snapshot
            .predecessor_dehumidification_control_type
            .map(dehumidification_control_name),
        "predecessor_dehumidification_control_none_case_completed_skip":
            snapshot.predecessor_dehumidification_control_none_case_completed_skip,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "predecessor_dehumidification_control_humidistat_case_completed_skip":
            snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip,
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        "predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break":
            snapshot.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "dehumidification_control_humidistat_case_completed_skip":
            snapshot.dehumidification_control_humidistat_case_completed_skip,
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        "predecessor_heating_on_read": snapshot.predecessor_heating_on_read,
        "predecessor_heating_on": snapshot.predecessor_heating_on,
        "predecessor_cooling_supply_humidity_ratio_humidification_body_entered":
            snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        "predecessor_heating_on_guard_false_fallthrough":
            snapshot.predecessor_heating_on_guard_false_fallthrough,
        "predecessor_humidification_control_type_read":
            snapshot.predecessor_humidification_control_type_read,
        "predecessor_humidification_control_type": snapshot
            .predecessor_humidification_control_type
            .map(humidification_control_name),
        "predecessor_humidification_control_type_humidistat":
            snapshot.predecessor_humidification_control_type_humidistat,
        "predecessor_humidification_control_body_entered":
            snapshot.predecessor_humidification_control_body_entered,
        "predecessor_humidification_control_guard_false_fallthrough":
            snapshot.predecessor_humidification_control_guard_false_fallthrough,
        "dehumidification_control_type_first_read":
            snapshot.dehumidification_control_type_first_read,
        "first_dehumidification_control_type": snapshot
            .first_dehumidification_control_type
            .map(dehumidification_control_name),
        "dehumidification_control_type_humidistat":
            snapshot.dehumidification_control_type_humidistat,
        "dehumidification_control_type_second_read":
            snapshot.dehumidification_control_type_second_read,
        "second_dehumidification_control_type": snapshot
            .second_dehumidification_control_type
            .map(dehumidification_control_name),
        "dehumidification_control_type_none": snapshot.dehumidification_control_type_none,
        "dehumidification_control_body_entered":
            snapshot.dehumidification_control_body_entered,
        "dehumidification_control_guard_false_fallthrough":
            snapshot.dehumidification_control_guard_false_fallthrough,
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

fn humidification_control_name(control: HumidificationControlType) -> &'static str {
    match control {
        HumidificationControlType::None => "None",
        HumidificationControlType::ConstantSupplyHumidityRatio => "ConstantSupplyHumidityRatio",
        HumidificationControlType::Humidistat => "Humidistat",
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_outer_false_serializes_zero_current_sites() {
        let value = snapshot_json(direct_snapshot());
        assert_eq!(value["predecessor_humidification_control_type"], "None");
        assert_eq!(value["dehumidification_control_type_first_read"], false);
        assert!(value["first_dehumidification_control_type"].is_null());
        assert_eq!(value["dehumidification_control_type_second_read"], false);
        assert!(value["second_dehumidification_control_type"].is_null());
        assert!(value["dehumidification_control_type_none"].is_null());
        assert_eq!(value["dehumidification_control_body_entered"], false);
        assert_eq!(
            value["dehumidification_control_guard_false_fallthrough"],
            false
        );
        assert_eq!(
            value["source_order"]
                .as_array()
                .expect("source order")
                .len(),
            5
        );
    }

    #[test]
    fn canonical_private_none_route_serializes_both_reads_and_body() {
        let mut snapshot = direct_snapshot();
        snapshot.predecessor_humidification_control_type =
            Some(HumidificationControlType::Humidistat);
        snapshot.predecessor_humidification_control_type_humidistat = Some(true);
        snapshot.predecessor_humidification_control_body_entered = true;
        snapshot.predecessor_humidification_control_guard_false_fallthrough = false;
        snapshot.dehumidification_control_type_first_read = true;
        snapshot.first_dehumidification_control_type = Some(DehumidificationControlType::None);
        snapshot.dehumidification_control_type_humidistat = Some(false);
        snapshot.dehumidification_control_type_second_read = true;
        snapshot.second_dehumidification_control_type = Some(DehumidificationControlType::None);
        snapshot.dehumidification_control_type_none = Some(true);
        snapshot.dehumidification_control_body_entered = true;
        let value = snapshot_json(snapshot);
        assert_eq!(value["first_dehumidification_control_type"], "None");
        assert_eq!(value["dehumidification_control_type_humidistat"], false);
        assert_eq!(value["second_dehumidification_control_type"], "None");
        assert_eq!(value["dehumidification_control_type_none"], true);
        assert_eq!(value["dehumidification_control_body_entered"], true);
    }

    #[test]
    fn control_only_snapshot_exposes_no_numeric_or_ieee_sidecar() {
        let value = snapshot_json(direct_snapshot());
        let object = value.as_object().expect("snapshot object");
        for key in object.keys() {
            assert!(!key.ends_with("_ieee_bits"), "{key}");
        }
        for forbidden in [
            "mixed_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
            "supply_humidity_ratio",
        ] {
            assert!(!object.contains_key(forbidden), "{forbidden}");
        }
    }

    fn direct_snapshot() -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot{
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
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
            predecessor_dehumidification_control_type: Some(DehumidificationControlType::None),
            predecessor_dehumidification_control_none_case_completed_skip: true,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
            predecessor_dehumidification_control_humidistat_case_completed_skip: false,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
            predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: false,
            dehumidification_control_none_case_completed_skip: true,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
            dehumidification_control_humidistat_case_completed_skip: false,
            dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
            predecessor_heating_on_read: true,
            predecessor_heating_on: Some(true),
            predecessor_cooling_supply_humidity_ratio_humidification_body_entered: true,
            predecessor_heating_on_guard_false_fallthrough: false,
            predecessor_humidification_control_type_read: true,
            predecessor_humidification_control_type: Some(HumidificationControlType::None),
            predecessor_humidification_control_type_humidistat: Some(false),
            predecessor_humidification_control_body_entered: false,
            predecessor_humidification_control_guard_false_fallthrough: true,
            dehumidification_control_type_first_read: false,
            first_dehumidification_control_type: None,
            dehumidification_control_type_humidistat: None,
            dehumidification_control_type_second_read: false,
            second_dehumidification_control_type: None,
            dehumidification_control_type_none: None,
            dehumidification_control_body_entered: false,
            dehumidification_control_guard_false_fallthrough: false,
        }
    }
}
