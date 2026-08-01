//! JSON serialization for one CP369 heating-availability guard snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot,
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
        "heating_on_read": snapshot.heating_on_read,
        "heating_on": snapshot.heating_on,
        "cooling_supply_humidity_ratio_humidification_body_entered":
            snapshot.cooling_supply_humidity_ratio_humidification_body_entered,
        "heating_on_guard_false_fallthrough":
            snapshot.heating_on_guard_false_fallthrough,
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

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_true_body_serializes_exact_two_site_guard() {
        let value = snapshot_json(snapshot(Some(DehumidificationControlType::None), true));
        assert_eq!(value["predecessor_dehumidification_control_type"], "None");
        assert_eq!(value["heating_on_read"], true);
        assert_eq!(value["heating_on"], true);
        assert_eq!(
            value["cooling_supply_humidity_ratio_humidification_body_entered"],
            true
        );
        assert_eq!(value["heating_on_guard_false_fallthrough"], false);
    }

    #[test]
    fn false_guard_serializes_fallthrough_without_body() {
        let value = snapshot_json(snapshot(Some(DehumidificationControlType::None), false));
        assert_eq!(value["heating_on"], false);
        assert_eq!(
            value["cooling_supply_humidity_ratio_humidification_body_entered"],
            false
        );
        assert_eq!(value["heating_on_guard_false_fallthrough"], true);
    }

    #[test]
    fn typed_routes_are_orthogonal_and_numeric_payload_is_excluded() {
        let value = snapshot_json(snapshot(
            Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
            true,
        ));
        assert_eq!(
            value["predecessor_dehumidification_control_type"],
            "ConstantSupplyHumidityRatio"
        );
        assert_eq!(
            value["dehumidification_control_constant_supply_humidity_ratio_case_completed_skip"],
            true
        );
        let object = value.as_object().expect("snapshot object");
        for forbidden in [
            "mixed_air_humidity_ratio",
            "minimum_cooling_supply_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
            "humidification_control_type",
            "supply_humidity_ratio",
        ] {
            assert!(!object.contains_key(forbidden), "{forbidden}");
        }
    }

    fn snapshot(
        control: Option<DehumidificationControlType>,
        heating_on: bool,
    ) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot
    {
        let active = control.is_some();
        let none_case = control == Some(DehumidificationControlType::None);
        let constant_shr = control == Some(DehumidificationControlType::ConstantSensibleHeatRatio);
        let humidistat = control == Some(DehumidificationControlType::Humidistat);
        let constant_supply =
            control == Some(DehumidificationControlType::ConstantSupplyHumidityRatio);
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
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
            predecessor_dehumidification_control_type: control,
            predecessor_dehumidification_control_none_case_completed_skip: none_case,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
                constant_shr,
            predecessor_dehumidification_control_humidistat_case_completed_skip: humidistat,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
                constant_supply,
            predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
                false,
            dehumidification_control_none_case_completed_skip: none_case,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: constant_shr,
            dehumidification_control_humidistat_case_completed_skip: humidistat,
            dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
                constant_supply,
            heating_on_read: active,
            heating_on: active.then_some(heating_on),
            cooling_supply_humidity_ratio_humidification_body_entered:
                active && heating_on,
            heating_on_guard_false_fallthrough: active && !heating_on,
        }
    }
}
