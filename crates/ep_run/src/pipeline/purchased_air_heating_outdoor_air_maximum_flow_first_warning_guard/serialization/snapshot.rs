//! Lossless JSON serialization for one CP437 first-warning-guard snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_body_volume_flow_assignment::serialization::snapshot::snapshot_json as cp436_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
) -> Value {
    let predecessor =
        heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot(snapshot);
    let mut value = cp436_snapshot_json(predecessor);
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
    ] {
        target.remove(key);
    }
    extend_object(
        target,
        json!({
            "predecessor_cp436_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp436_resulting_supply_humidity_ratio),
            "predecessor_cp436_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp436_resulting_supply_humidity_ratio),
            "predecessor_cp436_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp436_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp436_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp436_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp436_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp436_resulting_supply_temperature_c),
            "predecessor_cp436_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp436_resulting_supply_temperature_c),
            "heating_outdoor_air_maximum_flow_first_warning_guard_evaluated": snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated,
            "cp436_retained_supply_humidity_ratio_state_owned": snapshot.cp436_retained_supply_humidity_ratio_state_owned,
            "cp436_retained_supply_enthalpy_state_owned": snapshot.cp436_retained_supply_enthalpy_state_owned,
            "cp436_retained_supply_temperature_state_owned": snapshot.cp436_retained_supply_temperature_state_owned,
            "outdoor_air_flow_maximum_heating_output_error_count_state_owned": snapshot.outdoor_air_flow_maximum_heating_output_error_count_state_owned,
            "outdoor_air_flow_maximum_heating_output_error_count_read": snapshot.outdoor_air_flow_maximum_heating_output_error_count_read,
            "outdoor_air_flow_maximum_heating_output_error_count_before": snapshot.outdoor_air_flow_maximum_heating_output_error_count_before,
            "outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated": snapshot.outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated,
            "outdoor_air_flow_maximum_heating_output_error_count_less_than_one": snapshot.outdoor_air_flow_maximum_heating_output_error_count_less_than_one,
            "heating_outdoor_air_maximum_flow_first_warning_branch_entered": snapshot.heating_outdoor_air_maximum_flow_first_warning_branch_entered,
            "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough": snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough,
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
    fn serializer_source_preserves_cp436_prefix_and_extends_exact_23_key_tail() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp436_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.remove(key)").count(), 1);
        let keys = literal_keys(source);
        assert_eq!(keys.len(), 23);
        assert_eq!(
            keys.iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            6
        );
        assert_eq!(keys.iter().copied().collect::<BTreeSet<_>>().len(), 23);
        assert_eq!(keys[0], "predecessor_cp436_resulting_supply_humidity_ratio");
        assert_eq!(
            keys[6],
            "heating_outdoor_air_maximum_flow_first_warning_guard_evaluated"
        );
        assert_eq!(
            keys[16],
            "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough"
        );
        assert_eq!(keys[22], "resulting_supply_temperature_c_ieee_bits");
    }
}
