//! Actual ReadEPlusWeatherForDay2738–3040 on parser references and retained owners.
use super::{Error, Failure, ordinal};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::{WeatherDayError, WeatherDayState};
use crate::weather::raw::{RawEpwHeaderState, RawEpwOutputs, source_ordinal_day};

fn increment(value: &mut i32) -> Result<(), Error> {
    *value = value.checked_add(1).ok_or(Error::Unavailable(
        "actual signed WeatherVarCounts increment",
    ))?;
    Ok(())
}
fn source_int(value: f64) -> Result<i32, Error> {
    let truncated = value.trunc();
    if !truncated.is_finite() || truncated < f64::from(i32::MIN) || truncated > f64::from(i32::MAX)
    {
        return Err(Error::Unavailable(
            "actual native defined Real64-to-int conversion domain",
        ));
    }
    Ok(truncated as i32)
}
/// Actual2738-2824 reference normalization and optional radiation counters.
/// Called once per physical record, before the leap retry decision.
pub(super) fn prepare_source_hour(
    state: &mut WeatherDayState,
    args: &mut RawEpwOutputs,
) -> Result<(), Error> {
    let [
        _dry,
        _dew,
        rh,
        pressure,
        et_horizontal,
        et_direct,
        infrared,
        global_horizontal,
        beam,
        diffuse,
        global_illuminance,
        direct_illuminance,
        diffuse_illuminance,
        zenith_luminance,
        wind_direction,
        wind_speed,
        total_sky,
        opaque_sky,
        visibility,
        ceiling,
    ] = &mut args.mandatory_reals;
    let [
        precipitable_water,
        aerosol,
        snow_depth,
        days_since_snow,
        albedo,
        liquid,
    ] = &mut args.optional_reals;
    let observation = &mut args.observation_indicator;
    if *et_horizontal < 0.0 {
        *et_horizontal = 9999.0;
    }
    if *et_direct < 0.0 {
        *et_direct = 9999.0;
    }
    if *infrared <= 0.0 {
        *infrared = 9999.0;
    }
    if *global_horizontal < 0.0 {
        *global_horizontal = 9999.0;
    }
    if state.environment.display_weather_missing_data_warnings {
        if *beam >= 9999.0 {
            increment(&mut state.missed_counts.beam_solar_rad)?;
        }
        if *diffuse >= 9999.0 {
            state.missed_counts.dif_solar_rad = state
                .missed_counts
                .beam_solar_rad
                .checked_add(1)
                .ok_or(Error::Unavailable(
                "actual signed diffuse counter addition2756",
            ))?;
        }
        if *beam < 0.0 {
            *beam = 9999.0;
            increment(&mut state.out_of_range_counts.beam_solar_rad)?;
        }
        if *diffuse < 0.0 {
            *diffuse = 9999.0;
            increment(&mut state.out_of_range_counts.dif_solar_rad)?;
        }
    }
    if *global_illuminance < 0.0 {
        *global_illuminance = 999999.0;
    }
    if *direct_illuminance < 0.0 {
        *direct_illuminance = 999999.0;
    }
    if *diffuse_illuminance < 0.0 {
        *diffuse_illuminance = 999999.0;
    }
    if *zenith_luminance < 0.0 {
        *zenith_luminance = 99999.0;
    }
    if *pressure < 0.0 {
        *pressure = 999999.0;
    }
    if *wind_speed < 0.0 {
        *wind_speed = 999.0;
    }
    if *wind_direction < -360.0 || *wind_direction > 360.0 {
        *wind_direction = 999.0;
    }
    if *total_sky < 0.0 {
        *total_sky = 99.0;
    }
    if *rh < 0.0 {
        *rh = 999.0;
    }
    if *opaque_sky < 0.0 {
        *opaque_sky = 99.0;
    }
    if *visibility < 0.0 {
        *visibility = 9999.0;
    }
    if *ceiling < 0.0 {
        *ceiling = 9999.0;
    }
    if *observation < 0 {
        *observation = 9;
    }
    if *precipitable_water < 0.0 {
        *precipitable_water = 999.0;
    }
    if *aerosol < 0.0 {
        *aerosol = 999.0;
    }
    if *snow_depth < 0.0 {
        *snow_depth = 999.0;
    }
    if *days_since_snow < 0.0 {
        *days_since_snow = 99.0;
    }
    if *albedo < 0.0 {
        *albedo = 999.0;
    }
    if *liquid < 0.0 {
        *liquid = 999.0;
    }
    Ok(())
}

pub(super) fn process_source_hour(
    state: &mut WeatherDayState,
    header: &RawEpwHeaderState,
    environ: usize,
    hour: usize,
    args: &mut RawEpwOutputs,
    psychrometrics: &mut SourcePsychrometricAccess,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    let [
        dry,
        dew,
        rh,
        pressure,
        et_horizontal,
        et_direct,
        infrared,
        global_horizontal,
        beam,
        diffuse,
        global_illuminance,
        direct_illuminance,
        diffuse_illuminance,
        zenith_luminance,
        wind_direction,
        wind_speed,
        total_sky,
        opaque_sky,
        visibility,
        ceiling,
    ] = &mut args.mandatory_reals;
    let [
        precipitable_water,
        aerosol,
        snow_depth,
        days_since_snow,
        albedo,
        liquid,
    ] = &mut args.optional_reals;
    let observation = &mut args.observation_indicator;
    let codes = &args.weather_codes;
    if hour == 1 {
        let row = state
            .weather
            .source_environments
            .as_ref()
            .and_then(|v| v.get(environ))
            .ok_or(Error::Unavailable(
                "actual ReadEPlusWeatherForDay Environment reference",
            ))?;
        if row.actual_weather
            && state.environment.current_year_is_leap_year
            && args.dates[1] == 3
            && args.dates[2] == 1
            && state.environment.month == 2
            && state.environment.day_of_month == 28
        {
            return Err(super::fatal(
                backend,
                "ReadEPlusWeatherForDay: Current year is a leap year, but Feb29 data is missing.",
            ));
        }
        state.tomorrow_variables.year = args.dates[0];
        state.tomorrow_variables.month = args.dates[1];
        state.tomorrow_variables.day_of_month = args.dates[2];
        state.tomorrow_variables.day_of_year =
            source_ordinal_day(args.dates[1], args.dates[2], header.leap_year_add)?;
        state.tomorrow_variables.day_of_year_schedule =
            source_ordinal_day(args.dates[1], args.dates[2], 1)?;
        // Same shared Native4128–4234 kernel writes Sin, Cos, Equation in place;
        // its remaining four outputs are the genuine call's local scratch.
        let _scratch = super::super::design_day::solar::source_daily_coefficients(
            &mut state.tomorrow_variables,
            state.environment.latitude,
        );
        if state.weather.cur_day_of_week <= 7 {
            state.weather.cur_day_of_week = state.weather.cur_day_of_week % 7 + 1;
        }
        state.tomorrow_variables.day_of_week = state.weather.cur_day_of_week;
        state.tomorrow_variables.daylight_saving_index = *state
            .weather
            .dst_index
            .get(ordinal(state.tomorrow_variables.day_of_year)?)
            .ok_or(Error::Unavailable("actual DSTIndex day2873"))?;
        state.tomorrow_variables.holiday_index = *state
            .weather
            .special_day_types
            .get(ordinal(state.tomorrow_variables.day_of_year)?)
            .ok_or(Error::Unavailable("actual SpecialDayTypes day2875"))?;
    }
    if *dry >= 99.9 {
        *dry = state.missing_values.base.out_dry_bulb_temp;
        increment(&mut state.missed_counts.out_dry_bulb_temp)?;
    }
    if *dry < -90.0 || *dry > 70.0 {
        increment(&mut state.out_of_range_counts.out_dry_bulb_temp)?;
    }

    if *dew >= 99.9 {
        *dew = state.missing_values.base.out_dew_point_temp;
        increment(&mut state.missed_counts.out_dew_point_temp)?;
    }
    if *dew < -90.0 || *dew > 70.0 {
        increment(&mut state.out_of_range_counts.out_dew_point_temp)?;
    }

    if *rh >= 999.0 {
        *rh = state.missing_values.base.out_rel_hum;
        increment(&mut state.missed_counts.out_rel_hum)?;
    }
    if *rh < 0.0 || *rh > 110.0 {
        increment(&mut state.out_of_range_counts.out_rel_hum)?;
    }

    if *pressure >= 999999.0 {
        *pressure = state.missing_values.base.out_baro_press;
        increment(&mut state.missed_counts.out_baro_press)?;
    }
    if *pressure <= 31000.0 || *pressure > 120000.0 {
        increment(&mut state.out_of_range_counts.out_baro_press)?;
        *pressure = state.missing_values.base.out_baro_press;
    }

    if *wind_direction >= 999.0 {
        *wind_direction = state.missing_values.base.wind_dir;
        increment(&mut state.missed_counts.wind_dir)?;
    }
    if *wind_direction < 0.0 || *wind_direction > 360.0 {
        increment(&mut state.out_of_range_counts.wind_dir)?;
    }

    if *wind_speed >= 999.0 {
        *wind_speed = state.missing_values.base.wind_speed;
        increment(&mut state.missed_counts.wind_speed)?;
    }
    if *wind_speed < 0.0 || *wind_speed > 40.0 {
        increment(&mut state.out_of_range_counts.wind_speed)?;
    }

    if *total_sky >= 99.0 {
        *total_sky = state.missing_values.base.total_sky_cover;
        increment(&mut state.missed_counts.total_sky_cover)?;
    }

    if *opaque_sky >= 99.0 {
        *opaque_sky = state.missing_values.base.opaque_sky_cover;
        increment(&mut state.missed_counts.opaque_sky_cover)?;
    }

    if *snow_depth >= 999.0 {
        *snow_depth = state.missing_values.snow_depth;
        increment(&mut state.missed_counts.snow_depth)?;
    }

    if *albedo >= 999.0 {
        *albedo = state.missing_values.base.albedo;
        increment(&mut state.missed_counts.albedo)?;
    }

    if *liquid >= 999.0 {
        *liquid = state.missing_values.base.liquid_precip;
        increment(&mut state.missed_counts.liquid_precip)?;
    }
    let tomorrow = state
        .tomorrow_values
        .hour_mut(hour)
        .map_err(WeatherDayError::from)?
        .get_mut(0)
        .ok_or(Error::Unavailable("actual Tomorrow hourly slot2962"))?;
    tomorrow.out_dry_bulb_temp = *dry;
    tomorrow.out_dew_point_temp = *dew;
    tomorrow.out_baro_press = *pressure;
    tomorrow.out_rel_hum = *rh;
    *rh *= 0.01;
    tomorrow.wind_speed = *wind_speed;
    tomorrow.wind_dir = *wind_direction;
    tomorrow.liquid_precip = *liquid;
    tomorrow.total_sky_cover = *total_sky;
    tomorrow.opaque_sky_cover = *opaque_sky;
    super::sky::calc_source_sky(
        &state.weather.source_environments,
        &state.weather.envrn,
        &mut tomorrow.horiz_ir_sky,
        &mut tomorrow.sky_temp,
        *opaque_sky,
        *dry,
        *dew,
        *rh,
        *infrared,
        psychrometrics,
    )?;
    if *et_horizontal >= 9999.0 {
        *et_horizontal = 0.0;
    }
    if *et_direct >= 9999.0 {
        *et_direct = 0.0;
    }
    if *global_horizontal >= 9999.0 {
        *global_horizontal = 0.0;
    }
    if *beam >= 9999.0 {
        *beam = 0.0;
    }
    if *diffuse >= 9999.0 {
        *diffuse = 0.0;
    }
    if *global_illuminance >= 999900.0 {
        *global_illuminance = 0.0;
    }
    if *direct_illuminance >= 999900.0 {
        *direct_illuminance = 0.0;
    }
    if *diffuse_illuminance >= 999900.0 {
        *diffuse_illuminance = 0.0;
    }
    if *zenith_luminance >= 99990.0 {
        *zenith_luminance = 0.0;
    }
    if state.environment.ignore_solar_radiation {
        *global_horizontal = 0.0;
        *beam = 0.0;
        *diffuse = 0.0;
    }
    if state.environment.ignore_beam_radiation {
        *beam = 0.0;
    }
    if state.environment.ignore_diffuse_radiation {
        *diffuse = 0.0;
    }

    tomorrow.beam_solar_rad = *beam;
    tomorrow.dif_solar_rad = *diffuse;

    tomorrow.is_rain = false;
    if *observation == 0 {
        if codes[0] < 9 || codes[1] < 9 || codes[2] < 9 {
            tomorrow.is_rain = true;
        }
    } else {
        tomorrow.is_rain = false;
    }
    tomorrow.is_snow = *snow_depth > 0.0;

    if tomorrow.is_rain && tomorrow.liquid_precip == 0.0 {
        tomorrow.liquid_precip = 2.0;
    }

    state.missing_values.base.out_dry_bulb_temp = *dry;
    state.missing_values.base.out_dew_point_temp = *dew;
    state.missing_values.base.out_rel_hum = f64::from(source_int((*rh * 100.0).round())?);
    state.missing_values.base.out_baro_press = *pressure;
    state.missing_values.base.wind_dir = *wind_direction;
    state.missing_values.base.wind_speed = *wind_speed;
    state.missing_values.base.total_sky_cover = *total_sky;
    state.missing_values.base.opaque_sky_cover = *opaque_sky;
    state.missing_values.visibility = *visibility;
    state.missing_values.ceiling = *ceiling;
    state.missing_values.base.water_precip = *precipitable_water;
    state.missing_values.aer_opt_depth = *aerosol;
    state.missing_values.snow_depth = *snow_depth;
    state.missing_values.days_last_snow = source_int(*days_since_snow)?;
    state.missing_values.base.albedo = *albedo;
    Ok(())
}
