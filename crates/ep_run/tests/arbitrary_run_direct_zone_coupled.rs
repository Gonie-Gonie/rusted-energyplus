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
const COUPLED_SOURCE_ORDER: [&str; 3] = [
    "predict-system-loads",
    "sim-purchased-air",
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

#[test]
fn all_hard_sized_finite_limit_branches_limit_live_cooling()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, limit) in [
        ("finite-capacity-cooling", "LimitCapacity"),
        ("finite-flow-cooling", "LimitFlowRate"),
        ("finite-flow-capacity-cooling", "LimitFlowRateAndCapacity"),
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
