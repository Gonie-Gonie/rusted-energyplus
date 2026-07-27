//! Direct-Zone predictor/PurchasedAir arbitrary-run integration tests.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ep_run::{
    PartialRunPolicy, RunConfig, RunExitCode, RunMode, RunOutputFormat, RunResultState,
    SupportStatus, TraceLevel, TraceSelection, run_arbitrary_idf,
};
use serde_json::Value;

#[allow(dead_code)]
#[path = "arbitrary_run/fixtures.rs"]
mod fixtures;

use fixtures::ONE_DAY_EPW;

const DIRECT_ZONE_COUPLED_RUNTIME_CLASS: &str = "ideal-loads-direct-zone-coupled-compatibility";
const ZONE_DEMAND_SOURCE: &str = "rust-predictor-source-setpoint-thresholds";
const RECIRCULATION_SOURCE: &str = "rust-direct-zone-return-projection";
const RECIRCULATION_NODE: &str = "ZONE ONE RETURN";
const INIT_LIFECYCLE_SOURCE: &str = "rust-persistent-init-purchased-air";
const CALC_ENTRY_LIFECYCLE_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:1967,1971-2022";
const CALC_MINIMUM_OA_PREFIX_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2023-2040";
const CALC_MINIMUM_OA_CHILD_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2762-2810; bounded no-OA route 2781,2783,2785,2806-2809";
const CALC_COOLING_ENTRY_GATE_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2046-2047";
const CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2056";
const CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2056-2057";
const CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2058";
const CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2058-2078";
const CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2082";
const CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE: &str =
    "EnergyPlus 26.1 UtilityRoutines.cc:1146-1194,1293-1379; max-only optional argument";
const CALC_COOLING_ECONOMIZER_GUARD_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2082";
const CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2083";
const CALC_COOLING_ECONOMIZER_CONDITION_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2083-2086";
const CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2089";
const CALC_COOLING_ECONOMIZER_BODY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2089-2101";
const CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2109";
const CALC_COOLING_SENSIBLE_FLOW_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2109-2116";
const CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2119";
const CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER: [&str; 19] = [
    "assign-supply-mass-flow-rate-for-cool-zero",
    "read-cooling-on",
    "enter-cooling-on-body-if-true",
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
    "read-minimum-cooling-supply-air-temperature",
    "read-zone-node-temperature",
    "subtract-zone-temperature-from-minimum-cooling-supply-air-temperature",
    "assign-local-delta-temperature",
    "read-delta-temperature-for-small-temperature-difference-gate",
    "compare-strict-delta-temperature-below-negative-small-temperature-difference",
    "enter-delta-temperature-body-if-satisfied",
    "read-zone-cooling-setpoint-load-after-delta-temperature-match",
    "read-local-cp-air-for-first-division",
    "calculate-zone-cooling-setpoint-load-divided-by-cp-air",
    "read-local-delta-temperature-for-second-division",
    "calculate-first-division-intermediate-divided-by-delta-temperature",
    "assign-supply-mass-flow-rate-for-cool",
];
const CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER: [&str; 37] = [
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
    "read-outdoor-air-node-temperature",
    "read-zone-node-temperature",
    "subtract-zone-temperature-from-outdoor-air-temperature",
    "assign-local-delta-temperature",
    "read-delta-temperature-for-small-temperature-difference-gate",
    "compare-strict-delta-temperature-below-negative-small-temperature-difference",
    "enter-delta-temperature-body-if-satisfied",
    "read-zone-cooling-setpoint-load-after-delta-temperature-match",
    "read-local-cp-air-for-first-division",
    "calculate-zone-cooling-setpoint-load-divided-by-cp-air",
    "read-local-delta-temperature-for-second-division",
    "calculate-first-division-intermediate-divided-by-delta-temperature",
    "assign-initial-supply-mass-flow-rate",
    "read-cooling-limit-for-flow-rate",
    "compare-cooling-limit-equal-to-flow-rate",
    "read-cooling-limit-for-flow-rate-and-capacity-after-short-circuit",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "read-maximum-cooling-air-mass-flow-after-selector-match",
    "compare-strict-maximum-cooling-air-mass-flow-above-zero",
    "enter-maximum-flow-clamp-body-if-satisfied",
    "read-supply-mass-flow-rate-for-inner-maximum",
    "apply-source-shaped-maximum-with-zero",
    "reread-maximum-cooling-air-mass-flow-as-clamp-upper-bound",
    "apply-source-shaped-minimum-with-maximum-cooling-air-mass-flow",
    "assign-clamped-supply-mass-flow-rate",
    "read-resulting-supply-mass-flow-rate",
    "read-current-outdoor-air-mass-flow-rate",
    "compare-strict-supply-mass-flow-above-outdoor-air-mass-flow",
    "enter-economizer-activation-body-if-satisfied",
    "assign-economizer-on-true-after-mass-flow-match",
    "reread-supply-mass-flow-for-outdoor-air-mass-flow-assignment",
    "assign-outdoor-air-mass-flow-from-supply-mass-flow",
    "read-system-time-step",
    "assign-economizer-active-time",
];
const CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER: [&str; 12] = [
    "read-economizer-type-for-differential-dry-bulb",
    "compare-economizer-type-equal-to-differential-dry-bulb",
    "read-outdoor-air-node-temperature-after-dry-bulb-match",
    "read-zone-recirculation-air-node-temperature-after-dry-bulb-match",
    "compare-strict-outdoor-temperature-below-zone-recirculation-temperature",
    "read-economizer-type-for-differential-enthalpy-after-dry-bulb-arm-false",
    "compare-economizer-type-equal-to-differential-enthalpy",
    "read-outdoor-air-node-enthalpy-after-enthalpy-match",
    "read-zone-recirculation-air-node-enthalpy-after-enthalpy-match",
    "compare-strict-outdoor-enthalpy-below-zone-recirculation-enthalpy",
    "select-excluded-line-2089-if-compound-condition-satisfied",
    "select-excluded-line-2109-if-compound-condition-false",
];
const COUPLED_SOURCE_ORDER: [&str; 6] = [
    "predict-system-loads",
    "init-purchased-air",
    "calc-purch-air-loads",
    "update-purchased-air",
    "report-purchased-air",
    "correct-zone-air-temps",
];

const DIRECT_ZONE_COUPLED_EPJSON: &str = r#"{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Building": {"Direct Zone Building": {"terrain": "Suburbs"}},
  "Timestep": {
    "One Hour": {"number_of_timesteps_per_hour": 1}
  },
  "Site:Location": {
    "Direct Zone Site": {
      "latitude": 39.74,
      "longitude": -105.18,
      "elevation": 1609.0
    }
  },
  "Schedule:Constant": {
    "Control Type": {"hourly_value": 4.0},
    "Heating Setpoint": {"hourly_value": 30.0},
    "Cooling Setpoint": {"hourly_value": 35.0}
  },
  "Zone": {
    "Zone One": {
      "volume": 100.0,
      "multiplier": 1
    }
  },
  "ThermostatSetpoint:DualSetpoint": {
    "Dual Setpoints": {
      "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
      "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
    }
  },
  "ZoneControl:Thermostat": {
    "Zone Thermostat": {
      "zone_or_zonelist_name": "Zone One",
      "control_type_schedule_name": "Control Type",
      "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
      "control_1_name": "Dual Setpoints",
      "temperature_difference_between_cutout_and_setpoint": 0.0
    }
  },
  "ZoneHVAC:IdealLoadsAirSystem": {
    "Zone Ideal Loads": {
      "zone_supply_air_node_name": "Zone One Inlet",
      "heating_limit": "NoLimit",
      "cooling_limit": "NoLimit",
      "dehumidification_control_type": "None",
      "humidification_control_type": "None",
      "demand_controlled_ventilation_type": "None",
      "outdoor_air_economizer_type": "NoEconomizer",
      "heat_recovery_type": "None"
    }
  },
  "ZoneHVAC:EquipmentList": {
    "Zone Equipment": {
      "load_distribution_scheme": "SequentialLoad",
      "equipment": [
        {
          "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
          "zone_equipment_name": "Zone Ideal Loads",
          "zone_equipment_cooling_sequence": 1,
          "zone_equipment_heating_or_no_load_sequence": 1
        }
      ]
    }
  },
  "ZoneHVAC:EquipmentConnections": {
    "Zone One Equipment Connections": {
      "zone_name": "Zone One",
      "zone_conditioning_equipment_list_name": "Zone Equipment",
      "zone_air_inlet_node_or_nodelist_name": "Zone One Inlet",
      "zone_air_node_name": "Zone One Air Node",
      "zone_return_air_node_or_nodelist_name": "Zone One Return"
    }
  }
}"#;

#[test]
fn direct_zone_predictor_purchased_air_runs_in_release_order()
-> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("direct-zone-coupled")?;
    let input_path = case_dir.join("direct-zone-coupled.epJSON");
    let weather_path = case_dir.join("weather.epw");
    let output_dir = case_dir.join("out");
    write_text(&input_path, DIRECT_ZONE_COUPLED_EPJSON)?;
    write_text(&weather_path, ONE_DAY_EPW)?;

    let outcome = run_arbitrary_idf(&run_config(
        input_path,
        Some(weather_path),
        output_dir.clone(),
    ))?;

    assert_eq!(outcome.exit_code, RunExitCode::Success);
    assert_eq!(
        outcome.support_status,
        SupportStatus::SupportedCompatibility
    );
    assert_eq!(
        outcome.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "success");
    assert_eq!(
        summary["support"]["runtime_class"],
        DIRECT_ZONE_COUPLED_RUNTIME_CLASS
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        DIRECT_ZONE_COUPLED_RUNTIME_CLASS
    );
    assert_eq!(summary["rust_runtime"]["samples"], 2);
    assert_eq!(
        summary["rust_runtime"]["zone_demand_source"],
        ZONE_DEMAND_SOURCE
    );
    assert_eq!(
        summary["rust_runtime"]["fixture_demand_injection_used"],
        false
    );
    assert_eq!(
        summary["rust_runtime"]["purchased_air_branch"],
        "no_oa_sensible"
    );
    assert_eq!(
        summary["rust_runtime"]["recirculation_node"],
        RECIRCULATION_NODE
    );
    assert_eq!(
        summary["rust_runtime"]["recirculation_state_source"],
        RECIRCULATION_SOURCE
    );
    assert_eq!(
        string_array(&summary["rust_runtime"]["actual_coupled_source_order"]),
        COUPLED_SOURCE_ORDER
    );
    assert_persistent_init_lifecycle(&summary, 2);

    let results = read_json(&output_dir.join("results").join("result-store.json"))?;
    assert_eq!(results["profile"]["sample_count"], 2);
    assert_unique_output_handles(&results);

    let heating_rate = find_series(
        &results,
        "ZONE IDEAL LOADS",
        "Zone Ideal Loads Zone Total Heating Rate",
    );
    let heating_energy = find_series(
        &results,
        "ZONE IDEAL LOADS",
        "Zone Ideal Loads Zone Total Heating Energy",
    );
    let predicted_heating_demand = find_series(
        &results,
        "ZONE ONE",
        "Zone System Predicted Sensible Load to Heating Setpoint Heat Transfer Rate",
    );
    let supply_temperature = find_series(&results, "ZONE ONE INLET", "System Node Temperature");
    for series in [
        heating_rate,
        heating_energy,
        predicted_heating_demand,
        supply_temperature,
    ] {
        assert_eq!(series["samples"], 2);
        assert_finite_endpoints(series);
    }

    let rate_values = endpoint_values(heating_rate);
    let energy_values = endpoint_values(heating_energy);
    assert!(
        rate_values.iter().any(|value| value.abs() > 0.0),
        "the elevated heating setpoint must produce at least one nonzero heating rate"
    );
    for (rate_w, energy_j) in rate_values.into_iter().zip(energy_values) {
        assert_close(energy_j, rate_w * 3_600.0, 1.0e-9);
    }
    assert!(
        endpoint_values(predicted_heating_demand)
            .iter()
            .any(|value| value.abs() > 0.0),
        "the predictor-demand series must retain the forced heating demand"
    );
    Ok(())
}

#[test]
fn all_hard_sized_finite_limit_branches_use_the_live_coupled_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, limit, expected_branch) in [
        ("finite-capacity", "LimitCapacity", "finite_capacity"),
        ("finite-flow", "LimitFlowRate", "finite_flow"),
        (
            "finite-flow-capacity",
            "LimitFlowRateAndCapacity",
            "flow_and_capacity",
        ),
    ] {
        let case_dir = unique_case_dir(name)?;
        let input_path = case_dir.join("direct-zone-finite.epJSON");
        let weather_path = case_dir.join("weather.epw");
        let output_dir = case_dir.join("out");
        let fixture = finite_limit_fixture(limit);
        write_text(&input_path, &fixture)?;
        write_text(&weather_path, ONE_DAY_EPW)?;

        let outcome = run_arbitrary_idf(&run_config(
            input_path,
            Some(weather_path),
            output_dir.clone(),
        ))?;

        assert_eq!(outcome.exit_code, RunExitCode::Success);
        assert_eq!(
            outcome.support_status,
            SupportStatus::SupportedCompatibility
        );
        let summary = read_json(&output_dir.join("run-summary.json"))?;
        assert_eq!(
            summary["support"]["runtime_class"],
            DIRECT_ZONE_COUPLED_RUNTIME_CLASS
        );
        assert_eq!(
            summary["support"]["matched_capability_ids"][0],
            "ideal_loads_finite_limits"
        );
        assert_eq!(
            summary["rust_runtime"]["zone_demand_source"],
            ZONE_DEMAND_SOURCE
        );
        assert_eq!(
            summary["rust_runtime"]["fixture_demand_injection_used"], false,
            "{name}"
        );
        assert_eq!(
            summary["rust_runtime"]["purchased_air_branch"],
            expected_branch
        );
        assert_eq!(
            summary["rust_runtime"]["recirculation_node"],
            RECIRCULATION_NODE
        );
        assert_eq!(
            summary["rust_runtime"]["recirculation_state_source"],
            RECIRCULATION_SOURCE
        );
        assert_eq!(
            string_array(&summary["rust_runtime"]["actual_coupled_source_order"]),
            COUPLED_SOURCE_ORDER
        );
        assert_persistent_init_lifecycle(&summary, 2);

        let results = read_json(&output_dir.join("results").join("result-store.json"))?;
        let heating_rate = find_series(
            &results,
            "ZONE IDEAL LOADS",
            "Zone Ideal Loads Zone Total Heating Rate",
        );
        let heating_energy = find_series(
            &results,
            "ZONE IDEAL LOADS",
            "Zone Ideal Loads Zone Total Heating Energy",
        );
        let predicted_heating = find_series(
            &results,
            "ZONE ONE",
            "Zone System Predicted Sensible Load to Heating Setpoint Heat Transfer Rate",
        );
        let rate_values = endpoint_values(heating_rate);
        let energy_values = endpoint_values(heating_energy);
        let predicted_values = endpoint_values(predicted_heating);
        assert!(rate_values.iter().any(|value| *value > 0.0));
        let mut constrained_positive_demand = false;
        for ((rate_w, energy_j), predicted_w) in rate_values
            .into_iter()
            .zip(energy_values)
            .zip(predicted_values)
        {
            assert_close(energy_j, rate_w * 3_600.0, 1.0e-9);
            if predicted_w > 0.0 {
                assert!(rate_w <= predicted_w);
                constrained_positive_demand |= rate_w < predicted_w;
            } else {
                assert_eq!(rate_w, 0.0);
            }
        }
        assert!(
            constrained_positive_demand,
            "the deliberately undersized {expected_branch} branch must limit positive predicted demand"
        );
    }
    Ok(())
}

fn assert_persistent_init_lifecycle(summary: &Value, expected_calls: u64) {
    let runtime = &summary["rust_runtime"];
    let lifecycle = &runtime["purchased_air_init_lifecycle"];
    assert_eq!(runtime["purchased_air_coupling_call_count"], expected_calls);
    assert_eq!(lifecycle["source"], INIT_LIFECYCLE_SOURCE);
    assert_eq!(lifecycle["flags"]["state_machine_used"], true);
    assert_eq!(lifecycle["flags"]["one_time_checked"], true);
    assert_eq!(lifecycle["flags"]["environment_initialized"], true);
    assert_eq!(
        lifecycle["flags"]["environment_initialization_needed"],
        expected_calls > 1
    );
    assert_eq!(lifecycle["flags"]["sizing_checked"], true);
    assert_eq!(lifecycle["flags"]["equipment_list_checked"], true);
    assert_eq!(lifecycle["flags"]["return_plenum_inactive"], true);
    assert_eq!(lifecycle["module_initialization_count"], 1);
    assert_eq!(lifecycle["equipment_list_check_count"], 1);
    assert_eq!(
        lifecycle["declared_system_order"],
        lifecycle["equipment_list_scan_order"]
    );
    assert_eq!(
        lifecycle["declared_system_order"].as_array().map(Vec::len),
        Some(1)
    );
    assert_eq!(lifecycle["equipment_list_scanned_unit_count"], 1);
    assert_eq!(lifecycle["equipment_list_missing_unit_count"], 0);
    assert_eq!(
        lifecycle["equipment_list_diagnostics"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );
    assert_eq!(lifecycle["equipment_list_scan_ordinal"], 1);
    assert!(lifecycle["first_matching_equipment_list"].is_u64());
    assert_eq!(lifecycle["equipment_list_membership_found"], true);
    assert_eq!(lifecycle["init_call_count"], expected_calls);
    assert_eq!(lifecycle["one_time_initialization_count"], 1);
    assert_eq!(lifecycle["sizing_attempt_count"], 1);
    assert_eq!(lifecycle["sizing_check_count"], 1);
    assert_eq!(
        lifecycle["sizing_outcome"]["route"],
        "direct_hard_sized_no_sizing_run"
    );
    assert_eq!(
        lifecycle["sizing_outcome"]["fields"]
            .as_array()
            .map(Vec::len),
        Some(4)
    );
    assert!(lifecycle["sized_limits"].is_object());
    assert_eq!(lifecycle["environment_initialization_count"], 1);
    assert_eq!(
        lifecycle["environment_rearm_count"],
        u64::from(expected_calls > 1)
    );
    assert!(
        lifecycle["standard_air_density_kg_per_m3"]
            .as_f64()
            .is_some_and(|value| value > 0.0)
    );
    assert!(
        lifecycle["maximum_heating_air_mass_flow_rate_kg_per_s"]
            .as_f64()
            .is_some_and(|value| value >= 0.0)
    );
    assert!(
        lifecycle["maximum_cooling_air_mass_flow_rate_kg_per_s"]
            .as_f64()
            .is_some_and(|value| value >= 0.0)
    );
    assert_eq!(
        lifecycle["supply_temperature_diagnostic_registry"]["registered_recurring_diagnostic_count"],
        0
    );
    assert_eq!(
        lifecycle["supply_temperature_diagnostic_registry"]["event_count"],
        0
    );
    assert_eq!(
        lifecycle["supply_temperature_diagnostic_registry"]["characterized_severe_error_count_increment"],
        0
    );
    assert!(lifecycle["supply_temperature_diagnostic_registry"]["cooling_error_index"].is_null());
    assert!(lifecycle["supply_temperature_diagnostic_registry"]["heating_error_index"].is_null());
    assert_eq!(
        lifecycle["supply_temperature_diagnostic_registry"]["identities"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );

    let calc_lifecycle = &runtime["purchased_air_calc_entry_lifecycle"];
    assert_eq!(calc_lifecycle["source"], CALC_ENTRY_LIFECYCLE_SOURCE);
    assert_eq!(calc_lifecycle["call_count"], expected_calls);
    assert_eq!(calc_lifecycle["reset_count"], expected_calls);
    assert_eq!(calc_lifecycle["demand_read_count"], expected_calls);
    assert_eq!(
        calc_lifecycle["overall_availability_read_count"],
        expected_calls
    );
    assert_eq!(
        calc_lifecycle["heating_availability_read_count"],
        expected_calls
    );
    assert_eq!(
        calc_lifecycle["cooling_availability_read_count"],
        expected_calls
    );
    assert_eq!(
        calc_lifecycle["availability_manager_read_count"],
        expected_calls
    );
    assert_eq!(
        calc_lifecycle["availability_manager_zone_write_count"],
        expected_calls
    );
    assert_eq!(
        calc_lifecycle["availability_status_copy_count"],
        expected_calls
    );
    assert_eq!(calc_lifecycle["availability_status"], "no_action");
    assert_eq!(
        calc_lifecycle["availability_manager_zone"],
        lifecycle["controlled_zone"]
    );
    assert_eq!(calc_lifecycle["force_off_count"], 0);
    assert_eq!(calc_lifecycle["heating_on_count"], expected_calls);
    assert_eq!(calc_lifecycle["cooling_on_count"], expected_calls);
    let latest = &calc_lifecycle["latest"];
    assert_eq!(latest["source"], CALC_ENTRY_LIFECYCLE_SOURCE);
    assert_eq!(latest["call_ordinal"], expected_calls);
    assert_eq!(latest["controlled_zone"], lifecycle["controlled_zone"]);
    assert_eq!(latest["supply_node"], lifecycle["supply_node"]);
    assert_eq!(
        latest["recirculation_node"],
        lifecycle["recirculation_node"]
    );
    assert!(latest["outdoor_air_node"].is_null());
    assert_eq!(latest["reset"]["field_count"], 12);
    assert_eq!(
        latest["reset"]["targets"].as_array().map(Vec::len),
        Some(12)
    );
    assert_eq!(latest["reset"]["all_zero"], true);
    assert_eq!(
        latest["demand"]["sensible_input_kind"],
        "source_setpoint_thresholds"
    );
    assert_eq!(latest["demand"]["zone"], latest["controlled_zone"]);
    assert_eq!(latest["availability_manager_read_site_visited"], true);
    assert_eq!(latest["availability_manager_zone_written"], true);
    assert_eq!(latest["copied_availability_status"], "no_action");
    assert_eq!(latest["heating_availability"], 1.0);
    assert_eq!(latest["cooling_availability"], 1.0);
    assert_eq!(latest["heating_on"], true);
    assert_eq!(latest["cooling_on"], true);
    assert_eq!(latest["unit_body_entered"], latest["unit_on"]);

    let minimum_oa = &runtime["purchased_air_calc_minimum_oa_prefix_lifecycle"];
    assert_eq!(minimum_oa["source"], CALC_MINIMUM_OA_PREFIX_SOURCE);
    assert_eq!(
        minimum_oa["minimum_oa_child_source"],
        CALC_MINIMUM_OA_CHILD_SOURCE
    );
    assert_eq!(minimum_oa["transition_count"], expected_calls);
    assert_eq!(minimum_oa["source_execution_count"], expected_calls);
    assert_eq!(minimum_oa["unit_off_skip_count"], 0);
    assert_eq!(
        minimum_oa["zone_heat_balance_reference_count"],
        expected_calls
    );
    assert_eq!(minimum_oa["minimum_oa_child_call_count"], expected_calls);
    assert_eq!(
        minimum_oa["minimum_oa_child_no_outdoor_air_count"],
        expected_calls
    );
    assert_eq!(
        minimum_oa["retained_minimum_outdoor_air_write_count"],
        expected_calls
    );
    assert_eq!(minimum_oa["ems_override_flag_read_count"], expected_calls);
    assert_eq!(minimum_oa["ems_override_apply_count"], 0);
    assert_eq!(minimum_oa["outdoor_air_flag_read_count"], expected_calls);
    assert_eq!(minimum_oa["outdoor_air_effect_count"], 0);
    assert_eq!(
        minimum_oa["no_outdoor_air_zero_branch_count"],
        expected_calls
    );
    assert_eq!(minimum_oa["psychrometric_call_count"], 0);
    let latest_minimum_oa = &minimum_oa["latest"];
    assert_eq!(latest_minimum_oa["source"], CALC_MINIMUM_OA_PREFIX_SOURCE);
    assert_eq!(
        latest_minimum_oa["minimum_oa_child_source"],
        CALC_MINIMUM_OA_CHILD_SOURCE
    );
    assert_eq!(latest_minimum_oa["parent_call_ordinal"], expected_calls);
    assert_eq!(
        latest_minimum_oa["controlled_zone"],
        lifecycle["controlled_zone"]
    );
    assert_eq!(latest_minimum_oa["unit_body_entered"], true);
    assert_eq!(latest_minimum_oa["zone_heat_balance_reference_bound"], true);
    assert_eq!(latest_minimum_oa["minimum_oa_child_called"], true);
    assert_eq!(latest_minimum_oa["ems_override_enabled"], false);
    assert_eq!(latest_minimum_oa["ems_override_applied"], false);
    assert_eq!(latest_minimum_oa["outdoor_air_enabled"], false);
    assert_eq!(
        latest_minimum_oa["retained_minimum_outdoor_air_mass_flow_rate_kg_per_s"],
        0.0
    );
    assert_eq!(
        latest_minimum_oa["working_outdoor_air_mass_flow_rate_kg_per_s"],
        0.0
    );
    assert_eq!(
        latest_minimum_oa["minimum_outdoor_air_sensible_output_w"],
        0.0
    );
    assert_eq!(
        latest_minimum_oa["minimum_outdoor_air_moisture_output_kg_per_s"],
        0.0
    );

    let cooling_entry = &runtime["purchased_air_calc_cooling_entry_gate_lifecycle"];
    assert_eq!(cooling_entry["source"], CALC_COOLING_ENTRY_GATE_SOURCE);
    assert_eq!(
        cooling_entry["first_excluded_source"],
        CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(cooling_entry["transition_count"], expected_calls);
    assert_eq!(cooling_entry["source_execution_count"], expected_calls);
    assert_eq!(cooling_entry["unit_off_skip_count"], 0);
    assert_eq!(cooling_entry["sensible_comparison_count"], expected_calls);
    assert_eq!(cooling_entry["sensible_comparison_satisfied_count"], 0);
    assert_eq!(cooling_entry["temperature_control_type_read_count"], 0);
    assert_eq!(cooling_entry["single_heat_block_count"], 0);
    assert_eq!(cooling_entry["cooling_body_entry_count"], 0);
    assert_eq!(cooling_entry["operating_mode_assignment_count"], 0);
    assert_eq!(cooling_entry["active_fallthrough_count"], expected_calls);
    let latest_cooling_entry = &cooling_entry["latest"];
    assert_eq!(
        latest_cooling_entry["source"],
        CALC_COOLING_ENTRY_GATE_SOURCE
    );
    assert_eq!(
        latest_cooling_entry["first_excluded_source"],
        CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(latest_cooling_entry["parent_call_ordinal"], expected_calls);
    assert_eq!(
        latest_cooling_entry["controlled_zone"],
        lifecycle["controlled_zone"]
    );
    assert_eq!(
        latest_cooling_entry["minimum_outdoor_air_sensible_output_w"],
        0.0
    );
    assert_eq!(latest_cooling_entry["sensible_comparison_evaluated"], true);
    assert_eq!(latest_cooling_entry["sensible_comparison_satisfied"], false);
    assert_eq!(latest_cooling_entry["temperature_control_type_read"], false);
    assert!(latest_cooling_entry["temperature_control_type"].is_null());
    assert_eq!(latest_cooling_entry["single_heat_blocked"], false);
    assert_eq!(latest_cooling_entry["cooling_body_entered"], false);
    assert!(latest_cooling_entry["assigned_operating_mode"].is_null());

    let cooling_oa_max_flow = &runtime["purchased_air_calc_cooling_oa_max_flow_gate_lifecycle"];
    assert_eq!(
        cooling_oa_max_flow["source"],
        CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
    );
    assert_eq!(
        cooling_oa_max_flow["first_excluded_source"],
        CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(cooling_oa_max_flow["transition_count"], expected_calls);
    assert_eq!(cooling_oa_max_flow["source_execution_count"], 0);
    assert_eq!(cooling_oa_max_flow["unit_off_skip_count"], 0);
    assert_eq!(
        cooling_oa_max_flow["non_cooling_skip_count"],
        expected_calls
    );
    assert_eq!(
        cooling_oa_max_flow["cooling_limit_flow_rate_comparison_count"],
        0
    );
    assert_eq!(
        cooling_oa_max_flow["cooling_limit_flow_rate_and_capacity_comparison_count"],
        0
    );
    assert_eq!(
        cooling_oa_max_flow["outdoor_air_mass_flow_rate_read_count"],
        0
    );
    assert_eq!(
        cooling_oa_max_flow["maximum_cooling_air_mass_flow_rate_read_count"],
        0
    );
    assert_eq!(cooling_oa_max_flow["strict_mass_flow_comparison_count"], 0);
    assert_eq!(
        cooling_oa_max_flow["maximum_cooling_flow_body_entry_count"],
        0
    );
    let latest_cooling_oa_max_flow = &cooling_oa_max_flow["latest"];
    assert_eq!(
        latest_cooling_oa_max_flow["source"],
        CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
    );
    assert_eq!(
        latest_cooling_oa_max_flow["first_excluded_source"],
        CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        latest_cooling_oa_max_flow["parent_call_ordinal"],
        expected_calls
    );
    assert_eq!(
        latest_cooling_oa_max_flow["controlled_zone"],
        lifecycle["controlled_zone"]
    );
    assert_eq!(
        latest_cooling_oa_max_flow["predecessor_cooling_body_entered"],
        false
    );
    assert_eq!(latest_cooling_oa_max_flow["non_cooling_skipped"], true);
    assert_eq!(
        latest_cooling_oa_max_flow["cooling_limit_flow_rate_comparison_evaluated"],
        false
    );
    assert!(latest_cooling_oa_max_flow["cooling_limit_flow_rate_value"].is_null());
    assert_eq!(
        latest_cooling_oa_max_flow["strict_mass_flow_comparison_evaluated"],
        false
    );
    assert_eq!(
        latest_cooling_oa_max_flow["maximum_cooling_flow_body_entered"],
        false
    );
    assert_zero_effect_cooling_oa_max_flow_body(runtime, expected_calls, expected_calls, 0);
    assert_cooling_economizer_guard(runtime, expected_calls, expected_calls, 0);
    assert_cooling_economizer_condition(runtime, expected_calls, expected_calls, 0);
    assert_cooling_economizer_body(runtime, expected_calls, expected_calls, 0);
    assert_cooling_sensible_flow(runtime, expected_calls, expected_calls, 0);
}

fn assert_zero_effect_cooling_oa_max_flow_body(
    runtime: &Value,
    expected_calls: u64,
    expected_non_cooling_skips: u64,
    expected_active_guard_false_fallthroughs: u64,
) {
    let body = &runtime["purchased_air_calc_cooling_oa_max_flow_body_lifecycle"];
    assert!(
        body.is_object(),
        "direct runtime must publish the CP314 key"
    );
    assert_eq!(body["source"], CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE);
    assert_eq!(
        body["first_excluded_source"],
        CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        body["recurring_warning_child_source"],
        CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
    );
    assert_eq!(body["transition_count"], expected_calls);
    assert_eq!(body["body_entry_count"], 0);
    assert_eq!(body["body_skip_count"], expected_calls);
    assert_eq!(body["unit_off_skip_count"], 0);
    assert_eq!(body["non_cooling_skip_count"], expected_non_cooling_skips);
    assert_eq!(
        body["active_guard_false_economizer_fallthrough_count"],
        expected_active_guard_false_fallthroughs
    );
    for field in [
        "outdoor_air_mass_flow_rate_read_count",
        "standard_air_density_read_count",
        "outdoor_air_volume_flow_calculation_count",
        "warning_counter_read_count",
        "outdoor_air_flow_max_cooling_output_error_count",
        "first_warning_branch_count",
        "warning_counter_increment_count",
        "first_warning_call_site_count",
        "maximum_cooling_air_volume_flow_rate_read_count",
        "continue_warning_call_site_count",
        "continue_warning_timestamp_call_site_count",
        "recurring_warning_branch_count",
        "recurring_warning_call_site_count",
        "characterized_recurring_warning_index_allocation_count",
        "characterized_recurring_warning_index_reuse_count",
        "characterized_recurring_warning_occurrence_count",
        "outdoor_air_flow_max_cooling_output_index",
        "characterized_total_warning_error_increment_count",
        "maximum_cooling_air_mass_flow_rate_read_count",
        "outdoor_air_mass_flow_clamp_assignment_count",
    ] {
        assert_eq!(body[field], 0, "{field}");
    }
    assert_eq!(
        body["characterized_recurring_warning_index_allocated"],
        false
    );
    assert!(body["characterized_recurring_warning_report_maximum_m3_per_s"].is_null());

    let latest = &body["latest"];
    assert_eq!(latest["source"], CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE);
    assert_eq!(
        latest["first_excluded_source"],
        CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        latest["recurring_warning_child_source"],
        CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE
    );
    assert_eq!(latest["parent_call_ordinal"], expected_calls);
    assert_eq!(latest["source_order"].as_array().map(Vec::len), Some(17));
    assert_eq!(latest["body_skipped"], true);
    assert_eq!(latest["unit_off_skipped"], false);
    assert_eq!(
        latest["non_cooling_skipped"],
        expected_non_cooling_skips > 0
    );
    assert_eq!(
        latest["active_guard_false_economizer_fallthrough"],
        expected_active_guard_false_fallthroughs > 0
    );
    assert_eq!(
        latest["predecessor_maximum_cooling_flow_body_entered"],
        false
    );
    for field in [
        "outdoor_air_mass_flow_rate_read",
        "standard_air_density_read",
        "outdoor_air_volume_flow_rate_calculated",
        "warning_counter_read",
        "first_warning_branch_entered",
        "warning_counter_incremented",
        "first_warning_call_site_reached",
        "maximum_cooling_air_volume_flow_rate_read",
        "continue_warning_call_site_reached",
        "continue_warning_timestamp_call_site_reached",
        "recurring_warning_branch_entered",
        "recurring_warning_call_site_reached",
        "characterized_recurring_warning_index_allocated_on_call",
        "characterized_recurring_warning_index_reused_on_call",
        "characterized_total_warning_error_incremented",
        "maximum_cooling_air_mass_flow_rate_read",
        "outdoor_air_mass_flow_clamp_assignment_performed",
    ] {
        assert_eq!(latest[field], false, "{field}");
    }
    for field in [
        "outdoor_air_mass_flow_rate_before_clamp_kg_per_s",
        "standard_air_density_kg_per_m3",
        "outdoor_air_volume_flow_rate_m3_per_s",
        "warning_counter_before",
        "first_warning_predicate_satisfied",
        "warning_counter_after",
        "maximum_cooling_air_volume_flow_rate_m3_per_s",
        "recurring_warning_report_maximum_input_m3_per_s",
        "characterized_recurring_warning_index_before",
        "characterized_recurring_warning_index_after",
        "characterized_recurring_warning_occurrence_ordinal",
        "characterized_recurring_warning_report_maximum_m3_per_s",
        "maximum_cooling_air_mass_flow_rate_kg_per_s",
        "outdoor_air_mass_flow_rate_after_clamp_kg_per_s",
    ] {
        assert!(latest[field].is_null(), "{field}");
    }
}

fn assert_cooling_economizer_guard(
    runtime: &Value,
    expected_calls: u64,
    expected_non_cooling_skips: u64,
    expected_guard_evaluations: u64,
) {
    let guard = &runtime["purchased_air_calc_cooling_economizer_guard_lifecycle"];
    assert!(
        guard.is_object(),
        "direct runtime must publish the CP315 key"
    );
    assert_eq!(guard["source"], CALC_COOLING_ECONOMIZER_GUARD_SOURCE);
    assert_eq!(
        guard["first_excluded_source"],
        CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(guard["transition_count"], expected_calls);
    assert_eq!(guard["guard_evaluation_count"], expected_guard_evaluations);
    assert_eq!(guard["unit_off_skip_count"], 0);
    assert_eq!(guard["non_cooling_skip_count"], expected_non_cooling_skips);
    assert_eq!(guard["maximum_cooling_flow_body_sibling_skip_count"], 0);
    assert_eq!(
        guard["economizer_type_read_count"],
        expected_guard_evaluations
    );
    assert_eq!(
        guard["no_economizer_comparison_count"],
        expected_guard_evaluations
    );
    assert_eq!(guard["economizer_body_entry_count"], 0);
    assert_eq!(
        guard["no_economizer_fallthrough_count"],
        expected_guard_evaluations
    );

    let latest = &guard["latest"];
    assert_eq!(latest["source"], CALC_COOLING_ECONOMIZER_GUARD_SOURCE);
    assert_eq!(
        latest["first_excluded_source"],
        CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(latest["parent_call_ordinal"], expected_calls);
    let evaluated = expected_guard_evaluations > 0;
    assert_eq!(latest["economizer_guard_evaluated"], evaluated);
    assert_eq!(latest["economizer_type_read"], evaluated);
    assert_eq!(latest["no_economizer_comparison_evaluated"], evaluated);
    assert_eq!(latest["economizer_body_entered"], false);
    assert_eq!(latest["no_economizer_fallthrough"], evaluated);
    if evaluated {
        assert_eq!(latest["economizer_type"], "NoEconomizer");
        assert_eq!(latest["economizer_not_no_economizer"], false);
        assert_eq!(
            latest["predecessor_active_guard_false_economizer_fallthrough"],
            true
        );
    } else {
        assert!(latest["economizer_type"].is_null());
        assert!(latest["economizer_not_no_economizer"].is_null());
        assert_eq!(latest["non_cooling_skipped"], true);
    }
}

fn assert_cooling_economizer_condition(
    runtime: &Value,
    expected_calls: u64,
    expected_non_cooling_skips: u64,
    expected_outer_false_skips: u64,
) {
    assert!(expected_calls > 0);
    assert_eq!(
        expected_non_cooling_skips + expected_outer_false_skips,
        expected_calls,
        "this helper only accepts homogeneous non-cooling or NoEconomizer runs"
    );
    let latest_non_cooling_skipped = expected_non_cooling_skips == expected_calls;
    let latest_outer_false_skipped = expected_outer_false_skips == expected_calls;
    assert_ne!(latest_non_cooling_skipped, latest_outer_false_skipped);

    let condition = &runtime["purchased_air_calc_cooling_economizer_condition_lifecycle"];
    assert!(
        condition.is_object(),
        "direct runtime must publish the CP316 key"
    );
    assert_eq!(
        condition["source"],
        CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
    );
    assert_eq!(
        condition["first_excluded_source"],
        CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(condition["system"], 0);
    assert_eq!(condition["transition_count"], expected_calls);
    assert_eq!(condition["condition_evaluation_count"], 0);
    assert_eq!(condition["unit_off_skip_count"], 0);
    assert_eq!(
        condition["non_cooling_skip_count"],
        expected_non_cooling_skips
    );
    assert_eq!(condition["maximum_cooling_flow_body_sibling_skip_count"], 0);
    assert_eq!(
        condition["no_economizer_outer_guard_fallthrough_skip_count"],
        expected_outer_false_skips
    );
    for field in [
        "differential_dry_bulb_economizer_type_read_count",
        "differential_dry_bulb_selector_comparison_count",
        "differential_dry_bulb_selector_match_count",
        "outdoor_air_temperature_read_count",
        "recirculation_air_temperature_read_count",
        "dry_bulb_temperature_comparison_count",
        "dry_bulb_temperature_comparison_satisfied_count",
        "differential_enthalpy_economizer_type_read_count",
        "differential_enthalpy_selector_comparison_count",
        "differential_enthalpy_selector_match_count",
        "outdoor_air_enthalpy_read_count",
        "recirculation_air_enthalpy_read_count",
        "enthalpy_comparison_count",
        "enthalpy_comparison_satisfied_count",
        "economizer_calculation_body_entry_count",
        "economizer_condition_fallthrough_count",
    ] {
        assert_eq!(condition[field], 0, "{field}");
    }

    let latest = &condition["latest"];
    assert_eq!(latest["source"], CALC_COOLING_ECONOMIZER_CONDITION_SOURCE);
    assert_eq!(
        latest["first_excluded_source"],
        CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(latest["system"], 0);
    assert_eq!(latest["controlled_zone"], 0);
    assert_eq!(latest["parent_call_ordinal"], expected_calls);
    assert_eq!(
        string_array(&latest["source_order"]),
        CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER
    );
    assert_eq!(latest["unit_body_entered"], true);
    assert_eq!(
        latest["predecessor_cooling_body_entered"],
        latest_outer_false_skipped
    );
    assert_eq!(
        latest["predecessor_maximum_cooling_flow_body_entered"],
        false
    );
    assert_eq!(
        latest["predecessor_active_guard_false_economizer_fallthrough"],
        latest_outer_false_skipped
    );
    assert_eq!(
        latest["predecessor_economizer_guard_evaluated"],
        latest_outer_false_skipped
    );
    assert_eq!(latest["predecessor_economizer_body_entered"], false);
    assert_eq!(
        latest["predecessor_no_economizer_fallthrough"],
        latest_outer_false_skipped
    );
    assert_eq!(latest["economizer_condition_evaluated"], false);
    assert_eq!(latest["economizer_calculation_body_entered"], false);
    assert_eq!(latest["economizer_condition_fallthrough"], false);
    assert_eq!(
        latest["no_economizer_outer_guard_fallthrough_skipped"],
        latest_outer_false_skipped
    );
    assert_eq!(latest["non_cooling_skipped"], latest_non_cooling_skipped);
    for field in [
        "differential_dry_bulb_economizer_type_read",
        "differential_dry_bulb_selector_comparison_evaluated",
        "outdoor_air_temperature_read",
        "recirculation_air_temperature_read",
        "dry_bulb_temperature_comparison_evaluated",
        "differential_enthalpy_economizer_type_read",
        "differential_enthalpy_selector_comparison_evaluated",
        "outdoor_air_enthalpy_read",
        "recirculation_air_enthalpy_read",
        "enthalpy_comparison_evaluated",
    ] {
        assert_eq!(latest[field], false, "{field}");
    }
    for field in [
        "differential_dry_bulb_economizer_type",
        "differential_dry_bulb_selector_matched",
        "outdoor_air_temperature_c",
        "recirculation_air_temperature_c",
        "outdoor_air_temperature_below_recirculation_temperature",
        "differential_enthalpy_economizer_type",
        "differential_enthalpy_selector_matched",
        "outdoor_air_enthalpy_j_per_kg",
        "recirculation_air_enthalpy_j_per_kg",
        "outdoor_air_enthalpy_below_recirculation_enthalpy",
        "economizer_condition_satisfied",
    ] {
        assert!(latest[field].is_null(), "{field}");
    }
}

fn assert_cooling_economizer_body(
    runtime: &Value,
    expected_calls: u64,
    expected_non_cooling_skips: u64,
    expected_outer_false_skips: u64,
) {
    assert!(expected_calls > 0);
    assert_eq!(
        expected_non_cooling_skips + expected_outer_false_skips,
        expected_calls,
        "this helper only accepts homogeneous non-cooling or NoEconomizer runs"
    );
    let latest_non_cooling_skipped = expected_non_cooling_skips == expected_calls;
    let latest_outer_false_skipped = expected_outer_false_skips == expected_calls;
    assert_ne!(latest_non_cooling_skipped, latest_outer_false_skipped);

    let body = &runtime["purchased_air_calc_cooling_economizer_body_lifecycle"];
    assert!(
        body.is_object(),
        "direct runtime must publish the CP317 key"
    );
    assert_eq!(body["source"], CALC_COOLING_ECONOMIZER_BODY_SOURCE);
    assert_eq!(
        body["first_excluded_source"],
        CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(body["system"], 0);
    assert_eq!(body["transition_count"], expected_calls);
    assert_eq!(body["body_execution_count"], 0);
    assert_eq!(body["unit_off_skip_count"], 0);
    assert_eq!(body["non_cooling_skip_count"], expected_non_cooling_skips);
    assert_eq!(body["maximum_cooling_flow_body_sibling_skip_count"], 0);
    assert_eq!(
        body["no_economizer_outer_guard_fallthrough_skip_count"],
        expected_outer_false_skips
    );
    assert_eq!(body["economizer_condition_fallthrough_skip_count"], 0);
    for field in [
        "zone_humidity_ratio_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_count",
        "outdoor_air_temperature_read_count",
        "zone_temperature_read_count",
        "delta_temperature_calculation_count",
        "delta_temperature_assignment_count",
        "delta_temperature_for_gate_read_count",
        "delta_temperature_comparison_count",
        "delta_temperature_comparison_satisfied_count",
        "delta_temperature_body_entry_count",
        "delta_temperature_fallthrough_count",
        "zone_cooling_setpoint_load_read_count",
        "cp_air_for_first_division_read_count",
        "zone_cooling_setpoint_load_over_cp_air_calculation_count",
        "delta_temperature_for_second_division_read_count",
        "supply_mass_flow_rate_calculation_count",
        "initial_supply_mass_flow_rate_assignment_count",
        "cooling_limit_flow_rate_read_count",
        "cooling_limit_flow_rate_comparison_count",
        "cooling_limit_flow_rate_match_count",
        "cooling_limit_flow_rate_and_capacity_read_count",
        "cooling_limit_flow_rate_and_capacity_comparison_count",
        "cooling_limit_flow_rate_and_capacity_match_count",
        "maximum_cooling_air_mass_flow_rate_read_count",
        "maximum_cooling_air_mass_flow_rate_positive_comparison_count",
        "maximum_cooling_air_mass_flow_rate_positive_count",
        "maximum_flow_clamp_body_entry_count",
        "supply_mass_flow_rate_for_clamp_read_count",
        "inner_max_evaluation_count",
        "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count",
        "outer_min_evaluation_count",
        "supply_mass_flow_rate_clamp_count",
        "clamped_supply_mass_flow_rate_assignment_count",
        "resulting_supply_mass_flow_rate_read_count",
        "outdoor_air_mass_flow_rate_read_count",
        "supply_above_outdoor_air_mass_flow_comparison_count",
        "supply_above_outdoor_air_mass_flow_comparison_satisfied_count",
        "economizer_activation_body_entry_count",
        "outdoor_air_mass_flow_comparison_fallthrough_count",
        "economizer_on_assignment_count",
        "supply_mass_flow_rate_for_outdoor_air_assignment_read_count",
        "outdoor_air_mass_flow_rate_assignment_count",
        "system_time_step_read_count",
        "economizer_active_time_assignment_count",
    ] {
        assert_eq!(body[field], 0, "{field}");
    }

    let latest = &body["latest"];
    assert_eq!(latest["source"], CALC_COOLING_ECONOMIZER_BODY_SOURCE);
    assert_eq!(
        latest["first_excluded_source"],
        CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(latest["system"], 0);
    assert_eq!(latest["controlled_zone"], 0);
    assert_eq!(latest["parent_call_ordinal"], expected_calls);
    assert_eq!(
        string_array(&latest["source_order"]),
        CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER
    );
    assert_eq!(latest["unit_body_entered"], true);
    assert_eq!(
        latest["predecessor_cooling_body_entered"],
        latest_outer_false_skipped
    );
    assert_eq!(
        latest["predecessor_maximum_cooling_flow_body_entered"],
        false
    );
    assert_eq!(
        latest["predecessor_active_guard_false_economizer_fallthrough"],
        latest_outer_false_skipped
    );
    assert_eq!(
        latest["predecessor_economizer_guard_evaluated"],
        latest_outer_false_skipped
    );
    assert_eq!(latest["predecessor_economizer_body_entered"], false);
    assert_eq!(
        latest["predecessor_no_economizer_fallthrough"],
        latest_outer_false_skipped
    );
    assert_eq!(latest["predecessor_economizer_condition_evaluated"], false);
    assert!(latest["predecessor_economizer_condition_satisfied"].is_null());
    assert_eq!(
        latest["predecessor_economizer_calculation_body_entered"],
        false
    );
    assert_eq!(latest["non_cooling_skipped"], latest_non_cooling_skipped);
    assert_eq!(
        latest["no_economizer_outer_guard_fallthrough_skipped"],
        latest_outer_false_skipped
    );
    assert_eq!(latest["economizer_condition_fallthrough_skipped"], false);
    assert_eq!(latest["economizer_calculation_body_executed"], false);
    for field in [
        "zone_humidity_ratio_read",
        "psychrometric_cp_air_evaluated",
        "cp_air_assigned",
        "outdoor_air_temperature_read",
        "zone_temperature_read",
        "delta_temperature_calculated",
        "delta_temperature_assigned",
        "delta_temperature_for_gate_read",
        "delta_temperature_comparison_evaluated",
        "delta_temperature_body_entered",
        "zone_cooling_setpoint_load_read",
        "cp_air_for_first_division_read",
        "zone_cooling_setpoint_load_over_cp_air_calculated",
        "delta_temperature_for_second_division_read",
        "supply_mass_flow_rate_calculated",
        "initial_supply_mass_flow_rate_assigned",
        "cooling_limit_flow_rate_comparison_evaluated",
        "cooling_limit_flow_rate_read",
        "cooling_limit_flow_rate_and_capacity_comparison_evaluated",
        "cooling_limit_flow_rate_and_capacity_read",
        "maximum_cooling_air_mass_flow_rate_read",
        "maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated",
        "maximum_flow_clamp_body_entered",
        "supply_mass_flow_rate_clamped",
        "supply_mass_flow_rate_for_clamp_read",
        "inner_max_evaluated",
        "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read",
        "outer_min_evaluated",
        "clamped_supply_mass_flow_rate_assigned",
        "resulting_supply_mass_flow_rate_read",
        "outdoor_air_mass_flow_rate_read",
        "supply_above_outdoor_air_mass_flow_comparison_evaluated",
        "economizer_activation_body_entered",
        "economizer_on_assigned",
        "supply_mass_flow_rate_for_outdoor_air_assignment_read",
        "outdoor_air_mass_flow_rate_assigned",
        "system_time_step_read",
        "economizer_active_time_assigned",
    ] {
        assert_eq!(latest[field], false, "{field}");
    }
    for field in [
        "zone_humidity_ratio",
        "psychrometric_cp_air_result_j_per_kg_k",
        "cp_air_j_per_kg_k",
        "outdoor_air_temperature_c",
        "zone_temperature_c",
        "delta_temperature_c",
        "assigned_delta_temperature_c",
        "delta_temperature_for_gate_c",
        "delta_temperature_below_negative_small_temp_diff",
        "zone_cooling_setpoint_load_w",
        "cp_air_for_first_division_j_per_kg_k",
        "zone_cooling_setpoint_load_over_cp_air_kg_k_per_s",
        "delta_temperature_for_second_division_c",
        "calculated_supply_mass_flow_rate_kg_per_s",
        "initial_supply_mass_flow_rate_kg_per_s",
        "cooling_limit_flow_rate_value",
        "cooling_limit_flow_rate_comparison_satisfied",
        "cooling_limit_flow_rate_and_capacity_value",
        "cooling_limit_flow_rate_and_capacity_comparison_satisfied",
        "cooling_flow_limit_active",
        "maximum_cooling_air_mass_flow_rate_kg_per_s",
        "maximum_cooling_air_mass_flow_rate_positive",
        "supply_mass_flow_rate_for_clamp_kg_per_s",
        "nonnegative_supply_mass_flow_rate_kg_per_s",
        "maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s",
        "clamped_supply_mass_flow_rate_kg_per_s",
        "resulting_supply_mass_flow_rate_kg_per_s",
        "outdoor_air_mass_flow_rate_kg_per_s",
        "supply_mass_flow_above_outdoor_air_mass_flow",
        "economizer_on",
        "supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s",
        "assigned_outdoor_air_mass_flow_rate_kg_per_s",
        "system_time_step_hours",
        "assigned_economizer_active_time_hours",
    ] {
        assert!(latest[field].is_null(), "{field}");
    }
}

fn assert_cooling_sensible_flow(
    runtime: &Value,
    expected_calls: u64,
    expected_non_cooling_skips: u64,
    expected_cooling_entries: u64,
) {
    assert!(expected_calls > 0);
    assert_eq!(
        expected_non_cooling_skips + expected_cooling_entries,
        expected_calls
    );
    let flow = &runtime["purchased_air_calc_cooling_sensible_flow_lifecycle"];
    assert!(
        flow.is_object(),
        "direct runtime must publish the CP318 key"
    );
    assert_exact_object_keys(
        flow,
        &[
            "source",
            "first_excluded_source",
            "system",
            "transition_count",
            "cooling_body_entry_count",
            "unit_off_skip_count",
            "non_cooling_skip_count",
            "supply_mass_flow_rate_for_cool_reset_assignment_count",
            "cooling_on_read_count",
            "cooling_on_body_entry_count",
            "cooling_on_fallthrough_count",
            "zone_humidity_ratio_read_count",
            "psychrometric_cp_air_evaluation_count",
            "cp_air_assignment_count",
            "minimum_cooling_supply_air_temperature_read_count",
            "zone_temperature_read_count",
            "delta_temperature_calculation_count",
            "delta_temperature_assignment_count",
            "delta_temperature_for_gate_read_count",
            "delta_temperature_comparison_count",
            "delta_temperature_comparison_satisfied_count",
            "delta_temperature_body_entry_count",
            "delta_temperature_fallthrough_count",
            "zone_cooling_setpoint_load_read_count",
            "cp_air_for_first_division_read_count",
            "zone_cooling_setpoint_load_over_cp_air_calculation_count",
            "delta_temperature_for_second_division_read_count",
            "supply_mass_flow_rate_for_cool_calculation_count",
            "supply_mass_flow_rate_for_cool_assignment_count",
            "latest",
        ],
    );
    assert_eq!(flow["source"], CALC_COOLING_SENSIBLE_FLOW_SOURCE);
    assert_eq!(
        flow["first_excluded_source"],
        CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(flow["system"], 0);
    assert_eq!(flow["transition_count"], expected_calls);
    assert_eq!(flow["cooling_body_entry_count"], expected_cooling_entries);
    assert_eq!(flow["unit_off_skip_count"], 0);
    assert_eq!(flow["non_cooling_skip_count"], expected_non_cooling_skips);
    for field in [
        "supply_mass_flow_rate_for_cool_reset_assignment_count",
        "cooling_on_read_count",
        "cooling_on_body_entry_count",
        "zone_humidity_ratio_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_count",
        "minimum_cooling_supply_air_temperature_read_count",
        "zone_temperature_read_count",
        "delta_temperature_calculation_count",
        "delta_temperature_assignment_count",
        "delta_temperature_for_gate_read_count",
        "delta_temperature_comparison_count",
        "delta_temperature_comparison_satisfied_count",
        "delta_temperature_body_entry_count",
        "zone_cooling_setpoint_load_read_count",
        "cp_air_for_first_division_read_count",
        "zone_cooling_setpoint_load_over_cp_air_calculation_count",
        "delta_temperature_for_second_division_read_count",
        "supply_mass_flow_rate_for_cool_calculation_count",
        "supply_mass_flow_rate_for_cool_assignment_count",
    ] {
        assert_eq!(flow[field], expected_cooling_entries, "{field}");
    }
    assert_eq!(flow["cooling_on_fallthrough_count"], 0);
    assert_eq!(flow["delta_temperature_fallthrough_count"], 0);

    let latest = &flow["latest"];
    assert_exact_object_keys(
        latest,
        &[
            "source",
            "first_excluded_source",
            "system",
            "parent_call_ordinal",
            "source_order",
            "controlled_zone",
            "unit_body_entered",
            "predecessor_cooling_body_entered",
            "predecessor_maximum_cooling_flow_body_sibling_skipped",
            "predecessor_no_economizer_outer_guard_fallthrough_skipped",
            "predecessor_economizer_condition_fallthrough_skipped",
            "predecessor_economizer_calculation_body_executed",
            "unit_off_skipped",
            "non_cooling_skipped",
            "cooling_body_entered",
            "supply_mass_flow_rate_for_cool_reset_assigned",
            "reset_supply_mass_flow_rate_for_cool_kg_per_s",
            "cooling_on_read",
            "cooling_on",
            "cooling_on_body_entered",
            "zone_humidity_ratio_read",
            "zone_humidity_ratio",
            "psychrometric_cp_air_evaluated",
            "psychrometric_cp_air_result_j_per_kg_k",
            "cp_air_assigned",
            "cp_air_j_per_kg_k",
            "minimum_cooling_supply_air_temperature_read",
            "minimum_cooling_supply_air_temperature_c",
            "zone_temperature_read",
            "zone_temperature_c",
            "delta_temperature_calculated",
            "delta_temperature_c",
            "delta_temperature_assigned",
            "assigned_delta_temperature_c",
            "delta_temperature_for_gate_read",
            "delta_temperature_for_gate_c",
            "delta_temperature_comparison_evaluated",
            "delta_temperature_below_negative_small_temp_diff",
            "delta_temperature_body_entered",
            "zone_cooling_setpoint_load_read",
            "zone_cooling_setpoint_load_w",
            "cp_air_for_first_division_read",
            "cp_air_for_first_division_j_per_kg_k",
            "zone_cooling_setpoint_load_over_cp_air_calculated",
            "zone_cooling_setpoint_load_over_cp_air_kg_k_per_s",
            "delta_temperature_for_second_division_read",
            "delta_temperature_for_second_division_c",
            "supply_mass_flow_rate_for_cool_calculated",
            "calculated_supply_mass_flow_rate_for_cool_kg_per_s",
            "supply_mass_flow_rate_for_cool_assigned",
            "assigned_supply_mass_flow_rate_for_cool_kg_per_s",
            "resulting_supply_mass_flow_rate_for_cool_kg_per_s",
        ],
    );
    assert_eq!(latest["source"], CALC_COOLING_SENSIBLE_FLOW_SOURCE);
    assert_eq!(
        latest["first_excluded_source"],
        CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(latest["system"], 0);
    assert_eq!(latest["controlled_zone"], 0);
    assert_eq!(latest["parent_call_ordinal"], expected_calls);
    assert_eq!(
        string_array(&latest["source_order"]),
        CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER
    );
    assert_eq!(latest["unit_body_entered"], true);
    assert_eq!(
        latest["predecessor_cooling_body_entered"],
        expected_cooling_entries > 0
    );
    assert_eq!(
        latest["predecessor_maximum_cooling_flow_body_sibling_skipped"],
        false
    );
    assert_eq!(
        latest["predecessor_no_economizer_outer_guard_fallthrough_skipped"],
        expected_cooling_entries > 0
    );
    assert_eq!(
        latest["predecessor_economizer_condition_fallthrough_skipped"],
        false
    );
    assert_eq!(
        latest["predecessor_economizer_calculation_body_executed"],
        false
    );
    assert_eq!(latest["unit_off_skipped"], false);
    assert_eq!(
        latest["non_cooling_skipped"],
        expected_non_cooling_skips > 0
    );
    assert_eq!(latest["cooling_body_entered"], expected_cooling_entries > 0);

    if expected_cooling_entries == 0 {
        for field in [
            "supply_mass_flow_rate_for_cool_reset_assigned",
            "cooling_on_read",
            "cooling_on_body_entered",
            "zone_humidity_ratio_read",
            "psychrometric_cp_air_evaluated",
            "cp_air_assigned",
            "minimum_cooling_supply_air_temperature_read",
            "zone_temperature_read",
            "delta_temperature_calculated",
            "delta_temperature_assigned",
            "delta_temperature_for_gate_read",
            "delta_temperature_comparison_evaluated",
            "delta_temperature_body_entered",
            "zone_cooling_setpoint_load_read",
            "cp_air_for_first_division_read",
            "zone_cooling_setpoint_load_over_cp_air_calculated",
            "delta_temperature_for_second_division_read",
            "supply_mass_flow_rate_for_cool_calculated",
            "supply_mass_flow_rate_for_cool_assigned",
        ] {
            assert_eq!(latest[field], false, "{field}");
        }
        for field in [
            "reset_supply_mass_flow_rate_for_cool_kg_per_s",
            "cooling_on",
            "zone_humidity_ratio",
            "psychrometric_cp_air_result_j_per_kg_k",
            "cp_air_j_per_kg_k",
            "minimum_cooling_supply_air_temperature_c",
            "zone_temperature_c",
            "delta_temperature_c",
            "assigned_delta_temperature_c",
            "delta_temperature_for_gate_c",
            "delta_temperature_below_negative_small_temp_diff",
            "zone_cooling_setpoint_load_w",
            "cp_air_for_first_division_j_per_kg_k",
            "zone_cooling_setpoint_load_over_cp_air_kg_k_per_s",
            "delta_temperature_for_second_division_c",
            "calculated_supply_mass_flow_rate_for_cool_kg_per_s",
            "assigned_supply_mass_flow_rate_for_cool_kg_per_s",
            "resulting_supply_mass_flow_rate_for_cool_kg_per_s",
        ] {
            assert!(latest[field].is_null(), "{field}");
        }
        return;
    }

    for field in [
        "supply_mass_flow_rate_for_cool_reset_assigned",
        "cooling_on_read",
        "cooling_on_body_entered",
        "zone_humidity_ratio_read",
        "psychrometric_cp_air_evaluated",
        "cp_air_assigned",
        "minimum_cooling_supply_air_temperature_read",
        "zone_temperature_read",
        "delta_temperature_calculated",
        "delta_temperature_assigned",
        "delta_temperature_for_gate_read",
        "delta_temperature_comparison_evaluated",
        "delta_temperature_body_entered",
        "zone_cooling_setpoint_load_read",
        "cp_air_for_first_division_read",
        "zone_cooling_setpoint_load_over_cp_air_calculated",
        "delta_temperature_for_second_division_read",
        "supply_mass_flow_rate_for_cool_calculated",
        "supply_mass_flow_rate_for_cool_assigned",
    ] {
        assert_eq!(latest[field], true, "{field}");
    }
    assert_eq!(latest["reset_supply_mass_flow_rate_for_cool_kg_per_s"], 0.0);
    assert_eq!(latest["cooling_on"], true);
    assert_eq!(
        latest["delta_temperature_below_negative_small_temp_diff"],
        true
    );
    let cp_air = latest["cp_air_j_per_kg_k"]
        .as_f64()
        .expect("CP318 CpAir must be numeric");
    assert_eq!(
        latest["psychrometric_cp_air_result_j_per_kg_k"].as_f64(),
        Some(cp_air)
    );
    assert_eq!(
        latest["cp_air_for_first_division_j_per_kg_k"].as_f64(),
        Some(cp_air)
    );
    let minimum_supply = latest["minimum_cooling_supply_air_temperature_c"]
        .as_f64()
        .expect("CP318 minimum supply temperature must be numeric");
    let zone_temperature = latest["zone_temperature_c"]
        .as_f64()
        .expect("CP318 Zone temperature must be numeric");
    let delta_temperature = latest["delta_temperature_c"]
        .as_f64()
        .expect("CP318 DeltaT must be numeric");
    assert_close(
        delta_temperature,
        minimum_supply - zone_temperature,
        1.0e-12,
    );
    for field in [
        "assigned_delta_temperature_c",
        "delta_temperature_for_gate_c",
        "delta_temperature_for_second_division_c",
    ] {
        assert_eq!(
            latest[field].as_f64().map(f64::to_bits),
            Some(delta_temperature.to_bits()),
            "{field}"
        );
    }
    let load = latest["zone_cooling_setpoint_load_w"]
        .as_f64()
        .expect("CP318 cooling load must be numeric");
    let first_division = latest["zone_cooling_setpoint_load_over_cp_air_kg_k_per_s"]
        .as_f64()
        .expect("CP318 first division must be numeric");
    assert_close(first_division, load / cp_air, 1.0e-12);
    let calculated_flow = latest["calculated_supply_mass_flow_rate_for_cool_kg_per_s"]
        .as_f64()
        .expect("CP318 calculated flow must be numeric");
    assert_close(calculated_flow, first_division / delta_temperature, 1.0e-12);
    for field in [
        "assigned_supply_mass_flow_rate_for_cool_kg_per_s",
        "resulting_supply_mass_flow_rate_for_cool_kg_per_s",
    ] {
        assert_eq!(
            latest[field].as_f64().map(f64::to_bits),
            Some(calculated_flow.to_bits()),
            "{field}"
        );
    }
}

#[test]
fn all_hard_sized_finite_limit_branches_limit_live_cooling()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, limit, first_matches, second_comparisons, second_matches, flow_reads) in [
        ("finite-capacity-cooling", "LimitCapacity", 0, 2, 0, 0),
        ("finite-flow-cooling", "LimitFlowRate", 2, 0, 0, 2),
        (
            "finite-flow-capacity-cooling",
            "LimitFlowRateAndCapacity",
            0,
            2,
            2,
            2,
        ),
    ] {
        let case_dir = unique_case_dir(name)?;
        let input_path = case_dir.join("direct-zone-finite-cooling.epJSON");
        let weather_path = case_dir.join("weather.epw");
        let output_dir = case_dir.join("out");
        let fixture = finite_limit_cooling_fixture(limit);
        write_text(&input_path, &fixture)?;
        write_text(&weather_path, ONE_DAY_EPW)?;

        let outcome = run_arbitrary_idf(&run_config(
            input_path,
            Some(weather_path),
            output_dir.clone(),
        ))?;

        assert_eq!(outcome.exit_code, RunExitCode::Success);
        let summary = read_json(&output_dir.join("run-summary.json"))?;
        assert_eq!(
            summary["support"]["runtime_class"],
            DIRECT_ZONE_COUPLED_RUNTIME_CLASS
        );
        assert_eq!(
            summary["support"]["matched_capability_ids"][0],
            "ideal_loads_finite_limits"
        );
        assert_eq!(
            summary["rust_runtime"]["zone_demand_source"],
            ZONE_DEMAND_SOURCE
        );
        let cooling_entry =
            &summary["rust_runtime"]["purchased_air_calc_cooling_entry_gate_lifecycle"];
        assert_eq!(cooling_entry["source"], CALC_COOLING_ENTRY_GATE_SOURCE);
        assert_eq!(cooling_entry["transition_count"], 2);
        assert_eq!(cooling_entry["sensible_comparison_satisfied_count"], 2);
        assert_eq!(cooling_entry["temperature_control_type_read_count"], 2);
        assert_eq!(cooling_entry["single_heat_block_count"], 0);
        assert_eq!(cooling_entry["cooling_body_entry_count"], 2);
        assert_eq!(cooling_entry["operating_mode_assignment_count"], 2);
        assert_eq!(cooling_entry["active_fallthrough_count"], 0);
        assert_eq!(
            cooling_entry["latest"]["temperature_control_type"],
            "DualHeatCool"
        );
        assert_eq!(cooling_entry["latest"]["cooling_body_entered"], true);
        assert_eq!(
            cooling_entry["latest"]["assigned_operating_mode"],
            "Cooling"
        );
        let cooling_oa_max_flow =
            &summary["rust_runtime"]["purchased_air_calc_cooling_oa_max_flow_gate_lifecycle"];
        assert_eq!(
            cooling_oa_max_flow["source"],
            CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
        );
        assert_eq!(cooling_oa_max_flow["transition_count"], 2);
        assert_eq!(cooling_oa_max_flow["source_execution_count"], 2);
        assert_eq!(
            cooling_oa_max_flow["cooling_limit_flow_rate_comparison_count"],
            2
        );
        assert_eq!(
            cooling_oa_max_flow["cooling_limit_flow_rate_match_count"],
            first_matches
        );
        assert_eq!(
            cooling_oa_max_flow["cooling_limit_flow_rate_and_capacity_comparison_count"],
            second_comparisons
        );
        assert_eq!(
            cooling_oa_max_flow["cooling_limit_flow_rate_and_capacity_match_count"],
            second_matches
        );
        assert_eq!(
            cooling_oa_max_flow["outdoor_air_mass_flow_rate_read_count"],
            flow_reads
        );
        assert_eq!(
            cooling_oa_max_flow["maximum_cooling_air_mass_flow_rate_read_count"],
            flow_reads
        );
        assert_eq!(
            cooling_oa_max_flow["strict_mass_flow_comparison_count"],
            flow_reads
        );
        assert_eq!(
            cooling_oa_max_flow["strict_mass_flow_comparison_satisfied_count"],
            0
        );
        assert_eq!(
            cooling_oa_max_flow["maximum_cooling_flow_body_entry_count"],
            0
        );
        assert_eq!(cooling_oa_max_flow["active_fallthrough_count"], 2);
        assert_eq!(
            cooling_oa_max_flow["latest"]["cooling_limit_flow_rate_value"],
            limit
        );
        assert_eq!(
            cooling_oa_max_flow["latest"]["cooling_flow_limit_active"],
            flow_reads > 0
        );
        assert_eq!(
            cooling_oa_max_flow["latest"]["outdoor_air_mass_flow_rate_read"],
            flow_reads > 0
        );
        assert_eq!(
            cooling_oa_max_flow["latest"]["strict_mass_flow_comparison_evaluated"],
            flow_reads > 0
        );
        assert_eq!(
            cooling_oa_max_flow["latest"]["maximum_cooling_flow_body_entered"],
            false
        );
        if flow_reads > 0 {
            assert_eq!(
                cooling_oa_max_flow["latest"]["outdoor_air_mass_flow_rate_kg_per_s"],
                0.0
            );
            assert!(
                cooling_oa_max_flow["latest"]["maximum_cooling_air_mass_flow_rate_kg_per_s"]
                    .as_f64()
                    .is_some_and(|value| value >= 0.0)
            );
            assert_eq!(
                cooling_oa_max_flow["latest"]["outdoor_air_mass_flow_above_maximum"],
                false
            );
        } else {
            assert!(cooling_oa_max_flow["latest"]["outdoor_air_mass_flow_rate_kg_per_s"].is_null());
            assert!(
                cooling_oa_max_flow["latest"]["maximum_cooling_air_mass_flow_rate_kg_per_s"]
                    .is_null()
            );
            assert!(cooling_oa_max_flow["latest"]["outdoor_air_mass_flow_above_maximum"].is_null());
        }
        assert_zero_effect_cooling_oa_max_flow_body(&summary["rust_runtime"], 2, 0, 2);
        assert_cooling_economizer_guard(&summary["rust_runtime"], 2, 0, 2);
        assert_cooling_economizer_condition(&summary["rust_runtime"], 2, 0, 2);
        assert_cooling_economizer_body(&summary["rust_runtime"], 2, 0, 2);
        assert_cooling_sensible_flow(&summary["rust_runtime"], 2, 0, 2);

        let results = read_json(&output_dir.join("results").join("result-store.json"))?;
        let cooling_rate = find_series(
            &results,
            "ZONE IDEAL LOADS",
            "Zone Ideal Loads Zone Total Cooling Rate",
        );
        let cooling_energy = find_series(
            &results,
            "ZONE IDEAL LOADS",
            "Zone Ideal Loads Zone Total Cooling Energy",
        );
        let predicted_cooling = find_series(
            &results,
            "ZONE ONE",
            "Zone System Predicted Sensible Load to Cooling Setpoint Heat Transfer Rate",
        );
        let rate_values = endpoint_values(cooling_rate);
        let energy_values = endpoint_values(cooling_energy);
        let predicted_values = endpoint_values(predicted_cooling);
        assert!(rate_values.iter().any(|value| *value > 0.0));
        let mut constrained_positive_demand = false;
        for ((rate_w, energy_j), predicted_threshold_w) in rate_values
            .into_iter()
            .zip(energy_values)
            .zip(predicted_values)
        {
            assert_close(energy_j, rate_w * 3_600.0, 1.0e-9);
            let cooling_demand_w = (-predicted_threshold_w).max(0.0);
            if cooling_demand_w > 0.0 {
                assert!(rate_w <= cooling_demand_w);
                constrained_positive_demand |= rate_w < cooling_demand_w;
            } else {
                assert_eq!(rate_w, 0.0);
            }
        }
        assert!(
            constrained_positive_demand,
            "the deliberately undersized cooling branch {limit} must limit positive predicted demand"
        );
    }
    Ok(())
}

#[test]
fn direct_zone_coupled_runtime_requires_weather_before_execution()
-> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("direct-zone-coupled-missing-weather")?;
    let input_path = case_dir.join("direct-zone-coupled.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, DIRECT_ZONE_COUPLED_EPJSON)?;

    let outcome = run_arbitrary_idf(&run_config(input_path, None, output_dir.clone()))?;

    assert_eq!(outcome.exit_code, RunExitCode::Args);
    assert_eq!(
        outcome.support_status,
        SupportStatus::SupportedCompatibility
    );
    assert_eq!(
        outcome.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );
    assert!(
        !output_dir
            .join("results")
            .join("result-store.json")
            .exists()
    );

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "args");
    assert_eq!(summary["exit_code"], RunExitCode::Args.code());
    assert_eq!(
        summary["support"]["runtime_class"],
        DIRECT_ZONE_COUPLED_RUNTIME_CLASS
    );
    assert!(summary["rust_runtime"].is_null());

    let diagnostics = read_json(&output_dir.join("diagnostics.json"))?;
    assert!(
        diagnostics["diagnostics"]
            .as_array()
            .expect("diagnostics should be an array")
            .iter()
            .any(|diagnostic| {
                diagnostic["severity"] == "error"
                    && diagnostic["code"] == "MissingWeatherFile"
                    && diagnostic["stage"] == "input"
            })
    );
    Ok(())
}

fn finite_limit_fixture(limit: &str) -> String {
    DIRECT_ZONE_COUPLED_EPJSON.replace(
        "      \"heating_limit\": \"NoLimit\",\n      \"cooling_limit\": \"NoLimit\",",
        &format!(
            "      \"heating_limit\": \"{limit}\",\n      \"maximum_heating_air_flow_rate\": 0.005,\n      \"maximum_sensible_heating_capacity\": 300.0,\n      \"cooling_limit\": \"{limit}\",\n      \"maximum_cooling_air_flow_rate\": 0.005,\n      \"maximum_total_cooling_capacity\": 300.0,"
        ),
    )
}

fn finite_limit_cooling_fixture(limit: &str) -> String {
    finite_limit_fixture(limit)
        .replace(
            "\"Heating Setpoint\": {\"hourly_value\": 30.0}",
            "\"Heating Setpoint\": {\"hourly_value\": 10.0}",
        )
        .replace(
            "\"Cooling Setpoint\": {\"hourly_value\": 35.0}",
            "\"Cooling Setpoint\": {\"hourly_value\": 15.0}",
        )
}

fn run_config(
    input_path: PathBuf,
    weather_path: Option<PathBuf>,
    output_dir: PathBuf,
) -> RunConfig {
    RunConfig {
        input_path,
        weather_path,
        output_dir,
        mode: RunMode::Compatibility,
        partial_policy: PartialRunPolicy::Deny,
        output_format: RunOutputFormat::RustNative,
        overwrite: true,
        keep_intermediate: true,
        trace_level: TraceLevel::Normal,
        trace_selection: TraceSelection::default(),
        fail_on_warning: false,
        dry_run: false,
        oracle_baseline: false,
        compare_oracle: false,
        json_stdout: false,
        oracle_root: None,
        hours: Some(2),
    }
}

fn find_series<'a>(results: &'a Value, key: &str, variable_name: &str) -> &'a Value {
    results["series"]
        .as_array()
        .expect("result series should be an array")
        .iter()
        .find(|series| series["key"] == key && series["variable_name"] == variable_name)
        .expect("requested output series should be present")
}

fn assert_unique_output_handles(results: &Value) {
    let series = results["series"]
        .as_array()
        .expect("result series should be an array");
    let handles = series
        .iter()
        .map(|entry| {
            entry["handle"]
                .as_u64()
                .expect("output handle should be an unsigned integer")
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        handles.len(),
        series.len(),
        "every result series must have a unique output handle"
    );
}

fn assert_exact_object_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("value should be an object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_finite_endpoints(series: &Value) {
    for value in endpoint_values(series) {
        assert!(value.is_finite(), "result endpoint must be finite");
    }
}

fn endpoint_values(series: &Value) -> [f64; 2] {
    [
        series["first"]
            .as_f64()
            .expect("first result value should be numeric"),
        series["last"]
            .as_f64()
            .expect("last result value should be numeric"),
    ]
}

fn string_array(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value should be a string array")
        .iter()
        .map(|entry| entry.as_str().expect("array entry should be a string"))
        .collect()
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}, tolerance {tolerance}"
    );
}

fn unique_case_dir(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rusted-energyplus-ep-run-{name}-{}-{timestamp}",
        std::process::id()
    ));
    if path.exists() {
        std::fs::remove_dir_all(&path)?;
    }
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

fn write_text(path: &Path, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, contents)?;
    Ok(())
}

fn read_json(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}
