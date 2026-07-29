//! CP352 run-summary assertions for the direct IdealLoads integration route.

use serde_json::Value;

const SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2219";
const FIRST_EXCLUDED_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2221";
const SOURCE_ORDER: [&str; 6] = [
    "read-retained-mixed-air-enthalpy-for-constant-sensible-heat-ratio-supply-enthalpy-difference",
    "read-retained-cooling-total-output-for-constant-sensible-heat-ratio-specific-cooling-output-division",
    "read-retained-supply-mass-flow-rate-for-constant-sensible-heat-ratio-specific-cooling-output-division",
    "calculate-cooling-total-output-divided-by-supply-mass-flow-rate-for-constant-sensible-heat-ratio-supply-enthalpy",
    "calculate-mixed-air-enthalpy-minus-specific-cooling-output-for-constant-sensible-heat-ratio-supply-enthalpy",
    "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-case",
];

pub(super) fn assert_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment(
    runtime: &Value,
) {
    let cp336 =
        &runtime["purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle"];
    let cp351 = &runtime["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle"];
    let cp352 = &runtime["purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle"];

    assert_eq!(cp352["source"], SOURCE);
    assert_eq!(cp352["first_excluded_source"], FIRST_EXCLUDED_SOURCE);
    assert_eq!(
        cp352["latest"]["source_order"]
            .as_array()
            .expect("CP352 source order"),
        &SOURCE_ORDER
    );
    for (cp352_field, cp351_field) in [
        (
            "predecessor_dehumidification_control_type",
            "predecessor_dehumidification_control_type",
        ),
        (
            "predecessor_dehumidification_control_none_case_completed_skip",
            "dehumidification_control_none_case_completed_skip",
        ),
        (
            "predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed",
            "dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed",
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
            cp352["latest"][cp352_field], cp351["latest"][cp351_field],
            "CP352 must retain immediate CP351 {cp351_field} lineage"
        );
    }
    assert_eq!(
        cp352["latest"]["dehumidification_control_none_case_completed_skip"],
        true
    );
    assert_eq!(
        cp352["latest"]["dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed"],
        false
    );
    for field in [
        "mixed_air_enthalpy_read",
        "cooling_total_output_read",
        "supply_mass_flow_rate_read",
        "specific_cooling_output_calculated",
        "supply_enthalpy_calculated",
        "supply_enthalpy_assigned",
    ] {
        assert_eq!(cp352["latest"][field], false, "{field}");
    }
    for field in [
        "mixed_air_enthalpy_j_per_kg",
        "mixed_air_enthalpy_j_per_kg_ieee_bits",
        "cooling_total_output_w",
        "cooling_total_output_w_ieee_bits",
        "supply_mass_flow_rate_kg_per_s",
        "supply_mass_flow_rate_kg_per_s_ieee_bits",
        "specific_cooling_output_j_per_kg",
        "specific_cooling_output_j_per_kg_ieee_bits",
        "calculated_supply_enthalpy_j_per_kg",
        "calculated_supply_enthalpy_j_per_kg_ieee_bits",
        "assigned_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    ] {
        assert!(cp352["latest"][field].is_null(), "{field}");
    }
    assert_eq!(
        cp352["dehumidification_control_none_case_completed_skip_count"],
        cp351["dehumidification_control_none_case_completed_skip_count"]
    );
    assert_eq!(
        cp352["dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_count"],
        0
    );
    for field in [
        "source_site_execution_count",
        "mixed_air_enthalpy_read_count",
        "cooling_total_output_read_count",
        "supply_mass_flow_rate_read_count",
        "specific_cooling_output_calculation_count",
        "supply_enthalpy_calculation_count",
        "supply_enthalpy_assignment_write_count",
    ] {
        assert_eq!(cp352[field], 0, "{field}");
    }

    assert!(
        cp336["latest"]["supply_enthalpy_j_per_kg_ieee_bits"]
            .as_str()
            .is_some(),
        "CP352 complete-null evidence must not replace existing numerical supply enthalpy"
    );
}
