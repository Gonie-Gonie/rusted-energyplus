//! Exit-code focused integration tests for the arbitrary-run pipeline.

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

#[test]
fn invalid_weather_file_returns_runtime_failure() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("runtime-invalid-weather")?;
    let input_path = case_dir.join("one-zone.epJSON");
    let weather_path = case_dir.join("invalid-weather.epw");
    let output_dir = case_dir.join("out");
    write_text(&input_path, ONE_ZONE_EPJSON)?;
    write_text(&weather_path, "LOCATION,Invalid Weather\n")?;

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
        trace_selection: TraceSelection::default(),
        fail_on_warning: false,
        dry_run: false,
        oracle_baseline: false,
        compare_oracle: false,
        json_stdout: false,
        oracle_root: None,
        hours: Some(1),
    })?;

    assert_eq!(outcome.exit_code, RunExitCode::Runtime);
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
    assert_eq!(summary["status"], "runtime");
    assert_eq!(summary["exit_code"], 6);
    assert_eq!(
        summary["support"]["runtime_class"],
        "one-zone-heat-balance-compatibility"
    );
    assert!(summary["rust_runtime"].is_null());

    let diagnostics = read_json(&output_dir.join("diagnostics.json"))?;
    let diagnostics = diagnostics["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["severity"] == "error"
            && diagnostic["code"] == "RuntimeConvergenceFailure"
            && diagnostic["stage"] == "runtime"
    }));
    Ok(())
}

#[test]
fn output_path_file_returns_output_export_failure() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("output-export-file")?;
    let input_path = case_dir.join("one-zone.epJSON");
    let output_path = case_dir.join("output-is-a-file");
    write_text(&input_path, ONE_ZONE_EPJSON)?;
    write_text(&output_path, "not a directory")?;

    let error = run_arbitrary_idf(&RunConfig {
        input_path,
        weather_path: None,
        output_dir: output_path.clone(),
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
    })
    .expect_err("output path file should fail before summary export");

    assert_eq!(error.exit_code, RunExitCode::OutputExport);
    assert!(error.message.contains("failed to remove output directory"));
    assert!(output_path.is_file());
    Ok(())
}

fn unique_case_dir(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rusted-energyplus-ep-run-exit-codes-{name}-{}-{timestamp}",
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
