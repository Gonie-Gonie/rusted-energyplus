//! CP337 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2195";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2196";
const SOURCE_ORDER: [&str; 5] = [
    "read-cooling-limit-for-capacity-comparison",
    "compare-cooling-limit-equal-to-capacity",
    "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "enter-capacity-limit-body-if-compound-condition-satisfied",
];

pub(super) fn assert_cooling_positive_supply_capacity_limit_guard(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
    expected_limit: &str,
) {
    let lifecycle =
        &runtime["purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP337");
    assert_exact_object_keys(
        lifecycle,
        &[
            "source",
            "first_excluded_source",
            "system",
            "transition_count",
            "unit_off_skip_count",
            "non_cooling_skip_count",
            "positive_guard_false_fallthrough_skip_count",
            "capacity_limit_guard_evaluation_count",
            "source_site_execution_count",
            "first_cooling_limit_read_count",
            "cooling_limit_capacity_comparison_count",
            "cooling_limit_capacity_match_count",
            "second_cooling_limit_read_count",
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            "cooling_limit_flow_rate_and_capacity_match_count",
            "cooling_limit_rejected_count",
            "capacity_limit_body_entry_count",
            "active_guard_false_fallthrough_count",
            "latest",
        ],
    );
    assert_eq!(lifecycle["source"], SOURCE);
    assert_eq!(lifecycle["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(lifecycle["transition_count"], expected_calls);
    assert_eq!(lifecycle["unit_off_skip_count"], expected_unit_off_skips);
    assert_eq!(
        lifecycle["non_cooling_skip_count"],
        expected_non_cooling_skips
    );

    let predecessor =
        &runtime["purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
    }
    let active = predecessor["supply_enthalpy_assignment_count"]
        .as_u64()
        .expect("CP336 assignment count");
    assert_eq!(lifecycle["capacity_limit_guard_evaluation_count"], active);
    let capacity_matches = if expected_limit == "LimitCapacity" {
        active
    } else {
        0
    };
    let second_comparisons = active - capacity_matches;
    let combined_matches = if expected_limit == "LimitFlowRateAndCapacity" {
        active
    } else {
        0
    };
    let body_entries = capacity_matches + combined_matches;
    let false_fallthroughs = active - body_entries;
    let source_sites = 2 * active + 2 * second_comparisons + body_entries;
    for (field, expected) in [
        ("source_site_execution_count", source_sites),
        ("first_cooling_limit_read_count", active),
        ("cooling_limit_capacity_comparison_count", active),
        ("cooling_limit_capacity_match_count", capacity_matches),
        ("second_cooling_limit_read_count", second_comparisons),
        (
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            second_comparisons,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_match_count",
            combined_matches,
        ),
        ("cooling_limit_rejected_count", false_fallthroughs),
        ("capacity_limit_body_entry_count", body_entries),
        ("active_guard_false_fallthrough_count", false_fallthroughs),
    ] {
        assert_eq!(lifecycle[field], expected, "{field}");
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
            "predecessor_no_outdoor_air_fallback_entered",
            "predecessor_positive_supply_mass_flow_body_entered",
            "predecessor_active_guard_false_fallthrough",
            "unit_off_skipped",
            "non_cooling_skipped",
            "positive_guard_false_fallthrough_skipped",
            "capacity_limit_guard_evaluated",
            "first_cooling_limit_read",
            "first_cooling_limit",
            "cooling_limit_capacity_comparison_evaluated",
            "cooling_limit_capacity",
            "second_cooling_limit_read",
            "second_cooling_limit",
            "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
            "cooling_limit_flow_rate_and_capacity",
            "cooling_limit_condition_satisfied",
            "cooling_limit_rejected",
            "capacity_limit_body_entered",
            "active_guard_false_fallthrough",
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
        "predecessor_cooling_body_entered",
        "predecessor_no_outdoor_air_fallback_entered",
        "predecessor_positive_supply_mass_flow_body_entered",
        "predecessor_active_guard_false_fallthrough",
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }
    let latest_active = predecessor_latest["supply_enthalpy_assignment_executed"] == true;
    assert_eq!(latest["capacity_limit_guard_evaluated"], latest_active);
    if !latest_active {
        for field in [
            "first_cooling_limit_read",
            "cooling_limit_capacity_comparison_evaluated",
            "second_cooling_limit_read",
            "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
            "cooling_limit_rejected",
            "capacity_limit_body_entered",
            "active_guard_false_fallthrough",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        for field in [
            "first_cooling_limit",
            "cooling_limit_capacity",
            "second_cooling_limit",
            "cooling_limit_flow_rate_and_capacity",
            "cooling_limit_condition_satisfied",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        return;
    }

    let capacity_match = expected_limit == "LimitCapacity";
    let second_comparison = !capacity_match;
    let combined_match = second_comparison && expected_limit == "LimitFlowRateAndCapacity";
    let selected = capacity_match || combined_match;
    assert_eq!(latest["first_cooling_limit_read"], true);
    assert_eq!(latest["first_cooling_limit"], expected_limit);
    assert_eq!(latest["cooling_limit_capacity_comparison_evaluated"], true);
    assert_eq!(latest["cooling_limit_capacity"], capacity_match);
    assert_eq!(latest["second_cooling_limit_read"], second_comparison);
    if second_comparison {
        assert_eq!(latest["second_cooling_limit"], expected_limit);
        assert_eq!(
            latest["cooling_limit_flow_rate_and_capacity"],
            combined_match
        );
    } else {
        assert!(latest["second_cooling_limit"].is_null());
        assert!(latest["cooling_limit_flow_rate_and_capacity"].is_null());
    }
    assert_eq!(
        latest["cooling_limit_flow_rate_and_capacity_comparison_evaluated"],
        second_comparison
    );
    assert_eq!(latest["cooling_limit_condition_satisfied"], selected);
    assert_eq!(latest["cooling_limit_rejected"], !selected);
    assert_eq!(latest["capacity_limit_body_entered"], selected);
    assert_eq!(latest["active_guard_false_fallthrough"], !selected);
}
