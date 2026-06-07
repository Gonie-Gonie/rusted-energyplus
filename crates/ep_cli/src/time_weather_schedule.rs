use std::path::{Path, PathBuf};

use ep_compare::{
    SeriesAlignment, SeriesComparisonStatus, SeriesDivergenceKind, SeriesSample, Tolerance,
    compare_series_samples_v2, load_eso_time_series,
};
use ep_compiler::compile_raw_model;
use ep_conformance::{
    ComparisonClass, ConformanceCase, OutputFrequency, OutputLevel, OutputRequest, SourceArtifact,
    VariableClass,
};
use ep_model::{DayOfWeek, TypedModel};
use ep_raw_model::load_epjson_file;
use ep_runtime::{TimeAxis, TimePoint, build_hourly_time_axis, load_epw_records};

use crate::conformance_artifacts::{BaselineSummary, generate_conformance_baseline_in_dir};
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
}

pub(crate) fn generate_time_weather_schedule_report(
    case_path: &Path,
    manifest: &ConformanceCase,
    oracle_root: &Path,
    output_root: &Path,
) -> Result<TimeWeatherScheduleReportSummary, String> {
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
    let time_axis = build_hourly_time_axis(&model)
        .map_err(|error| format!("failed to build time axis: {error}"))?;

    let weather_records = if manifest
        .outputs
        .iter()
        .any(|output| output.class == VariableClass::Weather)
    {
        let weather = baseline
            .weather
            .as_ref()
            .ok_or_else(|| "weather output comparison requires input.weather".to_string())?;
        Some(load_epw_records(weather).map_err(|error| format!("failed to load EPW: {error}"))?)
    } else {
        None
    };

    let mut rows = Vec::new();
    for output in &manifest.outputs {
        let expected = load_eso_time_series(&baseline.eso, &output.key, &output.variable)
            .map_err(|error| format!("failed to load ESO series: {error}"))?;
        let observed = observed_samples(output, &model, &time_axis, weather_records.as_deref())?;
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
            status,
        });
    }

    Ok(TimeWeatherScheduleContext {
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
    weather_records: Option<&[ep_runtime::EpwRecord]>,
) -> Result<Vec<SeriesSample>, String> {
    match output.class {
        VariableClass::Schedule => schedule_samples(output, model, time_axis),
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
) -> Result<Vec<SeriesSample>, String> {
    let schedule = model
        .schedules
        .iter()
        .find(|schedule| schedule.name.0.eq_ignore_ascii_case(&output.key))
        .ok_or_else(|| format!("missing Schedule:Constant {}", output.key))?;
    let values = ep_runtime::simulate_schedule_values(model, time_axis.sample_count());
    let trace = values
        .iter()
        .find(|trace| trace.schedule_id == schedule.id)
        .ok_or_else(|| format!("missing schedule trace {}", output.key))?;
    Ok(samples_with_time_axis(&trace.values, time_axis))
}

fn weather_samples(
    output: &OutputRequest,
    time_axis: &TimeAxis,
    weather_records: Option<&[ep_runtime::EpwRecord]>,
) -> Result<Vec<SeriesSample>, String> {
    let weather_records = weather_records
        .ok_or_else(|| "weather output comparison requires EPW records".to_string())?;
    if weather_records.len() < time_axis.sample_count() {
        return Err(format!(
            "EPW has {} samples but time axis requires {}",
            weather_records.len(),
            time_axis.sample_count()
        ));
    }
    let values = weather_records
        .iter()
        .take(time_axis.sample_count())
        .map(|record| weather_value(output, record))
        .collect::<Result<Vec<_>, _>>()?;
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

fn write_report(report_dir: &Path, context: &TimeWeatherScheduleContext<'_>) -> Result<(), String> {
    std::fs::create_dir_all(report_dir)
        .map_err(|error| format!("failed to create report directory: {error}"))?;
    std::fs::write(
        report_dir.join("compare-report.md"),
        render_markdown(context),
    )
    .map_err(|error| format!("failed to write time/weather/schedule report: {error}"))?;
    std::fs::write(
        report_dir.join("compare-summary.json"),
        render_json(context),
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
    report.push_str(&format!(
        "typed_schedules: {}\n\n",
        context.model.schedules.len()
    ));

    report.push_str("## Series\n\n");
    report.push_str("| key | variable | level | class | frequency | source | alignment | expected | observed | compared | max_abs_delta | rmse_delta | max_rel_delta | tolerance | max_rmse_tolerance | status | first_divergence |\n");
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
    json.push_str(&format!(
        "  \"time_axis_samples\": {},\n",
        context.time_axis.sample_count()
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
