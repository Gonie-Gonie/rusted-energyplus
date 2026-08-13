//! Lossless JSON serialization for one CP423 supply-temperature assignment snapshot.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_predecessor_cp422_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment::serialization::snapshot::snapshot_json as cp422_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> Value {
    let predecessor =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_predecessor_cp422_snapshot(snapshot);
    let mut value = cp422_snapshot_json(predecessor);
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
            "predecessor_cp422_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp422_resulting_supply_humidity_ratio),
            "predecessor_cp422_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp422_resulting_supply_humidity_ratio),
            "predecessor_cp422_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp422_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp422_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp422_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp422_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp422_resulting_supply_temperature_c),
            "predecessor_cp422_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp422_resulting_supply_temperature_c),
            "post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed": snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed,
            "cp422_retained_supply_humidity_ratio_state_owned": snapshot.cp422_retained_supply_humidity_ratio_state_owned,
            "cp422_retained_supply_enthalpy_state_owned": snapshot.cp422_retained_supply_enthalpy_state_owned,
            "cp422_retained_supply_temperature_state_owned": snapshot.cp422_retained_supply_temperature_state_owned,
            "cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read": snapshot.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read,
            "mixed_air_temperature_for_sensible_output_supply_temperature_read": snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_read,
            "mixed_air_temperature_for_sensible_output_supply_temperature_c": json_number(snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c),
            "mixed_air_temperature_for_sensible_output_supply_temperature_c_ieee_bits": ieee_bits(snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c),
            "cp422_retained_cooling_sensible_output_owned_read": snapshot.cp422_retained_cooling_sensible_output_owned_read,
            "cooling_sensible_output_for_supply_temperature_read": snapshot.cooling_sensible_output_for_supply_temperature_read,
            "cooling_sensible_output_for_supply_temperature_w": json_number(snapshot.cooling_sensible_output_for_supply_temperature_w),
            "cooling_sensible_output_for_supply_temperature_w_ieee_bits": ieee_bits(snapshot.cooling_sensible_output_for_supply_temperature_w),
            "cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read": snapshot.cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read,
            "cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated": snapshot.cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated,
            "supply_mass_flow_rate_for_sensible_output_supply_temperature_read": snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_read,
            "supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s": json_number(snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s),
            "supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s_ieee_bits": ieee_bits(snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s),
            "cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read": snapshot.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read,
            "cp_air_for_sensible_output_supply_temperature_read": snapshot.cp_air_for_sensible_output_supply_temperature_read,
            "cp_air_for_sensible_output_supply_temperature_j_per_kg_k": json_number(snapshot.cp_air_for_sensible_output_supply_temperature_j_per_kg_k),
            "cp_air_for_sensible_output_supply_temperature_j_per_kg_k_ieee_bits": ieee_bits(snapshot.cp_air_for_sensible_output_supply_temperature_j_per_kg_k),
            "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated": snapshot.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated,
            "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k": json_number(snapshot.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k),
            "supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k_ieee_bits": ieee_bits(snapshot.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k),
            "cooling_sensible_output_over_air_capacity_rate_calculated": snapshot.cooling_sensible_output_over_air_capacity_rate_calculated,
            "cooling_sensible_output_over_air_capacity_rate_k": json_number(snapshot.cooling_sensible_output_over_air_capacity_rate_k),
            "cooling_sensible_output_over_air_capacity_rate_k_ieee_bits": ieee_bits(snapshot.cooling_sensible_output_over_air_capacity_rate_k),
            "sensible_output_supply_temperature_calculated": snapshot.sensible_output_supply_temperature_calculated,
            "calculated_sensible_output_supply_temperature_c": json_number(snapshot.calculated_sensible_output_supply_temperature_c),
            "calculated_sensible_output_supply_temperature_c_ieee_bits": ieee_bits(snapshot.calculated_sensible_output_supply_temperature_c),
            "sensible_output_supply_temperature_assignment_performed": snapshot.sensible_output_supply_temperature_assignment_performed,
            "assigned_sensible_output_supply_temperature_c": json_number(snapshot.assigned_sensible_output_supply_temperature_c),
            "assigned_sensible_output_supply_temperature_c_ieee_bits": ieee_bits(snapshot.assigned_sensible_output_supply_temperature_c),
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
    fn serializer_source_preserves_cp422_prefix_and_extends_exact_tail() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp422_snapshot_json(predecessor)"));
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
    fn static_schema_is_first_311_exact_then_45_unique_entries_with_94_sidecars() {
        let cp420 = include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment/serialization/snapshot.rs"
        );
        let cp421 = include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard/serialization/snapshot.rs"
        );
        let cp422 = include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment/serialization/snapshot.rs"
        );
        let cp423 = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        let cp420_keys = literal_keys(cp420);
        let cp421_tail = literal_keys(cp421);
        let cp422_tail = literal_keys(cp422);
        let tail_keys = literal_keys(cp423);
        assert_eq!(cp420_keys.len(), 273);
        assert_eq!(cp421_tail.len(), 29);
        assert_eq!(cp422_tail.len(), 27);
        let mut cp421_keys = cp420_keys[..267].to_vec();
        cp421_keys.extend(cp421_tail);
        let mut cp422_keys = cp421_keys[..290].to_vec();
        cp422_keys.extend(cp422_tail);
        assert_eq!(cp422_keys.len(), 317);
        assert_eq!(tail_keys.len(), 45);
        let mut unique_tail = tail_keys.clone();
        unique_tail.sort_unstable();
        unique_tail.dedup();
        assert_eq!(unique_tail.len(), 45);
        assert!(
            cp422_keys[..311]
                .iter()
                .all(|key| !unique_tail.contains(key))
        );
        let mut final_keys = cp422_keys[..311].to_vec();
        final_keys.extend(tail_keys.iter().copied());
        let mut unique_final = final_keys.clone();
        unique_final.sort_unstable();
        unique_final.dedup();
        assert_eq!(final_keys.len(), 356);
        assert_eq!(unique_final.len(), 356);
        assert_eq!(
            cp422_keys[..311]
                .iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
                + tail_keys
                    .iter()
                    .filter(|key| key.ends_with("_ieee_bits"))
                    .count(),
            94
        );
    }
}
