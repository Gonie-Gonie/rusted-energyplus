//! Passive observations of actual existing owners; no cloud or sky calculations.
use super::{existing_current_fields, raw_dto};
use ep_runtime::weather::day::{WeatherDayState, WeatherSession, WeatherVars};
use ep_runtime::weather::raw::RawEpwCursorOwner;
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn unavailable(reason: &str) -> Value {
    json!({"available":false,"value":null,"reason":reason,"counted_as_PASS":false})
}
fn sky(value: &WeatherVars) -> Value {
    json!({"SkyTemp":raw_dto::scalar(value.sky_temp),
        "HorizIRSky":raw_dto::scalar(value.horiz_ir_sky),
        "TotalSkyCover":raw_dto::scalar(value.total_sky_cover),
        "OpaqueSkyCover":raw_dto::scalar(value.opaque_sky_cover)})
}
fn selected(state: &WeatherDayState) -> Value {
    if let (Ok(hour), Ok(step)) = (
        usize::try_from(state.global.hour_of_day),
        usize::try_from(state.global.time_step),
    ) && let Ok(values) = state.today_values.hour(hour)
        && let Some(value) = step.checked_sub(1).and_then(|i| values.get(i))
    {
        return json!({"hour":hour,"time_step":step,"value":sky(value)});
    }
    Value::Null
}
pub(super) fn state_snapshot(
    state: &WeatherDayState,
    cursor: Option<&RawEpwCursorOwner>,
    file: Option<&Path>,
    full: bool,
) -> Value {
    // Shared passive serializers retain all scalar/header/cursor fields, counts19,
    // complete17-field LastHour/NextHour/missing storage and actual Current13.
    let mut result = existing_current_fields::state_snapshot(state, cursor, file, full);
    for key in ["today_values", "tomorrow_values"] {
        result[key]["contents_available"] = json!(full);
        result[key]["observed_fields"] =
            json!(["SkyTemp", "HorizIRSky", "TotalSkyCover", "OpaqueSkyCover"]);
        if full {
            if let Some(slots) = result[key]["slots"].as_array_mut() {
                for slot in slots {
                    let old = &slot["value"];
                    slot["value"] = json!({"SkyTemp":old["SkyTemp"],"HorizIRSky":old["HorizIRSky"],
                        "TotalSkyCover":old["TotalSkyCover"],"OpaqueSkyCover":old["OpaqueSkyCover"]});
                }
            }
        } else {
            result[key]["slots"] = Value::Null;
            result[key]["unavailability_reason"] =
                json!("pre-execution declared compact unselected sky/cloud grid");
        }
    }
    // Current13 is passive context only. Its full17 selected slot does not become
    // an additional cross-engine grid generation observation for this card.
    let current = result["clk04_observation"].clone();
    if let Some(value) = result.as_object_mut() {
        value.remove("clk04_observation");
    }
    let e = &state.environment;
    let mut globals = json!({});
    for name in [
        "weather.HorizIRSky",
        "environment.SkyTemp",
        "environment.SkyTempKelvin",
        "environment.TotalCloudCover",
        "environment.OpaqueCloudCover",
    ] {
        globals[name] = unavailable(
            "no corresponding existing Rust current-sky global owner; native-only/unpaired",
        );
    }
    result["clk06_observation"] = json!({"selected_today_sky_slot":selected(state),
        "caller_controls":{"DisplayWeatherMissingDataWarnings":e.display_weather_missing_data_warnings,
            "IgnoreSolarRadiation":e.ignore_solar_radiation,"IgnoreBeamRadiation":e.ignore_beam_radiation,
            "IgnoreDiffuseRadiation":e.ignore_diffuse_radiation},
        "actual_sky_environment":unavailable("existing Rust has no WeatherProperty sky model/useIR selected Environment registry"),
        "native_current_sky_globals":globals,"native_current_sky_globals_paired":false,
        "existing_current_context":current,"private_hourly_ESky_or_branch_locals_observed":false,
        "whole_other_current_physics_paired":false});
    result["full_sky_grid_contents_available"] = json!(full);
    result["full_17_field_grid_generation_parity_claimed"] = json!(false);
    result["whole_native_preparation_available"] = json!(false);
    result["unavailable_counted_as_PASS"] = json!(false);
    result
}
pub(super) fn session_snapshot(session: &WeatherSession, file: &Path, full: bool) -> Value {
    let mut result = state_snapshot(&session.state, Some(&session.cursor), Some(file), full);
    result["selected_environment_context"] =
        json!({"CurrentCycle":session.current_cycle,"SetWeekDays":session.set_week_days});
    result["actual_rust_available"] = json!(session.available);
    result["actual_rust_errors_found"] = json!(session.errors_found);
    result["actual_rust_print_environment_stamp"] = json!(session.print_environment_stamp);
    result
}
