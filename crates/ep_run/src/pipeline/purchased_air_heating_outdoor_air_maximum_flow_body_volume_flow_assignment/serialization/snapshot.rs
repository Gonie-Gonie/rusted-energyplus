//! Lossless JSON serialization for one CP436 volume-flow-assignment snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_predecessor_cp435_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_heating_outdoor_air_maximum_flow_guard::serialization::snapshot::snapshot_json as cp435_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
) -> Value {
    let predecessor =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_predecessor_cp435_snapshot(
            snapshot,
        );
    let mut value = cp435_snapshot_json(predecessor);
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
            "predecessor_cp435_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp435_resulting_supply_humidity_ratio),
            "predecessor_cp435_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp435_resulting_supply_humidity_ratio),
            "predecessor_cp435_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp435_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp435_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp435_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp435_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp435_resulting_supply_temperature_c),
            "predecessor_cp435_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp435_resulting_supply_temperature_c),
            "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed": snapshot.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed,
            "cp435_retained_supply_humidity_ratio_state_owned": snapshot.cp435_retained_supply_humidity_ratio_state_owned,
            "cp435_retained_supply_enthalpy_state_owned": snapshot.cp435_retained_supply_enthalpy_state_owned,
            "cp435_retained_supply_temperature_state_owned": snapshot.cp435_retained_supply_temperature_state_owned,
            "cp435_retained_outdoor_air_mass_flow_rate_owned_read": snapshot.cp435_retained_outdoor_air_mass_flow_rate_owned_read,
            "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read": snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read,
            "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s": json_number(snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s),
            "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s_ieee_bits": ieee_bits(snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s),
            "begin_environment_standard_air_density_owned_read": snapshot.begin_environment_standard_air_density_owned_read,
            "standard_air_density_for_outdoor_air_volume_flow_division_read": snapshot.standard_air_density_for_outdoor_air_volume_flow_division_read,
            "standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3": json_number(snapshot.standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3),
            "standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3_ieee_bits": ieee_bits(snapshot.standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3),
            "outdoor_air_mass_flow_rate_standard_air_density_division_evaluated": snapshot.outdoor_air_mass_flow_rate_standard_air_density_division_evaluated,
            "calculated_outdoor_air_volume_flow_rate_m3_per_s": json_number(snapshot.calculated_outdoor_air_volume_flow_rate_m3_per_s),
            "calculated_outdoor_air_volume_flow_rate_m3_per_s_ieee_bits": ieee_bits(snapshot.calculated_outdoor_air_volume_flow_rate_m3_per_s),
            "local_outdoor_air_volume_flow_rate_assignment_performed": snapshot.local_outdoor_air_volume_flow_rate_assignment_performed,
            "assigned_outdoor_air_volume_flow_rate_m3_per_s": json_number(snapshot.assigned_outdoor_air_volume_flow_rate_m3_per_s),
            "assigned_outdoor_air_volume_flow_rate_m3_per_s_ieee_bits": ieee_bits(snapshot.assigned_outdoor_air_volume_flow_rate_m3_per_s),
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
    fn serializer_source_preserves_cp435_prefix_and_extends_exact_30_key_tail() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp435_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.remove(key)").count(), 1);
        let keys = literal_keys(source);
        assert_eq!(keys.len(), 30);
        assert_eq!(
            keys.iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            10
        );
        assert_eq!(keys.iter().copied().collect::<BTreeSet<_>>().len(), 30);
        assert_eq!(keys[0], "predecessor_cp435_resulting_supply_humidity_ratio");
        assert_eq!(
            keys[6],
            "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed"
        );
        assert_eq!(
            keys[23],
            "assigned_outdoor_air_volume_flow_rate_m3_per_s_ieee_bits"
        );
        assert_eq!(keys[29], "resulting_supply_temperature_c_ieee_bits");
    }
}
