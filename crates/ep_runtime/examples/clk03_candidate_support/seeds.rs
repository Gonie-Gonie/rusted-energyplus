//! Literal caller seeding, preserving source partial-write canaries and finite input bits.
use super::{
    Result,
    inputs::{array, flag, require},
};
use ep_runtime::weather::day::{
    DailyWeatherVariables, ExtendedWeatherVars, WeatherDayState, WeatherDayValues,
    WeatherEnvironmentState, WeatherGlobalState, WeatherOwnerState, WeatherVarCounts, WeatherVars,
};
use serde_json::Value;

fn int(value: &Value) -> Result<i32> {
    Ok(i32::try_from(
        value
            .as_i64()
            .ok_or("typed signed integer input required")?,
    )?)
}
fn real(value: &Value) -> Result<f64> {
    let object = value.as_object().ok_or("binary64 input object required")?;
    let bits = value["bits"]
        .as_str()
        .ok_or("binary64 input bits required")?;
    require(
        object.len() == 1
            && bits.len() == 16
            && bits
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "exact lower-case binary64 bits required",
    )?;
    let result = f64::from_bits(u64::from_str_radix(bits, 16)?);
    require(result.is_finite(), "finite binary64 inputs required")?;
    Ok(result)
}
fn weather(owner: &mut WeatherVars, input: &Value) -> Result<()> {
    for (name, value) in input.as_object().ok_or("typed canary object required")? {
        match name.as_str() {
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
            _ => return Err(format!("unknown caller field {name}").into()),
        }
    }
    Ok(())
}

fn day(owner: &mut DailyWeatherVariables, input: &Value) -> Result<()> {
    for (name, value) in input.as_object().ok_or("typed canary object required")? {
        match name.as_str() {
            "DayOfYear" => owner.day_of_year = int(value)?,
            "DayOfYear_Schedule" => owner.day_of_year_schedule = int(value)?,
            "Year" => owner.year = int(value)?,
            "Month" => owner.month = int(value)?,
            "DayOfMonth" => owner.day_of_month = int(value)?,
            "DayOfWeek" => owner.day_of_week = int(value)?,
            "DaylightSavingIndex" => owner.daylight_saving_index = int(value)?,
            "HolidayIndex" => owner.holiday_index = int(value)?,
            "SinSolarDeclinAngle" => owner.sin_solar_declin_angle = real(value)?,
            "CosSolarDeclinAngle" => owner.cos_solar_declin_angle = real(value)?,
            "EquationOfTime" => owner.equation_of_time = real(value)?,
            _ => return Err(format!("unknown caller field {name}").into()),
        }
    }
    Ok(())
}

fn counts(owner: &mut WeatherVarCounts, input: &Value) -> Result<()> {
    for (name, value) in input.as_object().ok_or("typed canary object required")? {
        match name.as_str() {
            "OutDryBulbTemp" => owner.out_dry_bulb_temp = int(value)?,
            "OutDewPointTemp" => owner.out_dew_point_temp = int(value)?,
            "OutRelHum" => owner.out_rel_hum = int(value)?,
            "OutBaroPress" => owner.out_baro_press = int(value)?,
            "WindDir" => owner.wind_dir = int(value)?,
            "WindSpeed" => owner.wind_speed = int(value)?,
            "BeamSolarRad" => owner.beam_solar_rad = int(value)?,
            "DifSolarRad" => owner.dif_solar_rad = int(value)?,
            "TotalSkyCover" => owner.total_sky_cover = int(value)?,
            "OpaqueSkyCover" => owner.opaque_sky_cover = int(value)?,
            "Visibility" => owner.visibility = int(value)?,
            "Ceiling" => owner.ceiling = int(value)?,
            "LiquidPrecip" => owner.liquid_precip = int(value)?,
            "WaterPrecip" => owner.water_precip = int(value)?,
            "AerOptDepth" => owner.aer_opt_depth = int(value)?,
            "SnowDepth" => owner.snow_depth = int(value)?,
            "DaysLastSnow" => owner.days_last_snow = int(value)?,
            "WeathCodes" => owner.weath_codes = int(value)?,
            "Albedo" => owner.albedo = int(value)?,
            _ => return Err(format!("unknown caller field {name}").into()),
        }
    }
    Ok(())
}

fn global(owner: &mut WeatherGlobalState, input: &Value) -> Result<()> {
    for (name, value) in input.as_object().ok_or("typed canary object required")? {
        match name.as_str() {
            "BeginSimFlag" => owner.begin_sim_flag = flag(value)?,
            "BeginEnvrnFlag" => owner.begin_envrn_flag = flag(value)?,
            "BeginDayFlag" => owner.begin_day_flag = flag(value)?,
            "BeginHourFlag" => owner.begin_hour_flag = flag(value)?,
            "BeginTimeStepFlag" => owner.begin_time_step_flag = flag(value)?,
            "EndDayFlag" => owner.end_day_flag = flag(value)?,
            "EndHourFlag" => owner.end_hour_flag = flag(value)?,
            "EndEnvrnFlag" => owner.end_envrn_flag = flag(value)?,
            "EndDesignDayEnvrnsFlag" => owner.end_design_day_envrns_flag = flag(value)?,
            "WarmupFlag" => owner.warmup_flag = flag(value)?,
            "DayOfSim" => owner.day_of_sim = int(value)?,
            "CalendarYear" => owner.calendar_year = int(value)?,
            "PreviousHour" => owner.previous_hour = int(value)?,
            "HourOfDay" => owner.hour_of_day = int(value)?,
            "NumOfDayInEnvrn" => owner.num_of_day_in_envrn = int(value)?,
            "TimeStepsInHour" => owner.time_steps_in_hour = int(value)?,
            "TimeStep" => owner.time_step = int(value)?,
            _ => return Err(format!("unknown caller field {name}").into()),
        }
    }
    Ok(())
}

fn environment(owner: &mut WeatherEnvironmentState, input: &Value) -> Result<()> {
    for (name, value) in input.as_object().ok_or("typed canary object required")? {
        match name.as_str() {
            "DayOfYear" => owner.day_of_year = int(value)?,
            "DayOfYear_Schedule" => owner.day_of_year_schedule = int(value)?,
            "Year" => owner.year = int(value)?,
            "Month" => owner.month = int(value)?,
            "DayOfMonth" => owner.day_of_month = int(value)?,
            "DayOfWeek" => owner.day_of_week = int(value)?,
            "HolidayIndex" => owner.holiday_index = int(value)?,
            "DSTIndicator" => owner.dst_indicator = int(value)?,
            "YearTomorrow" => owner.year_tomorrow = int(value)?,
            "MonthTomorrow" => owner.month_tomorrow = int(value)?,
            "DayOfMonthTomorrow" => owner.day_of_month_tomorrow = int(value)?,
            "DayOfWeekTomorrow" => owner.day_of_week_tomorrow = int(value)?,
            "HolidayIndexTomorrow" => owner.holiday_index_tomorrow = int(value)?,
            "SinSolarDeclinAngle" => owner.sin_solar_declin_angle = real(value)?,
            "CosSolarDeclinAngle" => owner.cos_solar_declin_angle = real(value)?,
            "EquationOfTime" => owner.equation_of_time = real(value)?,
            _ => return Err(format!("unknown caller field {name}").into()),
        }
    }
    Ok(())
}

fn owner(owner: &mut WeatherOwnerState, input: &Value) -> Result<()> {
    for (name, value) in input.as_object().ok_or("typed canary object required")? {
        match name.as_str() {
            "RptDayType" => owner.rpt_day_type = int(value)?,
            "CurDayOfWeek" => owner.cur_day_of_week = int(value)?,
            "LastHourSet" => owner.last_hour_set = flag(value)?,
            "ReadEPlusWeatherCurTime" => owner.read_e_plus_weather_cur_time = real(value)?,
            _ => return Err(format!("unknown caller field {name}").into()),
        }
    }
    Ok(())
}

fn slots(owner: &mut WeatherDayValues, input: &Value) -> Result<()> {
    let values = array(input)?;
    require(
        owner.is_allocated()
            && owner.time_steps() == 4
            && owner.hours() == 24
            && values.len() == 96,
        "declared whole4x24 carrier required",
    )?;
    for (target, value) in owner.slots_mut().iter_mut().zip(values) {
        weather(target, value)?;
    }
    Ok(())
}
fn missing(owner: &mut ExtendedWeatherVars, input: &Value) -> Result<()> {
    let mut base = input
        .as_object()
        .ok_or("missing canary object required")?
        .clone();
    for key in [
        "Visibility",
        "Ceiling",
        "AerOptDepth",
        "SnowDepth",
        "DaysLastSnow",
    ] {
        base.remove(key);
    }
    weather(&mut owner.base, &Value::Object(base))?;
    if let Some(value) = input.get("Visibility") {
        owner.visibility = real(value)?;
    }
    if let Some(value) = input.get("Ceiling") {
        owner.ceiling = real(value)?;
    }
    if let Some(value) = input.get("AerOptDepth") {
        owner.aer_opt_depth = real(value)?;
    }
    if let Some(value) = input.get("SnowDepth") {
        owner.snow_depth = real(value)?;
    }
    if let Some(value) = input.get("DaysLastSnow") {
        owner.days_last_snow = int(value)?;
    }
    Ok(())
}
pub(super) fn seed(state: &mut WeatherDayState, input: &Value) -> Result<()> {
    for (name, value) in input.as_object().ok_or("caller state object required")? {
        match name.as_str() {
            "global" => global(&mut state.global, value)?,
            "environment" => environment(&mut state.environment, value)?,
            "weather" => owner(&mut state.weather, value)?,
            "today_variables" => day(&mut state.today_variables, value)?,
            "tomorrow_variables" => day(&mut state.tomorrow_variables, value)?,
            "today_values" => slots(&mut state.today_values, value)?,
            "tomorrow_values" => slots(&mut state.tomorrow_values, value)?,
            "last_hour" => weather(&mut state.last_hour, value)?,
            "next_hour" => weather(&mut state.next_hour, value)?,
            "missing_values" => missing(&mut state.missing_values, value)?,
            "missed_counts" => counts(&mut state.missed_counts, value)?,
            "out_of_range_counts" => counts(&mut state.out_of_range_counts, value)?,
            _ => return Err(format!("unknown caller group {name}").into()),
        }
    }
    require(
        state.global.time_steps_in_hour == 4,
        "caller changed frozen four-step allocation",
    )?;
    Ok(())
}
