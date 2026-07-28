//! CP330 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2183";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2185";
const SOURCE_ORDER: [&str; 3] = [
    "read-retained-supply-mass-flow-rate",
    "compare-supply-mass-flow-rate-strictly-greater-than-positive-zero",
    "enter-positive-supply-mass-flow-body-if-satisfied",
];

pub(super) fn assert_cooling_supply_mass_flow_positive_guard(
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
        &runtime["purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP330");
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
            "source_site_execution_count",
            "supply_mass_flow_rate_read_count",
            "supply_mass_flow_rate_strictly_positive_comparison_count",
            "positive_supply_mass_flow_body_entry_count",
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
    assert_eq!(
        lifecycle["supply_mass_flow_rate_read_count"],
        expected_cooling_entries
    );
    assert_eq!(
        lifecycle["supply_mass_flow_rate_strictly_positive_comparison_count"],
        expected_cooling_entries
    );
    let positive_entries = lifecycle["positive_supply_mass_flow_body_entry_count"]
        .as_u64()
        .expect("CP330 positive-flow body entry count");
    let false_fallthroughs = lifecycle["active_guard_false_fallthrough_count"]
        .as_u64()
        .expect("CP330 active false-fallthrough count");
    assert_eq!(
        positive_entries + false_fallthroughs,
        expected_cooling_entries
    );
    assert_eq!(
        lifecycle["source_site_execution_count"],
        expected_cooling_entries * 2 + positive_entries
    );

    let predecessor = &runtime["purchased_air_calc_cooling_mixed_air_call_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
    }
    assert_eq!(
        lifecycle["cooling_body_entry_count"],
        predecessor["cooling_call_count"]
    );

    let upstream =
        &runtime["purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle"];
    assert_eq!(
        lifecycle["positive_supply_mass_flow_body_entry_count"],
        upstream["active_guard_false_fallthrough_count"]
    );
    assert_eq!(
        lifecycle["active_guard_false_fallthrough_count"],
        upstream["zero_flow_reset_body_entry_count"]
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
            "predecessor_cooling_call_executed",
            "predecessor_zero_flow_reset_body_entered",
            "predecessor_active_guard_false_fallthrough",
            "predecessor_no_outdoor_air_fallback_entered",
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "supply_mass_flow_rate_read",
            "supply_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_strictly_positive_comparison_evaluated",
            "supply_mass_flow_rate_strictly_positive",
            "positive_supply_mass_flow_body_entered",
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
        "unit_off_skipped",
        "non_cooling_skipped",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }
    assert_eq!(
        latest["predecessor_cooling_call_executed"],
        predecessor_latest["cooling_call_executed"]
    );
    assert_eq!(
        latest["predecessor_zero_flow_reset_body_entered"],
        predecessor_latest["predecessor_zero_flow_reset_body_entered"]
    );
    assert_eq!(
        latest["predecessor_active_guard_false_fallthrough"],
        predecessor_latest["predecessor_active_guard_false_fallthrough"]
    );
    assert_eq!(
        latest["predecessor_no_outdoor_air_fallback_entered"],
        predecessor_latest["no_outdoor_air_fallback_entered"]
    );
    assert_eq!(
        latest["cooling_body_entered"],
        predecessor_latest["cooling_call_executed"]
    );

    if latest["cooling_body_entered"] != true {
        assert_eq!(latest["supply_mass_flow_rate_read"], false);
        assert!(latest["supply_mass_flow_rate_kg_per_s"].is_null());
        assert!(latest["supply_mass_flow_rate_kg_per_s_ieee_bits"].is_null());
        assert_eq!(
            latest["supply_mass_flow_rate_strictly_positive_comparison_evaluated"],
            false
        );
        assert!(latest["supply_mass_flow_rate_strictly_positive"].is_null());
        assert_eq!(latest["positive_supply_mass_flow_body_entered"], false);
        assert_eq!(latest["active_guard_false_fallthrough"], false);
        return;
    }

    assert_eq!(latest["supply_mass_flow_rate_read"], true);
    assert_eq!(
        latest["supply_mass_flow_rate_strictly_positive_comparison_evaluated"],
        true
    );
    assert_serialized_value_and_bits(
        latest,
        "supply_mass_flow_rate_kg_per_s",
        &predecessor_latest["supply_mass_flow_rate_kg_per_s"],
        &predecessor_latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
    );
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        predecessor_latest["child_supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        predecessor_latest["resulting_recirculation_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    let supply = f64_from_ieee_bits(
        &latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        "CP330 retained supply bits",
    );
    let positive = supply > 0.0;
    assert_eq!(latest["supply_mass_flow_rate_strictly_positive"], positive);
    assert_eq!(latest["positive_supply_mass_flow_body_entered"], positive);
    assert_eq!(latest["active_guard_false_fallthrough"], !positive);
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
