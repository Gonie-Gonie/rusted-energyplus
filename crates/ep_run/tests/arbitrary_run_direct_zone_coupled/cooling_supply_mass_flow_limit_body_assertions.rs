//! CP326 run-summary assertions shared by the direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2163";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2166";
const SOURCE_ORDER: [&str; 4] = [
    "read-supply-mass-flow-rate-for-minimum",
    "reread-maximum-cooling-air-mass-flow-rate-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-supply-mass-flow-rate",
];

pub(super) fn assert_cooling_supply_mass_flow_limit_body(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
    expected_cooling_entries: u64,
) {
    assert_eq!(
        expected_unit_off_skips + expected_non_cooling_skips + expected_cooling_entries,
        expected_calls
    );
    let lifecycle = &runtime["purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP326");
    assert_exact_object_keys(
        lifecycle,
        &[
            "source",
            "first_excluded_source",
            "system",
            "transition_count",
            "cooling_body_entry_count",
            "unit_off_skip_count",
            "non_cooling_skip_count",
            "supply_mass_flow_limit_body_entry_count",
            "body_skip_count",
            "active_guard_false_fallthrough_count",
            "supply_mass_flow_rate_for_minimum_read_count",
            "maximum_cooling_air_mass_flow_rate_for_minimum_read_count",
            "source_shaped_two_argument_minimum_evaluation_count",
            "supply_mass_flow_rate_assignment_count",
            "latest",
        ],
    );
    assert_eq!(lifecycle["source"], SOURCE);
    assert_eq!(lifecycle["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(lifecycle["transition_count"], expected_calls);
    assert_eq!(
        lifecycle["cooling_body_entry_count"],
        expected_cooling_entries
    );
    assert_eq!(lifecycle["unit_off_skip_count"], expected_unit_off_skips);
    assert_eq!(
        lifecycle["non_cooling_skip_count"],
        expected_non_cooling_skips
    );

    let predecessor = &runtime["purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle"];
    let expected_body_entries = predecessor["supply_mass_flow_limit_body_entry_count"]
        .as_u64()
        .expect("CP325 body-entry count");
    let expected_active_fallthrough = predecessor["active_guard_false_fallthrough_count"]
        .as_u64()
        .expect("CP325 active fallthrough count");
    assert_eq!(
        lifecycle["supply_mass_flow_limit_body_entry_count"],
        expected_body_entries
    );
    assert_eq!(
        lifecycle["body_skip_count"],
        expected_calls - expected_body_entries
    );
    assert_eq!(
        lifecycle["active_guard_false_fallthrough_count"],
        expected_active_fallthrough
    );
    for field in [
        "supply_mass_flow_rate_for_minimum_read_count",
        "maximum_cooling_air_mass_flow_rate_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_mass_flow_rate_assignment_count",
    ] {
        assert_eq!(lifecycle[field], expected_body_entries, "{field}");
    }

    let latest = &lifecycle["latest"];
    assert_exact_object_keys(
        latest,
        &[
            "source",
            "first_excluded_source",
            "source_order",
            "system",
            "parent_call_ordinal",
            "controlled_zone",
            "unit_body_entered",
            "predecessor_cooling_body_entered",
            "predecessor_ems_supply_mass_flow_override_body_entered",
            "predecessor_ems_supply_mass_flow_override_body_skipped",
            "predecessor_ems_disabled_fallthrough",
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "supply_mass_flow_limit_body_entered",
            "body_skipped",
            "active_guard_false_fallthrough",
            "supply_mass_flow_rate_for_minimum_read",
            "supply_mass_flow_rate_before_limit_kg_per_s",
            "supply_mass_flow_rate_before_limit_kg_per_s_ieee_bits",
            "maximum_cooling_air_mass_flow_rate_for_minimum_read",
            "maximum_cooling_air_mass_flow_rate_kg_per_s",
            "maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits",
            "source_shaped_two_argument_minimum_evaluated",
            "minimum_supply_mass_flow_rate_kg_per_s",
            "minimum_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_assignment_performed",
            "assigned_supply_mass_flow_rate_kg_per_s",
            "assigned_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "resulting_supply_mass_flow_rate_kg_per_s",
            "resulting_supply_mass_flow_rate_kg_per_s_ieee_bits",
        ],
    );
    assert_eq!(latest["source"], SOURCE);
    assert_eq!(latest["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(string_array(&latest["source_order"]), SOURCE_ORDER);
    assert_eq!(latest["system"], lifecycle["system"]);
    assert_eq!(latest["parent_call_ordinal"], expected_calls);

    let predecessor_latest = &predecessor["latest"];
    for field in [
        "controlled_zone",
        "unit_body_entered",
        "unit_off_skipped",
        "non_cooling_skipped",
        "cooling_body_entered",
        "supply_mass_flow_limit_body_entered",
        "active_guard_false_fallthrough",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }
    assert_eq!(
        latest["predecessor_cooling_body_entered"],
        predecessor_latest["cooling_body_entered"]
    );
    for field in [
        "predecessor_ems_supply_mass_flow_override_body_entered",
        "predecessor_ems_supply_mass_flow_override_body_skipped",
        "predecessor_ems_disabled_fallthrough",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }

    let active = latest["cooling_body_entered"] == true;
    let body_entered = latest["supply_mass_flow_limit_body_entered"] == true;
    assert_eq!(latest["body_skipped"], !body_entered);
    if !active {
        for field in [
            "supply_mass_flow_rate_before_limit_kg_per_s",
            "supply_mass_flow_rate_before_limit_kg_per_s_ieee_bits",
            "maximum_cooling_air_mass_flow_rate_kg_per_s",
            "maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits",
            "minimum_supply_mass_flow_rate_kg_per_s",
            "minimum_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "assigned_supply_mass_flow_rate_kg_per_s",
            "assigned_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "resulting_supply_mass_flow_rate_kg_per_s",
            "resulting_supply_mass_flow_rate_kg_per_s_ieee_bits",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        for field in [
            "supply_mass_flow_rate_for_minimum_read",
            "maximum_cooling_air_mass_flow_rate_for_minimum_read",
            "source_shaped_two_argument_minimum_evaluated",
            "supply_mass_flow_rate_assignment_performed",
            "active_guard_false_fallthrough",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        return;
    }

    let source_latest =
        &runtime["purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle"]["latest"];
    let source_supply = source_latest["resulting_supply_mass_flow_rate_kg_per_s"]
        .as_f64()
        .expect("active CP322 resulting supply flow");
    let source_bits = &source_latest["resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"];
    if !body_entered {
        assert_eq!(latest["active_guard_false_fallthrough"], true);
        for field in [
            "supply_mass_flow_rate_before_limit_kg_per_s",
            "supply_mass_flow_rate_before_limit_kg_per_s_ieee_bits",
            "maximum_cooling_air_mass_flow_rate_kg_per_s",
            "maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits",
            "minimum_supply_mass_flow_rate_kg_per_s",
            "minimum_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "assigned_supply_mass_flow_rate_kg_per_s",
            "assigned_supply_mass_flow_rate_kg_per_s_ieee_bits",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        for field in [
            "supply_mass_flow_rate_for_minimum_read",
            "maximum_cooling_air_mass_flow_rate_for_minimum_read",
            "source_shaped_two_argument_minimum_evaluated",
            "supply_mass_flow_rate_assignment_performed",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        assert_number_and_bits(
            latest,
            "resulting_supply_mass_flow_rate_kg_per_s",
            source_supply,
            source_bits,
        );
        return;
    }

    assert_eq!(latest["active_guard_false_fallthrough"], false);
    for field in [
        "supply_mass_flow_rate_for_minimum_read",
        "maximum_cooling_air_mass_flow_rate_for_minimum_read",
        "source_shaped_two_argument_minimum_evaluated",
        "supply_mass_flow_rate_assignment_performed",
    ] {
        assert_eq!(latest[field], true, "{field}");
    }
    assert_number_and_bits(
        latest,
        "supply_mass_flow_rate_before_limit_kg_per_s",
        source_supply,
        source_bits,
    );
    let maximum =
        runtime["purchased_air_init_lifecycle"]["maximum_cooling_air_mass_flow_rate_kg_per_s"]
            .as_f64()
            .expect("finite initialized maximum");
    let maximum_bits = Value::String(format!("0x{:016x}", maximum.to_bits()));
    assert_number_and_bits(
        latest,
        "maximum_cooling_air_mass_flow_rate_kg_per_s",
        maximum,
        &maximum_bits,
    );
    let minimum = if source_supply < maximum {
        source_supply
    } else {
        maximum
    };
    let minimum_bits = Value::String(format!("0x{:016x}", minimum.to_bits()));
    for field in [
        "minimum_supply_mass_flow_rate_kg_per_s",
        "assigned_supply_mass_flow_rate_kg_per_s",
        "resulting_supply_mass_flow_rate_kg_per_s",
    ] {
        assert_number_and_bits(latest, field, minimum, &minimum_bits);
    }
}

fn assert_number_and_bits(object: &Value, field: &str, expected: f64, expected_bits: &Value) {
    let actual = object[field].as_f64().expect(field);
    assert_eq!(actual.to_bits(), expected.to_bits(), "{field}");
    assert_eq!(
        object[format!("{field}_ieee_bits")],
        *expected_bits,
        "{field}_ieee_bits"
    );
}
