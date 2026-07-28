//! CP351 run-summary assertions for the direct IdealLoads integration route.

use serde_json::Value;

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2218";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2219";
const SOURCE_ORDER: [&str; 4] = [
    "read-retained-cooling-sensible-output-for-constant-sensible-heat-ratio-total-output-numerator",
    "read-purchased-air-cooling-sensible-heat-ratio-for-constant-sensible-heat-ratio-total-output-denominator",
    "calculate-cooling-sensible-output-divided-by-cooling-sensible-heat-ratio-for-constant-sensible-heat-ratio-total-output",
    "assign-local-cooling-total-output-for-constant-sensible-heat-ratio-case",
];

pub(super) fn assert_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment(
    runtime: &Value,
) {
    let cp350 = &runtime["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle"];
    let cp351 = &runtime["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle"];

    assert_eq!(cp351["source"], SOURCE);
    assert_eq!(cp351["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(
        cp351["latest"]["source_order"]
            .as_array()
            .expect("CP351 source order"),
        &SOURCE_ORDER
    );
    for (cp351_field, cp350_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed",
            "dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed",
        ),
        (
            "predecessor_dehumidification_control_humidistat_case_selected_skip",
            "dehumidification_control_humidistat_case_selected_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip",
        ),
    ] {
        assert_eq!(
            cp351["latest"][cp351_field], cp350["latest"][cp350_field],
            "CP351 must retain immediate CP350 {cp350_field} lineage"
        );
    }
    assert_eq!(
        cp351["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp351["latest"]["dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed"],
        false
    );
    for field in [
        "cooling_sensible_output_read",
        "cooling_sensible_heat_ratio_read",
        "cooling_total_output_calculated",
        "cooling_total_output_assigned",
    ] {
        assert_eq!(cp351["latest"][field], false, "{field}");
    }
    for field in [
        "cooling_sensible_output_w",
        "cooling_sensible_output_w_ieee_bits",
        "cooling_sensible_heat_ratio",
        "cooling_sensible_heat_ratio_ieee_bits",
        "calculated_cooling_total_output_w",
        "calculated_cooling_total_output_w_ieee_bits",
        "cooling_total_output_w",
        "cooling_total_output_w_ieee_bits",
    ] {
        assert!(cp351["latest"][field].is_null(), "{field}");
    }
    assert_eq!(
        cp351["dehumidification_control_none_case_completed_skip_count"],
        cp350["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp351["dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count"],
        0
    );
    for field in [
        "source_site_execution_count",
        "cooling_sensible_output_read_count",
        "cooling_sensible_heat_ratio_read_count",
        "cooling_total_output_calculation_count",
        "cooling_total_output_assignment_write_count",
    ] {
        assert_eq!(cp351[field], 0, "{field}");
    }
}
