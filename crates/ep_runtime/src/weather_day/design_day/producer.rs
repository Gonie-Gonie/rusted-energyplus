//! Selected real SetUpDesignDay and InitializeWeather branches.
use super::{DesignDayHumidity, DesignDayInput, solar};
use crate::psychrometrics::{
    energyplus_psy_psat_fn_temp, energyplus_psy_tsat_fn_pb, energyplus_psy_twb_fn_tdb_w_pb,
    energyplus_psy_w_fn_tdb_twb_pb, set_psychrometric_warmup,
};
use crate::weather::day::{WeatherDayError, WeatherDayState, WeatherSession};

fn humidity_from_dew_point(temperature: f64, pressure: f64) -> Result<f64, WeatherDayError> {
    let saturation = energyplus_psy_psat_fn_temp(temperature);
    let humidity = saturation * 0.62198 / (pressure - saturation);
    if humidity < 0.0 {
        return Err(WeatherDayError::admission(
            "DesignDay PsyWFnTdpPb diagnostic branch requires actual backend",
        ));
    }
    Ok(humidity)
}

fn dew_point(humidity: f64, pressure: f64) -> f64 {
    // PsyTdpFnWPb must use the real cached outer TsatPb route, not the older
    // convenience composition that bypasses that owner.
    let humidity = if humidity < 1.0e-5 { 1.0e-5 } else { humidity };
    energyplus_psy_tsat_fn_pb(pressure * humidity / (0.62198 + humidity))
}

fn relative_humidity(
    temperature: f64,
    humidity: f64,
    pressure: f64,
) -> Result<f64, WeatherDayError> {
    let saturation = energyplus_psy_psat_fn_temp(temperature);
    let humidity = if humidity < 1.0e-5 { 1.0e-5 } else { humidity };
    let degree = humidity / (0.62198 * saturation / (pressure - saturation));
    let relative = degree / (1.0 - (1.0 - degree) * (saturation / pressure));
    if !(-0.05..=1.01).contains(&relative) {
        return Err(WeatherDayError::admission(
            "DesignDay PsyRhFnTdbWPb diagnostic branch requires actual backend",
        ));
    }
    Ok(if relative < 0.0 {
        0.01
    } else if relative > 1.0 {
        1.0
    } else {
        relative
    })
}

impl DesignDayInput {
    fn setup(
        &mut self,
        state: &mut WeatherDayState,
        current_year: i32,
        time_zone: f64,
        do_weather_init_reporting: bool,
    ) -> Result<(), WeatherDayError> {
        let steps = usize::try_from(state.global.time_steps_in_hour)
            .ok()
            .filter(|&steps| steps > 0)
            .ok_or_else(|| {
                WeatherDayError::admission("DesignDay requires positive allocated timestep owner")
            })?;
        if do_weather_init_reporting
            || !(1000..=9999).contains(&current_year)
            || self.dry_bulb_modifiers.len() != 24 * steps
            || state.tomorrow_values.time_steps() != steps
        {
            return Err(WeatherDayError::admission(
                "DesignDay requires actual Setup reporting=false, wall year and complete storage",
            ));
        }
        let standard = state.environment.std_baro_press;
        if !standard.is_finite()
            || standard <= 0.0
            || (self.pressure_entered
                && ((self.resolved_pressure - standard) / standard).abs() > 0.1)
        {
            return Err(WeatherDayError::admission(
                "DesignDay pressure diagnostic requires real backend",
            ));
        }
        let saved_warmup = state.global.warmup_flag;
        state.global.warmup_flag = true;
        set_psychrometric_warmup(true);
        if state.global.begin_sim_flag {
            state.weather.print_design_day_header = true;
        }
        let input = &self.parameters;
        let day = &mut self.daily_variables;
        day.year = current_year;
        day.month = input.month;
        day.day_of_month = input.day_of_month;
        day.day_of_year = super::super::configuration::ordinal(day.month, day.day_of_month, 0)
            .map_err(WeatherDayError::admission)?;
        state.environment.current_month_day =
            format!("{:02}/{:02}", input.month, input.day_of_month);
        state.environment.run_period_environment = false;
        state.environment.environment_start_end = format!(
            "{} - {}",
            state.environment.current_month_day, state.environment.current_month_day
        );
        if !self.pressure_entered {
            self.resolved_pressure = standard;
        }
        let pressure = self.resolved_pressure;
        day.day_of_week = 2;
        if input.day_type <= 7 {
            day.day_of_week = input.day_type;
        }
        day.holiday_index = 0;
        if input.day_type > 7 {
            day.holiday_index = input.day_type;
        }
        day.daylight_saving_index = i32::from(input.daylight_saving);
        let coefficients = solar::daily_coefficients(day.day_of_year, state.environment.latitude);
        day.equation_of_time = coefficients.equation_of_time;
        day.sin_solar_declin_angle = coefficients.sin_declination;
        day.cos_solar_declin_angle = coefficients.cos_declination;
        state.global.current_time = 25.0;
        let humidity = match input.humidity_type {
            DesignDayHumidity::WetBulb => energyplus_psy_w_fn_tdb_twb_pb(
                input.maximum_dry_bulb,
                input.humidity_indicator,
                pressure,
            ),
            DesignDayHumidity::DewPoint => {
                humidity_from_dew_point(input.humidity_indicator, pressure)?
            }
        };
        for slot in state.tomorrow_values.slots_mut() {
            slot.is_rain = input.rain;
            slot.liquid_precip = if input.rain { 3.0 } else { 0.0 };
        }
        for slot in state.tomorrow_values.slots_mut() {
            slot.is_snow = input.snow;
        }
        for slot in state.tomorrow_values.slots_mut() {
            slot.out_baro_press = pressure;
            slot.wind_speed = input.wind_speed;
            slot.wind_dir = input.wind_direction % 360.0;
            slot.albedo = 0.0;
        }
        let opaque_sky = if input.rain { 10.0 } else { 0.0 };
        let latitude = state.environment.latitude;
        let longitude = state.environment.longitude;
        let fraction = state.weather.time_step_fraction;
        for hour in 1..=24 {
            for (timestep, slot) in state.tomorrow_values.hour_mut(hour)?.iter_mut().enumerate() {
                let modifier = self.dry_bulb_modifiers[(hour - 1) * steps + timestep];
                slot.out_dry_bulb_temp =
                    input.maximum_dry_bulb - modifier * input.daily_dry_bulb_range;
                let wet_bulb =
                    energyplus_psy_twb_fn_tdb_w_pb(slot.out_dry_bulb_temp, humidity, pressure);
                let saturation_humidity =
                    humidity_from_dew_point(slot.out_dry_bulb_temp, pressure)?;
                let out_humidity = if humidity > saturation_humidity {
                    // Original assigns WetBulb=DB, then uses the already
                    // computed saturation humidity without calling W-from-WB.
                    saturation_humidity
                } else {
                    energyplus_psy_w_fn_tdb_twb_pb(slot.out_dry_bulb_temp, wet_bulb, pressure)
                };
                slot.out_dew_point_temp = dew_point(out_humidity, pressure);
                slot.out_rel_hum =
                    relative_humidity(slot.out_dry_bulb_temp, out_humidity, pressure)? * 100.0;
                super::super::sky::initialize_design_day_sky(slot, opaque_sky);
                let time = if steps != 1 {
                    (hour - 1) as f64 + (timestep + 1) as f64 * fraction
                } else {
                    hour as f64 - 0.5
                };
                let zenith =
                    solar::cosine_zenith(time, coefficients, latitude, longitude, time_zone);
                let (mut beam, mut diffuse) =
                    solar::clear_sky_radiation(zenith, input.sky_clearness, coefficients);
                if state.environment.ignore_solar_radiation
                    || state.environment.ignore_beam_radiation
                {
                    beam = 0.0;
                }
                if state.environment.ignore_solar_radiation
                    || state.environment.ignore_diffuse_radiation
                {
                    diffuse = 0.0;
                }
                slot.beam_solar_rad = beam;
                slot.dif_solar_rad = diffuse;
            }
        }
        state.global.warmup_flag = saved_warmup;
        set_psychrometric_warmup(saved_warmup);
        Ok(())
    }
}

impl WeatherSession {
    /// Actual selected InitializeWeather, retaining one live owner and cursor.
    ///
    /// # Errors
    /// Reporting=true/pressure/psychrometric diagnostic branches need real sinks;
    /// none is silently suppressed. Actual Setup supplies Source-authority false.
    pub fn initialize_design_day_weather(
        &mut self,
        index: usize,
        current_year: i32,
        do_weather_init_reporting: bool,
    ) -> Result<(), WeatherDayError> {
        let ordinal = index
            .checked_add(1)
            .and_then(|value| i32::try_from(value).ok())
            .ok_or_else(|| WeatherDayError::admission("DesignDay ordinal is not representable"))?;
        if self.state.global.kind_of_sim != 1
            || !self.available
            || self.state.weather.envrn != ordinal
        {
            return Err(WeatherDayError::admission(
                "DesignDay InitializeWeather requires actual selected environment",
            ));
        }
        if self.state.global.begin_sim_flag && self.state.weather.first_call {
            self.state.weather.first_call = false;
            self.state.environment.end_month_flag = false;
        }
        if self.state.global.begin_envrn_flag {
            let input = self
                .configuration
                .design_days
                .get_mut(index)
                .ok_or_else(|| WeatherDayError::admission("DesignDay input owner missing"))?;
            input.setup(
                &mut self.state,
                current_year,
                self.configuration.site.time_zone_hours,
                do_weather_init_reporting,
            )?;
            self.state.environment.environment_name = input.name().into();
            self.initialize_missing_values();
            self.state.tomorrow_variables = self.configuration.design_days[index].daily_variables;
        }
        if self.state.global.begin_day_flag {
            super::super::update_weather_data(&mut self.state);
            // DD has no ReadWeatherForDay branch or RunPeriod raw provenance.
            self.state.environment.end_year_flag = false;
            let day = self.state.tomorrow_variables;
            let ends = [
                31,
                28 + self.state.weather.leap_year_add,
                31,
                30,
                31,
                30,
                31,
                31,
                30,
                31,
                30,
                31,
            ];
            if self.state.environment.day_of_month
                == ends[self.state.environment.month as usize - 1]
            {
                self.state.environment.end_month_flag = true;
                self.state.environment.end_year_flag = self.state.environment.month == 12;
            }
            self.state.environment.month_tomorrow = day.month;
            self.state.environment.day_of_month_tomorrow = day.day_of_month;
            self.state.environment.day_of_week_tomorrow = day.day_of_week;
            self.state.environment.holiday_index_tomorrow = day.holiday_index;
            self.state.environment.year_tomorrow = day.year;
        }
        // Original InitializeWeather's selected final source assignments.
        self.state.global.end_design_day_envrns_flag = self.state.global.end_envrn_flag
            && self.state.weather.envrn == self.state.environment.tot_des_days;
        self.state.weather.water_mains_parameter_report = false;
        Ok(())
    }
}
