//! Actual owner reads; absent native current-weather/Setup owners remain null.
use super::{legacy_fields, raw_dto};
use ep_runtime::weather::day::{WeatherSession, WeatherVars};
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn unavailable(reason: &str) -> Value {
    json!({"available":false,"value":null,"reason":reason,"counted_as_PASS":false})
}
pub(super) fn slot(value: &WeatherVars) -> Value {
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
pub(super) fn selected(session: &WeatherSession) -> Value {
    let hour = session.state.global.hour_of_day;
    let step = session.state.global.time_step;
    if let (Ok(hour), Ok(step)) = (usize::try_from(hour), usize::try_from(step))
        && let Ok(values) = session.state.today_values.hour(hour)
        && let Some(value) = step.checked_sub(1).and_then(|index| values.get(index))
    {
        let value = slot(value);
        let finite = value.as_object().is_some_and(|fields| {
            fields.values().all(|field| {
                field.is_boolean() || field["value"].as_f64().is_some_and(f64::is_finite)
            })
        });
        return json!({"available":true,"hour":hour,"time_step":step,"value":value,
            "all_fifteen_reals_finite":finite,"existing_Today_storage_observed":true,
            "native_SetCurrentWeather_result_claimed":false,"counted_as_PASS":false});
    }
    unavailable("actual current Rust Today slot is outside the allocated indexed owner")
}
pub(super) fn snapshot(session: &WeatherSession, weather: &Path, full_grids: bool) -> Value {
    let mut result = legacy_fields::snapshot(&session.state, Some(&session.cursor), Some(weather));
    for key in ["today_values", "tomorrow_values"] {
        if !full_grids {
            result[key]["slots"] = Value::Null;
        }
        result[key]["full_contents_available"] = json!(full_grids);
        result[key]["full_contents_availability_reason"] = if full_grids {
            Value::Null
        } else {
            json!("unavailable_by_predeclared_projection")
        };
        result[key]["unavailable_counted_as_PASS"] = json!(false);
    }
    result["selected_environment_context"] =
        json!({"CurrentCycle":session.current_cycle,"SetWeekDays":session.set_week_days});
    result["actual_rust_available"] = json!(session.available);
    result["actual_rust_errors_found"] = json!(session.errors_found);
    result["actual_rust_print_environment_stamp"] = json!(session.print_environment_stamp);
    result["clk04_observation"] = json!({
        "current_non_solar_weather":unavailable("existing mutable SetCurrentWeather environment owner absent"),
        "legacy_selected_today_slot":selected(session),
        "weights":unavailable("existing stored WeightNow and WeightPreviousHour owners absent"),
        "clock":unavailable("existing CurrentTime, SimTimeSteps, seconds and minutes owners absent"),
        "selected_weather_control":{
            "NextHour":unavailable("existing SetCurrentWeather NextHour owner absent"),
            "RptIsRain":unavailable("existing SetCurrentWeather reporting rain owner absent"),
            "UseRainValues":unavailable("existing native weather UseRainValues owner absent"),
            "UseSnowValues":unavailable("existing native weather UseSnowValues owner absent")},
        "existing_configuration_rain_snow_inputs":{
            "owner":"WeatherSession.configuration.run_period",
            "use_weather_file_rain_indicators":session.configuration.run_period.use_weather_file_rain_indicators,
            "use_weather_file_snow_indicators":session.configuration.run_period.use_weather_file_snow_indicators,
            "native_weather_owner_equivalence_claimed":false,"counted_as_PASS":false},
        "actual_ems_overrides":unavailable("whole existing native EMS registration absent"),
        "whole_ground_water_solar_daylight_math_paired":false,
        "native_internal_record_index_observed":false,"error_text_parity_claimed":false});
    result
}
