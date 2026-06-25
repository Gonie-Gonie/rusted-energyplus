use std::path::Path;

use ep_conformance::ConformanceCase;

use crate::{comparison_class_label, json_string, markdown_cell};

pub(super) struct EnergyPlusErrSummary {
    pub(super) warning_count: usize,
    pub(super) severe_count: usize,
    pub(super) fatal_count: usize,
    pub(super) warnings: Vec<String>,
}

pub(super) struct ReportSeriesRow {
    pub(super) key: String,
    pub(super) variable: String,
    pub(super) frequency: &'static str,
    pub(super) variable_class: &'static str,
    pub(super) source: &'static str,
    pub(super) samples: usize,
    pub(super) first: String,
    pub(super) last: String,
    pub(super) min: String,
    pub(super) mean: String,
    pub(super) max: String,
    pub(super) nonzero_count: usize,
}

pub(super) fn render_report_skeleton(
    manifest: &ConformanceCase,
    rows: &[ReportSeriesRow],
    warning_summary: &EnergyPlusErrSummary,
) -> String {
    let mut report = String::new();
    report.push_str("# Conformance Report Skeleton\n\n");
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
    report.push_str("tolerance_policy: none\n");
    report.push_str("status: baseline-only\n\n");
    report.push_str("## EnergyPlus ERR\n\n");
    report.push_str(&format!(
        "energyplus_warnings: {}\n",
        warning_summary.warning_count
    ));
    report.push_str(&format!(
        "energyplus_severes: {}\n",
        warning_summary.severe_count
    ));
    report.push_str(&format!(
        "energyplus_fatals: {}\n\n",
        warning_summary.fatal_count
    ));
    if !warning_summary.warnings.is_empty() {
        report.push_str("| index | warning |\n");
        report.push_str("|---:|---|\n");
        for (index, warning) in warning_summary.warnings.iter().enumerate() {
            report.push_str(&format!("| {} | {} |\n", index + 1, markdown_cell(warning)));
        }
        report.push('\n');
    }
    report.push_str("## Series\n\n");
    report.push_str(
        "| key | variable | frequency | class | source | baseline_samples | first | last | baseline_min | baseline_mean | baseline_max | baseline_nonzero_count | status |\n",
    );
    report.push_str("|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|\n");
    for row in rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | baseline-only |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            row.frequency,
            row.variable_class,
            row.source,
            row.samples,
            row.first,
            row.last,
            row.min,
            row.mean,
            row.max,
            row.nonzero_count
        ));
    }
    report
}

pub(super) fn render_report_skeleton_summary_json(
    manifest: &ConformanceCase,
    rows: &[ReportSeriesRow],
    warning_summary: &EnergyPlusErrSummary,
) -> String {
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
    json.push_str("  \"tolerance_policy\": \"none\",\n");
    json.push_str("  \"status\": \"baseline-only\",\n");
    json.push_str("  \"artifacts\": {\n");
    json.push_str("    \"compare_report_md\": \"compare-report.md\",\n");
    json.push_str("    \"compare_summary_json\": \"compare-summary.json\"\n");
    json.push_str("  },\n");
    json.push_str("  \"energyplus_err\": {\n");
    json.push_str(&format!(
        "    \"warnings\": {},\n",
        warning_summary.warning_count
    ));
    json.push_str(&format!(
        "    \"severes\": {},\n",
        warning_summary.severe_count
    ));
    json.push_str(&format!(
        "    \"fatals\": {},\n",
        warning_summary.fatal_count
    ));
    json.push_str("    \"warning_messages\": [");
    for (index, warning) in warning_summary.warnings.iter().enumerate() {
        if index > 0 {
            json.push_str(", ");
        }
        json.push_str(&json_string(warning));
    }
    json.push_str("]\n");
    json.push_str("  },\n");
    json.push_str("  \"requested_outputs\": [\n");
    for (index, row) in rows.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"key\": {},\n", json_string(&row.key)));
        json.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&row.variable)
        ));
        json.push_str(&format!(
            "      \"frequency\": {},\n",
            json_string(row.frequency)
        ));
        json.push_str(&format!(
            "      \"class\": {},\n",
            json_string(row.variable_class)
        ));
        json.push_str(&format!("      \"source\": {},\n", json_string(row.source)));
        json.push_str(&format!("      \"baseline_samples\": {},\n", row.samples));
        json.push_str(&format!("      \"first\": {},\n", json_string(&row.first)));
        json.push_str(&format!("      \"last\": {},\n", json_string(&row.last)));
        json.push_str(&format!(
            "      \"baseline_min\": {},\n",
            json_string(&row.min)
        ));
        json.push_str(&format!(
            "      \"baseline_mean\": {},\n",
            json_string(&row.mean)
        ));
        json.push_str(&format!(
            "      \"baseline_max\": {},\n",
            json_string(&row.max)
        ));
        json.push_str(&format!(
            "      \"baseline_nonzero_count\": {},\n",
            row.nonzero_count
        ));
        json.push_str("      \"status\": \"baseline-only\"\n");
        if index + 1 == rows.len() {
            json.push_str("    }\n");
        } else {
            json.push_str("    },\n");
        }
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    json
}

pub(super) fn read_energyplus_err_summary(path: &Path) -> Result<EnergyPlusErrSummary, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read EnergyPlus ERR: {error}"))?;
    Ok(energyplus_err_summary(&contents))
}

fn energyplus_err_summary(contents: &str) -> EnergyPlusErrSummary {
    let mut warnings = Vec::new();
    let mut severe_count = 0;
    let mut fatal_count = 0;

    for line in contents.lines() {
        if line.contains("** Warning **") {
            warnings.push(clean_energyplus_message(line));
        } else if line.contains("** Severe  **") || line.contains("** Severe **") {
            severe_count += 1;
        } else if line.contains("** Fatal  **") || line.contains("** Fatal **") {
            fatal_count += 1;
        }
    }

    EnergyPlusErrSummary {
        warning_count: warnings.len(),
        severe_count,
        fatal_count,
        warnings,
    }
}

fn clean_energyplus_message(line: &str) -> String {
    line.replace("** Warning **", "").trim().to_string()
}
