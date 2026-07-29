//! JSON serialization for one CP366 constant-supply-humidity-ratio case break.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot,
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
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed,
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "dehumidification_control_humidistat_case_completed_skip":
            snapshot.dehumidification_control_humidistat_case_completed_skip,
        "dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break,
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
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_none_release_serializes_complete_false_break_skip() {
        let value = snapshot_json(snapshot(false));
        assert_eq!(value["predecessor_dehumidification_control_type"], "None");
        assert_eq!(
            value["dehumidification_control_none_case_completed_skip"],
            true
        );
        assert_eq!(
            value["dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break"],
            false
        );
    }

    #[test]
    fn active_constant_supply_break_has_no_numeric_payload_or_default_fallthrough() {
        let value = snapshot_json(snapshot(true));
        assert_eq!(
            value["predecessor_dehumidification_control_type"],
            "ConstantSupplyHumidityRatio"
        );
        assert_eq!(
            value["dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break"],
            true
        );
        let object = value.as_object().expect("snapshot object");
        for forbidden in [
            "minimum_cooling_supply_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
            "minimum_cooling_supply_air_humidity_ratio_ieee_bits",
            "assigned_supply_humidity_ratio_ieee_bits",
            "resulting_supply_humidity_ratio_ieee_bits",
            "default_case_entered",
            "default_case_fallthrough",
        ] {
            assert!(!object.contains_key(forbidden), "{forbidden}");
        }
    }

    fn snapshot(
        active: bool,
    ) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot {
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
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
            predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed:
                active,
            dehumidification_control_none_case_completed_skip: !active,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
            dehumidification_control_humidistat_case_completed_skip: false,
            dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break: active,
        }
    }
}
