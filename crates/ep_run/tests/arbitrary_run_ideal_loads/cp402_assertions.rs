//! CP402 shared-case latent-output maximum-capacity guard assertions.

use serde_json::{Map, Value, json};

const CP321_KEY: &str = "purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle";
const CP340_KEY: &str =
    "purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle";
const CP401_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_lifecycle";
const CP402_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_lifecycle";
const ORDER: [&str; 4] = [
    "read-retained-cooling-latent-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-comparison",
    "compare-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cooling-latent-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
    "enter-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-body-if-comparison-satisfied",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP401_KEY];
    let owner = &runtime[CP321_KEY];
    let corroborator = &runtime[CP340_KEY];
    let lifecycle = &runtime[CP402_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2297"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2298"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    assert_eq!(
        lifecycle["predecessor_route_counts"],
        predecessor["predecessor_route_counts"]
    );

    let routes = array(lifecycle, "predecessor_route_counts");
    let guard_false_routes = array(lifecycle, "guard_false_fallthrough_route_counts");
    let body_routes = array(lifecycle, "adjustment_body_entry_route_counts");
    assert_eq!(routes.len(), 30);
    assert_eq!(guard_false_routes.len(), 30);
    assert_eq!(body_routes.len(), 30);
    let route = |index: usize| routes[index].as_u64().unwrap_or_default();
    let guard_false = |index: usize| guard_false_routes[index].as_u64().unwrap_or_default();
    let body = |index: usize| body_routes[index].as_u64().unwrap_or_default();
    let transitions = count(lifecycle, "transition_count");
    assert_eq!(
        routes
            .iter()
            .map(|value| value.as_u64().unwrap_or_default())
            .sum::<u64>(),
        transitions
    );
    let evaluations = count(
        lifecycle,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count",
    );
    assert_eq!(route(20) + route(24), evaluations);
    assert_eq!(
        evaluations,
        count(
            predecessor,
            "dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count"
        )
    );
    for index in 0..30 {
        if !(0..=8).contains(&index) && index != 20 && index != 24 {
            assert_eq!(route(index), 0, "public direct CP402 route {index}");
        }
        let expected = if matches!(index, 20 | 21 | 24 | 25 | 27 | 29) {
            route(index)
        } else {
            0
        };
        assert_eq!(
            guard_false(index) + body(index),
            expected,
            "CP402 successor route {index}"
        );
    }
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + evaluations,
        transitions
    );
    let false_total = guard_false_routes
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum::<u64>();
    let body_total = body_routes
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum::<u64>();
    assert_eq!(false_total + body_total, evaluations);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        3 * evaluations + body_total
    );
    let selected_sum = |indices: &[usize]| indices.iter().map(|index| route(*index)).sum::<u64>();
    let humidity = selected_sum(&[18, 19, 22, 23, 26, 28]);
    let enthalpy = selected_sum(&[
        5, 8, 11, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    ]);
    let temperature = selected_sum(&[
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29,
    ]);
    for (field, expected) in [
        ("cp401_supply_humidity_ratio_state_owner_count", humidity),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity,
        ),
        ("cp401_supply_enthalpy_state_owner_count", enthalpy),
        ("unchanged_supply_enthalpy_preservation_count", enthalpy),
        ("cp401_supply_temperature_state_owner_count", temperature),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature,
        ),
        (
            "cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count",
            body_total,
        ),
        (
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count",
            body_total,
        ),
        (
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count",
            false_total,
        ),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP402 {field}");
    }
    for field in [
        "cp401_cooling_latent_output_owned_read_count",
        "cooling_latent_output_read_count",
        "cp321_maximum_total_cooling_capacity_owned_read_count",
        "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count",
        "maximum_total_cooling_capacity_read_count",
        "cooling_latent_output_maximum_total_cooling_capacity_comparison_count",
    ] {
        assert_eq!(count(lifecycle, field), evaluations, "CP402 {field}");
    }

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    let capacity_owner = &owner["latest"];
    let capacity_corroborator = &corroborator["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    for evidence in [prior, capacity_owner, capacity_corroborator] {
        assert_eq!(latest["system"], evidence["system"]);
        assert_eq!(latest["controlled_zone"], evidence["controlled_zone"]);
        assert_eq!(
            latest["parent_call_ordinal"],
            evidence["parent_call_ordinal"]
        );
    }
    let active = prior
        ["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed"]
        .as_bool()
        == Some(true);
    for field in [
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated",
        "cp401_retained_cooling_latent_output_owned_read",
        "cooling_latent_output_read",
        "cp321_maximum_total_cooling_capacity_owned_read",
        "cp340_same_call_maximum_total_cooling_capacity_bit_corroborated",
        "maximum_total_cooling_capacity_read",
        "cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated",
    ] {
        assert_eq!(latest[field], active, "CP402 {field}");
    }
    for (next, previous) in inherited_numeric_lineage() {
        assert_same_bits(latest, prior, next, previous);
    }
    for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
        assert_same_bits(
            latest,
            prior,
            &format!("resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
    }
    for field in [
        "cooling_latent_output_w",
        "maximum_total_cooling_capacity_w",
    ] {
        assert_eq!(
            latest[format!("{field}_ieee_bits")].is_string(),
            active,
            "CP402 {field} active bits"
        );
    }
    if active {
        assert_eq!(
            latest["cooling_latent_output_w_ieee_bits"], prior["cooling_latent_output_w_ieee_bits"],
            "CP402 must read the CP401-owned latent output"
        );
        assert_eq!(
            latest["maximum_total_cooling_capacity_w_ieee_bits"],
            capacity_owner["maximum_total_cooling_capacity_w_ieee_bits"],
            "CP402 must read the CP321-owned capacity"
        );
        assert_eq!(
            latest["maximum_total_cooling_capacity_w_ieee_bits"],
            capacity_corroborator["maximum_total_cooling_capacity_w_ieee_bits"],
            "CP340 must bit-corroborate the CP321 capacity"
        );
        let comparison = bits(latest, "cooling_latent_output_w")
            >= bits(latest, "maximum_total_cooling_capacity_w");
        assert_eq!(
            latest["cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity"],
            comparison
        );
        assert_eq!(
            latest["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered"],
            comparison
        );
        assert_eq!(
            latest["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough"],
            !comparison
        );
    } else {
        assert!(latest["cooling_latent_output_w"].is_null());
        assert!(latest["maximum_total_cooling_capacity_w"].is_null());
        assert!(
            latest["cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity"]
                .is_null()
        );
    }

    let latest_object = latest.as_object().expect("CP402 latest object");
    assert_eq!(
        latest_object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        35
    );
    for field in numeric_fields() {
        assert!(latest_object.contains_key(field), "CP402 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP402 {field} IEEE sidecar"
        );
        let encoded = &latest[format!("{field}_ieee_bits")];
        if encoded.is_string() {
            let value = bits(latest, field);
            assert_eq!(latest[field].is_number(), value.is_finite());
            assert_eq!(latest[field].is_null(), !value.is_finite());
        } else {
            assert!(latest[field].is_null());
        }
    }
    for forbidden in ["supply_node", "numerical_dto", "coupling"] {
        assert!(
            !latest_object.contains_key(forbidden),
            "CP402 evidence boundary must not expose {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP402_KEY),
        "CP402 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP402_KEY));
    assert!(
        runtime[CP402_KEY].is_null(),
        "non-direct runtime must not publish CP402 evidence"
    );
}

fn inherited_numeric_lineage() -> [(&'static str, &'static str); 30] {
    [
        (
            "predecessor_cp397_resulting_supply_humidity_ratio",
            "predecessor_cp397_resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp397_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp397_resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp397_resulting_supply_temperature_c",
            "predecessor_cp397_resulting_supply_temperature_c",
        ),
        (
            "predecessor_cp398_resulting_supply_humidity_ratio",
            "predecessor_cp398_resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp398_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp398_resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp398_resulting_supply_temperature_c",
            "predecessor_cp398_resulting_supply_temperature_c",
        ),
        (
            "predecessor_mixed_air_humidity_ratio",
            "predecessor_mixed_air_humidity_ratio",
        ),
        (
            "predecessor_psychrometric_cp_air_result_j_per_kg_k",
            "predecessor_psychrometric_cp_air_result_j_per_kg_k",
        ),
        (
            "predecessor_cp_air_j_per_kg_k",
            "predecessor_cp_air_j_per_kg_k",
        ),
        (
            "predecessor_cp399_resulting_supply_humidity_ratio",
            "predecessor_cp399_resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp399_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp399_resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp399_resulting_supply_temperature_c",
            "predecessor_cp399_resulting_supply_temperature_c",
        ),
        (
            "predecessor_supply_mass_flow_rate_kg_per_s",
            "predecessor_supply_mass_flow_rate_kg_per_s",
        ),
        (
            "predecessor_cp400_cp_air_j_per_kg_k",
            "predecessor_cp400_cp_air_j_per_kg_k",
        ),
        (
            "predecessor_supply_mass_flow_rate_times_cp_air_w_per_k",
            "predecessor_supply_mass_flow_rate_times_cp_air_w_per_k",
        ),
        (
            "predecessor_mixed_air_temperature_c",
            "predecessor_mixed_air_temperature_c",
        ),
        (
            "predecessor_supply_temperature_c",
            "predecessor_supply_temperature_c",
        ),
        (
            "predecessor_mixed_air_minus_supply_temperature_k",
            "predecessor_mixed_air_minus_supply_temperature_k",
        ),
        (
            "predecessor_calculated_cooling_sensible_output_w",
            "predecessor_calculated_cooling_sensible_output_w",
        ),
        (
            "predecessor_cooling_sensible_output_w",
            "predecessor_cooling_sensible_output_w",
        ),
        (
            "predecessor_cp400_resulting_supply_humidity_ratio",
            "predecessor_cp400_resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp400_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp400_resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp400_resulting_supply_temperature_c",
            "predecessor_cp400_resulting_supply_temperature_c",
        ),
        (
            "predecessor_cooling_total_output_w",
            "cooling_total_output_w",
        ),
        (
            "predecessor_cp401_cooling_sensible_output_w",
            "cooling_sensible_output_w",
        ),
        (
            "predecessor_calculated_cooling_latent_output_w",
            "calculated_cooling_latent_output_w",
        ),
        (
            "predecessor_cooling_latent_output_w",
            "cooling_latent_output_w",
        ),
        (
            "predecessor_cp401_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp401_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp401_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ]
}

fn numeric_fields() -> [&'static str; 35] {
    [
        "predecessor_cp397_resulting_supply_humidity_ratio",
        "predecessor_cp397_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp397_resulting_supply_temperature_c",
        "predecessor_cp398_resulting_supply_humidity_ratio",
        "predecessor_cp398_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp398_resulting_supply_temperature_c",
        "predecessor_mixed_air_humidity_ratio",
        "predecessor_psychrometric_cp_air_result_j_per_kg_k",
        "predecessor_cp_air_j_per_kg_k",
        "predecessor_cp399_resulting_supply_humidity_ratio",
        "predecessor_cp399_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp399_resulting_supply_temperature_c",
        "predecessor_supply_mass_flow_rate_kg_per_s",
        "predecessor_cp400_cp_air_j_per_kg_k",
        "predecessor_supply_mass_flow_rate_times_cp_air_w_per_k",
        "predecessor_mixed_air_temperature_c",
        "predecessor_supply_temperature_c",
        "predecessor_mixed_air_minus_supply_temperature_k",
        "predecessor_calculated_cooling_sensible_output_w",
        "predecessor_cooling_sensible_output_w",
        "predecessor_cp400_resulting_supply_humidity_ratio",
        "predecessor_cp400_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp400_resulting_supply_temperature_c",
        "predecessor_cooling_total_output_w",
        "predecessor_cp401_cooling_sensible_output_w",
        "predecessor_calculated_cooling_latent_output_w",
        "predecessor_cooling_latent_output_w",
        "predecessor_cp401_resulting_supply_humidity_ratio",
        "predecessor_cp401_resulting_supply_enthalpy_j_per_kg",
        "predecessor_cp401_resulting_supply_temperature_c",
        "cooling_latent_output_w",
        "maximum_total_cooling_capacity_w",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP402 {next_field} lineage"
    );
}

fn bits(value: &Value, field: &str) -> f64 {
    let encoded = value[format!("{field}_ieee_bits")]
        .as_str()
        .expect("CP402 IEEE sidecar");
    let raw =
        u64::from_str_radix(encoded.trim_start_matches("0x"), 16).expect("CP402 hexadecimal bits");
    f64::from_bits(raw)
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP402 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    value[field].as_array().expect("CP402 route array")
}
