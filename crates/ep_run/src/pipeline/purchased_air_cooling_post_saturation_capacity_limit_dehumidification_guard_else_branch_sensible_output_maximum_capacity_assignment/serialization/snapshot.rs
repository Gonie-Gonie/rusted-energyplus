//! Lossless JSON serialization for one CP422 maximum-capacity assignment snapshot.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_predecessor_cp421_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard::serialization::snapshot::snapshot_json as cp421_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> Value {
    let predecessor =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_predecessor_cp421_snapshot(snapshot);
    let mut value = cp421_snapshot_json(predecessor);
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
            "predecessor_cp421_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp421_resulting_supply_humidity_ratio),
            "predecessor_cp421_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp421_resulting_supply_humidity_ratio),
            "predecessor_cp421_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp421_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp421_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp421_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp421_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp421_resulting_supply_temperature_c),
            "predecessor_cp421_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp421_resulting_supply_temperature_c),
            "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed": snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed,
            "cp421_retained_supply_humidity_ratio_state_owned": snapshot.cp421_retained_supply_humidity_ratio_state_owned,
            "cp421_retained_supply_enthalpy_state_owned": snapshot.cp421_retained_supply_enthalpy_state_owned,
            "cp421_retained_supply_temperature_state_owned": snapshot.cp421_retained_supply_temperature_state_owned,
            "preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w": json_number(snapshot.preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w),
            "preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w_ieee_bits": ieee_bits(snapshot.preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w),
            "cp421_retained_maximum_total_cooling_capacity_owned_read": snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read,
            "maximum_total_cooling_capacity_for_sensible_output_assignment_read": snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read,
            "maximum_total_cooling_capacity_for_sensible_output_assignment_w": json_number(snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_w),
            "maximum_total_cooling_capacity_for_sensible_output_assignment_w_ieee_bits": ieee_bits(snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_w),
            "cooling_sensible_output_maximum_capacity_assignment_performed": snapshot.cooling_sensible_output_maximum_capacity_assignment_performed,
            "assigned_cooling_sensible_output_from_maximum_capacity_w": json_number(snapshot.assigned_cooling_sensible_output_from_maximum_capacity_w),
            "assigned_cooling_sensible_output_from_maximum_capacity_w_ieee_bits": ieee_bits(snapshot.assigned_cooling_sensible_output_from_maximum_capacity_w),
            "resulting_cooling_sensible_output_after_maximum_capacity_assignment_w": json_number(snapshot.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w),
            "resulting_cooling_sensible_output_after_maximum_capacity_assignment_w_ieee_bits": ieee_bits(snapshot.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w),
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
    fn serializer_source_preserves_cp421_prefix_and_extends_exact_tail() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp421_snapshot_json(predecessor)"));
        assert_eq!(source.matches("target.remove(key)").count(), 1);
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
    fn static_schema_is_first_290_exact_then_27_unique_entries_with_83_sidecars() {
        let cp420 = include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment/serialization/snapshot.rs"
        );
        let cp421 = include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard/serialization/snapshot.rs"
        );
        let cp422 = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        let cp420_keys = literal_keys(cp420);
        let cp421_tail = literal_keys(cp421);
        let tail_keys = literal_keys(cp422);
        assert_eq!(cp420_keys.len(), 273);
        assert_eq!(cp421_tail.len(), 29);
        let mut cp421_keys = cp420_keys[..267].to_vec();
        cp421_keys.extend(cp421_tail);
        assert_eq!(cp421_keys.len(), 296);
        let mut unique_cp421 = cp421_keys.clone();
        unique_cp421.sort_unstable();
        unique_cp421.dedup();
        assert_eq!(unique_cp421.len(), 296);
        assert_eq!(tail_keys.len(), 27);
        let mut unique_tail = tail_keys.clone();
        unique_tail.sort_unstable();
        unique_tail.dedup();
        assert_eq!(unique_tail.len(), 27);
        assert!(
            cp421_keys[..290]
                .iter()
                .all(|key| !unique_tail.contains(key))
        );
        let mut final_keys = cp421_keys[..290].to_vec();
        final_keys.extend(tail_keys.iter().copied());
        let mut unique_final = final_keys.clone();
        unique_final.sort_unstable();
        unique_final.dedup();
        assert_eq!(final_keys.len(), 317);
        assert_eq!(unique_final.len(), 317);
        assert_eq!(
            cp421_keys[..290]
                .iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
                + tail_keys
                    .iter()
                    .filter(|key| key.ends_with("_ieee_bits"))
                    .count(),
            83
        );
    }
}
