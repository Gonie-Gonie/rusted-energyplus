//! CP323 run-summary assertions shared by the direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2157";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2158";
const SOURCE_ORDER: [&str; 3] = [
    "read-ems-supply-mass-flow-override-flag",
    "evaluate-ems-supply-mass-flow-override-guard",
    "enter-ems-supply-mass-flow-override-body-if-enabled",
];

pub(super) fn assert_cooling_supply_mass_flow_ems_override_guard(
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
        &runtime["purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle"];
    assert!(
        lifecycle.is_object(),
        "direct runtime must publish the CP323 key"
    );
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
            "ems_supply_mass_flow_override_flag_read_count",
            "ems_supply_mass_flow_override_guard_evaluation_count",
            "ems_supply_mass_flow_override_body_entry_count",
            "ems_supply_mass_flow_override_guard_false_fallthrough_count",
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
    assert_eq!(
        lifecycle["ems_supply_mass_flow_override_flag_read_count"],
        expected_cooling_entries
    );
    assert_eq!(
        lifecycle["ems_supply_mass_flow_override_guard_evaluation_count"],
        expected_cooling_entries
    );
    assert_eq!(
        lifecycle["ems_supply_mass_flow_override_body_entry_count"],
        0
    );
    assert_eq!(
        lifecycle["ems_supply_mass_flow_override_guard_false_fallthrough_count"],
        expected_cooling_entries
    );

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
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "ems_supply_mass_flow_override_flag_read",
            "ems_supply_mass_flow_override_enabled",
            "ems_supply_mass_flow_override_guard_evaluated",
            "ems_supply_mass_flow_override_body_entered",
            "ems_supply_mass_flow_override_guard_false_fallthrough",
        ],
    );
    assert_eq!(latest["source"], SOURCE);
    assert_eq!(latest["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(string_array(&latest["source_order"]), SOURCE_ORDER);
    assert_eq!(latest["system"], lifecycle["system"]);
    assert_eq!(latest["parent_call_ordinal"], expected_calls);

    let predecessor =
        &runtime["purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle"]["latest"];
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

    if expected_cooling_entries == expected_calls {
        assert_eq!(latest["ems_supply_mass_flow_override_flag_read"], true);
        assert_eq!(latest["ems_supply_mass_flow_override_enabled"], false);
        assert_eq!(
            latest["ems_supply_mass_flow_override_guard_evaluated"],
            true
        );
        assert_eq!(latest["ems_supply_mass_flow_override_body_entered"], false);
        assert_eq!(
            latest["ems_supply_mass_flow_override_guard_false_fallthrough"],
            true
        );
    } else {
        assert_eq!(latest["ems_supply_mass_flow_override_flag_read"], false);
        assert!(latest["ems_supply_mass_flow_override_enabled"].is_null());
        assert_eq!(
            latest["ems_supply_mass_flow_override_guard_evaluated"],
            false
        );
        assert_eq!(latest["ems_supply_mass_flow_override_body_entered"], false);
        assert_eq!(
            latest["ems_supply_mass_flow_override_guard_false_fallthrough"],
            false
        );
    }
}
