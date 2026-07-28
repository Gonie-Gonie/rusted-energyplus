//! CP329 run-summary assertions shared by direct-Zone integration routes.

use serde_json::Value;

use super::string_array;

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2171-2178";
const CHILD_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2812-2939; bounded no-OA route 2851,2854-2861,2869-2874,2876,2878,2932-2937";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2183";
const SOURCE_ORDER: [&str; 9] = [
    "bind-state-reference",
    "read-purchased-air-number",
    "read-outdoor-air-mass-flow-rate",
    "read-supply-mass-flow-rate",
    "bind-mixed-air-temperature-output-reference",
    "bind-mixed-air-humidity-ratio-output-reference",
    "bind-mixed-air-enthalpy-output-reference",
    "read-operating-mode",
    "call-calc-purch-air-mixed-air",
];
const NO_OA_CHILD_SOURCE_ORDER: [&str; 22] = [
    "bind-purchased-air-alias",
    "copy-outdoor-air-node-number",
    "copy-recirculation-node-number",
    "initialize-recirculation-mass-flow-rate-positive-zero",
    "read-recirculation-temperature",
    "read-recirculation-humidity-ratio",
    "read-recirculation-enthalpy-projection",
    "evaluate-outdoor-air-initialization-guard",
    "assign-outdoor-air-inlet-temperature-positive-zero",
    "assign-outdoor-air-inlet-humidity-ratio-positive-zero",
    "assign-outdoor-air-inlet-enthalpy-positive-zero",
    "assign-outdoor-air-after-heat-recovery-temperature",
    "assign-outdoor-air-after-heat-recovery-humidity-ratio",
    "assign-outdoor-air-after-heat-recovery-enthalpy",
    "assign-heat-recovery-on-false",
    "evaluate-outdoor-air-active-guard-first-operand",
    "assign-recirculation-mass-flow-rate-from-supply",
    "assign-mixed-air-temperature",
    "assign-mixed-air-humidity-ratio",
    "assign-mixed-air-enthalpy-projection",
    "assign-heat-recovery-sensible-output-positive-zero",
    "assign-heat-recovery-latent-output-positive-zero",
];
const POSITIVE_ZERO_BITS: &str = "0x0000000000000000";

pub(super) fn assert_cooling_mixed_air_call(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
    expected_cooling_calls: u64,
) {
    assert_eq!(
        expected_unit_off_skips + expected_non_cooling_skips + expected_cooling_calls,
        expected_calls
    );
    let lifecycle = &runtime["purchased_air_calc_cooling_mixed_air_call_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP329");
    assert_eq!(lifecycle["source"], SOURCE);
    assert_eq!(lifecycle["child_source"], CHILD_SOURCE);
    assert_eq!(lifecycle["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(lifecycle["transition_count"], expected_calls);
    assert_eq!(lifecycle["unit_off_skip_count"], expected_unit_off_skips);
    assert_eq!(
        lifecycle["non_cooling_skip_count"],
        expected_non_cooling_skips
    );
    assert_eq!(lifecycle["cooling_call_count"], expected_cooling_calls);

    let predecessor =
        &runtime["purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
    }
    assert_eq!(
        lifecycle["cooling_call_count"],
        predecessor["cooling_body_entry_count"]
    );
    for (field, factor) in [
        ("caller_source_site_execution_count", 9),
        ("child_source_site_execution_count", 22),
        ("state_reference_bind_count", 1),
        ("purchased_air_number_read_count", 1),
        ("outdoor_air_mass_flow_rate_read_count", 1),
        ("supply_mass_flow_rate_read_count", 1),
        ("mixed_air_output_reference_bind_count", 3),
        ("operating_mode_read_count", 1),
        ("mixed_air_child_call_count", 1),
        ("no_outdoor_air_fallback_count", 1),
        ("recirculation_enthalpy_projection_count", 1),
        ("mixed_air_output_assignment_count", 3),
        ("heat_recovery_output_positive_zero_assignment_count", 2),
    ] {
        assert_eq!(lifecycle[field], expected_cooling_calls * factor, "{field}");
    }

    let latest = &lifecycle["latest"];
    assert_eq!(latest["source"], SOURCE);
    assert_eq!(latest["child_source"], CHILD_SOURCE);
    assert_eq!(latest["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(string_array(&latest["source_order"]), SOURCE_ORDER);
    assert_eq!(
        string_array(&latest["no_oa_child_source_order"]),
        NO_OA_CHILD_SOURCE_ORDER
    );
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
        latest["predecessor_zero_flow_reset_body_entered"],
        predecessor_latest["zero_flow_reset_body_entered"]
    );
    assert_eq!(
        latest["predecessor_active_guard_false_fallthrough"],
        predecessor_latest["active_guard_false_fallthrough"]
    );

    if latest["cooling_call_executed"] != true {
        for field in [
            "state_reference_bound",
            "purchased_air_number_read",
            "outdoor_air_mass_flow_rate_read",
            "supply_mass_flow_rate_read",
            "mixed_air_temperature_output_reference_bound",
            "mixed_air_humidity_ratio_output_reference_bound",
            "mixed_air_enthalpy_output_reference_bound",
            "operating_mode_read",
            "calc_purch_air_mixed_air_called",
            "purchased_air_alias_bound",
            "outdoor_air_node_number_copied",
            "recirculation_node_number_copied",
            "recirculation_enthalpy_projection_read",
            "no_outdoor_air_fallback_entered",
            "mixed_air_temperature_assigned",
            "mixed_air_humidity_ratio_assigned",
            "mixed_air_enthalpy_projection_assigned",
            "heat_recovery_sensible_output_positive_zero_assigned",
            "heat_recovery_latent_output_positive_zero_assigned",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        for field in [
            "outdoor_air_mass_flow_rate_kg_per_s",
            "supply_mass_flow_rate_kg_per_s",
            "operating_mode",
            "recirculation_node",
            "recirculation_temperature_c",
            "recirculation_humidity_ratio",
            "recirculation_enthalpy_projection_j_per_kg",
            "mixed_air_temperature_c",
            "mixed_air_humidity_ratio",
            "mixed_air_enthalpy_projection_j_per_kg",
            "heat_recovery_sensible_output_w",
            "heat_recovery_latent_output_w",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        return;
    }

    for field in [
        "state_reference_bound",
        "purchased_air_number_read",
        "outdoor_air_mass_flow_rate_read",
        "supply_mass_flow_rate_read",
        "mixed_air_temperature_output_reference_bound",
        "mixed_air_humidity_ratio_output_reference_bound",
        "mixed_air_enthalpy_output_reference_bound",
        "operating_mode_read",
        "calc_purch_air_mixed_air_called",
        "purchased_air_alias_bound",
        "outdoor_air_node_number_copied",
        "recirculation_node_number_copied",
        "recirculation_mass_flow_rate_initialized",
        "recirculation_temperature_read",
        "recirculation_humidity_ratio_read",
        "recirculation_enthalpy_projection_read",
        "outdoor_air_initialization_guard_evaluated",
        "heat_recovery_on_false_assigned",
        "outdoor_air_active_guard_first_operand_evaluated",
        "no_outdoor_air_fallback_entered",
        "child_supply_mass_flow_rate_read",
        "recirculation_mass_flow_rate_assigned_from_supply",
        "mixed_air_temperature_assigned",
        "mixed_air_humidity_ratio_assigned",
        "mixed_air_enthalpy_projection_assigned",
        "heat_recovery_sensible_output_positive_zero_assigned",
        "heat_recovery_latent_output_positive_zero_assigned",
    ] {
        assert_eq!(latest[field], true, "{field}");
    }
    assert_eq!(latest["operating_mode"], "Cooling");
    assert_eq!(latest["outdoor_air_enabled"], false);
    assert_eq!(latest["heat_recovery_on"], false);
    assert_eq!(
        latest["outdoor_air_mass_flow_positive_comparison_evaluated"],
        false
    );
    assert!(latest["outdoor_air_node"].is_null());
    assert_eq!(
        latest["outdoor_air_mass_flow_rate_kg_per_s_ieee_bits"],
        POSITIVE_ZERO_BITS
    );
    assert_eq!(
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"],
        predecessor_latest["resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_eq!(
        latest["child_supply_mass_flow_rate_kg_per_s_ieee_bits"],
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_recirculation_mass_flow_rate_kg_per_s_ieee_bits"],
        latest["supply_mass_flow_rate_kg_per_s_ieee_bits"]
    );
    assert_projection_copy(
        latest,
        "recirculation_temperature_c",
        "mixed_air_temperature_c",
    );
    assert_projection_copy(
        latest,
        "recirculation_humidity_ratio",
        "mixed_air_humidity_ratio",
    );
    assert_projection_copy(
        latest,
        "recirculation_enthalpy_projection_j_per_kg",
        "mixed_air_enthalpy_projection_j_per_kg",
    );
    for field in [
        "initial_recirculation_mass_flow_rate_kg_per_s",
        "outdoor_air_inlet_temperature_c",
        "outdoor_air_inlet_humidity_ratio",
        "outdoor_air_inlet_enthalpy_j_per_kg",
        "outdoor_air_after_heat_recovery_temperature_c",
        "outdoor_air_after_heat_recovery_humidity_ratio",
        "outdoor_air_after_heat_recovery_enthalpy_j_per_kg",
        "heat_recovery_sensible_output_w",
        "heat_recovery_latent_output_w",
    ] {
        assert_eq!(
            latest[format!("{field}_ieee_bits")],
            POSITIVE_ZERO_BITS,
            "{field}"
        );
    }
}

fn assert_projection_copy(snapshot: &Value, source: &str, target: &str) {
    assert_eq!(snapshot[target], snapshot[source], "{target}");
    assert_eq!(
        snapshot[format!("{target}_ieee_bits")],
        snapshot[format!("{source}_ieee_bits")],
        "{target}_ieee_bits"
    );
}
