//! CP322 run-summary assertions shared by the direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2155";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2157";
const SOURCE_ORDER: [&str; 6] = [
    "read-outdoor-air-mass-flow-rate",
    "read-supply-mass-flow-rate-for-cool",
    "read-supply-mass-flow-rate-for-dehumidification",
    "read-supply-mass-flow-rate-for-humidification",
    "apply-source-shaped-five-argument-maximum-with-positive-zero-floor",
    "assign-supply-mass-flow-rate",
];

pub(super) fn assert_cooling_supply_mass_flow_maximum(
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
    let lifecycle = &runtime["purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle"];
    assert!(
        lifecycle.is_object(),
        "direct runtime must publish the CP322 key"
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
            "outdoor_air_mass_flow_rate_read_count",
            "supply_mass_flow_rate_for_cool_read_count",
            "supply_mass_flow_rate_for_dehumidification_read_count",
            "supply_mass_flow_rate_for_humidification_read_count",
            "positive_zero_vs_outdoor_air_comparison_count",
            "cooling_vs_dehumidification_comparison_count",
            "leading_vs_candidate_pair_comparison_count",
            "leading_vs_humidification_comparison_count",
            "maximum_evaluation_count",
            "supply_mass_flow_rate_assignment_count",
            "latest",
        ],
    );
    assert_eq!(lifecycle["source"], SOURCE);
    assert_eq!(lifecycle["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(
        lifecycle["system"],
        runtime["purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle"]["system"]
    );
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
        "outdoor_air_mass_flow_rate_read_count",
        "supply_mass_flow_rate_for_cool_read_count",
        "supply_mass_flow_rate_for_dehumidification_read_count",
        "supply_mass_flow_rate_for_humidification_read_count",
        "positive_zero_vs_outdoor_air_comparison_count",
        "cooling_vs_dehumidification_comparison_count",
        "leading_vs_candidate_pair_comparison_count",
        "leading_vs_humidification_comparison_count",
        "maximum_evaluation_count",
        "supply_mass_flow_rate_assignment_count",
    ] {
        assert_eq!(lifecycle[field], expected_cooling_entries, "{field}");
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
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "outdoor_air_mass_flow_rate_read",
            "outdoor_air_mass_flow_rate_kg_per_s",
            "outdoor_air_mass_flow_rate_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_for_cool_read",
            "supply_mass_flow_rate_for_cool_kg_per_s",
            "supply_mass_flow_rate_for_cool_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_for_dehumidification_read",
            "supply_mass_flow_rate_for_dehumidification_kg_per_s",
            "supply_mass_flow_rate_for_dehumidification_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_for_humidification_read",
            "supply_mass_flow_rate_for_humidification_kg_per_s",
            "supply_mass_flow_rate_for_humidification_kg_per_s_ieee_bits",
            "positive_zero_vs_outdoor_air_comparison_evaluated",
            "positive_zero_less_than_outdoor_air",
            "positive_zero_outdoor_air_winner",
            "positive_zero_outdoor_air_maximum_kg_per_s",
            "positive_zero_outdoor_air_maximum_kg_per_s_ieee_bits",
            "cooling_vs_dehumidification_comparison_evaluated",
            "cooling_less_than_dehumidification",
            "cooling_dehumidification_winner",
            "cooling_dehumidification_maximum_kg_per_s",
            "cooling_dehumidification_maximum_kg_per_s_ieee_bits",
            "leading_vs_candidate_pair_comparison_evaluated",
            "leading_less_than_candidate_pair",
            "leading_candidate_pair_winner",
            "leading_candidate_pair_maximum_kg_per_s",
            "leading_candidate_pair_maximum_kg_per_s_ieee_bits",
            "leading_vs_humidification_comparison_evaluated",
            "leading_less_than_humidification",
            "final_winner",
            "maximum_supply_mass_flow_rate_kg_per_s",
            "maximum_supply_mass_flow_rate_kg_per_s_ieee_bits",
            "supply_mass_flow_rate_assigned",
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

    let predecessor =
        &runtime["purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle"]["latest"];
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
        assert_active_snapshot(runtime, latest, predecessor);
    } else {
        assert_skip_snapshot(latest);
    }
}

fn assert_active_snapshot(runtime: &Value, latest: &Value, predecessor: &Value) {
    for field in [
        "outdoor_air_mass_flow_rate_read",
        "supply_mass_flow_rate_for_cool_read",
        "supply_mass_flow_rate_for_dehumidification_read",
        "supply_mass_flow_rate_for_humidification_read",
        "positive_zero_vs_outdoor_air_comparison_evaluated",
        "cooling_vs_dehumidification_comparison_evaluated",
        "leading_vs_candidate_pair_comparison_evaluated",
        "leading_vs_humidification_comparison_evaluated",
        "supply_mass_flow_rate_assigned",
    ] {
        assert_eq!(latest[field], true, "{field}");
    }

    let outdoor_air = &runtime["purchased_air_calc_minimum_oa_prefix_lifecycle"]["latest"]["working_outdoor_air_mass_flow_rate_kg_per_s"];
    assert_bits_eq(
        &latest["outdoor_air_mass_flow_rate_kg_per_s"],
        outdoor_air,
        "CP311 outdoor-air lineage",
    );
    assert_eq!(
        outdoor_air.as_f64().map(f64::to_bits),
        Some(0.0_f64.to_bits()),
        "the bounded no-OA route must feed CP322 exact positive zero"
    );
    for (maximum_field, predecessor_field) in [
        (
            "supply_mass_flow_rate_for_cool_kg_per_s",
            "resulting_supply_mass_flow_rate_for_cool_kg_per_s",
        ),
        (
            "supply_mass_flow_rate_for_dehumidification_kg_per_s",
            "resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s",
        ),
        (
            "supply_mass_flow_rate_for_humidification_kg_per_s",
            "resulting_supply_mass_flow_rate_for_humidification_kg_per_s",
        ),
    ] {
        assert_bits_eq(
            &latest[maximum_field],
            &predecessor[predecessor_field],
            "CP321 candidate bit-lineage",
        );
    }

    let oa = number(&latest["outdoor_air_mass_flow_rate_kg_per_s"]);
    let cooling = number(&latest["supply_mass_flow_rate_for_cool_kg_per_s"]);
    let dehumidification = number(&latest["supply_mass_flow_rate_for_dehumidification_kg_per_s"]);
    let humidification = number(&latest["supply_mass_flow_rate_for_humidification_kg_per_s"]);
    let floor_oa = source_pair(0.0, "PositiveZeroFloor", oa, "OutdoorAir");
    let cooling_dehumidification =
        source_pair(cooling, "Cooling", dehumidification, "Dehumidification");
    let leading = source_pair(
        floor_oa.0,
        floor_oa.1,
        cooling_dehumidification.0,
        cooling_dehumidification.1,
    );
    let final_pair = source_pair(leading.0, leading.1, humidification, "Humidification");

    assert_eq!(latest["positive_zero_less_than_outdoor_air"], 0.0 < oa);
    assert_eq!(latest["positive_zero_outdoor_air_winner"], floor_oa.1);
    assert_number_bits(
        &latest["positive_zero_outdoor_air_maximum_kg_per_s"],
        floor_oa.0,
    );
    assert_eq!(
        latest["cooling_less_than_dehumidification"],
        cooling < dehumidification
    );
    assert_eq!(
        latest["cooling_dehumidification_winner"],
        cooling_dehumidification.1
    );
    assert_number_bits(
        &latest["cooling_dehumidification_maximum_kg_per_s"],
        cooling_dehumidification.0,
    );
    assert_eq!(
        latest["leading_less_than_candidate_pair"],
        floor_oa.0 < cooling_dehumidification.0
    );
    assert_eq!(latest["leading_candidate_pair_winner"], leading.1);
    assert_number_bits(
        &latest["leading_candidate_pair_maximum_kg_per_s"],
        leading.0,
    );
    assert_eq!(
        latest["leading_less_than_humidification"],
        leading.0 < humidification
    );
    assert_eq!(latest["final_winner"], final_pair.1);
    for field in [
        "maximum_supply_mass_flow_rate_kg_per_s",
        "assigned_supply_mass_flow_rate_kg_per_s",
        "resulting_supply_mass_flow_rate_kg_per_s",
    ] {
        assert_number_bits(&latest[field], final_pair.0);
    }
    for field in [
        "outdoor_air_mass_flow_rate_kg_per_s",
        "supply_mass_flow_rate_for_cool_kg_per_s",
        "supply_mass_flow_rate_for_dehumidification_kg_per_s",
        "supply_mass_flow_rate_for_humidification_kg_per_s",
        "positive_zero_outdoor_air_maximum_kg_per_s",
        "cooling_dehumidification_maximum_kg_per_s",
        "leading_candidate_pair_maximum_kg_per_s",
        "maximum_supply_mass_flow_rate_kg_per_s",
        "assigned_supply_mass_flow_rate_kg_per_s",
        "resulting_supply_mass_flow_rate_kg_per_s",
    ] {
        assert_ieee_bits(latest, field);
    }
}

fn assert_skip_snapshot(latest: &Value) {
    for field in [
        "outdoor_air_mass_flow_rate_read",
        "supply_mass_flow_rate_for_cool_read",
        "supply_mass_flow_rate_for_dehumidification_read",
        "supply_mass_flow_rate_for_humidification_read",
        "positive_zero_vs_outdoor_air_comparison_evaluated",
        "cooling_vs_dehumidification_comparison_evaluated",
        "leading_vs_candidate_pair_comparison_evaluated",
        "leading_vs_humidification_comparison_evaluated",
        "supply_mass_flow_rate_assigned",
    ] {
        assert_eq!(latest[field], false, "{field}");
    }
    for field in [
        "outdoor_air_mass_flow_rate_kg_per_s",
        "supply_mass_flow_rate_for_cool_kg_per_s",
        "supply_mass_flow_rate_for_dehumidification_kg_per_s",
        "supply_mass_flow_rate_for_humidification_kg_per_s",
        "positive_zero_less_than_outdoor_air",
        "positive_zero_outdoor_air_winner",
        "positive_zero_outdoor_air_maximum_kg_per_s",
        "cooling_less_than_dehumidification",
        "cooling_dehumidification_winner",
        "cooling_dehumidification_maximum_kg_per_s",
        "leading_less_than_candidate_pair",
        "leading_candidate_pair_winner",
        "leading_candidate_pair_maximum_kg_per_s",
        "leading_less_than_humidification",
        "final_winner",
        "maximum_supply_mass_flow_rate_kg_per_s",
        "assigned_supply_mass_flow_rate_kg_per_s",
        "resulting_supply_mass_flow_rate_kg_per_s",
    ] {
        assert!(latest[field].is_null(), "{field}");
        assert!(
            latest[format!("{field}_ieee_bits")].is_null(),
            "{field}_ieee_bits"
        );
    }
}

fn source_pair<'a>(
    left: f64,
    left_name: &'a str,
    right: f64,
    right_name: &'a str,
) -> (f64, &'a str) {
    if left < right {
        (right, right_name)
    } else {
        (left, left_name)
    }
}

fn number(value: &Value) -> f64 {
    value.as_f64().expect("active CP322 value must be numeric")
}

fn assert_bits_eq(actual: &Value, expected: &Value, label: &str) {
    assert_eq!(
        actual.as_f64().map(f64::to_bits),
        expected.as_f64().map(f64::to_bits),
        "{label}"
    );
}

fn assert_number_bits(actual: &Value, expected: f64) {
    assert_eq!(actual.as_f64().map(f64::to_bits), Some(expected.to_bits()));
}

fn assert_ieee_bits(snapshot: &Value, field: &str) {
    let expected = snapshot[field]
        .as_f64()
        .map(|value| format!("0x{:016x}", value.to_bits()));
    assert_eq!(
        snapshot[format!("{field}_ieee_bits")].as_str(),
        expected.as_deref(),
        "{field} IEEE bits"
    );
}
