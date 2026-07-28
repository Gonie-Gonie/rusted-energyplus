//! CP331 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2185";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2186";
const SOURCE_ORDER: [&str; 3] = [
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
];

pub(super) fn assert_cooling_positive_supply_cp_air_assignment(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
) {
    let lifecycle =
        &runtime["purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP331");
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
            "cp_air_assignment_count",
            "source_site_execution_count",
            "zone_humidity_ratio_read_count",
            "psychrometric_cp_air_evaluation_count",
            "cp_air_assignment_write_count",
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
        &runtime["purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
    }
    assert_eq!(
        lifecycle["positive_guard_false_fallthrough_skip_count"],
        predecessor["active_guard_false_fallthrough_count"]
    );
    assert_eq!(
        lifecycle["cp_air_assignment_count"],
        predecessor["positive_supply_mass_flow_body_entry_count"]
    );

    let assignments = lifecycle["cp_air_assignment_count"]
        .as_u64()
        .expect("CP331 assignment count");
    let false_skips = lifecycle["positive_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP331 false-guard skips");
    assert_eq!(
        expected_unit_off_skips + expected_non_cooling_skips + false_skips + assignments,
        expected_calls
    );
    assert_eq!(lifecycle["source_site_execution_count"], assignments * 3);
    for field in [
        "zone_humidity_ratio_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_write_count",
    ] {
        assert_eq!(lifecycle[field], assignments, "{field}");
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
            "cp_air_assignment_executed",
            "zone_humidity_ratio_read",
            "zone_humidity_ratio",
            "zone_humidity_ratio_ieee_bits",
            "psychrometric_cp_air_evaluated",
            "psychrometric_cp_air_result_j_per_kg_k",
            "psychrometric_cp_air_result_j_per_kg_k_ieee_bits",
            "cp_air_assigned",
            "cp_air_j_per_kg_k",
            "cp_air_j_per_kg_k_ieee_bits",
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
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }
    assert_eq!(
        latest["predecessor_cooling_body_entered"],
        predecessor_latest["cooling_body_entered"]
    );
    assert_eq!(
        latest["predecessor_no_outdoor_air_fallback_entered"],
        predecessor_latest["predecessor_no_outdoor_air_fallback_entered"]
    );
    assert_eq!(
        latest["predecessor_positive_supply_mass_flow_body_entered"],
        predecessor_latest["positive_supply_mass_flow_body_entered"]
    );
    assert_eq!(
        latest["predecessor_active_guard_false_fallthrough"],
        predecessor_latest["active_guard_false_fallthrough"]
    );
    assert_eq!(
        latest["positive_guard_false_fallthrough_skipped"],
        predecessor_latest["active_guard_false_fallthrough"]
    );
    assert_eq!(
        latest["cp_air_assignment_executed"],
        predecessor_latest["positive_supply_mass_flow_body_entered"]
    );

    if latest["cp_air_assignment_executed"] != true {
        assert_eq!(latest["zone_humidity_ratio_read"], false);
        assert!(latest["zone_humidity_ratio"].is_null());
        assert!(latest["zone_humidity_ratio_ieee_bits"].is_null());
        assert_eq!(latest["psychrometric_cp_air_evaluated"], false);
        assert!(latest["psychrometric_cp_air_result_j_per_kg_k"].is_null());
        assert!(latest["psychrometric_cp_air_result_j_per_kg_k_ieee_bits"].is_null());
        assert_eq!(latest["cp_air_assigned"], false);
        assert!(latest["cp_air_j_per_kg_k"].is_null());
        assert!(latest["cp_air_j_per_kg_k_ieee_bits"].is_null());
        return;
    }

    assert_eq!(latest["zone_humidity_ratio_read"], true);
    assert_eq!(latest["psychrometric_cp_air_evaluated"], true);
    assert_eq!(latest["cp_air_assigned"], true);
    let mixed_air = &runtime["purchased_air_calc_cooling_mixed_air_call_lifecycle"]["latest"];
    assert_eq!(
        latest["zone_humidity_ratio_ieee_bits"],
        mixed_air["recirculation_humidity_ratio_ieee_bits"]
    );
    assert_eq!(
        latest["zone_humidity_ratio_ieee_bits"],
        mixed_air["mixed_air_humidity_ratio_ieee_bits"]
    );
    assert_eq!(
        latest["psychrometric_cp_air_result_j_per_kg_k_ieee_bits"],
        latest["cp_air_j_per_kg_k_ieee_bits"]
    );

    let humidity = f64_from_ieee_bits(
        &latest["zone_humidity_ratio_ieee_bits"],
        "CP331 Zone humidity bits",
    );
    let expected_cp_air = 1.004_84e3 + humidity.max(1.0e-5) * 1.858_95e3;
    assert_eq!(
        latest["cp_air_j_per_kg_k_ieee_bits"],
        format!("0x{:016x}", expected_cp_air.to_bits())
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
