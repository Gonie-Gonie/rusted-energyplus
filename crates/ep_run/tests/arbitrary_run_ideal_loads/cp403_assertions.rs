//! CP403 shared-case supply-temperature mixed-air-assignment assertions.

use serde_json::{Map, Value, json};

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP402_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_lifecycle";
const CP403_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-retained-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-supply-temperature-assignment",
    "assign-purchased-air-supply-temperature-from-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP402_KEY];
    let owner = &runtime[CP329_KEY];
    let lifecycle = &runtime[CP403_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2298"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2299"
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
    assert_eq!(
        lifecycle["predecessor_guard_false_fallthrough_route_counts"],
        predecessor["guard_false_fallthrough_route_counts"]
    );
    assert_eq!(
        lifecycle["supply_temperature_mixed_air_assignment_route_counts"],
        predecessor["adjustment_body_entry_route_counts"]
    );

    let routes = array(lifecycle, "predecessor_route_counts");
    let false_routes = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let assignment_routes = array(
        lifecycle,
        "supply_temperature_mixed_air_assignment_route_counts",
    );
    assert_eq!(routes.len(), 30);
    assert_eq!(false_routes.len(), 30);
    assert_eq!(assignment_routes.len(), 30);
    let transitions = count(lifecycle, "transition_count");
    let inherited_inactive = count(lifecycle, "inactive_transition_count");
    let guard_false = count(lifecycle, "predecessor_guard_false_fallthrough_count");
    let assignments = count(lifecycle, "supply_temperature_mixed_air_assignment_count");
    let route = |index: usize| routes[index].as_u64().unwrap_or_default();
    assert_eq!(route(20) + route(24), guard_false + assignments);
    assert_eq!(sum(routes), transitions);
    assert_eq!(sum(false_routes), guard_false);
    assert_eq!(sum(assignment_routes), assignments);
    assert_eq!(inherited_inactive + guard_false + assignments, transitions);
    assert_eq!(
        assignments,
        count(
            predecessor,
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count"
        )
    );
    assert_eq!(
        guard_false,
        count(
            predecessor,
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count"
        )
    );
    for (index, value) in routes.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public CP403 route {index}"
            );
        }
    }
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        2 * assignments
    );
    for field in [
        "cp329_mixed_air_temperature_owned_read_count",
        "cp402_same_call_mixed_air_temperature_bit_corroboration_count",
        "mixed_air_temperature_read_count",
        "supply_temperature_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP403 {field}");
    }
    for (field, predecessor_field) in [
        (
            "cp402_supply_humidity_ratio_state_owner_count",
            "cp401_supply_humidity_ratio_state_owner_count",
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp401_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp402_supply_enthalpy_state_owner_count",
            "cp401_supply_enthalpy_state_owner_count",
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            "cp401_supply_enthalpy_state_owner_count",
        ),
        (
            "cp402_supply_temperature_state_owner_count",
            "cp401_supply_temperature_state_owner_count",
        ),
    ] {
        assert_eq!(
            count(lifecycle, field),
            count(predecessor, predecessor_field),
            "CP403 {field}"
        );
    }
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count") + assignments,
        count(predecessor, "cp401_supply_temperature_state_owner_count")
    );

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    for (next, previous) in inherited_numeric_lineage() {
        assert_same_bits(latest, prior, next, previous);
    }
    let assignment = prior
        ["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered"]
        .as_bool()
        == Some(true);
    for field in [
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed",
        "cp329_retained_mixed_air_temperature_owned_read",
        "cp402_same_call_mixed_air_temperature_bit_corroborated",
        "mixed_air_temperature_read",
        "supply_temperature_assigned",
    ] {
        assert_eq!(latest[field], assignment, "CP403 {field}");
    }
    for suffix in ["humidity_ratio", "enthalpy_j_per_kg"] {
        assert_same_bits(
            latest,
            prior,
            &format!("resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
    }
    if assignment {
        let owned = &owner["latest"];
        for evidence in [prior, owned] {
            assert_eq!(latest["system"], evidence["system"]);
            assert_eq!(latest["controlled_zone"], evidence["controlled_zone"]);
            assert_eq!(
                latest["parent_call_ordinal"],
                evidence["parent_call_ordinal"]
            );
        }
        for field in [
            "mixed_air_temperature_c",
            "assigned_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert_eq!(
                latest[format!("{field}_ieee_bits")],
                owned["mixed_air_temperature_c_ieee_bits"],
                "CP403 {field} CP329 ownership"
            );
        }
        assert_eq!(
            latest["mixed_air_temperature_c_ieee_bits"],
            prior["predecessor_mixed_air_temperature_c_ieee_bits"],
            "CP402 must carry the CP329 mixed-air temperature"
        );
    } else {
        for field in ["mixed_air_temperature_c", "assigned_supply_temperature_c"] {
            assert!(latest[field].is_null(), "CP403 inactive {field}");
            assert!(latest[format!("{field}_ieee_bits")].is_null());
        }
        assert_same_bits(
            latest,
            prior,
            "resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        );
    }

    let latest_object = latest.as_object().expect("CP403 latest object");
    assert_eq!(
        latest_object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        40
    );
    for field in numeric_fields() {
        assert!(latest_object.contains_key(field), "CP403 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP403 {field} IEEE sidecar"
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
            "CP403 evidence boundary must not expose {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP403_KEY),
        "CP403 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP403_KEY));
    assert!(
        runtime[CP403_KEY].is_null(),
        "non-direct runtime must not publish CP403 evidence"
    );
}

fn inherited_numeric_lineage() -> [(&'static str, &'static str); 35] {
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
            "predecessor_cooling_total_output_w",
        ),
        (
            "predecessor_cp401_cooling_sensible_output_w",
            "predecessor_cp401_cooling_sensible_output_w",
        ),
        (
            "predecessor_calculated_cooling_latent_output_w",
            "predecessor_calculated_cooling_latent_output_w",
        ),
        (
            "predecessor_cooling_latent_output_w",
            "predecessor_cooling_latent_output_w",
        ),
        (
            "predecessor_cp401_resulting_supply_humidity_ratio",
            "predecessor_cp401_resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp401_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp401_resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp401_resulting_supply_temperature_c",
            "predecessor_cp401_resulting_supply_temperature_c",
        ),
        (
            "predecessor_cp402_cooling_latent_output_w",
            "cooling_latent_output_w",
        ),
        (
            "predecessor_maximum_total_cooling_capacity_w",
            "maximum_total_cooling_capacity_w",
        ),
        (
            "predecessor_cp402_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp402_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp402_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ]
}

fn numeric_fields() -> [&'static str; 40] {
    let lineage = inherited_numeric_lineage();
    let mut fields = [""; 40];
    let mut index = 0;
    while index < lineage.len() {
        fields[index] = lineage[index].0;
        index += 1;
    }
    fields[35] = "mixed_air_temperature_c";
    fields[36] = "assigned_supply_temperature_c";
    fields[37] = "resulting_supply_humidity_ratio";
    fields[38] = "resulting_supply_enthalpy_j_per_kg";
    fields[39] = "resulting_supply_temperature_c";
    fields
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP403 {next_field} lineage"
    );
}

fn bits(value: &Value, field: &str) -> f64 {
    let encoded = value[format!("{field}_ieee_bits")]
        .as_str()
        .expect("CP403 IEEE sidecar");
    let raw =
        u64::from_str_radix(encoded.trim_start_matches("0x"), 16).expect("CP403 hexadecimal bits");
    f64::from_bits(raw)
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP403 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    value[field].as_array().expect("CP403 route array")
}

fn sum(values: &[Value]) -> u64 {
    values
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum()
}
