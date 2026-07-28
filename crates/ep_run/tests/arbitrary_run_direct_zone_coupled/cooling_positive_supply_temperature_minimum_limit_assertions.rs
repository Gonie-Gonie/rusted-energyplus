//! CP333 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2187";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2189";
const SOURCE_ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-maximum",
    "reread-minimum-cooling-supply-air-temperature-for-maximum",
    "apply-source-shaped-two-argument-maximum",
    "assign-purchased-air-supply-temperature",
];

pub(super) fn assert_cooling_positive_supply_temperature_minimum_limit(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
) {
    let lifecycle =
        &runtime["purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP333");
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
            "supply_temperature_minimum_limit_count",
            "source_site_execution_count",
            "supply_temperature_for_maximum_read_count",
            "minimum_cooling_supply_air_temperature_for_maximum_read_count",
            "source_shaped_two_argument_maximum_evaluation_count",
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
        &runtime["purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle"];
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
        lifecycle["supply_temperature_minimum_limit_count"],
        predecessor["supply_temperature_assignment_count"]
    );

    let executions = lifecycle["supply_temperature_minimum_limit_count"]
        .as_u64()
        .expect("CP333 execution count");
    let false_skips = lifecycle["positive_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP333 false-guard skips");
    assert_eq!(
        expected_unit_off_skips + expected_non_cooling_skips + false_skips + executions,
        expected_calls
    );
    assert_eq!(
        lifecycle["source_site_execution_count"],
        executions * SOURCE_ORDER.len() as u64
    );
    for field in [
        "supply_temperature_for_maximum_read_count",
        "minimum_cooling_supply_air_temperature_for_maximum_read_count",
        "source_shaped_two_argument_maximum_evaluation_count",
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
            "supply_temperature_minimum_limit_executed",
            "supply_temperature_for_maximum_read",
            "supply_temperature_before_minimum_limit_c",
            "supply_temperature_before_minimum_limit_c_ieee_bits",
            "minimum_cooling_supply_air_temperature_for_maximum_read",
            "minimum_cooling_supply_air_temperature_c",
            "minimum_cooling_supply_air_temperature_c_ieee_bits",
            "source_shaped_two_argument_maximum_evaluated",
            "maximum_supply_temperature_c",
            "maximum_supply_temperature_c_ieee_bits",
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
        latest["supply_temperature_minimum_limit_executed"],
        predecessor_latest["supply_temperature_assignment_executed"]
    );

    if latest["supply_temperature_minimum_limit_executed"] != true {
        for field in [
            "supply_temperature_for_maximum_read",
            "minimum_cooling_supply_air_temperature_for_maximum_read",
            "source_shaped_two_argument_maximum_evaluated",
            "supply_temperature_assignment_performed",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        for field in [
            "supply_temperature_before_minimum_limit_c",
            "supply_temperature_before_minimum_limit_c_ieee_bits",
            "minimum_cooling_supply_air_temperature_c",
            "minimum_cooling_supply_air_temperature_c_ieee_bits",
            "maximum_supply_temperature_c",
            "maximum_supply_temperature_c_ieee_bits",
            "assigned_supply_temperature_c",
            "assigned_supply_temperature_c_ieee_bits",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        return;
    }

    for field in [
        "supply_temperature_for_maximum_read",
        "minimum_cooling_supply_air_temperature_for_maximum_read",
        "source_shaped_two_argument_maximum_evaluated",
        "supply_temperature_assignment_performed",
    ] {
        assert_eq!(latest[field], true, "{field}");
    }
    assert_eq!(
        latest["supply_temperature_before_minimum_limit_c_ieee_bits"],
        predecessor_latest["supply_temperature_c_ieee_bits"]
    );

    let sensible_flow_latest =
        &runtime["purchased_air_calc_cooling_sensible_flow_lifecycle"]["latest"];
    let minimum_cooling_supply_air_temperature =
        sensible_flow_latest["minimum_cooling_supply_air_temperature_c"]
            .as_f64()
            .expect("CP318 minimum cooling supply-air temperature");
    assert_eq!(
        latest["minimum_cooling_supply_air_temperature_c_ieee_bits"],
        format!(
            "0x{:016x}",
            minimum_cooling_supply_air_temperature.to_bits()
        )
    );

    let left = f64_from_ieee_bits(
        &latest["supply_temperature_before_minimum_limit_c_ieee_bits"],
        "CP333 left operand bits",
    );
    let right = f64_from_ieee_bits(
        &latest["minimum_cooling_supply_air_temperature_c_ieee_bits"],
        "CP333 right operand bits",
    );
    let maximum = if left < right { right } else { left };
    assert_eq!(
        latest["maximum_supply_temperature_c_ieee_bits"],
        format!("0x{:016x}", maximum.to_bits())
    );
    assert_eq!(
        latest["assigned_supply_temperature_c_ieee_bits"],
        latest["maximum_supply_temperature_c_ieee_bits"]
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
