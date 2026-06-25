//! Integration tests for the arbitrary-run pipeline.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ep_run::{
    PartialRunPolicy, RunConfig, RunExitCode, RunMode, RunOutputFormat, RunResultState,
    SupportStatus, TraceLevel, run_arbitrary_idf,
};
use serde_json::Value;

#[path = "arbitrary_run/fixtures.rs"]
mod fixtures;

use fixtures::*;

#[path = "arbitrary_run/output_manifest.rs"]
mod output_manifest;

use output_manifest::{
    BLOCKED_AFTER_SUPPORT_MANIFEST, SUPPORTED_RUNTIME_MANIFEST, assert_output_manifest,
};

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
    assert_output_manifest(&output_dir, SUPPORTED_RUNTIME_MANIFEST)?;
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
    assert_eq!(summary["source_order_gate"]["matches"], true);
    assert_eq!(
        summary["source_order_gate"]["expected_source_order_stages"][0],
        "get-heat-balance-input"
    );
    assert!(
        summary["source_order_gate"]["expected_source_order_stages"]
            .as_array()
            .expect("source-order stages should be an array")
            .iter()
            .any(|stage| stage == "manage-zone-air-updates")
    );
    assert_eq!(
        summary["source_order_gate"]["actual_executed_source_order_stages"],
        summary["source_order_gate"]["expected_source_order_stages"]
    );
    assert_eq!(
        summary["rust_runtime"]["source_order_stages"][0],
        "get-heat-balance-input"
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
    assert_output_manifest(&output_dir, BLOCKED_AFTER_SUPPORT_MANIFEST)?;
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
fn unsupported_plant_loop_blocks_before_runtime() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("unsupported-plant-loop")?;
    let input_path = case_dir.join("plant-loop.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, PLANT_LOOP_EPJSON)?;

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

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "unsupported");
    assert!(summary["rust_runtime"].is_null());

    let diagnostics = std::fs::read_to_string(output_dir.join("diagnostics.json"))?;
    assert!(diagnostics.contains("UnsupportedPlantObject"));
    let support_report = std::fs::read_to_string(output_dir.join("support-report.md"))?;
    assert!(support_report.contains("Plant objects are typed for graph diagnostics"));
    Ok(())
}

#[test]
fn unsupported_ems_blocks_before_runtime() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("unsupported-ems")?;
    let input_path = case_dir.join("ems.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, EMS_EPJSON)?;

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
    assert!(
        !output_dir
            .join("results")
            .join("result-store.json")
            .exists()
    );

    let diagnostics = std::fs::read_to_string(output_dir.join("diagnostics.json"))?;
    assert!(diagnostics.contains("UnsupportedEMS"));
    let support_report = std::fs::read_to_string(output_dir.join("support-report.md"))?;
    assert!(support_report.contains("Runtime-modifying plugin/network features are not ported."));
    Ok(())
}

#[test]
fn missing_weather_blocks_heat_balance_before_runtime() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("missing-weather")?;
    let input_path = case_dir.join("one-zone.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, ONE_ZONE_EPJSON)?;

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
    assert_eq!(
        summary["support"]["runtime_class"],
        "one-zone-heat-balance-compatibility"
    );
    assert!(summary["rust_runtime"].is_null());
    let diagnostics = std::fs::read_to_string(output_dir.join("diagnostics.json"))?;
    assert!(diagnostics.contains("MissingWeatherFile"));
    Ok(())
}

#[test]
fn invalid_epjson_returns_import_parse_failure() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("invalid-epjson")?;
    let input_path = case_dir.join("invalid.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, "{ not valid epjson")?;

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

    assert_eq!(outcome.exit_code, RunExitCode::ImportParse);
    assert_eq!(outcome.support_status, SupportStatus::Unsupported);
    assert_eq!(outcome.run_result_state, RunResultState::RunBlocked);

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "import-parse");
    assert!(!output_dir.join("support-assessment.json").exists());
    assert!(
        !output_dir
            .join("results")
            .join("result-store.json")
            .exists()
    );
    let diagnostics = std::fs::read_to_string(output_dir.join("diagnostics.json"))?;
    assert!(diagnostics.contains("RawModelParseFailed"));
    Ok(())
}

#[test]
fn unresolved_model_reference_returns_compile_reference_failure()
-> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("compile-reference")?;
    let input_path = case_dir.join("missing-zone-reference.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, MISSING_SURFACE_ZONE_EPJSON)?;

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

    assert_eq!(outcome.exit_code, RunExitCode::CompileReference);
    assert_eq!(outcome.support_status, SupportStatus::Unsupported);
    assert_eq!(outcome.run_result_state, RunResultState::RunBlocked);
    for file in [
        "run-summary.json",
        "diagnostics.json",
        "support-assessment.json",
        "support-report.md",
        "model/raw-model-summary.json",
        "model/typed-model-summary.json",
        "reports/run-report.md",
        "reports/compatibility-boundary.md",
    ] {
        assert!(
            output_dir.join(file).is_file(),
            "missing output artifact {file}"
        );
    }
    assert!(!output_dir.join("model").join("graph-summary.json").exists());
    assert!(
        !output_dir
            .join("model")
            .join("execution-plan.json")
            .exists()
    );
    assert!(
        !output_dir
            .join("results")
            .join("result-store.json")
            .exists()
    );

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "compile-reference");
    assert_eq!(summary["exit_code"], 3);
    assert_eq!(summary["support"]["status"], "unsupported");
    assert!(summary["rust_runtime"].is_null());

    let diagnostics = read_json(&output_dir.join("diagnostics.json"))?;
    let diagnostics = diagnostics["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["severity"] == "error"
            && diagnostic["code"] == "MissingReference"
            && diagnostic["stage"] == "compile"
            && diagnostic["object_type"] == "BuildingSurface:Detailed"
            && diagnostic["object_name"] == "Wall One"
            && diagnostic["field"] == "zone_name"
    }));
    Ok(())
}

#[test]
fn fail_on_warning_promotes_warning_to_non_success() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("fail-on-warning")?;
    let input_path = case_dir.join("one-zone-output-request.epJSON");
    let weather_path = case_dir.join("weather.epw");
    let output_dir = case_dir.join("out");
    let mut input_text = ONE_ZONE_EPJSON.trim_end().trim_end_matches('}').to_string();
    input_text.push_str(
        r#",
  "Output:Variable": {
    "Zone Air Temperature Request": {
      "key_value": "*",
      "variable_name": "Zone Mean Air Temperature",
      "reporting_frequency": "Hourly"
    }
  }
}"#,
    );
    write_text(&input_path, &input_text)?;
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
        fail_on_warning: true,
        dry_run: false,
        oracle_baseline: false,
        compare_oracle: false,
        json_stdout: false,
        oracle_root: None,
        hours: Some(2),
    })?;

    assert_eq!(outcome.exit_code, RunExitCode::Unsupported);
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
    assert_eq!(summary["status"], "unsupported");
    assert!(!summary["rust_runtime"].is_null());
    assert_eq!(summary["config"]["fail_on_warning"], true);

    let diagnostics = std::fs::read_to_string(output_dir.join("diagnostics.json"))?;
    assert!(diagnostics.contains("UnsupportedObjectIgnored"));
    Ok(())
}

#[test]
fn dry_run_skips_runtime_oracle_and_compare() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("dry-run")?;
    let input_path = case_dir.join("one-zone.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, ONE_ZONE_EPJSON)?;

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
        dry_run: true,
        oracle_baseline: true,
        compare_oracle: true,
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

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "success");
    assert_eq!(summary["message"], "dry run completed");
    assert_eq!(summary["config"]["dry_run"], true);
    assert_eq!(summary["config"]["oracle_baseline"], true);
    assert_eq!(summary["config"]["compare_oracle"], true);
    assert_eq!(summary["oracle_status"], "skipped-dry-run");
    assert_eq!(summary["compare_status"], "skipped-dry-run");
    assert!(summary["rust_runtime"].is_null());
    assert!(summary["oracle"].is_null());
    assert!(summary["comparison"].is_null());
    assert!(
        !output_dir
            .join("results")
            .join("result-store.json")
            .exists()
    );
    assert!(!output_dir.join("oracle").join("eplusout.eso").exists());
    assert!(
        !output_dir
            .join("compare")
            .join("compare-summary.json")
            .exists()
    );
    Ok(())
}

#[test]
fn diagnostic_allow_runs_partial_supported_projection() -> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("partial-supported-projection")?;
    let input_path = case_dir.join("mixed-ideal-loads.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, IDEAL_LOADS_MIXED_BRANCH_EPJSON)?;

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

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(summary["status"], "success");
    assert_eq!(summary["config"]["mode"], "diagnostic");
    assert_eq!(summary["config"]["partial_policy"], "allow");
    assert_eq!(summary["support"]["status"], "supported-diagnostic-only");
    assert_eq!(
        summary["support"]["run_result_state"],
        "partial_supported_run"
    );
    assert_eq!(
        summary["support"]["runtime_class"],
        "ideal-loads-node-state-projection"
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "ideal-loads-node-state-projection"
    );
    assert_eq!(summary["support"]["conformance_claim"], false);
    assert_eq!(summary["source_order_gate"]["matches"], true);
    assert_output_layout(&output_dir, true)?;
    Ok(())
}

#[test]
fn ideal_loads_diagnostic_run_uses_branch_compatibility_runtime_class()
-> Result<(), Box<dyn std::error::Error>> {
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
        SupportStatus::SupportedCompatibility
    );
    assert_eq!(
        outcome.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );
    assert_output_layout(&output_dir, true)?;

    let summary = read_json(&output_dir.join("run-summary.json"))?;
    assert_eq!(
        summary["support"]["runtime_class"],
        "ideal-loads-no-oa-sensible-compatibility"
    );
    assert_eq!(
        summary["support"]["matched_capability_ids"][0],
        "ideal_loads_no_oa_sensible"
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "ideal-loads-no-oa-sensible-compatibility"
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
    let plan = read_json(&output_dir.join("model").join("execution-plan.json"))?;
    assert_eq!(plan["source_order_gate"]["matches"], true);
    assert_eq!(
        plan["source_order_gate"]["actual_executed_source_order_stages"],
        plan["source_order_gate"]["expected_source_order_stages"]
    );
    assert!(
        plan["source_order_gate"]["expected_source_order_stages"]
            .as_array()
            .expect("source-order stages should be an array")
            .iter()
            .any(|stage| stage == "manage-zone-air-updates")
    );
    let results = std::fs::read_to_string(output_dir.join("results").join("result-store.json"))?;
    assert!(results.contains("Zone Ideal Loads Zone Total Heating Rate"));
    assert!(results.contains("System Node Temperature"));
    Ok(())
}

#[test]
fn ideal_loads_compatibility_mode_runs_declared_branch_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let case_dir = unique_case_dir("ideal-loads-compatibility")?;
    let input_path = case_dir.join("ideal-loads.epJSON");
    let output_dir = case_dir.join("out");
    write_text(&input_path, IDEAL_LOADS_EPJSON)?;

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
    assert_eq!(
        summary["support"]["runtime_class"],
        "ideal-loads-no-oa-sensible-compatibility"
    );
    assert_eq!(
        summary["rust_runtime"]["runtime_class"],
        "ideal-loads-no-oa-sensible-compatibility"
    );
    assert_eq!(summary["source_order_gate"]["matches"], true);
    assert_output_layout(&output_dir, true)?;
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
        "input/input-hashes.json",
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
