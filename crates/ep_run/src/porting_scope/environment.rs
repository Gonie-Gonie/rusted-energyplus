//! Reuse the production EPW/calendar/schedule preparation for admission traces.

use super::PortingScope;
use ep_model::TypedModel;
use ep_raw_model::RawModel;
use ep_runtime::schedules::precompute_schedule_value_series_for_environment_time_axis;
use ep_runtime::{
    build_environment_time_axes_with_weather_metadata,
    build_hourly_time_axis_with_weather_metadata, load_epw_weather_file,
    precompute_weather_timestep_series, select_epw_environment_weather,
};
use serde_json::{Value, json};
use std::path::Path;

/// Proof of exact raw calendar-family admission on the Source A/B route.
/// It supplies no dates, schedules or mutable runtime state.
pub(crate) struct SourceCalendarInputAdmission {
    _private: (),
}
impl SourceCalendarInputAdmission {
    pub(super) fn from_raw(raw: &RawModel, scope: PortingScope) -> Result<Self, String> {
        match scope {
            PortingScope::A => crate::ordinary_equipment_input::resolve_source_a(raw),
            PortingScope::B => crate::ordinary_equipment_input::resolve_source_b(raw),
        }?;
        Ok(Self { _private: () })
    }
}

pub(crate) fn porting_source_environment_trace(
    raw: &RawModel,
    model: &TypedModel,
    weather_path: &Path,
    scope: PortingScope,
) -> Result<(Value, SourceCalendarInputAdmission), String> {
    let admission = SourceCalendarInputAdmission::from_raw(raw, scope)?;
    let weather = load_epw_weather_file(weather_path).map_err(|error| error.to_string())?;
    let trace = source_environment_from_weather(
        model,
        &weather,
        &weather_path.to_string_lossy(),
        &admission,
    )?;
    Ok((trace, admission))
}

/// Civil dates, weather coverage and step counts only. The actual Source route
/// consumes no DST/holiday/schedule values from this preparation-time axis.
/// Native's surprising end-Nth predicate must reach the genuine calendar child,
/// rather than being vetoed by the legacy preview date resolver.
pub(crate) fn source_coverage_axis(
    raw: &RawModel,
    scope: PortingScope,
    model: &TypedModel,
    metadata: &ep_runtime::weather::EpwCalendarMetadata,
) -> Result<ep_runtime::time_axis::TimeAxis, String> {
    let _admission = SourceCalendarInputAdmission::from_raw(raw, scope)?;
    let mut coverage = model.clone();
    coverage.run_period_daylight_saving_time = None;
    coverage.run_period_special_days.clear();
    let mut calendar = metadata.clone();
    calendar.daylight_saving_period = None;
    calendar.holidays.clear();
    build_hourly_time_axis_with_weather_metadata(&coverage, &calendar)
        .map_err(|error| error.to_string())
}

pub(super) fn source_environment_from_weather(
    model: &TypedModel,
    weather: &ep_runtime::weather::EpwWeatherFile,
    weather_path: &str,
    _admission: &SourceCalendarInputAdmission,
) -> Result<Value, String> {
    let mut coverage = model.clone();
    coverage.run_period_daylight_saving_time = None;
    coverage.run_period_special_days.clear();
    let mut weather_coverage = weather.clone();
    weather_coverage.calendar_metadata.daylight_saving_period = None;
    weather_coverage.calendar_metadata.holidays.clear();
    let mut trace = environment_from_weather(&coverage, &weather_coverage, weather_path)?;
    let object = trace
        .as_object_mut()
        .ok_or("Source input trace is not an object")?;
    for key in [
        "active_special_day_count",
        "active_dst_step_count",
        "daylight_saving",
        "schedule_values",
        "prepared_calendar",
    ] {
        object.remove(key);
    }
    if let Some(first) = object.get_mut("first").and_then(Value::as_object_mut) {
        first.remove("dst");
        first.remove("day_type");
    }
    object.insert(
        "source".into(),
        json!("Rust coverage-only EPW/civil-date preparation; Source calendar values deferred"),
    );
    object.insert("trace_semantics".into(), json!("coverage inputs only; actual Source calendar/schedule children execute from original raw input and live header owners"));
    object.insert("source_calendar_controls".into(), json!({
        "status": "deferred_to_execution",
        "execution_owner": "actual Source raw header/getters and SetDSTDateRanges/SetSpecialDayDates",
        "input_file_DST_declared": model.run_period_daylight_saving_time.is_some(),
        "input_file_special_day_declarations": model.run_period_special_days.len(),
        "weather_file_DST_declared": weather.calendar_metadata.daylight_saving_period.is_some(),
        "weather_file_special_day_declarations": weather.calendar_metadata.holidays.len(),
        "preview_values_used_by_Source": false,
    }));
    Ok(trace)
}

pub(crate) fn validate_source_environment(
    environment: &Value,
    _admission: &SourceCalendarInputAdmission,
) -> Vec<String> {
    let mut errors = super::validate_porting_environment(environment);
    if environment["source_calendar_controls"]["status"] != "deferred_to_execution"
        || environment["source_calendar_controls"]["execution_owner"]
            != "actual Source raw header/getters and SetDSTDateRanges/SetSpecialDayDates"
    {
        errors.push("Source calendar preparation lacks its validated execution-owner fact".into());
    }
    errors
}

pub(super) fn environment_from_weather(
    model: &TypedModel,
    weather: &ep_runtime::weather::EpwWeatherFile,
    weather_path: &str,
) -> Result<Value, String> {
    let hourly = build_hourly_time_axis_with_weather_metadata(model, &weather.calendar_metadata)
        .map_err(|error| error.to_string())?;
    let selected =
        select_epw_environment_weather(weather, &hourly).map_err(|error| error.to_string())?;
    let environments =
        build_environment_time_axes_with_weather_metadata(model, &weather.calendar_metadata)
            .map_err(|error| error.to_string())?;
    let axis = environments
        .first()
        .ok_or_else(|| "no weather environment was resolved".to_string())?;
    let series = precompute_weather_timestep_series(
        selected.hourly_records(),
        hourly.zone_timestep.timesteps_per_hour,
        hourly.first_hour_interpolation_starting_values,
    );
    let schedules = precompute_schedule_value_series_for_environment_time_axis(model, axis);
    let first = axis.points.first().map(|point| json!({
        "year": point.year, "month": point.month, "day_of_month": point.day_of_month,
        "day_of_week": format!("{:?}", point.day_of_week), "day_type": format!("{:?}", point.day_type),
        "day_of_year": point.day_of_year, "schedule_day_of_year": point.schedule_day_of_year,
        "hour": point.hour, "zone_timestep": point.zone_timestep,
        "end_minute": point.end_minute, "dst": point.dst,
    }));
    let last = axis.points.last().map(|point| json!({
        "year": point.year, "month": point.month, "day_of_month": point.day_of_month,
        "hour": point.hour, "zone_timestep": point.zone_timestep, "end_minute": point.end_minute,
    }));
    Ok(json!({
        "source": "Rust production EPW/calendar/schedule preparation",
        "weather_path": weather_path,
        "weather_record_year": selected.hourly_records().first().map(|record| record.year),
        "calendar_year": axis.calendar.start_year,
        "rust_materialized_environment_index": axis.environment_index,
        "environment_ordinal_semantics": "Rust materialized index excludes disabled design-day definitions; compare EP native Envrn separately",
        "resolved_start_year": axis.calendar.start_year,
        "resolved_start_day_of_week": format!("{:?}", axis.calendar.start_day_of_week),
        "hourly_samples": hourly.sample_count(), "zone_timestep_samples": axis.sample_count(),
        "timesteps_per_hour": hourly.zone_timestep.timesteps_per_hour,
        "active_special_day_count": axis.points.iter().filter(|point| point.special_day_type.is_some())
            .map(|point|point.day_of_sim).collect::<std::collections::BTreeSet<_>>().len(),
        "active_dst_step_count": axis.points.iter().filter(|point|point.dst).count(),
        "first": first, "last": last,
        "prepared_calendar": crate::clock_trace::prepared_calendar(&hourly),
        "daylight_saving": {
            "active": axis.daylight_saving.active,
            "effective_source": axis.daylight_saving.effective_source.as_str(),
            "run_period_uses_weather_file_period": axis.daylight_saving.run_period_uses_weather_file_period,
            "weather_file_period_declared": axis.daylight_saving.weather_file_period_declared,
            "input_file_period_declared": axis.daylight_saving.input_file_period_declared,
        },
        "source_start_record_index": selected.source_start_record_index,
        "selected_source_record_indices": selected.selected_source_record_indices,
        "first_weather_steps": series.timestep_samples().iter().take(8).map(|sample| json!({
            "record_index": sample.record_index, "timestep": sample.timestep,
            "dry_bulb_c": sample.dry_bulb_c, "wet_bulb_c": sample.wet_bulb_c,
            "relative_humidity_percent": sample.relative_humidity_percent,
            "humidity_ratio": sample.outdoor_humidity_ratio, "pressure_pa": sample.atmospheric_pressure_pa,
            "wind_speed_m_per_s": sample.wind_speed_m_per_s,
            "direct_normal_radiation_w_per_m2": sample.direct_normal_radiation_w_per_m2,
            "diffuse_horizontal_radiation_w_per_m2": sample.diffuse_horizontal_radiation_w_per_m2,
        })).collect::<Vec<_>>(),
        "schedule_values": schedules.iter().map(|schedule| json!({
            "id": schedule.schedule_id.0, "name": schedule.schedule_name, "values": schedule.values,
        })).collect::<Vec<_>>(),
        "trace_semantics": "prepared inputs only; weather/schedule numerical completion is gated by CLK/SCH cards",
    }))
}
