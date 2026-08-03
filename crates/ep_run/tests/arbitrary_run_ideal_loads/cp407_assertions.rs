//! CP407 psychrometric supply-temperature assignment assertions.

#[path = "cp408_assertions.rs"]
mod cp408_assertions;

use serde_json::{Map, Value, json};

const CP378_KEY: &str =
    "purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle";
const CP385_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle";
const CP406_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_lifecycle";
const CP407_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-cp385-retained-supply-enthalpy-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature-dry-bulb-inversion",
    "read-cp378-retained-supply-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature-dry-bulb-inversion",
    "evaluate-psy-tdb-fn-h-w-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature",
    "assign-purchased-air-supply-temperature-after-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-guard-else-branch",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let predecessor = &runtime[CP406_KEY];
    let lifecycle = &runtime[CP407_KEY];
    assert_eq!(
        lifecycle["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2302"
    );
    assert_eq!(
        lifecycle["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2304"
    );
    assert_eq!(lifecycle["latest"]["source_order"], json!(ORDER));
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(
        lifecycle["transition_count"],
        predecessor["transition_count"]
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
            "else_branch_entry_route_counts",
        ),
        (
            "supply_temperature_assignment_route_counts",
            "else_branch_entry_route_counts",
        ),
    ] {
        assert_eq!(lifecycle[next], predecessor[previous], "CP407 {next}");
        assert_eq!(array(lifecycle, next).len(), 30, "CP407 {next} width");
    }

    let transitions = count(lifecycle, "transition_count");
    let inactive = count(lifecycle, "inactive_transition_count");
    let entries = count(lifecycle, "predecessor_else_branch_entry_count");
    let assignments = count(
        lifecycle,
        "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count",
    );
    assert_eq!(inactive, count(predecessor, "inactive_transition_count"));
    assert_eq!(
        entries,
        count(
            predecessor,
            "dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count"
        )
    );
    assert_eq!(assignments, entries);
    assert_eq!(inactive + assignments, transitions);
    assert_eq!(
        count(lifecycle, "source_site_execution_count"),
        assignments * ORDER.len() as u64
    );
    for field in [
        "cp385_retained_supply_enthalpy_owned_read_count",
        "cp406_same_call_supply_enthalpy_bit_corroboration_count",
        "supply_enthalpy_for_dry_bulb_inversion_read_count",
        "cp378_retained_supply_humidity_ratio_owned_read_count",
        "supply_humidity_ratio_for_dry_bulb_inversion_read_count",
        "psychrometric_supply_temperature_evaluation_count",
        "supply_temperature_assignment_write_count",
    ] {
        assert_eq!(count(lifecycle, field), assignments, "CP407 {field}");
    }
    for (index, value) in array(lifecycle, "predecessor_route_counts")
        .iter()
        .enumerate()
    {
        if !matches!(index, 0..=8 | 20 | 24) {
            assert_eq!(
                value.as_u64().unwrap_or_default(),
                0,
                "public CP407 predecessor route {index}"
            );
        }
        assert_eq!(
            array(lifecycle, "supply_temperature_assignment_route_counts")[index],
            array(predecessor, "else_branch_entry_route_counts")[index],
            "CP407 assignment partition {index}"
        );
    }

    assert_latest_lineage(
        &lifecycle["latest"],
        &predecessor["latest"],
        &runtime[CP385_KEY]["latest"],
        &runtime[CP378_KEY]["latest"],
        transitions,
    );
    assert!(
        !results.to_string().contains(CP407_KEY),
        "CP407 lifecycle must remain outside numerical result state"
    );
    cp408_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP407_KEY));
    assert!(
        runtime[CP407_KEY].is_null(),
        "non-direct runtime must not publish CP407 evidence"
    );
    cp408_assertions::assert_non_direct(runtime);
}

fn assert_latest_lineage(
    latest: &Value,
    predecessor: &Value,
    enthalpy_owner: &Value,
    humidity_owner: &Value,
    transitions: u64,
) {
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["system"], predecessor["system"]);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    for field in inherited_control_fields() {
        assert_eq!(latest[field], predecessor[field], "CP407 {field} lineage");
    }
    assert_eq!(
        latest["predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered"],
        predecessor["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered"]
    );
    for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
        assert_same_bits(
            latest,
            predecessor,
            &format!("predecessor_cp406_resulting_supply_{suffix}"),
            &format!("resulting_supply_{suffix}"),
        );
    }

    let executed = latest
        ["dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed"]
        .as_bool()
        .expect("CP407 assignment flag");
    if executed {
        for field in [
            "cp385_retained_supply_enthalpy_owned_read",
            "cp406_same_call_supply_enthalpy_bit_corroborated",
            "supply_enthalpy_for_dry_bulb_inversion_read",
            "cp378_retained_supply_humidity_ratio_owned_read",
            "supply_humidity_ratio_for_dry_bulb_inversion_read",
            "cp406_retained_supply_temperature_state_owned",
            "psychrometric_supply_temperature_evaluated",
            "supply_temperature_assigned",
        ] {
            assert_eq!(latest[field], true, "CP407 {field}");
        }
        assert_same_bits(
            latest,
            enthalpy_owner,
            "supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        );
        assert_same_bits(
            latest,
            predecessor,
            "supply_enthalpy_j_per_kg",
            "resulting_supply_enthalpy_j_per_kg",
        );
        assert_same_bits(
            latest,
            humidity_owner,
            "supply_humidity_ratio",
            "resulting_supply_humidity_ratio",
        );
        assert_same_bits(
            latest,
            predecessor,
            "preexisting_supply_temperature_c",
            "resulting_supply_temperature_c",
        );
        let enthalpy = latest["supply_enthalpy_j_per_kg"]
            .as_f64()
            .expect("finite CP407 enthalpy");
        let humidity = latest["supply_humidity_ratio"]
            .as_f64()
            .expect("finite CP407 humidity ratio");
        let expected = ep_runtime::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
        for field in [
            "psychrometric_supply_temperature_result_c",
            "assigned_supply_temperature_c",
            "resulting_supply_temperature_c",
        ] {
            assert_eq!(
                latest[format!("{field}_ieee_bits")],
                format!("0x{:016x}", expected.to_bits()),
                "CP407 {field}"
            );
        }
        assert_same_bits(
            latest,
            latest,
            "resulting_supply_humidity_ratio",
            "supply_humidity_ratio",
        );
        assert_same_bits(
            latest,
            latest,
            "resulting_supply_enthalpy_j_per_kg",
            "supply_enthalpy_j_per_kg",
        );
    } else {
        for field in [
            "supply_enthalpy_j_per_kg",
            "supply_humidity_ratio",
            "psychrometric_supply_temperature_result_c",
            "assigned_supply_temperature_c",
        ] {
            assert!(latest[field].is_null(), "inactive CP407 {field}");
            assert!(
                latest[format!("{field}_ieee_bits")].is_null(),
                "inactive CP407 {field} bits"
            );
        }
        for suffix in ["humidity_ratio", "enthalpy_j_per_kg", "temperature_c"] {
            assert_same_bits(
                latest,
                predecessor,
                &format!("resulting_supply_{suffix}"),
                &format!("resulting_supply_{suffix}"),
            );
        }
    }

    let object = latest.as_object();
    assert!(object.is_some(), "CP407 latest must be an object");
    let Some(object) = object else {
        return;
    };
    assert_eq!(object.len(), 71, "CP407 JSON width");
    assert_eq!(
        object
            .keys()
            .filter(|key| key.ends_with("_ieee_bits"))
            .count(),
        11,
        "CP407 IEEE sidecar count"
    );
    for forbidden in [
        "numerical_dto",
        "coupling",
        "supply_node",
        "zone_load",
        "report",
    ] {
        assert!(!object.contains_key(forbidden), "CP407 forbids {forbidden}");
    }
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

fn assert_same_bits(next: &Value, previous: &Value, next_field: &str, previous_field: &str) {
    assert_eq!(
        next[format!("{next_field}_ieee_bits")],
        previous[format!("{previous_field}_ieee_bits")],
        "CP407 {next_field} lineage"
    );
}

fn count(value: &Value, field: &str) -> u64 {
    value[field].as_u64().expect("CP407 count")
}

fn array<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    value[field].as_array().expect("CP407 route array")
}
