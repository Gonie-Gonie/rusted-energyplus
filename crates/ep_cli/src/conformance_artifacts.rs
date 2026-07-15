use ep_compare::load_eso_series;
use ep_conformance::{ConformanceCase, OutputRegistry, SourceArtifact, TimestampContract};
use ep_raw_model::{RawModel, load_epjson_file_with_idf_order};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

use crate::{
    comparison_class_label, first_value_label, json_string, last_value_label, max_value_label,
    mean_value_label, min_value_label, nonzero_count, output_frequency_label, report_format_label,
    resolve_manifest_path, source_artifact_label, variable_class_label,
};

mod output_injection;
mod report_skeleton;

#[cfg(test)]
use output_injection::render_output_request_injection;
use output_injection::{OutputInjectionSummary, stage_idf_with_output_requests};
use report_skeleton::{
    ReportSeriesRow, read_energyplus_err_summary, render_report_skeleton,
    render_report_skeleton_summary_json,
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
    pub(crate) timing: BaselineTimingSummary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AuxiliaryFileProvenance {
    source: PathBuf,
    staged: PathBuf,
}

impl BaselineSummary {
    pub(crate) fn load_raw_model(&self) -> Result<RawModel, String> {
        load_epjson_file_with_idf_order(&self.epjson, &self.idf).map_err(|error| {
            format!(
                "failed to load baseline converted epJSON with staged IDF declaration order: {error}"
            )
        })
    }
}

pub(crate) struct ReportSkeletonSummary {
    pub(crate) report_path: PathBuf,
    pub(crate) series: usize,
    pub(crate) warning_count: usize,
    pub(crate) severe_count: usize,
    pub(crate) fatal_count: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BaselineTimingSummary {
    pub(crate) input_stage_wall_seconds: f64,
    pub(crate) energyplus_oracle_wall_seconds: f64,
    pub(crate) idf_converter_wall_seconds: f64,
    pub(crate) oracle_output_copy_wall_seconds: f64,
    pub(crate) expanded_manifest_write_wall_seconds: f64,
    pub(crate) total_wall_seconds: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ReportTimingSummary {
    pub(crate) baseline: BaselineTimingSummary,
    pub(crate) rust_context_wall_seconds: f64,
    pub(crate) rust_artifact_write_wall_seconds: f64,
    pub(crate) rust_compare_report_wall_seconds: f64,
    pub(crate) total_wall_seconds: f64,
}

pub(crate) fn elapsed_seconds_since(start: Instant) -> f64 {
    duration_seconds(start.elapsed())
}

pub(crate) fn append_timing_to_json_object(
    mut json: String,
    timing: &ReportTimingSummary,
) -> String {
    let trailing_whitespace_len = json.len() - json.trim_end().len();
    if trailing_whitespace_len > 0 {
        json.truncate(json.len() - trailing_whitespace_len);
    }
    if !json.ends_with('}') {
        return json;
    }
    json.pop();
    json.push_str(",\n  \"timing\": ");
    json.push_str(&report_timing_json(timing));
    json.push_str("\n}\n");
    json
}

fn duration_seconds(duration: Duration) -> f64 {
    duration.as_secs_f64()
}

fn report_timing_json(timing: &ReportTimingSummary) -> String {
    format!(
        concat!(
            "{{\n",
            "    \"schema_version\": 1,\n",
            "    \"measurement\": \"wall-clock seconds measured inside ep_cli unless noted\",\n",
            "    \"primary_comparison_scope\": \"EnergyPlus oracle output production wall-clock versus Rust compare/evidence production wall-clock for the same conformance case\",\n",
            "    \"energyplus_oracle_wall_seconds\": {:.9},\n",
            "    \"rust_compare_report_wall_seconds\": {:.9},\n",
            "    \"ep_cli_total_wall_seconds\": {:.9},\n",
            "    \"phases\": [\n",
            "      {{ \"name\": \"oracle_input_stage\", \"engine\": \"ep_cli\", \"wall_seconds\": {:.9}, \"scope\": \"stage IDF/weather/output requests before EnergyPlus\" }},\n",
            "      {{ \"name\": \"energyplus_oracle\", \"engine\": \"EnergyPlus\", \"wall_seconds\": {:.9}, \"scope\": \"energyplus.exe process that produces oracle ESO/EIO/MTR/ERR files\" }},\n",
            "      {{ \"name\": \"idf_converter\", \"engine\": \"EnergyPlus ConvertInputFormat\", \"wall_seconds\": {:.9}, \"scope\": \"ConvertInputFormat.exe writes epJSON used by Rust comparison\" }},\n",
            "      {{ \"name\": \"oracle_output_copy\", \"engine\": \"ep_cli\", \"wall_seconds\": {:.9}, \"scope\": \"copy EnergyPlus outputs from short run dir when required\" }},\n",
            "      {{ \"name\": \"expanded_manifest_write\", \"engine\": \"ep_cli\", \"wall_seconds\": {:.9}, \"scope\": \"persist expanded case manifest\" }},\n",
            "      {{ \"name\": \"baseline_total\", \"engine\": \"ep_cli+EnergyPlus\", \"wall_seconds\": {:.9}, \"scope\": \"all oracle baseline preparation and EnergyPlus output production\" }},\n",
            "      {{ \"name\": \"rust_context\", \"engine\": \"rusted-energyplus\", \"wall_seconds\": {:.9}, \"scope\": \"load model/oracle files, evaluate Rust path, and compare requested series\" }},\n",
            "      {{ \"name\": \"rust_artifact_write\", \"engine\": \"rusted-energyplus\", \"wall_seconds\": {:.9}, \"scope\": \"write compare JSON/CSV/Markdown artifacts\" }},\n",
            "      {{ \"name\": \"rust_compare_report\", \"engine\": \"rusted-energyplus\", \"wall_seconds\": {:.9}, \"scope\": \"Rust context plus artifact write after oracle files exist\" }},\n",
            "      {{ \"name\": \"ep_cli_total\", \"engine\": \"ep_cli\", \"wall_seconds\": {:.9}, \"scope\": \"full ep_cli conformance command excluding cargo/script startup\" }}\n",
            "    ]\n",
            "  }}"
        ),
        timing.baseline.energyplus_oracle_wall_seconds,
        timing.rust_compare_report_wall_seconds,
        timing.total_wall_seconds,
        timing.baseline.input_stage_wall_seconds,
        timing.baseline.energyplus_oracle_wall_seconds,
        timing.baseline.idf_converter_wall_seconds,
        timing.baseline.oracle_output_copy_wall_seconds,
        timing.baseline.expanded_manifest_write_wall_seconds,
        timing.baseline.total_wall_seconds,
        timing.rust_context_wall_seconds,
        timing.rust_artifact_write_wall_seconds,
        timing.rust_compare_report_wall_seconds,
        timing.total_wall_seconds
    )
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
    let total_start = Instant::now();
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

    let input_stage_start = Instant::now();
    std::fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create baseline output directory: {error}"))?;
    let input_idf = output_dir.join("input.idf");
    let injection = stage_idf_with_output_requests(&source_idf, &input_idf, manifest)?;
    let auxiliary_files =
        stage_case_auxiliary_files(case_path, &manifest.input.auxiliary_files, output_dir)?;
    let staged_weather = source_weather
        .as_ref()
        .map(|weather| {
            let staged = output_dir.join("weather.epw");
            std::fs::copy(weather, &staged)
                .map(|_| staged)
                .map_err(|error| format!("failed to stage EnergyPlus weather.epw: {error}"))
        })
        .transpose()?;

    let input_arg = process_path_argument(&input_idf);
    let output_arg = process_path_argument(output_dir);
    let use_short_run_dir = input_arg.len() > 180 || output_arg.len() > 180;
    let short_run_dir = if use_short_run_dir {
        let run_dir = short_energyplus_run_dir(output_dir)?;
        if run_dir.exists() {
            std::fs::remove_dir_all(&run_dir).map_err(|error| {
                format!(
                    "failed to remove previous EnergyPlus run directory {}: {error}",
                    run_dir.display()
                )
            })?;
        }
        std::fs::create_dir_all(&run_dir).map_err(|error| {
            format!(
                "failed to create EnergyPlus run directory {}: {error}",
                run_dir.display()
            )
        })?;
        std::fs::copy(&input_idf, run_dir.join("input.idf"))
            .map_err(|error| format!("failed to stage EnergyPlus input.idf: {error}"))?;
        if let Some(weather) = staged_weather.as_ref() {
            std::fs::copy(weather, run_dir.join("weather.epw"))
                .map_err(|error| format!("failed to stage EnergyPlus weather.epw: {error}"))?;
        }
        copy_staged_auxiliary_files(&auxiliary_files, &run_dir)?;
        Some(run_dir)
    } else {
        None
    };
    let input_stage_wall_seconds = elapsed_seconds_since(input_stage_start);

    let mut energyplus_command = Command::new(&energyplus);
    if let Some(run_dir) = short_run_dir.as_ref() {
        energyplus_command.current_dir(run_dir);
        if source_weather.is_some() {
            energyplus_command.arg("-w").arg("weather.epw");
        }
        energyplus_command.arg("-d").arg(".").arg("input.idf");
    } else {
        energyplus_command.current_dir(output_dir);
        if staged_weather.is_some() {
            energyplus_command.arg("-w").arg("weather.epw");
        }
        energyplus_command.arg("-d").arg(".").arg("input.idf");
    }
    let energyplus_start = Instant::now();
    let energyplus_output = energyplus_command
        .output()
        .map_err(|error| format!("failed to start EnergyPlus: {error}"))?;
    let energyplus_oracle_wall_seconds = elapsed_seconds_since(energyplus_start);
    if !energyplus_output.status.success() {
        let err_path = short_run_dir
            .as_ref()
            .map(|run_dir| run_dir.join("eplusout.err"))
            .unwrap_or_else(|| output_dir.join("eplusout.err"));
        return Err(command_failure_message(
            "EnergyPlus baseline",
            &energyplus_output,
            Some(&err_path),
        ));
    }

    let converter_dir = short_run_dir.as_deref().unwrap_or(output_dir);
    let converter_start = Instant::now();
    let converter_output = Command::new(&converter)
        .arg("input.idf")
        .current_dir(converter_dir)
        .output()
        .map_err(|error| format!("failed to start IDF converter: {error}"))?;
    let idf_converter_wall_seconds = elapsed_seconds_since(converter_start);
    if !converter_output.status.success() {
        return Err(command_failure_message(
            "IDF conversion",
            &converter_output,
            None,
        ));
    }

    let output_copy_start = Instant::now();
    if let Some(run_dir) = short_run_dir.as_ref() {
        copy_regular_files(run_dir, output_dir)?;
    }
    let oracle_output_copy_wall_seconds = elapsed_seconds_since(output_copy_start);

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
    let expanded_manifest_start = Instant::now();
    std::fs::write(
        &expanded_manifest,
        render_expanded_case_manifest(
            manifest,
            source_weather.as_deref(),
            &auxiliary_files,
            &injection,
        ),
    )
    .map_err(|error| format!("failed to write expanded case manifest: {error}"))?;
    let expanded_manifest_write_wall_seconds = elapsed_seconds_since(expanded_manifest_start);
    let total_wall_seconds = elapsed_seconds_since(total_start);

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
        timing: BaselineTimingSummary {
            input_stage_wall_seconds,
            energyplus_oracle_wall_seconds,
            idf_converter_wall_seconds,
            oracle_output_copy_wall_seconds,
            expanded_manifest_write_wall_seconds,
            total_wall_seconds,
        },
    })
}

fn render_expanded_case_manifest(
    manifest: &ConformanceCase,
    source_weather: Option<&Path>,
    auxiliary_files: &[AuxiliaryFileProvenance],
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

    if let Some(trace) = manifest.trace.as_ref() {
        toml.push_str("[trace]\n");
        push_toml_string_field(&mut toml, "level", &trace.level);
        toml.push('\n');
    }

    toml.push_str("[input]\n");
    push_toml_string_field(&mut toml, "source_idf", &manifest.input.idf);
    push_toml_string_field(&mut toml, "staged_idf", "input.idf");
    if let Some(weather) = source_weather {
        push_toml_string_field(&mut toml, "source_weather", &weather.display().to_string());
    }
    push_toml_string_field(&mut toml, "converted_epjson", "input.epJSON");
    toml.push('\n');
    for auxiliary in auxiliary_files {
        toml.push_str("[[input.auxiliary_files]]\n");
        push_toml_string_field(&mut toml, "source", &auxiliary.source.display().to_string());
        let staged_name = auxiliary
            .staged
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        push_toml_string_field(&mut toml, "staged", staged_name);
        toml.push('\n');
    }

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
        if let Some(timestamp_contract) = output.timestamp_contract {
            push_toml_string_field(
                &mut toml,
                "timestamp_contract",
                timestamp_contract_label(timestamp_contract),
            );
        }
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

fn timestamp_contract_label(contract: TimestampContract) -> &'static str {
    match contract {
        TimestampContract::OrderedExactUnique => "ordered-exact-unique",
    }
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

fn short_process_path(path: &Path) -> PathBuf {
    let Ok(current_dir) = std::env::current_dir() else {
        return path.to_path_buf();
    };
    let Ok(relative) = path.strip_prefix(&current_dir) else {
        return path.to_path_buf();
    };
    if relative.as_os_str().is_empty() {
        path.to_path_buf()
    } else {
        relative.to_path_buf()
    }
}

fn process_path_argument(path: &Path) -> String {
    short_process_path(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn stage_case_auxiliary_files(
    case_path: &Path,
    auxiliary_file_names: &[String],
    output_dir: &Path,
) -> Result<Vec<AuxiliaryFileProvenance>, String> {
    let case_dir = case_path.parent().unwrap_or_else(|| Path::new("."));
    let mut staged_files = Vec::with_capacity(auxiliary_file_names.len());
    for file_name in auxiliary_file_names {
        let source = case_dir.join(file_name);
        if !source.is_file() {
            return Err(format!("missing case auxiliary file: {}", source.display()));
        }
        let staged = output_dir.join(file_name);
        std::fs::copy(&source, &staged).map_err(|error| {
            format!(
                "failed to stage auxiliary file {} as {}: {error}",
                source.display(),
                staged.display()
            )
        })?;
        staged_files.push(AuxiliaryFileProvenance { source, staged });
    }
    Ok(staged_files)
}

fn copy_staged_auxiliary_files(
    auxiliary_files: &[AuxiliaryFileProvenance],
    destination_dir: &Path,
) -> Result<(), String> {
    for auxiliary in auxiliary_files {
        let staged_name = auxiliary.staged.file_name().ok_or_else(|| {
            format!(
                "staged auxiliary file has no basename: {}",
                auxiliary.staged.display()
            )
        })?;
        let destination = destination_dir.join(staged_name);
        std::fs::copy(&auxiliary.staged, &destination).map_err(|error| {
            format!(
                "failed to copy staged auxiliary file {} as {}: {error}",
                auxiliary.staged.display(),
                destination.display()
            )
        })?;
    }
    Ok(())
}

fn short_energyplus_run_dir(output_dir: &Path) -> Result<PathBuf, String> {
    let current_dir =
        std::env::current_dir().map_err(|error| format!("failed to read current dir: {error}"))?;
    let mut hasher = DefaultHasher::new();
    output_dir.to_string_lossy().hash(&mut hasher);
    Ok(current_dir
        .join(".runtime")
        .join("energyplus-runs")
        .join(format!("{:016x}", hasher.finish())))
}

fn copy_regular_files(source_dir: &Path, target_dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(target_dir).map_err(|error| {
        format!(
            "failed to create EnergyPlus output directory {}: {error}",
            target_dir.display()
        )
    })?;
    for entry in std::fs::read_dir(source_dir).map_err(|error| {
        format!(
            "failed to read EnergyPlus run directory {}: {error}",
            source_dir.display()
        )
    })? {
        let entry = entry.map_err(|error| format!("failed to read EnergyPlus output: {error}"))?;
        let file_type = entry
            .file_type()
            .map_err(|error| format!("failed to inspect EnergyPlus output: {error}"))?;
        if file_type.is_file() {
            std::fs::copy(entry.path(), target_dir.join(entry.file_name())).map_err(|error| {
                format!(
                    "failed to copy EnergyPlus output {}: {error}",
                    entry.path().display()
                )
            })?;
        }
    }
    Ok(())
}

fn command_failure_message(label: &str, output: &Output, err_path: Option<&Path>) -> String {
    let mut message = format!("{label} failed with status {}", output.status);
    append_process_stream_tail(&mut message, "stdout", &output.stdout);
    append_process_stream_tail(&mut message, "stderr", &output.stderr);
    if let Some(path) = err_path
        && let Ok(contents) = std::fs::read_to_string(path)
    {
        append_text_tail(&mut message, &format!("{} tail", path.display()), &contents);
    }
    message
}

fn append_process_stream_tail(message: &mut String, label: &str, bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }
    let text = String::from_utf8_lossy(bytes);
    append_text_tail(message, label, &text);
}

fn append_text_tail(message: &mut String, label: &str, text: &str) {
    let lines = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    if lines.is_empty() {
        return;
    }
    message.push_str("\n--- ");
    message.push_str(label);
    message.push_str(" ---");
    for line in lines
        .iter()
        .rev()
        .take(40)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        message.push('\n');
        message.push_str(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_conformance::parse_case_str;

    #[test]
    fn expanded_manifest_preserves_output_timestamp_contract()
    -> Result<(), Box<dyn std::error::Error>> {
        let manifest = parse_case_str(
            r#"
id = "ordered_timestamps_001"
title = "ordered timestamps"
milestone = "test"
purpose = "test expanded timestamp contract"
comparison_class = "diagnostic-only"
conformance_claim = false
oracle_version = "26.1.0"

[input]
idf = "input.idf"

[[outputs]]
key = "*"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"
timestamp_contract = "ordered-exact-unique"
"#,
        )?;
        let injection = OutputInjectionSummary {
            outputs: 1,
            meters: 0,
            surface_details: false,
        };

        let expanded = render_expanded_case_manifest(&manifest, None, &[], &injection);

        assert!(expanded.contains("timestamp_contract = \"ordered-exact-unique\""));
        Ok(())
    }

    #[test]
    fn expanded_manifest_records_auxiliary_source_and_staged_provenance()
    -> Result<(), Box<dyn std::error::Error>> {
        let manifest = parse_case_str(
            r#"
id = "auxiliary_provenance_001"
title = "auxiliary provenance"
milestone = "test"
purpose = "test expanded auxiliary provenance"
comparison_class = "smoke"
conformance_claim = false
oracle_version = "26.1.0"

[input]
idf = "input.idf"
auxiliary_files = ["schedule.csv"]
"#,
        )?;
        let injection = OutputInjectionSummary {
            outputs: 0,
            meters: 0,
            surface_details: false,
        };
        let auxiliary_files = [AuxiliaryFileProvenance {
            source: PathBuf::from("case").join("schedule.csv"),
            staged: PathBuf::from("oracle").join("schedule.csv"),
        }];

        let expanded = render_expanded_case_manifest(&manifest, None, &auxiliary_files, &injection);

        assert!(expanded.contains("[[input.auxiliary_files]]"));
        let expected_source = format!(
            "source = {}",
            json_string(
                &PathBuf::from("case")
                    .join("schedule.csv")
                    .display()
                    .to_string()
            )
        );
        assert!(expanded.contains(&expected_source));
        assert!(expanded.contains("staged = \"schedule.csv\""));
        Ok(())
    }

    #[test]
    fn auxiliary_files_are_staged_for_normal_and_short_energyplus_run_directories()
    -> Result<(), Box<dyn std::error::Error>> {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rusted-energyplus-auxiliary-stage-{}-{unique}",
            std::process::id()
        ));
        let case_dir = root.join("case");
        let output_dir = root.join("oracle");
        let short_run_dir = root.join("short");
        std::fs::create_dir_all(&case_dir)?;
        std::fs::create_dir_all(&output_dir)?;
        std::fs::create_dir_all(&short_run_dir)?;
        let case_path = case_dir.join("case.toml");
        std::fs::write(&case_path, "")?;
        std::fs::write(case_dir.join("schedule.csv"), b"header\n1.25\n")?;

        let staged =
            stage_case_auxiliary_files(&case_path, &["schedule.csv".to_string()], &output_dir)?;
        copy_staged_auxiliary_files(&staged, &short_run_dir)?;

        assert_eq!(
            std::fs::read(output_dir.join("schedule.csv"))?,
            b"header\n1.25\n"
        );
        assert_eq!(
            std::fs::read(short_run_dir.join("schedule.csv"))?,
            b"header\n1.25\n"
        );
        std::fs::remove_dir_all(root)?;
        Ok(())
    }

    #[test]
    fn output_injection_distinguishes_existing_variable_frequency()
    -> Result<(), Box<dyn std::error::Error>> {
        let manifest = parse_case_str(
            r#"
id = "frequency_sensitive_output_001"
title = "frequency sensitive output"
milestone = "test"
purpose = "test output request injection"
comparison_class = "diagnostic-only"
conformance_claim = false
oracle_version = "26.1.0"

[input]
idf = "input.idf"

[[outputs]]
key = "ZN001:ROOF001"
variable = "Surface Inside Face Temperature"
frequency = "hourly"
class = "surface-state"
source = "eso"
"#,
        )?;
        let idf = "Output:Variable,*,Surface Inside Face Temperature,daily;\n";

        let injection = render_output_request_injection(&manifest, idf);

        assert_eq!(injection.outputs, 1);
        assert!(injection.text.contains("Surface Inside Face Temperature"));
        assert!(injection.text.contains("Hourly"));
        Ok(())
    }

    #[test]
    fn process_failure_tail_omits_blank_lines_and_keeps_recent_context() {
        let text = (1..=45)
            .map(|index| format!("line {index}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut message = String::from("failed");

        append_text_tail(&mut message, "stderr", &format!("\n{text}\n\n"));

        assert!(message.contains("--- stderr ---"));
        assert!(!message.contains("line 1\n"));
        assert!(message.contains("line 6"));
        assert!(message.contains("line 45"));
    }

    #[test]
    fn process_paths_are_shortened_when_they_are_under_current_directory() {
        let current_dir = std::env::current_dir().expect("current dir");
        let path = current_dir
            .join("target")
            .join("diagnostic")
            .join("input.idf");

        assert_eq!(
            short_process_path(&path),
            PathBuf::from("target").join("diagnostic").join("input.idf")
        );
        assert_eq!(process_path_argument(&path), "target/diagnostic/input.idf");
    }

    #[test]
    fn output_injection_reuses_existing_variable_with_same_frequency()
    -> Result<(), Box<dyn std::error::Error>> {
        let manifest = parse_case_str(
            r#"
id = "frequency_sensitive_output_002"
title = "frequency sensitive output"
milestone = "test"
purpose = "test output request injection"
comparison_class = "diagnostic-only"
conformance_claim = false
oracle_version = "26.1.0"

[input]
idf = "input.idf"

[[outputs]]
key = "ZN001:ROOF001"
variable = "Surface Inside Face Temperature"
frequency = "hourly"
class = "surface-state"
source = "eso"
"#,
        )?;
        let idf = "Output:Variable,*,Surface Inside Face Temperature,hourly;\n";

        let injection = render_output_request_injection(&manifest, idf);

        assert_eq!(injection.outputs, 0);
        assert!(injection.text.contains("no new output requests"));
        Ok(())
    }

    #[test]
    fn output_injection_distinguishes_existing_meter_frequency()
    -> Result<(), Box<dyn std::error::Error>> {
        let manifest = parse_case_str(
            r#"
id = "frequency_sensitive_meter_001"
title = "frequency sensitive meter"
milestone = "test"
purpose = "test meter request injection"
comparison_class = "diagnostic-only"
conformance_claim = false
oracle_version = "26.1.0"

[input]
idf = "input.idf"

[[meters]]
name = "Electricity:Facility"
frequency = "hourly"
source = "mtr"
domain = "meter"
level = "diagnostic"
"#,
        )?;
        let idf = "Output:Meter,Electricity:Facility,annual;\n";

        let injection = render_output_request_injection(&manifest, idf);

        assert_eq!(injection.meters, 1);
        assert!(injection.text.contains("Electricity:Facility"));
        assert!(injection.text.contains("Hourly"));
        Ok(())
    }
}
