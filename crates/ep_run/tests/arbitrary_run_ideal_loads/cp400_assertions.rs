//! CP400 shared-case sensible-output assignment assertions.

use serde_json::{Map, Value, json};

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP330_KEY: &str = "purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle";
const CP399_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_lifecycle";
const CP400_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_lifecycle";
const ORDER: [&str; 8] = [
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output-first-product",
    "read-local-cp-air-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output-first-product",
    "calculate-supply-mass-flow-rate-times-cp-air-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output",
    "read-purchased-air-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output-difference",
    "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output-difference",
    "calculate-mixed-air-temperature-minus-supply-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output",
    "calculate-mass-flow-cp-air-product-times-temperature-difference-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output",
    "assign-local-cooling-sensible-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-case",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP399_KEY];
    let flow_owner = &runtime[CP330_KEY];
    let mixed_owner = &runtime[CP329_KEY];
    let lifecycle = &runtime[CP400_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2295"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2296"
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
        .expect("CP400 predecessor route counts");
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
        "dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count",
    );
    assert_eq!(route(20) + route(24), assignments);
    assert_eq!(
        assignments,
        count(
            predecessor,
            "dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count"
        )
    );
    for (index, value) in routes.iter().enumerate() {
        if !(0..=8).contains(&index) && index != 20 && index != 24 {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public direct CP400 route {index}"
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
            "cp399_supply_humidity_ratio_state_owner_count",
            humidity_carriers,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_carriers,
        ),
        ("cp399_supply_enthalpy_state_owner_count", enthalpy_carriers),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_carriers,
        ),
        (
            "cp399_supply_temperature_state_owner_count",
            temperature_carriers,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_carriers,
        ),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP400 {field}");
    }
    for field in [
        "supply_mass_flow_rate_owned_read_count",
        "supply_mass_flow_rate_bit_corroboration_count",
        "supply_mass_flow_rate_read_count",
        "cp_air_owned_read_count",
        "cp_air_read_count",
        "supply_mass_flow_rate_times_cp_air_calculation_count",
        "mixed_air_temperature_owned_read_count",
        "mixed_air_temperature_read_count",
        "supply_temperature_owned_read_count",
        "supply_temperature_read_count",
        "mixed_air_minus_supply_temperature_calculation_count",
        "cooling_sensible_output_calculation_count",
        "cooling_sensible_output_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP400 {field}");
    }
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        ORDER.len() as u64 * assignments
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    let flow = &flow_owner["latest"];
    let mixed = &mixed_owner["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    assert_eq!(latest["system"], flow["system"]);
    assert_eq!(latest["system"], mixed["system"]);
    let active = prior
        ["dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed"]
        .as_bool()
        == Some(true);
    for field in [
        "dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed",
        "cp330_retained_supply_mass_flow_rate_owned_read",
        "cp329_supply_mass_flow_rate_bit_corroborated",
        "supply_mass_flow_rate_read",
        "cp399_retained_cp_air_owned_read",
        "cp_air_read",
        "supply_mass_flow_rate_times_cp_air_calculated",
        "cp329_retained_mixed_air_temperature_owned_read",
        "mixed_air_temperature_read",
        "cp399_retained_supply_temperature_owned_read",
        "supply_temperature_read",
        "mixed_air_minus_supply_temperature_calculated",
        "cooling_sensible_output_calculated",
        "cooling_sensible_output_assigned",
    ] {
        assert_eq!(latest[field], active, "CP400 {field}");
    }
    for (flag, carrier) in [
        (
            "cp399_retained_supply_humidity_ratio_state_owned",
            "resulting_supply_humidity_ratio_ieee_bits",
        ),
        (
            "cp399_retained_supply_enthalpy_state_owned",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "cp399_retained_supply_temperature_state_owned",
            "resulting_supply_temperature_c_ieee_bits",
        ),
    ] {
        assert_eq!(
            latest[flag].as_bool(),
            Some(prior[carrier].is_string()),
            "CP400 {flag}"
        );
    }

    for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
        assert_same_bits(
            latest,
            prior,
            &format!("predecessor_cp397_resulting_supply_{suffix}"),
            &format!("predecessor_cp397_resulting_supply_{suffix}"),
        );
        assert_same_bits(
            latest,
            prior,
            &format!("predecessor_cp398_resulting_supply_{suffix}"),
            &format!("predecessor_cp398_resulting_supply_{suffix}"),
        );
        assert_same_bits(
            latest,
            prior,
            &format!("predecessor_cp399_resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
        assert_same_bits(
            latest,
            prior,
            &format!("resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
    }
    for (next, previous) in [
        (
            "predecessor_mixed_air_humidity_ratio",
            "mixed_air_humidity_ratio",
        ),
        (
            "predecessor_psychrometric_cp_air_result_j_per_kg_k",
            "psychrometric_cp_air_result_j_per_kg_k",
        ),
        ("predecessor_cp_air_j_per_kg_k", "cp_air_j_per_kg_k"),
    ] {
        assert_same_bits(latest, prior, next, previous);
    }

    for field in local_numeric_fields() {
        assert_eq!(
            latest[format!("{field}_ieee_bits")].is_string(),
            active,
            "CP400 {field} bits"
        );
        if active {
            let value = bits(latest, field);
            assert_eq!(
                latest[field].is_number(),
                value.is_finite(),
                "CP400 {field} finite JSON projection"
            );
            assert_eq!(
                latest[field].is_null(),
                !value.is_finite(),
                "CP400 {field} nonfinite JSON projection"
            );
        } else {
            assert!(latest[field].is_null(), "CP400 {field} inactive value");
        }
    }
    if active {
        assert_eq!(
            latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
            flow["supply_mass_flow_rate_kg_per_s_ieee_bits"],
            "CP400 must read the CP330-owned supply mass flow"
        );
        assert_eq!(
            mixed["supply_mass_flow_rate_kg_per_s_ieee_bits"],
            flow["supply_mass_flow_rate_kg_per_s_ieee_bits"],
            "CP329 must corroborate the CP330-owned supply mass flow"
        );
        assert_eq!(
            mixed["child_supply_mass_flow_rate_kg_per_s_ieee_bits"],
            flow["supply_mass_flow_rate_kg_per_s_ieee_bits"],
            "CP329 child input must corroborate the CP330-owned supply mass flow"
        );
        assert_eq!(
            latest["mixed_air_temperature_c_ieee_bits"], mixed["mixed_air_temperature_c_ieee_bits"],
            "CP400 must read the CP329-owned mixed-air temperature"
        );
        assert_eq!(
            latest["cp_air_j_per_kg_k_ieee_bits"], prior["cp_air_j_per_kg_k_ieee_bits"],
            "CP400 must read the CP399-owned CpAir"
        );
        assert_eq!(
            latest["supply_temperature_c_ieee_bits"],
            prior["resulting_supply_temperature_c_ieee_bits"],
            "CP400 must read the CP399-carried supply temperature"
        );

        let flow = bits(latest, "supply_mass_flow_rate_kg_per_s");
        let cp_air = bits(latest, "cp_air_j_per_kg_k");
        let mixed_temperature = bits(latest, "mixed_air_temperature_c");
        let supply_temperature = bits(latest, "supply_temperature_c");
        let first_product = flow * cp_air;
        let difference = mixed_temperature - supply_temperature;
        let output = first_product * difference;
        assert_eq!(
            bits(latest, "supply_mass_flow_rate_times_cp_air_w_per_k").to_bits(),
            first_product.to_bits()
        );
        assert_eq!(
            bits(latest, "mixed_air_minus_supply_temperature_k").to_bits(),
            difference.to_bits()
        );
        assert_eq!(
            bits(latest, "calculated_cooling_sensible_output_w").to_bits(),
            output.to_bits()
        );
        assert_eq!(
            bits(latest, "cooling_sensible_output_w").to_bits(),
            output.to_bits()
        );
    }

    let latest_object = latest.as_object().expect("CP400 latest object");
    assert_eq!(
        latest_object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        numeric_fields().len()
    );
    for field in numeric_fields() {
        assert!(latest_object.contains_key(field), "CP400 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP400 {field} IEEE sidecar"
        );
    }
    for forbidden in [
        "cooling_latent_output_w",
        "maximum_total_cooling_capacity_w",
        "supply_node",
        "numerical_dto",
    ] {
        assert!(
            !latest_object.contains_key(forbidden),
            "CP400 boundary must not expose {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP400_KEY),
        "CP400 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP400_KEY));
    assert!(
        runtime[CP400_KEY].is_null(),
        "non-direct runtime must not publish CP400 evidence"
    );
}

fn local_numeric_fields() -> [&'static str; 8] {
    [
        "supply_mass_flow_rate_kg_per_s",
        "cp_air_j_per_kg_k",
        "supply_mass_flow_rate_times_cp_air_w_per_k",
        "mixed_air_temperature_c",
        "supply_temperature_c",
        "mixed_air_minus_supply_temperature_k",
        "calculated_cooling_sensible_output_w",
        "cooling_sensible_output_w",
    ]
}

fn numeric_fields() -> [&'static str; 23] {
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
        "supply_mass_flow_rate_kg_per_s",
        "cp_air_j_per_kg_k",
        "supply_mass_flow_rate_times_cp_air_w_per_k",
        "mixed_air_temperature_c",
        "supply_temperature_c",
        "mixed_air_minus_supply_temperature_k",
        "calculated_cooling_sensible_output_w",
        "cooling_sensible_output_w",
        "resulting_supply_humidity_ratio",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_temperature_c",
    ]
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP400 {next_field} lineage"
    );
}

fn bits(value: &Value, field: &str) -> f64 {
    let encoded = value[format!("{field}_ieee_bits")].as_str();
    assert!(encoded.is_some(), "CP400 {field} bits");
    let raw = u64::from_str_radix(encoded.unwrap_or_default().trim_start_matches("0x"), 16).ok();
    assert!(raw.is_some(), "CP400 {field} hexadecimal bits");
    f64::from_bits(raw.unwrap_or_default())
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP400 {field} count");
    count.unwrap_or_default()
}
