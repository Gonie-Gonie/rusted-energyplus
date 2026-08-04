//! CP412 saturation humidity-ratio assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp413_assertions.rs"]
mod cp413_assertions;

const CP411_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle";
const CP412_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-saturation-humidity-ratio",
    "read-environment-outdoor-barometric-pressure-for-saturation-humidity-ratio",
    "evaluate-psy-w-fn-tdb-rh-pb-at-unity-relative-humidity",
    "assign-local-saturation-supply-humidity-ratio",
];
const ROUTE_FIELDS: [&str; 5] = [
    "predecessor_route_counts",
    "predecessor_guard_false_fallthrough_route_counts",
    "predecessor_maximum_capacity_assignment_route_counts",
    "predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts",
    "supply_humidity_ratio_saturation_assignment_route_counts",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP411_KEY];
    let lifecycle = &runtime[CP412_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2314"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2315"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );

    for field in ROUTE_FIELDS {
        assert_eq!(array(lifecycle, field).len(), 30, "CP412 {field} width");
    }
    for (next, previous) in [
        ("predecessor_route_counts", "predecessor_route_counts"),
        (
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
        ),
        (
            "predecessor_maximum_capacity_assignment_route_counts",
            "predecessor_maximum_capacity_assignment_route_counts",
        ),
        (
            "predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts",
            "supply_humidity_ratio_pre_saturation_original_assignment_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP412 {next}");
    }
    assert_route_contract(lifecycle);
    assert_public_route_firewall(lifecycle);

    let routes = array(lifecycle, "predecessor_route_counts");
    let transitions = count(lifecycle, "transition_count");
    let assignments = count(
        lifecycle,
        "supply_humidity_ratio_saturation_assignment_count",
    );
    let expected_assignments = sum(&routes[18..]);
    let enthalpy_owners = sum_indices(routes, &[5, 8, 11, 14, 17]) + expected_assignments;
    let temperature_owners = sum(&routes[3..]);
    assert_eq!(assignments, expected_assignments);
    assert_eq!(
        count(
            lifecycle,
            "predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count",
        ),
        assignments
    );
    assert_eq!(
        count(
            predecessor,
            "supply_humidity_ratio_pre_saturation_original_assignment_count",
        ),
        assignments
    );
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + assignments,
        transitions
    );
    assert_eq!(
        count(lifecycle, "predecessor_guard_false_fallthrough_count"),
        count(predecessor, "predecessor_guard_false_fallthrough_count")
    );
    assert_eq!(
        count(lifecycle, "predecessor_maximum_capacity_assignment_count"),
        count(predecessor, "predecessor_maximum_capacity_assignment_count")
    );
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        4 * assignments
    );
    for field in [
        "cp411_supply_humidity_ratio_state_owner_count",
        "unchanged_supply_humidity_ratio_preservation_count",
    ] {
        assert_eq!(
            count(lifecycle, field),
            expected_assignments,
            "CP412 {field}"
        );
    }
    for field in [
        "cp411_supply_enthalpy_state_owner_count",
        "unchanged_supply_enthalpy_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), enthalpy_owners, "CP412 {field}");
    }
    for field in [
        "cp411_supply_temperature_state_owner_count",
        "unchanged_supply_temperature_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), temperature_owners, "CP412 {field}");
    }
    for field in [
        "cp411_retained_supply_temperature_owned_read_count",
        "purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count",
        "environment_outdoor_barometric_pressure_owner_count",
        "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count",
        "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count",
        "local_saturation_supply_humidity_ratio_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP412 {field}");
    }

    assert_latest_lineage(&lifecycle["latest"], &predecessor["latest"], transitions);

    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP412 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP412_KEY),
        "CP412 lifecycle must remain outside numerical result state"
    );
    cp413_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP412_KEY));
    assert!(
        runtime[CP412_KEY].is_null(),
        "non-direct runtime must not publish CP412 evidence"
    );
    cp413_assertions::assert_non_direct(runtime);
}

fn assert_route_contract(lifecycle: &Value) {
    let split_indices = [20, 21, 24, 25, 27, 29];
    let public_indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 24];
    let total = 30 + split_indices.len();
    let public_active = public_indices
        .into_iter()
        .filter(|index| *index >= 18)
        .map(|index| 1 + usize::from(split_indices.contains(&index)))
        .sum::<usize>();
    assert_eq!((total, total - 18, 18, public_active), (36, 18, 18, 4));

    let routes = array(lifecycle, "predecessor_route_counts");
    let guard_false = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let maximum = array(
        lifecycle,
        "predecessor_maximum_capacity_assignment_route_counts",
    );
    let predecessor_assignment = array(
        lifecycle,
        "predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts",
    );
    let assignment = array(
        lifecycle,
        "supply_humidity_ratio_saturation_assignment_route_counts",
    );
    for index in 0..routes.len() {
        let route_count = count_value(&routes[index], "route count");
        let branch_count = count_value(&guard_false[index], "guard-false count")
            .checked_add(count_value(&maximum[index], "maximum count"));
        let expected_branch = if split_indices.contains(&index) {
            Some(route_count)
        } else {
            Some(0)
        };
        assert_eq!(branch_count, expected_branch, "CP412 split route {index}");
        let expected_assignment = if index >= 18 { route_count } else { 0 };
        assert_eq!(
            count_value(
                &predecessor_assignment[index],
                "predecessor assignment count"
            ),
            expected_assignment,
            "CP412 predecessor assignment route {index}"
        );
        assert_eq!(
            count_value(&assignment[index], "assignment count"),
            expected_assignment,
            "CP412 assignment route {index}"
        );
    }
}

fn assert_public_route_firewall(lifecycle: &Value) {
    for field in ROUTE_FIELDS {
        for (index, value) in array(lifecycle, field).iter().enumerate() {
            if !matches!(index, 0..=8 | 20 | 24) {
                assert_eq!(
                    count_value(value, "private route count"),
                    0,
                    "public CP412 {field} route {index}"
                );
            }
        }
    }
}

fn assert_latest_lineage(latest: &Value, predecessor: &Value, transitions: u64) {
    assert_eq!(
        latest["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2314"
    );
    assert_eq!(
        latest["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2315"
    );
    assert_eq!(latest["source_order"], json!(ORDER));
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], predecessor["system"]);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    for field in inherited_scalar_fields() {
        assert_eq!(latest[field], predecessor[field], "CP412 {field} lineage");
    }
    for field in inherited_numeric_fields() {
        assert_same_bits(latest, predecessor, field, field);
    }
    for (next, previous) in [
        (
            "predecessor_cp411_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp411_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp411_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_same_bits(latest, predecessor, next, previous);
    }

    let active = boolean(
        predecessor,
        "post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed",
    );
    assert_eq!(
        latest["post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed"],
        active
    );
    for (owner, predecessor_field) in [
        (
            "cp411_retained_supply_humidity_ratio_state_owned",
            "resulting_supply_humidity_ratio",
        ),
        (
            "cp411_retained_supply_enthalpy_state_owned",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "cp411_retained_supply_temperature_state_owned",
            "resulting_supply_temperature_c",
        ),
    ] {
        assert_eq!(
            latest[owner],
            ieee_bits(predecessor, predecessor_field).is_some(),
            "CP412 {owner}"
        );
    }
    for field in [
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ] {
        assert_same_bits(latest, predecessor, field, field);
    }

    let action_fields = [
        "cp411_retained_supply_temperature_owned_read",
        "purchased_air_supply_temperature_for_saturation_humidity_ratio_read",
        "environment_outdoor_barometric_pressure_owned_read",
        "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read",
        "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated",
        "local_saturation_supply_humidity_ratio_assignment_performed",
    ];
    for field in action_fields {
        assert_eq!(latest[field], active, "CP412 {field}");
    }
    if active {
        assert_same_bits(
            latest,
            predecessor,
            "supply_temperature_for_saturation_humidity_ratio_c",
            "resulting_supply_temperature_c",
        );
        let temperature = number(latest, "supply_temperature_for_saturation_humidity_ratio_c");
        let pressure = number(latest, "outdoor_barometric_pressure_pa");
        assert!(temperature.is_finite());
        assert!(pressure.is_finite() && pressure > 0.0);
        let saturation =
            ep_runtime::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure);
        assert!(saturation.is_finite());
        for field in [
            "saturation_supply_humidity_ratio",
            "assigned_saturation_supply_humidity_ratio",
            "resulting_saturation_supply_humidity_ratio",
        ] {
            assert_eq!(
                ieee_bits(latest, field),
                Some(saturation.to_bits()),
                "CP412 {field}"
            );
        }
    } else {
        for field in [
            "supply_temperature_for_saturation_humidity_ratio_c",
            "outdoor_barometric_pressure_pa",
            "saturation_supply_humidity_ratio",
            "assigned_saturation_supply_humidity_ratio",
            "resulting_saturation_supply_humidity_ratio",
        ] {
            assert!(latest[field].is_null(), "CP412 inactive {field}");
            assert!(
                latest[format!("{field}_ieee_bits")].is_null(),
                "CP412 inactive {field} bits"
            );
        }
    }

    assert_eq!(latest.as_object().map(|object| object.len()), Some(97));
    assert_eq!(
        latest.as_object().map(|object| {
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count()
        }),
        Some(20)
    );
}

fn inherited_scalar_fields() -> [&'static str; 41] {
    [
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "heating_availability_guard_false_fallthrough",
        "humidification_control_guard_false_fallthrough",
        "dehumidification_control_humidistat_maximum_assignment_executed",
        "dehumidification_control_none_maximum_assignment_executed",
        "dehumidification_control_guard_false_fallthrough",
        "predecessor_capacity_limit_guard_evaluated",
        "predecessor_capacity_limit_body_entered",
        "predecessor_active_capacity_limit_guard_false_fallthrough",
        "predecessor_dehumidification_guard_evaluated",
        "predecessor_dehumidification_body_entered",
        "predecessor_dehumidification_guard_false_fallthrough",
        "predecessor_dehumidification_total_output_assignment_executed",
        "predecessor_dehumidification_total_output_capacity_guard_evaluated",
        "predecessor_dehumidification_total_output_capacity_adjustment_body_entered",
        "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_maximum_capacity_assignment_executed",
        "predecessor_supply_enthalpy_assignment_executed",
        "predecessor_dehumidification_control_type_read",
        "predecessor_dehumidification_control_type",
        "predecessor_dehumidification_control_switch_dispatched",
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered",
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break",
        "predecessor_dehumidification_control_humidistat_case_entered",
        "predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed",
        "predecessor_dehumidification_control_humidistat_case_exited_via_break",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break",
        "predecessor_dehumidification_control_default_case_exited_via_break",
        "post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed",
        "cp410_retained_supply_humidity_ratio_state_owned",
        "cp410_retained_supply_enthalpy_state_owned",
        "cp410_retained_supply_temperature_state_owned",
        "cp410_retained_supply_humidity_ratio_owned_read",
        "purchased_air_supply_humidity_ratio_read",
        "local_supply_humidity_ratio_original_assignment_performed",
    ]
}

fn inherited_numeric_fields() -> [&'static str; 9] {
    [
        "predecessor_cp409_resulting_supply_humidity_ratio",
        "predecessor_cp409_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp409_resulting_supply_temperature_c",
        "predecessor_cp410_resulting_supply_humidity_ratio",
        "predecessor_cp410_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp410_resulting_supply_temperature_c",
        "purchased_air_supply_humidity_ratio_before_saturation_check",
        "assigned_supply_humidity_ratio_original",
        "resulting_supply_humidity_ratio_original",
    ]
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        ieee_bits(next, next_field),
        ieee_bits(previous, previous_field),
        "CP412 {next_field} lineage"
    );
}

fn ieee_bits(value: &Value, field: &str) -> Option<u64> {
    let sidecar = &value[format!("{field}_ieee_bits")];
    if sidecar.is_null() {
        return None;
    }
    let text = sidecar
        .as_str()
        .expect("CP412 IEEE sidecar must be a string");
    let hex = text
        .strip_prefix("0x")
        .expect("CP412 IEEE sidecar must use a hexadecimal prefix");
    let bits = u64::from_str_radix(hex, 16).expect("CP412 IEEE sidecar must contain binary64 bits");
    Some(bits)
}

fn number(value: &Value, field: &str) -> f64 {
    value[field]
        .as_f64()
        .expect("CP412 value must be a finite JSON number")
}

fn boolean(value: &Value, field: &str) -> bool {
    value[field]
        .as_bool()
        .expect("CP412 value must be a boolean")
}

fn count(value: &Value, field: &str) -> u64 {
    count_value(&value[field], field)
}

fn count_value(value: &Value, _field: &str) -> u64 {
    value
        .as_u64()
        .expect("CP412 value must be an unsigned count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    match value[field].as_array() {
        Some(values) => values.as_slice(),
        None => &[],
    }
}

fn sum(values: &[Value]) -> u64 {
    values
        .iter()
        .map(|value| count_value(value, "route count"))
        .sum()
}

fn sum_indices(values: &[Value], indices: &[usize]) -> u64 {
    indices
        .iter()
        .map(|index| count_value(&values[*index], "route count"))
        .sum()
}
