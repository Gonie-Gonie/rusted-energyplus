//! CP373 humidification supply-humidity-ratio assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp374_assertions.rs"]
mod cp374_assertions;

const CP372_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle";
const CP373_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle";
const ORDER: [&str; 6] = [
    "read-local-zone-humidifying-setpoint-moisture-demand-for-supply-humidity-ratio-division",
    "read-retained-supply-mass-flow-rate-for-supply-humidity-ratio-division",
    "calculate-zone-humidifying-setpoint-moisture-demand-divided-by-supply-mass-flow-rate",
    "read-zone-node-humidity-ratio-for-humidification-supply-humidity-ratio",
    "add-zone-node-humidity-ratio-to-moisture-demand-derived-supply-humidity-ratio",
    "assign-local-supply-humidity-ratio-for-humidification",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp372 = &runtime[CP372_KEY];
    let cp373 = &runtime[CP373_KEY];
    assert_eq!(
        cp373["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2249"
    );
    assert_eq!(
        cp373["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2250"
    );
    assert_eq!(cp373["latest"]["source_order"], json!(ORDER));

    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "heating_availability_guard_false_fallthrough_count",
        "humidification_control_guard_false_fallthrough_count",
        "dehumidification_control_guard_false_fallthrough_count",
    ] {
        assert_eq!(
            cp373[field], cp372[field],
            "CP373 direct counters must retain exact CP372 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count",
            "dehumidification_control_humidistat_moisture_demand_assignment_count",
        ),
        (
            "dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count",
            "dehumidification_control_none_moisture_demand_assignment_count",
        ),
    ] {
        assert_eq!(
            cp373[current], cp372[predecessor],
            "CP373 active-route counts must derive only from CP372"
        );
    }

    for field in [
        "unit_body_entered",
        "predecessor_cooling_body_entered",
        "predecessor_no_outdoor_air_fallback_entered",
        "predecessor_positive_supply_mass_flow_body_entered",
        "unit_off_skipped",
        "non_cooling_skipped",
        "positive_guard_false_fallthrough_skipped",
        "predecessor_dehumidification_control_type",
        "predecessor_dehumidification_control_none_case_completed_skip",
        "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
        "predecessor_dehumidification_control_humidistat_case_completed_skip",
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
        "predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break",
        "dehumidification_control_none_case_completed_skip",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
        "dehumidification_control_humidistat_case_completed_skip",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
        "predecessor_heating_on_read",
        "predecessor_heating_on",
        "predecessor_cooling_supply_humidity_ratio_humidification_body_entered",
        "predecessor_heating_on_guard_false_fallthrough",
        "predecessor_humidification_control_type_read",
        "predecessor_humidification_control_type",
        "predecessor_humidification_control_type_humidistat",
        "predecessor_humidification_control_body_entered",
        "predecessor_humidification_control_guard_false_fallthrough",
        "predecessor_dehumidification_control_type_first_read",
        "predecessor_first_dehumidification_control_type",
        "predecessor_dehumidification_control_type_humidistat",
        "predecessor_dehumidification_control_type_second_read",
        "predecessor_second_dehumidification_control_type",
        "predecessor_dehumidification_control_type_none",
        "predecessor_dehumidification_control_body_entered",
        "predecessor_dehumidification_control_guard_false_fallthrough",
    ] {
        assert_eq!(
            cp373["latest"][field], cp372["latest"][field],
            "CP373 must retain route-independent CP372 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "predecessor_humidification_moisture_demand_assignment_executed",
            "humidification_moisture_demand_assignment_executed",
        ),
        (
            "predecessor_zone_humidifying_setpoint_moisture_demand_read",
            "zone_humidifying_setpoint_moisture_demand_read",
        ),
        (
            "predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s",
            "zone_humidifying_setpoint_moisture_demand_kg_per_s",
        ),
        (
            "predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
            "zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
        ),
        (
            "predecessor_zone_humidifying_setpoint_moisture_demand_assigned",
            "zone_humidifying_setpoint_moisture_demand_assigned",
        ),
        (
            "predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s",
            "assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        ),
        (
            "predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
            "assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
        ),
        (
            "predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s",
            "resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        ),
        (
            "predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
            "resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s_ieee_bits",
        ),
    ] {
        assert_eq!(
            cp373["latest"][current], cp372["latest"][predecessor],
            "CP373 must carry exact CP372 assignment evidence"
        );
    }

    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count",
        "dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count",
        "zone_humidifying_setpoint_moisture_demand_read_count",
        "supply_mass_flow_rate_read_count",
        "moisture_demand_derived_supply_humidity_ratio_calculation_count",
        "zone_node_humidity_ratio_read_count",
        "supply_humidity_ratio_for_humidification_calculation_count",
        "supply_humidity_ratio_for_humidification_assignment_count",
        "source_site_execution_count",
    ] {
        assert_eq!(cp373[field], 0, "public direct CP373 {field}");
    }
    for field in [
        "dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed",
        "dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed",
        "zone_humidifying_setpoint_moisture_demand_read",
        "supply_mass_flow_rate_read",
        "moisture_demand_derived_supply_humidity_ratio_calculated",
        "zone_node_humidity_ratio_read",
        "supply_humidity_ratio_for_humidification_calculated",
        "supply_humidity_ratio_for_humidification_assigned",
    ] {
        assert_eq!(cp373["latest"][field], false, "{field}");
    }
    for field in [
        "predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "supply_mass_flow_rate_kg_per_s",
        "moisture_demand_derived_supply_humidity_ratio",
        "zone_node_humidity_ratio",
        "calculated_supply_humidity_ratio_for_humidification",
        "assigned_supply_humidity_ratio_for_humidification",
        "resulting_supply_humidity_ratio_for_humidification",
    ] {
        assert!(cp373["latest"][field].is_null(), "{field}");
        assert!(
            cp373["latest"][format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }

    cp374_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP373_KEY));
    assert!(
        runtime[CP373_KEY].is_null(),
        "non-direct runtime must not publish CP373 evidence"
    );
    cp374_assertions::assert_non_direct(runtime);
}
