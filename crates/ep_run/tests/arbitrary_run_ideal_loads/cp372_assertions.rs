//! CP372 humidifying-setpoint moisture-demand assignment assertions.

use serde_json::{Map, Value, json};

#[path = "cp373_assertions.rs"]
mod cp373_assertions;

const CP371_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle";
const CP372_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle";
const ORDER: [&str; 2] = [
    "read-zone-humidifying-setpoint-moisture-demand",
    "assign-local-zone-humidifying-setpoint-moisture-demand",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp371 = &runtime[CP371_KEY];
    let cp372 = &runtime[CP372_KEY];
    assert_eq!(
        cp372["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2248"
    );
    assert_eq!(
        cp372["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2249"
    );
    assert_eq!(cp372["latest"]["source_order"], json!(ORDER));

    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
    ] {
        assert_eq!(
            cp372[field], cp371[field],
            "CP372 direct counters must retain exact CP371 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "heating_availability_guard_false_fallthrough_count",
            "heating_on_guard_false_fallthrough_count",
        ),
        (
            "humidification_control_guard_false_fallthrough_count",
            "humidification_control_guard_false_fallthrough_count",
        ),
        (
            "dehumidification_control_humidistat_moisture_demand_assignment_count",
            "dehumidification_control_type_humidistat_match_count",
        ),
        (
            "dehumidification_control_none_moisture_demand_assignment_count",
            "dehumidification_control_type_none_match_count",
        ),
        (
            "dehumidification_control_guard_false_fallthrough_count",
            "dehumidification_control_guard_false_fallthrough_count",
        ),
    ] {
        assert_eq!(
            cp372[current], cp371[predecessor],
            "CP372 route partition must be derived only from CP371"
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
    ] {
        assert_eq!(
            cp372["latest"][field], cp371["latest"][field],
            "CP372 must retain route-independent CP371 lineage"
        );
    }
    for (current, predecessor) in [
        (
            "predecessor_dehumidification_control_type_first_read",
            "dehumidification_control_type_first_read",
        ),
        (
            "predecessor_first_dehumidification_control_type",
            "first_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_type_humidistat",
            "dehumidification_control_type_humidistat",
        ),
        (
            "predecessor_dehumidification_control_type_second_read",
            "dehumidification_control_type_second_read",
        ),
        (
            "predecessor_second_dehumidification_control_type",
            "second_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_type_none",
            "dehumidification_control_type_none",
        ),
        (
            "predecessor_dehumidification_control_body_entered",
            "dehumidification_control_body_entered",
        ),
        (
            "predecessor_dehumidification_control_guard_false_fallthrough",
            "dehumidification_control_guard_false_fallthrough",
        ),
    ] {
        assert_eq!(
            cp372["latest"][current], cp371["latest"][predecessor],
            "CP372 must carry the exact CP371 guard result"
        );
    }

    for field in [
        "humidification_moisture_demand_assignment_count",
        "zone_humidifying_setpoint_moisture_demand_read_count",
        "zone_humidifying_setpoint_moisture_demand_assignment_count",
        "source_site_execution_count",
    ] {
        assert_eq!(
            cp372[field], 0,
            "public direct CP372 sites must remain zero"
        );
    }
    assert_eq!(
        cp372["latest"]["humidification_moisture_demand_assignment_executed"],
        false
    );
    assert_eq!(
        cp372["latest"]["zone_humidifying_setpoint_moisture_demand_read"],
        false
    );
    assert_eq!(
        cp372["latest"]["zone_humidifying_setpoint_moisture_demand_assigned"],
        false
    );
    for field in [
        "zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s",
        "resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s",
    ] {
        assert!(cp372["latest"][field].is_null(), "{field}");
        assert!(
            cp372["latest"][format!("{field}_ieee_bits")].is_null(),
            "{field} bits"
        );
    }
    let latest = cp372["latest"].as_object().expect("CP372 latest object");
    for forbidden in [
        "supply_humidity_ratio_for_humidification",
        "supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
    ] {
        assert!(!latest.contains_key(forbidden), "{forbidden}");
    }

    cp373_assertions::assert_direct(runtime, results);
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP372_KEY));
    assert!(
        runtime[CP372_KEY].is_null(),
        "non-direct runtime must not publish CP372 evidence"
    );
    cp373_assertions::assert_non_direct(runtime);
}
