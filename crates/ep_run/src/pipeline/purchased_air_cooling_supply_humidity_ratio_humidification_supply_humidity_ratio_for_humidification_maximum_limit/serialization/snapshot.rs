//! JSON serialization for one CP374 maximum-limit snapshot.

use ep_model::{DehumidificationControlType, HumidificationControlType};
use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot,
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
        "predecessor_dehumidification_control_type_first_read":
            snapshot.predecessor_dehumidification_control_type_first_read,
        "predecessor_first_dehumidification_control_type": snapshot
            .predecessor_first_dehumidification_control_type
            .map(dehumidification_control_name),
        "predecessor_dehumidification_control_type_humidistat":
            snapshot.predecessor_dehumidification_control_type_humidistat,
        "predecessor_dehumidification_control_type_second_read":
            snapshot.predecessor_dehumidification_control_type_second_read,
        "predecessor_second_dehumidification_control_type": snapshot
            .predecessor_second_dehumidification_control_type
            .map(dehumidification_control_name),
        "predecessor_dehumidification_control_type_none":
            snapshot.predecessor_dehumidification_control_type_none,
        "predecessor_dehumidification_control_body_entered":
            snapshot.predecessor_dehumidification_control_body_entered,
        "predecessor_dehumidification_control_guard_false_fallthrough":
            snapshot.predecessor_dehumidification_control_guard_false_fallthrough,
        "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed":
            snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed,
        "predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed":
            snapshot.predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed,
        "predecessor_resulting_supply_humidity_ratio_for_humidification":
            json_number(snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification),
        "predecessor_resulting_supply_humidity_ratio_for_humidification_ieee_bits":
            ieee_bits(snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification),
        "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed":
            snapshot.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed,
        "dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed":
            snapshot.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed,
        "supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read":
            snapshot.supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read,
        "supply_humidity_ratio_for_humidification_before_maximum_limit":
            json_number(snapshot.supply_humidity_ratio_for_humidification_before_maximum_limit),
        "supply_humidity_ratio_for_humidification_before_maximum_limit_ieee_bits":
            ieee_bits(snapshot.supply_humidity_ratio_for_humidification_before_maximum_limit),
        "maximum_heating_supply_air_humidity_ratio_for_minimum_read":
            snapshot.maximum_heating_supply_air_humidity_ratio_for_minimum_read,
        "maximum_heating_supply_air_humidity_ratio":
            json_number(snapshot.maximum_heating_supply_air_humidity_ratio),
        "maximum_heating_supply_air_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.maximum_heating_supply_air_humidity_ratio),
        "source_shaped_two_argument_minimum_evaluated":
            snapshot.source_shaped_two_argument_minimum_evaluated,
        "minimum_supply_humidity_ratio_for_humidification":
            json_number(snapshot.minimum_supply_humidity_ratio_for_humidification),
        "minimum_supply_humidity_ratio_for_humidification_ieee_bits":
            ieee_bits(snapshot.minimum_supply_humidity_ratio_for_humidification),
        "supply_humidity_ratio_for_humidification_assignment_performed":
            snapshot.supply_humidity_ratio_for_humidification_assignment_performed,
        "assigned_supply_humidity_ratio_for_humidification":
            json_number(snapshot.assigned_supply_humidity_ratio_for_humidification),
        "assigned_supply_humidity_ratio_for_humidification_ieee_bits":
            ieee_bits(snapshot.assigned_supply_humidity_ratio_for_humidification),
        "resulting_supply_humidity_ratio_for_humidification":
            json_number(snapshot.resulting_supply_humidity_ratio_for_humidification),
        "resulting_supply_humidity_ratio_for_humidification_ieee_bits":
            ieee_bits(snapshot.resulting_supply_humidity_ratio_for_humidification),
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

fn json_number(value: Option<f64>) -> Value {
    value
        .filter(|value| value.is_finite())
        .map_or(Value::Null, |value| json!(value))
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}

#[cfg(test)]
mod tests;
