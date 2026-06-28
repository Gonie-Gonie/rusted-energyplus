use std::path::Path;

use ep_compare::{SeriesComparisonStatus, SeriesDivergenceKind};
use ep_conformance::{ConformanceCase, OutputLevel};

use crate::conformance_artifacts::{ReportTimingSummary, append_timing_to_json_object};
use crate::{
    comparison_class_label, json_number, json_string, markdown_cell, output_frequency_label,
    report_format_label, source_artifact_label, variable_class_label,
};

use super::{InternalGainContext, InternalGainRow, report_outputs};

pub(super) fn write_report(
    report_dir: &Path,
    context: &InternalGainContext<'_>,
    timing: &ReportTimingSummary,
) -> Result<(), String> {
    std::fs::create_dir_all(report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;
    std::fs::write(
        report_dir.join("compare-report.md"),
        render_markdown(context),
    )
    .map_err(|error| format!("failed to write internal-gains report: {error}"))?;
    std::fs::write(
        report_dir.join("compare-summary.json"),
        append_timing_to_json_object(render_json(context), timing),
    )
    .map_err(|error| format!("failed to write internal-gains summary: {error}"))?;
    Ok(())
}

fn render_markdown(context: &InternalGainContext<'_>) -> String {
    let manifest = context.manifest;
    let mut report = String::new();
    report.push_str("# Internal Gains Conformance Report\n\n");
    report.push_str("## Manifest\n\n");
    report.push_str(&format!("case_id: {}\n", manifest.id));
    report.push_str(&format!(
        "comparison_class: {}\n",
        comparison_class_label(manifest.comparison_class)
    ));
    report.push_str(&format!(
        "conformance_claim: {}\n",
        manifest.conformance_claim
    ));
    report.push_str(&format!("oracle_version: {}\n", manifest.oracle_version));
    report.push_str(&format!("outputs: {}\n", manifest.outputs.len()));
    report.push_str(&format!(
        "eso_report_outputs: {}\n",
        report_outputs(manifest).len()
    ));
    if let Some(report_contract) = manifest.report.as_ref() {
        report.push_str(&format!(
            "report_format: {}\n",
            report_format_label(report_contract.format)
        ));
        report.push_str(&format!("report_path: {}\n", report_contract.path));
    }
    if let Some(gate) = manifest.gate.as_ref() {
        report.push_str(&format!("gate_script: {}\n", gate.script));
        report.push_str(&format!("gate_blocking: {}\n", gate.blocking));
    }
    report.push_str("claim_boundary: configured Zone Total Internal Convective/Radiant Heating Rate ESO traces only; static EIO nominal rows remain diagnostic here\n");
    report.push_str(
        "timestamp_rule: hour-ending hourly samples aligned by EnergyPlus ESO timestamp labels\n\n",
    );

    report.push_str("## Result\n\n");
    report.push_str(&format!("status: {}\n", overall_status(&context.rows)));
    report.push_str("runtime_class: internal-gain-trace\n");
    report.push_str(&format!("series: {}\n", context.rows.len()));
    report.push_str(&format!(
        "conformance_series: {}\n",
        context
            .rows
            .iter()
            .filter(|row| row.is_conformance())
            .count()
    ));
    report.push_str(&format!(
        "time_axis_samples: {}\n",
        context.time_axis.sample_count()
    ));
    report.push_str(&format!("zone_count: {}\n", context.model.zones.len()));
    report.push_str(&format!(
        "other_equipment_count: {}\n\n",
        context.model.other_equipment.len()
    ));

    report.push_str("## Series\n\n");
    report.push_str("| key | variable | level | class | frequency | source | alignment | expected | observed | compared | max_abs_delta_w | rmse_delta_w | max_rel_delta | tolerance | max_rmse_tolerance | status | first_divergence |\n");
    report
        .push_str("|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---|---:|---|---|\n");
    for row in &context.rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {} | {} | {} | {} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            output_level_label(row.level),
            variable_class_label(row.variable_class),
            output_frequency_label(row.frequency),
            source_artifact_label(row.source),
            row.alignment_label(),
            row.expected_samples,
            row.observed_samples,
            row.compared_samples,
            row.max_abs_delta,
            row.rmse_delta,
            row.max_rel_delta,
            row.tolerance_label,
            row.max_rmse_tolerance
                .map_or_else(|| "none".to_string(), |value| format!("{value:.12}")),
            row.status_label(),
            first_divergence_label(row.first_divergence.as_ref())
        ));
    }
    report
}

fn render_json(context: &InternalGainContext<'_>) -> String {
    let manifest = context.manifest;
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 1,\n");
    json.push_str(&format!("  \"case_id\": {},\n", json_string(&manifest.id)));
    json.push_str(&format!(
        "  \"oracle_version\": {},\n",
        json_string(&manifest.oracle_version)
    ));
    json.push_str(&format!(
        "  \"comparison_class\": {},\n",
        json_string(comparison_class_label(manifest.comparison_class))
    ));
    json.push_str(&format!(
        "  \"conformance_claim\": {},\n",
        manifest.conformance_claim
    ));
    json.push_str("  \"runtime_class\": \"internal-gain-trace\",\n");
    json.push_str("  \"claim_boundary\": \"configured Zone Total Internal Convective/Radiant Heating Rate ESO traces only; static EIO nominal rows remain diagnostic in this dynamic report\",\n");
    json.push_str(&format!(
        "  \"report_contract\": {},\n",
        report_contract_json(manifest)
    ));
    json.push_str(&format!("  \"gate\": {},\n", gate_json(manifest)));
    json.push_str(&format!(
        "  \"status\": {},\n",
        json_string(overall_status(&context.rows))
    ));
    json.push_str("  \"timestamp_rule\": \"hour-ending hourly samples aligned by EnergyPlus ESO timestamp labels\",\n");
    json.push_str(&format!(
        "  \"time_axis_samples\": {},\n",
        context.time_axis.sample_count()
    ));
    json.push_str(&format!(
        "  \"samples\": {},\n",
        context.time_axis.sample_count()
    ));
    json.push_str(&format!(
        "  \"zone_count\": {},\n",
        context.model.zones.len()
    ));
    json.push_str(&format!(
        "  \"other_equipment_count\": {},\n",
        context.model.other_equipment.len()
    ));
    json.push_str(&format!("  \"series_count\": {},\n", context.rows.len()));
    json.push_str(&format!(
        "  \"conformance_series_count\": {},\n",
        context
            .rows
            .iter()
            .filter(|row| row.is_conformance())
            .count()
    ));
    json.push_str(&format!(
        "  \"max_abs_delta_c\": {},\n",
        json_number(max_abs_delta(context))
    ));
    json.push_str(&format!(
        "  \"rmse_delta_c\": {},\n",
        json_number(max_rmse_delta(context))
    ));
    json.push_str(&format!(
        "  \"max_rel_delta\": {},\n",
        json_number(max_rel_delta(context))
    ));
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\"\n");
    json.push_str("  },\n");
    json.push_str("  \"series\": [\n");
    for (index, row) in context.rows.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"key\": {},\n", json_string(&row.key)));
        json.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&row.variable)
        ));
        json.push_str(&format!(
            "      \"level\": {},\n",
            json_string(output_level_label(row.level))
        ));
        json.push_str(&format!(
            "      \"class\": {},\n",
            json_string(variable_class_label(row.variable_class))
        ));
        json.push_str(&format!(
            "      \"frequency\": {},\n",
            json_string(output_frequency_label(row.frequency))
        ));
        json.push_str(&format!(
            "      \"source\": {},\n",
            json_string(source_artifact_label(row.source))
        ));
        json.push_str(&format!(
            "      \"alignment\": {},\n",
            json_string(row.alignment_label())
        ));
        json.push_str(&format!(
            "      \"expected_samples\": {},\n",
            row.expected_samples
        ));
        json.push_str(&format!(
            "      \"observed_samples\": {},\n",
            row.observed_samples
        ));
        json.push_str(&format!(
            "      \"compared_samples\": {},\n",
            row.compared_samples
        ));
        json.push_str(&format!(
            "      \"max_abs_delta\": {},\n",
            json_number(row.max_abs_delta)
        ));
        json.push_str(&format!(
            "      \"rmse_delta\": {},\n",
            json_number(row.rmse_delta)
        ));
        json.push_str(&format!(
            "      \"max_rel_delta\": {},\n",
            json_number(row.max_rel_delta)
        ));
        json.push_str(&format!(
            "      \"tolerance_policy\": {},\n",
            json_string(&row.tolerance_label)
        ));
        json.push_str(&format!(
            "      \"max_abs_tolerance\": {},\n",
            json_number(row.tolerance.absolute)
        ));
        json.push_str(&format!(
            "      \"max_rel_tolerance\": {},\n",
            json_number(row.tolerance.relative)
        ));
        json.push_str(&format!(
            "      \"max_rmse_tolerance\": {},\n",
            optional_number_json(row.max_rmse_tolerance)
        ));
        json.push_str(&format!(
            "      \"status\": {},\n",
            json_string(row.status_label())
        ));
        json.push_str(&format!(
            "      \"first_divergence\": {}\n",
            first_divergence_json(row.first_divergence.as_ref())
        ));
        json.push_str("    }");
        if index + 1 < context.rows.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

fn max_abs_delta(context: &InternalGainContext<'_>) -> f64 {
    context
        .rows
        .iter()
        .map(|row| row.max_abs_delta)
        .fold(0.0, f64::max)
}

fn max_rmse_delta(context: &InternalGainContext<'_>) -> f64 {
    context
        .rows
        .iter()
        .map(|row| row.rmse_delta)
        .fold(0.0, f64::max)
}

fn max_rel_delta(context: &InternalGainContext<'_>) -> f64 {
    context
        .rows
        .iter()
        .map(|row| row.max_rel_delta)
        .fold(0.0, f64::max)
}

fn overall_status(rows: &[InternalGainRow]) -> &'static str {
    if rows
        .iter()
        .filter(|row| row.is_conformance())
        .all(|row| row.status == SeriesComparisonStatus::Pass)
    {
        "pass"
    } else {
        "fail"
    }
}

fn output_level_label(level: Option<OutputLevel>) -> &'static str {
    match level {
        Some(OutputLevel::Required) => "required",
        Some(OutputLevel::Optional) => "optional",
        Some(OutputLevel::Baseline) => "baseline",
        Some(OutputLevel::Diagnostic) => "diagnostic",
        Some(OutputLevel::Conformance) => "conformance",
        None => "unspecified",
    }
}

fn report_contract_json(manifest: &ConformanceCase) -> String {
    let Some(report) = manifest.report.as_ref() else {
        return "null".to_string();
    };
    format!(
        "{{\"format\": {}, \"path\": {}}}",
        json_string(report_format_label(report.format)),
        json_string(&report.path)
    )
}

fn gate_json(manifest: &ConformanceCase) -> String {
    let Some(gate) = manifest.gate.as_ref() else {
        return "null".to_string();
    };
    format!(
        "{{\"script\": {}, \"blocking\": {}}}",
        json_string(&gate.script),
        gate.blocking
    )
}

fn first_divergence_label(divergence: Option<&ep_compare::SeriesDivergenceV2>) -> String {
    let Some(divergence) = divergence else {
        return "none".to_string();
    };
    format!(
        "{} index={} timestamp={} expected={} observed={} abs_delta={}",
        divergence_kind_label(divergence.kind),
        divergence.index,
        divergence.timestamp.as_deref().unwrap_or("none"),
        optional_number_label(divergence.expected),
        optional_number_label(divergence.observed),
        optional_number_label(divergence.abs_delta)
    )
}

fn first_divergence_json(divergence: Option<&ep_compare::SeriesDivergenceV2>) -> String {
    let Some(divergence) = divergence else {
        return "null".to_string();
    };
    format!(
        "{{\"index\": {}, \"timestamp\": {}, \"kind\": {}, \"expected\": {}, \"observed\": {}, \"abs_delta\": {}, \"rel_delta\": {}}}",
        divergence.index,
        divergence
            .timestamp
            .as_ref()
            .map_or_else(|| "null".to_string(), |value| json_string(value)),
        json_string(divergence_kind_label(divergence.kind)),
        optional_number_json(divergence.expected),
        optional_number_json(divergence.observed),
        optional_number_json(divergence.abs_delta),
        optional_number_json(divergence.rel_delta)
    )
}

fn divergence_kind_label(kind: SeriesDivergenceKind) -> &'static str {
    match kind {
        SeriesDivergenceKind::Tolerance => "tolerance",
        SeriesDivergenceKind::MissingExpectedSample => "missing-expected-sample",
        SeriesDivergenceKind::MissingObservedSample => "missing-observed-sample",
    }
}

fn optional_number_label(value: Option<f64>) -> String {
    value.map_or_else(|| "none".to_string(), |value| format!("{value:.12}"))
}

fn optional_number_json(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), json_number)
}
