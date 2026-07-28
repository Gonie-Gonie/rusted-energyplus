//! CP339 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2197";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2198";
const SOURCE_ORDER: [&str; 6] = [
    "read-retained-supply-mass-flow-rate-for-sensible-output-product",
    "read-retained-mixed-air-enthalpy-for-sensible-output-difference",
    "read-retained-supply-enthalpy-for-sensible-output-difference",
    "calculate-mixed-air-enthalpy-minus-supply-enthalpy",
    "calculate-supply-mass-flow-rate-times-enthalpy-difference",
    "assign-local-cooling-sensible-output",
];

pub(super) fn assert_cooling_positive_supply_capacity_limit_sensible_output_assignment(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
) {
    let lifecycle = &runtime["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP339");
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
            "capacity_limit_sensible_output_assignment_count",
            "source_site_execution_count",
            "supply_mass_flow_rate_read_count",
            "mixed_air_enthalpy_read_count",
            "supply_enthalpy_read_count",
            "enthalpy_difference_calculation_count",
            "cooling_sensible_output_calculation_count",
            "cooling_sensible_output_assignment_write_count",
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

    let predecessor = &runtime["purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle"];
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
        lifecycle["capacity_limit_sensible_output_assignment_count"],
        predecessor["capacity_limit_cp_air_assignment_count"]
    );

    let assignments = lifecycle["capacity_limit_sensible_output_assignment_count"]
        .as_u64()
        .expect("CP339 assignment count");
    let positive_skips = lifecycle["positive_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP339 positive-guard skips");
    let capacity_skips = lifecycle["capacity_limit_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP339 capacity-guard skips");
    assert_eq!(
        expected_unit_off_skips
            + expected_non_cooling_skips
            + positive_skips
            + capacity_skips
            + assignments,
        expected_calls
    );
    assert_eq!(lifecycle["source_site_execution_count"], assignments * 6);
    for field in [
        "supply_mass_flow_rate_read_count",
        "mixed_air_enthalpy_read_count",
        "supply_enthalpy_read_count",
        "enthalpy_difference_calculation_count",
        "cooling_sensible_output_calculation_count",
        "cooling_sensible_output_assignment_write_count",
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
            "predecessor_capacity_limit_guard_evaluated",
            "predecessor_capacity_limit_body_entered",
            "predecessor_active_capacity_limit_guard_false_fallthrough",
            "predecessor_capacity_limit_cp_air_assignment_executed",
            "unit_off_skipped",
            "non_cooling_skipped",
            "positive_guard_false_fallthrough_skipped",
            "capacity_limit_guard_false_fallthrough_skipped",
            "capacity_limit_sensible_output_assignment_executed",
            "supply_mass_flow_rate_read",
            "supply_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s_ieee_bits",
            "mixed_air_enthalpy_read",
            "mixed_air_enthalpy_j_per_kg",
            "mixed_air_enthalpy_j_per_kg_ieee_bits",
            "supply_enthalpy_read",
            "supply_enthalpy_j_per_kg",
            "supply_enthalpy_j_per_kg_ieee_bits",
            "enthalpy_difference_calculated",
            "mixed_air_minus_supply_enthalpy_j_per_kg",
            "mixed_air_minus_supply_enthalpy_j_per_kg_ieee_bits",
            "cooling_sensible_output_calculated",
            "calculated_cooling_sensible_output_w",
            "calculated_cooling_sensible_output_w_ieee_bits",
            "cooling_sensible_output_assigned",
            "cooling_sensible_output_w",
            "cooling_sensible_output_w_ieee_bits",
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
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "capacity_limit_guard_false_fallthrough_skipped",
    ] {
        assert_eq!(latest[field], predecessor_latest[field], "{field}");
    }
    assert_eq!(
        latest["predecessor_capacity_limit_cp_air_assignment_executed"],
        predecessor_latest["capacity_limit_cp_air_assignment_executed"]
    );
    assert_eq!(
        latest["capacity_limit_sensible_output_assignment_executed"],
        predecessor_latest["capacity_limit_cp_air_assignment_executed"]
    );

    if latest["capacity_limit_sensible_output_assignment_executed"] != true {
        for flag in [
            "supply_mass_flow_rate_read",
            "mixed_air_enthalpy_read",
            "supply_enthalpy_read",
            "enthalpy_difference_calculated",
            "cooling_sensible_output_calculated",
            "cooling_sensible_output_assigned",
        ] {
            assert_eq!(latest[flag], false, "{flag}");
        }
        for value in [
            "supply_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s_ieee_bits",
            "mixed_air_enthalpy_j_per_kg",
            "mixed_air_enthalpy_j_per_kg_ieee_bits",
            "supply_enthalpy_j_per_kg",
            "supply_enthalpy_j_per_kg_ieee_bits",
            "mixed_air_minus_supply_enthalpy_j_per_kg",
            "mixed_air_minus_supply_enthalpy_j_per_kg_ieee_bits",
            "calculated_cooling_sensible_output_w",
            "calculated_cooling_sensible_output_w_ieee_bits",
            "cooling_sensible_output_w",
            "cooling_sensible_output_w_ieee_bits",
        ] {
            assert!(latest[value].is_null(), "{value}");
        }
        return;
    }

    for flag in [
        "supply_mass_flow_rate_read",
        "mixed_air_enthalpy_read",
        "supply_enthalpy_read",
        "enthalpy_difference_calculated",
        "cooling_sensible_output_calculated",
        "cooling_sensible_output_assigned",
    ] {
        assert_eq!(latest[flag], true, "{flag}");
    }
    let supply_flow =
        &runtime["purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle"]["latest"];
    let mixed_air = &runtime["purchased_air_calc_cooling_mixed_air_call_lifecycle"]["latest"];
    let supply_enthalpy = &runtime["purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle"]
        ["latest"];
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        supply_flow["supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_eq!(
        latest["mixed_air_enthalpy_j_per_kg_ieee_bits"],
        mixed_air["mixed_air_enthalpy_projection_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["supply_enthalpy_j_per_kg_ieee_bits"],
        supply_enthalpy["supply_enthalpy_j_per_kg_ieee_bits"]
    );

    let flow = f64_from_ieee_bits(
        &latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        "CP339 supply-flow bits",
    );
    let mixed = f64_from_ieee_bits(
        &latest["mixed_air_enthalpy_j_per_kg_ieee_bits"],
        "CP339 mixed-air enthalpy bits",
    );
    let supply = f64_from_ieee_bits(
        &latest["supply_enthalpy_j_per_kg_ieee_bits"],
        "CP339 supply enthalpy bits",
    );
    let expected_difference = mixed - supply;
    let expected_output = flow * expected_difference;
    assert_eq!(
        latest["mixed_air_minus_supply_enthalpy_j_per_kg_ieee_bits"],
        format!("0x{:016x}", expected_difference.to_bits())
    );
    assert_eq!(
        latest["calculated_cooling_sensible_output_w_ieee_bits"],
        format!("0x{:016x}", expected_output.to_bits())
    );
    assert_eq!(
        latest["cooling_sensible_output_w_ieee_bits"],
        format!("0x{:016x}", expected_output.to_bits())
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
