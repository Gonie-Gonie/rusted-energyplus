//! CP340 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2198";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2199";
const SOURCE_ORDER: [&str; 4] = [
    "read-retained-cooling-sensible-output-for-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-sensible-output-comparison",
    "compare-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
    "enter-cooling-capacity-adjustment-body-if-comparison-satisfied",
];

pub(super) fn assert_cooling_positive_supply_capacity_limit_sensible_output_guard(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
    expected_comparison_false_fallthroughs: u64,
    expected_adjustment_body_entries: u64,
) {
    let lifecycle = &runtime["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP340");
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
            "capacity_limit_guard_false_fallthrough_skip_count",
            "capacity_limit_sensible_output_guard_evaluation_count",
            "source_site_execution_count",
            "cooling_sensible_output_read_count",
            "maximum_total_cooling_capacity_read_count",
            "cooling_sensible_output_maximum_capacity_comparison_count",
            "capacity_limit_sensible_output_guard_false_fallthrough_count",
            "capacity_limit_sensible_output_adjustment_body_entry_count",
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
    assert_eq!(
        lifecycle["capacity_limit_sensible_output_guard_false_fallthrough_count"],
        expected_comparison_false_fallthroughs
    );
    assert_eq!(
        lifecycle["capacity_limit_sensible_output_adjustment_body_entry_count"],
        expected_adjustment_body_entries
    );

    let predecessor = &runtime["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "capacity_limit_guard_false_fallthrough_skip_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
    }
    assert_eq!(
        lifecycle["capacity_limit_sensible_output_guard_evaluation_count"],
        predecessor["capacity_limit_sensible_output_assignment_count"]
    );

    let evaluations = lifecycle["capacity_limit_sensible_output_guard_evaluation_count"]
        .as_u64()
        .expect("CP340 evaluation count");
    let positive_skips = lifecycle["positive_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP340 positive-guard skips");
    let capacity_skips = lifecycle["capacity_limit_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP340 capacity-limit skips");
    assert_eq!(
        expected_unit_off_skips
            + expected_non_cooling_skips
            + positive_skips
            + capacity_skips
            + expected_comparison_false_fallthroughs
            + expected_adjustment_body_entries,
        expected_calls
    );
    assert_eq!(
        expected_comparison_false_fallthroughs + expected_adjustment_body_entries,
        evaluations
    );
    assert_eq!(
        lifecycle["source_site_execution_count"],
        3 * evaluations + expected_adjustment_body_entries
    );
    for field in [
        "cooling_sensible_output_read_count",
        "maximum_total_cooling_capacity_read_count",
        "cooling_sensible_output_maximum_capacity_comparison_count",
    ] {
        assert_eq!(lifecycle[field], evaluations, "{field}");
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
            "predecessor_capacity_limit_guard_evaluated",
            "predecessor_capacity_limit_body_entered",
            "predecessor_active_capacity_limit_guard_false_fallthrough",
            "predecessor_capacity_limit_cp_air_assignment_executed",
            "predecessor_capacity_limit_sensible_output_assignment_executed",
            "unit_off_skipped",
            "non_cooling_skipped",
            "positive_guard_false_fallthrough_skipped",
            "capacity_limit_guard_false_fallthrough_skipped",
            "capacity_limit_sensible_output_guard_evaluated",
            "cooling_sensible_output_read",
            "cooling_sensible_output_w",
            "cooling_sensible_output_w_ieee_bits",
            "maximum_total_cooling_capacity_read",
            "maximum_total_cooling_capacity_w",
            "maximum_total_cooling_capacity_w_ieee_bits",
            "cooling_sensible_output_maximum_capacity_comparison_evaluated",
            "cooling_sensible_output_at_or_above_maximum_capacity",
            "capacity_limit_sensible_output_guard_false_fallthrough",
            "capacity_limit_sensible_output_adjustment_body_entered",
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
        "predecessor_capacity_limit_guard_evaluated",
        "predecessor_capacity_limit_body_entered",
        "predecessor_active_capacity_limit_guard_false_fallthrough",
        "predecessor_capacity_limit_cp_air_assignment_executed",
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "capacity_limit_guard_false_fallthrough_skipped",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }
    assert_eq!(
        latest["predecessor_capacity_limit_sensible_output_assignment_executed"],
        predecessor_latest["capacity_limit_sensible_output_assignment_executed"]
    );
    assert_eq!(
        latest["capacity_limit_sensible_output_guard_evaluated"],
        predecessor_latest["capacity_limit_sensible_output_assignment_executed"]
    );

    if latest["capacity_limit_sensible_output_guard_evaluated"] != true {
        for flag in [
            "cooling_sensible_output_read",
            "maximum_total_cooling_capacity_read",
            "cooling_sensible_output_maximum_capacity_comparison_evaluated",
            "capacity_limit_sensible_output_guard_false_fallthrough",
            "capacity_limit_sensible_output_adjustment_body_entered",
        ] {
            assert_eq!(latest[flag], false, "{flag}");
        }
        for value in [
            "cooling_sensible_output_w",
            "cooling_sensible_output_w_ieee_bits",
            "maximum_total_cooling_capacity_w",
            "maximum_total_cooling_capacity_w_ieee_bits",
            "cooling_sensible_output_at_or_above_maximum_capacity",
        ] {
            assert!(latest[value].is_null(), "{value}");
        }
        return;
    }

    for flag in [
        "cooling_sensible_output_read",
        "maximum_total_cooling_capacity_read",
        "cooling_sensible_output_maximum_capacity_comparison_evaluated",
    ] {
        assert_eq!(latest[flag], true, "{flag}");
    }
    assert_eq!(
        latest["cooling_sensible_output_w_ieee_bits"],
        predecessor_latest["cooling_sensible_output_w_ieee_bits"]
    );
    let retained_capacity = &runtime["purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle"]
        ["latest"]["maximum_total_cooling_capacity_w"];
    let retained_capacity = retained_capacity
        .as_f64()
        .expect("active CP340 must have finite CP321 capacity");
    assert_eq!(
        latest["maximum_total_cooling_capacity_w_ieee_bits"],
        format!("0x{:016x}", retained_capacity.to_bits())
    );

    let cooling_output = f64_from_ieee_bits(
        &latest["cooling_sensible_output_w_ieee_bits"],
        "CP340 cooling sensible output bits",
    );
    let maximum_capacity = f64_from_ieee_bits(
        &latest["maximum_total_cooling_capacity_w_ieee_bits"],
        "CP340 maximum cooling capacity bits",
    );
    let expected_comparison = cooling_output >= maximum_capacity;
    assert_eq!(
        latest["cooling_sensible_output_at_or_above_maximum_capacity"],
        expected_comparison
    );
    assert_eq!(
        latest["capacity_limit_sensible_output_guard_false_fallthrough"],
        !expected_comparison
    );
    assert_eq!(
        latest["capacity_limit_sensible_output_adjustment_body_entered"],
        expected_comparison
    );
}

fn f64_from_ieee_bits(value: &Value, label: &str) -> f64 {
    let encoded = value.as_str().expect(label);
    let bits = u64::from_str_radix(
        encoded
            .strip_prefix("0x")
            .expect("IEEE bit string must use 0x prefix"),
        16,
    )
    .expect("IEEE bit string must contain sixteen hexadecimal digits");
    f64::from_bits(bits)
}
