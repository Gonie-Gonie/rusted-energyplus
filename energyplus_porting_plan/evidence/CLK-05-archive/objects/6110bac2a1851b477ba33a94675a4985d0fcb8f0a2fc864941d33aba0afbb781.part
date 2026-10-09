//! Output-only observations of existing public owners; no weather calculations.
use super::{legacy_fields, raw_dto};
use ep_runtime::weather::day::{WeatherDayState, WeatherSession};
use ep_runtime::weather::raw::RawEpwCursorOwner;
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn unavailable(reason: &str) -> Value {
    json!({"available":false,"value":null,"reason":reason,"counted_as_PASS":false})
}

fn controls() -> Value {
    let mut result = json!({});
    for name in [
        "DisplayWeatherMissingDataWarnings",
        "IgnoreSolarRadiation",
        "IgnoreBeamRadiation",
        "IgnoreDiffuseRadiation",
    ] {
        result[name] =
            unavailable("existing baseline has no owned solar-control registration or transport");
    }
    result
}

fn selected(state: &WeatherDayState) -> Value {
    if let (Ok(hour), Ok(step)) = (
        usize::try_from(state.global.hour_of_day),
        usize::try_from(state.global.time_step),
    ) {
        if let Ok(values) = state.today_values.hour(hour) {
            if let Some(value) = step.checked_sub(1).and_then(|i| values.get(i)) {
                return json!({"hour":hour,"time_step":step,"value":{
                    "BeamSolarRad":raw_dto::scalar(value.beam_solar_rad),
                    "DifSolarRad":raw_dto::scalar(value.dif_solar_rad)}});
            }
        }
    }
    Value::Null
}

pub(super) fn state_snapshot(
    state: &WeatherDayState,
    cursor: Option<&RawEpwCursorOwner>,
    file: Option<&Path>,
    full: bool,
) -> Value {
    // Prior-card serializer retains all scalar/calendar/header/cursor fields,
    // all19 counts and complete17-field LastHour/NextHour/missing storage.
    let mut result = legacy_fields::snapshot(state, cursor, file);
    if let Some(labels) = result["unowned_native_fields"].as_array_mut() {
        labels.retain(|label| {
            !matches!(
                label.as_str(),
                Some("Interpolation arrays" | "SolarInterpolation arrays")
            )
        });
    }
    if cursor.is_none() {
        for key in [
            "header",
            "stream",
            "cursor_observations",
            "selected_environment_context",
        ] {
            result[key] =
                unavailable("pure Setup state owns no header/input/selected-environment registry");
        }
    }
    for key in ["today_values", "tomorrow_values"] {
        result[key]["contents_available"] = json!(full);
        result[key]["observed_fields"] = json!(["BeamSolarRad", "DifSolarRad"]);
        if full {
            if let Some(slots) = result[key]["slots"].as_array_mut() {
                for slot in slots {
                    let beam = slot["value"]["BeamSolarRad"].clone();
                    let diffuse = slot["value"]["DifSolarRad"].clone();
                    slot["value"] = json!({"BeamSolarRad":beam,"DifSolarRad":diffuse});
                }
            }
        } else {
            result[key]["slots"] = Value::Null;
            result[key]["contents_available"] = json!(false);
            result[key]["unavailability_reason"] =
                json!("pre-execution declared compact unselected solar grid");
        }
    }
    result["full_solar_grid_contents_available"] = json!(full);
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
    result["clk05_observation"] = json!({"solar_controls":controls(),
        "selected_today_solar_slot":selected(state),"clock":{
            "TimeStepZoneSec":raw_dto::scalar(state.global.time_step_zone_sec),
            "MinutesInTimeStep":state.global.minutes_in_time_step,
            "CurrentTime":raw_dto::scalar(state.global.current_time),
            "SimTimeSteps":state.global.sim_time_steps},"private_hourly_or_weight_locals_observed":false,
        "current_solar_night_physics_paired":false});
    result["full_17_field_grid_generation_parity_claimed"] = json!(false);
    result["passive_CurrentTime_SimTimeSteps_or_CurrentWeather_parity_claimed"] = json!(false);
    result["whole_native_preparation_available"] = json!(false);
    result["unavailable_counted_as_PASS"] = json!(false);
    result
}

pub(super) fn session_snapshot(session: &WeatherSession, file: &Path, full: bool) -> Value {
    let mut result = state_snapshot(&session.state, Some(&session.cursor), Some(file), full);
    result["selected_environment_context"] = json!({"CurrentCycle":session.current_cycle,
        "SetWeekDays":session.set_week_days});
    result["actual_rust_available"] = json!(session.available);
    result["actual_rust_errors_found"] = json!(session.errors_found);
    result["actual_rust_print_environment_stamp"] = json!(session.print_environment_stamp);
    result
}
