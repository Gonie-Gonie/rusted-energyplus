use std::path::{Path, PathBuf};
use std::time::Instant;

use ep_compare::{
    OrderedTimestampDivergence, SeriesAlignment, SeriesComparisonStatus, SeriesDivergenceKind,
    SeriesSample, Tolerance, compare_ordered_timestamp_samples_v2, compare_series_samples_v2,
    load_eso_time_series,
};
use ep_compiler::compile_raw_model;
use ep_conformance::{
    ComparisonClass, ConformanceCase, OutputFrequency, OutputLevel, OutputRequest, SourceArtifact,
    TimestampContract, VariableClass,
};
use ep_model::TypedModel;
use ep_raw_model::load_epjson_file;
use ep_runtime::{
    EpwEnvironmentWeather, ScheduleValueSeries, TimeAxis, build_hourly_time_axis,
    build_hourly_time_axis_with_weather_metadata, load_epw_weather_file,
    normalized_hourly_timestamp_label, precompute_schedule_value_series_for_time_axis,
    select_epw_environment_weather,
};

use crate::conformance_artifacts::{
    BaselineSummary, ReportTimingSummary, append_timing_to_json_object, elapsed_seconds_since,
    generate_conformance_baseline_in_dir,
};
use crate::{
    comparison_class_label, json_number, json_string, markdown_cell, output_frequency_label,
    report_format_label, source_artifact_label, variable_class_label,
};

pub(crate) struct TimeWeatherScheduleReportSummary {
    pub(crate) baseline: BaselineSummary,
    pub(crate) report_dir: PathBuf,
    pub(crate) compare_report: PathBuf,
    pub(crate) compare_summary: PathBuf,
    pub(crate) series_count: usize,
    pub(crate) conformance_series_count: usize,
    pub(crate) status: &'static str,
}

struct TimeWeatherScheduleContext<'a> {
    manifest: &'a ConformanceCase,
    model: TypedModel,
    time_axis: TimeAxis,
    weather_environment: Option<EpwEnvironmentWeather>,
    rows: Vec<TimeWeatherScheduleRow>,
}

struct TimeWeatherScheduleRow {
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
    timestamp_contract: Option<TimestampContract>,
    timestamp_expected_unique: Option<bool>,
    timestamp_observed_unique: Option<bool>,
    timestamp_order_match: Option<bool>,
    timestamp_status: Option<SeriesComparisonStatus>,
    first_timestamp_divergence: Option<OrderedTimestampDivergence>,
    expected_first_timestamp: Option<String>,
    expected_last_timestamp: Option<String>,
    observed_first_timestamp: Option<String>,
    observed_last_timestamp: Option<String>,
    status: SeriesComparisonStatus,
}

impl TimeWeatherScheduleRow {
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

    fn timestamp_contract_label(&self) -> &'static str {
        timestamp_contract_label(self.timestamp_contract)
    }
}

pub(crate) fn generate_time_weather_schedule_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<TimeWeatherScheduleReportSummary, String> {
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

    Ok(TimeWeatherScheduleReportSummary {
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
            "time/weather/schedule report requires comparison_class conformance, got {}",
            comparison_class_label(manifest.comparison_class)
        ));
    }
    if !manifest.conformance_claim {
        return Err("time/weather/schedule report requires conformance_claim true".to_string());
    }
    if manifest.outputs.is_empty() {
        return Err("time/weather/schedule report requires at least one output".to_string());
    }
    if !manifest
        .outputs
        .iter()
        .any(|output| output.level == Some(OutputLevel::Conformance))
    {
        return Err(
            "time/weather/schedule report requires at least one conformance output".to_string(),
        );
    }
    for output in &manifest.outputs {
        if output.frequency != OutputFrequency::Hourly {
            return Err(format!(
                "time/weather/schedule report requires hourly outputs, got {} for {}",
                output_frequency_label(output.frequency),
                output.variable
            ));
        }
        if output.source != SourceArtifact::Eso {
            return Err(format!(
                "time/weather/schedule report requires eso source, got {} for {}",
                source_artifact_label(output.source),
                output.variable
            ));
        }
        if !matches!(
            output.class,
            VariableClass::Schedule | VariableClass::Weather
        ) {
            return Err(format!(
                "time/weather/schedule report supports schedule or weather classes, got {} for {}",
                variable_class_label(output.class),
                output.variable
            ));
        }
    }

    let Some(report) = manifest.report.as_ref() else {
        return Err("time/weather/schedule report requires a report contract".to_string());
    };
    if report.path.trim().is_empty() {
        return Err("time/weather/schedule report contract has an empty path".to_string());
    }

    let Some(gate) = manifest.gate.as_ref() else {
        return Err("time/weather/schedule report requires a gate contract".to_string());
    };
    if gate.script.trim().is_empty() {
        return Err("time/weather/schedule gate contract has an empty script".to_string());
    }
    if !gate.blocking {
        return Err("time/weather/schedule conformance gate must be blocking".to_string());
    }

    Ok(())
}

fn build_context<'a>(
    manifest: &'a ConformanceCase,
    baseline: &BaselineSummary,
) -> Result<TimeWeatherScheduleContext<'a>, String> {
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
    let weather_file = baseline
        .weather
        .as_ref()
        .map(|weather| {
            load_epw_weather_file(weather).map_err(|error| format!("failed to load EPW: {error}"))
        })
        .transpose()?;
    let time_axis = weather_file
        .as_ref()
        .map_or_else(
            || build_hourly_time_axis(&model),
            |weather_file| {
                build_hourly_time_axis_with_weather_metadata(
                    &model,
                    &weather_file.calendar_metadata,
                )
            },
        )
        .map_err(|error| format!("failed to build time axis: {error}"))?;
    let schedule_series = manifest
        .outputs
        .iter()
        .any(|output| output.class == VariableClass::Schedule)
        .then(|| precompute_schedule_value_series_for_time_axis(&model, &time_axis));
    let has_weather_output = manifest
        .outputs
        .iter()
        .any(|output| output.class == VariableClass::Weather);
    let weather_environment = if has_weather_output {
        Some(
            weather_file
                .as_ref()
                .ok_or_else(|| "weather output comparison requires input.weather".to_string())
                .and_then(|weather_file| {
                    select_epw_environment_weather(weather_file, &time_axis).map_err(|error| {
                        format!("failed to select EPW environment records: {error}")
                    })
                })?,
        )
    } else {
        None
    };

    let weather_records = weather_environment
        .as_ref()
        .map(EpwEnvironmentWeather::hourly_records);

    let mut rows = Vec::new();
    for output in &manifest.outputs {
        let expected = load_eso_time_series(&baseline.eso, &output.key, &output.variable)
            .map_err(|error| format!("failed to load ESO series: {error}"))?;
        let observed = observed_samples(
            output,
            &model,
            &time_axis,
            schedule_series.as_deref(),
            weather_records,
        )?;
        let tolerance = tolerance_for_output(manifest, output)?;
        let max_rmse_tolerance = max_rmse_tolerance_for_output(manifest, output)?;
        let (
            comparison,
            timestamp_expected_unique,
            timestamp_observed_unique,
            timestamp_order_match,
            timestamp_status,
            first_timestamp_divergence,
        ) = match output.timestamp_contract {
            Some(TimestampContract::OrderedExactUnique) => {
                let ordered =
                    compare_ordered_timestamp_samples_v2(&expected.samples, &observed, tolerance);
                (
                    ordered.comparison,
                    Some(ordered.expected_unique_timestamps),
                    Some(ordered.observed_unique_timestamps),
                    Some(ordered.timestamp_order_match),
                    Some(ordered.contract_status),
                    ordered.first_timestamp_divergence,
                )
            }
            None => (
                compare_series_samples_v2(&expected.samples, &observed, tolerance),
                None,
                None,
                None,
                None,
                None,
            ),
        };
        let status = if comparison.status == SeriesComparisonStatus::Pass
            && timestamp_status.is_none_or(|status| status == SeriesComparisonStatus::Pass)
            && max_rmse_tolerance.is_none_or(|max_rmse| comparison.rmse_delta <= max_rmse)
        {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };
        rows.push(TimeWeatherScheduleRow {
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
            timestamp_contract: output.timestamp_contract,
            timestamp_expected_unique,
            timestamp_observed_unique,
            timestamp_order_match,
            timestamp_status,
            first_timestamp_divergence,
            expected_first_timestamp: contract_timestamp(
                output.timestamp_contract,
                expected.samples.first(),
            ),
            expected_last_timestamp: contract_timestamp(
                output.timestamp_contract,
                expected.samples.last(),
            ),
            observed_first_timestamp: contract_timestamp(
                output.timestamp_contract,
                observed.first(),
            ),
            observed_last_timestamp: contract_timestamp(output.timestamp_contract, observed.last()),
            status,
        });
    }

    Ok(TimeWeatherScheduleContext {
        manifest,
        model,
        time_axis,
        weather_environment,
        rows,
    })
}

fn observed_samples(
    output: &OutputRequest,
    model: &TypedModel,
    time_axis: &TimeAxis,
    schedule_series: Option<&[ScheduleValueSeries]>,
    weather_records: Option<&[ep_runtime::EpwRecord]>,
) -> Result<Vec<SeriesSample>, String> {
    match output.class {
        VariableClass::Schedule => schedule_samples(
            output,
            model,
            time_axis,
            schedule_series.ok_or_else(|| "missing precomputed schedule series".to_string())?,
        ),
        VariableClass::Weather => weather_samples(output, time_axis, weather_records),
        _ => Err(format!(
            "unsupported output class for time/weather/schedule report: {}",
            variable_class_label(output.class)
        )),
    }
}

fn schedule_samples(
    output: &OutputRequest,
    model: &TypedModel,
    time_axis: &TimeAxis,
    schedule_series: &[ScheduleValueSeries],
) -> Result<Vec<SeriesSample>, String> {
    let schedule_id = model
        .schedule_names
        .resolve(&output.key)
        .ok_or_else(|| format!("missing schedule {}", output.key))?;
    let trace = schedule_series
        .iter()
        .find(|trace| trace.schedule_id == schedule_id)
        .ok_or_else(|| format!("missing schedule trace {}", output.key))?;
    if trace.values.len() != time_axis.sample_count() {
        return Err(format!(
            "schedule trace {} has {} samples but time axis requires {}",
            output.key,
            trace.values.len(),
            time_axis.sample_count()
        ));
    }
    Ok(samples_with_time_axis(&trace.values, time_axis))
}

fn weather_samples(
    output: &OutputRequest,
    time_axis: &TimeAxis,
    weather_records: Option<&[ep_runtime::EpwRecord]>,
) -> Result<Vec<SeriesSample>, String> {
    let weather_records = weather_records
        .ok_or_else(|| "weather output comparison requires EPW records".to_string())?;
    if weather_records.len() != time_axis.sample_count() {
        return Err(format!(
            "selected EPW environment has {} samples but time axis requires {}",
            weather_records.len(),
            time_axis.sample_count()
        ));
    }
    let values = if output
        .variable
        .eq_ignore_ascii_case("Site Daylight Saving Time Status")
    {
        time_axis
            .points
            .iter()
            .map(|point| f64::from(u8::from(point.dst)))
            .collect()
    } else {
        weather_records
            .iter()
            .map(|record| weather_value(output, record))
            .collect::<Result<Vec<_>, _>>()?
    };
    Ok(samples_with_time_axis(&values, time_axis))
}

fn weather_value(output: &OutputRequest, record: &ep_runtime::EpwRecord) -> Result<f64, String> {
    match output.variable.as_str() {
        value if value.eq_ignore_ascii_case("Site Outdoor Air Drybulb Temperature") => {
            Ok(record.dry_bulb_c)
        }
        value if value.eq_ignore_ascii_case("Site Outdoor Air Dewpoint Temperature") => {
            Ok(record.dew_point_c)
        }
        value if value.eq_ignore_ascii_case("Site Outdoor Air Relative Humidity") => {
            Ok(record.relative_humidity_percent)
        }
        value if value.eq_ignore_ascii_case("Site Outdoor Air Barometric Pressure") => {
            Ok(record.atmospheric_pressure_pa)
        }
        value if value.eq_ignore_ascii_case("Site Wind Speed") => Ok(record.wind_speed_m_per_s),
        value if value.eq_ignore_ascii_case("Site Wind Direction") => Ok(record.wind_direction_deg),
        _ => Err(format!(
            "unsupported weather output variable: {}",
            output.variable
        )),
    }
}

fn samples_with_time_axis(values: &[f64], time_axis: &TimeAxis) -> Vec<SeriesSample> {
    values
        .iter()
        .copied()
        .zip(&time_axis.points)
        .map(|(value, point)| {
            SeriesSample::timestamped(
                point.sample_index,
                normalized_hourly_timestamp_label(time_axis, point),
                value,
            )
        })
        .collect()
}

fn contract_timestamp(
    contract: Option<TimestampContract>,
    sample: Option<&SeriesSample>,
) -> Option<String> {
    contract?;
    sample?.timestamp.clone()
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

fn write_report(
    report_dir: &Path,
    context: &TimeWeatherScheduleContext<'_>,
    timing: &ReportTimingSummary,
) -> Result<(), String> {
    std::fs::create_dir_all(report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;
    std::fs::write(
        report_dir.join("compare-report.md"),
        render_markdown(context),
    )
    .map_err(|error| format!("failed to write time/weather/schedule report: {error}"))?;
    std::fs::write(
        report_dir.join("compare-summary.json"),
        append_timing_to_json_object(render_json(context), timing),
    )
    .map_err(|error| format!("failed to write time/weather/schedule summary: {error}"))?;
    Ok(())
}

fn render_markdown(context: &TimeWeatherScheduleContext<'_>) -> String {
    let manifest = context.manifest;
    let mut report = String::new();
    report.push_str("# Time, Weather, and Schedule Conformance Report\n\n");
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
    report.push_str(
        "timestamp_rule: hour-ending hourly samples aligned by EnergyPlus ESO timestamp labels\n\n",
    );
    report.push_str(
        "timestamp_contract_rule: ordered-exact-unique is opt-in and requires present, unique, same-index exact timestamp labels\n\n",
    );

    report.push_str("## Result\n\n");
    report.push_str(&format!("status: {}\n", overall_status(&context.rows)));
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
    if let Some(calendar) = context.time_axis.weather_calendar.as_ref() {
        report.push_str("weather_calendar_policy_applied: true\n");
        report.push_str(&format!(
            "weather_file_allows_leap_years: {}\n",
            calendar.weather_file_allows_leap_years
        ));
        report.push_str(&format!(
            "gregorian_calendar_days: {}\n",
            calendar.gregorian.total_days
        ));
        report.push_str(&format!(
            "weather_effective_calendar_days: {}\n",
            calendar.total_days
        ));
        report.push_str(&format!(
            "leap_days_skipped: {}\n",
            calendar.leap_days_skipped
        ));
        report.push_str(&format!(
            "start_year_gregorian_leap: {}\n",
            calendar.gregorian.start_year_is_leap_year
        ));
        report.push_str(&format!(
            "start_year_weather_effective_leap: {}\n",
            calendar.start_year_is_weather_effective_leap_year
        ));
        report.push_str(&format!(
            "weather_file_daylight_saving_period_declared: {}\n",
            context
                .time_axis
                .daylight_saving
                .weather_file_period_declared
        ));
        report.push_str(&format!(
            "run_period_uses_weather_file_daylight_saving_period: {}\n",
            context
                .time_axis
                .daylight_saving
                .run_period_uses_weather_file_period
        ));
        report.push_str(&format!(
            "daylight_saving_active: {}\n",
            context.time_axis.daylight_saving.active
        ));
        if let Some(period) = context.time_axis.daylight_saving.resolved_period {
            report.push_str(&format!(
                "daylight_saving_resolved_period: {}/{} through {}/{} (wraps_year={})\n",
                period.start.month,
                period.start.day_of_month,
                period.end.month,
                period.end.day_of_month,
                period.wraps_year
            ));
        } else {
            report.push_str("daylight_saving_resolved_period: none\n");
        }
        report.push_str(&format!(
            "daylight_saving_hourly_samples: {}\n",
            context
                .time_axis
                .points
                .iter()
                .filter(|point| point.dst)
                .count()
        ));
    } else {
        report.push_str("weather_calendar_policy_applied: false\n");
    }
    if let Some(weather) = context.weather_environment.as_ref() {
        report.push_str("weather_record_selection_applied: true\n");
        report.push_str(&format!(
            "weather_data_period_index: {}\n",
            weather.data_period_index + 1
        ));
        report.push_str(&format!(
            "weather_source_start_record_index: {}\n",
            weather.source_start_record_index
        ));
        report.push_str(&format!(
            "weather_initial_tomorrow_record_index: {}\n",
            weather.initial_tomorrow_source_record_start
        ));
        report.push_str(&format!(
            "weather_selected_hourly_records: {}\n",
            weather.hourly_records().len()
        ));
        report.push_str(&format!(
            "weather_skipped_raw_february_29_days: {}\n",
            weather.skipped_february_29_source_record_starts.len()
        ));
        report.push_str(&format!(
            "weather_day_buffer_transitions: {}\n",
            weather.day_buffer_transitions.len()
        ));
    } else {
        report.push_str("weather_record_selection_applied: false\n");
    }
    report.push_str(&format!(
        "typed_schedules: {}\n\n",
        context.model.schedule_names.len()
    ));

    report.push_str("## Series\n\n");
    report.push_str("| key | variable | level | class | frequency | source | alignment | expected | observed | compared | max_abs_delta | rmse_delta | max_rel_delta | tolerance | max_rmse_tolerance | status | first_divergence | timestamp_contract | timestamp_status |\n");
    report.push_str(
        "|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---|---:|---|---|---|---|\n",
    );
    for row in &context.rows {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.12} | {:.12} | {:.12} | {} | {} | {} | {} | {} | {} |\n",
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
            first_divergence_label(row.first_divergence.as_ref()),
            row.timestamp_contract_label(),
            optional_comparison_status_label(row.timestamp_status),
        ));
    }

    if context
        .rows
        .iter()
        .any(|row| row.timestamp_contract.is_some())
    {
        report.push_str("\n## Ordered Timestamp Contracts\n\n");
        report.push_str("| key | contract | expected_unique | observed_unique | order_match | expected_first | expected_last | observed_first | observed_last | first_timestamp_divergence |\n");
        report.push_str("|---|---|---|---|---|---|---|---|---|---|\n");
        for row in context
            .rows
            .iter()
            .filter(|row| row.timestamp_contract.is_some())
        {
            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                markdown_cell(&row.key),
                row.timestamp_contract_label(),
                optional_bool_label(row.timestamp_expected_unique),
                optional_bool_label(row.timestamp_observed_unique),
                optional_bool_label(row.timestamp_order_match),
                optional_timestamp_label(row.expected_first_timestamp.as_deref()),
                optional_timestamp_label(row.expected_last_timestamp.as_deref()),
                optional_timestamp_label(row.observed_first_timestamp.as_deref()),
                optional_timestamp_label(row.observed_last_timestamp.as_deref()),
                first_timestamp_divergence_label(row.first_timestamp_divergence.as_ref()),
            ));
        }
    }
    report
}

fn render_json(context: &TimeWeatherScheduleContext<'_>) -> String {
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
    json.push_str("  \"timestamp_contract_rule\": \"ordered-exact-unique is opt-in and requires present, unique, same-index exact timestamp labels\",\n");
    json.push_str(&format!(
        "  \"time_axis_samples\": {},\n",
        context.time_axis.sample_count()
    ));
    json.push_str(&format!(
        "  \"weather_calendar\": {},\n",
        weather_calendar_json(&context.time_axis)
    ));
    json.push_str(&format!(
        "  \"weather_record_selection\": {},\n",
        weather_record_selection_json(context.weather_environment.as_ref())
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
            "      \"first_divergence\": {},\n",
            first_divergence_json(row.first_divergence.as_ref())
        ));
        json.push_str(&format!(
            "      \"timestamp_contract\": {},\n",
            optional_timestamp_contract_json(row.timestamp_contract)
        ));
        json.push_str(&format!(
            "      \"timestamp_expected_unique\": {},\n",
            optional_bool_json(row.timestamp_expected_unique)
        ));
        json.push_str(&format!(
            "      \"timestamp_observed_unique\": {},\n",
            optional_bool_json(row.timestamp_observed_unique)
        ));
        json.push_str(&format!(
            "      \"timestamp_order_match\": {},\n",
            optional_bool_json(row.timestamp_order_match)
        ));
        json.push_str(&format!(
            "      \"timestamp_status\": {},\n",
            optional_comparison_status_json(row.timestamp_status)
        ));
        json.push_str(&format!(
            "      \"expected_first_timestamp\": {},\n",
            optional_string_json(row.expected_first_timestamp.as_deref())
        ));
        json.push_str(&format!(
            "      \"expected_last_timestamp\": {},\n",
            optional_string_json(row.expected_last_timestamp.as_deref())
        ));
        json.push_str(&format!(
            "      \"observed_first_timestamp\": {},\n",
            optional_string_json(row.observed_first_timestamp.as_deref())
        ));
        json.push_str(&format!(
            "      \"observed_last_timestamp\": {},\n",
            optional_string_json(row.observed_last_timestamp.as_deref())
        ));
        json.push_str(&format!(
            "      \"first_timestamp_divergence\": {}\n",
            first_timestamp_divergence_json(row.first_timestamp_divergence.as_ref())
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

fn overall_status(rows: &[TimeWeatherScheduleRow]) -> &'static str {
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

fn weather_calendar_json(time_axis: &TimeAxis) -> String {
    let Some(calendar) = time_axis.weather_calendar.as_ref() else {
        return "null".to_string();
    };
    let daylight_saving_period = time_axis
        .daylight_saving
        .resolved_period
        .map_or_else(|| "null".to_string(), |period| {
            format!(
                "{{\"start_month\": {}, \"start_day\": {}, \"start_day_of_year\": {}, \"end_month\": {}, \"end_day\": {}, \"end_day_of_year\": {}, \"wraps_year\": {}}}",
                period.start.month,
                period.start.day_of_month,
                period.start.day_of_year,
                period.end.month,
                period.end.day_of_month,
                period.end.day_of_year,
                period.wraps_year,
            )
        });
    format!(
        "{{\"policy_applied\": true, \"weather_file_allows_leap_years\": {}, \"gregorian_calendar_days\": {}, \"weather_effective_calendar_days\": {}, \"leap_days_skipped\": {}, \"start_year_gregorian_leap\": {}, \"start_year_weather_effective_leap\": {}, \"daylight_saving\": {{\"weather_file_period_declared\": {}, \"run_period_uses_weather_file_period\": {}, \"active\": {}, \"resolved_period\": {}}}, \"daylight_saving_hourly_samples\": {}}}",
        calendar.weather_file_allows_leap_years,
        calendar.gregorian.total_days,
        calendar.total_days,
        calendar.leap_days_skipped,
        calendar.gregorian.start_year_is_leap_year,
        calendar.start_year_is_weather_effective_leap_year,
        time_axis.daylight_saving.weather_file_period_declared,
        time_axis
            .daylight_saving
            .run_period_uses_weather_file_period,
        time_axis.daylight_saving.active,
        daylight_saving_period,
        time_axis.points.iter().filter(|point| point.dst).count(),
    )
}

fn weather_record_selection_json(weather: Option<&EpwEnvironmentWeather>) -> String {
    let Some(weather) = weather else {
        return "null".to_string();
    };
    format!(
        "{{\"applied\": true, \"data_period_index\": {}, \"source_start_record_index\": {}, \"initial_tomorrow_source_record_index\": {}, \"selected_hourly_records\": {}, \"skipped_raw_february_29_days\": {}, \"day_buffer_transitions\": {}}}",
        weather.data_period_index + 1,
        weather.source_start_record_index,
        weather.initial_tomorrow_source_record_start,
        weather.hourly_records().len(),
        weather.skipped_february_29_source_record_starts.len(),
        weather.day_buffer_transitions.len(),
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

fn timestamp_contract_label(contract: Option<TimestampContract>) -> &'static str {
    match contract {
        Some(TimestampContract::OrderedExactUnique) => "ordered-exact-unique",
        None => "none",
    }
}

fn optional_timestamp_contract_json(contract: Option<TimestampContract>) -> String {
    contract.map_or_else(
        || "null".to_string(),
        |contract| json_string(timestamp_contract_label(Some(contract))),
    )
}

fn optional_comparison_status_label(status: Option<SeriesComparisonStatus>) -> &'static str {
    match status {
        Some(SeriesComparisonStatus::Pass) => "pass",
        Some(SeriesComparisonStatus::Fail) => "fail",
        None => "none",
    }
}

fn optional_comparison_status_json(status: Option<SeriesComparisonStatus>) -> String {
    status.map_or_else(
        || "null".to_string(),
        |status| json_string(optional_comparison_status_label(Some(status))),
    )
}

fn optional_bool_label(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "true",
        Some(false) => "false",
        None => "none",
    }
}

fn optional_bool_json(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "true",
        Some(false) => "false",
        None => "null",
    }
}

fn optional_timestamp_label(value: Option<&str>) -> String {
    value.map_or_else(|| "none".to_string(), markdown_cell)
}

fn optional_string_json(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_string(), json_string)
}

fn first_timestamp_divergence_label(divergence: Option<&OrderedTimestampDivergence>) -> String {
    let Some(divergence) = divergence else {
        return "none".to_string();
    };
    format!(
        "{} index={} expected={} observed={}",
        divergence.reason.as_str(),
        divergence.index,
        divergence.expected.as_deref().unwrap_or("none"),
        divergence.observed.as_deref().unwrap_or("none")
    )
}

fn first_timestamp_divergence_json(divergence: Option<&OrderedTimestampDivergence>) -> String {
    let Some(divergence) = divergence else {
        return "null".to_string();
    };
    format!(
        "{{\"index\": {}, \"reason\": {}, \"expected\": {}, \"observed\": {}}}",
        divergence.index,
        json_string(divergence.reason.as_str()),
        optional_string_json(divergence.expected.as_deref()),
        optional_string_json(divergence.observed.as_deref())
    )
}

fn optional_number_label(value: Option<f64>) -> String {
    value.map_or_else(|| "none".to_string(), |value| format!("{value:.12}"))
}

fn optional_number_json(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), json_number)
}

#[cfg(test)]
#[path = "time_weather_schedule_tests.rs"]
mod tests;
