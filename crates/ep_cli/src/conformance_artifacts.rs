use ep_compare::load_eso_series;
use ep_conformance::{ConformanceCase, OutputRegistry, SourceArtifact};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::{
    comparison_class_label, first_value_label, json_string, last_value_label, markdown_cell,
    max_value_label, mean_value_label, min_value_label, nonzero_count, output_frequency_idf_label,
    output_frequency_label, report_format_label, resolve_manifest_path, source_artifact_label,
    variable_class_label,
};

pub(crate) struct BaselineSummary {
    pub(crate) output_dir: PathBuf,
    pub(crate) idf: PathBuf,
    pub(crate) weather: Option<PathBuf>,
    pub(crate) epjson: PathBuf,
    pub(crate) eso: PathBuf,
    pub(crate) eio: PathBuf,
    pub(crate) expanded_manifest: PathBuf,
    pub(crate) injected_outputs: usize,
    pub(crate) injected_meters: usize,
    pub(crate) injected_surface_details: bool,
}

pub(crate) struct ReportSkeletonSummary {
    pub(crate) report_path: PathBuf,
    pub(crate) series: usize,
    pub(crate) warning_count: usize,
    pub(crate) severe_count: usize,
    pub(crate) fatal_count: usize,
}

pub(crate) fn generate_conformance_baseline(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<BaselineSummary, String> {
    generate_conformance_baseline_in_dir(
        case_path,
        manifest,
        oracle_root,
        &output_root.join(&manifest.id),
    )
}

pub(crate) fn generate_conformance_baseline_in_dir(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_dir: &Path,
) -> Result<BaselineSummary, String> {
    let energyplus = oracle_root.join("energyplus.exe");
    if !energyplus.is_file() {
        return Err(format!(
            "missing EnergyPlus executable: {}",
            energyplus.display()
        ));
    }
    let converter = oracle_root.join("ConvertInputFormat.exe");
    if !converter.is_file() {
        return Err(format!("missing IDF converter: {}", converter.display()));
    }

    let source_idf = resolve_manifest_path(case_path, &manifest.input.idf)
        .map_err(|error| format!("failed to resolve input.idf: {error}"))?;
    if !source_idf.is_file() {
        return Err(format!("missing case IDF: {}", source_idf.display()));
    }
    let source_weather = match manifest.input.weather.as_deref() {
        Some(weather) => {
            let resolved = resolve_manifest_path(case_path, weather)
                .map_err(|error| format!("failed to resolve input.weather: {error}"))?;
            if !resolved.is_file() {
                return Err(format!("missing case weather: {}", resolved.display()));
            }
            Some(resolved)
        }
        None => None,
    };

    std::fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create baseline output directory: {error}"))?;
    let input_idf = output_dir.join("input.idf");
    let injection = stage_idf_with_output_requests(&source_idf, &input_idf, manifest)?;

    let mut energyplus_command = Command::new(&energyplus);
    if let Some(weather) = source_weather.as_ref() {
        energyplus_command.arg("-w").arg(weather);
    }
    energyplus_command
        .arg("-d")
        .arg(output_dir)
        .arg(&input_idf)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let energyplus_status = energyplus_command
        .status()
        .map_err(|error| format!("failed to start EnergyPlus: {error}"))?;
    if !energyplus_status.success() {
        return Err(format!(
            "EnergyPlus baseline failed with status {energyplus_status}"
        ));
    }

    let converter_status = Command::new(&converter)
        .arg("input.idf")
        .current_dir(output_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("failed to start IDF converter: {error}"))?;
    if !converter_status.success() {
        return Err(format!(
            "IDF conversion failed with status {converter_status}"
        ));
    }

    let eso = output_dir.join("eplusout.eso");
    if !eso.is_file() {
        return Err(format!("EnergyPlus did not write {}", eso.display()));
    }
    let eio = output_dir.join("eplusout.eio");
    if !eio.is_file() {
        return Err(format!("EnergyPlus did not write {}", eio.display()));
    }
    let err = output_dir.join("eplusout.err");
    if !err.is_file() {
        return Err(format!("EnergyPlus did not write {}", err.display()));
    }
    let epjson = output_dir.join("input.epJSON");
    if !epjson.is_file() {
        return Err(format!("IDF converter did not write {}", epjson.display()));
    }
    let expanded_manifest = output_dir.join("case-expanded.toml");
    std::fs::write(
        &expanded_manifest,
        render_expanded_case_manifest(manifest, source_weather.as_deref(), &injection),
    )
    .map_err(|error| format!("failed to write expanded case manifest: {error}"))?;

    Ok(BaselineSummary {
        output_dir: output_dir.to_path_buf(),
        idf: input_idf,
        weather: source_weather,
        epjson,
        eso,
        eio,
        expanded_manifest,
        injected_outputs: injection.outputs,
        injected_meters: injection.meters,
        injected_surface_details: injection.surface_details,
    })
}

struct OutputInjectionSummary {
    outputs: usize,
    meters: usize,
    surface_details: bool,
}

fn stage_idf_with_output_requests(
    source_idf: &Path,
    staged_idf: &Path,
    manifest: &ConformanceCase,
) -> Result<OutputInjectionSummary, String> {
    let mut idf = std::fs::read_to_string(source_idf)
        .map_err(|error| format!("failed to read case IDF for output injection: {error}"))?;
    let injection = render_output_request_injection(manifest, &idf);
    if !injection.text.is_empty() {
        if !idf.ends_with('\n') {
            idf.push('\n');
        }
        idf.push_str(&injection.text);
    }
    std::fs::write(staged_idf, idf)
        .map_err(|error| format!("failed to stage case IDF with output requests: {error}"))?;
    Ok(OutputInjectionSummary {
        outputs: injection.outputs,
        meters: injection.meters,
        surface_details: injection.surface_details,
    })
}

struct RenderedOutputInjection {
    text: String,
    outputs: usize,
    meters: usize,
    surface_details: bool,
}

fn render_output_request_injection(
    manifest: &ConformanceCase,
    existing_idf: &str,
) -> RenderedOutputInjection {
    let mut text = String::new();
    let existing_outputs = existing_output_variables(existing_idf);
    let existing_meters = existing_output_meters(existing_idf);
    let existing_surface_details = has_existing_output_surfaces_details(existing_idf);
    let requested_output_count = manifest
        .outputs
        .iter()
        .filter(|output| output.source == SourceArtifact::Eso)
        .count();
    let requested_meter_count = manifest.meters.len();
    let requested_surface_details = manifest.outputs.iter().any(|output| {
        output.source == SourceArtifact::Eio
            && output
                .variable
                .trim()
                .to_ascii_lowercase()
                .starts_with("heattransfer surface")
    });

    if requested_output_count == 0 && requested_meter_count == 0 && !requested_surface_details {
        return RenderedOutputInjection {
            text,
            outputs: 0,
            meters: 0,
            surface_details: false,
        };
    }

    text.push_str("\n!- eplus-rs output request injection begin\n");
    text.push_str(&format!("!- case_id: {}\n", manifest.id));
    text.push_str("!- source: case manifest outputs/meters\n");
    let mut output_count = 0;
    let mut meter_count = 0;
    let mut surface_details = false;
    if requested_surface_details && !existing_surface_details {
        text.push_str("Output:Surfaces:List,Details;\n\n");
        surface_details = true;
    }
    for output in &manifest.outputs {
        if output.source != SourceArtifact::Eso {
            continue;
        }
        if has_existing_output_variable(&existing_outputs, &output.key, &output.variable) {
            continue;
        }
        if output_count == 0 {
            text.push_str("Output:VariableDictionary,Regular;\n\n");
        }
        text.push_str("Output:Variable,\n");
        text.push_str(&format!("  {},  !- Key Value\n", idf_field(&output.key)));
        text.push_str(&format!(
            "  {},  !- Variable Name\n",
            idf_field(&output.variable)
        ));
        text.push_str(&format!(
            "  {};  !- Reporting Frequency\n\n",
            output_frequency_idf_label(output.frequency)
        ));
        output_count += 1;
    }
    for meter in &manifest.meters {
        if has_existing_output_meter(&existing_meters, &meter.name) {
            continue;
        }
        text.push_str("Output:Meter,\n");
        text.push_str(&format!("  {},  !- Key Name\n", idf_field(&meter.name)));
        text.push_str(&format!(
            "  {};  !- Reporting Frequency\n\n",
            output_frequency_idf_label(meter.frequency)
        ));
        meter_count += 1;
    }
    if output_count == 0 && meter_count == 0 && !surface_details {
        text.push_str("!- no new output requests; staged IDF already contains manifest requests\n");
    }
    text.push_str("!- eplus-rs output request injection end\n");

    RenderedOutputInjection {
        text,
        outputs: output_count,
        meters: meter_count,
        surface_details,
    }
}

fn existing_output_variables(idf: &str) -> Vec<(String, String)> {
    idf_objects(idf, "Output:Variable")
        .into_iter()
        .filter_map(|fields| {
            if fields.len() >= 3 {
                Some((fields[1].clone(), fields[2].clone()))
            } else {
                None
            }
        })
        .collect()
}

fn existing_output_meters(idf: &str) -> Vec<String> {
    idf_objects(idf, "Output:Meter")
        .into_iter()
        .filter_map(|fields| {
            if fields.len() >= 2 {
                Some(fields[1].clone())
            } else {
                None
            }
        })
        .collect()
}

fn has_existing_output_surfaces_details(idf: &str) -> bool {
    idf_objects(idf, "Output:Surfaces:List")
        .into_iter()
        .any(|fields| {
            fields
                .get(1)
                .is_some_and(|field| field.eq_ignore_ascii_case("details"))
        })
}

fn idf_objects(idf: &str, object_type: &str) -> Vec<Vec<String>> {
    let object_type = normalize_idf_request_field(object_type);
    let mut objects = Vec::new();
    let mut current = String::new();

    for line in idf.lines() {
        let content = line.split('!').next().unwrap_or("").trim();
        if content.is_empty() {
            continue;
        }
        current.push_str(content);
        current.push(' ');
        if content.contains(';') {
            let fields: Vec<String> = current
                .replace(';', ",")
                .split(',')
                .map(normalize_idf_request_field)
                .filter(|field| !field.is_empty())
                .collect();
            if fields
                .first()
                .is_some_and(|field| field.as_str() == object_type)
            {
                objects.push(fields);
            }
            current.clear();
        }
    }

    objects
}

fn has_existing_output_variable(existing: &[(String, String)], key: &str, variable: &str) -> bool {
    let key = normalize_idf_request_field(key);
    let variable = normalize_idf_request_field(variable);
    existing.iter().any(|(existing_key, existing_variable)| {
        (existing_key == "*" || existing_key == &key) && existing_variable == &variable
    })
}

fn has_existing_output_meter(existing: &[String], name: &str) -> bool {
    let name = normalize_idf_request_field(name);
    existing.iter().any(|existing_name| existing_name == &name)
}

fn normalize_idf_request_field(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn idf_field(value: &str) -> String {
    value
        .replace(['\r', '\n'], " ")
        .replace([';', ','], " ")
        .trim()
        .to_string()
}

fn render_expanded_case_manifest(
    manifest: &ConformanceCase,
    source_weather: Option<&Path>,
    injection: &OutputInjectionSummary,
) -> String {
    let mut toml = String::new();
    toml.push_str("# Generated by eplus-rs conformance baseline.\n");
    toml.push_str("schema = \"rusted-energyplus.baseline-expanded.v1\"\n");
    push_toml_string_field(&mut toml, "id", &manifest.id);
    push_toml_string_field(&mut toml, "title", &manifest.title);
    push_toml_string_field(&mut toml, "milestone", &manifest.milestone);
    push_toml_string_field(
        &mut toml,
        "comparison_class",
        comparison_class_label(manifest.comparison_class),
    );
    toml.push_str(&format!(
        "conformance_claim = {}\n",
        manifest.conformance_claim
    ));
    push_toml_string_field(&mut toml, "oracle_version", &manifest.oracle_version);
    toml.push('\n');

    toml.push_str("[input]\n");
    push_toml_string_field(&mut toml, "source_idf", &manifest.input.idf);
    push_toml_string_field(&mut toml, "staged_idf", "input.idf");
    if let Some(weather) = source_weather {
        push_toml_string_field(&mut toml, "source_weather", &weather.display().to_string());
    }
    push_toml_string_field(&mut toml, "converted_epjson", "input.epJSON");
    toml.push('\n');

    toml.push_str("[output_injection]\n");
    toml.push_str("schema = \"rusted-energyplus.output-injection.v1\"\n");
    toml.push_str("staged_idf_contains_manifest_requests = true\n");
    toml.push_str(&format!("outputs = {}\n", injection.outputs));
    toml.push_str(&format!("meters = {}\n", injection.meters));
    toml.push_str(&format!(
        "surface_details = {}\n\n",
        injection.surface_details
    ));

    toml.push_str("[artifacts]\n");
    push_toml_string_field(&mut toml, "err", "eplusout.err");
    push_toml_string_field(&mut toml, "eso", "eplusout.eso");
    push_toml_string_field(&mut toml, "eio", "eplusout.eio");
    push_toml_string_field(&mut toml, "rdd", "eplusout.rdd");
    push_toml_string_field(&mut toml, "mdd", "eplusout.mdd");
    push_toml_string_field(&mut toml, "expanded_manifest", "case-expanded.toml");
    toml.push('\n');

    for output in &manifest.outputs {
        toml.push_str("[[outputs]]\n");
        push_toml_string_field(&mut toml, "key", &output.key);
        push_toml_string_field(&mut toml, "variable", &output.variable);
        push_toml_string_field(
            &mut toml,
            "frequency",
            output_frequency_label(output.frequency),
        );
        push_toml_string_field(&mut toml, "class", variable_class_label(output.class));
        push_toml_string_field(&mut toml, "source", source_artifact_label(output.source));
        toml.push('\n');
    }

    if let Some(report) = manifest.report.as_ref() {
        toml.push_str("[report]\n");
        push_toml_string_field(&mut toml, "format", report_format_label(report.format));
        push_toml_string_field(&mut toml, "path", &report.path);
        toml.push('\n');
    }
    if let Some(gate) = manifest.gate.as_ref() {
        toml.push_str("[gate]\n");
        push_toml_string_field(&mut toml, "script", &gate.script);
        toml.push_str(&format!("blocking = {}\n", gate.blocking));
    }

    toml
}

fn push_toml_string_field(output: &mut String, key: &str, value: &str) {
    output.push_str(key);
    output.push_str(" = ");
    output.push_str(&json_string(value));
    output.push('\n');
}

pub(crate) fn generate_conformance_report_skeleton(
    manifest: &ConformanceCase,
    baseline_case_dir: &Path,
    report_root: &Path,
) -> Result<ReportSkeletonSummary, String> {
    let eso = baseline_case_dir.join("eplusout.eso");
    if !eso.is_file() {
        return Err(format!("missing baseline ESO: {}", eso.display()));
    }
    let err = baseline_case_dir.join("eplusout.err");
    if !err.is_file() {
        return Err(format!("missing baseline ERR: {}", err.display()));
    }
    let warning_summary = read_energyplus_err_summary(&err)?;

    let report_dir = report_root.join(&manifest.id);
    std::fs::create_dir_all(&report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;
    let report_path = report_dir.join("compare-report.md");
    let summary_path = report_dir.join("compare-summary.json");

    let registry = OutputRegistry::from_case(manifest)
        .map_err(|error| format!("invalid registry: {error}"))?;
    let mut rows = Vec::new();
    for output in registry.series() {
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "report skeleton currently supports eso output sources, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        let values = load_eso_series(&eso, &output.key, &output.variable)
            .map_err(|error| format!("failed to load baseline series: {error}"))?;
        rows.push(ReportSeriesRow {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output_frequency_label(output.frequency),
            variable_class: variable_class_label(output.class),
            source: source_artifact_label(output.source),
            samples: values.len(),
            first: first_value_label(&values),
            last: last_value_label(&values),
            min: min_value_label(&values),
            mean: mean_value_label(&values),
            max: max_value_label(&values),
            nonzero_count: nonzero_count(&values),
        });
    }

    let report = render_report_skeleton(manifest, &rows, &warning_summary);
    std::fs::write(&report_path, report)
        .map_err(|error| format!("failed to write report skeleton: {error}"))?;
    std::fs::write(
        &summary_path,
        render_report_skeleton_summary_json(manifest, &rows, &warning_summary),
    )
    .map_err(|error| format!("failed to write report summary: {error}"))?;

    Ok(ReportSkeletonSummary {
        report_path,
        series: rows.len(),
        warning_count: warning_summary.warning_count,
        severe_count: warning_summary.severe_count,
        fatal_count: warning_summary.fatal_count,
    })
}

struct EnergyPlusErrSummary {
    warning_count: usize,
    severe_count: usize,
    fatal_count: usize,
    warnings: Vec<String>,
}

struct ReportSeriesRow {
    key: String,
    variable: String,
    frequency: &'static str,
    variable_class: &'static str,
    source: &'static str,
    samples: usize,
    first: String,
    last: String,
    min: String,
    mean: String,
    max: String,
    nonzero_count: usize,
}

fn render_report_skeleton(
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

fn render_report_skeleton_summary_json(
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

fn read_energyplus_err_summary(path: &Path) -> Result<EnergyPlusErrSummary, String> {
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
