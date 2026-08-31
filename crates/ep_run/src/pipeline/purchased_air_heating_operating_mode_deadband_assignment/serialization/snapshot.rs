//! Lossless JSON serialization for one CP434 Deadband-assignment snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
    heating_operating_mode_deadband_assignment_predecessor_cp433_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_cooling_entry_gate::operating_mode_name;
use crate::pipeline::purchased_air_heating_mode_guard_else_branch_entry::serialization::snapshot::snapshot_json as cp433_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
) -> Value {
    let predecessor =
        heating_operating_mode_deadband_assignment_predecessor_cp433_snapshot(snapshot);
    let mut value = cp433_snapshot_json(predecessor);
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
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c",
        "resulting_supply_temperature_c_ieee_bits",
        "heating_mode_guard_else_branch_entered",
    ] {
        target.remove(key);
    }
    extend_object(
        target,
        json!({
            "predecessor_cp433_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp433_resulting_supply_humidity_ratio),
            "predecessor_cp433_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp433_resulting_supply_humidity_ratio),
            "predecessor_cp433_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp433_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp433_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp433_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp433_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp433_resulting_supply_temperature_c),
            "predecessor_cp433_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp433_resulting_supply_temperature_c),
            "heating_mode_guard_else_branch_entered": snapshot.heating_mode_guard_else_branch_entered,
            "heating_operating_mode_deadband_assignment_executed": snapshot.heating_operating_mode_deadband_assignment_executed,
            "cp433_retained_supply_humidity_ratio_state_owned": snapshot.cp433_retained_supply_humidity_ratio_state_owned,
            "cp433_retained_supply_enthalpy_state_owned": snapshot.cp433_retained_supply_enthalpy_state_owned,
            "cp433_retained_supply_temperature_state_owned": snapshot.cp433_retained_supply_temperature_state_owned,
            "heating_operating_mode_deadband_assignment_performed": snapshot.heating_operating_mode_deadband_assignment_performed,
            "assigned_heating_operating_mode_deadband": snapshot.assigned_heating_operating_mode_deadband.map(operating_mode_name),
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
    use std::collections::BTreeSet;

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
    fn serializer_source_preserves_cp433_prefix_and_extends_exact_19_key_tail() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp433_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.remove(key)").count(), 1);
        let keys = literal_keys(source);
        assert_eq!(keys.len(), 19);
        assert_eq!(
            keys.iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            6
        );
        assert_eq!(keys.iter().copied().collect::<BTreeSet<_>>().len(), 19);
        assert_eq!(keys[0], "predecessor_cp433_resulting_supply_humidity_ratio");
        assert_eq!(keys[6], "heating_mode_guard_else_branch_entered");
        assert_eq!(keys[12], "assigned_heating_operating_mode_deadband");
        assert_eq!(keys[18], "resulting_supply_temperature_c_ieee_bits");
        assert!(source.contains("operating_mode_name"));
    }
}
