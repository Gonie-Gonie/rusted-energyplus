//! Integration tests for the arbitrary-run pipeline.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ep_run::{
    PartialRunPolicy, RunConfig, RunExitCode, RunMode, RunOutputFormat, RunResultState,
    SupportStatus, TraceLevel, run_arbitrary_idf,
};
use serde_json::Value;

#[test]
fn one_zone_runtime_writes_stable_output_layout() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("one-zone-runtime")?;
    let input_path = case_dir.join("one-zone.epJSON");
    let weather_path = case_dir.join("weather.epw");
    let output_dir = case_dir.join("out");
    write_text(&input_path, ONE_ZONE_EPJSON)?;
    write_text(&weather_path, TWO_HOUR_EPW)?;

    let outcome = run_arbitrary_idf(&RunConfig {
        input_path,
        weather_path: Some(weather_path),
        output_dir: output_dir.clone(),
        mode: RunMode::Compatibility,
        partial_policy: PartialRunPolicy::Deny,
        output_format: RunOutputFormat::RustNative,
        overwrite: true,
        keep_intermediate: true,
        trace_level: TraceLevel::Normal,
        fail_on_warning: false,
        dry_run: false,
        oracle_baseline: false,
        compare_oracle: false,
        json_stdout: false,
        oracle_root: None,
        hours: Some(2),
    })?;

    assert_eq!(outcome.exit_code, RunExitCode::Success);
    assert_eq!(
        outcome.support_status,
        SupportStatus::SupportedCompatibility
    );
    assert_eq!(
        outcome.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );

    assert_output_layout(&output_dir, true)?;
    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "success");
    assert_eq!(
        summary["support"]["runtime_class"],
        "one-zone-heat-balance-compatibility"
    );
    assert_eq!(
        summary["support"]["matched_capability_ids"][0],
        "official_1zone_uncontrolled_declared_heat_balance"
    );
    assert_eq!(summary["rust_runtime"]["samples"], 2);
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "one-zone-heat-balance-compatibility"
    );
    Ok(())
}

#[test]
fn unsupported_air_loop_blocks_before_runtime() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("unsupported-air-loop")?;
    let input_path = case_dir.join("air-loop.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, AIR_LOOP_EPJSON)?;

    let outcome = run_arbitrary_idf(&RunConfig {
        input_path,
        weather_path: None,
        output_dir: output_dir.clone(),
        mode: RunMode::Compatibility,
        partial_policy: PartialRunPolicy::Deny,
        output_format: RunOutputFormat::RustNative,
        overwrite: true,
        keep_intermediate: true,
        trace_level: TraceLevel::Normal,
        fail_on_warning: false,
        dry_run: false,
        oracle_baseline: false,
        compare_oracle: false,
        json_stdout: false,
        oracle_root: None,
        hours: Some(1),
    })?;

    assert_eq!(outcome.exit_code, RunExitCode::Unsupported);
    assert_eq!(outcome.support_status, SupportStatus::Unsupported);
    assert_eq!(outcome.run_result_state, RunResultState::RunBlocked);
    assert_output_layout(&output_dir, false)?;
    assert!(
        !output_dir
            .join("results")
            .join("result-store.json")
            .exists()
    );

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "unsupported");
    assert_eq!(summary["support"]["status"], "unsupported");
    assert!(summary["rust_runtime"].is_null());

    let support_report = std::fs::read_to_string(output_dir.join("support-report.md"))?;
    assert!(support_report.contains("Broad HVAC air-loop semantics are not ported."));
    Ok(())
}

#[test]
fn ideal_loads_diagnostic_run_uses_branch_runtime_class() -> Result<(), Box<dyn std::error::Error>>
{
    let case_dir = unique_case_dir("ideal-loads-diagnostic")?;
    let input_path = case_dir.join("ideal-loads.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, IDEAL_LOADS_EPJSON)?;

    let outcome = run_arbitrary_idf(&RunConfig {
        input_path,
        weather_path: None,
        output_dir: output_dir.clone(),
        mode: RunMode::Diagnostic,
        partial_policy: PartialRunPolicy::Allow,
        output_format: RunOutputFormat::RustNative,
        overwrite: true,
        keep_intermediate: true,
        trace_level: TraceLevel::Normal,
        fail_on_warning: false,
        dry_run: false,
        oracle_baseline: false,
        compare_oracle: false,
        json_stdout: false,
        oracle_root: None,
        hours: Some(1),
    })?;

    assert_eq!(outcome.exit_code, RunExitCode::Success);
    assert_eq!(
        outcome.support_status,
        SupportStatus::SupportedDiagnosticOnly
    );
    assert_eq!(
        outcome.run_result_state,
        RunResultState::PartialSupportedRun
    );
    assert_output_layout(&output_dir, true)?;

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(
        summary["support"]["runtime_class"],
        "ideal-loads-no-oa-sensible-diagnostic-projection"
    );
    assert_eq!(
        summary["support"]["matched_capability_ids"][0],
        "ideal_loads_no_oa_sensible"
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "ideal-loads-no-oa-sensible-diagnostic-projection"
    );
    assert_eq!(summary["rust_runtime"]["samples"], 1);
    Ok(())
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

fn assert_output_layout(
    output_dir: &Path,
    expects_runtime_artifacts: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for directory in [
        "input", "model", "results", "reports", "logs", "oracle", "compare",
    ] {
        assert!(
            output_dir.join(directory).is_dir(),
            "missing output directory {directory}"
        );
    }

    for file in [
        "run-summary.json",
        "diagnostics.json",
        "support-assessment.json",
        "support-report.md",
        "eplusrs.err",
        "input/original.epJSON",
        "input/converted.epJSON",
        "model/raw-model-summary.json",
        "model/typed-model-summary.json",
        "model/graph-summary.json",
        "model/execution-plan.json",
        "reports/run-report.md",
        "reports/compatibility-boundary.md",
        "logs/command.log",
    ] {
        assert!(
            output_dir.join(file).is_file(),
            "missing output artifact {file}"
        );
    }

    if expects_runtime_artifacts {
        for file in [
            "results/result-store.json",
            "results/selected-outputs.csv",
            "results/meters.csv",
        ] {
            assert!(
                output_dir.join(file).is_file(),
                "missing runtime artifact {file}"
            );
        }
    }
    Ok(())
}

const TWO_HOUR_EPW: &str = r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
1999,1,1,1,0,Source,-3.0,-4.0,50,82000,0,0,300,10,20,30,0,0,0,0,180,2.5
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6
"#;

const ONE_ZONE_EPJSON: &str = r#"{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Building": {"Defaulted Building": {"terrain": "Suburbs"}},
  "Timestep": {"Timestep 1": {}},
  "Site:Location": {"Denver Site": {"latitude": 39.74, "longitude": -105.18}},
  "Material:NoMass": {"R13": {"thermal_resistance": 2.29}},
  "Construction": {"Wall Construction": {"outside_layer": "R13"}},
  "ScheduleTypeLimits": {
    "Fraction": {
      "lower_limit_value": 0.0,
      "numeric_type": "Continuous",
      "upper_limit_value": 1.0
    }
  },
  "Schedule:Constant": {
    "Always On": {"schedule_type_limits_name": "Fraction"}
  },
  "Zone": {"Zone One": {"volume": 100}},
  "BuildingSurface:Detailed": {
    "Wall One": {
      "construction_name": "Wall Construction",
      "outside_boundary_condition": "Outdoors",
      "surface_type": "Wall",
      "vertices": [
        {"vertex_x_coordinate": 0.0, "vertex_y_coordinate": 0.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 1.0, "vertex_y_coordinate": 0.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 1.0, "vertex_y_coordinate": 1.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 0.0, "vertex_y_coordinate": 1.0, "vertex_z_coordinate": 0.0}
      ],
      "zone_name": "Zone One"
    }
  }
}"#;

const AIR_LOOP_EPJSON: &str = r#"{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Zone": {"Zone One": {"volume": 100}},
  "AirLoopHVAC": {"Main Air Loop": {}}
}"#;

const IDEAL_LOADS_EPJSON: &str = r#"{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Zone": {"Zone One": {"volume": 100}},
  "NodeList": {
    "Zone Inlets": {
      "nodes": [{"node_name": "Zone One Inlet"}]
    }
  },
  "ZoneHVAC:IdealLoadsAirSystem": {
    "Zone Ideal Loads": {
      "zone_supply_air_node_name": "Zone Inlets",
      "dehumidification_control_type": "None",
      "humidification_control_type": "None"
    }
  },
  "ZoneHVAC:EquipmentList": {
    "Zone Equipment": {
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
    "Zone One": {
      "zone_name": "Zone One",
      "zone_conditioning_equipment_list_name": "Zone Equipment",
      "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
      "zone_air_node_name": "Zone One Air Node",
      "zone_return_air_node_or_nodelist_name": "Zone One Return"
    }
  }
}"#;
