//! Artifact writers shared by the arbitrary-run pipeline.

use std::path::Path;

use ep_runtime::ResultStore;
use serde::Serialize;
use serde_json::{Number, Value, json};

use crate::{RunDiagnostics, RunExitCode, RunResultState, SupportAssessment};

pub(crate) fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(value)
        .map_err(|error| format!("failed to serialize JSON {}: {error}", path.display()))?;
    std::fs::write(path, format!("{text}\n"))
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

pub(crate) fn write_text(path: &Path, text: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }
    std::fs::write(path, text)
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

pub(crate) fn result_store_json(results: &ResultStore) -> Value {
    let series = results
        .series
        .iter()
        .map(|series| {
            json!({
                "handle": series.handle.0,
                "key": series.key,
                "variable_name": series.variable_name,
                "units": series.units,
                "samples": series.values.len(),
                "first": series.values.first().copied().map(finite_json_number),
                "last": series.values.last().copied().map(finite_json_number),
                "minimum": finite_series_min(&series.values),
                "maximum": finite_series_max(&series.values),
            })
        })
        .collect::<Vec<_>>();
    let profile = results.profile();
    json!({
        "schema_version": 1,
        "profile": {
            "series_count": profile.series_count,
            "sample_count": profile.sample_count,
            "empty_series_count": profile.empty_series_count,
        },
        "series": series,
    })
}

pub(crate) fn write_selected_outputs_csv(path: &Path, results: &ResultStore) -> Result<(), String> {
    let mut csv = String::from("series_index,handle,key,variable_name,units,sample_index,value\n");
    for (series_index, series) in results.series.iter().enumerate() {
        for (sample_index, value) in series.values.iter().enumerate() {
            csv.push_str(&format!(
                "{series_index},{},{},{},{},{sample_index},{}\n",
                series.handle.0,
                csv_field(&series.key),
                csv_field(&series.variable_name),
                csv_field(&series.units),
                if value.is_finite() {
                    format!("{value:.12}")
                } else {
                    String::new()
                }
            ));
        }
    }
    write_text(path, &csv)
}

pub(crate) fn write_empty_meters_csv(path: &Path) -> Result<(), String> {
    write_text(path, "meter_name,frequency,sample_index,value\n")
}

pub(crate) fn render_eplusrs_err(diagnostics: &RunDiagnostics, exit_code: RunExitCode) -> String {
    let error_count = diagnostics.count_by_severity(crate::RunDiagnosticSeverity::Error);
    let warning_count = diagnostics.count_by_severity(crate::RunDiagnosticSeverity::Warning);
    let info_count = diagnostics.count_by_severity(crate::RunDiagnosticSeverity::Info);
    let mut text = String::new();
    text.push_str("EnergyPlus-RS Run Diagnostics\n");
    text.push_str(&format!(
        "exit_status: {} ({})\n",
        exit_code.id(),
        exit_code.code()
    ));
    text.push_str(&format!(
        "counts: errors={error_count}, warnings={warning_count}, info={info_count}\n\n"
    ));
    if diagnostics.diagnostics.is_empty() {
        text.push_str("No diagnostics were emitted.\n");
        return text;
    }
    for diagnostic in &diagnostics.diagnostics {
        text.push_str(&format!(
            "{} [{}] {}: {}\n",
            diagnostic.severity.id(),
            diagnostic.code,
            diagnostic.stage,
            diagnostic.message
        ));
    }
    text
}

pub(crate) fn render_support_report(assessment: &SupportAssessment) -> String {
    let mut report = String::new();
    report.push_str("# Support Assessment\n\n");
    report.push_str(&format!("status: {}\n", assessment.status.id()));
    report.push_str(&format!(
        "run_result_state: {}\n",
        assessment.run_result_state.id()
    ));
    report.push_str(&format!(
        "run_result_label: {}\n",
        assessment.run_result_state.label()
    ));
    report.push_str(&format!(
        "runtime_class: {}\n",
        assessment.runtime_class.id()
    ));
    report.push_str(&format!("mode: {}\n", assessment.mode));
    report.push_str(&format!(
        "capability_registry: {}\n",
        assessment.capability_registry
    ));
    report.push_str(&format!(
        "conformance_claim: {}\n\n",
        assessment.claim_boundary.conformance_claim
    ));

    if !assessment.matched_capability_ids.is_empty() {
        report.push_str("## Matched Capabilities\n\n");
        for capability_id in &assessment.matched_capability_ids {
            report.push_str(&format!("- `{}`\n", markdown_cell(capability_id)));
        }
        report.push('\n');
    }

    if !assessment.unsupported_objects.is_empty() {
        report.push_str("## Unsupported Objects\n\n");
        report.push_str("| object_type | count | note |\n");
        report.push_str("| --- | ---: | --- |\n");
        for entry in &assessment.unsupported_objects {
            report.push_str(&format!(
                "| {} | {} | {} |\n",
                markdown_cell(&entry.object_type),
                entry.count,
                markdown_cell(&entry.note)
            ));
        }
        report.push('\n');
    }

    if !assessment.ignored_raw_only_objects.is_empty() {
        report.push_str("## Ignored Raw-Only Objects\n\n");
        report.push_str("| object_type | count | note |\n");
        report.push_str("| --- | ---: | --- |\n");
        for entry in &assessment.ignored_raw_only_objects {
            report.push_str(&format!(
                "| {} | {} | {} |\n",
                markdown_cell(&entry.object_type),
                entry.count,
                markdown_cell(&entry.note)
            ));
        }
        report.push('\n');
    }

    report.push_str("## Diagnostics\n\n");
    report.push_str("| severity | code | stage | blocking | message |\n");
    report.push_str("| --- | --- | --- | --- | --- |\n");
    for diagnostic in &assessment.diagnostics.diagnostics {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            diagnostic.severity.id(),
            markdown_cell(&diagnostic.code),
            markdown_cell(&diagnostic.stage),
            diagnostic.blocking,
            markdown_cell(&diagnostic.message)
        ));
    }
    report
}

pub(crate) fn render_compatibility_boundary(assessment: &SupportAssessment) -> String {
    format!(
        concat!(
            "# Compatibility Boundary\n\n",
            "status: {}\n",
            "run_result_state: {}\n",
            "runtime_class: {}\n",
            "conformance_claim: false\n",
            "release_evidence: false\n\n",
            "{}\n\n",
            "This file is generated for every arbitrary run so support status and ",
            "claim boundaries remain explicit even when oracle comparison artifacts ",
            "are present.\n"
        ),
        assessment.status.id(),
        assessment.run_result_state.id(),
        assessment.runtime_class.id(),
        assessment.claim_boundary.statement
    )
}

pub(crate) fn render_run_report(
    assessment: &SupportAssessment,
    rust_results_written: bool,
    oracle_status: &str,
    compare_status: &str,
) -> String {
    let mut report = String::new();
    report.push_str("# Arbitrary Run Report\n\n");
    report.push_str(&format!("support_status: {}\n", assessment.status.id()));
    report.push_str(&format!(
        "run_result_state: {}\n",
        assessment.run_result_state.id()
    ));
    report.push_str(&format!(
        "runtime_class: {}\n",
        assessment.runtime_class.id()
    ));
    report.push_str(&format!("rust_results_written: {rust_results_written}\n"));
    report.push_str(&format!("oracle_status: {oracle_status}\n"));
    report.push_str(&format!("compare_status: {compare_status}\n"));
    report.push_str("conformance_claim: false\n\n");
    match assessment.run_result_state {
        RunResultState::SupportedCompatibilityRun => {
            report.push_str(
                "The input ran inside the current compatibility-mode arbitrary runtime boundary.\n",
            );
        }
        RunResultState::PartialSupportedRun => {
            report.push_str("The input ran only in a diagnostic runtime path and does not make a conformance claim.\n");
        }
        RunResultState::RunBlocked => {
            report.push_str("The Rust runtime did not execute because support assessment found blocking diagnostics.\n");
        }
    }
    report
}

fn finite_json_number(value: f64) -> Value {
    Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

fn finite_series_min(values: &[f64]) -> Value {
    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .reduce(f64::min)
        .map(finite_json_number)
        .unwrap_or(Value::Null)
}

fn finite_series_max(values: &[f64]) -> Value {
    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .reduce(f64::max)
        .map(finite_json_number)
        .unwrap_or(Value::Null)
}

fn csv_field(value: &str) -> String {
    let escaped = value.replace('"', "\"\"");
    format!("\"{escaped}\"")
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
