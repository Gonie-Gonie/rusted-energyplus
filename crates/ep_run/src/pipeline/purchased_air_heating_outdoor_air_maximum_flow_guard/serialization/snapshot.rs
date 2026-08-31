//! Lossless JSON serialization for one CP435 heating outdoor-air guard snapshot.

use ep_model::IdealLoadsLimit;
use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot,
    heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_heating_operating_mode_deadband_assignment::serialization::snapshot::snapshot_json as cp434_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot,
) -> Value {
    let predecessor = heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot(snapshot);
    let mut value = cp434_snapshot_json(predecessor);
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
            "predecessor_cp434_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp434_resulting_supply_humidity_ratio),
            "predecessor_cp434_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp434_resulting_supply_humidity_ratio),
            "predecessor_cp434_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp434_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp434_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp434_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp434_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp434_resulting_supply_temperature_c),
            "predecessor_cp434_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp434_resulting_supply_temperature_c),
            "heating_outdoor_air_maximum_flow_guard_evaluated": snapshot.heating_outdoor_air_maximum_flow_guard_evaluated,
            "heating_limit_flow_rate_comparison_evaluated": snapshot.heating_limit_flow_rate_comparison_evaluated,
            "heating_limit_flow_rate_value": snapshot.heating_limit_flow_rate_value.map(limit_name),
            "heating_limit_flow_rate_comparison_satisfied": snapshot.heating_limit_flow_rate_comparison_satisfied,
            "heating_limit_flow_rate_and_capacity_comparison_evaluated": snapshot.heating_limit_flow_rate_and_capacity_comparison_evaluated,
            "heating_limit_flow_rate_and_capacity_value": snapshot.heating_limit_flow_rate_and_capacity_value.map(limit_name),
            "heating_limit_flow_rate_and_capacity_comparison_satisfied": snapshot.heating_limit_flow_rate_and_capacity_comparison_satisfied,
            "heating_flow_limit_active": snapshot.heating_flow_limit_active,
            "heating_flow_limit_selector_rejected": snapshot.heating_flow_limit_selector_rejected,
            "cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated": snapshot.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated,
            "outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit": snapshot.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit,
            "outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s": json_number(snapshot.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s),
            "outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s_ieee_bits": ieee_bits(snapshot.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s),
            "maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit": snapshot.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit,
            "maximum_heating_air_mass_flow_rate_for_guard_kg_per_s": json_number(snapshot.maximum_heating_air_mass_flow_rate_for_guard_kg_per_s),
            "maximum_heating_air_mass_flow_rate_for_guard_kg_per_s_ieee_bits": ieee_bits(snapshot.maximum_heating_air_mass_flow_rate_for_guard_kg_per_s),
            "outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated": snapshot.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated,
            "outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate": snapshot.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate,
            "maximum_heating_flow_body_entered": snapshot.maximum_heating_flow_body_entered,
            "heating_outdoor_air_maximum_flow_guard_false_fallthrough": snapshot.heating_outdoor_air_maximum_flow_guard_false_fallthrough,
            "cp434_retained_supply_humidity_ratio_state_owned": snapshot.cp434_retained_supply_humidity_ratio_state_owned,
            "cp434_retained_supply_enthalpy_state_owned": snapshot.cp434_retained_supply_enthalpy_state_owned,
            "cp434_retained_supply_temperature_state_owned": snapshot.cp434_retained_supply_temperature_state_owned,
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

fn limit_name(limit: IdealLoadsLimit) -> &'static str {
    match limit {
        IdealLoadsLimit::NoLimit => "NoLimit",
        IdealLoadsLimit::LimitFlowRate => "LimitFlowRate",
        IdealLoadsLimit::LimitCapacity => "LimitCapacity",
        IdealLoadsLimit::LimitFlowRateAndCapacity => "LimitFlowRateAndCapacity",
    }
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
    fn serializer_source_preserves_cp434_prefix_and_extends_exact_35_key_tail() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp434_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.remove(key)").count(), 1);
        let keys = literal_keys(source);
        assert_eq!(keys.len(), 35);
        assert_eq!(
            keys.iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            8
        );
        assert_eq!(keys.iter().copied().collect::<BTreeSet<_>>().len(), 35);
        assert_eq!(keys[0], "predecessor_cp434_resulting_supply_humidity_ratio");
        assert_eq!(keys[6], "heating_outdoor_air_maximum_flow_guard_evaluated");
        assert_eq!(keys[28], "cp434_retained_supply_temperature_state_owned");
        assert_eq!(keys[34], "resulting_supply_temperature_c_ieee_bits");
        assert!(source.contains("limit_name"));
    }
}
