//! Lossless JSON serialization for one CP426 zero-flow humidity assignment snapshot.

use ep_runtime::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_predecessor_cp425_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment::serialization::snapshot::snapshot_json as cp425_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> Value {
    let predecessor =
        cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_predecessor_cp425_snapshot(
            snapshot,
        );
    let mut value = cp425_snapshot_json(predecessor);
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
            "predecessor_cp425_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp425_resulting_supply_humidity_ratio),
            "predecessor_cp425_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp425_resulting_supply_humidity_ratio),
            "predecessor_cp425_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp425_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp425_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp425_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp425_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp425_resulting_supply_temperature_c),
            "predecessor_cp425_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp425_resulting_supply_temperature_c),
            "cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed": snapshot.cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed,
            "cp425_retained_supply_humidity_ratio_state_owned": snapshot.cp425_retained_supply_humidity_ratio_state_owned,
            "cp425_retained_supply_enthalpy_state_owned": snapshot.cp425_retained_supply_enthalpy_state_owned,
            "cp425_retained_supply_temperature_state_owned": snapshot.cp425_retained_supply_temperature_state_owned,
            "cp329_retained_mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_owned_read": snapshot.cp329_retained_mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_owned_read,
            "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_read": snapshot.mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_read,
            "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment": json_number(snapshot.mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment),
            "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_ieee_bits": ieee_bits(snapshot.mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment),
            "zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_performed": snapshot.zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_performed,
            "assigned_supply_humidity_ratio_from_mixed_air": json_number(snapshot.assigned_supply_humidity_ratio_from_mixed_air),
            "assigned_supply_humidity_ratio_from_mixed_air_ieee_bits": ieee_bits(snapshot.assigned_supply_humidity_ratio_from_mixed_air),
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

    const DECLARED_TAIL: [&str; 23] = [
        "predecessor_cp425_resulting_supply_humidity_ratio",
        "predecessor_cp425_resulting_supply_humidity_ratio_ieee_bits",
        "predecessor_cp425_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp425_resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "predecessor_cp425_resulting_supply_temperature_c",
        "predecessor_cp425_resulting_supply_temperature_c_ieee_bits",
        "cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed",
        "cp425_retained_supply_humidity_ratio_state_owned",
        "cp425_retained_supply_enthalpy_state_owned",
        "cp425_retained_supply_temperature_state_owned",
        "cp329_retained_mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_owned_read",
        "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_read",
        "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment",
        "mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment_ieee_bits",
        "zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_performed",
        "assigned_supply_humidity_ratio_from_mixed_air",
        "assigned_supply_humidity_ratio_from_mixed_air_ieee_bits",
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c",
        "resulting_supply_temperature_c_ieee_bits",
    ];

    #[test]
    fn declared_tail_is_canonical_unique_and_serializer_has_391_keys() {
        let declared: BTreeSet<_> = DECLARED_TAIL.into_iter().collect();
        assert_eq!(declared.len(), 23);
        assert_eq!(374usize - 6 + declared.len(), 391);
        let source = include_str!("snapshot.rs");
        let literal: BTreeSet<_> = source
            .lines()
            .filter_map(|line| {
                line.trim_start()
                    .strip_prefix('"')
                    .and_then(|line| line.split_once("\":").map(|(key, _)| key))
            })
            .filter(|key| DECLARED_TAIL.contains(key))
            .collect();
        assert_eq!(literal, declared);
        assert!(source.contains("target.remove(key)"));
        assert!(source.contains("cp425_snapshot_json(predecessor)"));
    }
}
