//! CP369 Cooling humidification heating-availability guard assertions.

use serde_json::{Map, Value, json};

const CP368_KEY: &str =
    "purchased_air_calc_cooling_default_supply_humidity_ratio_case_break_lifecycle";
const CP369_KEY: &str = "purchased_air_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle";
const ORDER: [&str; 2] = [
    "read-local-heating-on-for-cooling-humidification-guard",
    "enter-cooling-supply-humidity-ratio-humidification-body-if-heating-on",
];

pub(super) fn assert_direct(runtime: &Value, results: &Value) {
    let cp368 = &runtime[CP368_KEY];
    let cp369 = &runtime[CP369_KEY];
    assert_eq!(
        cp369["source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2245"
    );
    assert_eq!(
        cp369["first_excluded_source"],
        "EnergyPlus 26.1 PurchasedAirManager.cc:2246"
    );
    assert_eq!(cp369["latest"]["source_order"], json!(ORDER));

    for (cp369_field, cp368_field) in [
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
            "predecessor_dehumidification_control_humidistat_case_completed_skip",
            "dehumidification_control_humidistat_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break",
            "dehumidification_control_default_supply_humidity_ratio_case_exited_via_break",
        ),
    ] {
        assert_eq!(
            cp369["latest"][cp369_field], cp368["latest"][cp368_field],
            "CP369 must retain immediate CP368 lineage"
        );
    }
    for field in [
        "dehumidification_control_none_case_completed_skip",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip",
        "dehumidification_control_humidistat_case_completed_skip",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip",
    ] {
        assert_eq!(
            cp369["latest"][field], cp368["latest"][field],
            "CP369 selector routing must stay orthogonal to HeatOn"
        );
    }

    assert_eq!(
        cp369["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp369["latest"]["dehumidification_control_constant_sensible_heat_ratio_case_completed_skip"],
        false
    );
    assert_eq!(
        cp369["latest"]["dehumidification_control_humidistat_case_completed_skip"],
        false
    );
    assert_eq!(
        cp369["latest"]["dehumidification_control_constant_supply_humidity_ratio_case_completed_skip"],
        false
    );
    assert_eq!(cp369["latest"]["heating_on_read"], true);
    assert_eq!(cp369["latest"]["heating_on"], true);
    assert_eq!(
        cp369["latest"]["cooling_supply_humidity_ratio_humidification_body_entered"],
        true
    );
    assert_eq!(cp369["latest"]["heating_on_guard_false_fallthrough"], false);

    for field in [
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_completed_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
    ] {
        assert_eq!(
            cp369[field], cp368[field],
            "CP369 route count must equal CP368 immediate predecessor count"
        );
    }

    let selected = [
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_completed_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
    ]
    .into_iter()
    .map(|field| cp369[field].as_u64().expect("CP369 count"))
    .sum::<u64>();
    let reads = cp369["heating_on_read_count"]
        .as_u64()
        .expect("CP369 HeatOn read count");
    let bodies = cp369["heating_on_body_entry_count"]
        .as_u64()
        .expect("CP369 body count");
    assert_eq!(reads, selected);
    assert_eq!(bodies, reads);
    assert_eq!(cp369["heating_on_guard_false_fallthrough_count"], 0);
    assert_eq!(cp369["source_site_execution_count"], reads + bodies);

    let latest = cp369["latest"].as_object().expect("CP369 latest object");
    for forbidden in [
        "mixed_air_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio",
        "supply_humidity_ratio",
        "humidification_control_type",
        "mixed_air_humidity_ratio_ieee_bits",
        "assigned_supply_humidity_ratio_ieee_bits",
        "resulting_supply_humidity_ratio_ieee_bits",
    ] {
        assert!(!latest.contains_key(forbidden), "{forbidden}");
    }
    super::super::super::super::super::super::super::super::super::super::super::super::assert_numerical_nonfeed(
        runtime, results,
    );
}

pub(super) fn assert_non_direct(runtime: &Map<String, Value>) {
    assert!(runtime.contains_key(CP369_KEY));
    assert!(
        runtime[CP369_KEY].is_null(),
        "non-direct runtime must not publish CP369 evidence"
    );
}
