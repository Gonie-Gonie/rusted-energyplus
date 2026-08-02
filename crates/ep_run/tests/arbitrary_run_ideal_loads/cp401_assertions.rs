//! CP401 shared-case latent-output assignment assertions.

use serde_json::{Map, Value, json};

const CP384_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_lifecycle";
const CP385_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle";
const CP400_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_lifecycle";
const CP401_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-retained-cooling-total-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-difference",
    "read-local-cooling-sensible-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-difference",
    "calculate-cooling-total-output-minus-cooling-sensible-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output",
    "assign-local-cooling-latent-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-case",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP400_KEY];
    let owner = &runtime[CP384_KEY];
    let corroborator = &runtime[CP385_KEY];
    let lifecycle = &runtime[CP401_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2296"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2297"
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

    let routes = lifecycle["predecessor_route_counts"]
        .as_array()
        .expect("CP401 predecessor route counts");
    assert_eq!(routes.len(), 30);
    let route = |index: usize| routes[index].as_u64().unwrap_or_default();
    let transitions = count(lifecycle, "transition_count");
    assert_eq!(
        routes
            .iter()
            .map(|value| value.as_u64().unwrap_or_default())
            .sum::<u64>(),
        transitions
    );
    let assignments = count(
        lifecycle,
        "dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count",
    );
    assert_eq!(route(20) + route(24), assignments);
    assert_eq!(
        assignments,
        count(
            predecessor,
            "dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count"
        )
    );
    for (index, value) in routes.iter().enumerate() {
        if !(0..=8).contains(&index) && index != 20 && index != 24 {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public direct CP401 route {index}"
            );
        }
    }
    assert_eq!(
        count(lifecycle, "inactive_transition_count") + assignments,
        transitions
    );
    let selected_sum = |indices: &[usize]| indices.iter().map(|index| route(*index)).sum::<u64>();
    let humidity_carriers = selected_sum(&[18, 19, 22, 23, 26, 28]);
    let enthalpy_carriers = selected_sum(&[
        5, 8, 11, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    ]);
    let temperature_carriers = selected_sum(&[
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29,
    ]);
    for (field, expected) in [
        (
            "cp400_supply_humidity_ratio_state_owner_count",
            humidity_carriers,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_carriers,
        ),
        ("cp400_supply_enthalpy_state_owner_count", enthalpy_carriers),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_carriers,
        ),
        (
            "cp400_supply_temperature_state_owner_count",
            temperature_carriers,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_carriers,
        ),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP401 {field}");
    }
    for field in [
        "cooling_total_output_owned_read_count",
        "cooling_total_output_bit_corroboration_count",
        "cooling_total_output_read_count",
        "cooling_sensible_output_owned_read_count",
        "cooling_sensible_output_read_count",
        "cooling_latent_output_calculation_count",
        "cooling_latent_output_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP401 {field}");
    }
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        ORDER.len() as u64 * assignments
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    let total_owner = &owner["latest"];
    let total_corroborator = &corroborator["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    for evidence in [prior, total_owner, total_corroborator] {
        assert_eq!(latest["system"], evidence["system"]);
        assert_eq!(latest["controlled_zone"], evidence["controlled_zone"]);
        assert_eq!(
            latest["parent_call_ordinal"],
            evidence["parent_call_ordinal"]
        );
    }
    let active = prior
        ["dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed"]
        .as_bool()
        == Some(true);
    for field in [
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed",
        "cp384_retained_cooling_total_output_owned_read",
        "cp385_cooling_total_output_bit_corroborated",
        "cooling_total_output_read",
        "cp400_retained_cooling_sensible_output_owned_read",
        "cooling_sensible_output_read",
        "cooling_latent_output_calculated",
        "cooling_latent_output_assigned",
    ] {
        assert_eq!(latest[field], active, "CP401 {field}");
    }
    for (flag, carrier) in [
        (
            "cp400_retained_supply_humidity_ratio_state_owned",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "cp400_retained_supply_enthalpy_state_owned",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "cp400_retained_supply_temperature_state_owned",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(
            latest[flag].as_bool(),
            Some(prior[carrier].is_string()),
            "CP401 {flag}"
        );
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

    for field in local_numeric_fields() {
        assert_eq!(
            latest[format!("{field}_ieee_bits")].is_string(),
            active,
            "CP401 {field} bits"
        );
        if active {
            let value = bits(latest, field);
            assert_eq!(
                latest[field].is_number(),
                value.is_finite(),
                "CP401 {field} finite JSON projection"
            );
            assert_eq!(
                latest[field].is_null(),
                !value.is_finite(),
                "CP401 {field} nonfinite JSON projection"
            );
        } else {
            assert!(latest[field].is_null(), "CP401 {field} inactive value");
        }
    }
    if active {
        assert_eq!(
            latest["cooling_total_output_w_ieee_bits"],
            total_owner["resulting_cooling_total_output_w_ieee_bits"],
            "CP401 must read the CP384-owned cooling total output"
        );
        assert_eq!(
            latest["cooling_total_output_w_ieee_bits"],
            total_corroborator["cooling_total_output_w_ieee_bits"],
            "CP385 must corroborate the CP384-owned cooling total output"
        );
        assert_eq!(
            latest["cooling_sensible_output_w_ieee_bits"],
            prior["cooling_sensible_output_w_ieee_bits"],
            "CP401 must read the CP400-owned cooling sensible output"
        );
        let total = bits(latest, "cooling_total_output_w");
        let sensible = bits(latest, "cooling_sensible_output_w");
        let latent = total - sensible;
        assert_eq!(
            bits(latest, "calculated_cooling_latent_output_w").to_bits(),
            latent.to_bits()
        );
        assert_eq!(
            bits(latest, "cooling_latent_output_w").to_bits(),
            latent.to_bits()
        );
    }

    let latest_object = latest.as_object().expect("CP401 latest object");
    assert_eq!(
        latest_object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        30
    );
    for field in numeric_fields() {
        assert!(latest_object.contains_key(field), "CP401 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP401 {field} IEEE sidecar"
        );
    }
    for forbidden in [
        "maximum_total_cooling_capacity_w",
        "supply_node",
        "numerical_dto",
    ] {
        assert!(
            !latest_object.contains_key(forbidden),
            "CP401 boundary must not expose {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP401_KEY),
        "CP401 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP401_KEY));
    assert!(
        runtime[CP401_KEY].is_null(),
        "non-direct runtime must not publish CP401 evidence"
    );
}

fn inherited_numeric_lineage() -> [(&'static str, &'static str); 23] {
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
            "supply_mass_flow_rate_kg_per_s",
        ),
        ("predecessor_cp400_cp_air_j_per_kg_k", "cp_air_j_per_kg_k"),
        (
            "predecessor_supply_mass_flow_rate_times_cp_air_w_per_k",
            "supply_mass_flow_rate_times_cp_air_w_per_k",
        ),
        (
            "predecessor_mixed_air_temperature_c",
            "mixed_air_temperature_c",
        ),
        ("predecessor_supply_temperature_c", "supply_temperature_c"),
        (
            "predecessor_mixed_air_minus_supply_temperature_k",
            "mixed_air_minus_supply_temperature_k",
        ),
        (
            "predecessor_calculated_cooling_sensible_output_w",
            "calculated_cooling_sensible_output_w",
        ),
        (
            "predecessor_cooling_sensible_output_w",
            "cooling_sensible_output_w",
        ),
        (
            "predecessor_cp400_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp400_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp400_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ]
}

fn local_numeric_fields() -> [&'static str; 4] {
    [
        "cooling_total_output_w",
        "cooling_sensible_output_w",
        "calculated_cooling_latent_output_w",
        "cooling_latent_output_w",
    ]
}

fn numeric_fields() -> [&'static str; 30] {
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
        "cooling_total_output_w",
        "cooling_sensible_output_w",
        "calculated_cooling_latent_output_w",
        "cooling_latent_output_w",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP401 {next_field} lineage"
    );
}

fn bits(value: &Value, field: &str) -> f64 {
    let encoded = value[format!("{field}_ieee_bits")].as_str();
    assert!(encoded.is_some(), "CP401 {field} bits");
    let raw = u64::from_str_radix(encoded.unwrap_or_default().trim_start_matches("0x"), 16).ok();
    assert!(raw.is_some(), "CP401 {field} hexadecimal bits");
    f64::from_bits(raw.unwrap_or_default())
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP401 {field} count");
    count.unwrap_or_default()
}
