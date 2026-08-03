//! CP404 shared-case supply-humidity-ratio-assignment assertions.

use serde_json::{Map, Value, json};

const CP403_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_lifecycle";
const CP404_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment",
    "assign-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP403_KEY];
    let lifecycle = &runtime[CP404_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2299"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2300"
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
        predecessor["predecessor_guard_false_fallthrough_route_counts"]
    );
    assert_eq!(
        lifecycle["supply_humidity_ratio_assignment_route_counts"],
        predecessor["supply_temperature_mixed_air_assignment_route_counts"]
    );

    let routes = array(lifecycle, "predecessor_route_counts");
    let false_routes = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let assignment_routes = array(lifecycle, "supply_humidity_ratio_assignment_route_counts");
    assert_eq!(
        (routes.len(), false_routes.len(), assignment_routes.len()),
        (30, 30, 30)
    );
    let transitions = count(lifecycle, "transition_count");
    let inherited_inactive = count(lifecycle, "inactive_transition_count");
    let guard_false = count(lifecycle, "predecessor_guard_false_fallthrough_count");
    let assignments = count(lifecycle, "supply_humidity_ratio_assignment_count");
    assert_eq!(sum(routes), transitions);
    assert_eq!(sum(false_routes), guard_false);
    assert_eq!(sum(assignment_routes), assignments);
    assert_eq!(
        assignments,
        count(predecessor, "supply_temperature_mixed_air_assignment_count")
    );
    assert_eq!(
        inherited_inactive,
        count(predecessor, "inactive_transition_count")
    );
    assert_eq!(inherited_inactive + guard_false + assignments, transitions);
    for (index, value) in routes.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public CP404 route {index}"
            );
        }
    }
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        4 * assignments
    );
    for field in [
        "supply_temperature_owned_read_count",
        "supply_temperature_for_humidity_ratio_inversion_read_count",
        "supply_enthalpy_owned_read_count",
        "cp385_same_call_supply_enthalpy_bit_corroboration_count",
        "supply_enthalpy_for_humidity_ratio_inversion_read_count",
        "psychrometric_supply_humidity_ratio_evaluation_count",
        "supply_humidity_ratio_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP404 {field}");
    }
    for (field, predecessor_field) in [
        (
            "cp403_supply_humidity_ratio_state_owner_count",
            "cp402_supply_humidity_ratio_state_owner_count",
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            "cp402_supply_humidity_ratio_state_owner_count",
        ),
        (
            "cp403_supply_enthalpy_state_owner_count",
            "cp402_supply_enthalpy_state_owner_count",
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            "cp402_supply_enthalpy_state_owner_count",
        ),
        (
            "cp403_supply_temperature_state_owner_count",
            "cp402_supply_temperature_state_owner_count",
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            "cp402_supply_temperature_state_owner_count",
        ),
    ] {
        assert_eq!(
            count(lifecycle, field),
            count(predecessor, predecessor_field),
            "CP404 {field}"
        );
    }

    let latest = &lifecycle["latest"];
    let prior = &predecessor["latest"];
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], prior["system"]);
    assert_eq!(latest["controlled_zone"], prior["controlled_zone"]);
    for (next, previous) in inherited_numeric_lineage() {
        assert_same_bits(latest, prior, next, previous);
    }
    for (next, previous) in [
        (
            "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed",
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed",
        ),
        (
            "predecessor_cp403_cp329_retained_mixed_air_temperature_owned_read",
            "cp329_retained_mixed_air_temperature_owned_read",
        ),
        (
            "predecessor_cp402_same_call_mixed_air_temperature_bit_corroborated",
            "cp402_same_call_mixed_air_temperature_bit_corroborated",
        ),
        (
            "predecessor_cp403_mixed_air_temperature_read",
            "mixed_air_temperature_read",
        ),
        (
            "predecessor_supply_temperature_assigned",
            "supply_temperature_assigned",
        ),
        (
            "predecessor_cp402_retained_supply_humidity_ratio_state_owned",
            "cp402_retained_supply_humidity_ratio_state_owned",
        ),
        (
            "predecessor_cp402_retained_supply_enthalpy_state_owned",
            "cp402_retained_supply_enthalpy_state_owned",
        ),
        (
            "predecessor_cp402_retained_supply_temperature_state_owned",
            "cp402_retained_supply_temperature_state_owned",
        ),
    ] {
        assert_eq!(latest[next], prior[previous], "CP404 {next} lineage");
    }

    let assignment = prior["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed"].as_bool() == Some(true);
    for field in [
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed",
        "cp403_retained_supply_temperature_owned_read",
        "supply_temperature_for_humidity_ratio_inversion_read",
        "cp403_retained_supply_enthalpy_owned_read",
        "supply_enthalpy_for_humidity_ratio_inversion_read",
        "psychrometric_supply_humidity_ratio_evaluated",
        "supply_humidity_ratio_assignment_performed",
    ] {
        assert_eq!(latest[field], assignment, "CP404 {field}");
    }
    for suffix in ["enthalpy_j_per_kg", "temperature_c"] {
        assert_same_bits(
            latest,
            prior,
            &format!("resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
    }
    if assignment {
        assert_eq!(
            prior["predecessor_supply_enthalpy_assignment_executed"], true,
            "CP385 enthalpy ownership must be retained"
        );
        assert_same_bits(
            latest,
            prior,
            "supply_temperature_c",
            "resulting_supply_temperature_c",
        );
        assert_same_bits(
            latest,
            prior,
            "supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        );
        let expected = ep_runtime::psychrometrics::energyplus_psy_w_fn_tdb_h(
            bits(latest, "supply_temperature_c"),
            bits(latest, "supply_enthalpy_j_per_kg"),
        );
        assert_eq!(
            bits(latest, "psychrometric_supply_humidity_ratio").to_bits(),
            expected.to_bits()
        );
        for field in [
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ] {
            assert_eq!(
                bits(latest, field).to_bits(),
                expected.to_bits(),
                "CP404 {field}"
            );
        }
    } else {
        for field in [
            "supply_temperature_c",
            "supply_enthalpy_j_per_kg",
            "psychrometric_supply_humidity_ratio",
            "assigned_supply_humidity_ratio",
        ] {
            assert!(latest[field].is_null(), "CP404 inactive {field}");
            assert!(latest[format!("{field}_ieee_bits")].is_null());
        }
        assert_same_bits(
            latest,
            prior,
            "resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        );
    }

    let latest_object = latest.as_object().expect("CP404 latest object");
    assert_eq!(
        latest_object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        47
    );
    for field in numeric_fields() {
        assert!(latest_object.contains_key(field), "CP404 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP404 {field} IEEE sidecar"
        );
    }
    for forbidden in ["supply_node", "numerical_dto", "coupling"] {
        assert!(
            !latest_object.contains_key(forbidden),
            "CP404 must not expose {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP404_KEY),
        "CP404 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP404_KEY));
    assert!(
        runtime[CP404_KEY].is_null(),
        "non-direct runtime must not publish CP404 evidence"
    );
}

fn inherited_numeric_lineage() -> [(&'static str, &'static str); 40] {
    let prior = super::numeric_fields();
    let mut fields = [("", ""); 40];
    let mut index = 0;
    while index < 35 {
        fields[index] = (prior[index], prior[index]);
        index += 1;
    }
    fields[35] = (
        "predecessor_cp403_mixed_air_temperature_c",
        "mixed_air_temperature_c",
    );
    fields[36] = (
        "predecessor_cp403_assigned_supply_temperature_c",
        "assigned_supply_temperature_c",
    );
    fields[37] = (
        "predecessor_cp403_resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    );
    fields[38] = (
        "predecessor_cp403_resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    );
    fields[39] = (
        "predecessor_cp403_resulting_supply_temperature_c",
        "resulting_supply_temperature_c",
    );
    fields
}

fn numeric_fields() -> [&'static str; 47] {
    let lineage = inherited_numeric_lineage();
    let mut fields = [""; 47];
    let mut index = 0;
    while index < lineage.len() {
        fields[index] = lineage[index].0;
        index += 1;
    }
    fields[40] = "supply_temperature_c";
    fields[41] = "supply_enthalpy_j_per_kg";
    fields[42] = "psychrometric_supply_humidity_ratio";
    fields[43] = "assigned_supply_humidity_ratio";
    fields[44] = "resulting_supply_humidity_ratio";
    fields[45] = "resulting_supply_enthalpy_j_per_kg";
    fields[46] = "resulting_supply_temperature_c";
    fields
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP404 {next_field} lineage"
    );
}

fn bits(value: &Value, field: &str) -> f64 {
    let encoded = value[format!("{field}_ieee_bits")]
        .as_str()
        .expect("CP404 IEEE sidecar");
    f64::from_bits(
        u64::from_str_radix(encoded.trim_start_matches("0x"), 16).expect("CP404 hexadecimal bits"),
    )
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP404 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    value[field].as_array().expect("CP404 route array")
}

fn sum(values: &[Value]) -> u64 {
    values
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum()
}
