//! Genuine SetUpDesignDay3494–4040 on the same actual input/modifier/daily owners.
mod profiles;
mod properties;
mod reports;
use super::*;
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::SourceWeatherInitializationError as Failure;
use crate::weather::day::{WeatherDayValues, WeatherEnvironmentState};
use chrono::{Datelike, Local};
use std::io::Write;

pub(crate) struct SourceDesignDaySetupOwners<'a, 'backend> {
    pub(crate) environments: &'a [SourceEnvironmentData],
    pub(crate) design_day_weather: &'a mut [DailyWeatherVariables],
    pub(crate) tomorrow_values: &'a mut WeatherDayValues,
    pub(crate) clock: &'a mut WeatherGlobalState,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) time_step_fraction: &'a f64,
    pub(crate) current_environment_number: &'a i32,
    pub(crate) print_design_day_header: &'a mut bool,
    pub(crate) report_flags: &'a SourceSolarReportFlags,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) eio: &'a mut dyn Write,
}
impl SourceDesignDayInputState {
    pub(crate) fn set_up_source_design_day(
        &mut self,
        number: i32,
        o: &mut SourceDesignDaySetupOwners<'_, '_>,
    ) -> Result<(), Failure> {
        // Real HourlyWeatherData constructor scratch; not retained output/fixture seeds.
        let mut hourly_beam_solar_radiation = [0.0; 24];
        let mut hourly_diffuse_solar_radiation = [0.0; 24];
        let environment_row = o
            .environments
            .get(ordinal(number)?)
            .ok_or(Error::Unavailable(
                "actual SetUpDesignDay Environment row3537",
            ))?;
        let saved_warmup = o.clock.warmup_flag;
        o.clock.warmup_flag = true;
        crate::psychrometrics::set_psychrometric_warmup(o.clock.warmup_flag);
        // Actual live local system year, not CalendarYear or a configuration year.
        let current_year = Local::now().year();
        if o.clock.begin_sim_flag {
            *o.print_design_day_header = true;
        }
        let design_day =
            o.design_day_weather
                .get_mut(ordinal(number)?)
                .ok_or(Error::Unavailable(
                    "actual SetUpDesignDay DesignDay row3550",
                ))?;
        let input = self
            .design_day_input
            .as_mut()
            .and_then(|v| v.get_mut(ordinal(number).ok()?))
            .ok_or(Error::Unavailable(
                "actual SetUpDesignDay DesDayInput row3551",
            ))?;
        design_day.year = current_year;
        design_day.month = input.month;
        design_day.day_of_month = input.day_of_month;
        design_day.day_of_year =
            crate::weather::raw::source_ordinal_day(design_day.month, design_day.day_of_month, 0)?;
        o.environment.current_month_day = format!("{:02}/{:02}", input.month, input.day_of_month);
        o.environment.run_period_environment = false;
        o.environment.environment_start_end = format!(
            "{} - {}",
            o.environment.current_month_day, o.environment.current_month_day
        );
        if input.pressure_entered {
            if ((input.barometric_pressure - o.environment.std_baro_press)
                / o.environment.std_baro_press)
                .abs()
                > 0.1
            {
                if o.backend.state.warmup_flag != o.clock.warmup_flag {
                    return Err(Error::Unavailable("actual SetUp pressure-warning backend must borrow current DataGlobal WarmupFlag").into());
                }
                o.backend.show_warning_error(&format!("SetUpDesignDay: Entered DesignDay Barometric Pressure={} differs by more than 10% from Standard Barometric Pressure={}.", rounded(input.barometric_pressure, 0)?, rounded(o.environment.std_baro_press, 0)?))?;
                o.backend.show_continue_error(&format!("...occurs in DesignDay={}, Standard Pressure (based on elevation) will be used.", o.environment.environment_name))?;
                input.barometric_pressure = o.environment.std_baro_press;
            }
        } else {
            input.barometric_pressure = o.environment.std_baro_press;
        }
        if input.humidity_indicator_type == 1 && input.dew_point_needs_set {
            let humidity = properties::from_relative_humidity(
                o.psychrometrics,
                input.maximum_dry_bulb,
                1.0,
                input.barometric_pressure,
            )?;
            input.humidity_indicator_value =
                properties::dew_point(o.psychrometrics, humidity, input.barometric_pressure)?;
        }
        design_day.day_of_week = 2;
        if input.day_type <= 7 {
            design_day.day_of_week = input.day_type;
        }
        design_day.holiday_index = 0;
        if input.day_type > 7 {
            design_day.holiday_index = input.day_type;
        }
        design_day.daylight_saving_index = input.daylight_saving_indicator;
        // Mutating pressure/dewpoint prefix has ended. Reborrow the same whole
        // input slice for Native3650's distinct global-Envrn WBProfDef lookup.
        let inputs = self
            .design_day_input
            .as_ref()
            .ok_or(Error::Unavailable("actual SetUpDesignDay input allocation"))?;
        let input = inputs
            .get(ordinal(number)?)
            .ok_or(Error::Unavailable("actual SetUpDesignDay input ordinal"))?;
        let coefficients = super::super::design_day::solar::source_daily_coefficients(
            design_day,
            o.environment.latitude,
        );
        reports::write(
            input,
            inputs,
            *o.current_environment_number,
            design_day,
            coefficients,
            o.print_design_day_header,
            o.report_flags,
            o.eio,
        )?;
        o.clock.current_time = 25.0;
        use profiles::HumidityState;
        let humidity = match input.humidity_indicator_type {
            0 => HumidityState::Constant(properties::from_wet_bulb(
                o.psychrometrics,
                input.maximum_dry_bulb,
                input.humidity_indicator_value,
                input.barometric_pressure,
            )?),
            1 => HumidityState::Constant(properties::from_dew_point(
                o.psychrometrics,
                input.humidity_indicator_value,
                input.barometric_pressure,
            )?),
            3 => HumidityState::Constant(input.humidity_indicator_value),
            2 => HumidityState::Constant(properties::from_enthalpy(
                o.psychrometrics,
                input.maximum_dry_bulb,
                input.humidity_indicator_value,
            )?),
            4 => {
                let modifiers = self
                    .design_day_modifiers
                    .as_ref()
                    .and_then(|v| v.get(ordinal(number).ok()?))
                    .and_then(Option::as_ref)
                    .ok_or(Error::Unavailable(
                        "actual DD scheduled humidity modifier array3700",
                    ))?;
                for hour in 1..=24 {
                    for timestep in 1..=o.clock.time_steps_in_hour {
                        profiles::slot_mut(o.tomorrow_values, hour, timestep)?.out_rel_hum =
                            modifiers.cell(timestep, hour)?.outdoor_relative_humidity;
                    }
                }
                HumidityState::Variable
            }
            5..=7 => HumidityState::Variable,
            _ => {
                if o.backend.state.warmup_flag != o.clock.warmup_flag {
                    return Err(Error::Unavailable("actual SetUp invalid-humidity diagnostic backend must borrow current DataGlobal WarmupFlag").into());
                }
                o.backend
                    .show_severe_error("SetUpDesignDay: Invalid Humidity Indicator type")?;
                o.backend
                    .show_continue_error(&format!("Occurred in Design Day={}", input.title))?;
                HumidityState::Uninitialized
            }
        };
        let opaque_sky;
        if input.rain_indicator != 0 {
            opaque_sky = 10.0;
            for hour in 1..=24 {
                for timestep in 1..=o.clock.time_steps_in_hour {
                    let slot = profiles::slot_mut(o.tomorrow_values, hour, timestep)?;
                    slot.is_rain = true;
                    slot.liquid_precip = 3.0;
                }
            }
        } else {
            opaque_sky = 0.0;
            for hour in 1..=24 {
                for timestep in 1..=o.clock.time_steps_in_hour {
                    let slot = profiles::slot_mut(o.tomorrow_values, hour, timestep)?;
                    slot.is_rain = false;
                    slot.liquid_precip = 0.0;
                }
            }
        }
        let _ground_reflectivity;
        if input.snow_indicator == 0 {
            _ground_reflectivity = 0.2;
            for hour in 1..=24 {
                for timestep in 1..=o.clock.time_steps_in_hour {
                    profiles::slot_mut(o.tomorrow_values, hour, timestep)?.is_snow = false;
                }
            }
        } else {
            _ground_reflectivity = 0.7;
            for hour in 1..=24 {
                for timestep in 1..=o.clock.time_steps_in_hour {
                    profiles::slot_mut(o.tomorrow_values, hour, timestep)?.is_snow = true;
                }
            }
        }
        for hour in 1..=24 {
            for timestep in 1..=o.clock.time_steps_in_hour {
                let slot = profiles::slot_mut(o.tomorrow_values, hour, timestep)?;
                slot.out_baro_press = input.barometric_pressure;
                slot.wind_speed = input.wind_speed;
                slot.wind_dir = input.wind_direction;
                slot.albedo = 0.0;
            }
        }
        let modifiers = self
            .design_day_modifiers
            .as_ref()
            .and_then(|v| v.get(ordinal(number).ok()?))
            .and_then(Option::as_ref)
            .ok_or(Error::Unavailable(
                "actual SetUpDesignDay modifier array3795",
            ))?;
        profiles::write(
            input,
            modifiers,
            design_day,
            coefficients,
            environment_row,
            opaque_sky,
            humidity,
            o.tomorrow_values,
            o.clock.time_steps_in_hour,
            o.time_step_fraction,
            o.environment,
            o.psychrometrics,
        )?;
        // Native3960–3980 performs real local hourly scratch backfill even though not consumed.
        for hour in 1..=24 {
            let previous_hour = (hour + 22) % 24 + 1;
            let now = profiles::slot(o.tomorrow_values, hour, o.clock.time_steps_in_hour)?;
            let previous =
                profiles::slot(o.tomorrow_values, previous_hour, o.clock.time_steps_in_hour)?;
            let mut beam = (previous.beam_solar_rad + now.beam_solar_rad) / 2.0;
            let mut diffuse = (previous.dif_solar_rad + now.dif_solar_rad) / 2.0;
            if o.clock.time_steps_in_hour > 1 {
                for timestep in 1..=o.clock.time_steps_in_hour - 1 {
                    let slot = profiles::slot(o.tomorrow_values, hour, timestep)?;
                    beam += slot.beam_solar_rad;
                    diffuse += slot.dif_solar_rad;
                }
            }
            hourly_beam_solar_radiation[ordinal(hour)?] =
                beam / f64::from(o.clock.time_steps_in_hour);
            hourly_diffuse_solar_radiation[ordinal(hour)?] =
                diffuse / f64::from(o.clock.time_steps_in_hour);
        }
        if environment_row.weather_properties_type_one != 0 {
            return Err(Error::Unavailable(
                "actual SetUpDesignDay WPSkyTemperature reference3983 before schedule child",
            )
            .into());
        }
        o.clock.warmup_flag = saved_warmup;
        crate::psychrometrics::set_psychrometric_warmup(o.clock.warmup_flag);
        Ok(())
    }
}
