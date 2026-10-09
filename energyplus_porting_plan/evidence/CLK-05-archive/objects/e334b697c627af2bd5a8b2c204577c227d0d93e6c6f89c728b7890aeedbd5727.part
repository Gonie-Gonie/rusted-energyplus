//! Typed literal public owner writes; no expected weather/interpolation answers.
use super::{Result, inputs::{array, flag, real, require}};
use ep_runtime::weather::day::{WeatherDayState, WeatherVarCounts, WeatherVars};
use serde_json::Value;

fn count(value: &Value) -> Result<i32> {
    let value = value.as_i64().ok_or("typed bounded count input required")?;
    require((0..=1_000_000).contains(&value), "count outside nonoverflow domain")?;
    Ok(i32::try_from(value)?)
}
fn counts(owner: &mut WeatherVarCounts, input: &Value) -> Result<()> {
    for (key, value) in input.as_object().ok_or("count seed object required")? {
        let value = count(value)?;
        match key.as_str() {
            "OutDryBulbTemp" => owner.out_dry_bulb_temp = value,
            "OutDewPointTemp" => owner.out_dew_point_temp = value,
            "OutRelHum" => owner.out_rel_hum = value,
            "OutBaroPress" => owner.out_baro_press = value,
            "WindDir" => owner.wind_dir = value,
            "WindSpeed" => owner.wind_speed = value,
            "BeamSolarRad" => owner.beam_solar_rad = value,
            "DifSolarRad" => owner.dif_solar_rad = value,
            "TotalSkyCover" => owner.total_sky_cover = value,
            "OpaqueSkyCover" => owner.opaque_sky_cover = value,
            "Visibility" => owner.visibility = value,
            "Ceiling" => owner.ceiling = value,
            "LiquidPrecip" => owner.liquid_precip = value,
            "WaterPrecip" => owner.water_precip = value,
            "AerOptDepth" => owner.aer_opt_depth = value,
            "SnowDepth" => owner.snow_depth = value,
            "DaysLastSnow" => owner.days_last_snow = value,
            "WeathCodes" => owner.weath_codes = value,
            "Albedo" => owner.albedo = value,
            _ => return Err(format!("unknown count seed {key}").into()),
        }
    }
    Ok(())
}
fn weather(owner: &mut WeatherVars, input: &Value) -> Result<()> {
    for (key, value) in input.as_object().ok_or("weather seed object required")? {
        match key.as_str() {
            "IsRain" => owner.is_rain = flag(value)?,
            "IsSnow" => owner.is_snow = flag(value)?,
            "OutDryBulbTemp" => owner.out_dry_bulb_temp = real(value)?,
            "OutDewPointTemp" => owner.out_dew_point_temp = real(value)?,
            "OutBaroPress" => owner.out_baro_press = real(value)?,
            "OutRelHum" => owner.out_rel_hum = real(value)?,
            "WindSpeed" => owner.wind_speed = real(value)?,
            "WindDir" => owner.wind_dir = real(value)?,
            "SkyTemp" => owner.sky_temp = real(value)?,
            "HorizIRSky" => owner.horiz_ir_sky = real(value)?,
            "BeamSolarRad" => owner.beam_solar_rad = real(value)?,
            "DifSolarRad" => owner.dif_solar_rad = real(value)?,
            "Albedo" => owner.albedo = real(value)?,
            "WaterPrecip" => owner.water_precip = real(value)?,
            "LiquidPrecip" => owner.liquid_precip = real(value)?,
            "TotalSkyCover" => owner.total_sky_cover = real(value)?,
            "OpaqueSkyCover" => owner.opaque_sky_cover = real(value)?,
            _ => return Err(format!("unknown weather seed {key}").into()),
        }
    }
    Ok(())
}
pub(super) fn context(state: &mut WeatherDayState, input: &Value) -> Result<()> {
    if let Some(seed) = input.get("caller_owned_seed") {
        for (key, value) in seed.as_object().ok_or("literal caller seed required")? {
            match key.as_str() {
                "missed_counts" => counts(&mut state.missed_counts, value)?,
                "out_of_range_counts" => counts(&mut state.out_of_range_counts, value)?,
                "next_hour" => weather(&mut state.next_hour, value)?,
                "last_hour" => weather(&mut state.last_hour, value)?,
                "weather" => {
                    require(value.as_object().is_some_and(|v| v.len() == 1 && v.contains_key("LastHourSet")),
                        "only LastHourSet weather seed admitted")?;
                    state.weather.last_hour_set = flag(&value["LastHourSet"])?;
                },
                _ => return Err(format!("unknown closed seed group {key}").into()),
            }
        }
    }
    if let Some(override_input) = input.get("stored_solar_interpolation_override") {
        let values = array(override_input)?;
        let steps = usize::try_from(state.global.time_steps_in_hour)?;
        let owner = state.weather.solar_interpolation.as_mut().ok_or("actual Setup solar allocation required")?;
        require(owner.len() == steps && values.len() == steps, "literal stored override length differs")?;
        for (target, value) in owner.iter_mut().zip(values) {*target = real(value)?;}
    }
    // All four requested controls remain unavailable/unapplied in this baseline.
    Ok(())
}
