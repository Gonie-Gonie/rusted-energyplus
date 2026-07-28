//! CP334 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2189";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2190";
const SOURCE_ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];

pub(super) fn assert_cooling_positive_supply_temperature_mixed_air_limit(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
) {
    let lifecycle = &runtime["purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP334");
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
            "supply_temperature_mixed_air_limit_count",
            "source_site_execution_count",
            "supply_temperature_for_minimum_read_count",
            "mixed_air_temperature_for_minimum_read_count",
            "source_shaped_two_argument_minimum_evaluation_count",
            "supply_temperature_assignment_count",
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
        &runtime["purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
    }
    assert_eq!(
        lifecycle["supply_temperature_mixed_air_limit_count"],
        predecessor["supply_temperature_minimum_limit_count"]
    );

    let executions = lifecycle["supply_temperature_mixed_air_limit_count"]
        .as_u64()
        .expect("CP334 execution count");
    let false_skips = lifecycle["positive_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP334 false-guard skips");
    assert_eq!(
        expected_unit_off_skips + expected_non_cooling_skips + false_skips + executions,
        expected_calls
    );
    assert_eq!(
        lifecycle["source_site_execution_count"],
        executions * SOURCE_ORDER.len() as u64
    );
    for field in [
        "supply_temperature_for_minimum_read_count",
        "mixed_air_temperature_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_temperature_assignment_count",
    ] {
        assert_eq!(lifecycle[field], executions, "{field}");
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
            "supply_temperature_mixed_air_limit_executed",
            "supply_temperature_for_minimum_read",
            "supply_temperature_before_mixed_air_limit_c",
            "supply_temperature_before_mixed_air_limit_c_ieee_bits",
            "mixed_air_temperature_for_minimum_read",
            "mixed_air_temperature_c",
            "mixed_air_temperature_c_ieee_bits",
            "source_shaped_two_argument_minimum_evaluated",
            "minimum_supply_temperature_c",
            "minimum_supply_temperature_c_ieee_bits",
            "supply_temperature_assignment_performed",
            "assigned_supply_temperature_c",
            "assigned_supply_temperature_c_ieee_bits",
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
    assert_eq!(
        latest["supply_temperature_mixed_air_limit_executed"],
        predecessor_latest["supply_temperature_minimum_limit_executed"]
    );

    if latest["supply_temperature_mixed_air_limit_executed"] != true {
        for field in [
            "supply_temperature_for_minimum_read",
            "mixed_air_temperature_for_minimum_read",
            "source_shaped_two_argument_minimum_evaluated",
            "supply_temperature_assignment_performed",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        for field in [
            "supply_temperature_before_mixed_air_limit_c",
            "supply_temperature_before_mixed_air_limit_c_ieee_bits",
            "mixed_air_temperature_c",
            "mixed_air_temperature_c_ieee_bits",
            "minimum_supply_temperature_c",
            "minimum_supply_temperature_c_ieee_bits",
            "assigned_supply_temperature_c",
            "assigned_supply_temperature_c_ieee_bits",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        return;
    }

    for field in [
        "supply_temperature_for_minimum_read",
        "mixed_air_temperature_for_minimum_read",
        "source_shaped_two_argument_minimum_evaluated",
        "supply_temperature_assignment_performed",
    ] {
        assert_eq!(latest[field], true, "{field}");
    }
    assert_eq!(
        latest["supply_temperature_before_mixed_air_limit_c_ieee_bits"],
        predecessor_latest["assigned_supply_temperature_c_ieee_bits"]
    );

    let mixed_air_latest =
        &runtime["purchased_air_calc_cooling_mixed_air_call_lifecycle"]["latest"];
    assert_eq!(
        latest["mixed_air_temperature_c_ieee_bits"],
        mixed_air_latest["mixed_air_temperature_c_ieee_bits"]
    );

    let left = f64_from_ieee_bits(
        &latest["supply_temperature_before_mixed_air_limit_c_ieee_bits"],
        "CP334 left operand bits",
    );
    let right = f64_from_ieee_bits(
        &latest["mixed_air_temperature_c_ieee_bits"],
        "CP334 right operand bits",
    );
    let minimum = if left < right { left } else { right };
    assert_eq!(
        latest["minimum_supply_temperature_c_ieee_bits"],
        format!("0x{:016x}", minimum.to_bits())
    );
    assert_eq!(
        latest["assigned_supply_temperature_c_ieee_bits"],
        latest["minimum_supply_temperature_c_ieee_bits"]
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
