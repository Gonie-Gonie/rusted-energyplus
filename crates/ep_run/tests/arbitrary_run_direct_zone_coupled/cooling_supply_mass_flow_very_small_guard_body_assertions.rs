//! CP328 run-summary assertions shared by the direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2167";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2171";
const SOURCE_ORDER: [&str; 1] = ["assign-supply-mass-flow-rate-positive-zero"];
const POSITIVE_ZERO_IEEE_BITS: &str = "0x0000000000000000";

pub(super) fn assert_cooling_supply_mass_flow_very_small_guard_body(
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
        &runtime["purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP328");
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
            "zero_flow_reset_body_entry_count",
            "body_skip_count",
            "active_guard_false_fallthrough_count",
            "supply_mass_flow_rate_positive_zero_assignment_count",
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

    let predecessor =
        &runtime["purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "cooling_body_entry_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "zero_flow_reset_body_entry_count",
        "active_guard_false_fallthrough_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
    }
    let zero_entries = predecessor["zero_flow_reset_body_entry_count"]
        .as_u64()
        .expect("CP327 zero-flow-reset body count");
    let false_fallthroughs = predecessor["active_guard_false_fallthrough_count"]
        .as_u64()
        .expect("CP327 active false-fallthrough count");
    assert_eq!(zero_entries + false_fallthroughs, expected_cooling_entries);
    assert_eq!(
        lifecycle["supply_mass_flow_rate_positive_zero_assignment_count"],
        zero_entries
    );
    assert_eq!(lifecycle["body_skip_count"], expected_calls - zero_entries);

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
            "predecessor_supply_mass_flow_limit_body_entered",
            "predecessor_supply_mass_flow_limit_body_skipped",
            "predecessor_supply_mass_flow_limit_active_guard_false_fallthrough",
            "predecessor_zero_flow_reset_body_entered",
            "predecessor_active_guard_false_fallthrough",
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "zero_flow_reset_body_entered",
            "body_skipped",
            "active_guard_false_fallthrough",
            "predecessor_supply_mass_flow_rate_kg_per_s",
            "predecessor_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_positive_zero_assignment_performed",
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
        "predecessor_supply_mass_flow_limit_body_entered",
        "predecessor_supply_mass_flow_limit_body_skipped",
        "predecessor_supply_mass_flow_limit_active_guard_false_fallthrough",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }
    assert_eq!(
        latest["predecessor_zero_flow_reset_body_entered"],
        predecessor_latest["zero_flow_reset_body_entered"]
    );
    assert_eq!(
        latest["predecessor_active_guard_false_fallthrough"],
        predecessor_latest["active_guard_false_fallthrough"]
    );

    let active = latest["cooling_body_entered"] == true;
    if !active {
        assert_eq!(latest["zero_flow_reset_body_entered"], false);
        assert_eq!(latest["body_skipped"], true);
        assert_eq!(latest["active_guard_false_fallthrough"], false);
        assert_eq!(
            latest["supply_mass_flow_rate_positive_zero_assignment_performed"],
            false
        );
        for field in [
            "predecessor_supply_mass_flow_rate_kg_per_s",
            "predecessor_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "assigned_supply_mass_flow_rate_kg_per_s",
            "assigned_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "resulting_supply_mass_flow_rate_kg_per_s",
            "resulting_supply_mass_flow_rate_kg_per_s_ieee_bits",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        return;
    }

    assert_serialized_value_and_bits(
        latest,
        "predecessor_supply_mass_flow_rate_kg_per_s",
        &predecessor_latest["supply_mass_flow_rate_kg_per_s"],
        &predecessor_latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
    );
    let body_entered = predecessor_latest["zero_flow_reset_body_entered"] == true;
    assert_eq!(latest["zero_flow_reset_body_entered"], body_entered);
    assert_eq!(latest["body_skipped"], !body_entered);
    assert_eq!(
        latest["active_guard_false_fallthrough"],
        predecessor_latest["active_guard_false_fallthrough"]
    );
    assert_eq!(
        latest["supply_mass_flow_rate_positive_zero_assignment_performed"],
        body_entered
    );
    if body_entered {
        assert_eq!(latest["assigned_supply_mass_flow_rate_kg_per_s"], 0.0);
        assert_eq!(
            latest["assigned_supply_mass_flow_rate_kg_per_s_ieee_bits"],
            POSITIVE_ZERO_IEEE_BITS
        );
        assert_eq!(latest["resulting_supply_mass_flow_rate_kg_per_s"], 0.0);
        assert_eq!(
            latest["resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"],
            POSITIVE_ZERO_IEEE_BITS
        );
    } else {
        assert!(latest["assigned_supply_mass_flow_rate_kg_per_s"].is_null());
        assert!(latest["assigned_supply_mass_flow_rate_kg_per_s_ieee_bits"].is_null());
        assert_serialized_value_and_bits(
            latest,
            "resulting_supply_mass_flow_rate_kg_per_s",
            &predecessor_latest["supply_mass_flow_rate_kg_per_s"],
            &predecessor_latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        );
    }
}

fn assert_serialized_value_and_bits(
    object: &Value,
    field: &str,
    expected_value: &Value,
    expected_bits: &Value,
) {
    assert_eq!(object[field], *expected_value, "{field}");
    assert_eq!(
        object[format!("{field}_ieee_bits")],
        *expected_bits,
        "{field}_ieee_bits"
    );
}
