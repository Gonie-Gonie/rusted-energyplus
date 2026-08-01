//! JSON serialization for one CP370 humidification-control Humidistat-guard snapshot.

use ep_model::{DehumidificationControlType, HumidificationControlType};
use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
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
            snapshot.predecessor_dehumidification_control_type.map(dehumidification_control_name),
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
        "humidification_control_type_read": snapshot.humidification_control_type_read,
        "humidification_control_type":
            snapshot.humidification_control_type.map(humidification_control_name),
        "humidification_control_type_humidistat":
            snapshot.humidification_control_type_humidistat,
        "humidification_control_body_entered": snapshot.humidification_control_body_entered,
        "humidification_control_guard_false_fallthrough":
            snapshot.humidification_control_guard_false_fallthrough,
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
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_none_serializes_false_guard_with_two_sites() {
        let value = snapshot_json(snapshot(HumidificationControlType::None));
        assert_eq!(value["humidification_control_type"], "None");
        assert_eq!(value["humidification_control_type_humidistat"], false);
        assert_eq!(value["humidification_control_body_entered"], false);
        assert_eq!(
            value["humidification_control_guard_false_fallthrough"],
            true
        );
        assert_eq!(
            value["source_order"]
                .as_array()
                .expect("source-order array")
                .len(),
            3
        );
    }

    #[test]
    fn private_humidistat_serializes_true_body_with_three_sites() {
        let value = snapshot_json(snapshot(HumidificationControlType::Humidistat));
        assert_eq!(value["humidification_control_type"], "Humidistat");
        assert_eq!(value["humidification_control_type_humidistat"], true);
        assert_eq!(value["humidification_control_body_entered"], true);
        assert_eq!(
            value["humidification_control_guard_false_fallthrough"],
            false
        );
    }

    #[test]
    fn control_only_snapshot_exposes_no_numeric_humidity_payload() {
        let value = snapshot_json(snapshot(
            HumidificationControlType::ConstantSupplyHumidityRatio,
        ));
        assert_eq!(
            value["humidification_control_type"],
            "ConstantSupplyHumidityRatio"
        );
        let object = value.as_object().expect("snapshot object");
        for forbidden in [
            "mixed_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
            "supply_humidity_ratio",
            "humidification_control_type_ieee_bits",
            "assigned_supply_humidity_ratio_ieee_bits",
        ] {
            assert!(!object.contains_key(forbidden), "{forbidden}");
        }
    }

    fn snapshot(
        control: HumidificationControlType,
    ) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot
    {
        let humidistat = control == HumidificationControlType::Humidistat;
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
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
            predecessor_dehumidification_control_type:
                Some(DehumidificationControlType::None),
            predecessor_dehumidification_control_none_case_completed_skip: true,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
                false,
            predecessor_dehumidification_control_humidistat_case_completed_skip: false,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
                false,
            predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
                false,
            dehumidification_control_none_case_completed_skip: true,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
            dehumidification_control_humidistat_case_completed_skip: false,
            dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: false,
            predecessor_heating_on_read: true,
            predecessor_heating_on: Some(true),
            predecessor_cooling_supply_humidity_ratio_humidification_body_entered: true,
            predecessor_heating_on_guard_false_fallthrough: false,
            humidification_control_type_read: true,
            humidification_control_type: Some(control),
            humidification_control_type_humidistat: Some(humidistat),
            humidification_control_body_entered: humidistat,
            humidification_control_guard_false_fallthrough: !humidistat,
        }
    }
}
