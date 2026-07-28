//! CP324 run-summary assertions shared by the direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2158-2159";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2161";
const SOURCE_ORDER: [&str; 6] = [
    "read-ems-supply-mass-flow-override-value",
    "assign-supply-mass-flow-rate-from-ems-override",
    "read-outdoor-air-mass-flow-rate-for-minimum",
    "read-supply-mass-flow-rate-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-outdoor-air-mass-flow-rate",
];

pub(super) fn assert_cooling_supply_mass_flow_ems_override_body(
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
    let lifecycle =
        &runtime["purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle"];
    assert!(
        lifecycle.is_object(),
        "direct runtime must publish the CP324 key"
    );
    assert_exact_object_keys(
        lifecycle,
        &[
            "source",
            "first_excluded_source",
            "system",
            "transition_count",
            "cooling_body_entry_count",
            "body_entry_count",
            "body_skip_count",
            "unit_off_skip_count",
            "non_cooling_skip_count",
            "ems_disabled_fallthrough_count",
            "ems_supply_mass_flow_override_value_read_count",
            "supply_mass_flow_rate_override_assignment_count",
            "outdoor_air_mass_flow_rate_for_minimum_read_count",
            "supply_mass_flow_rate_for_minimum_read_count",
            "source_shaped_two_argument_minimum_evaluation_count",
            "outdoor_air_mass_flow_rate_assignment_count",
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
    assert_eq!(lifecycle["body_entry_count"], 0);
    assert_eq!(lifecycle["body_skip_count"], expected_calls);
    assert_eq!(lifecycle["unit_off_skip_count"], expected_unit_off_skips);
    assert_eq!(
        lifecycle["non_cooling_skip_count"],
        expected_non_cooling_skips
    );
    assert_eq!(
        lifecycle["ems_disabled_fallthrough_count"],
        expected_cooling_entries
    );
    for field in [
        "ems_supply_mass_flow_override_value_read_count",
        "supply_mass_flow_rate_override_assignment_count",
        "outdoor_air_mass_flow_rate_for_minimum_read_count",
        "supply_mass_flow_rate_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "outdoor_air_mass_flow_rate_assignment_count",
    ] {
        assert_eq!(lifecycle[field], 0, "{field} must remain zero");
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
            "predecessor_ems_supply_mass_flow_override_guard_false_fallthrough",
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "body_skipped",
            "ems_disabled_fallthrough",
            "ems_supply_mass_flow_override_value_read",
            "ems_supply_mass_flow_override_value_kg_per_s",
            "supply_mass_flow_rate_override_assignment_performed",
            "assigned_supply_mass_flow_rate_kg_per_s",
            "outdoor_air_mass_flow_rate_for_minimum_read",
            "outdoor_air_mass_flow_rate_before_override_kg_per_s",
            "supply_mass_flow_rate_for_minimum_read",
            "supply_mass_flow_rate_for_minimum_kg_per_s",
            "source_shaped_two_argument_minimum_evaluated",
            "minimum_outdoor_air_mass_flow_rate_kg_per_s",
            "outdoor_air_mass_flow_rate_assignment_performed",
            "assigned_outdoor_air_mass_flow_rate_kg_per_s",
        ],
    );
    assert_eq!(latest["source"], SOURCE);
    assert_eq!(latest["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(string_array(&latest["source_order"]), SOURCE_ORDER);
    assert_eq!(latest["system"], lifecycle["system"]);
    assert_eq!(latest["parent_call_ordinal"], expected_calls);

    let predecessor = &runtime["purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle"]
        ["latest"];
    assert_eq!(lifecycle["system"], predecessor["system"]);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    assert_eq!(
        latest["unit_body_entered"],
        predecessor["unit_body_entered"]
    );
    assert_eq!(
        latest["predecessor_cooling_body_entered"],
        predecessor["cooling_body_entered"]
    );
    assert_eq!(
        latest["predecessor_ems_supply_mass_flow_override_body_entered"],
        predecessor["ems_supply_mass_flow_override_body_entered"]
    );
    assert_eq!(
        latest["predecessor_ems_supply_mass_flow_override_guard_false_fallthrough"],
        predecessor["ems_supply_mass_flow_override_guard_false_fallthrough"]
    );
    assert_eq!(
        latest["unit_off_skipped"],
        expected_unit_off_skips == expected_calls
    );
    assert_eq!(
        latest["non_cooling_skipped"],
        expected_non_cooling_skips == expected_calls
    );
    assert_eq!(
        latest["cooling_body_entered"],
        expected_cooling_entries == expected_calls
    );
    assert_eq!(latest["body_skipped"], true);
    assert_eq!(
        latest["ems_disabled_fallthrough"],
        expected_cooling_entries == expected_calls
    );

    for field in [
        "ems_supply_mass_flow_override_value_read",
        "supply_mass_flow_rate_override_assignment_performed",
        "outdoor_air_mass_flow_rate_for_minimum_read",
        "supply_mass_flow_rate_for_minimum_read",
        "source_shaped_two_argument_minimum_evaluated",
        "outdoor_air_mass_flow_rate_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "{field} must remain false");
    }
    for field in [
        "ems_supply_mass_flow_override_value_kg_per_s",
        "assigned_supply_mass_flow_rate_kg_per_s",
        "outdoor_air_mass_flow_rate_before_override_kg_per_s",
        "supply_mass_flow_rate_for_minimum_kg_per_s",
        "minimum_outdoor_air_mass_flow_rate_kg_per_s",
        "assigned_outdoor_air_mass_flow_rate_kg_per_s",
    ] {
        assert!(latest[field].is_null(), "{field} must remain null");
    }
}
