//! JSON serialization for one CP358 Humidistat case-entry snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(snapshot: PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot) -> Value {
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
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break":
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        "predecessor_dehumidification_control_humidistat_case_selected_skip":
            snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip,
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        "dehumidification_control_none_case_completed_skip":
            snapshot.dehumidification_control_none_case_completed_skip,
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip":
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        "dehumidification_control_humidistat_case_entered":
            snapshot.dehumidification_control_humidistat_case_entered,
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip":
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
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
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE,
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn direct_none_release_serializes_complete_false_entry_skip() {
        let value = snapshot_json(snapshot(false));
        assert_eq!(value["predecessor_dehumidification_control_type"], "None");
        assert_eq!(
            value["dehumidification_control_none_case_completed_skip"],
            true
        );
        assert_eq!(
            value["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
            false
        );
        assert_eq!(
            value["dehumidification_control_humidistat_case_entered"],
            false
        );
    }

    #[test]
    fn active_humidistat_entry_serializes_true_without_numeric_payload() {
        let value = snapshot_json(snapshot(true));
        assert_eq!(
            value["predecessor_dehumidification_control_type"],
            "Humidistat"
        );
        assert_eq!(
            value["dehumidification_control_humidistat_case_entered"],
            true
        );
        assert!(
            value
                .get("assigned_supply_humidity_ratio_ieee_bits")
                .is_none()
        );
        assert!(value.get("MdotZnDehumidSP").is_none());
    }

    fn snapshot(active: bool) -> PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot {
        PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot {
            source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
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
                DehumidificationControlType::Humidistat
            } else {
                DehumidificationControlType::None
            }),
            predecessor_dehumidification_control_none_case_completed_skip: !active,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
                false,
            predecessor_dehumidification_control_humidistat_case_selected_skip: active,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
                false,
            dehumidification_control_none_case_completed_skip: !active,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
            dehumidification_control_humidistat_case_entered: active,
            dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: false,
        }
    }
}
