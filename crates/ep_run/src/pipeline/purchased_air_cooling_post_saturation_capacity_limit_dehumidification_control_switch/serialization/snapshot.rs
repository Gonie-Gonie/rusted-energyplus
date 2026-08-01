//! JSON serialization for one CP386 switch-dispatch snapshot.

use ep_model::DehumidificationControlType;
use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_lexical_source": snapshot.first_excluded_lexical_source,
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
        "predecessor_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_resulting_supply_enthalpy_j_per_kg),
        "predecessor_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_resulting_supply_enthalpy_j_per_kg),
        "dehumidification_control_type_read": snapshot.dehumidification_control_type_read,
        "dehumidification_control_type": snapshot.dehumidification_control_type.map(control_type_name),
        "dehumidification_control_switch_dispatched": snapshot.dehumidification_control_switch_dispatched,
        "resulting_supply_enthalpy_j_per_kg": json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
        "resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
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
    use super::*;

    #[test]
    fn selector_names_are_stable_and_nonfinite_values_retain_ieee_sidecars() {
        assert_eq!(control_type_name(DehumidificationControlType::None), "None");
        let value = Some(f64::from_bits(0x7ff8_0000_0000_0386));
        assert!(json_number(value).is_null());
        assert_eq!(ieee_bits(value), Some("0x7ff8000000000386".to_string()));
    }
}
