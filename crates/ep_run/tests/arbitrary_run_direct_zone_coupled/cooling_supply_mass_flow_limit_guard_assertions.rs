//! CP325 run-summary assertions shared by the direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2161-2162";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2163";
const SOURCE_ORDER: [&str; 7] = [
    "read-cooling-limit-for-flow-rate-comparison",
    "compare-cooling-limit-equal-to-flow-rate",
    "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "read-maximum-cooling-air-mass-flow-rate-after-limit-condition-true",
    "compare-maximum-cooling-air-mass-flow-rate-strictly-above-zero",
    "enter-supply-mass-flow-limit-body-if-compound-condition-satisfied",
];

pub(super) fn assert_cooling_supply_mass_flow_limit_guard(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
    expected_cooling_entries: u64,
    expected_limit: Option<&str>,
) {
    assert_eq!(
        expected_unit_off_skips + expected_non_cooling_skips + expected_cooling_entries,
        expected_calls
    );
    let lifecycle = &runtime["purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP325");
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
            "first_cooling_limit_read_count",
            "cooling_limit_flow_rate_comparison_count",
            "cooling_limit_flow_rate_match_count",
            "second_cooling_limit_read_count",
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            "cooling_limit_flow_rate_and_capacity_match_count",
            "cooling_limit_rejected_count",
            "maximum_cooling_air_mass_flow_rate_read_count",
            "maximum_cooling_air_mass_flow_rate_positive_comparison_count",
            "maximum_cooling_air_mass_flow_rate_strictly_positive_count",
            "maximum_cooling_air_mass_flow_rate_not_positive_count",
            "supply_mass_flow_limit_body_entry_count",
            "active_guard_false_fallthrough_count",
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

    let first_match = expected_limit == Some("LimitFlowRate");
    let combined = expected_limit == Some("LimitFlowRateAndCapacity");
    let selected = first_match || combined;
    let second_reads = if first_match {
        0
    } else {
        expected_cooling_entries
    };
    let selected_count = if selected {
        expected_cooling_entries
    } else {
        0
    };
    let maximum =
        runtime["purchased_air_init_lifecycle"]["maximum_cooling_air_mass_flow_rate_kg_per_s"]
            .as_f64()
            .expect("finite initialization maximum");
    let positive = selected && maximum > 0.0;
    let positive_count = if positive {
        expected_cooling_entries
    } else {
        0
    };
    assert_eq!(
        lifecycle["first_cooling_limit_read_count"],
        expected_cooling_entries
    );
    assert_eq!(
        lifecycle["cooling_limit_flow_rate_comparison_count"],
        expected_cooling_entries
    );
    assert_eq!(
        lifecycle["cooling_limit_flow_rate_match_count"],
        if first_match {
            expected_cooling_entries
        } else {
            0
        }
    );
    assert_eq!(lifecycle["second_cooling_limit_read_count"], second_reads);
    assert_eq!(
        lifecycle["cooling_limit_flow_rate_and_capacity_comparison_count"],
        second_reads
    );
    assert_eq!(
        lifecycle["cooling_limit_flow_rate_and_capacity_match_count"],
        if combined {
            expected_cooling_entries
        } else {
            0
        }
    );
    assert_eq!(
        lifecycle["cooling_limit_rejected_count"],
        expected_cooling_entries - selected_count
    );
    assert_eq!(
        lifecycle["maximum_cooling_air_mass_flow_rate_read_count"],
        selected_count
    );
    assert_eq!(
        lifecycle["maximum_cooling_air_mass_flow_rate_positive_comparison_count"],
        selected_count
    );
    assert_eq!(
        lifecycle["maximum_cooling_air_mass_flow_rate_strictly_positive_count"],
        positive_count
    );
    assert_eq!(
        lifecycle["maximum_cooling_air_mass_flow_rate_not_positive_count"],
        selected_count - positive_count
    );
    assert_eq!(
        lifecycle["supply_mass_flow_limit_body_entry_count"],
        positive_count
    );
    assert_eq!(
        lifecycle["active_guard_false_fallthrough_count"],
        expected_cooling_entries - positive_count
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
            "predecessor_ems_supply_mass_flow_override_body_entered",
            "predecessor_ems_supply_mass_flow_override_body_skipped",
            "predecessor_ems_disabled_fallthrough",
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "first_cooling_limit_read",
            "first_cooling_limit",
            "cooling_limit_flow_rate_comparison_evaluated",
            "cooling_limit_flow_rate",
            "second_cooling_limit_read",
            "second_cooling_limit",
            "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
            "cooling_limit_flow_rate_and_capacity",
            "cooling_limit_condition_satisfied",
            "maximum_cooling_air_mass_flow_rate_read",
            "maximum_cooling_air_mass_flow_rate_kg_per_s",
            "maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits",
            "maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated",
            "maximum_cooling_air_mass_flow_rate_strictly_positive",
            "supply_mass_flow_limit_body_entered",
            "active_guard_false_fallthrough",
        ],
    );
    assert_eq!(latest["source"], SOURCE);
    assert_eq!(latest["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(string_array(&latest["source_order"]), SOURCE_ORDER);
    assert_eq!(latest["system"], lifecycle["system"]);
    assert_eq!(latest["parent_call_ordinal"], expected_calls);
    let predecessor = &runtime["purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle"]
        ["latest"];
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
        predecessor["predecessor_ems_supply_mass_flow_override_body_entered"]
    );
    assert_eq!(
        latest["predecessor_ems_supply_mass_flow_override_body_skipped"],
        predecessor["body_skipped"]
    );
    assert_eq!(
        latest["predecessor_ems_disabled_fallthrough"],
        predecessor["ems_disabled_fallthrough"]
    );
    let active = expected_cooling_entries == expected_calls;
    assert_eq!(latest["cooling_body_entered"], active);
    assert_eq!(
        latest["unit_off_skipped"],
        expected_unit_off_skips == expected_calls
    );
    assert_eq!(
        latest["non_cooling_skipped"],
        expected_non_cooling_skips == expected_calls
    );
    if !active {
        for field in [
            "first_cooling_limit",
            "cooling_limit_flow_rate",
            "second_cooling_limit",
            "cooling_limit_flow_rate_and_capacity",
            "cooling_limit_condition_satisfied",
            "maximum_cooling_air_mass_flow_rate_kg_per_s",
            "maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits",
            "maximum_cooling_air_mass_flow_rate_strictly_positive",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        for field in [
            "first_cooling_limit_read",
            "cooling_limit_flow_rate_comparison_evaluated",
            "second_cooling_limit_read",
            "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
            "maximum_cooling_air_mass_flow_rate_read",
            "maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated",
            "supply_mass_flow_limit_body_entered",
            "active_guard_false_fallthrough",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        return;
    }

    let limit = expected_limit.expect("active CP325 limit");
    assert_eq!(latest["first_cooling_limit_read"], true);
    assert_eq!(latest["first_cooling_limit"], limit);
    assert_eq!(latest["cooling_limit_flow_rate_comparison_evaluated"], true);
    assert_eq!(latest["cooling_limit_flow_rate"], first_match);
    assert_eq!(latest["second_cooling_limit_read"], !first_match);
    assert_eq!(
        latest["second_cooling_limit"],
        if first_match {
            Value::Null
        } else {
            Value::String(limit.to_string())
        }
    );
    assert_eq!(
        latest["cooling_limit_flow_rate_and_capacity_comparison_evaluated"],
        !first_match
    );
    assert_eq!(
        latest["cooling_limit_flow_rate_and_capacity"],
        if first_match {
            Value::Null
        } else {
            Value::Bool(combined)
        }
    );
    assert_eq!(latest["cooling_limit_condition_satisfied"], selected);
    assert_eq!(latest["maximum_cooling_air_mass_flow_rate_read"], selected);
    assert_eq!(
        latest["maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated"],
        selected
    );
    assert_eq!(latest["supply_mass_flow_limit_body_entered"], positive);
    assert_eq!(latest["active_guard_false_fallthrough"], !positive);
    if selected {
        assert_eq!(
            latest["maximum_cooling_air_mass_flow_rate_kg_per_s"],
            maximum
        );
        assert_eq!(
            latest["maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits"],
            format!("0x{:016x}", maximum.to_bits())
        );
        assert_eq!(
            latest["maximum_cooling_air_mass_flow_rate_strictly_positive"],
            positive
        );
    } else {
        assert!(latest["maximum_cooling_air_mass_flow_rate_kg_per_s"].is_null());
        assert!(latest["maximum_cooling_air_mass_flow_rate_kg_per_s_ieee_bits"].is_null());
        assert!(latest["maximum_cooling_air_mass_flow_rate_strictly_positive"].is_null());
    }
}
