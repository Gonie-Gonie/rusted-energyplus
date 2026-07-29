//! CP359 Humidistat moisture-demand assignment and numerical-nonfeed assertions.

use serde_json::{Map, Value, json};

#[path = "cp360_assertions.rs"]
mod cp360_assertions;

const CP358_KEY: &str = "purchased_air_calc_cooling_humidistat_case_entry_lifecycle";
const CP359_KEY: &str =
    "purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-zone-dehumidifying-setpoint-moisture-demand",
    "assign-local-zone-dehumidifying-setpoint-moisture-demand",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp358 = &runtime[CP358_KEY];
    let cp359 = &runtime[CP359_KEY];
    assert_eq!(
        cp359["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2229"
    );
    assert_eq!(
        cp359["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2230"
    );
    assert_eq!(cp359["latest"]["source_order"], json!(ORDER));
    for (cp359_field, cp358_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
            "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_humidistat_case_entered",
            "dehumidification_control_humidistat_case_entered",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
    ] {
        assert_eq!(
            cp359["latest"][cp359_field], cp358["latest"][cp358_field],
            "CP359 must retain immediate CP358 lineage"
        );
    }
    assert_eq!(
        cp359["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp359["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp359["latest"]["dehumidification_control_humidistat_moisture_demand_assignment_executed"],
        false
    );
    assert_eq!(
        cp359["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"],
        false
    );
    assert_eq!(
        cp359["dehumidification_control_none_case_completed_skip_count"],
        cp358["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp359["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"],
        cp358["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"]
    );
    assert_eq!(
        cp359["dehumidification_control_humidistat_moisture_demand_assignment_count"],
        cp358["dehumidification_control_humidistat_case_entry_count"]
    );
    assert_eq!(
        cp359["dehumidification_control_humidistat_moisture_demand_assignment_count"],
        0
    );
    for field in [
        "zone_dehumidifying_setpoint_moisture_demand_read_count",
        "zone_dehumidifying_setpoint_moisture_demand_assignment_count",
        "source_site_execution_count",
    ] {
        assert_eq!(cp359[field], 0, "{field}");
    }
    assert_eq!(
        cp359["latest"]["zone_dehumidifying_setpoint_moisture_demand_read"],
        false
    );
    assert_eq!(
        cp359["latest"]["zone_dehumidifying_setpoint_moisture_demand_assigned"],
        false
    );
    for field in [
        "zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
        "zone_dehumidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
        "assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
        "assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
        "resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
        "resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
    ] {
        assert!(cp359["latest"][field].is_null(), "{field}");
    }
    cp360_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP359_KEY));
    assert!(
        runtime[CP359_KEY].is_null(),
        "non-direct runtime must not publish CP359 evidence"
    );
    cp360_assertions::assert_non_direct(runtime);
}
