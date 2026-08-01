//! CP387 constant-SHR case-entry and `CpAir` assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp388_assertions.rs"]
mod cp388_assertions;

const CP386_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch_lifecycle";
const CP387_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle";
const ORDER: [&str; 4] = [
    "enter-purchased-air-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case",
    "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-cp-air",
    "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-cp-air",
    "assign-local-cp-air-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-case",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp386 = &runtime[CP386_KEY];
    let cp387 = &runtime[CP387_KEY];
    assert_eq!(
        cp387["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2273-2277"
    );
    assert_eq!(
        cp387["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2278"
    );
    assert_eq!(cp387["latest"]["source_order"], json!(ORDER));
    assert_eq!(cp387["system"], cp386["system"]);
    assert_eq!(cp387["transition_count"], cp386["transition_count"]);

    let transitions = count(cp387, "transition_count");
    assert_eq!(count(cp387, "inactive_transition_count"), transitions);
    assert_eq!(
        count(
            cp387,
            "dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count",
        ),
        0
    );
    for field in [
        "source_site_execution_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_entry_count",
        "mixed_air_humidity_ratio_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_write_count",
    ] {
        assert_eq!(count(cp387, field), 0, "public direct CP387 {field}");
    }

    let predecessor_counts = cp386["predecessor_route_counts"]
        .as_array()
        .expect("CP386 predecessor route counts");
    let route_counts = cp387["predecessor_route_counts"]
        .as_array()
        .expect("CP387 predecessor route counts");
    assert_eq!(predecessor_counts.len(), 23);
    assert_eq!(route_counts.len(), 30);
    let expected_nonzero = [
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 7),
        (7, 8),
        (8, 9),
        (20, 6),
        (24, 10),
    ];
    for (index, actual) in route_counts.iter().enumerate() {
        let expected = expected_nonzero
            .iter()
            .find_map(|(cp387_index, cp386_index)| {
                (*cp387_index == index).then(|| predecessor_counts[*cp386_index].clone())
            })
            .unwrap_or(Value::from(0));
        assert_eq!(*actual, expected, "CP387 route slot {index}");
    }

    let predecessor = &cp386["latest"];
    let latest = &cp387["latest"];
    for field in inherited_flag_fields() {
        assert_eq!(latest[field], predecessor[field], "CP387 CP386 {field}");
    }
    assert_eq!(
        latest["predecessor_supply_enthalpy_assignment_executed"],
        predecessor["predecessor_supply_enthalpy_assignment_executed"]
    );
    assert_eq!(
        latest["predecessor_dehumidification_control_type_read"],
        predecessor["dehumidification_control_type_read"]
    );
    assert_eq!(
        latest["predecessor_dehumidification_control_type"],
        predecessor["dehumidification_control_type"]
    );
    assert_eq!(
        latest["predecessor_dehumidification_control_switch_dispatched"],
        predecessor["dehumidification_control_switch_dispatched"]
    );
    assert_eq!(
        latest["predecessor_resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        predecessor["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        predecessor["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    for field in [
        "dehumidification_control_constant_sensible_heat_ratio_case_entered",
        "dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed",
        "mixed_air_humidity_ratio_read",
        "psychrometric_cp_air_evaluated",
        "cp_air_assigned",
    ] {
        assert_eq!(latest[field], false, "public direct CP387 {field}");
    }
    for field in [
        "mixed_air_humidity_ratio",
        "mixed_air_humidity_ratio_ieee_bits",
        "psychrometric_cp_air_result_j_per_kg_k",
        "psychrometric_cp_air_result_j_per_kg_k_ieee_bits",
        "cp_air_j_per_kg_k",
        "cp_air_j_per_kg_k_ieee_bits",
    ] {
        assert!(latest[field].is_null(), "public direct CP387 {field}");
    }

    let serialized = cp387.to_string().to_ascii_lowercase();
    for forbidden in [
        "supply_node",
        "load",
        "report",
        "reconciled",
        "numerical_dto",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "CP387 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP387_KEY),
        "CP387 lifecycle must remain outside numerical result state"
    );
    cp388_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP387_KEY));
    assert!(
        runtime[CP387_KEY].is_null(),
        "non-direct runtime must not publish CP387 evidence"
    );
    cp388_assertions::assert_non_direct(runtime);
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
    assert!(count.is_some(), "CP387 {field} count");
    count.unwrap_or_default()
}
