//! JSON serialization for one compact CP409 shared-case break snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot,
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
        "positive_guard_false_fallthrough_skipped": snapshot.positive_guard_false_fallthrough_skipped,
        "heating_availability_guard_false_fallthrough": snapshot.heating_availability_guard_false_fallthrough,
        "humidification_control_guard_false_fallthrough": snapshot.humidification_control_guard_false_fallthrough,
        "dehumidification_control_humidistat_maximum_assignment_executed": snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        "dehumidification_control_none_maximum_assignment_executed": snapshot.dehumidification_control_none_maximum_assignment_executed,
        "dehumidification_control_guard_false_fallthrough": snapshot.dehumidification_control_guard_false_fallthrough,
        "predecessor_capacity_limit_guard_evaluated": snapshot.predecessor_capacity_limit_guard_evaluated,
        "predecessor_capacity_limit_body_entered": snapshot.predecessor_capacity_limit_body_entered,
        "predecessor_active_capacity_limit_guard_false_fallthrough": snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        "predecessor_dehumidification_guard_evaluated": snapshot.predecessor_dehumidification_guard_evaluated,
        "predecessor_dehumidification_body_entered": snapshot.predecessor_dehumidification_body_entered,
        "predecessor_dehumidification_guard_false_fallthrough": snapshot.predecessor_dehumidification_guard_false_fallthrough,
        "predecessor_dehumidification_total_output_assignment_executed": snapshot.predecessor_dehumidification_total_output_assignment_executed,
        "predecessor_dehumidification_total_output_capacity_guard_evaluated": snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        "predecessor_dehumidification_total_output_capacity_adjustment_body_entered": snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough": snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        "dehumidification_total_output_capacity_guard_false_fallthrough": snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        "dehumidification_total_output_maximum_capacity_assignment_executed": snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        "predecessor_supply_enthalpy_assignment_executed": snapshot.predecessor_supply_enthalpy_assignment_executed,
        "predecessor_dehumidification_control_type_read": snapshot.predecessor_dehumidification_control_type_read,
        "predecessor_dehumidification_control_type": snapshot.predecessor_dehumidification_control_type.map(control_type_name),
        "predecessor_dehumidification_control_switch_dispatched": snapshot.predecessor_dehumidification_control_switch_dispatched,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered": snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break": snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        "predecessor_dehumidification_control_humidistat_case_entered": snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed": snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        "predecessor_dehumidification_control_humidistat_case_exited_via_break": snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered": snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough": snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed": snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        "predecessor_cp408_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp408_resulting_supply_humidity_ratio),
        "predecessor_cp408_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp408_resulting_supply_humidity_ratio),
        "predecessor_cp408_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp408_resulting_supply_enthalpy_j_per_kg),
        "predecessor_cp408_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp408_resulting_supply_enthalpy_j_per_kg),
        "predecessor_cp408_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp408_resulting_supply_temperature_c),
        "predecessor_cp408_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp408_resulting_supply_temperature_c),
        "dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break": snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        "resulting_supply_humidity_ratio": json_number(snapshot.resulting_supply_humidity_ratio),
        "resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.resulting_supply_humidity_ratio),
        "resulting_supply_enthalpy_j_per_kg": json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_temperature_c": json_number(snapshot.resulting_supply_temperature_c),
        "resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.resulting_supply_temperature_c),
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
mod tests;
#[cfg(test)]
pub(in crate::pipeline) use tests::snapshot as test_snapshot;
