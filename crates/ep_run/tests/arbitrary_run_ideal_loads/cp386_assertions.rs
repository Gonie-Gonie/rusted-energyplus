//! CP386 post-saturation dehumidification-control switch assertions.

use serde_json::{Map, Value, json};

const CP385_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle";
const CP386_KEY: &str = "purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch_lifecycle";
const ORDER: [&str; 2] = [
    "read-purchased-air-dehumidification-control-type",
    "dispatch-dehumidification-control-switch",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp385 = &runtime[CP385_KEY];
    let cp386 = &runtime[CP386_KEY];
    assert_eq!(
        cp386["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2272"
    );
    assert_eq!(
        cp386["first_excluded_lexical_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2273"
    );
    assert_eq!(
        cp386["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2277"
    );
    assert_eq!(cp386["latest"]["source_order"], json!(ORDER));
    assert_eq!(cp386["system"], cp385["system"]);
    assert_eq!(cp386["transition_count"], cp385["transition_count"]);

    let assignments = count(
        cp385,
        "post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count",
    );
    let transitions = count(cp386, "transition_count");
    assert_eq!(
        count(cp386, "inactive_transition_count"),
        transitions - assignments
    );
    for field in [
        "dehumidification_control_switch_count",
        "dehumidification_control_type_read_count",
        "dehumidification_control_switch_dispatch_count",
        "dehumidification_control_none_case_selection_count",
    ] {
        assert_eq!(count(cp386, field), assignments, "CP386 {field}");
    }
    assert_eq!(count(cp386, "source_site_execution_count"), 2 * assignments);
    for field in [
        "dehumidification_control_constant_sensible_heat_ratio_case_selection_count",
        "dehumidification_control_humidistat_case_selection_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_selection_count",
    ] {
        assert_eq!(count(cp386, field), 0, "public direct CP386 {field}");
    }

    let predecessor = &cp385["latest"];
    let latest = &cp386["latest"];
    for field in inherited_flag_fields() {
        assert_eq!(latest[field], predecessor[field], "CP386 CP385 {field}");
    }
    let active = predecessor["supply_enthalpy_assignment_executed"] == true;
    assert_eq!(
        latest["predecessor_supply_enthalpy_assignment_executed"],
        active
    );
    assert_eq!(latest["dehumidification_control_type_read"], active);
    assert_eq!(latest["dehumidification_control_switch_dispatched"], active);
    if active {
        assert_eq!(latest["dehumidification_control_type"], "None");
    } else {
        assert!(latest["dehumidification_control_type"].is_null());
    }
    assert_eq!(
        latest["predecessor_resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        predecessor["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );
    assert_eq!(
        latest["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
        predecessor["resulting_supply_enthalpy_j_per_kg_ieee_bits"]
    );

    let serialized = cp386.to_string().to_ascii_lowercase();
    for forbidden in ["supply_node", "report", "reconciled", "numerical_dto"] {
        assert!(
            !serialized.contains(forbidden),
            "CP386 must not feed {forbidden}"
        );
    }
    assert!(
        !results.to_string().contains(CP386_KEY),
        "CP386 lifecycle must remain outside numerical result state"
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP386_KEY));
    assert!(
        runtime[CP386_KEY].is_null(),
        "non-direct runtime must not publish CP386 evidence"
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
    assert!(count.is_some(), "CP386 {field} count");
    count.unwrap_or_default()
}
