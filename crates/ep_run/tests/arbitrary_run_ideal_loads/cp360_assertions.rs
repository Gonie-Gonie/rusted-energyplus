//! CP360 Humidistat supply-humidity-ratio assignment and numerical-nonfeed assertions.

use serde_json::{Map, Value, json};

#[path = "cp361_assertions.rs"]
mod cp361_assertions;

const CP359_KEY: &str =
    "purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle";
const CP360_KEY: &str = "purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle";
const ORDER: [&str; 6] = [
    "read-local-zone-dehumidifying-setpoint-moisture-demand-for-supply-humidity-ratio-division",
    "read-retained-supply-mass-flow-rate-for-supply-humidity-ratio-division",
    "calculate-zone-dehumidifying-setpoint-moisture-demand-divided-by-supply-mass-flow-rate",
    "read-zone-node-humidity-ratio-for-dehumidification-supply-humidity-ratio",
    "add-zone-node-humidity-ratio-to-moisture-demand-derived-supply-humidity-ratio",
    "assign-local-supply-humidity-ratio-for-dehumidification",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp359 = &runtime[CP359_KEY];
    let cp360 = &runtime[CP360_KEY];
    assert_eq!(
        cp360["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2230"
    );
    assert_eq!(
        cp360["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2231"
    );
    assert_eq!(cp360["latest"]["source_order"], json!(ORDER));
    for (cp360_field, cp359_field) in [
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
            "predecessor_dehumidification_control_humidistat_moisture_demand_assignment_executed",
            "dehumidification_control_humidistat_moisture_demand_assignment_executed",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
        (
            "predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
            "resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
        ),
    ] {
        assert_eq!(
            cp360["latest"][cp360_field], cp359["latest"][cp359_field],
            "CP360 must retain immediate CP359 lineage"
        );
    }
    assert_eq!(
        cp360["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp360["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp360["latest"]["dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed"],
        false
    );
    assert_eq!(
        cp360["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"],
        false
    );
    assert_eq!(
        cp360["dehumidification_control_none_case_completed_skip_count"],
        cp359["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp360["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"],
        cp359["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"]
    );
    assert_eq!(
        cp360["dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count"],
        cp359["dehumidification_control_humidistat_moisture_demand_assignment_count"]
    );
    assert_eq!(
        cp360["dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count"],
        0
    );
    for field in [
        "zone_dehumidifying_setpoint_moisture_demand_read_count",
        "supply_mass_flow_rate_read_count",
        "moisture_demand_derived_supply_humidity_ratio_calculation_count",
        "zone_node_humidity_ratio_read_count",
        "supply_humidity_ratio_for_dehumidification_calculation_count",
        "supply_humidity_ratio_for_dehumidification_assignment_count",
        "source_site_execution_count",
    ] {
        assert_eq!(cp360[field], 0, "{field}");
    }
    for field in [
        "zone_dehumidifying_setpoint_moisture_demand_read",
        "supply_mass_flow_rate_read",
        "moisture_demand_derived_supply_humidity_ratio_calculated",
        "zone_node_humidity_ratio_read",
        "supply_humidity_ratio_for_dehumidification_calculated",
        "supply_humidity_ratio_for_dehumidification_assigned",
    ] {
        assert_eq!(cp360["latest"][field], false, "{field}");
    }
    for field in [
        "predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
        "predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
        "zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
        "zone_dehumidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
        "supply_mass_flow_rate_kg_per_s",
        "supply_mass_flow_rate_kg_per_s_ieee_bits",
        "moisture_demand_derived_supply_humidity_ratio",
        "moisture_demand_derived_supply_humidity_ratio_ieee_bits",
        "zone_node_humidity_ratio",
        "zone_node_humidity_ratio_ieee_bits",
        "calculated_supply_humidity_ratio_for_dehumidification",
        "calculated_supply_humidity_ratio_for_dehumidification_ieee_bits",
        "assigned_supply_humidity_ratio_for_dehumidification",
        "assigned_supply_humidity_ratio_for_dehumidification_ieee_bits",
        "resulting_supply_humidity_ratio_for_dehumidification",
        "resulting_supply_humidity_ratio_for_dehumidification_ieee_bits",
    ] {
        assert!(cp360["latest"][field].is_null(), "{field}");
    }
    cp361_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP360_KEY));
    assert!(
        runtime[CP360_KEY].is_null(),
        "non-direct runtime must not publish CP360 evidence"
    );
    cp361_assertions::assert_non_direct(runtime);
}
