//! JSON serialization for one CP346 snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
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
        "predecessor_capacity_limit_guard_false_fallthrough":
            snapshot.predecessor_capacity_limit_guard_false_fallthrough,
        "predecessor_capacity_limit_sensible_output_guard_false_fallthrough":
            snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
        "predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed":
            snapshot
                .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
        "predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed":
            snapshot
                .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed,
        "predecessor_assigned_supply_humidity_ratio":
            json_number(snapshot.predecessor_assigned_supply_humidity_ratio),
        "dehumidification_control_type_read":
            snapshot.dehumidification_control_type_read,
        "dehumidification_control_type":
            snapshot.dehumidification_control_type.map(control_type_name),
        "dehumidification_control_switch_dispatched":
            snapshot.dehumidification_control_switch_dispatched,
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

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn serializer_uses_stable_symbolic_control_names_without_ordinals_or_ieee_bits() {
        for (control, expected) in [
            (DehumidificationControlType::None, "None"),
            (
                DehumidificationControlType::ConstantSensibleHeatRatio,
                "ConstantSensibleHeatRatio",
            ),
            (DehumidificationControlType::Humidistat, "Humidistat"),
            (
                DehumidificationControlType::ConstantSupplyHumidityRatio,
                "ConstantSupplyHumidityRatio",
            ),
        ] {
            let value = snapshot_json(snapshot(Some(control)));
            assert_eq!(value["dehumidification_control_type"], expected);
            assert!(value.as_object().is_some_and(|object| {
                object.keys().all(|key| {
                    key != "dehumidification_control_type_ordinal" && !key.contains("ieee_bits")
                })
            }));
        }
    }

    #[test]
    fn serializer_maps_skipped_selector_and_operand_to_null() {
        let value = snapshot_json(snapshot(None));
        assert!(value["dehumidification_control_type"].is_null());
        assert!(value["predecessor_assigned_supply_humidity_ratio"].is_null());
        assert_eq!(value["dehumidification_control_type_read"], false);
        assert_eq!(value["dehumidification_control_switch_dispatched"], false);
    }

    fn snapshot(
        control: Option<DehumidificationControlType>,
    ) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot
    {
        let active = control.is_some();
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: active,
            predecessor_cooling_body_entered: active,
            predecessor_no_outdoor_air_fallback_entered: active,
            predecessor_positive_supply_mass_flow_body_entered: active,
            unit_off_skipped: !active,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            predecessor_capacity_limit_guard_false_fallthrough: active,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: false,
            predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
                false,
            predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed:
                active,
            predecessor_assigned_supply_humidity_ratio: active.then_some(0.008),
            dehumidification_control_type_read: active,
            dehumidification_control_type: control,
            dehumidification_control_switch_dispatched: active,
        }
    }
}
