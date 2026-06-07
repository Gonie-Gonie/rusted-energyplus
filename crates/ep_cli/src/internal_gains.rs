use std::path::{Path, PathBuf};

use ep_compare::{
    SeriesAlignment, SeriesComparisonStatus, SeriesDivergenceKind, SeriesSample, Tolerance,
    compare_series, compare_series_samples_v2, load_eso_series, load_eso_time_series,
};
use ep_compiler::compile_raw_model;
use ep_conformance::{
    ComparisonClass, ConformanceCase, OutputFrequency, OutputLevel, OutputRequest, SourceArtifact,
    VariableClass,
};
use ep_model::{DayOfWeek, TypedModel};
use ep_raw_model::load_epjson_file;
use ep_runtime::{
    TimeAxis, TimePoint, build_hourly_time_axis, simulate_zone_internal_convective_gains,
};

use crate::conformance_artifacts::{BaselineSummary, generate_conformance_baseline_in_dir};
use crate::{
    comparison_class_label, json_number, json_string, markdown_cell, output_frequency_label,
    print_compile_diagnostics, report_format_label, source_artifact_label, variable_class_label,
};

pub(crate) struct InternalGainsReportSummary {
    pub(crate) baseline: BaselineSummary,
    pub(crate) report_dir: PathBuf,
    pub(crate) compare_report: PathBuf,
    pub(crate) compare_summary: PathBuf,
    pub(crate) series_count: usize,
    pub(crate) conformance_series_count: usize,
    pub(crate) status: &'static str,
}

struct InternalGainContext<'a> {
    manifest: &'a ConformanceCase,
    model: TypedModel,
    time_axis: TimeAxis,
    rows: Vec<InternalGainRow>,
}

struct InternalGainRow {
    key: String,
    variable: String,
    frequency: OutputFrequency,
    variable_class: VariableClass,
    source: SourceArtifact,
    level: Option<OutputLevel>,
    tolerance: Tolerance,
    max_rmse_tolerance: Option<f64>,
    tolerance_label: String,
    expected_samples: usize,
    observed_samples: usize,
    compared_samples: usize,
    max_abs_delta: f64,
    rmse_delta: f64,
    max_rel_delta: f64,
    alignment: SeriesAlignment,
    first_divergence: Option<ep_compare::SeriesDivergenceV2>,
    status: SeriesComparisonStatus,
}

impl InternalGainRow {
    fn is_conformance(&self) -> bool {
        self.level == Some(OutputLevel::Conformance)
    }

    fn status_label(&self) -> &'static str {
        match self.status {
            SeriesComparisonStatus::Pass => "pass",
            SeriesComparisonStatus::Fail => "fail",
        }
    }

    fn alignment_label(&self) -> &'static str {
        match self.alignment {
            SeriesAlignment::Index => "index",
            SeriesAlignment::Timestamp => "timestamp",
        }
    }
}

pub(crate) fn run_compare_internal_convective_gain(args: &[String]) -> i32 {
    let Some(input_path) = args.first() else {
        eprintln!("missing input path");
        eprintln!("usage: eplus-rs compare internal-convective-gain <input.epJSON> <eplusout.eso>");
        return 2;
    };
    let Some(eso_path) = args.get(1) else {
        eprintln!("missing eplusout.eso path");
        eprintln!("usage: eplus-rs compare internal-convective-gain <input.epJSON> <eplusout.eso>");
        return 2;
    };

    let raw_model = match load_epjson_file(input_path) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let result = compile_raw_model(&raw_model);
    let Some(model) = result.model.as_ref() else {
        print_compile_diagnostics(&result.report);
        return 1;
    };
    if model.zones.is_empty() {
        eprintln!("no Zone objects are available for internal-convective-gain comparison");
        return 1;
    }

    let mut oracle_series = Vec::new();
    for zone in &model.zones {
        let values = match load_eso_series(
            eso_path,
            &zone.name.0,
            "Zone Total Internal Convective Heating Rate",
        ) {
            Ok(values) => values,
            Err(error) => {
                eprintln!("{error}");
                return 1;
            }
        };
        oracle_series.push((zone.id, zone.name.0.clone(), values));
    }

    let sample_count = oracle_series
        .iter()
        .map(|(_id, _name, values)| values.len())
        .max()
        .unwrap_or(0);
    let traces = simulate_zone_internal_convective_gains(model, sample_count);
    let mut passed = true;

    println!("Internal Convective Gain Comparison");
    println!("  comparison_class: smoke");
    println!("  conformance_claim: false");
    println!("  tolerance_policy: default");
    println!("  zones: {}", oracle_series.len());
    for (zone_id, zone_name, expected_values) in oracle_series {
        let Some(trace) = traces.iter().find(|trace| trace.zone_id == zone_id) else {
            eprintln!("missing Rust internal convective gain trace: {zone_name}");
            return 1;
        };
        let comparison = compare_series(
            &expected_values,
            &trace.values_w[..expected_values.len()],
            Tolerance::default(),
        );
        if !comparison.passed {
            passed = false;
        }
        println!(
            "  zone: {} samples: {} max_abs_delta: {} status: {}",
            zone_name,
            comparison.samples,
            comparison.max_abs_delta,
            if comparison.passed { "pass" } else { "fail" }
        );
        print_first_divergence("  ", comparison.first_divergence);
    }
    println!("  status: {}", if passed { "pass" } else { "fail" });

    if passed { 0 } else { 1 }
}

pub(crate) fn generate_internal_gains_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<InternalGainsReportSummary, String> {
    validate_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let context = build_context(manifest, &baseline)?;
    write_report(&compare_dir, &context)?;

    let conformance_rows = context
        .rows
        .iter()
        .filter(|row| row.is_conformance())
        .collect::<Vec<_>>();
    let passed = conformance_rows
        .iter()
        .all(|row| row.status == SeriesComparisonStatus::Pass);

    Ok(InternalGainsReportSummary {
        baseline,
        report_dir: compare_dir.clone(),
        compare_report: compare_dir.join("compare-report.md"),
        compare_summary: compare_dir.join("compare-summary.json"),
        series_count: context.rows.len(),
        conformance_series_count: conformance_rows.len(),
        status: if passed { "pass" } else { "fail" },
    })
}

fn validate_manifest(manifest: &ConformanceCase) -> Result<(), String> {
    if manifest.comparison_class != ComparisonClass::Conformance {
        return Err(format!(
            "internal-gains report requires comparison_class conformance, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if !manifest.conformance_claim {
        return Err("internal-gains report requires conformance_claim true".to_string());
    }

    let conformance_outputs = report_outputs(manifest)
        .into_iter()
        .filter(|output| output.level == Some(OutputLevel::Conformance))
        .collect::<Vec<_>>();
    if conformance_outputs.is_empty() {
        return Err("internal-gains report requires at least one conformance output".to_string());
    }
    for output in conformance_outputs {
        validate_report_output(output)?;
    }

    let Some(report) = manifest.report.as_ref() else {
        return Err("internal-gains report requires a report contract".to_string());
    };
    if report.path.trim().is_empty() {
        return Err("internal-gains report contract has an empty path".to_string());
    }

    let Some(gate) = manifest.gate.as_ref() else {
        return Err("internal-gains report requires a gate contract".to_string());
    };
    if gate.script.trim().is_empty() {
        return Err("internal-gains gate contract has an empty script".to_string());
    }
    if !gate.blocking {
        return Err("internal-gains conformance gate must be blocking".to_string());
    }

    Ok(())
}

fn validate_report_output(output: &OutputRequest) -> Result<(), String> {
    if output.frequency != OutputFrequency::Hourly {
        return Err(format!(
            "internal-gains conformance output must be hourly, got {} for {}",
            output_frequency_label(output.frequency),
            output.variable
        ));
    }
    if output.source != SourceArtifact::Eso {
        return Err(format!(
            "internal-gains conformance output must use eso source, got {} for {}",
            source_artifact_label(output.source),
            output.variable
        ));
    }
    if output.class != VariableClass::InternalGain {
        return Err(format!(
            "internal-gains conformance output must use internal-gain class, got {} for {}",
            variable_class_label(output.class),
            output.variable
        ));
    }
    if !output
        .variable
        .eq_ignore_ascii_case("Zone Total Internal Convective Heating Rate")
    {
        return Err(format!(
            "internal-gains report currently supports Zone Total Internal Convective Heating Rate, got {}",
            output.variable
        ));
    }
    Ok(())
}

fn report_outputs(manifest: &ConformanceCase) -> Vec<&OutputRequest> {
    manifest
        .outputs
        .iter()
        .filter(|output| output.source == SourceArtifact::Eso)
        .collect()
}

fn build_context<'a>(
    manifest: &'a ConformanceCase,
    baseline: &BaselineSummary,
) -> Result<InternalGainContext<'a>, String> {
    let raw_model = load_epjson_file(&baseline.epjson)
        .map_err(|error| format!("failed to load baseline epJSON: {error}"))?;
    let compile_result = compile_raw_model(&raw_model);
    let model = compile_result.model.ok_or_else(|| {
        let diagnostics = compile_result
            .report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        format!("failed to compile baseline epJSON: {diagnostics}")
    })?;
    let time_axis = build_hourly_time_axis(&model)
        .map_err(|error| format!("failed to build time axis: {error}"))?;

    let mut rows = Vec::new();
    for output in report_outputs(manifest) {
        let expected = load_eso_time_series(&baseline.eso, &output.key, &output.variable)
            .map_err(|error| format!("failed to load ESO series: {error}"))?;
        let observed = observed_samples(output, &model, &time_axis)?;
        let tolerance = tolerance_for_output(manifest, output)?;
        let max_rmse_tolerance = max_rmse_tolerance_for_output(manifest, output)?;
        let comparison = compare_series_samples_v2(&expected.samples, &observed, tolerance);
        let status = if comparison.status == SeriesComparisonStatus::Pass
            && max_rmse_tolerance.is_none_or(|max_rmse| comparison.rmse_delta <= max_rmse)
        {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };
        rows.push(InternalGainRow {
            key: output.key.clone(),
            variable: output.variable.clone(),
            frequency: output.frequency,
            variable_class: output.class,
            source: output.source,
            level: output.level,
            tolerance,
            max_rmse_tolerance,
            tolerance_label: tolerance_label(tolerance),
            expected_samples: comparison.expected_samples,
            observed_samples: comparison.observed_samples,
            compared_samples: comparison.compared_samples,
            max_abs_delta: comparison.max_abs_delta,
            rmse_delta: comparison.rmse_delta,
            max_rel_delta: comparison.max_rel_delta,
            alignment: comparison.alignment,
            first_divergence: comparison.first_divergence,
            status,
        });
    }

    Ok(InternalGainContext {
        manifest,
        model,
        time_axis,
        rows,
    })
}

fn observed_samples(
    output: &OutputRequest,
    model: &TypedModel,
    time_axis: &TimeAxis,
) -> Result<Vec<SeriesSample>, String> {
    let traces = simulate_zone_internal_convective_gains(model, time_axis.sample_count());
    let trace = traces
        .iter()
        .find(|trace| trace.zone_name.eq_ignore_ascii_case(&output.key))
        .ok_or_else(|| {
            format!(
                "missing Rust internal convective gain trace: {}",
                output.key
            )
        })?;
    Ok(samples_with_time_axis(&trace.values_w, time_axis))
}

fn samples_with_time_axis(values: &[f64], time_axis: &TimeAxis) -> Vec<SeriesSample> {
    values
        .iter()
        .copied()
        .zip(&time_axis.points)
        .map(|(value, point)| {
            SeriesSample::timestamped(point.sample_index, timestamp_label(time_axis, point), value)
        })
        .collect()
}

fn timestamp_label(time_axis: &TimeAxis, point: &TimePoint) -> String {
    format!(
        "env={};day={};month={};date={};dst=0;hour={};start=0.00;end=60.00;day_type={}",
        time_axis.run_period_name.to_ascii_uppercase(),
        point.sample_index / 24 + 1,
        point.month,
        point.day_of_month,
        point.hour,
        day_type_label(day_of_week_for_point(time_axis, point))
    )
}

fn day_of_week_for_point(time_axis: &TimeAxis, point: &TimePoint) -> DayOfWeek {
    let first = time_axis
        .points
        .first()
        .map(|first| weekday_from_date(first.year, first.month, first.day_of_month))
        .unwrap_or(DayOfWeek::Tuesday);
    let offset = point.sample_index / 24;
    advance_day(first, offset)
}

fn weekday_from_date(year: u32, month: u32, day: u32) -> DayOfWeek {
    let mut y = i64::from(year);
    let mut m = i64::from(month);
    let d = i64::from(day);
    if m < 3 {
        m += 12;
        y -= 1;
    }
    let k = y % 100;
    let j = y / 100;
    let h = (d + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    match h {
        0 => DayOfWeek::Saturday,
        1 => DayOfWeek::Sunday,
        2 => DayOfWeek::Monday,
        3 => DayOfWeek::Tuesday,
        4 => DayOfWeek::Wednesday,
        5 => DayOfWeek::Thursday,
        _ => DayOfWeek::Friday,
    }
}

fn advance_day(day: DayOfWeek, offset_days: usize) -> DayOfWeek {
    let start = match day {
        DayOfWeek::Monday => 0,
        DayOfWeek::Tuesday => 1,
        DayOfWeek::Wednesday => 2,
        DayOfWeek::Thursday => 3,
        DayOfWeek::Friday => 4,
        DayOfWeek::Saturday => 5,
        DayOfWeek::Sunday => 6,
    };
    match (start + offset_days) % 7 {
        0 => DayOfWeek::Monday,
        1 => DayOfWeek::Tuesday,
        2 => DayOfWeek::Wednesday,
        3 => DayOfWeek::Thursday,
        4 => DayOfWeek::Friday,
        5 => DayOfWeek::Saturday,
        _ => DayOfWeek::Sunday,
    }
}

fn day_type_label(day: DayOfWeek) -> &'static str {
    match day {
        DayOfWeek::Monday => "Monday",
        DayOfWeek::Tuesday => "Tuesday",
        DayOfWeek::Wednesday => "Wednesday",
        DayOfWeek::Thursday => "Thursday",
        DayOfWeek::Friday => "Friday",
        DayOfWeek::Saturday => "Saturday",
        DayOfWeek::Sunday => "Sunday",
    }
}

fn tolerance_for_output(
    manifest: &ConformanceCase,
    output: &OutputRequest,
) -> Result<Tolerance, String> {
    let rule = manifest
        .tolerances
        .iter()
        .find(|rule| rule.variable_class == output.class)
        .ok_or_else(|| {
            format!(
                "missing tolerance rule for {} output {}",
                variable_class_label(output.class),
                output.variable
            )
        })?;

    Ok(Tolerance {
        absolute: output.abs_tol.or(rule.max_abs).unwrap_or(0.0),
        relative: output.rel_tol.or(rule.max_rel).unwrap_or(0.0),
    })
}

fn max_rmse_tolerance_for_output(
    manifest: &ConformanceCase,
    output: &OutputRequest,
) -> Result<Option<f64>, String> {
    let rule = manifest
        .tolerances
        .iter()
        .find(|rule| rule.variable_class == output.class)
        .ok_or_else(|| {
            format!(
                "missing tolerance rule for {} output {}",
                variable_class_label(output.class),
                output.variable
            )
        })?;
    Ok(output.rmse_tol.or(rule.max_rmse))
}

fn tolerance_label(tolerance: Tolerance) -> String {
    format!(
        "absolute-{:.12}-relative-{:.12}",
        tolerance.absolute, tolerance.relative
    )
}

fn write_report(report_dir: &Path, context: &InternalGainContext<'_>) -> Result<(), String> {
    std::fs::create_dir_all(report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;
    std::fs::write(
        report_dir.join("compare-report.md"),
        render_markdown(context),
    )
    .map_err(|error| format!("failed to write internal-gains report: {error}"))?;
    std::fs::write(
        report_dir.join("compare-summary.json"),
        render_json(context),
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
    report.push_str("claim_boundary: Zone Total Internal Convective Heating Rate only; static EIO nominal rows remain diagnostic here\n");
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
    json.push_str("  \"claim_boundary\": \"Zone Total Internal Convective Heating Rate only; static EIO nominal rows remain diagnostic in this dynamic report\",\n");
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

fn print_first_divergence(prefix: &str, divergence: Option<ep_compare::SeriesDivergence>) {
    let Some(divergence) = divergence else {
        println!("{prefix}first_divergence: none");
        return;
    };

    match (
        divergence.expected,
        divergence.observed,
        divergence.abs_delta,
    ) {
        (Some(expected), Some(observed), Some(abs_delta)) => println!(
            "{prefix}first_divergence: index {} expected {:.12} observed {:.12} abs_delta {:.12}",
            divergence.index, expected, observed, abs_delta
        ),
        (expected, observed, None) => println!(
            "{prefix}first_divergence: index {} expected {} observed {} length_mismatch",
            divergence.index,
            legacy_optional_number_label(expected),
            legacy_optional_number_label(observed)
        ),
        (expected, observed, Some(abs_delta)) => println!(
            "{prefix}first_divergence: index {} expected {} observed {} abs_delta {:.12}",
            divergence.index,
            legacy_optional_number_label(expected),
            legacy_optional_number_label(observed),
            abs_delta
        ),
    }
}

fn legacy_optional_number_label(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("{value:.12}"),
        None => "missing".to_string(),
    }
}
