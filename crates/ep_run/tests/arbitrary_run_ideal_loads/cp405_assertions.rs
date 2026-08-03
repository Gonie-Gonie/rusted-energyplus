//! CP405 shared-case latent-output maximum-capacity-assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp406_assertions.rs"]
mod cp406_assertions;

const CP404_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_lifecycle";
const CP405_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-assignment",
    "assign-local-cooling-latent-output-from-maximum-total-cooling-capacity",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP404_KEY];
    let lifecycle = &runtime[CP405_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2300"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2302"
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
        lifecycle["cooling_latent_output_maximum_capacity_assignment_route_counts"],
        predecessor["supply_humidity_ratio_assignment_route_counts"]
    );

    let routes = array(lifecycle, "predecessor_route_counts");
    let false_routes = array(
        lifecycle,
        "predecessor_guard_false_fallthrough_route_counts",
    );
    let assignment_routes = array(
        lifecycle,
        "cooling_latent_output_maximum_capacity_assignment_route_counts",
    );
    assert_eq!(
        (routes.len(), false_routes.len(), assignment_routes.len()),
        (30, 30, 30)
    );
    let transitions = count(lifecycle, "transition_count");
    let inherited_inactive = count(lifecycle, "inactive_transition_count");
    let guard_false = count(lifecycle, "predecessor_guard_false_fallthrough_count");
    let assignments = count(
        lifecycle,
        "cooling_latent_output_maximum_capacity_assignment_count",
    );
    assert_eq!(sum(routes), transitions);
    assert_eq!(sum(false_routes), guard_false);
    assert_eq!(sum(assignment_routes), assignments);
    assert_eq!(
        assignments,
        count(predecessor, "supply_humidity_ratio_assignment_count")
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
                "public CP405 route {index}"
            );
        }
    }
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        2 * assignments
    );
    for field in [
        "cp404_retained_maximum_total_cooling_capacity_owned_read_count",
        "maximum_total_cooling_capacity_read_count",
        "cooling_latent_output_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP405 {field}");
    }
    let predecessor_humidity_owners =
        count(predecessor, "cp403_supply_humidity_ratio_state_owner_count");
    let current_humidity_owners = predecessor_humidity_owners + assignments;
    for (field, expected) in [
        (
            "cp404_supply_humidity_ratio_state_owner_count",
            current_humidity_owners,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            current_humidity_owners,
        ),
        (
            "cp404_supply_enthalpy_state_owner_count",
            count(predecessor, "cp403_supply_enthalpy_state_owner_count"),
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            count(predecessor, "cp403_supply_enthalpy_state_owner_count"),
        ),
        (
            "cp404_supply_temperature_state_owner_count",
            count(predecessor, "cp403_supply_temperature_state_owner_count"),
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            count(predecessor, "cp403_supply_temperature_state_owner_count"),
        ),
    ] {
        assert_eq!(count(lifecycle, field), expected, "CP405 {field}");
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
            "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed",
        ),
        (
            "predecessor_cp403_cp329_retained_mixed_air_temperature_owned_read",
            "predecessor_cp403_cp329_retained_mixed_air_temperature_owned_read",
        ),
        (
            "predecessor_cp402_same_call_mixed_air_temperature_bit_corroborated",
            "predecessor_cp402_same_call_mixed_air_temperature_bit_corroborated",
        ),
        (
            "predecessor_cp403_mixed_air_temperature_read",
            "predecessor_cp403_mixed_air_temperature_read",
        ),
        (
            "predecessor_supply_temperature_assigned",
            "predecessor_supply_temperature_assigned",
        ),
        (
            "predecessor_cp402_retained_supply_humidity_ratio_state_owned",
            "predecessor_cp402_retained_supply_humidity_ratio_state_owned",
        ),
        (
            "predecessor_cp402_retained_supply_enthalpy_state_owned",
            "predecessor_cp402_retained_supply_enthalpy_state_owned",
        ),
        (
            "predecessor_cp402_retained_supply_temperature_state_owned",
            "predecessor_cp402_retained_supply_temperature_state_owned",
        ),
    ] {
        assert_eq!(latest[next], prior[previous], "CP405 {next} lineage");
    }
    for (next, previous) in [
        (
            "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed",
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed",
        ),
        (
            "predecessor_cp403_retained_supply_humidity_ratio_state_owned",
            "cp403_retained_supply_humidity_ratio_state_owned",
        ),
        (
            "predecessor_cp403_retained_supply_temperature_state_owned",
            "cp403_retained_supply_temperature_state_owned",
        ),
        (
            "predecessor_cp403_retained_supply_enthalpy_state_owned",
            "cp403_retained_supply_enthalpy_state_owned",
        ),
        (
            "predecessor_cp404_cp403_retained_supply_temperature_owned_read",
            "cp403_retained_supply_temperature_owned_read",
        ),
        (
            "predecessor_supply_temperature_for_humidity_ratio_inversion_read",
            "supply_temperature_for_humidity_ratio_inversion_read",
        ),
        (
            "predecessor_cp404_cp403_retained_supply_enthalpy_owned_read",
            "cp403_retained_supply_enthalpy_owned_read",
        ),
        (
            "predecessor_supply_enthalpy_for_humidity_ratio_inversion_read",
            "supply_enthalpy_for_humidity_ratio_inversion_read",
        ),
        (
            "predecessor_psychrometric_supply_humidity_ratio_evaluated",
            "psychrometric_supply_humidity_ratio_evaluated",
        ),
        (
            "predecessor_supply_humidity_ratio_assignment_performed",
            "supply_humidity_ratio_assignment_performed",
        ),
    ] {
        assert_eq!(latest[next], prior[previous], "CP405 {next} lineage");
    }

    let assignment = prior["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed"].as_bool() == Some(true);
    let guard_evaluated = prior["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated"].as_bool() == Some(true);
    for field in [
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed",
        "cp404_retained_maximum_total_cooling_capacity_owned_read",
        "maximum_total_cooling_capacity_read",
        "cooling_latent_output_assigned",
    ] {
        assert_eq!(latest[field], assignment, "CP405 {field}");
    }
    for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
        assert_same_bits(
            latest,
            prior,
            &format!("resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
    }
    if !guard_evaluated {
        for field in [
            "preexisting_cooling_latent_output_w",
            "maximum_total_cooling_capacity_w",
            "assigned_cooling_latent_output_w",
            "resulting_cooling_latent_output_w",
        ] {
            assert!(latest[field].is_null(), "CP405 inactive {field}");
            assert!(latest[format!("{field}_ieee_bits")].is_null());
        }
    } else {
        assert_same_bits(
            latest,
            prior,
            "preexisting_cooling_latent_output_w",
            "predecessor_cp402_cooling_latent_output_w",
        );
    }
    if assignment {
        assert_same_bits(
            latest,
            prior,
            "maximum_total_cooling_capacity_w",
            "predecessor_maximum_total_cooling_capacity_w",
        );
        for field in [
            "assigned_cooling_latent_output_w",
            "resulting_cooling_latent_output_w",
        ] {
            assert_eq!(
                bits(latest, field).to_bits(),
                bits(latest, "maximum_total_cooling_capacity_w").to_bits(),
                "CP405 {field}"
            );
        }
    } else if guard_evaluated {
        for field in [
            "maximum_total_cooling_capacity_w",
            "assigned_cooling_latent_output_w",
        ] {
            assert!(latest[field].is_null(), "CP405 guard-false {field}");
            assert!(latest[format!("{field}_ieee_bits")].is_null());
        }
        assert_same_bits(
            latest,
            latest,
            "resulting_cooling_latent_output_w",
            "preexisting_cooling_latent_output_w",
        );
    }

    let latest_object = latest.as_object().expect("CP405 latest object");
    assert_eq!(
        latest_object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        54
    );
    for field in numeric_fields() {
        assert!(latest_object.contains_key(field), "CP405 {field}");
        assert!(
            latest_object.contains_key(&format!("{field}_ieee_bits")),
            "CP405 {field} IEEE sidecar"
        );
    }
    for forbidden in ["supply_node", "numerical_dto", "coupling"] {
        assert!(
            !latest_object.contains_key(forbidden),
            "CP405 must not expose {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP405_KEY),
        "CP405 lifecycle must remain outside numerical result state"
    );
    cp406_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP405_KEY));
    assert!(
        runtime[CP405_KEY].is_null(),
        "non-direct runtime must not publish CP405 evidence"
    );
    cp406_assertions::assert_non_direct(runtime);
}

fn inherited_numeric_lineage() -> [(&'static str, &'static str); 47] {
    let prior = super::numeric_fields();
    let mut fields = [("", ""); 47];
    let mut index = 0;
    while index < 40 {
        fields[index] = (prior[index], prior[index]);
        index += 1;
    }
    fields[40] = (
        "predecessor_cp404_supply_temperature_c",
        "supply_temperature_c",
    );
    fields[41] = (
        "predecessor_cp404_supply_enthalpy_j_per_kg",
        "supply_enthalpy_j_per_kg",
    );
    fields[42] = (
        "predecessor_cp404_psychrometric_supply_humidity_ratio",
        "psychrometric_supply_humidity_ratio",
    );
    fields[43] = (
        "predecessor_cp404_assigned_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
    );
    fields[44] = (
        "predecessor_cp404_resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    );
    fields[45] = (
        "predecessor_cp404_resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg",
    );
    fields[46] = (
        "predecessor_cp404_resulting_supply_temperature_c",
        "resulting_supply_temperature_c",
    );
    fields
}

fn numeric_fields() -> [&'static str; 54] {
    let lineage = inherited_numeric_lineage();
    let mut fields = [""; 54];
    let mut index = 0;
    while index < lineage.len() {
        fields[index] = lineage[index].0;
        index += 1;
    }
    fields[47] = "preexisting_cooling_latent_output_w";
    fields[48] = "maximum_total_cooling_capacity_w";
    fields[49] = "assigned_cooling_latent_output_w";
    fields[50] = "resulting_cooling_latent_output_w";
    fields[51] = "resulting_supply_humidity_ratio";
    fields[52] = "resulting_supply_enthalpy_j_per_kg";
    fields[53] = "resulting_supply_temperature_c";
    fields
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP405 {next_field} lineage"
    );
}

fn bits(value: &Value, field: &str) -> f64 {
    let encoded = value[format!("{field}_ieee_bits")]
        .as_str()
        .expect("CP405 IEEE sidecar");
    f64::from_bits(
        u64::from_str_radix(encoded.trim_start_matches("0x"), 16).expect("CP405 hexadecimal bits"),
    )
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP405 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    value[field].as_array().expect("CP405 route array")
}

fn sum(values: &[Value]) -> u64 {
    values
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum()
}
