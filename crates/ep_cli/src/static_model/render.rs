use std::path::Path;

use ep_conformance::ConformanceCase;

use crate::conformance_artifacts::BaselineSummary;
use crate::{comparison_class_label, json_number, json_string, markdown_cell};

use super::{CLAIM_BOUNDARY, StaticModelReport};

pub(super) fn write_static_model_report(
    compare_dir: &Path,
    manifest: &ConformanceCase,
    baseline: &BaselineSummary,
    report: &StaticModelReport,
) -> Result<(), String> {
    std::fs::create_dir_all(compare_dir)
        .map_err(|error| format!("failed to create static report directory: {error}"))?;
    std::fs::write(
        compare_dir.join("compare-report.md"),
        render_static_model_report_markdown(manifest, baseline, report),
    )
    .map_err(|error| format!("failed to write static model markdown report: {error}"))?;
    std::fs::write(
        compare_dir.join("compare-summary.json"),
        render_static_model_report_json(manifest, baseline, report),
    )
    .map_err(|error| format!("failed to write static model JSON report: {error}"))?;
    Ok(())
}

fn render_static_model_report_markdown(
    manifest: &ConformanceCase,
    baseline: &BaselineSummary,
    report: &StaticModelReport,
) -> String {
    let mut output = String::new();
    output.push_str("# Static Model Conformance Report\n\n");
    output.push_str(&format!("case_id: {}\n", manifest.id));
    output.push_str(&format!("oracle_version: {}\n", manifest.oracle_version));
    output.push_str(&format!(
        "comparison_class: {}\n",
        comparison_class_label(manifest.comparison_class)
    ));
    output.push_str(&format!(
        "conformance_claim: {}\n",
        manifest.conformance_claim
    ));
    output.push_str(&format!("claim_boundary: {CLAIM_BOUNDARY}\n"));
    output.push_str("runtime_class: static-model\n");
    output.push_str(&format!(
        "baseline_dir: {}\n",
        baseline.output_dir.display()
    ));
    output.push_str(&format!("staged_idf: {}\n", baseline.idf.display()));
    output.push_str(&format!("oracle_eio: {}\n", baseline.eio.display()));
    output.push_str(&format!(
        "surface_details_injected: {}\n",
        baseline.injected_surface_details
    ));
    if let Some(report_contract) = manifest.report.as_ref() {
        output.push_str(&format!("report_path: {}\n", report_contract.path));
    }
    if let Some(gate) = manifest.gate.as_ref() {
        output.push_str(&format!("gate_script: {}\n", gate.script));
        output.push_str(&format!("gate_blocking: {}\n", gate.blocking));
    }
    output.push_str(&format!("status: {}\n\n", report.status));

    output.push_str("## Object Counts\n\n");
    output.push_str("| object | oracle | rust |\n");
    output.push_str("|---|---:|---:|\n");
    output.push_str(&format!(
        "| heat-transfer surfaces | {} | {} |\n",
        report.object_counts.oracle_surfaces, report.object_counts.surfaces
    ));
    output.push_str(&format!(
        "| constructions | {} | {} |\n",
        report.object_counts.oracle_constructions, report.object_counts.constructions
    ));
    output.push_str(&format!(
        "| materials | {} | {} |\n",
        report.object_counts.oracle_materials, report.object_counts.materials
    ));
    output.push_str(&format!(
        "| other equipment | {} | {} |\n\n",
        report.object_counts.oracle_other_equipment, report.object_counts.other_equipment
    ));

    output.push_str("## Output Comparisons\n\n");
    output.push_str("| key | variable | class | source | level | expected_objects | observed_objects | compared_objects | max_abs_delta | max_rel_delta | max_abs_tolerance | max_rel_tolerance | first_divergence | status |\n");
    output.push_str("|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|---|\n");
    for row in &report.rows {
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            markdown_cell(&row.key),
            markdown_cell(&row.variable),
            row.class,
            row.source,
            row.level,
            row.expected_objects,
            row.observed_objects,
            row.compared_objects,
            row.max_abs_delta,
            row.max_rel_delta,
            optional_number_label(row.max_abs_tolerance),
            optional_number_label(row.max_rel_tolerance),
            markdown_cell(row.first_divergence.as_deref().unwrap_or("none")),
            row.status
        ));
    }
    output
}

fn render_static_model_report_json(
    manifest: &ConformanceCase,
    baseline: &BaselineSummary,
    report: &StaticModelReport,
) -> String {
    let mut output = String::new();
    output.push_str("{\n");
    output.push_str("  \"schema_version\": 1,\n");
    output.push_str(&format!("  \"case_id\": {},\n", json_string(&manifest.id)));
    output.push_str(&format!(
        "  \"oracle_version\": {},\n",
        json_string(&manifest.oracle_version)
    ));
    output.push_str(&format!(
        "  \"comparison_class\": {},\n",
        json_string(comparison_class_label(manifest.comparison_class))
    ));
    output.push_str(&format!(
        "  \"conformance_claim\": {},\n",
        manifest.conformance_claim
    ));
    output.push_str("  \"runtime_class\": \"static-model\",\n");
    output.push_str(&format!(
        "  \"claim_boundary\": {},\n",
        json_string(CLAIM_BOUNDARY)
    ));
    output.push_str(&format!("  \"status\": {},\n", json_string(report.status)));
    output.push_str(&format!(
        "  \"baseline_dir\": {},\n",
        json_string(&baseline.output_dir.display().to_string())
    ));
    output.push_str(&format!(
        "  \"staged_idf\": {},\n",
        json_string(&baseline.idf.display().to_string())
    ));
    output.push_str(&format!(
        "  \"oracle_eio\": {},\n",
        json_string(&baseline.eio.display().to_string())
    ));
    output.push_str(&format!(
        "  \"surface_details_injected\": {},\n",
        baseline.injected_surface_details
    ));
    output.push_str(&format!(
        "  \"injected_outputs\": {},\n",
        baseline.injected_outputs
    ));
    output.push_str(&format!(
        "  \"injected_meters\": {},\n",
        baseline.injected_meters
    ));
    output.push_str("  \"object_counts\": {\n");
    output.push_str(&format!(
        "    \"surfaces\": {},\n",
        report.object_counts.surfaces
    ));
    output.push_str(&format!(
        "    \"oracle_surfaces\": {},\n",
        report.object_counts.oracle_surfaces
    ));
    output.push_str(&format!(
        "    \"constructions\": {},\n",
        report.object_counts.constructions
    ));
    output.push_str(&format!(
        "    \"oracle_constructions\": {},\n",
        report.object_counts.oracle_constructions
    ));
    output.push_str(&format!(
        "    \"materials\": {},\n",
        report.object_counts.materials
    ));
    output.push_str(&format!(
        "    \"oracle_materials\": {},\n",
        report.object_counts.oracle_materials
    ));
    output.push_str(&format!(
        "    \"other_equipment\": {},\n",
        report.object_counts.other_equipment
    ));
    output.push_str(&format!(
        "    \"oracle_other_equipment\": {}\n",
        report.object_counts.oracle_other_equipment
    ));
    output.push_str("  },\n");
    output.push_str("  \"rows\": [\n");
    for (index, row) in report.rows.iter().enumerate() {
        if index > 0 {
            output.push_str(",\n");
        }
        output.push_str("    {\n");
        output.push_str(&format!("      \"key\": {},\n", json_string(&row.key)));
        output.push_str(&format!(
            "      \"variable\": {},\n",
            json_string(&row.variable)
        ));
        output.push_str(&format!("      \"class\": {},\n", json_string(row.class)));
        output.push_str(&format!("      \"source\": {},\n", json_string(row.source)));
        output.push_str(&format!(
            "      \"frequency\": {},\n",
            json_string(row.frequency)
        ));
        output.push_str(&format!("      \"level\": {},\n", json_string(row.level)));
        output.push_str(&format!(
            "      \"expected_objects\": {},\n",
            row.expected_objects
        ));
        output.push_str(&format!(
            "      \"observed_objects\": {},\n",
            row.observed_objects
        ));
        output.push_str(&format!(
            "      \"compared_objects\": {},\n",
            row.compared_objects
        ));
        output.push_str(&format!(
            "      \"max_abs_delta\": {},\n",
            json_number(row.max_abs_delta)
        ));
        output.push_str(&format!(
            "      \"max_rel_delta\": {},\n",
            json_number(row.max_rel_delta)
        ));
        output.push_str(&format!(
            "      \"max_abs_tolerance\": {},\n",
            optional_json_number(row.max_abs_tolerance)
        ));
        output.push_str(&format!(
            "      \"max_rel_tolerance\": {},\n",
            optional_json_number(row.max_rel_tolerance)
        ));
        output.push_str(&format!(
            "      \"first_divergence\": {},\n",
            optional_json_string(row.first_divergence.as_deref())
        ));
        output.push_str(&format!("      \"status\": {}\n", json_string(row.status)));
        output.push_str("    }");
    }
    output.push_str("\n  ]\n");
    output.push_str("}\n");
    output
}

fn optional_number_label(value: Option<f64>) -> String {
    value.map_or_else(|| "none".to_string(), |number| format!("{number:.12}"))
}

fn optional_json_number(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), json_number)
}

fn optional_json_string(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_string(), json_string)
}
