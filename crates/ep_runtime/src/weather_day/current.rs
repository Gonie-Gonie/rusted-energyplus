//! Selected current weather and solar state in EnergyPlus source order.
//! WeatherManager.cc:2036-2273, at the frozen EnergyPlus 26.1 source pin.

use super::{WeatherDayError, WeatherDayState, WeatherSession};
use crate::psychrometrics::{
    energyplus_psy_h_fn_tdb_w, energyplus_psy_rho_air_fn_pb_tdb_w, energyplus_psy_tsat_fn_pb,
    energyplus_psy_twb_fn_tdb_w_pb, energyplus_psy_w_fn_tdb_rh_pb, energyplus_psy_w_fn_tdb_twb_pb,
    set_psychrometric_warmup,
};

/// Mutable selected DataEnvironment fields, independent of the Today carrier.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CurrentWeatherState {
    /// Outdoor dry bulb, in Celsius.
    pub out_dry_bulb_temp: f64,
    /// Outdoor dew point, in Celsius.
    pub out_dew_point_temp: f64,
    /// Outdoor pressure, in Pa.
    pub out_baro_press: f64,
    /// Outdoor relative humidity, in percent.
    pub out_rel_hum: f64,
    /// Outdoor relative humidity fraction.
    pub out_rel_hum_value: f64,
    /// Outdoor humidity ratio, in kg/kg.
    pub out_hum_rat: f64,
    /// Outdoor wet bulb, in Celsius.
    pub out_wet_bulb_temp: f64,
    /// Wind speed, in m/s.
    pub wind_speed: f64,
    /// Wind direction, in degrees.
    pub wind_dir: f64,
    /// Liquid precipitation, in m per zone timestep.
    pub liquid_precipitation: f64,
    /// Selected rain control applied to the Today indicator.
    pub is_rain: bool,
    /// Actual SunIsUp after DetermineSunUpDown.
    pub sun_is_up: bool,
    /// Actual SunIsUpPrevTS, assigned before the current decision.
    pub sun_is_up_previous_timestep: bool,
    /// Actual SOLCOS x/y/z; z is retained even when the sun is down.
    pub solar_direction_cosines: [f64; 3],
    /// Actual BeamSolarRad after the nighttime mask, in W/m2.
    pub beam_solar_rad: f64,
    /// Actual DifSolarRad after the nighttime mask, in W/m2.
    pub dif_solar_rad: f64,
    /// Actual GndSolarRad after reflection and nighttime masking, in W/m2.
    pub gnd_solar_rad: f64,
    /// Actual monthly GndReflectance, with the reached snow modifier.
    pub gnd_reflectance: f64,
    /// Independent ground reflectance for daylighting.
    pub gnd_reflectance_for_daylighting: f64,
    /// Actual sky temperature from Today, in Celsius.
    pub sky_temp: f64,
    /// Actual SkyTempKelvin, calculated from the current Celsius owner.
    pub sky_temp_kelvin: f64,
    /// Actual UseSnowValues selection of Today's IsSnow.
    pub is_snow: bool,
    /// Actual total cloud cover from Today.
    pub total_cloud_cover: f64,
    /// Actual opaque cloud cover from Today.
    pub opaque_cloud_cover: f64,
    /// Outdoor enthalpy, in J/kg.
    pub out_enthalpy: f64,
    /// Outdoor air density, in kg/m3.
    pub out_air_density: f64,
}

impl WeatherDayState {
    /// Allocates the actual source interpolation owners without opening a file.
    /// This changes only the two arrays written by SetupInterpolationValues.
    pub fn setup_interpolation_values(&mut self) -> Result<(), WeatherDayError> {
        let steps = usize::try_from(self.global.time_steps_in_hour)
            .ok()
            .filter(|&steps| steps > 0)
            .ok_or_else(|| WeatherDayError::admission("interpolation needs positive timesteps"))?;
        let mut linear = vec![0.0; steps];
        let mut solar = vec![0.0; steps];
        for timestep in 1..=steps {
            linear[timestep - 1] = if steps == 1 {
                1.0
            } else {
                (timestep as f64 / steps as f64).min(1.0)
            };
        }
        if steps % 2 == 0 {
            let half = steps / 2;
            solar[half - 1] = 1.0;
            let weight = 1.0 / steps as f64;
            for (offset, timestep) in (half + 1..=steps).enumerate() {
                solar[timestep - 1] = 1.0 - (offset + 1) as f64 * weight;
            }
            for (offset, timestep) in (1..half).rev().enumerate() {
                solar[timestep - 1] = 1.0 - (offset + 1) as f64 * weight;
            }
        } else if steps == 1 {
            solar[0] = 0.5;
        } else if steps == 3 {
            solar[0] = 5.0 / 6.0;
            solar[1] = 5.0 / 6.0;
            solar[2] = 0.5;
        } else {
            let weight = 1.0 / steps as f64;
            let half = steps / 2;
            let half_weight = 1.0 - weight / 2.0;
            solar[half - 1] = half_weight;
            solar[half] = half_weight;
            for (offset, timestep) in (half + 2..=steps).enumerate() {
                solar[timestep - 1] = half_weight - (offset + 1) as f64 * weight;
            }
            for (offset, timestep) in (1..half).rev().enumerate() {
                solar[timestep - 1] = half_weight - (offset + 1) as f64 * weight;
            }
        }
        self.weather.interpolation = Some(linear);
        self.weather.solar_interpolation = Some(solar);
        Ok(())
    }
}

impl WeatherSession {
    /// Delegates the genuine state-level setup used by production and pure tests.
    pub fn setup_interpolation_values(&mut self) -> Result<(), WeatherDayError> {
        self.state.setup_interpolation_values()
    }

    /// Selects Today and updates the admitted weather/solar environment.
    /// EMS, optional ground/water models and daylight illuminance remain separate.
    pub fn set_current_weather(&mut self) -> Result<(), WeatherDayError> {
        let before = super::production_trace::snapshot(self);
        let result = self.set_current_weather_inner();
        super::production_trace::record_operation(
            "SetCurrentWeather",
            before,
            self,
            result.as_ref().err(),
        );
        result
    }

    fn set_current_weather_inner(&mut self) -> Result<(), WeatherDayError> {
        let state = &mut self.state;
        let hour = state.global.hour_of_day;
        let timestep = state.global.time_step;
        let steps = state.global.time_steps_in_hour;
        if !(1..=24).contains(&hour) || !(1..=steps).contains(&timestep) {
            return Err(WeatherDayError::admission(
                "current weather outside allocated clock",
            ));
        }
        state.weather.next_hour = if hour == 24 { 1 } else { hour + 1 };
        if hour == 1 {
            state.environment.day_of_year_schedule = super::configuration::ordinal(
                state.environment.month,
                state.environment.day_of_month,
                1,
            )
            .map_err(WeatherDayError::admission)?;
        }
        state.global.weight_now = *state
            .weather
            .interpolation
            .as_ref()
            .and_then(|values| values.get(timestep as usize - 1))
            .ok_or_else(|| WeatherDayError::admission("current interpolation owner unavailable"))?;
        state.global.weight_previous_hour = 1.0 - state.global.weight_now;
        state.global.current_time =
            (hour - 1) as f64 + timestep as f64 * state.weather.time_step_fraction;
        state.global.sim_time_steps =
            (state.global.day_of_sim - 1) * 24 * steps + (hour - 1) * steps + timestep;
        let month = usize::try_from(state.environment.month - 1)
            .ok()
            .filter(|&month| month < 12)
            .ok_or_else(|| {
                WeatherDayError::admission("current ground reflectance month unavailable")
            })?;
        state.environment.current_weather.gnd_reflectance =
            state.weather.ground_reflectances[month];
        state
            .environment
            .current_weather
            .gnd_reflectance_for_daylighting = state.environment.current_weather.gnd_reflectance;
        determine_sun_up_down(state);
        if state.environment.current_weather.sun_is_up && state.weather.solar_altitude_angle < 0.0 {
            return Err(WeatherDayError::source_fatal(format!(
                "SetCurrentWeather: At {:02}/{:02} {:02} Sun is Up but Solar Altitude Angle is < 0.0",
                state.environment.month,
                state.environment.day_of_month,
                hour - 1
            )));
        }
        let today = state
            .today_values
            .hour(hour as usize)?
            .get(timestep as usize - 1)
            .copied()
            .ok_or_else(|| WeatherDayError::admission("current Today slot unavailable"))?;
        // Share the accepted cache owner; synchronize the caller control only.
        set_psychrometric_warmup(state.global.warmup_flag);
        let current = &mut state.environment.current_weather;
        current.out_dry_bulb_temp = today.out_dry_bulb_temp;
        current.out_baro_press = today.out_baro_press;
        current.out_dew_point_temp = today.out_dew_point_temp;
        current.out_rel_hum = today.out_rel_hum;
        current.out_rel_hum_value = current.out_rel_hum / 100.0;
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
        current.wind_speed = today.wind_speed;
        current.wind_dir = today.wind_dir;
        state.weather.horizontal_infrared_sky = today.horiz_ir_sky;
        current.sky_temp = today.sky_temp;
        current.sky_temp_kelvin = current.sky_temp + 273.15;
        current.dif_solar_rad = today.dif_solar_rad;
        current.beam_solar_rad = today.beam_solar_rad;
        current.liquid_precipitation = today.liquid_precip / 1000.0;
        current.is_rain = if state.weather.use_rain_values {
            today.is_rain
        } else {
            false
        };
        current.total_cloud_cover = today.total_sky_cover;
        current.opaque_cloud_cover = today.opaque_sky_cover;
        current.is_snow = state.weather.use_snow_values && today.is_snow;
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
        state.weather.rpt_is_rain = if current.is_rain { 1 } else { 0 };
        state.weather.rpt_is_snow = if current.is_snow { 1 } else { 0 };
        Ok(())
    }
}

/// Literal DetermineSunUpDown; the daily and location operands have real owners.
fn determine_sun_up_down(state: &mut WeatherDayState) {
    const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
    let env = &mut state.environment;
    let daily = &state.today_variables;
    let weather = &mut state.weather;
    weather.hour_angle = if state.global.time_steps_in_hour != 1 {
        15.0 * (12.0 - (state.global.current_time + daily.equation_of_time))
            + (env.time_zone_meridian - env.longitude)
    } else {
        15.0 * (12.0
            - ((state.global.current_time + env.one_step_solar_time_offset)
                + daily.equation_of_time))
            + (env.time_zone_meridian - env.longitude)
    };
    let h = weather.hour_angle * DEG_TO_RAD;
    let cos_zenith = env.sin_latitude * daily.sin_solar_declin_angle
        + env.cos_latitude * daily.cos_solar_declin_angle * h.cos();
    let solar_zenith = cos_zenith.acos();
    let sin_altitude = env.cos_latitude * daily.cos_solar_declin_angle * h.cos()
        + env.sin_latitude * daily.sin_solar_declin_angle;
    let solar_altitude = sin_altitude.asin();
    let cos_azimuth = -(env.sin_latitude * cos_zenith - daily.sin_solar_declin_angle)
        / (env.cos_latitude * solar_zenith.sin());
    // Source max(CosAzimuth,-1), then min(1,CosAzimuth), without new clamps.
    let cos_azimuth = if cos_azimuth < -1.0 {
        -1.0
    } else {
        cos_azimuth
    };
    let cos_azimuth = if 1.0 < cos_azimuth { 1.0 } else { cos_azimuth };
    let solar_azimuth = cos_azimuth.acos();
    weather.solar_altitude_angle = solar_altitude / DEG_TO_RAD;
    weather.solar_azimuth_angle = solar_azimuth / DEG_TO_RAD;
    if weather.hour_angle < 0.0 {
        weather.solar_azimuth_angle = 360.0 - weather.solar_azimuth_angle;
    }
    let current = &mut env.current_weather;
    current.solar_direction_cosines[2] = cos_zenith;
    current.sun_is_up_previous_timestep = current.sun_is_up;
    if cos_zenith < 0.00001 {
        current.sun_is_up = false;
        current.solar_direction_cosines[1] = 0.0;
        current.solar_direction_cosines[0] = 0.0;
    } else {
        current.sun_is_up = true;
        current.solar_direction_cosines[1] = daily.sin_solar_declin_angle * env.cos_latitude
            - daily.cos_solar_declin_angle * env.sin_latitude * h.cos();
        current.solar_direction_cosines[0] = daily.cos_solar_declin_angle * h.sin();
    }
}
