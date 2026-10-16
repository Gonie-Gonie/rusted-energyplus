//! Genuine SetCurrentWeather2036–2273 using same live daily/session/module owners.
mod precipitation;
use super::WeatherDayError;
use super::{SourceWeatherInitializationError as Failure, WeatherSession};
use crate::heat_balance::{
    error_backend::SourceErrorBackend,
    ordinary_hvac_manager::water_manager::{SourceRainfallMode, SourceWaterManager},
    selected_heat_balance_input::HeatBalanceInputError as Error,
    selected_schedule_manager::SelectedScheduleManager,
};
use crate::psychrometrics::{
    energyplus_psy_h_fn_tdb_w, energyplus_psy_rho_air_fn_pb_tdb_w, energyplus_psy_tsat_fn_pb,
    energyplus_psy_twb_fn_tdb_w_pb, energyplus_psy_w_fn_tdb_rh_pb, energyplus_psy_w_fn_tdb_twb_pb,
    set_psychrometric_warmup,
};
pub(crate) use precipitation::SourceEcoRoofData;
pub(crate) struct SourceCurrentWeatherOwners<'a, 'backend> {
    pub(crate) schedules: &'a mut SelectedScheduleManager,
    pub(crate) water: &'a mut SourceWaterManager,
    pub(crate) eco_roof: &'a mut SourceEcoRoofData,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
fn month_index(month: i32) -> Result<usize, Error> {
    month
        .checked_sub(1)
        .and_then(|i| usize::try_from(i).ok())
        .filter(|i| *i < 12)
        .ok_or(Error::Unavailable("actual rainfall month array domain"))
}
impl WeatherSession {
    pub(crate) fn set_source_current_weather(
        &mut self,
        o: &mut SourceCurrentWeatherOwners<'_, '_>,
    ) -> Result<(), Failure> {
        let state = &mut self.state;
        let hour = state.global.hour_of_day;
        let timestep = state.global.time_step;
        let steps = state.global.time_steps_in_hour;
        state.weather.next_hour = hour
            .checked_add(1)
            .ok_or(Error::Unavailable("actual NextHour signed addition"))?;
        if hour == 24 {
            state.weather.next_hour = 1;
        }
        if hour == 1 {
            state.environment.day_of_year_schedule = crate::weather::raw::source_ordinal_day(
                state.environment.month,
                state.environment.day_of_month,
                1,
            )?;
        }
        o.schedules.update_source_schedule_values()?;
        let report_hour = hour.checked_sub(1).ok_or(Error::Unavailable(
            "actual CurMnDyHr hour subtraction domain",
        ))? as u16;
        state.environment.current_month_day_hour = format!(
            "{:02}/{:02} {:02}",
            state.environment.month, state.environment.day_of_month, report_hour
        );

        state.environment.current_month_day = format!(
            "{:02}/{:02}",
            state.environment.month, state.environment.day_of_month
        );
        state.environment.current_month_day_year = format!(
            "{:02}/{:02}/{:04}",
            state.environment.month, state.environment.day_of_month, state.global.calendar_year
        );
        let timestep_index = timestep
            .checked_sub(1)
            .and_then(|v| usize::try_from(v).ok())
            .ok_or(Error::Unavailable(
                "actual Interpolation one-based subscript domain",
            ))?;
        state.global.weight_now = *state
            .weather
            .interpolation
            .as_ref()
            .and_then(|values| values.get(timestep_index))
            .ok_or_else(|| WeatherDayError::admission("current interpolation owner unavailable"))?;
        state.global.weight_previous_hour = 1.0 - state.global.weight_now;
        state.global.current_time =
            (hour - 1) as f64 + timestep as f64 * state.weather.time_step_fraction;
        let day_term = state
            .global
            .day_of_sim
            .checked_sub(1)
            .and_then(|v| v.checked_mul(24))
            .and_then(|v| v.checked_mul(steps))
            .ok_or(Error::Unavailable(
                "actual SimTimeSteps day signed arithmetic domain",
            ))?;
        let hour_term = hour
            .checked_sub(1)
            .and_then(|v| v.checked_mul(steps))
            .ok_or(Error::Unavailable(
                "actual SimTimeSteps hour signed arithmetic domain",
            ))?;
        state.global.sim_time_steps = day_term
            .checked_add(hour_term)
            .and_then(|v| v.checked_add(timestep))
            .ok_or(Error::Unavailable("actual SimTimeSteps sum signed domain"))?;
        state
            .ground_temperature_models
            .update_current(&mut state.environment)?;
        let month = month_index(state.environment.month)?;
        state.environment.current_weather.gnd_reflectance =
            state.weather.ground_reflectances[month];
        state
            .environment
            .current_weather
            .gnd_reflectance_for_daylighting = state.environment.current_weather.gnd_reflectance;
        super::auxiliary_input::calculate_water_mains_temperature(
            &state.weather,
            &mut state.environment,
        )?;
        super::current::determine_sun_up_down(state);
        if state.environment.current_weather.sun_is_up && state.weather.solar_altitude_angle < 0.0 {
            if o.backend.state.warmup_flag != state.global.warmup_flag {
                return Err(Error::Unavailable(
                    "actual SetCurrentWeather fatal backend must borrow live DataGlobal WarmupFlag",
                )
                .into());
            }
            return match o.backend.show_fatal_error(&format!(
                "SetCurrentWeather: At {} Sun is Up but Solar Altitude Angle is < 0.0",
                state.environment.current_month_day_hour
            )) {
                Ok(never) => match never {},
                Err(cause) => Err(Error::Fatal(cause).into()),
            };
        }

        let today = state
            .today_values
            .hour(hour as usize)
            .map_err(WeatherDayError::from)?
            .get(timestep_index)
            .ok_or_else(|| WeatherDayError::admission("current Today slot unavailable"))?;
        // Share the accepted cache owner; synchronize the caller control only.
        set_psychrometric_warmup(state.global.warmup_flag);
        let current = &mut state.environment.current_weather;
        current.out_dry_bulb_temp = today.out_dry_bulb_temp;
        if state.environment.ems_out_dry_bulb_override_on {
            current.out_dry_bulb_temp = state.environment.ems_out_dry_bulb_override_value;
        }
        current.out_baro_press = today.out_baro_press;
        current.out_dew_point_temp = today.out_dew_point_temp;
        if state.environment.ems_out_dew_point_temp_override_on {
            current.out_dew_point_temp = state.environment.ems_out_dew_point_temp_override_value;
        }
        current.out_rel_hum = today.out_rel_hum;
        current.out_rel_hum_value = current.out_rel_hum / 100.0;
        if state.environment.ems_out_rel_hum_override_on {
            current.out_rel_hum_value = state.environment.ems_out_rel_hum_override_value / 100.0;
            current.out_rel_hum = state.environment.ems_out_rel_hum_override_value;
        }
        current.out_hum_rat = energyplus_psy_w_fn_tdb_rh_pb(
            current.out_dry_bulb_temp,
            current.out_rel_hum_value,
            current.out_baro_press,
        );
        current.out_wet_bulb_temp = energyplus_psy_twb_fn_tdb_w_pb(
            current.out_dry_bulb_temp,
            current.out_hum_rat,
            current.out_baro_press,
        );
        if current.out_dry_bulb_temp < current.out_wet_bulb_temp {
            current.out_wet_bulb_temp = current.out_dry_bulb_temp;
            let humidity = energyplus_psy_w_fn_tdb_twb_pb(
                current.out_dry_bulb_temp,
                current.out_wet_bulb_temp,
                current.out_baro_press,
            );
            // PsyTdpFnWPb delegates to cached PsyTsatFnPb at this source pin.
            let humidity = if humidity < 1.0e-5 { 1.0e-5 } else { humidity };
            let dew_pressure = current.out_baro_press * humidity / (0.62198 + humidity);
            current.out_dew_point_temp = energyplus_psy_tsat_fn_pb(dew_pressure);
        }
        if current.out_dew_point_temp > current.out_wet_bulb_temp {
            current.out_dew_point_temp = current.out_wet_bulb_temp;
        }
        state
            .weather
            .source_design_day_input
            .update_source_site_schedule_values(
                &state.global,
                &state.environment,
                state.weather.envrn,
                state.weather.source_environments.as_deref(),
            )?;
        let current = &mut state.environment.current_weather;
        current.wind_speed = today.wind_speed;
        if state.environment.ems_wind_speed_override_on {
            current.wind_speed = state.environment.ems_wind_speed_override_value;
        }
        current.wind_dir = today.wind_dir;
        if state.environment.ems_wind_dir_override_on {
            current.wind_dir = state.environment.ems_wind_dir_override_value;
        }
        state.weather.horizontal_infrared_sky = today.horiz_ir_sky;
        current.sky_temp = today.sky_temp;
        current.sky_temp_kelvin = current.sky_temp + 273.15;
        current.dif_solar_rad = today.dif_solar_rad;
        if state.environment.ems_dif_solar_rad_override_on {
            current.dif_solar_rad = state.environment.ems_dif_solar_rad_override_value;
        }
        current.beam_solar_rad = today.beam_solar_rad;
        if state.environment.ems_beam_solar_rad_override_on {
            current.beam_solar_rad = state.environment.ems_beam_solar_rad_override_value;
        }
        current.liquid_precipitation = today.liquid_precip / 1000.0;
        if state.environment.run_period_environment && !state.global.warmup_flag {
            let month = month_index(state.environment.month)?;
            o.water.data.rainfall.monthly_total_precipitation_weather[month] +=
                current.liquid_precipitation * 1000.0;
            if current.liquid_precipitation > 0.0 && state.global.time_step == 1 {
                let cell = &mut o.water.data.rainfall.number_rainy_hours_weather[month];
                *cell = cell.checked_add(1).ok_or(Error::Unavailable(
                    "actual rainy-hours counter signed domain",
                ))?;
            }
        }
        o.water.update_source_precipitation(
            &state.global,
            &state.environment,
            o.schedules,
            o.eco_roof,
        )?;
        let current = &mut state.environment.current_weather;
        current.total_cloud_cover = today.total_sky_cover;
        current.opaque_cloud_cover = today.opaque_sky_cover;
        if state.weather.use_rain_values {
            current.is_rain = today.is_rain;
            if o.water.data.rainfall.mode == SourceRainfallMode::RainSchedDesign
                && state.environment.run_period_environment
            {
                current.is_rain = o.water.data.rainfall.current_amount
                    >= state.weather.is_rain_threshold / 1000.0;
            }
        } else {
            current.is_rain = false;
        }
        if state.weather.use_snow_values {
            current.is_snow = today.is_snow;
        } else {
            current.is_snow = false;
        }
        if current.is_snow {
            // Literal max(min(value,1),0) argument order, including NaN behavior.
            let reflectance =
                current.gnd_reflectance * state.weather.snow_ground_reflectance_modifier;
            let reflectance = if reflectance < 1.0 { reflectance } else { 1.0 };
            current.gnd_reflectance = if reflectance < 0.0 { 0.0 } else { reflectance };
            let daylight = current.gnd_reflectance_for_daylighting
                * state
                    .weather
                    .snow_ground_reflectance_for_daylighting_modifier;
            let daylight = if daylight < 1.0 { daylight } else { 1.0 };
            current.gnd_reflectance_for_daylighting = if daylight < 0.0 { 0.0 } else { daylight };
        }
        let ground = (current.beam_solar_rad * current.solar_direction_cosines[2]
            + current.dif_solar_rad)
            * current.gnd_reflectance;
        current.gnd_solar_rad = if ground < 0.0 { 0.0 } else { ground };
        if !current.sun_is_up {
            current.dif_solar_rad = 0.0;
            current.beam_solar_rad = 0.0;
            current.gnd_solar_rad = 0.0;
        }
        current.out_enthalpy =
            energyplus_psy_h_fn_tdb_w(current.out_dry_bulb_temp, current.out_hum_rat);
        current.out_air_density = energyplus_psy_rho_air_fn_pb_tdb_w(
            current.out_baro_press,
            current.out_dry_bulb_temp,
            current.out_hum_rat,
        );
        if current.out_dry_bulb_temp < current.out_wet_bulb_temp {
            current.out_wet_bulb_temp = current.out_dry_bulb_temp;
        }
        if current.out_dew_point_temp > current.out_wet_bulb_temp {
            current.out_dew_point_temp = current.out_wet_bulb_temp;
        }
        state.environment.calculate_source_exterior_daylight()?;
        state.weather.rpt_is_rain = if state.environment.current_weather.is_rain {
            1
        } else {
            0
        };
        state.weather.rpt_is_snow = if state.environment.current_weather.is_snow {
            1
        } else {
            0
        };
        Ok(())
    }
}
