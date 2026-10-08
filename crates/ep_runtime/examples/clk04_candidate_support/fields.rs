//! Output-only serialization of actual weather state and storage.
//! Unavailable owners remain explicit; this module performs no weather calculations.
use super::{legacy_fields, raw_dto};
use ep_runtime::weather::day::{WeatherDayState, WeatherSession, WeatherVars};
use ep_runtime::weather::raw::RawEpwCursorOwner;
use serde_json::{Value, json};
use std::path::Path;

fn unavailable(reason: &str) -> Value {
    json!({"available":false,"value":null,"reason":reason,"counted_as_PASS":false})
}
fn weather(value: &WeatherVars) -> Value {
    let mut result = json!({"IsRain":value.is_rain,"IsSnow":value.is_snow});
    for (key, number) in [
        ("OutDryBulbTemp", value.out_dry_bulb_temp),
        ("OutDewPointTemp", value.out_dew_point_temp),
        ("OutBaroPress", value.out_baro_press),
        ("OutRelHum", value.out_rel_hum),
        ("WindSpeed", value.wind_speed),
        ("WindDir", value.wind_dir),
        ("SkyTemp", value.sky_temp),
        ("HorizIRSky", value.horiz_ir_sky),
        ("BeamSolarRad", value.beam_solar_rad),
        ("DifSolarRad", value.dif_solar_rad),
        ("Albedo", value.albedo),
        ("WaterPrecip", value.water_precip),
        ("LiquidPrecip", value.liquid_precip),
        ("TotalSkyCover", value.total_sky_cover),
        ("OpaqueSkyCover", value.opaque_sky_cover),
    ] {
        result[key] = raw_dto::scalar(number);
    }
    result
}

pub(super) fn selected(state: &WeatherDayState) -> Value {
    if let (Ok(hour), Ok(step)) = (
        usize::try_from(state.global.hour_of_day),
        usize::try_from(state.global.time_step),
    ) && let Ok(values) = state.today_values.hour(hour)
        && let Some(value) = step.checked_sub(1).and_then(|index| values.get(index))
    {
        return json!({"hour":hour,"time_step":step,"value":weather(value)});
    }
    Value::Null
}

pub(super) fn state_snapshot(
    state: &WeatherDayState,
    cursor: Option<&RawEpwCursorOwner>,
    file: Option<&Path>,
    full_grids: bool,
) -> Value {
    let mut result = legacy_fields::snapshot(state, cursor, file);
    if cursor.is_none() {
        for key in [
            "header",
            "stream",
            "cursor_observations",
            "selected_environment_context",
        ] {
            result[key] = unavailable(
                "pure state-level Setup owns no input/header/stream/selected-environment registry",
            );
        }
    }
    for key in ["today_values", "tomorrow_values"] {
        if !full_grids {
            result[key]["slots"] = Value::Null;
            result[key]["contents_available"] = json!(false);
            result[key]["unavailability_reason"] =
                json!("pre-execution declared compact unselected observation");
        }
    }
    result["full_grid_contents_available"] = json!(full_grids);
    result["observation_detail"] = json!(if full_grids {
        "full-selected-or-lifecycle-boundary"
    } else {
        "compact-unselected-operation"
    });
    result["full_17_field_generation_parity_claimed"] = json!(false);
    for (name, storage) in [
        ("Interpolation", &state.weather.interpolation),
        ("SolarInterpolation", &state.weather.solar_interpolation),
    ] {
        result[format!("{name}_allocated")] = json!(storage.is_some());
        result[name] = json!(
            storage
                .as_ref()
                .map(|values| values
                    .iter()
                    .copied()
                    .map(raw_dto::scalar)
                    .collect::<Vec<_>>())
                .unwrap_or_default()
        );
    }
    let current = &state.environment.current_weather;
    let mut current_values = json!({"IsRain":current.is_rain});
    for (name, value) in [
        ("OutDryBulbTemp", current.out_dry_bulb_temp),
        ("OutDewPointTemp", current.out_dew_point_temp),
        ("OutBaroPress", current.out_baro_press),
        ("OutRelHum", current.out_rel_hum),
        ("OutRelHumValue", current.out_rel_hum_value),
        ("OutHumRat", current.out_hum_rat),
        ("OutWetBulbTemp", current.out_wet_bulb_temp),
        ("WindSpeed", current.wind_speed),
        ("WindDir", current.wind_dir),
        ("LiquidPrecipitation", current.liquid_precipitation),
        ("OutEnthalpy", current.out_enthalpy),
        ("OutAirDensity", current.out_air_density),
    ] {
        current_values[name] = raw_dto::scalar(value);
    }
    result["clk04_observation"] = json!({
        "current_non_solar_weather":current_values,
        "selected_today_slot":selected(state),
        "weights":{
            "WeightNow":raw_dto::scalar(state.global.weight_now),
            "WeightPreviousHour":raw_dto::scalar(state.global.weight_previous_hour)},
        "clock":{
            "CurrentTime":raw_dto::scalar(state.global.current_time),
            "SimTimeSteps":state.global.sim_time_steps,
            "TimeStepZoneSec":raw_dto::scalar(state.global.time_step_zone_sec),
            "MinutesInTimeStep":state.global.minutes_in_time_step},
        "selected_weather_control":{
            "NextHour":state.weather.next_hour,
            "UseRainValues":state.weather.use_rain_values,
            "UseSnowValues":state.weather.use_snow_values,
            "RptIsRain":state.weather.rpt_is_rain},
        "actual_ems_overrides":unavailable("EMS registration and override owners are outside the declared Rust scope"),
        "unpaired_genuine_context":unavailable("whole ground, water, solar and daylight owners are outside the declared Rust scope"),
        "native_internal_record_index_observed":false,"error_text_parity_claimed":false});
    result["unowned_native_fields"] = json!([
        "EnvironmentData records",
        "DSTIndex/SpecialDayTypes/WeekDayTypes source-only arrays",
        "whole native Factory constructor/constant initialization/input preparation",
        "native std-ios private bit codes and source error_state_to_string text",
        "ground, water, solar/daylight and EMS owners"
    ]);
    result
}

pub(super) fn session_snapshot(session: &WeatherSession, file: &Path, full_grids: bool) -> Value {
    let mut result = state_snapshot(
        &session.state,
        Some(&session.cursor),
        Some(file),
        full_grids,
    );
    result["selected_environment_context"] =
        json!({"CurrentCycle":session.current_cycle,"SetWeekDays":session.set_week_days});
    result["actual_rust_available"] = json!(session.available);
    result["actual_rust_errors_found"] = json!(session.errors_found);
    result["actual_rust_print_environment_stamp"] = json!(session.print_environment_stamp);
    result
}
