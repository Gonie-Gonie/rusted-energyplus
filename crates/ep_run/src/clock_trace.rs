//! Output-only serialization of prepared calendars and actual zone invocations.

use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::psychrometrics::production_trace::EnvironmentObservation;
use ep_runtime::time_axis::TimeAxis;
use ep_runtime::time_axis::clock_trace::{CalendarFrame, ClockTrace};
use serde_json::{Value, json};
use std::io::{BufWriter, Write};

pub(crate) fn calendar_frame(frame: &CalendarFrame) -> Value {
    json!({
        "hourly_sample_index": frame.hourly_sample_index, "day_of_sim": frame.day_of_sim,
        "year": frame.year, "month": frame.month, "day_of_month": frame.day_of_month,
        "gregorian_day_of_year": frame.gregorian_day_of_year,
        "weather_day_of_year": frame.weather_day_of_year,
        "schedule_day_of_year": frame.schedule_day_of_year,
        "gregorian_day_of_week": frame.gregorian_day_of_week, "day_of_week": frame.day_of_week,
        "day_type": frame.day_type, "day_type_label": frame.day_type_label,
        "gregorian_year_is_leap_year": frame.gregorian_year_is_leap_year,
        "weather_effective_year_is_leap_year": frame.weather_effective_year_is_leap_year,
        "leap_year_add": frame.leap_year_add, "dst": frame.dst,
        "special_day_type": frame.special_day_type, "hour_ending": frame.hour_ending,
    })
}

/// Preparation rows are distinct from observed execution events.
pub(crate) fn prepared_calendar(axis: &TimeAxis) -> Value {
    let daily: Vec<_> = axis
        .points
        .iter()
        .filter(|point| point.hour == 1)
        .map(CalendarFrame::from)
        .collect();
    json!({
        "schema": "clk01-prepared-calendar.v1", "preparation_only": true,
        "physics_executed": false, "run_period_name": axis.run_period_name,
        "hourly_sample_count": axis.sample_count(),
        "daily_frames": daily.iter().map(calendar_frame).collect::<Vec<_>>(),
        "month_start_frames": daily.iter().filter(|frame| frame.day_of_month == 1)
            .map(calendar_frame).collect::<Vec<_>>(),
    })
}

fn environment_point(point: &EnvironmentObservation) -> Value {
    json!({
        "materialized_environment_index": point.environment_index,
        "sample_index": point.sample_index, "simulation_timestep": point.simulation_timestep,
        "zone_timestep": point.zone_timestep,
        "start_minute_bits": format!("{:016x}", point.start_minute_bits),
        "end_minute_bits": format!("{:016x}", point.end_minute_bits),
        "current_time_hours_bits": format!("{:016x}", point.current_time_hours_bits),
        "begin_environment": point.begin_environment, "end_environment": point.end_environment,
        "begin_day": point.begin_day, "end_day": point.end_day,
        "begin_hour": point.begin_hour, "end_hour": point.end_hour,
    })
}

pub(crate) fn write_clock_trace(config: &RunConfig, trace: &ClockTrace) -> Result<(), RunError> {
    let omitted = trace.total_invocation_count - trace.zone_invocations.len() as u64;
    let artifact = json!({
        "schema": "clk01-clock-trace.v1",
        "capture_source": "existing-physical-zone-loop-hook",
        "thread_coverage": "collecting-thread-only",
        "phase_contract": "actual-Rust-zone-invocation; copied-current-calendar-before-reporting",
        "input_origin": "actual-Rust-prepared-axis-and-loop-operands; no-EP-or-fixture-seeding",
        "run_period_name": trace.run_period_name,
        "parsed_input_design_day_declaration_count": trace.parsed_input_design_day_declaration_count,
        "design_day_count_origin": "parsed-RawModel-object-count; not-native-InputNumDesignDays-state",
        "source_environment_number": null,
        "materialized_environment_index": trace.materialized_environment_index,
        "environment_ordinal_boundary": "Rust-materialized-index-is-separate-from-native-EP-source-ordinal",
        "prepared_hourly_frames": trace.prepared_hourly_frames.iter().map(calendar_frame).collect::<Vec<_>>(),
        "prepared_environment_points": trace.prepared_environment_points.iter().map(environment_point).collect::<Vec<_>>(),
        "prepared_rows_are_executed_events": false,
        "invocation_limit": trace.invocation_limit,
        "total_invocation_count": trace.total_invocation_count,
        "recorded_invocation_count": trace.zone_invocations.len(),
        "omitted_invocation_count": omitted,
        "complete_on_collecting_thread": omitted == 0,
        "truncation_reason": if omitted == 0 { None } else { Some("invocation_limit") },
        "zone_invocations": trace.zone_invocations.iter().map(|event| json!({
            "sequence": event.sequence, "hour_index": event.hour_index,
            "zone_timestep": event.zone_timestep, "zone_steps_per_hour": event.zone_steps_per_hour,
            "timestep_seconds_bits": format!("{:016x}", event.timestep_seconds_bits),
            "calendar_frame_index": event.calendar_frame_index,
            "environment_point_index": event.environment_point_index,
        })).collect::<Vec<_>>(),
        "unclaimed_state": ["Tomorrow-handoff", "DatesShouldBeReset", "OutputProcessor-year-end-global-mutation",
            "native-EP-global-Begin/End-flags", "adaptive-system-order", "warmup-clock"],
    });
    let path = config.output_dir.join("clock-calls.json");
    let write = || -> Result<(), String> {
        let file = std::fs::File::create(&path)
            .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &artifact)
            .map_err(|error| format!("failed to serialize {}: {error}", path.display()))?;
        writer
            .write_all(b"\n")
            .and_then(|()| writer.flush())
            .map_err(|error| format!("failed to write {}: {error}", path.display()))
    };
    write().map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })
}
