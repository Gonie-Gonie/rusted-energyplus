use std::path::{Path, PathBuf};
use std::time::Instant;

use ep_compare::{
    SeriesAlignment, SeriesComparisonStatus, SeriesSample, Tolerance, compare_series,
    compare_series_samples_v2, load_eso_series, load_eso_time_series,
};
use ep_compiler::compile_raw_model;
use ep_conformance::{
    ComparisonClass, ConformanceCase, OutputFrequency, OutputLevel, OutputRequest, SourceArtifact,
    VariableClass,
};
use ep_model::{DayOfWeek, TypedModel};
use ep_raw_model::load_epjson_file;
use ep_runtime::{
    TimeAxis, TimePoint, ZONE_TOTAL_INTERNAL_CONVECTIVE_HEATING_RATE_VARIABLE,
    ZONE_TOTAL_INTERNAL_RADIANT_HEATING_RATE_VARIABLE, build_hourly_time_axis,
    simulate_zone_internal_convective_gains, simulate_zone_internal_radiant_gains,
};

use crate::conformance_artifacts::{
    BaselineSummary, ReportTimingSummary, elapsed_seconds_since,
    generate_conformance_baseline_in_dir,
};
use crate::{
    comparison_class_label, output_frequency_label, print_compile_diagnostics,
    source_artifact_label, variable_class_label,
};

mod render;

use render::write_report;

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
            ZONE_TOTAL_INTERNAL_CONVECTIVE_HEATING_RATE_VARIABLE,
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
    let total_start = Instant::now();
    validate_manifest(manifest)?;

    let case_output_dir = output_root.join(&manifest.id);
    let oracle_output_dir = case_output_dir.join("oracle");
    let compare_dir = case_output_dir.join("compare");

    let baseline =
        generate_conformance_baseline_in_dir(case_path, manifest, oracle_root, &oracle_output_dir)?;
    let rust_context_start = Instant::now();
    let context = build_context(manifest, &baseline)?;
    let rust_context_wall_seconds = elapsed_seconds_since(rust_context_start);
    let rust_artifact_start = Instant::now();
    let timing = ReportTimingSummary {
        baseline: baseline.timing,
        rust_context_wall_seconds,
        rust_artifact_write_wall_seconds: 0.0,
        rust_compare_report_wall_seconds: 0.0,
        total_wall_seconds: 0.0,
    };
    write_report(&compare_dir, &context, &timing)?;
    let rust_artifact_write_wall_seconds = elapsed_seconds_since(rust_artifact_start);
    let timing = ReportTimingSummary {
        baseline: baseline.timing,
        rust_context_wall_seconds,
        rust_artifact_write_wall_seconds,
        rust_compare_report_wall_seconds: rust_context_wall_seconds
            + rust_artifact_write_wall_seconds,
        total_wall_seconds: elapsed_seconds_since(total_start),
    };
    write_report(&compare_dir, &context, &timing)?;

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
    if !is_supported_internal_gain_rate_variable(&output.variable) {
        return Err(format!(
            "internal-gains report supports Zone Total Internal Convective Heating Rate and Zone Total Internal Radiant Heating Rate, got {}",
            output.variable
        ));
    }
    Ok(())
}

fn is_supported_internal_gain_rate_variable(variable: &str) -> bool {
    variable.eq_ignore_ascii_case(ZONE_TOTAL_INTERNAL_CONVECTIVE_HEATING_RATE_VARIABLE)
        || variable.eq_ignore_ascii_case(ZONE_TOTAL_INTERNAL_RADIANT_HEATING_RATE_VARIABLE)
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
    let traces = if output
        .variable
        .eq_ignore_ascii_case(ZONE_TOTAL_INTERNAL_CONVECTIVE_HEATING_RATE_VARIABLE)
    {
        simulate_zone_internal_convective_gains(model, time_axis.sample_count())
    } else if output
        .variable
        .eq_ignore_ascii_case(ZONE_TOTAL_INTERNAL_RADIANT_HEATING_RATE_VARIABLE)
    {
        simulate_zone_internal_radiant_gains(model, time_axis.sample_count())
    } else {
        return Err(format!(
            "unsupported internal gain output variable: {}",
            output.variable
        ));
    };
    let trace = traces
        .iter()
        .find(|trace| trace.zone_name.eq_ignore_ascii_case(&output.key))
        .ok_or_else(|| {
            format!(
                "missing Rust internal gain trace for {}: {}",
                output.variable, output.key
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
