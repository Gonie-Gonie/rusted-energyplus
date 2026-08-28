//! Lossless JSON serialization for one CP431 heating-mode-guard snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingModeGuardSnapshot, heating_mode_guard_predecessor_cp430_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_cooling_entry_gate::temperature_control_type_name;
use crate::pipeline::purchased_air_heating_or_no_load_case_entry::serialization::snapshot::snapshot_json as cp430_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingModeGuardSnapshot,
) -> Value {
    let predecessor = heating_mode_guard_predecessor_cp430_snapshot(snapshot);
    let mut value = cp430_snapshot_json(predecessor);
    let Value::Object(target) = &mut value else {
        return Value::Null;
    };

    target.insert("source".to_string(), json!(snapshot.source));
    target.insert(
        "first_excluded_source".to_string(),
        json!(snapshot.first_excluded_source),
    );
    target.insert("source_order".to_string(), json!(snapshot.source_order));
    for key in [
        "heating_or_no_load_case_entered",
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c",
        "resulting_supply_temperature_c_ieee_bits",
    ] {
        target.remove(key);
    }
    extend_object(
        target,
        json!({
            "predecessor_cp430_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp430_resulting_supply_humidity_ratio),
            "predecessor_cp430_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp430_resulting_supply_humidity_ratio),
            "predecessor_cp430_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp430_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp430_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp430_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp430_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp430_resulting_supply_temperature_c),
            "predecessor_cp430_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp430_resulting_supply_temperature_c),
            "heating_or_no_load_case_entered": snapshot.heating_or_no_load_case_entered,
            "heating_mode_guard_evaluated": snapshot.heating_mode_guard_evaluated,
            "cp311_retained_minimum_outdoor_air_sensible_output_owned_read": snapshot.cp311_retained_minimum_outdoor_air_sensible_output_owned_read,
            "cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated": snapshot.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroborated,
            "minimum_outdoor_air_sensible_output_for_heating_mode_guard_read": snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read,
            "minimum_outdoor_air_sensible_output_for_heating_mode_guard_w": json_number(snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w),
            "minimum_outdoor_air_sensible_output_for_heating_mode_guard_w_ieee_bits": ieee_bits(snapshot.minimum_outdoor_air_sensible_output_for_heating_mode_guard_w),
            "cp310_retained_heating_setpoint_demand_owned_read": snapshot.cp310_retained_heating_setpoint_demand_owned_read,
            "heating_setpoint_demand_for_heating_mode_guard_read": snapshot.heating_setpoint_demand_for_heating_mode_guard_read,
            "heating_setpoint_demand_for_heating_mode_guard_w": json_number(snapshot.heating_setpoint_demand_for_heating_mode_guard_w),
            "heating_setpoint_demand_for_heating_mode_guard_w_ieee_bits": ieee_bits(snapshot.heating_setpoint_demand_for_heating_mode_guard_w),
            "minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated": snapshot.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_evaluated,
            "minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand": snapshot.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand,
            "prevalidated_temperature_control_type_owned_read": snapshot.prevalidated_temperature_control_type_owned_read,
            "temperature_control_type_read_after_sensible_comparison_short_circuit": snapshot.temperature_control_type_read_after_sensible_comparison_short_circuit,
            "temperature_control_type": snapshot.temperature_control_type.map(temperature_control_type_name),
            "temperature_control_type_single_cool_comparison_evaluated": snapshot.temperature_control_type_single_cool_comparison_evaluated,
            "temperature_control_type_permits_heating": snapshot.temperature_control_type_permits_heating,
            "single_cool_blocked": snapshot.single_cool_blocked,
            "heating_operating_mode_body_entered": snapshot.heating_operating_mode_body_entered,
            "heating_mode_guard_false_fallthrough": snapshot.heating_mode_guard_false_fallthrough,
            "cp430_retained_supply_humidity_ratio_state_owned": snapshot.cp430_retained_supply_humidity_ratio_state_owned,
            "cp430_retained_supply_enthalpy_state_owned": snapshot.cp430_retained_supply_enthalpy_state_owned,
            "cp430_retained_supply_temperature_state_owned": snapshot.cp430_retained_supply_temperature_state_owned,
            "resulting_supply_humidity_ratio": json_number(snapshot.resulting_supply_humidity_ratio),
            "resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.resulting_supply_humidity_ratio),
            "resulting_supply_enthalpy_j_per_kg": json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
            "resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
            "resulting_supply_temperature_c": json_number(snapshot.resulting_supply_temperature_c),
            "resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.resulting_supply_temperature_c),
        }),
    );
    value
}

fn extend_object(target: &mut serde_json::Map<String, Value>, extension: Value) {
    if let Value::Object(extension) = extension {
        target.extend(extension);
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
    fn literal_keys(source: &str) -> Vec<&str> {
        source
            .lines()
            .filter_map(|line| {
                let line = line.trim_start();
                line.strip_prefix('"')
                    .and_then(|line| line.split_once("\":").map(|(key, _)| key))
            })
            .collect()
    }

    #[test]
    fn serializer_source_preserves_cp430_prefix_and_extends_exact_tail() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp430_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.remove(key)").count(), 1);
        let keys = literal_keys(source);
        assert_eq!(keys.len(), 36);
        assert_eq!(
            keys.iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            8
        );
        let mut unique = keys.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 36);
        for forbidden in [
            "DirectZonePurchasedAirCouplingInput",
            "numerical_dto",
            "prediction",
            "feedback",
            "nodes",
            "loads",
            "reports",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
