//! Genuine ManageWeather131–155 and InitializeWeather1725–1992 on one session.
mod calendar;
mod hourly;
mod interpolation;
mod reader;
mod reports;
mod sky;
use super::source_initialization::SourceWeatherInitializationOwners;
use super::{SourceWeatherInitializationError as Failure, WeatherDayError, WeatherSession};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::raw::source_ordinal_day;
use std::io::Write;

/// Same underlying Source EIO stream, with its actual goodbit read per call.
/// The physical file host supplies this capability; no successful sink default.
pub(crate) trait SourceWeatherEio: Write {
    fn good(&self) -> bool;
}
/// Real SetCurrentWeather2036–2273 child. Its separate Source implementation must
/// use this same session and the per-call owners; no retained mutable aliases.
pub(crate) trait SourceManageWeatherChildren {
    fn set_current_source_weather(
        &mut self,
        session: &mut WeatherSession,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        psychrometrics: &mut SourcePsychrometricAccess,
    ) -> Result<(), Failure>;
}
fn ordinal(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("actual positive weather array ordinal"))
}
fn fatal(backend: &mut SourceErrorBackend<'_>, text: &str) -> Failure {
    match backend.show_fatal_error(text) {
        Ok(never) => match never {},
        Err(error) => Error::Fatal(error).into(),
    }
}
fn leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
fn add(a: i32, b: i32) -> Result<i32, Error> {
    a.checked_add(b).ok_or(Error::Unavailable(
        "actual InitializeWeather signed addition",
    ))
}

impl WeatherSession {
    pub(crate) fn manage_source_weather(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        psychrometrics: &mut SourcePsychrometricAccess,
        children: &mut impl SourceManageWeatherChildren,
    ) -> Result<(), Failure> {
        self.initialize_source_weather(owners, psychrometrics)?;
        if !owners.globals.doing_sizing && !owners.globals.kickoff_simulation {
            // Native local starts false before ManageEMS itself writes false.
            let mut any_ems_ran = Some(false);
            crate::heat_balance::ordinary_ems_plugin::manage_source_heat_balance_ems(
                owners.globals,
                crate::heat_balance::ordinary_ems_plugin::SourceHeatBalanceEmsCallingPoint::BeginZoneTimestepBeforeSetCurrentWeather,
                &mut any_ems_ran,
            )?;
        }
        children.set_current_source_weather(self, owners, psychrometrics)?;
        self.report_source_weather_and_time(owners)
    }

    pub(crate) fn initialize_source_weather(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        psychrometrics: &mut SourcePsychrometricAccess,
    ) -> Result<(), Failure> {
        if self.state.global.begin_sim_flag && self.state.weather.first_call {
            self.state.weather.first_call = false;
            self.state.environment.end_month_flag = false;
        }
        // Source captures Environment(Envrn) here, before BeginEnvrn. Retain its
        // ordinal identity across real child calls, then reborrow that same row.
        let current = ordinal(self.state.weather.envrn)?;
        self.state
            .weather
            .source_environments
            .as_ref()
            .and_then(|v| v.get(current))
            .ok_or(Error::Unavailable("actual InitializeWeather envCurr1744"))?;
        if self.state.global.begin_envrn_flag {
            let row = self
                .state
                .weather
                .source_environments
                .as_ref()
                .and_then(|v| v.get(current))
                .ok_or(Error::Unavailable("actual BeginEnvrn Environment1748"))?;
            if row.kind_of_environment != 3 && row.design_day_number > 0 {
                let number = row.design_day_number;
                let weather = &mut self.state.weather;
                let mut setup = super::source_design_day::SourceDesignDaySetupOwners {
                    environments: weather
                        .source_environments
                        .as_deref()
                        .ok_or(Error::Unavailable("actual SetUp Environment array"))?,
                    design_day_weather: weather
                        .source_design_day_weather
                        .as_deref_mut()
                        .ok_or(Error::Unavailable("actual SetUp DesignDay array"))?,
                    tomorrow_values: &mut self.state.tomorrow_values,
                    clock: &mut self.state.global,
                    environment: &mut self.state.environment,
                    time_step_fraction: &weather.time_step_fraction,
                    current_environment_number: &weather.envrn,
                    print_design_day_header: &mut weather.print_design_day_header,
                    report_flags: owners.report_flags,
                    psychrometrics,
                    backend: owners.backend,
                    eio: owners.eio,
                };
                weather
                    .source_design_day_input
                    .set_up_source_design_day(number, &mut setup)?;
                self.state.environment.environment_name = self
                    .state
                    .weather
                    .source_environments
                    .as_ref()
                    .and_then(|v| v.get(current))
                    .ok_or(Error::Unavailable("actual EnvironmentName1751"))?
                    .title
                    .clone();
            }
            let missing = &mut self.state.missing_values;
            missing.base.out_baro_press = self.state.environment.std_baro_press;
            missing.base.out_dry_bulb_temp = 6.0;
            missing.base.out_dew_point_temp = 3.0;
            missing.base.out_rel_hum = 50.0;
            missing.base.wind_speed = 2.5;
            missing.base.wind_dir = 180.0;
            missing.base.total_sky_cover = 5.0;
            missing.base.opaque_sky_cover = 5.0;
            missing.visibility = 777.7;
            missing.ceiling = 77777.0;
            missing.aer_opt_depth = 0.0;
            missing.snow_depth = 0.0;
            missing.days_last_snow = 88;
            missing.base.albedo = 0.0;
            missing.base.liquid_precip = 0.0;
            self.state.missed_counts = Default::default();
            self.state.out_of_range_counts = Default::default();
            self.state.weather.is_rain_threshold =
                0.8 / f64::from(self.state.global.time_steps_in_hour);
            if !self.state.weather.rp_read_all_weather_data {
                self.print_environment_stamp = true;
            }
            let header = self.cursor.source_header_mut();
            for number in 1..=header.number_special_days {
                header
                    .special_days
                    .values
                    .get_mut(ordinal(number)?)
                    .ok_or(Error::Unavailable("actual SpecialDays Used1797"))?
                    .used = false;
            }
            if self.state.global.kind_of_sim != 1 && self.state.global.kind_of_sim != 4 {
                self.read_source_weather_for_day(
                    1,
                    self.state.weather.envrn,
                    false,
                    owners.globals,
                    psychrometrics,
                    owners.backend,
                )?;
            } else {
                let row = self
                    .state
                    .weather
                    .source_environments
                    .as_ref()
                    .and_then(|v| v.get(current))
                    .ok_or(Error::Unavailable("actual DD daily-copy environment1811"))?;
                self.state.tomorrow_variables = *self
                    .state
                    .weather
                    .source_design_day_weather
                    .as_ref()
                    .and_then(|v| v.get(ordinal(row.design_day_number).ok()?))
                    .ok_or(Error::Unavailable("actual DesignDay daily-copy1811"))?;
            }
        }
        if self.state.global.begin_day_flag {
            super::update_weather_data(&mut self.state);
            let kind = self
                .state
                .weather
                .source_environments
                .as_ref()
                .and_then(|v| v.get(current))
                .ok_or(Error::Unavailable("actual BeginDay envCurr1819"))?
                .kind_of_environment;
            if !self.state.global.warmup_flag
                && kind != 1
                && kind != 4
                && self.state.global.day_of_sim < self.state.global.num_of_day_in_envrn
            {
                if self.state.global.day_of_sim
                    == self.state.weather.cur_sim_day_for_end_of_run_period
                {
                    let raw_days = self
                        .state
                        .weather
                        .source_environments
                        .as_ref()
                        .and_then(|v| v.get(current))
                        .ok_or(Error::Unavailable("actual RawSimDays1823"))?
                        .raw_simulation_days;
                    self.state.weather.cur_sim_day_for_end_of_run_period = add(
                        self.state.weather.cur_sim_day_for_end_of_run_period,
                        raw_days,
                    )?;
                    if self.state.weather.start_dates_cycle_should_be_reset {
                        self.reset_source_calendar(current, false, owners.backend)?;
                    }
                    self.state.weather.year_of_sim = add(self.state.weather.year_of_sim, 1)?;
                    self.read_source_weather_for_day(
                        1,
                        self.state.weather.envrn,
                        false,
                        owners.globals,
                        psychrometrics,
                        owners.backend,
                    )?;
                } else {
                    self.read_source_weather_for_day(
                        add(self.state.global.day_of_sim, 1)?,
                        self.state.weather.envrn,
                        false,
                        owners.globals,
                        psychrometrics,
                        owners.backend,
                    )?;
                }
            }
            self.state.environment.end_year_flag = false;
            let month = ordinal(self.state.environment.month)?;
            if self.state.environment.day_of_month
                == *self
                    .state
                    .weather
                    .end_day_of_month_with_leap_day
                    .get(month)
                    .ok_or(Error::Unavailable("actual EndDayOfMonthWithLeapDay1840"))?
            {
                self.state.environment.end_month_flag = true;
                self.state.environment.end_year_flag = self.state.environment.month == 12;
            }
            self.state.environment.month_tomorrow = self.state.tomorrow_variables.month;
            self.state.environment.day_of_month_tomorrow =
                self.state.tomorrow_variables.day_of_month;
            self.state.environment.day_of_week_tomorrow = self.state.tomorrow_variables.day_of_week;
            self.state.environment.holiday_index_tomorrow =
                self.state.tomorrow_variables.holiday_index;
            self.state.environment.year_tomorrow = self.state.tomorrow_variables.year;
            self.reset_source_january_calendar(current, owners.backend)?;
            if owners.globals.air_loop_hvac_doas_used_in_sim {
                let row = self
                    .state
                    .weather
                    .source_environments
                    .as_mut()
                    .and_then(|v| v.get_mut(current))
                    .ok_or(Error::Unavailable("actual DOAS Environment1915"))?;
                if row.kind_of_environment == 2 || row.kind_of_environment == 1 {
                    for hour in 1..=24 {
                        for timestep in 1..=self.state.global.time_steps_in_hour {
                            let slot = self
                                .state
                                .today_values
                                .hour(hour)
                                .map_err(WeatherDayError::from)?
                                .get(ordinal(timestep)?)
                                .ok_or(Error::Unavailable("actual DOAS Today slot1918"))?;
                            let dry = slot.out_dry_bulb_temp;
                            let dew = slot.out_dew_point_temp;
                            if dry > row.maximum_cooling_outdoor_air_temperature_sizing {
                                row.maximum_cooling_outdoor_air_temperature_sizing = dry;
                                row.maximum_cooling_outdoor_air_dew_point_sizing = dew;
                            }
                            if dry < row.minimum_heating_outdoor_air_temperature_sizing {
                                row.minimum_heating_outdoor_air_temperature_sizing = dry;
                                row.minimum_heating_outdoor_air_dew_point_sizing = dew;
                            }
                        }
                    }
                }
            }
        }
        let row = self
            .state
            .weather
            .source_environments
            .as_ref()
            .and_then(|v| v.get(current))
            .ok_or(Error::Unavailable("actual InitializeWeather envCurr1936"))?;
        if !self.state.global.begin_day_flag
            && !self.state.global.warmup_flag
            && (self.state.environment.month != row.start_month
                || self.state.environment.day_of_month != row.start_day)
            && !self.state.weather.dates_should_be_reset
            && row.kind_of_environment == 3
        {
            self.state.weather.dates_should_be_reset = true;
        }
        if self.state.global.end_envrn_flag
            && row.kind_of_environment != 1
            && row.kind_of_environment != 4
        {
            self.cursor.rewind_and_skip_header()?;
            reports::report_source_missing_range(&self.state, owners.backend)?;
        }
        self.state.global.end_design_day_envrns_flag = false;
        if self.state.global.end_envrn_flag {
            if self.state.weather.envrn < self.state.weather.num_of_envrn {
                let rows = self
                    .state
                    .weather
                    .source_environments
                    .as_ref()
                    .ok_or(Error::Unavailable("actual Environment kind array1959"))?;
                if rows
                    .get(current)
                    .ok_or(Error::Unavailable("actual current Environment kind1959"))?
                    .kind_of_environment
                    != rows
                        .get(ordinal(add(self.state.weather.envrn, 1)?)?)
                        .ok_or(Error::Unavailable("actual next Environment kind1959"))?
                        .kind_of_environment
                {
                    self.state.global.end_design_day_envrns_flag = true;
                }
            } else {
                self.state.global.end_design_day_envrns_flag = true;
            }
        }
        if self.state.weather.water_mains_parameter_report {
            if self.state.weather.water_mains_temperature_method == 2 {
                return Err(Error::Unavailable(
                    "actual OADryBulbAverage processed predicate1978 and annual/monthly producer",
                )
                .into());
            }
            reports::report_source_water_mains(&self.state.weather, owners.eio)?;
            self.state.weather.water_mains_parameter_report = false;
        }
        Ok(())
    }

    fn reset_source_calendar(
        &mut self,
        current: usize,
        mid_sim: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Failure> {
        let weather = &mut self.state.weather;
        let row = weather
            .source_environments
            .as_mut()
            .and_then(|v| v.get_mut(current))
            .ok_or(Error::Unavailable("actual reset calendar Environment"))?;
        let header = self.cursor.source_header_mut();
        calendar::reset_weekdays(row, header, mid_sim)?;
        if weather.daylight_saving_is_active {
            calendar::set_dst_dates(&row.month_week_day, &mut weather.dst_index, header)?;
        }
        calendar::set_special_days(
            &row.month_week_day,
            &mut weather.special_day_types,
            &weather.use_special_days,
            header,
            backend,
        )
    }
    fn reset_source_january_calendar(
        &mut self,
        current: usize,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Failure> {
        let row = self
            .state
            .weather
            .source_environments
            .as_mut()
            .and_then(|v| v.get_mut(current))
            .ok_or(Error::Unavailable("actual January Environment1854"))?;
        if row.kind_of_environment == 3 {
            if self.state.environment.month == 1
                && self.state.environment.day_of_month == 1
                && row.actual_weather
            {
                if self.state.weather.dates_should_be_reset && row.treat_years_as_consecutive {
                    row.current_year = add(row.current_year, 1)?;
                    row.is_leap_year = leap_year(row.current_year);
                    self.state.environment.current_year_is_leap_year = row.is_leap_year;
                    let header = self.cursor.source_header_mut();
                    header.leap_year_add = i32::from(
                        self.state.environment.current_year_is_leap_year
                            && header.allows_leap_years,
                    );
                    let first =
                        source_ordinal_day(row.start_month, row.start_day, header.leap_year_add)?;
                    let last =
                        source_ordinal_day(row.end_month, row.end_day, header.leap_year_add)?;
                    if !row.actual_weather {
                        self.state.weather.cur_sim_day_for_end_of_run_period = add(
                            add(
                                add(self.state.global.day_of_sim, row.raw_simulation_days)?,
                                header.leap_year_add,
                            )?,
                            -1,
                        )?;
                    }
                    let mut day = first;
                    let mut weekday = self.state.environment.day_of_week;
                    loop {
                        *self
                            .state
                            .weather
                            .weekday_types
                            .get_mut(ordinal(day)?)
                            .ok_or(Error::Unavailable("actual WeekDayTypes1870"))? = weekday;
                        weekday = weekday % 7 + 1;
                        day = add(day, 1)?;
                        if day > 366 {
                            day = 1;
                        }
                        if day == last {
                            break;
                        }
                    }
                    self.reset_source_calendar(current, false, backend)?;
                }
            } else if self.state.environment.month == 1
                && self.state.environment.day_of_month == 1
                && self.state.weather.dates_should_be_reset
                && self.state.weather.jan1_dates_should_be_reset
            {
                if row.treat_years_as_consecutive {
                    row.current_year = add(row.current_year, 1)?;
                    row.is_leap_year = leap_year(row.current_year);
                    self.state.environment.current_year_is_leap_year = row.is_leap_year;
                    if self.state.environment.current_year_is_leap_year
                        && !self.cursor.header().allows_leap_years
                    {
                        self.state.environment.current_year_is_leap_year = false;
                    }
                    if self.state.global.day_of_sim
                        < self.state.weather.cur_sim_day_for_end_of_run_period
                        && self.state.environment.current_year_is_leap_year
                    {
                        self.state.weather.cur_sim_day_for_end_of_run_period =
                            add(self.state.weather.cur_sim_day_for_end_of_run_period, 1)?;
                    }
                }
                let header = self.cursor.source_header_mut();
                header.leap_year_add = i32::from(
                    self.state.environment.current_year_is_leap_year && header.allows_leap_years,
                );
                if self.state.global.day_of_sim
                    < self.state.weather.cur_sim_day_for_end_of_run_period
                {
                    let mid_sim = row.roll_day_type_on_repeat
                        || self.state.environment.current_year_is_leap_year;
                    self.reset_source_calendar(current, mid_sim, backend)?;
                }
            }
        }
        Ok(())
    }
}
