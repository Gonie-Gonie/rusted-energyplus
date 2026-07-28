//! CP327 run-summary assertions shared by the direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2166";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2167";
const THRESHOLD_SOURCE: &str = "EnergyPlus 26.1 DataHVACGlobals.hh:89";
const THRESHOLD_KG_PER_S: f64 = 1.0e-30;
const THRESHOLD_IEEE_BITS: &str = "0x39b4484bfeebc2a0";
const SOURCE_ORDER: [&str; 4] = [
    "read-retained-supply-mass-flow-rate",
    "read-hvac-very-small-mass-flow",
    "compare-supply-mass-flow-rate-less-than-or-equal-to-hvac-very-small-mass-flow",
    "enter-zero-flow-reset-body-if-at-or-below-threshold",
];

pub(super) fn assert_cooling_supply_mass_flow_very_small_guard(
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
        &runtime["purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP327");
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
            "supply_mass_flow_rate_read_count",
            "hvac_very_small_mass_flow_read_count",
            "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count",
            "zero_flow_reset_body_entry_count",
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
    for field in [
        "supply_mass_flow_rate_read_count",
        "hvac_very_small_mass_flow_read_count",
        "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count",
    ] {
        assert_eq!(lifecycle[field], expected_cooling_entries, "{field}");
    }
    let zero_entries = lifecycle["zero_flow_reset_body_entry_count"]
        .as_u64()
        .expect("CP327 zero-flow-reset body count");
    let false_fallthroughs = lifecycle["active_guard_false_fallthrough_count"]
        .as_u64()
        .expect("CP327 active false-fallthrough count");
    assert_eq!(zero_entries + false_fallthroughs, expected_cooling_entries);

    let predecessor = &runtime["purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "cooling_body_entry_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
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
            "predecessor_ems_supply_mass_flow_override_body_skipped",
            "predecessor_ems_disabled_fallthrough",
            "predecessor_supply_mass_flow_limit_body_entered",
            "predecessor_supply_mass_flow_limit_body_skipped",
            "predecessor_supply_mass_flow_limit_active_guard_false_fallthrough",
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "supply_mass_flow_rate_read",
            "supply_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s_ieee_bits",
            "hvac_very_small_mass_flow_read",
            "hvac_very_small_mass_flow_source",
            "hvac_very_small_mass_flow_kg_per_s",
            "hvac_very_small_mass_flow_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated",
            "supply_mass_flow_rate_at_or_below_very_small_mass_flow",
            "zero_flow_reset_body_entered",
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
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }
    assert_eq!(
        latest["predecessor_supply_mass_flow_limit_body_entered"],
        predecessor_latest["supply_mass_flow_limit_body_entered"]
    );
    assert_eq!(
        latest["predecessor_supply_mass_flow_limit_body_skipped"],
        predecessor_latest["body_skipped"]
    );
    assert_eq!(
        latest["predecessor_supply_mass_flow_limit_active_guard_false_fallthrough"],
        predecessor_latest["active_guard_false_fallthrough"]
    );

    let active = latest["cooling_body_entered"] == true;
    if !active {
        for field in [
            "supply_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s_ieee_bits",
            "hvac_very_small_mass_flow_source",
            "hvac_very_small_mass_flow_kg_per_s",
            "hvac_very_small_mass_flow_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_at_or_below_very_small_mass_flow",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        for field in [
            "supply_mass_flow_rate_read",
            "hvac_very_small_mass_flow_read",
            "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated",
            "zero_flow_reset_body_entered",
            "active_guard_false_fallthrough",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        return;
    }

    assert_eq!(latest["supply_mass_flow_rate_read"], true);
    assert_eq!(latest["hvac_very_small_mass_flow_read"], true);
    assert_eq!(
        latest["supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated"],
        true
    );
    let supply_value = &predecessor_latest["resulting_supply_mass_flow_rate_kg_per_s"];
    let supply_bits = &predecessor_latest["resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"];
    assert!(supply_value.is_number(), "active CP326 retained supply");
    let supply = f64_from_ieee_bits(supply_bits, "active CP326 retained supply bits");
    assert_serialized_value_and_bits(
        latest,
        "supply_mass_flow_rate_kg_per_s",
        supply_value,
        supply_bits,
    );
    assert_eq!(latest["hvac_very_small_mass_flow_source"], THRESHOLD_SOURCE);
    let serialized_threshold = latest["hvac_very_small_mass_flow_kg_per_s"]
        .as_f64()
        .expect("serialized HVAC::VerySmallMassFlow");
    assert!(
        (serialized_threshold - THRESHOLD_KG_PER_S).abs() <= f64::EPSILON * THRESHOLD_KG_PER_S,
        "serialized HVAC::VerySmallMassFlow value"
    );
    assert_eq!(
        latest["hvac_very_small_mass_flow_kg_per_s_ieee_bits"], THRESHOLD_IEEE_BITS,
        "authoritative HVAC::VerySmallMassFlow bits"
    );
    let comparison = supply <= THRESHOLD_KG_PER_S;
    assert_eq!(
        latest["supply_mass_flow_rate_at_or_below_very_small_mass_flow"],
        comparison
    );
    assert_eq!(latest["zero_flow_reset_body_entered"], comparison);
    assert_eq!(latest["active_guard_false_fallthrough"], !comparison);
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
            .expect("IEEE bits must use a 0x prefix"),
        16,
    )
    .expect("IEEE bits must be hexadecimal");
    f64::from_bits(bits)
}
