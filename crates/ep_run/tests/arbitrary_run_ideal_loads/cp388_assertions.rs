//! CP388 constant-SHR sensible-output assignment assertions.

use serde_json::{Map, Value, json};

const CP387_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle";
const CP388_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "read-retained-cooling-total-output-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output-first-factor",
    "read-purchased-air-cooling-sensible-heat-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output-second-factor",
    "calculate-cooling-total-output-times-cooling-sensible-heat-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output",
    "assign-local-cooling-sensible-output-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-case",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp387 = &runtime[CP387_KEY];
    let cp388 = &runtime[CP388_KEY];
    assert_eq!(
        cp388["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2278"
    );
    assert_eq!(
        cp388["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2279"
    );
    assert_eq!(cp388["latest"]["source_order"], json!(ORDER));
    assert_eq!(cp388["system"], cp387["system"]);
    assert_eq!(cp388["transition_count"], cp387["transition_count"]);

    let transitions = count(cp388, "transition_count");
    let assignments = count(
        cp388,
        "dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count",
    );
    assert_eq!(count(cp388, "inactive_transition_count"), transitions);
    assert_eq!(
        assignments,
        count(
            cp387,
            "dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count",
        )
    );
    assert_eq!(assignments, 0, "public direct CP388 assignment count");
    assert_eq!(
        count(cp388, "source_site_execution_count"),
        assignments * ORDER.len() as u64
    );
    for field in [
        "cooling_total_output_owned_read_count",
        "cooling_total_output_bit_corroboration_count",
        "cooling_sensible_heat_ratio_read_count",
        "cooling_sensible_output_calculation_count",
        "cooling_sensible_output_assignment_write_count",
    ] {
        assert_eq!(
            count(cp388, field),
            assignments,
            "public direct CP388 {field}"
        );
    }

    let predecessor_counts = cp387["predecessor_route_counts"]
        .as_array()
        .expect("CP387 predecessor route counts");
    let route_counts = cp388["predecessor_route_counts"]
        .as_array()
        .expect("CP388 predecessor route counts");
    assert_eq!(predecessor_counts.len(), 30);
    assert_eq!(route_counts.len(), 30);
    assert_eq!(route_counts, predecessor_counts);

    let predecessor = &cp387["latest"];
    let latest = &cp388["latest"];
    assert_eq!(latest["system"], predecessor["system"]);
    assert_eq!(latest["system"], cp388["system"]);
    assert_eq!(
        latest["parent_call_ordinal"],
        predecessor["parent_call_ordinal"]
    );
    assert_eq!(latest["parent_call_ordinal"], transitions);
    assert_eq!(latest["controlled_zone"], predecessor["controlled_zone"]);
    for field in inherited_flag_fields() {
        assert_eq!(latest[field], predecessor[field], "CP388 CP387 {field}");
    }
    for (current, prior) in [
        (
            "predecessor_supply_enthalpy_assignment_executed",
            "predecessor_supply_enthalpy_assignment_executed",
        ),
        (
            "predecessor_dehumidification_control_type_read",
            "predecessor_dehumidification_control_type_read",
        ),
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_switch_dispatched",
            "predecessor_dehumidification_control_switch_dispatched",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered",
            "dehumidification_control_constant_sensible_heat_ratio_case_entered",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed",
            "dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed",
        ),
        (
            "predecessor_mixed_air_humidity_ratio_read",
            "mixed_air_humidity_ratio_read",
        ),
        (
            "predecessor_psychrometric_cp_air_evaluated",
            "psychrometric_cp_air_evaluated",
        ),
        ("predecessor_cp_air_assigned", "cp_air_assigned"),
    ] {
        assert_eq!(latest[current], predecessor[prior], "CP388 CP387 {current}");
    }
    for (current, prior) in [
        (
            "predecessor_resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
        (
            "predecessor_mixed_air_humidity_ratio_ieee_bits",
            "mixed_air_humidity_ratio_ieee_bits",
        ),
        (
            "predecessor_psychrometric_cp_air_result_j_per_kg_k_ieee_bits",
            "psychrometric_cp_air_result_j_per_kg_k_ieee_bits",
        ),
        (
            "predecessor_cp_air_j_per_kg_k_ieee_bits",
            "cp_air_j_per_kg_k_ieee_bits",
        ),
        (
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
            "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        ),
    ] {
        assert_eq!(latest[current], predecessor[prior], "CP388 CP387 {current}");
    }

    for field in [
        "dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed",
        "cp384_retained_cooling_total_output_owned_read",
        "cp385_cooling_total_output_bit_corroborated",
        "cooling_total_output_read",
        "cooling_sensible_heat_ratio_read",
        "cooling_sensible_output_calculated",
        "cooling_sensible_output_assigned",
    ] {
        assert_eq!(latest[field], false, "public direct CP388 {field}");
    }
    for field in [
        "cooling_total_output_w",
        "cooling_total_output_w_ieee_bits",
        "cooling_sensible_heat_ratio",
        "cooling_sensible_heat_ratio_ieee_bits",
        "calculated_cooling_sensible_output_w",
        "calculated_cooling_sensible_output_w_ieee_bits",
        "cooling_sensible_output_w",
        "cooling_sensible_output_w_ieee_bits",
    ] {
        assert!(latest[field].is_null(), "public direct CP388 {field}");
    }

    let serialized = cp388.to_string().to_ascii_lowercase();
    for forbidden in [
        "supply_node",
        "load",
        "report",
        "reconciled",
        "numerical_dto",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "CP388 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP388_KEY),
        "CP388 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP388_KEY));
    assert!(
        runtime[CP388_KEY].is_null(),
        "non-direct runtime must not publish CP388 evidence"
    );
}

fn inherited_flag_fields() -> [&'static str; 20] {
    [
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "heating_availability_guard_false_fallthrough",
        "humidification_control_guard_false_fallthrough",
        "dehumidification_control_humidistat_maximum_assignment_executed",
        "dehumidification_control_none_maximum_assignment_executed",
        "dehumidification_control_guard_false_fallthrough",
        "predecessor_capacity_limit_guard_evaluated",
        "predecessor_capacity_limit_body_entered",
        "predecessor_active_capacity_limit_guard_false_fallthrough",
        "predecessor_dehumidification_guard_evaluated",
        "predecessor_dehumidification_body_entered",
        "predecessor_dehumidification_guard_false_fallthrough",
        "predecessor_dehumidification_total_output_assignment_executed",
        "predecessor_dehumidification_total_output_capacity_guard_evaluated",
        "predecessor_dehumidification_total_output_capacity_adjustment_body_entered",
        "predecessor_dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_capacity_guard_false_fallthrough",
        "dehumidification_total_output_maximum_capacity_assignment_executed",
    ]
}

fn count(value: &Value, field: &str) -> u64 {
    let count = value[field].as_u64();
    assert!(count.is_some(), "CP388 {field} count");
    count.unwrap_or_default()
}
