//! Reuse the production EPW/calendar/schedule preparation for admission traces.

use ep_model::TypedModel;
use ep_runtime::schedules::precompute_schedule_value_series_for_environment_time_axis;
use ep_runtime::{
    build_environment_time_axes_with_weather_metadata,
    build_hourly_time_axis_with_weather_metadata, load_epw_weather_file,
    precompute_weather_timestep_series, select_epw_environment_weather,
};
use serde_json::{Value, json};
use std::path::Path;

pub(crate) fn porting_environment_trace(
    model: &TypedModel,
    weather_path: &Path,
) -> Result<Value, String> {
    let weather = load_epw_weather_file(weather_path).map_err(|error| error.to_string())?;
    environment_from_weather(model, &weather, &weather_path.to_string_lossy())
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
