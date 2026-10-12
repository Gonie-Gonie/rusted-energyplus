//! Compare the full weather arrays and the explicitly weather-owned clocks.
use super::{TestResult, real};
use crate::weather::day::{DailyWeatherVariables, WeatherDayState, WeatherVars};
use serde_json::Value;

pub(super) fn verify(state: &WeatherDayState, event: &Value, ordinal: usize) -> TestResult {
    let native = &event["weather"];
    for (name, values) in [
        ("wvarsHrTsToday", &state.today_values),
        ("wvarsHrTsTomorrow", &state.tomorrow_values),
    ] {
        let array = &native[name];
        assert_eq!(Some(values.is_allocated()), array["allocated"].as_bool());
        assert_eq!(Some(values.slots().len() as u64), array["size"].as_u64());
        assert_eq!(Some(values.time_steps() as u64), array["size1"].as_u64());
        assert_eq!(Some(values.hours() as u64), array["size2"].as_u64());
        assert_eq!(array["lower1"], 1);
        assert_eq!(array["lower2"], 1);
        assert_eq!(Some(values.time_steps() as u64), array["upper1"].as_u64());
        assert_eq!(Some(values.hours() as u64), array["upper2"].as_u64());
        let rows = array["rows"].as_array().ok_or("weather array rows")?;
        assert_eq!(values.slots().len(), rows.len());
        for (index, (actual, row)) in values.slots().iter().zip(rows).enumerate() {
            assert_eq!(row["hour"], index / values.time_steps() + 1);
            assert_eq!(row["timestep"], index % values.time_steps() + 1);
            weather(
                actual,
                &row["value"],
                &format!("call{ordinal}.{name}[{index}]"),
            )?;
        }
    }
    let year = state.today_variables.year;
    daily(&state.today_variables, &native["TodayVariables"], year)?;
    daily(
        &state.tomorrow_variables,
        &native["TomorrowVariables"],
        year,
    )?;
    let g = &state.global;
    let e = &state.environment;
    let w = &state.weather;
    for (key, value) in [
        ("Envrn", w.envrn),
        ("NumOfEnvrn", w.num_of_envrn),
        ("TotDesDays", e.tot_des_days),
        ("DayOfYear", e.day_of_year),
        ("DayOfYear_Schedule", e.day_of_year_schedule),
        ("DayOfWeek", e.day_of_week),
        ("HolidayIndex", e.holiday_index),
        ("DSTIndicator", e.dst_indicator),
        ("NextHour", w.next_hour),
        ("PreviousHour", g.previous_hour),
        ("SimTimeSteps", g.sim_time_steps),
        ("TimeStepsInHour", g.time_steps_in_hour),
    ] {
        assert_eq!(
            Some(i64::from(value)),
            native[key].as_i64(),
            "call{ordinal}.{key}"
        );
    }
    for (key, value) in [
        ("EndDayFlag", g.end_day_flag),
        ("EndHourFlag", g.end_hour_flag),
        ("EndEnvrnFlag", g.end_envrn_flag),
        ("EndDesignDayEnvrnsFlag", g.end_design_day_envrns_flag),
        ("EndMonthFlag", e.end_month_flag),
        ("EndYearFlag", e.end_year_flag),
        ("FirstCall", w.first_call),
        ("GetEnvironmentFirstCall", w.get_environment_first_call),
        ("PrintDDHeader", w.print_design_day_header),
        ("UseRainValues", w.use_rain_values),
        ("UseSnowValues", w.use_snow_values),
        ("DoDesDaySim", g.do_des_day_sim),
        ("DoWeathSim", g.do_weath_sim),
        ("RunPeriodEnvironment", e.run_period_environment),
    ] {
        assert_eq!(Some(value), native[key].as_bool(), "call{ordinal}.{key}");
    }
    assert_eq!(e.current_month_day, native["CurMnDy"]);
    assert_eq!(e.environment_start_end, native["EnvironmentStartEnd"]);
    for (name, values) in [
        ("Interpolation", &w.interpolation),
        ("SolarInterpolation", &w.solar_interpolation),
    ] {
        let values = values.as_ref().ok_or("interpolation allocation")?;
        let rows = native[name]["rows"]
            .as_array()
            .ok_or("Native interpolation")?;
        assert_eq!(values.len(), rows.len());
        for (actual, row) in values.iter().zip(rows) {
            real(*actual, &row["value"], name)?;
        }
    }
    real(
        w.time_step_fraction,
        &native["TimeStepFraction"],
        "TimeStepFraction",
    )?;
    let clock = &event["clock"];
    for (key, value) in [
        ("BeginSimFlag", g.begin_sim_flag),
        ("BeginEnvrnFlag", g.begin_envrn_flag),
        ("BeginDayFlag", g.begin_day_flag),
        ("BeginHourFlag", g.begin_hour_flag),
        ("WarmupFlag", g.warmup_flag),
    ] {
        assert_eq!(Some(value), clock[key].as_bool(), "call{ordinal}.{key}");
    }
    // BeginTimeStepFlag is cleared by downstream HB/HVAC. This weather-only
    // test must not pretend those consumers ran by applying their writes here.
    for (key, value) in [
        ("CalendarYear", g.calendar_year),
        ("DayOfSim", g.day_of_sim),
        ("HourOfDay", g.hour_of_day),
        ("TimeStep", g.time_step),
        ("KindOfSim", g.kind_of_sim),
        ("Month", e.month),
        ("DayOfMonth", e.day_of_month),
    ] {
        assert_eq!(
            Some(i64::from(value)),
            clock[key].as_i64(),
            "call{ordinal}.{key}"
        );
    }
    for (key, value) in [
        ("CurrentTime", g.current_time),
        ("TimeStepZone", g.time_step_zone),
        ("TimeStepZoneSec", g.time_step_zone_sec),
    ] {
        real(value, &clock[key], key)?;
    }
    current(state, native)?;
    Ok(())
}

pub(super) fn daily(actual: &DailyWeatherVariables, native: &Value, live_year: i32) -> TestResult {
    for (name, value) in [
        "DayOfYear",
        "DayOfYear_Schedule",
        "Year",
        "Month",
        "DayOfMonth",
        "DayOfWeek",
        "DaylightSavingIndex",
        "HolidayIndex",
    ]
    .into_iter()
    .zip(actual.integer_values())
    {
        if name == "Year" && native[name].as_i64() != Some(0) {
            assert_eq!(value, live_year, "live date_and_time year");
        } else {
            assert_eq!(
                Some(i64::from(value)),
                native[name].as_i64(),
                "daily.{name}"
            );
        }
    }
    for (name, value) in [
        "SinSolarDeclinAngle",
        "CosSolarDeclinAngle",
        "EquationOfTime",
    ]
    .into_iter()
    .zip(actual.real_values())
    {
        real(value, &native[name], name)?;
    }
    Ok(())
}

fn weather(actual: &WeatherVars, native: &Value, prefix: &str) -> TestResult {
    for (name, value) in ["IsRain", "IsSnow"]
        .into_iter()
        .zip(actual.boolean_values())
    {
        assert_eq!(Some(value), native[name].as_bool(), "{prefix}.{name}");
    }
    for (name, value) in [
        "OutDryBulbTemp",
        "OutDewPointTemp",
        "OutBaroPress",
        "OutRelHum",
        "WindSpeed",
        "WindDir",
        "SkyTemp",
        "HorizIRSky",
        "BeamSolarRad",
        "DifSolarRad",
        "Albedo",
        "WaterPrecip",
        "LiquidPrecip",
        "TotalSkyCover",
        "OpaqueSkyCover",
    ]
    .into_iter()
    .zip(actual.real_values())
    {
        real(value, &native[name], &format!("{prefix}.{name}"))?;
    }
    Ok(())
}

fn current(state: &WeatherDayState, native: &Value) -> TestResult {
    let c = state.environment.current_weather;
    let expected = &native["current"];
    for (name, value) in [
        ("IsRain", c.is_rain),
        ("IsSnow", c.is_snow),
        ("SunIsUp", c.sun_is_up),
        ("SunIsUpPrevTS", c.sun_is_up_previous_timestep),
    ] {
        assert_eq!(Some(value), expected[name].as_bool(), "current.{name}");
    }
    for (name, value) in [
        ("OutDryBulbTemp", c.out_dry_bulb_temp),
        ("OutDewPointTemp", c.out_dew_point_temp),
        ("OutBaroPress", c.out_baro_press),
        ("OutRelHum", c.out_rel_hum),
        ("OutRelHumValue", c.out_rel_hum_value),
        ("OutHumRat", c.out_hum_rat),
        ("OutWetBulbTemp", c.out_wet_bulb_temp),
        ("OutEnthalpy", c.out_enthalpy),
        ("OutAirDensity", c.out_air_density),
        ("WindSpeed", c.wind_speed),
        ("WindDir", c.wind_dir),
        ("LiquidPrecipitation", c.liquid_precipitation),
        ("BeamSolarRad", c.beam_solar_rad),
        ("DifSolarRad", c.dif_solar_rad),
        ("GndSolarRad", c.gnd_solar_rad),
        ("GndReflectance", c.gnd_reflectance),
        ("GndReflectanceForDayltg", c.gnd_reflectance_for_daylighting),
        ("SkyTemp", c.sky_temp),
        ("SkyTempKelvin", c.sky_temp_kelvin),
        ("TotalCloudCover", c.total_cloud_cover),
        ("OpaqueCloudCover", c.opaque_cloud_cover),
    ] {
        real(value, &expected[name], name)?;
    }
    let cosines = expected["SOLCOS"].as_array().ok_or("SOLCOS")?;
    assert_eq!(cosines.len(), c.solar_direction_cosines.len());
    for (actual, row) in c.solar_direction_cosines.iter().zip(cosines) {
        real(*actual, row, "SOLCOS")?;
    }
    Ok(())
}

// Held additive helper for design_day_native_state_tests.rs; no new test.
pub(super) fn site_and_solar(
    state: &WeatherDayState,
    event: &Value,
    time_zone_number: f64,
) -> TestResult {
    let e = &state.environment;
    let w = &state.weather;
    let native = &event["weather"];
    let site = &native["site"];
    for (name, value) in [
        ("CosLatitude", e.cos_latitude),
        ("Elevation", e.elevation),
        ("Latitude", e.latitude),
        ("Longitude", e.longitude),
        ("SinLatitude", e.sin_latitude),
        ("StdBaroPress", e.std_baro_press),
        ("TS1TimeOffset", e.one_step_solar_time_offset),
        ("TimeZoneMeridian", e.time_zone_meridian),
        ("TimeZoneNumber", time_zone_number),
    ] {
        real(value, &site[name], &format!("site.{name}"))?;
    }
    for (name, value) in [
        (
            "DisplayWeatherMissingDataWarnings",
            e.display_weather_missing_data_warnings,
        ),
        ("IgnoreBeamRadiation", e.ignore_beam_radiation),
        ("IgnoreDiffuseRadiation", e.ignore_diffuse_radiation),
        ("IgnoreSolarRadiation", e.ignore_solar_radiation),
    ] {
        assert_eq!(Some(value), site[name].as_bool(), "site.{name}");
    }
    let solar = &native["weather_solar"];
    for (name, value) in [
        ("HorizIRSky", w.horizontal_infrared_sky),
        ("HrAngle", w.hour_angle),
        ("SnowGndRefModifier", w.snow_ground_reflectance_modifier),
        (
            "SnowGndRefModifierForDayltg",
            w.snow_ground_reflectance_for_daylighting_modifier,
        ),
        ("SolarAltitudeAngle", w.solar_altitude_angle),
        ("SolarAzimuthAngle", w.solar_azimuth_angle),
    ] {
        real(value, &solar[name], &format!("weather_solar.{name}"))?;
    }
    for (name, value) in [("RptIsRain", w.rpt_is_rain), ("RptIsSnow", w.rpt_is_snow)] {
        assert_eq!(
            Some(i64::from(value)),
            solar[name].as_i64(),
            "weather_solar.{name}"
        );
    }
    let reflectance = &solar["GroundReflectances"];
    assert_eq!(reflectance["allocated"], true);
    assert_eq!(reflectance["lower"], 1);
    assert_eq!(reflectance["upper"], 12);
    assert_eq!(reflectance["size"], 12);
    let rows = reflectance["rows"]
        .as_array()
        .ok_or("GroundReflectances rows")?;
    assert_eq!(rows.len(), w.ground_reflectances.len());
    for (index, (actual, row)) in w.ground_reflectances.iter().zip(rows).enumerate() {
        assert_eq!(row["index"], index + 1);
        real(*actual, &row["value"], "weather_solar.GroundReflectances")?;
    }
    // PreviousSolRadPositive belongs to downstream InitSolarHeatGains. Its
    // Native false values here do not authorize pretending that HB ran.
    Ok(())
}
