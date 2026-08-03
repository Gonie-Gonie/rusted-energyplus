//! CP408 shared-case supply-temperature mixed-air-limit assertions.

#[path = "cp409_assertions.rs"]
mod cp409_assertions;

use serde_json::{Map, Value, json};

const CP329_KEY: &str = "purchased_air_calc_cooling_mixed_air_call_lifecycle";
const CP407_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_lifecycle";
const CP408_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_lifecycle";
const ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let mixed_air_owner = &runtime[CP329_KEY];
    let predecessor = &runtime[CP407_KEY];
    let lifecycle = &runtime[CP408_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2304"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2306"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(lifecycle["system"], mixed_air_owner["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
    );
    assert_eq!(
        lifecycle["transition_count"],
        mixed_air_owner["transition_count"]
    );
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
            "predecessor_else_branch_entry_route_counts",
            "predecessor_else_branch_entry_route_counts",
        ),
        (
            "predecessor_supply_temperature_assignment_route_counts",
            "supply_temperature_assignment_route_counts",
        ),
        (
            "supply_temperature_mixed_air_limit_route_counts",
            "supply_temperature_assignment_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP408 {next}");
        assert_eq!(array(lifecycle, next).len(), 30, "CP408 {next} width");
    }

    let transitions = count(lifecycle, "transition_count");
    let inactive = count(lifecycle, "inactive_transition_count");
    let assignments = count(
        lifecycle,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count",
    );
    assert_eq!(
        assignments,
        count(
            predecessor,
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count",
        )
    );
    assert_eq!(inactive, count(predecessor, "inactive_transition_count"));
    assert_eq!(inactive + assignments, transitions);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignments * ORDER.len() as u64
    );
    for field in [
        "cp407_retained_supply_temperature_owned_read_count",
        "supply_temperature_for_minimum_read_count",
        "cp329_retained_mixed_air_temperature_owned_read_count",
        "mixed_air_temperature_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_temperature_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP408 {field}");
    }

    let routes = array(lifecycle, "predecessor_route_counts");
    let owner_count = routes[3..]
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .sum::<u64>();
    assert_eq!(
        count(lifecycle, "cp407_supply_temperature_state_owner_count"),
        owner_count
    );
    assert!(owner_count >= assignments);
    assert_eq!(
        count(lifecycle, "unchanged_supply_temperature_preservation_count"),
        owner_count - assignments
    );
    for field in [
        "unchanged_supply_humidity_ratio_preservation_count",
        "unchanged_supply_enthalpy_preservation_count",
    ] {
        assert_eq!(count(lifecycle, field), count(predecessor, field));
    }
    for (index, value) in routes.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public CP408 predecessor route {index}"
            );
        }
    }

    assert_latest_lineage(
        &lifecycle["latest"],
        &predecessor["latest"],
        &mixed_air_owner["latest"],
        transitions,
    );
    let serialized = lifecycle.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "zone_load", "report", "numerical_dto"] {
        assert!(!serialized.contains(forbidden), "CP408 forbids {forbidden}");
    }
    assert!(
        !results.to_string().contains(CP408_KEY),
        "CP408 lifecycle must remain outside numerical result state"
    );
    cp409_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP408_KEY));
    assert!(
        runtime[CP408_KEY].is_null(),
        "non-direct runtime must not publish CP408 evidence"
    );
    cp409_assertions::assert_non_direct(runtime);
}

fn assert_latest_lineage(
    latest: &Value,
    predecessor: &Value,
    mixed_air_owner: &Value,
    transitions: u64,
) {
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], predecessor["system"]);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    for field in inherited_control_fields() {
        assert_eq!(latest[field], predecessor[field], "CP408 {field} lineage");
    }
    assert_eq!(
        latest["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered"],
        predecessor["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered"]
    );
    for (next, previous) in predecessor_bool_fields() {
        assert_eq!(latest[next], predecessor[previous], "CP408 {next} lineage");
    }
    for (next, previous) in predecessor_numeric_fields() {
        assert_same_bits(latest, predecessor, next, previous);
    }

    let executed = latest
        ["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed"]
        .as_bool()
        .expect("CP408 limit flag");
    assert_eq!(
        executed,
        predecessor
            ["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed"]
            .as_bool()
            .expect("CP407 assignment flag")
    );
    assert_same_bits(
        latest,
        predecessor,
        "preexisting_supply_temperature_c",
        "resulting_supply_temperature_c",
    );
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

    if executed {
        for field in [
            "cp407_retained_supply_temperature_state_owned",
            "cp407_retained_supply_temperature_owned_read",
            "supply_temperature_for_minimum_read",
            "cp329_retained_mixed_air_temperature_owned_read",
            "mixed_air_temperature_for_minimum_read",
            "source_shaped_two_argument_minimum_evaluated",
            "supply_temperature_assignment_performed",
        ] {
            assert_eq!(latest[field], true, "active CP408 {field}");
        }
        assert_same_bits(
            latest,
            predecessor,
            "supply_temperature_before_mixed_air_limit_c",
            "resulting_supply_temperature_c",
        );
        assert_same_bits(
            latest,
            mixed_air_owner,
            "mixed_air_temperature_c",
            "mixed_air_temperature_c",
        );
        let left = f64::from_bits(bits(latest, "supply_temperature_before_mixed_air_limit_c"));
        let right = f64::from_bits(bits(latest, "mixed_air_temperature_c"));
        let expected = if left < right { left } else { right };
        for field in [
            "minimum_supply_temperature_c",
            "assigned_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert_eq!(
                bits(latest, field),
                expected.to_bits(),
                "active CP408 {field}"
            );
        }
    } else {
        assert_eq!(
            latest["cp407_retained_supply_temperature_state_owned"],
            !predecessor["resulting_supply_temperature_c_ieee_bits"].is_null()
        );
        for field in [
            "cp407_retained_supply_temperature_owned_read",
            "supply_temperature_for_minimum_read",
            "cp329_retained_mixed_air_temperature_owned_read",
            "mixed_air_temperature_for_minimum_read",
            "source_shaped_two_argument_minimum_evaluated",
            "supply_temperature_assignment_performed",
        ] {
            assert_eq!(latest[field], false, "inactive CP408 {field}");
        }
        for field in [
            "supply_temperature_before_mixed_air_limit_c",
            "mixed_air_temperature_c",
            "minimum_supply_temperature_c",
            "assigned_supply_temperature_c",
        ] {
            assert!(latest[field].is_null(), "inactive CP408 {field}");
            assert!(
                latest[format!("{field}_ieee_bits")].is_null(),
                "inactive CP408 {field} bits"
            );
        }
        assert_same_bits(
            latest,
            predecessor,
            "resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        );
    }

    let object = latest.as_object().expect("CP408 latest object");
    assert_eq!(object.len(), 95, "CP408 JSON width");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        19,
        "CP408 IEEE sidecar count"
    );
}

fn inherited_control_fields() -> [&'static str; 33] {
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
        "predecessor_dehumidification_control_none_case_entered",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough",
        "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed",
    ]
}

fn predecessor_bool_fields() -> [(&'static str, &'static str); 9] {
    [
        (
            "predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed",
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed",
        ),
        (
            "predecessor_cp385_retained_supply_enthalpy_owned_read",
            "cp385_retained_supply_enthalpy_owned_read",
        ),
        (
            "predecessor_cp406_same_call_supply_enthalpy_bit_corroborated",
            "cp406_same_call_supply_enthalpy_bit_corroborated",
        ),
        (
            "predecessor_supply_enthalpy_for_dry_bulb_inversion_read",
            "supply_enthalpy_for_dry_bulb_inversion_read",
        ),
        (
            "predecessor_cp378_retained_supply_humidity_ratio_owned_read",
            "cp378_retained_supply_humidity_ratio_owned_read",
        ),
        (
            "predecessor_supply_humidity_ratio_for_dry_bulb_inversion_read",
            "supply_humidity_ratio_for_dry_bulb_inversion_read",
        ),
        (
            "predecessor_cp406_retained_supply_temperature_state_owned",
            "cp406_retained_supply_temperature_state_owned",
        ),
        (
            "predecessor_psychrometric_supply_temperature_evaluated",
            "psychrometric_supply_temperature_evaluated",
        ),
        (
            "predecessor_supply_temperature_assigned",
            "supply_temperature_assigned",
        ),
    ]
}

fn predecessor_numeric_fields() -> [(&'static str, &'static str); 11] {
    [
        (
            "predecessor_cp406_resulting_supply_humidity_ratio",
            "predecessor_cp406_resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_cp406_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp406_resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_cp406_resulting_supply_temperature_c",
            "predecessor_cp406_resulting_supply_temperature_c",
        ),
        (
            "predecessor_supply_enthalpy_j_per_kg",
            "supply_enthalpy_j_per_kg",
        ),
        ("predecessor_supply_humidity_ratio", "supply_humidity_ratio"),
        (
            "predecessor_preexisting_supply_temperature_c",
            "preexisting_supply_temperature_c",
        ),
        (
            "predecessor_psychrometric_supply_temperature_result_c",
            "psychrometric_supply_temperature_result_c",
        ),
        (
            "predecessor_assigned_supply_temperature_c",
            "assigned_supply_temperature_c",
        ),
        (
            "predecessor_resulting_supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        ),
        (
            "predecessor_resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        ),
        (
            "predecessor_resulting_supply_temperature_c",
            "resulting_supply_temperature_c",
        ),
    ]
}

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP408 {next_field} lineage"
    );
}

fn bits(value: &Value, field: &str) -> u64 {
    let encoded = value[format!("{field}_ieee_bits")].as_str();
    assert!(encoded.is_some(), "CP408 {field} IEEE bits");
    let parsed = u64::from_str_radix(encoded.unwrap_or_default().trim_start_matches("0x"), 16);
    assert!(parsed.is_ok(), "CP408 {field} valid IEEE bits");
    parsed.unwrap_or_default()
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP408 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    value[field].as_array().expect("CP408 route array")
}
