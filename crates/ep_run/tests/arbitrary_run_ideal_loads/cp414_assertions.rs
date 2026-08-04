//! CP414 saturation supply-temperature assignment assertions.

use ep_runtime::psychrometrics::energyplus_psy_tsat_fn_h_pb_raw;
use serde_json::{Map, Value, json};

const CP413_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_lifecycle";
const CP414_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-cp413-retained-supply-enthalpy-for-saturation-temperature",
    "read-environment-outdoor-barometric-pressure-for-saturation-temperature",
    "evaluate-psy-tsat-fn-h-pb",
    "assign-purchased-air-supply-temperature-to-saturation-temperature",
];
const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ROUTE_FIELDS: [&str; 4] = [
    "predecessor_route_counts",
    "predecessor_guard_false_fallthrough_route_counts",
    "predecessor_guard_body_entry_route_counts",
    "supply_temperature_saturation_assignment_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP413_KEY];
    let lifecycle = &runtime[CP414_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2316",
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2319",
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"],
    );

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 36, "CP414 {field} width");
    }
    for (next, previous) in [
        ("predecessor_route_counts", "predecessor_route_counts"),
        (
            "predecessor_guard_false_fallthrough_route_counts",
            "guard_false_fallthrough_route_counts",
        ),
        (
            "predecessor_guard_body_entry_route_counts",
            "guard_body_entry_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP414 {next}");
    }

    assert_route_contract(lifecycle);
    assert_public_route_firewall(lifecycle);
    assert_conceptual_contract();

    let routes = array(lifecycle, "predecessor_route_counts");
    let guard_false = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let body = array(lifecycle, "predecessor_guard_body_entry_route_counts");
    let assignments = array(
        lifecycle,
        "supply_temperature_saturation_assignment_route_counts",
    );
    let transitions = sum(routes);
    let active_outcomes = sum(&routes[18..]);
    let false_fallthroughs = sum(guard_false);
    let body_entries = sum(body);
    let assignment_count = sum(assignments);
    let enthalpy_owners =
        sum_predecessor_indices(routes, |index| matches!(index, 5 | 8 | 11 | 14 | 17..=29));
    let temperature_owners = sum_predecessor_indices(routes, |index| index >= 3);

    assert_eq!(count(lifecycle, "transition_count"), transitions);
    assert_eq!(false_fallthroughs + body_entries, active_outcomes);
    assert_eq!(assignment_count, body_entries);
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + assignment_count,
        transitions,
    );
    assert_eq!(
        count(lifecycle, "saturation_supply_temperature_assignment_count"),
        assignment_count,
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignment_count * 4,
    );
    for field in [
        "cp413_supply_humidity_ratio_state_owner_count",
        "unchanged_supply_humidity_ratio_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), active_outcomes, "CP414 {field}");
    }
    for field in [
        "cp413_supply_enthalpy_state_owner_count",
        "unchanged_supply_enthalpy_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), enthalpy_owners, "CP414 {field}");
    }
    assert_eq!(
        count(lifecycle, "cp413_supply_temperature_state_owner_count"),
        temperature_owners,
    );
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count",) + assignment_count,
        temperature_owners,
    );
    for field in [
        "cp414_saturation_supply_temperature_state_owner_count",
        "cp413_retained_supply_enthalpy_owned_read_count",
        "supply_enthalpy_for_saturation_temperature_read_count",
        "environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count",
        "environment_outdoor_barometric_pressure_for_saturation_temperature_read_count",
        "psy_tsat_fn_h_pb_evaluation_count",
        "purchased_air_supply_temperature_saturation_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignment_count, "CP414 {field}");
    }

    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);

    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP414 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP414_KEY),
        "CP414 lifecycle must remain outside numerical result state",
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP414_KEY));
    assert!(
        runtime[CP414_KEY].is_null(),
        "non-direct runtime must not publish CP414 evidence",
    );
}

fn assert_route_contract(lifecycle: &Value) {
    let routes = array(lifecycle, "predecessor_route_counts");
    let guard_false = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let body = array(lifecycle, "predecessor_guard_body_entry_route_counts");
    let assignments = array(
        lifecycle,
        "supply_temperature_saturation_assignment_route_counts",
    );
    for index in 0..36 {
        let outcomes = count_value(&guard_false[index]).checked_add(count_value(&body[index]));
        assert_eq!(
            outcomes,
            Some(if index >= 18 {
                count_value(&routes[index])
            } else {
                0
            }),
            "CP414 route outcome partition {index}",
        );
        assert_eq!(
            count_value(&assignments[index]),
            count_value(&body[index]),
            "CP414 assignment/body route {index}",
        );
    }
}

fn assert_public_route_firewall(lifecycle: &Value) {
    for field in ROUTE_FIELDS {
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) {
                assert_eq!(count_value(value), 0, "public CP414 {field} route {index}");
            }
        }
    }
}

fn assert_conceptual_contract() {
    let conceptual_outcomes = 36 + 18;
    let public_outcomes = PUBLIC_LOGICAL_INDICES.len() + 4;
    let private_outcomes = conceptual_outcomes - public_outcomes;
    let assignment_outcomes = 18;
    let public_assignment_outcomes = 4;
    let source_sites = assignment_outcomes * ORDER.len();
    let humidity_ratio_presence = 36;
    let enthalpy_presence = 41;
    let temperature_presence = 51;
    let unchanged_temperature = temperature_presence - assignment_outcomes;
    assert_eq!(
        (
            conceptual_outcomes,
            public_outcomes,
            private_outcomes,
            assignment_outcomes,
            public_assignment_outcomes,
            source_sites,
        ),
        (54, 17, 37, 18, 4, 72),
    );
    assert_eq!(
        (
            humidity_ratio_presence,
            enthalpy_presence,
            temperature_presence,
            humidity_ratio_presence,
            enthalpy_presence,
            unchanged_temperature,
        ),
        (36, 41, 51, 36, 41, 33),
        "CP414 W/H/T conceptual ownership and preservation",
    );
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(
        latest["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2316",
    );
    assert_eq!(
        latest["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2319",
    );
    assert_eq!(latest["source_order"], json!(ORDER));
    assert_eq!(latest["parent_call_ordinal"], transitions);
    for (field, value) in predecessor
        .as_object()
        .expect("CP413 latest must be an object")
    {
        if !matches!(
            field.as_str(),
            "source"
                | "first_excluded_source"
                | "source_order"
                | "resulting_supply_humidity_ratio"
                | "resulting_supply_humidity_ratio_ieee_bits"
                | "resulting_supply_enthalpy_j_per_kg"
                | "resulting_supply_enthalpy_j_per_kg_ieee_bits"
                | "resulting_supply_temperature_c"
                | "resulting_supply_temperature_c_ieee_bits"
        ) {
            assert_eq!(latest.get(field), Some(value), "CP414 {field} lineage");
        }
    }
    for (next, previous) in [
        (
            "predecessor_cp413_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp413_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp413_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_same_bits(latest, predecessor, next, previous);
    }

    let active = boolean(
        predecessor,
        "saturation_supply_humidity_ratio_guard_body_entered",
    );
    for field in [
        "post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed",
        "cp413_retained_supply_enthalpy_owned_read",
        "supply_enthalpy_for_saturation_temperature_read",
        "environment_outdoor_barometric_pressure_for_saturation_temperature_owned_read",
        "environment_outdoor_barometric_pressure_for_saturation_temperature_read",
        "psy_tsat_fn_h_pb_evaluated",
        "purchased_air_supply_temperature_saturation_assignment_performed",
    ] {
        assert_eq!(latest[field], active, "CP414 {field}");
    }
    for (owner, predecessor_field) in [
        (
            "cp413_retained_supply_humidity_ratio_state_owned",
            "resulting_supply_humidity_ratio",
        ),
        (
            "cp413_retained_supply_enthalpy_state_owned",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "cp413_retained_supply_temperature_state_owned",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_eq!(
            latest[owner],
            ieee_bits(predecessor, predecessor_field).is_some(),
            "CP414 {owner}",
        );
    }

    assert_same_bits(
        latest,
        predecessor,
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    );
    assert_same_bits(
        latest,
        predecessor,
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    );
    if active {
        assert_same_bits(
            latest,
            predecessor,
            "supply_enthalpy_for_saturation_temperature_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        );
        let enthalpy = number(
            latest,
            "supply_enthalpy_for_saturation_temperature_j_per_kg",
        );
        let pressure = number(
            latest,
            "outdoor_barometric_pressure_for_saturation_temperature_pa",
        );
        assert!(enthalpy.is_finite());
        assert!(pressure.is_finite() && pressure > 0.0);
        let expected_temperature = energyplus_psy_tsat_fn_h_pb_raw(enthalpy, pressure);
        for field in [
            "psychrometric_saturation_supply_temperature_result_c",
            "assigned_saturation_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert_eq!(
                ieee_bits(latest, field),
                Some(expected_temperature.to_bits()),
                "CP414 {field}",
            );
        }
    } else {
        for field in [
            "supply_enthalpy_for_saturation_temperature_j_per_kg",
            "outdoor_barometric_pressure_for_saturation_temperature_pa",
            "psychrometric_saturation_supply_temperature_result_c",
            "assigned_saturation_supply_temperature_c",
        ] {
            assert!(latest[field].is_null(), "CP414 inactive {field}");
            assert!(
                latest[format!("{field}_ieee_bits")].is_null(),
                "CP414 inactive {field} bits",
            );
        }
        assert_same_bits(
            latest,
            predecessor,
            "resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        );
    }

    assert_eq!(latest.as_object().map(|object| object.len()), Some(144));
    assert_eq!(
        latest.as_object().map(|object| {
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
        }),
        Some(32),
    );
}

fn sum_predecessor_indices(values: &[Value], include: impl Fn(usize) -> bool) -> u64 {
    let mut logical = 0usize;
    let mut total = 0u64;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(SPLIT_PREDECESSOR_INDICES.contains(&predecessor_index));
        if include(predecessor_index) {
            total += sum(&values[logical..logical + width]);
        }
        logical += width;
    }
    total
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        ieee_bits(next, next_field),
        ieee_bits(previous, previous_field),
        "CP414 {next_field} lineage",
    );
}

fn ieee_bits(value: &Value, field: &str) -> Option<u64> {
    let sidecar = &value[format!("{field}_ieee_bits")];
    if sidecar.is_null() {
        return None;
    }
    let text = sidecar
        .as_str()
        .expect("CP414 IEEE sidecar must be a string");
    let hex = text
        .strip_prefix("0x")
        .expect("CP414 IEEE sidecar must use a hexadecimal prefix");
    Some(u64::from_str_radix(hex, 16).expect("CP414 IEEE sidecar must contain binary64 bits"))
}

fn number(value: &Value, field: &str) -> f64 {
    value[field]
        .as_f64()
        .expect("CP414 value must be a finite JSON number")
}

fn boolean(value: &Value, field: &str) -> bool {
    value[field]
        .as_bool()
        .expect("CP414 value must be a boolean")
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field])
}

fn count_value(value: &Value) -> u64 {
    value
        .as_u64()
        .expect("CP414 value must be an unsigned count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    match value[field].as_array() {
        Some(values) => values.as_slice(),
        None => &[],
    }
}

fn sum(values: &[Value]) -> u64 {
    values.iter().map(count_value).sum()
}
