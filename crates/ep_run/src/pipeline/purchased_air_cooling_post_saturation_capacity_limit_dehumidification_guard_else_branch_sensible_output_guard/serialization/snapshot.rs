//! Lossless JSON serialization for one CP421 sensible-output maximum-capacity guard snapshot.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment::serialization::snapshot::snapshot_json as cp420_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot,
) -> Value {
    let predecessor =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot(snapshot);
    let mut value = cp420_snapshot_json(predecessor);
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
            "predecessor_cp420_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp420_resulting_supply_humidity_ratio),
            "predecessor_cp420_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp420_resulting_supply_humidity_ratio),
            "predecessor_cp420_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp420_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp420_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp420_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp420_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp420_resulting_supply_temperature_c),
            "predecessor_cp420_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp420_resulting_supply_temperature_c),
            "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated": snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated,
            "cp420_retained_cooling_sensible_output_owned_read": snapshot.cp420_retained_cooling_sensible_output_owned_read,
            "cooling_sensible_output_read": snapshot.cooling_sensible_output_read,
            "cp420_cooling_sensible_output_for_capacity_guard_w": json_number(snapshot.cp420_cooling_sensible_output_for_capacity_guard_w),
            "cp420_cooling_sensible_output_for_capacity_guard_w_ieee_bits": ieee_bits(snapshot.cp420_cooling_sensible_output_for_capacity_guard_w),
            "cp321_maximum_total_cooling_capacity_owned_read": snapshot.cp321_maximum_total_cooling_capacity_owned_read,
            "cp340_same_call_maximum_total_cooling_capacity_bit_corroborated": snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
            "maximum_total_cooling_capacity_read": snapshot.maximum_total_cooling_capacity_read,
            "maximum_total_cooling_capacity_w": json_number(snapshot.maximum_total_cooling_capacity_w),
            "maximum_total_cooling_capacity_w_ieee_bits": ieee_bits(snapshot.maximum_total_cooling_capacity_w),
            "cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated": snapshot.cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated,
            "cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity": snapshot.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
            "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered": snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered,
            "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough": snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough,
            "cp420_retained_supply_humidity_ratio_state_owned": snapshot.cp420_retained_supply_humidity_ratio_state_owned,
            "cp420_retained_supply_enthalpy_state_owned": snapshot.cp420_retained_supply_enthalpy_state_owned,
            "cp420_retained_supply_temperature_state_owned": snapshot.cp420_retained_supply_temperature_state_owned,
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
    fn serializer_source_preserves_cp420_prefix_and_extends_exact_tail() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp420_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.remove(key)").count(), 1);
        assert!(source.contains("cp420_cooling_sensible_output_for_capacity_guard_w_ieee_bits"));
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

    #[test]
    fn static_schema_is_first_267_exact_then_29_unique_entries_with_76_sidecars() {
        let cp420 = include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment/serialization/snapshot.rs"
        );
        let cp421 = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        let cp420_keys = literal_keys(cp420);
        let tail_keys = literal_keys(cp421);
        assert_eq!(cp420_keys.len(), 273);
        assert_eq!(
            cp420_keys
                .iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            71
        );
        assert_eq!(tail_keys.len(), 29);
        let mut unique_tail = tail_keys.clone();
        unique_tail.sort_unstable();
        unique_tail.dedup();
        assert_eq!(unique_tail.len(), 29);
        assert!(
            cp420_keys[..267]
                .iter()
                .all(|key| !unique_tail.contains(key))
        );
        assert_eq!(273 - 6, 267);
        assert_eq!(267 + tail_keys.len(), 296);
        assert_eq!(
            71 - 3
                + tail_keys
                    .iter()
                    .filter(|key| key.ends_with("_ieee_bits"))
                    .count(),
            76
        );
    }
}
