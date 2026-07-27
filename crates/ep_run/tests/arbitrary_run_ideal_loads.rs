//! IdealLoads arbitrary-run integration tests.

#![allow(clippy::expect_used, clippy::unwrap_used)]

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

use fixtures::*;

#[allow(dead_code)]
#[path = "arbitrary_run/output_manifest.rs"]
mod output_manifest;

use output_manifest::{SUPPORTED_RUNTIME_MANIFEST, assert_output_manifest};

#[test]
fn ideal_loads_no_oa_branch_runs_declared_compatibility_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let summary = assert_direct_ideal_loads_fixture_runs(
        "ideal-loads-no-oa",
        IDEAL_LOADS_EPJSON,
        "ideal-loads-direct-zone-coupled-compatibility",
        "ideal_loads_no_oa_sensible",
    )?;

    assert_eq!(summary["rust_runtime"]["samples"], 1);
    assert_eq!(
        summary["rust_runtime"]["zone_demand_source"],
        "rust-predictor-source-setpoint-thresholds"
    );
    assert_eq!(
        summary["rust_runtime"]["fixture_demand_injection_used"],
        false
    );
    assert_eq!(
        summary["rust_runtime"]["recirculation_state_source"],
        "rust-direct-zone-return-projection"
    );
    assert!(
        summary["rust_runtime"]["source_order_stages"]
            .as_array()
            .expect("source order stages should be an array")
            .iter()
            .any(|stage| stage == "calc-purch-air-loads")
    );
    Ok(())
}

#[test]
fn ideal_loads_fixture_demand_fallbacks_fail_closed_in_compatibility_mode()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, fixture) in [
        (
            "ideal-loads-constant-shr",
            IDEAL_LOADS_CONSTANT_SHR_EPJSON.to_string(),
        ),
        (
            "ideal-loads-constant-supply-humidity",
            IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_EPJSON.to_string(),
        ),
        (
            "ideal-loads-outdoor-air",
            IDEAL_LOADS_OUTDOOR_AIR_EPJSON.to_string(),
        ),
        (
            "ideal-loads-mixed",
            IDEAL_LOADS_MIXED_BRANCH_EPJSON.to_string(),
        ),
        (
            "ideal-loads-finite-hysteresis",
            finite_limit_hysteresis_fixture(),
        ),
    ] {
        let case_dir = unique_case_dir(name)?;
        let input_path = case_dir.join("ideal-loads.epJSON");
        let output_dir = case_dir.join("out");
        write_text(&input_path, &fixture)?;

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
            trace_selection: TraceSelection::default(),
            fail_on_warning: false,
            dry_run: false,
            oracle_baseline: false,
            compare_oracle: false,
            json_stdout: false,
            oracle_root: None,
            hours: Some(1),
        })?;

        assert_eq!(outcome.exit_code, RunExitCode::Unsupported, "{name}");
        assert_eq!(
            outcome.support_status,
            SupportStatus::SupportedDiagnosticOnly,
            "{name}"
        );
        assert_eq!(
            outcome.run_result_state,
            RunResultState::RunBlocked,
            "{name}"
        );

        let summary = read_json(&output_dir.join("run-summary.json"))?;
        assert_eq!(summary["status"], "unsupported", "{name}");
        assert_eq!(
            summary["support"]["status"], "supported-diagnostic-only",
            "{name}"
        );
        assert_eq!(
            summary["support"]["run_result_state"], "run_blocked",
            "{name}"
        );
        assert_eq!(
            summary["support"]["runtime_class"], "ideal-loads-fixture-demand-diagnostic",
            "{name}"
        );
        assert_eq!(
            summary["support"]["matched_capability_ids"]
                .as_array()
                .expect("matched capability ids should be an array")
                .len(),
            0,
            "{name}"
        );
        assert_eq!(
            summary["support"]["selected_algorithm_lane"]["id"], "diagnostic-probe",
            "{name}"
        );
        assert_eq!(
            summary["support"]["selected_algorithm_lane"]["conformance_promotion_allowed"], false,
            "{name}"
        );
        assert!(summary["rust_runtime"].is_null(), "{name}");
        assert!(
            !output_dir
                .join("results")
                .join("result-store.json")
                .exists(),
            "{name}"
        );
        let diagnostics = read_json(&output_dir.join("diagnostics.json"))?;
        let diagnostic_entries = diagnostics["diagnostics"]
            .as_array()
            .expect("diagnostics should be an array");
        assert!(
            diagnostic_entries.iter().any(|diagnostic| {
                diagnostic["code"] == "IdealLoadsFixtureDemandDiagnosticOnly"
            }),
            "{name}"
        );
        assert!(
            diagnostic_entries
                .iter()
                .any(|diagnostic| diagnostic["code"] == "DiagnosticOnlyRuntimeBlocked"),
            "{name}"
        );
    }
    Ok(())
}

#[test]
fn ideal_loads_fixture_demand_runs_only_as_explicit_diagnostic_with_provenance()
-> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("ideal-loads-fixture-demand-diagnostic")?;
    let input_path = case_dir.join("ideal-loads-constant-shr.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, IDEAL_LOADS_CONSTANT_SHR_EPJSON)?;

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
        trace_selection: TraceSelection::default(),
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

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "success");
    assert_eq!(summary["support"]["status"], "supported-diagnostic-only");
    assert_eq!(
        summary["support"]["run_result_state"],
        "partial_supported_run"
    );
    assert_eq!(
        summary["support"]["runtime_class"],
        "ideal-loads-fixture-demand-diagnostic"
    );
    assert_eq!(
        summary["support"]["matched_capability_ids"]
            .as_array()
            .expect("matched capability ids should be an array")
            .len(),
        0
    );
    assert_eq!(summary["support"]["conformance_claim"], false);
    assert_eq!(
        summary["support"]["selected_algorithm_lane"]["id"],
        "diagnostic-probe"
    );
    assert_eq!(
        summary["support"]["selected_algorithm_lane"]["diagnostic_probe_used"],
        true
    );
    assert_eq!(
        summary["support"]["selected_algorithm_lane"]["conformance_promotion_allowed"],
        false
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "ideal-loads-fixture-demand-diagnostic"
    );
    assert_eq!(
        summary["rust_runtime"]["zone_demand_source"],
        "rust-diagnostic-default-active-load-split"
    );
    assert_eq!(
        summary["rust_runtime"]["fixture_demand_injection_used"],
        true
    );
    assert_eq!(summary["rust_runtime"]["samples"], 1);
    let rust_runtime = summary["rust_runtime"]
        .as_object()
        .expect("diagnostic Rust runtime should be an object");
    assert!(rust_runtime.contains_key("purchased_air_calc_cooling_economizer_condition_lifecycle"));
    assert!(rust_runtime["purchased_air_calc_cooling_economizer_condition_lifecycle"].is_null());
    assert_eq!(summary["source_order_gate"]["matches"], true);
    assert_output_manifest(&output_dir, SUPPORTED_RUNTIME_MANIFEST)?;
    assert!(
        summary["rust_runtime"]["source_order_stages"]
            .as_array()
            .expect("source order stages should be an array")
            .iter()
            .any(|stage| stage == "calc-purch-air-loads")
    );
    let results = std::fs::read_to_string(output_dir.join("results").join("result-store.json"))?;
    assert!(results.contains("System Node Humidity Ratio"));
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

fn finite_limit_hysteresis_fixture() -> String {
    IDEAL_LOADS_EPJSON
        .replace(
            r#""control_1_name": "Dual Setpoints""#,
            r#""control_1_name": "Dual Setpoints",
      "temperature_difference_between_cutout_and_setpoint": 0.5"#,
        )
        .replace(
            r#""zone_supply_air_node_name": "Zone Inlets","#,
            r#""zone_supply_air_node_name": "Zone Inlets",
      "heating_limit": "LimitFlowRateAndCapacity",
      "maximum_heating_air_flow_rate": 0.01,
      "maximum_sensible_heating_capacity": 300.0,
      "cooling_limit": "LimitFlowRateAndCapacity",
      "maximum_cooling_air_flow_rate": 0.01,
      "maximum_total_cooling_capacity": 300.0,"#,
        )
}

fn assert_direct_ideal_loads_fixture_runs(
    name: &str,
    fixture: &str,
    expected_runtime_class: &str,
    expected_capability_id: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir(name)?;
    let input_path = case_dir.join("ideal-loads.epJSON");
    let weather_path = case_dir.join("weather.epw");
    let output_dir = case_dir.join("out");
    write_text(&input_path, fixture)?;
    let weather_path = if expected_runtime_class == "ideal-loads-direct-zone-coupled-compatibility"
    {
        write_text(&weather_path, ONE_DAY_EPW)?;
        Some(weather_path)
    } else {
        None
    };

    let outcome = run_arbitrary_idf(&RunConfig {
        input_path,
        weather_path,
        output_dir: output_dir.clone(),
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
        hours: Some(1),
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
    assert_output_manifest(&output_dir, SUPPORTED_RUNTIME_MANIFEST)?;

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "success");
    assert_eq!(summary["support"]["status"], "supported-compatibility");
    assert_eq!(
        summary["support"]["run_result_state"],
        "supported_compatibility_run"
    );
    assert_eq!(summary["support"]["runtime_class"], expected_runtime_class);
    assert_eq!(
        summary["support"]["matched_capability_ids"][0],
        expected_capability_id
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        expected_runtime_class
    );
    assert_eq!(summary["source_order_gate"]["matches"], true);
    Ok(summary)
}

fn read_json(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}
