//! CP336 run-summary assertions shared by direct-Zone integration routes.

use ep_runtime::psychrometrics::energyplus_psy_h_fn_tdb_w;
use serde_json::Value;

use super::{assert_exact_object_keys, string_array};

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2191";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2195";
const SOURCE_ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-enthalpy",
    "read-purchased-air-supply-humidity-ratio-for-enthalpy",
    "evaluate-psy-h-fn-tdb-w",
    "assign-local-supply-enthalpy",
];

pub(super) fn assert_cooling_positive_supply_enthalpy_assignment(
    runtime: &Value,
    expected_calls: u64,
    expected_unit_off_skips: u64,
    expected_non_cooling_skips: u64,
) {
    let lifecycle =
        &runtime["purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle"];
    assert!(lifecycle.is_object(), "direct runtime must publish CP336");
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
            "supply_enthalpy_assignment_count",
            "source_site_execution_count",
            "supply_temperature_for_enthalpy_read_count",
            "supply_humidity_ratio_for_enthalpy_read_count",
            "psychrometric_supply_enthalpy_evaluation_count",
            "supply_enthalpy_assignment_write_count",
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

    let predecessor = &runtime["purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle"];
    let temperature = &runtime["purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle"];
    for field in [
        "system",
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
    ] {
        assert_eq!(lifecycle[field], predecessor[field], "{field}");
        assert_eq!(lifecycle[field], temperature[field], "{field}");
    }
    assert_eq!(
        lifecycle["supply_enthalpy_assignment_count"],
        predecessor["supply_humidity_ratio_mixed_air_assignment_count"]
    );
    assert_eq!(
        lifecycle["supply_enthalpy_assignment_count"],
        temperature["supply_temperature_mixed_air_limit_count"]
    );

    let executions = lifecycle["supply_enthalpy_assignment_count"]
        .as_u64()
        .expect("CP336 execution count");
    let false_skips = lifecycle["positive_guard_false_fallthrough_skip_count"]
        .as_u64()
        .expect("CP336 false-guard skips");
    assert_eq!(
        expected_unit_off_skips + expected_non_cooling_skips + false_skips + executions,
        expected_calls
    );
    assert_eq!(
        lifecycle["source_site_execution_count"],
        executions * SOURCE_ORDER.len() as u64
    );
    for field in [
        "supply_temperature_for_enthalpy_read_count",
        "supply_humidity_ratio_for_enthalpy_read_count",
        "psychrometric_supply_enthalpy_evaluation_count",
        "supply_enthalpy_assignment_write_count",
    ] {
        assert_eq!(lifecycle[field], executions, "{field}");
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
            "supply_enthalpy_assignment_executed",
            "supply_temperature_for_enthalpy_read",
            "supply_temperature_c",
            "supply_temperature_c_ieee_bits",
            "supply_humidity_ratio_for_enthalpy_read",
            "supply_humidity_ratio",
            "supply_humidity_ratio_ieee_bits",
            "psychrometric_supply_enthalpy_evaluated",
            "psychrometric_supply_enthalpy_result_j_per_kg",
            "psychrometric_supply_enthalpy_result_j_per_kg_ieee_bits",
            "supply_enthalpy_assigned",
            "supply_enthalpy_j_per_kg",
            "supply_enthalpy_j_per_kg_ieee_bits",
        ],
    );
    assert_eq!(latest["source"], SOURCE);
    assert_eq!(latest["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(string_array(&latest["source_order"]), SOURCE_ORDER);
    assert_eq!(latest["system"], lifecycle["system"]);
    assert_eq!(latest["parent_call_ordinal"], expected_calls);

    let predecessor_latest = &predecessor["latest"];
    let temperature_latest = &temperature["latest"];
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
        latest["supply_enthalpy_assignment_executed"],
        predecessor_latest["supply_humidity_ratio_mixed_air_assignment_executed"]
    );

    if latest["supply_enthalpy_assignment_executed"] != true {
        for field in [
            "supply_temperature_for_enthalpy_read",
            "supply_humidity_ratio_for_enthalpy_read",
            "psychrometric_supply_enthalpy_evaluated",
            "supply_enthalpy_assigned",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        for field in [
            "supply_temperature_c",
            "supply_temperature_c_ieee_bits",
            "supply_humidity_ratio",
            "supply_humidity_ratio_ieee_bits",
            "psychrometric_supply_enthalpy_result_j_per_kg",
            "psychrometric_supply_enthalpy_result_j_per_kg_ieee_bits",
            "supply_enthalpy_j_per_kg",
            "supply_enthalpy_j_per_kg_ieee_bits",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        return;
    }

    for field in [
        "supply_temperature_for_enthalpy_read",
        "supply_humidity_ratio_for_enthalpy_read",
        "psychrometric_supply_enthalpy_evaluated",
        "supply_enthalpy_assigned",
    ] {
        assert_eq!(latest[field], true, "{field}");
    }
    assert_eq!(
        latest["supply_temperature_c_ieee_bits"],
        temperature_latest["assigned_supply_temperature_c_ieee_bits"]
    );
    assert_eq!(
        latest["supply_humidity_ratio_ieee_bits"],
        predecessor_latest["assigned_supply_humidity_ratio_ieee_bits"]
    );

    let supply_temperature_c = latest["supply_temperature_c"]
        .as_f64()
        .expect("CP336 supply temperature");
    let supply_humidity_ratio = latest["supply_humidity_ratio"]
        .as_f64()
        .expect("CP336 supply humidity ratio");
    let expected = energyplus_psy_h_fn_tdb_w(supply_temperature_c, supply_humidity_ratio);
    let expected_bits = format!("0x{:016x}", expected.to_bits());
    assert_eq!(
        latest["psychrometric_supply_enthalpy_result_j_per_kg_ieee_bits"],
        expected_bits
    );
    assert_eq!(latest["supply_enthalpy_j_per_kg_ieee_bits"], expected_bits);
    assert_eq!(
        latest["psychrometric_supply_enthalpy_result_j_per_kg"]
            .as_f64()
            .map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        latest["supply_enthalpy_j_per_kg"]
            .as_f64()
            .map(f64::to_bits),
        Some(expected.to_bits())
    );
}
