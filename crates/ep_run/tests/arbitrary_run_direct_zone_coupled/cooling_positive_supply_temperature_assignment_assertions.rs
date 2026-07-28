//! CP332 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2186";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2187";
const SOURCE_ORDER: [&str; 8] = [
    "read-zone-cooling-setpoint-load",
    "read-local-cp-air-for-denominator-product",
    "read-retained-supply-mass-flow-rate-for-denominator-product",
    "calculate-cp-air-times-supply-mass-flow-rate",
    "calculate-zone-cooling-setpoint-load-divided-by-denominator-product",
    "read-zone-node-temperature",
    "add-zone-node-temperature-to-load-derived-temperature",
    "assign-purchased-air-supply-temperature",
];

pub(super) fn assert_cooling_positive_supply_temperature_assignment(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
) {
    let lifecycle =
        &runtime["purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP332");
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
            "supply_temperature_assignment_count",
            "source_site_execution_count",
            "zone_cooling_setpoint_load_read_count",
            "cp_air_read_count",
            "supply_mass_flow_rate_read_count",
            "cp_air_times_supply_mass_flow_rate_calculation_count",
            "zone_cooling_setpoint_load_over_denominator_calculation_count",
            "zone_node_temperature_read_count",
            "supply_temperature_calculation_count",
            "supply_temperature_assignment_write_count",
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
        &runtime["purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle"];
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
        lifecycle["supply_temperature_assignment_count"],
        predecessor["cp_air_assignment_count"]
    );

    let assignments = lifecycle["supply_temperature_assignment_count"]
        .as_u64()
        .expect("CP332 assignment count");
    let false_skips = lifecycle["positive_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP332 false-guard skips");
    assert_eq!(
        expected_unit_off_skips + expected_non_cooling_skips + false_skips + assignments,
        expected_calls
    );
    assert_eq!(
        lifecycle["source_site_execution_count"],
        assignments * SOURCE_ORDER.len() as u64
    );
    for field in [
        "zone_cooling_setpoint_load_read_count",
        "cp_air_read_count",
        "supply_mass_flow_rate_read_count",
        "cp_air_times_supply_mass_flow_rate_calculation_count",
        "zone_cooling_setpoint_load_over_denominator_calculation_count",
        "zone_node_temperature_read_count",
        "supply_temperature_calculation_count",
        "supply_temperature_assignment_write_count",
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
            "supply_temperature_assignment_executed",
            "zone_cooling_setpoint_load_read",
            "zone_cooling_setpoint_load_w",
            "zone_cooling_setpoint_load_w_ieee_bits",
            "cp_air_read",
            "cp_air_j_per_kg_k",
            "cp_air_j_per_kg_k_ieee_bits",
            "supply_mass_flow_rate_read",
            "supply_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s_ieee_bits",
            "cp_air_times_supply_mass_flow_rate_calculated",
            "cp_air_times_supply_mass_flow_rate_w_per_k",
            "cp_air_times_supply_mass_flow_rate_w_per_k_ieee_bits",
            "zone_cooling_setpoint_load_over_denominator_calculated",
            "zone_cooling_setpoint_load_over_denominator_c",
            "zone_cooling_setpoint_load_over_denominator_c_ieee_bits",
            "zone_node_temperature_read",
            "zone_node_temperature_c",
            "zone_node_temperature_c_ieee_bits",
            "supply_temperature_calculated",
            "calculated_supply_temperature_c",
            "calculated_supply_temperature_c_ieee_bits",
            "supply_temperature_assigned",
            "supply_temperature_c",
            "supply_temperature_c_ieee_bits",
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
        latest["supply_temperature_assignment_executed"],
        predecessor_latest["cp_air_assignment_executed"]
    );

    if latest["supply_temperature_assignment_executed"] != true {
        for field in [
            "zone_cooling_setpoint_load_read",
            "cp_air_read",
            "supply_mass_flow_rate_read",
            "cp_air_times_supply_mass_flow_rate_calculated",
            "zone_cooling_setpoint_load_over_denominator_calculated",
            "zone_node_temperature_read",
            "supply_temperature_calculated",
            "supply_temperature_assigned",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        for field in [
            "zone_cooling_setpoint_load_w",
            "zone_cooling_setpoint_load_w_ieee_bits",
            "cp_air_j_per_kg_k",
            "cp_air_j_per_kg_k_ieee_bits",
            "supply_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s_ieee_bits",
            "cp_air_times_supply_mass_flow_rate_w_per_k",
            "cp_air_times_supply_mass_flow_rate_w_per_k_ieee_bits",
            "zone_cooling_setpoint_load_over_denominator_c",
            "zone_cooling_setpoint_load_over_denominator_c_ieee_bits",
            "zone_node_temperature_c",
            "zone_node_temperature_c_ieee_bits",
            "calculated_supply_temperature_c",
            "calculated_supply_temperature_c_ieee_bits",
            "supply_temperature_c",
            "supply_temperature_c_ieee_bits",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        return;
    }

    for field in [
        "zone_cooling_setpoint_load_read",
        "cp_air_read",
        "supply_mass_flow_rate_read",
        "cp_air_times_supply_mass_flow_rate_calculated",
        "zone_cooling_setpoint_load_over_denominator_calculated",
        "zone_node_temperature_read",
        "supply_temperature_calculated",
        "supply_temperature_assigned",
    ] {
        assert_eq!(latest[field], true, "{field}");
    }

    let entry_latest = &runtime["purchased_air_calc_entry_lifecycle"]["latest"];
    assert_eq!(
        latest["zone_cooling_setpoint_load_w"],
        entry_latest["demand"]["remaining_output_req_to_cool_sp_w"]
    );
    assert_eq!(
        latest["cp_air_j_per_kg_k_ieee_bits"],
        predecessor_latest["cp_air_j_per_kg_k_ieee_bits"]
    );

    let positive_guard_latest =
        &runtime["purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle"]["latest"];
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        positive_guard_latest["supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );

    let sensible_flow_latest =
        &runtime["purchased_air_calc_cooling_sensible_flow_lifecycle"]["latest"];
    assert_eq!(
        latest["zone_node_temperature_c"],
        sensible_flow_latest["zone_temperature_c"]
    );
    let mixed_air_latest =
        &runtime["purchased_air_calc_cooling_mixed_air_call_lifecycle"]["latest"];
    assert_eq!(
        latest["zone_node_temperature_c_ieee_bits"],
        mixed_air_latest["recirculation_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["zone_node_temperature_c_ieee_bits"],
        mixed_air_latest["mixed_air_temperature_c_ieee_bits"]
    );

    let zone_cooling_setpoint_load = f64_from_ieee_bits(
        &latest["zone_cooling_setpoint_load_w_ieee_bits"],
        "CP332 cooling load bits",
    );
    let cp_air = f64_from_ieee_bits(&latest["cp_air_j_per_kg_k_ieee_bits"], "CP332 CpAir bits");
    let supply_mass_flow_rate = f64_from_ieee_bits(
        &latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        "CP332 supply mass-flow bits",
    );
    let zone_node_temperature = f64_from_ieee_bits(
        &latest["zone_node_temperature_c_ieee_bits"],
        "CP332 Zone temperature bits",
    );
    let denominator = cp_air * supply_mass_flow_rate;
    let quotient = zone_cooling_setpoint_load / denominator;
    let supply_temperature = quotient + zone_node_temperature;
    assert_eq!(
        latest["cp_air_times_supply_mass_flow_rate_w_per_k_ieee_bits"],
        format!("0x{:016x}", denominator.to_bits())
    );
    assert_eq!(
        latest["zone_cooling_setpoint_load_over_denominator_c_ieee_bits"],
        format!("0x{:016x}", quotient.to_bits())
    );
    assert_eq!(
        latest["calculated_supply_temperature_c_ieee_bits"],
        format!("0x{:016x}", supply_temperature.to_bits())
    );
    assert_eq!(
        latest["supply_temperature_c_ieee_bits"],
        latest["calculated_supply_temperature_c_ieee_bits"]
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
