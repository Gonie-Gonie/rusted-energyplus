use ep_compare::{
    load_eio_construction_ctf, load_eio_heat_transfer_surfaces, load_eio_material_ctf_summary,
    load_eio_other_equipment_nominal,
};
use ep_compiler::compile_raw_model;
use ep_conformance::{
    ComparisonClass, ConformanceCase, OutputFrequency, OutputLevel, OutputRequest, SourceArtifact,
    ToleranceRule, VariableClass,
};
use ep_model::TypedModel;
use ep_raw_model::load_epjson_file;
use ep_runtime::surface_geometry_summaries;
use std::path::{Path, PathBuf};

use crate::conformance_artifacts::{BaselineSummary, generate_conformance_baseline_in_dir};
use crate::{
    angle_abs_delta_deg, comparison_class_label, construction_material_rows, json_number,
    json_string, markdown_cell, other_equipment_nominal_rows, output_frequency_label,
    output_level_label, source_artifact_label, surface_type_label, variable_class_label,
};

const CLAIM_BOUNDARY: &str = "static EIO model evidence only; no dynamic heat-balance, HVAC, plant, solar, fenestration, warmup, sizing, or meter conformance";

pub(crate) struct StaticModelReportSummary {
    pub(crate) baseline: BaselineSummary,
    pub(crate) report_dir: PathBuf,
    pub(crate) compare_report: PathBuf,
    pub(crate) compare_summary: PathBuf,
    pub(crate) output_count: usize,
    pub(crate) conformance_output_count: usize,
    pub(crate) status: &'static str,
}

struct StaticModelReport {
    rows: Vec<StaticModelRow>,
    object_counts: StaticObjectCounts,
    status: &'static str,
}

#[derive(Default)]
struct StaticObjectCounts {
    surfaces: usize,
    oracle_surfaces: usize,
    constructions: usize,
    oracle_constructions: usize,
    materials: usize,
    oracle_materials: usize,
    other_equipment: usize,
    oracle_other_equipment: usize,
}

struct StaticModelRow {
    key: String,
    variable: String,
    class: &'static str,
    source: &'static str,
    frequency: &'static str,
    level: &'static str,
    compared_objects: usize,
    expected_objects: usize,
    observed_objects: usize,
    max_abs_delta: f64,
    max_rel_delta: f64,
    max_abs_tolerance: Option<f64>,
    max_rel_tolerance: Option<f64>,
    first_divergence: Option<String>,
    status: &'static str,
}

struct FieldComparison {
    compared_objects: usize,
    expected_objects: usize,
    observed_objects: usize,
    max_abs_delta: f64,
    max_rel_delta: f64,
    first_divergence: Option<String>,
}

impl FieldComparison {
    fn new(expected_objects: usize, observed_objects: usize) -> Self {
        let first_divergence = if expected_objects == observed_objects {
            None
        } else {
            Some(format!(
                "object_count expected {expected_objects} observed {observed_objects}"
            ))
        };
        Self {
            compared_objects: 0,
            expected_objects,
            observed_objects,
            max_abs_delta: 0.0,
            max_rel_delta: 0.0,
            first_divergence,
        }
    }

    fn record_numeric(&mut self, expected: f64, observed: f64) {
        self.compared_objects += 1;
        let delta = (expected - observed).abs();
        let scale = expected.abs().max(observed.abs());
        let relative_delta = if scale > 0.0 { delta / scale } else { 0.0 };
        self.max_abs_delta = self.max_abs_delta.max(delta);
        self.max_rel_delta = self.max_rel_delta.max(relative_delta);
    }

    fn record_angle(&mut self, expected: f64, observed: f64) {
        self.compared_objects += 1;
        let delta = angle_abs_delta_deg(expected, observed);
        let scale = expected.abs().max(observed.abs());
        let relative_delta = if scale > 0.0 { delta / scale } else { 0.0 };
        self.max_abs_delta = self.max_abs_delta.max(delta);
        self.max_rel_delta = self.max_rel_delta.max(relative_delta);
    }

    fn record_string(&mut self, expected: &str, observed: &str) {
        self.compared_objects += 1;
        self.record_identity(expected, observed);
    }

    fn record_identity(&mut self, expected: &str, observed: &str) {
        if !expected.eq_ignore_ascii_case(observed) && self.first_divergence.is_none() {
            self.first_divergence = Some(format!("string expected {expected} observed {observed}"));
        }
    }

    fn record_missing(&mut self, label: &str) {
        if self.first_divergence.is_none() {
            self.first_divergence = Some(format!("{label} missing_in_oracle"));
        }
    }

    fn passed(&self, max_abs_tolerance: Option<f64>, max_rel_tolerance: Option<f64>) -> bool {
        if self.expected_objects != self.observed_objects {
            return false;
        }
        if self.compared_objects != self.observed_objects {
            return false;
        }
        if max_abs_tolerance.is_some_and(|tolerance| self.max_abs_delta > tolerance) {
            return false;
        }
        if max_rel_tolerance.is_some_and(|tolerance| self.max_rel_delta > tolerance) {
            return false;
        }
        self.first_divergence.is_none()
    }
}

pub(crate) fn generate_static_model_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<StaticModelReportSummary, String> {
    validate_static_model_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");
    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let report = build_static_model_report(manifest, &baseline)?;
    write_static_model_report(&compare_dir, manifest, &baseline, &report)?;

    Ok(StaticModelReportSummary {
        baseline,
        report_dir: compare_dir.clone(),
        compare_report: compare_dir.join("compare-report.md"),
        compare_summary: compare_dir.join("compare-summary.json"),
        output_count: report.rows.len(),
        conformance_output_count: report
            .rows
            .iter()
            .filter(|row| row.level == "conformance")
            .count(),
        status: report.status,
    })
}

fn validate_static_model_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    if manifest.comparison_class != ComparisonClass::Conformance {
        return Err(format!(
            "static model report requires comparison_class conformance, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if !manifest.conformance_claim {
        return Err("static model report requires conformance_claim true".to_string());
    }
    if manifest.outputs.is_empty() {
        return Err("static model report requires at least one output request".to_string());
    }
    for output in &manifest.outputs {
        if output.frequency != OutputFrequency::Static {
            return Err(format!(
                "static model report requires static output frequency, got {} for {}",
                output_frequency_label(output.frequency),
                output.variable
            ));
        }
        if output.source != SourceArtifact::Eio {
            return Err(format!(
                "static model report requires eio output source, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        if output.level != Some(OutputLevel::Conformance) {
            return Err(format!(
                "static model report requires conformance output level for {}",
                output.variable
            ));
        }
        match output.class {
            VariableClass::SurfaceState
            | VariableClass::ConstructionMaterial
            | VariableClass::InternalGain => {}
            class => {
                return Err(format!(
                    "static model report does not support {} output class for {}",
                    variable_class_label(class),
                    output.variable
                ));
            }
        }
    }
    for class in [
        VariableClass::SurfaceState,
        VariableClass::ConstructionMaterial,
        VariableClass::InternalGain,
    ] {
        if manifest.outputs.iter().any(|output| output.class == class)
            && tolerance_for_class(manifest, class).is_none()
        {
            return Err(format!(
                "static model report requires a tolerance rule for {}",
                variable_class_label(class)
            ));
        }
    }
    let Some(report) = manifest.report.as_ref() else {
        return Err("static model report requires a report contract".to_string());
    };
    if report.path.trim().is_empty() {
        return Err("static model report contract has an empty path".to_string());
    }
    let Some(gate) = manifest.gate.as_ref() else {
        return Err("static model report requires a blocking gate contract".to_string());
    };
    if gate.script.trim().is_empty() {
        return Err("static model report gate contract has an empty script".to_string());
    }
    if !gate.blocking {
        return Err("static model report gate must be blocking".to_string());
    }
    Ok(())
}

fn build_static_model_report(
    manifest: &ConformanceCase,
    baseline: &BaselineSummary,
) -> Result<StaticModelReport, String> {
    let raw_model = load_epjson_file(&baseline.epjson)
        .map_err(|error| format!("failed to load staged epJSON: {error}"))?;
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model else {
        return Err(format!(
            "failed to compile staged epJSON: {} diagnostics",
            result.report.diagnostics.len()
        ));
    };

    let mut object_counts = StaticObjectCounts::default();
    let mut rows = Vec::new();
    for output in &manifest.outputs {
        let row = match output.class {
            VariableClass::SurfaceState => {
                compare_surface_output(output, manifest, baseline, &model)?
            }
            VariableClass::ConstructionMaterial => {
                compare_construction_material_output(output, manifest, baseline, &model)?
            }
            VariableClass::InternalGain => {
                compare_internal_gain_output(output, manifest, baseline, &model)?
            }
            class => {
                return Err(format!(
                    "unsupported static output class {} for {}",
                    variable_class_label(class),
                    output.variable
                ));
            }
        };
        rows.push(row);
    }

    if manifest
        .outputs
        .iter()
        .any(|output| output.class == VariableClass::SurfaceState)
    {
        object_counts.surfaces = surface_geometry_summaries(&model).len();
        object_counts.oracle_surfaces = load_eio_heat_transfer_surfaces(&baseline.eio)
            .map_err(|error| format!("failed to load surface EIO rows: {error}"))?
            .len();
    }
    if manifest
        .outputs
        .iter()
        .any(|output| output.class == VariableClass::ConstructionMaterial)
    {
        let construction_rows = construction_material_rows(&model)?;
        object_counts.constructions = construction_rows.len();
        object_counts.materials = construction_rows
            .iter()
            .map(|row| row.outside_layer_material_name.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        object_counts.oracle_constructions = load_eio_construction_ctf(&baseline.eio)
            .map_err(|error| format!("failed to load construction EIO rows: {error}"))?
            .len();
        object_counts.oracle_materials = load_eio_material_ctf_summary(&baseline.eio)
            .map_err(|error| format!("failed to load material EIO rows: {error}"))?
            .len();
    }
    if manifest
        .outputs
        .iter()
        .any(|output| output.class == VariableClass::InternalGain)
    {
        object_counts.other_equipment = other_equipment_nominal_rows(&model).len();
        object_counts.oracle_other_equipment = load_eio_other_equipment_nominal(&baseline.eio)
            .map_err(|error| format!("failed to load OtherEquipment EIO rows: {error}"))?
            .len();
    }

    let status = if rows.iter().all(|row| row.status == "pass") {
        "pass"
    } else {
        "fail"
    };

    Ok(StaticModelReport {
        rows,
        object_counts,
        status,
    })
}

fn compare_surface_output(
    output: &OutputRequest,
    manifest: &ConformanceCase,
    baseline: &BaselineSummary,
    model: &TypedModel,
) -> Result<StaticModelRow, String> {
    let rust_surfaces: Vec<_> = surface_geometry_summaries(model)
        .into_iter()
        .filter(|surface| key_matches(&output.key, &surface.surface_name))
        .collect();
    let oracle_surfaces: Vec<_> = load_eio_heat_transfer_surfaces(&baseline.eio)
        .map_err(|error| format!("failed to load surface EIO rows: {error}"))?
        .into_iter()
        .filter(|surface| key_matches(&output.key, &surface.surface_name))
        .collect();
    let mut comparison = FieldComparison::new(oracle_surfaces.len(), rust_surfaces.len());
    for rust_surface in &rust_surfaces {
        let Some(oracle_surface) = oracle_surfaces.iter().find(|surface| {
            surface
                .surface_name
                .eq_ignore_ascii_case(&rust_surface.surface_name)
        }) else {
            comparison.record_missing(&format!("surface {}", rust_surface.surface_name));
            continue;
        };
        match output.variable.as_str() {
            "HeatTransfer Surface Class" => comparison.record_string(
                &oracle_surface.surface_class,
                surface_type_label(rust_surface.surface_type),
            ),
            "HeatTransfer Surface Area (Net)" => {
                comparison.record_numeric(oracle_surface.area_net_m2, rust_surface.area_m2);
            }
            "HeatTransfer Surface Area (Gross)" => {
                comparison.record_numeric(oracle_surface.area_gross_m2, rust_surface.area_m2);
            }
            "HeatTransfer Surface Azimuth" => {
                comparison.record_angle(oracle_surface.azimuth_deg, rust_surface.azimuth_deg);
            }
            "HeatTransfer Surface Tilt" => {
                comparison.record_angle(oracle_surface.tilt_deg, rust_surface.tilt_deg);
            }
            variable => return Err(format!("unsupported surface static variable: {variable}")),
        }
    }
    Ok(row_from_comparison(output, manifest, comparison))
}

fn compare_construction_material_output(
    output: &OutputRequest,
    manifest: &ConformanceCase,
    baseline: &BaselineSummary,
    model: &TypedModel,
) -> Result<StaticModelRow, String> {
    let rust_rows: Vec<_> = construction_material_rows(model)?
        .into_iter()
        .filter(|row| key_matches(&output.key, &row.construction_name))
        .collect();
    let oracle_constructions = load_eio_construction_ctf(&baseline.eio)
        .map_err(|error| format!("failed to load construction EIO rows: {error}"))?;
    let oracle_materials = load_eio_material_ctf_summary(&baseline.eio)
        .map_err(|error| format!("failed to load material EIO rows: {error}"))?;
    let oracle_count = oracle_constructions
        .iter()
        .filter(|row| key_matches(&output.key, &row.construction_name))
        .count();
    let mut comparison = FieldComparison::new(oracle_count, rust_rows.len());
    for rust_row in &rust_rows {
        let Some(oracle_construction) = oracle_constructions.iter().find(|row| {
            row.construction_name
                .eq_ignore_ascii_case(&rust_row.construction_name)
        }) else {
            comparison.record_missing(&format!("construction {}", rust_row.construction_name));
            continue;
        };
        let Some(oracle_material) = oracle_materials.iter().find(|row| {
            row.material_name
                .eq_ignore_ascii_case(&rust_row.outside_layer_material_name)
        }) else {
            comparison.record_missing(&format!(
                "material {}",
                rust_row.outside_layer_material_name
            ));
            continue;
        };
        match output.variable.as_str() {
            "Construction CTF Layer Count" => comparison.record_numeric(
                oracle_construction.layer_count as f64,
                rust_row.layer_count as f64,
            ),
            "Construction CTF Thermal Conductance" => comparison.record_numeric(
                oracle_construction.thermal_conductance_w_per_m2_k,
                rust_row.thermal_conductance_w_per_m2_k,
            ),
            "Material CTF Summary Thickness" => comparison.record_numeric(
                oracle_material.thickness_m,
                rust_row.material_thickness_m.unwrap_or(0.0),
            ),
            "Material CTF Summary Conductivity" => comparison.record_numeric(
                oracle_material.conductivity_w_per_m_k,
                rust_row.material_conductivity_w_per_m_k.unwrap_or(0.0),
            ),
            "Material CTF Summary Density" => comparison.record_numeric(
                oracle_material.density_kg_per_m3,
                rust_row.material_density_kg_per_m3.unwrap_or(0.0),
            ),
            "Material CTF Summary Specific Heat" => comparison.record_numeric(
                oracle_material.specific_heat_j_per_kg_k,
                rust_row.material_specific_heat_j_per_kg_k.unwrap_or(0.0),
            ),
            "Material CTF Summary Thermal Resistance" => comparison.record_numeric(
                oracle_material.thermal_resistance_m2_k_per_w,
                rust_row.material_thermal_resistance_m2_k_per_w,
            ),
            variable => {
                return Err(format!(
                    "unsupported construction/material static variable: {variable}"
                ));
            }
        }
    }
    Ok(row_from_comparison(output, manifest, comparison))
}

fn compare_internal_gain_output(
    output: &OutputRequest,
    manifest: &ConformanceCase,
    baseline: &BaselineSummary,
    model: &TypedModel,
) -> Result<StaticModelRow, String> {
    let rust_rows: Vec<_> = other_equipment_nominal_rows(model)
        .into_iter()
        .filter(|row| key_matches(&output.key, &row.equipment_name))
        .collect();
    let oracle_rows = load_eio_other_equipment_nominal(&baseline.eio)
        .map_err(|error| format!("failed to load OtherEquipment EIO rows: {error}"))?;
    let oracle_count = oracle_rows
        .iter()
        .filter(|row| key_matches(&output.key, &row.equipment_name))
        .count();
    let mut comparison = FieldComparison::new(oracle_count, rust_rows.len());
    for rust_row in &rust_rows {
        let Some(oracle_row) = oracle_rows.iter().find(|row| {
            row.equipment_name
                .eq_ignore_ascii_case(&rust_row.equipment_name)
        }) else {
            comparison.record_missing(&format!("other_equipment {}", rust_row.equipment_name));
            continue;
        };
        comparison.record_identity(&oracle_row.zone_name, &rust_row.zone_name);
        comparison.record_identity(&oracle_row.schedule_name, &rust_row.schedule_name);
        match output.variable.as_str() {
            "OtherEquipment Internal Gains Nominal Zone Floor Area" => {
                comparison
                    .record_numeric(oracle_row.zone_floor_area_m2, rust_row.zone_floor_area_m2);
            }
            "OtherEquipment Internal Gains Nominal Equipment Level" => {
                comparison.record_numeric(oracle_row.equipment_level_w, rust_row.equipment_level_w);
            }
            "OtherEquipment Internal Gains Nominal Equipment per Floor Area" => comparison
                .record_numeric(
                    oracle_row.equipment_per_floor_area_w_per_m2,
                    rust_row.equipment_per_floor_area_w_per_m2,
                ),
            "OtherEquipment Internal Gains Nominal Fraction Latent" => {
                comparison.record_numeric(oracle_row.fraction_latent, rust_row.fraction_latent);
            }
            "OtherEquipment Internal Gains Nominal Fraction Radiant" => {
                comparison.record_numeric(oracle_row.fraction_radiant, rust_row.fraction_radiant);
            }
            "OtherEquipment Internal Gains Nominal Fraction Lost" => {
                comparison.record_numeric(oracle_row.fraction_lost, rust_row.fraction_lost);
            }
            "OtherEquipment Internal Gains Nominal Fraction Convected" => {
                comparison
                    .record_numeric(oracle_row.fraction_convected, rust_row.fraction_convected);
            }
            variable => {
                return Err(format!(
                    "unsupported internal-gain static variable: {variable}"
                ));
            }
        }
    }
    Ok(row_from_comparison(output, manifest, comparison))
}

fn row_from_comparison(
    output: &OutputRequest,
    manifest: &ConformanceCase,
    mut comparison: FieldComparison,
) -> StaticModelRow {
    let tolerance_rule = tolerance_for_class(manifest, output.class);
    let max_abs_tolerance = output
        .abs_tol
        .or_else(|| tolerance_rule.and_then(|rule| rule.max_abs));
    let max_rel_tolerance = output
        .rel_tol
        .or_else(|| tolerance_rule.and_then(|rule| rule.max_rel));
    let status = if comparison.passed(max_abs_tolerance, max_rel_tolerance) {
        "pass"
    } else {
        if comparison.first_divergence.is_none() {
            comparison.first_divergence = Some(tolerance_failure_label(
                comparison.max_abs_delta,
                max_abs_tolerance,
                comparison.max_rel_delta,
                max_rel_tolerance,
            ));
        }
        "fail"
    };
    StaticModelRow {
        key: output.key.clone(),
        variable: output.variable.clone(),
        class: variable_class_label(output.class),
        source: source_artifact_label(output.source),
        frequency: output_frequency_label(output.frequency),
        level: output
            .level
            .map(output_level_label)
            .unwrap_or("unspecified"),
        compared_objects: comparison.compared_objects,
        expected_objects: comparison.expected_objects,
        observed_objects: comparison.observed_objects,
        max_abs_delta: comparison.max_abs_delta,
        max_rel_delta: comparison.max_rel_delta,
        max_abs_tolerance,
        max_rel_tolerance,
        first_divergence: comparison.first_divergence,
        status,
    }
}

fn tolerance_failure_label(
    max_abs_delta: f64,
    max_abs_tolerance: Option<f64>,
    max_rel_delta: f64,
    max_rel_tolerance: Option<f64>,
) -> String {
    if max_abs_tolerance.is_some_and(|tolerance| max_abs_delta > tolerance) {
        return format!(
            "max_abs_delta {:.12} exceeded tolerance {:.12}",
            max_abs_delta,
            max_abs_tolerance.unwrap_or(0.0)
        );
    }
    if max_rel_tolerance.is_some_and(|tolerance| max_rel_delta > tolerance) {
        return format!(
            "max_rel_delta {:.12} exceeded tolerance {:.12}",
            max_rel_delta,
            max_rel_tolerance.unwrap_or(0.0)
        );
    }
    "static comparison failed".to_string()
}

fn tolerance_for_class(manifest: &ConformanceCase, class: VariableClass) -> Option<ToleranceRule> {
    manifest
        .tolerances
        .iter()
        .copied()
        .find(|tolerance| tolerance.variable_class == class)
}

fn key_matches(key: &str, object_name: &str) -> bool {
    key.trim() == "*" || key.trim().eq_ignore_ascii_case(object_name)
}

fn write_static_model_report(
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
