//! IdealLoads arbitrary-run integration tests.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ep_run::{
    PartialRunPolicy, RunConfig, RunExitCode, RunMode, RunOutputFormat, RunResultState,
    SupportStatus, TraceLevel, run_arbitrary_idf,
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
fn mixed_declared_ideal_loads_runs_compatibility_runtime() -> Result<(), Box<dyn std::error::Error>>
{
    let case_dir = unique_case_dir("mixed-ideal-loads-compatibility")?;
    let input_path = case_dir.join("mixed-ideal-loads.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, IDEAL_LOADS_MIXED_BRANCH_EPJSON)?;

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
    assert_eq!(summary["support"]["status"], "supported-compatibility");
    assert_eq!(
        summary["support"]["run_result_state"],
        "supported_compatibility_run"
    );
    assert_eq!(
        summary["support"]["runtime_class"],
        "ideal-loads-mixed-declared-compatibility"
    );
    let capability_ids = summary["support"]["matched_capability_ids"]
        .as_array()
        .expect("matched capability ids should be an array");
    assert!(
        capability_ids
            .iter()
            .any(|id| id == "ideal_loads_no_oa_sensible")
    );
    assert!(
        capability_ids
            .iter()
            .any(|id| id == "ideal_loads_finite_limits")
    );
    assert_eq!(summary["support"]["conformance_claim"], false);
    assert!(
        summary["support"]["runtime_selection_note"]
            .as_str()
            .unwrap()
            .contains("selected runtime")
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "ideal-loads-mixed-declared-compatibility"
    );
    assert_eq!(summary["source_order_gate"]["matches"], true);
    assert_output_manifest(&output_dir, SUPPORTED_RUNTIME_MANIFEST)?;
    Ok(())
}

#[test]
fn ideal_loads_humidity_selected_branch_runs_declared_compatibility_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("ideal-loads-humidity-selected")?;
    let input_path = case_dir.join("ideal-loads-humidity.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, IDEAL_LOADS_CONSTANT_SUPPLY_HUMIDITY_EPJSON)?;

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
    assert_eq!(summary["support"]["status"], "supported-compatibility");
    assert_eq!(
        summary["support"]["runtime_class"],
        "ideal-loads-humidity-selected-branches-compatibility"
    );
    assert_eq!(
        summary["support"]["matched_capability_ids"][0],
        "ideal_loads_humidity_selected_branches"
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "ideal-loads-humidity-selected-branches-compatibility"
    );
    assert_eq!(summary["rust_runtime"]["samples"], 1);
    assert_eq!(summary["source_order_gate"]["matches"], true);
    assert!(
        summary["rust_runtime"]["source_order_stages"]
            .as_array()
            .expect("source order stages should be an array")
            .iter()
            .any(|stage| stage == "calc-purch-air-loads")
    );
    let results = std::fs::read_to_string(output_dir.join("results").join("result-store.json"))?;
    assert!(results.contains("System Node Humidity Ratio"));
    assert_output_manifest(&output_dir, SUPPORTED_RUNTIME_MANIFEST)?;
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
