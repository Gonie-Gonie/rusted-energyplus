//! Genuine SetupEnvironmentTypes8388–8519 on actual getter-produced RunPeriod rows.
use super::source_run_period::{checked, leap_year};
use super::{SourceWeatherInitializationError as Failure, WeatherSession};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::weather::raw::source_ordinal_day;

impl WeatherSession {
    pub(super) fn setup_source_environment_types(&mut self) -> Result<(), Failure> {
        self.state.weather.envrn = checked(self.state.environment.tot_des_days.checked_add(1))?;
        if self.state.weather.tot_run_des_pers > 0 {
            return Err(Error::Unavailable(
                "SetupEnvironmentTypes8405 genuine RunPeriodDesignInput first row",
            )
            .into());
        }
        let count = usize::try_from(self.state.weather.tot_run_pers)
            .map_err(|_| Error::Unavailable("actual SetupEnvironmentTypes RunPeriod count8431"))?;
        for ordinal in 0..count {
            let run_period = self
                .state
                .weather
                .source_run_period_input
                .as_ref()
                .and_then(|v| v.get(ordinal))
                .ok_or(Error::Unavailable(
                    "actual SetupEnvironmentTypes RunPeriodInput row8432",
                ))?;
            let index = usize::try_from(checked(self.state.weather.envrn.checked_sub(1))?)
                .map_err(|_| {
                    Error::Unavailable("actual SetupEnvironmentTypes Environment ordinal8433")
                })?;
            let environment = self
                .state
                .weather
                .source_environments
                .as_mut()
                .and_then(|v| v.get_mut(index))
                .ok_or(Error::Unavailable(
                    "actual SetupEnvironmentTypes Environment row8433",
                ))?;
            environment.start_month = run_period.start_month;
            environment.start_day = run_period.start_day;
            environment.start_year = run_period.start_year;
            environment.end_month = run_period.end_month;
            environment.end_day = run_period.end_day;
            environment.end_year = run_period.end_year;
            environment.number_simulation_years = run_period.number_simulation_years;
            environment.current_year = run_period.start_year;
            environment.is_leap_year = run_period.is_leap_year;
            environment.treat_years_as_consecutive = true;
            if run_period.actual_weather {
                // Native chained assignment writes right-hand target first.
                environment.start_date = run_period.start_julian_date;
                environment.start_julian_day = environment.start_date;
                environment.end_date = run_period.end_julian_date;
                environment.end_julian_day = environment.end_date;
                environment.total_days = checked(
                    environment
                        .end_date
                        .checked_sub(environment.start_date)
                        .and_then(|v| v.checked_add(1)),
                )?;
                environment.raw_simulation_days = checked(
                    environment
                        .end_date
                        .checked_sub(environment.start_date)
                        .and_then(|v| v.checked_add(1)),
                )?;
                environment.match_year = true;
                environment.actual_weather = true;
            } else {
                environment.roll_day_type_on_repeat = run_period.roll_day_type_on_repeat;
                if environment.start_year == environment.end_year {
                    environment.is_leap_year =
                        leap_year(environment.start_year) && self.cursor.header().allows_leap_years;
                    let local_leap_year_add = i32::from(environment.is_leap_year);
                    environment.start_julian_day = source_ordinal_day(
                        run_period.start_month,
                        run_period.start_day,
                        local_leap_year_add,
                    )?;
                    environment.end_julian_day = source_ordinal_day(
                        run_period.end_month,
                        run_period.end_day,
                        local_leap_year_add,
                    )?;
                    environment.raw_simulation_days = checked(
                        environment
                            .end_julian_day
                            .checked_sub(environment.start_julian_day)
                            .and_then(|v| v.checked_add(1)),
                    )?;
                    environment.total_days = environment.raw_simulation_days;
                } else {
                    environment.roll_day_type_on_repeat = run_period.roll_day_type_on_repeat;
                    environment.start_julian_day = source_ordinal_day(
                        run_period.start_month,
                        run_period.start_day,
                        i32::from(run_period.is_leap_year),
                    )?;
                    environment.end_julian_day = source_ordinal_day(
                        run_period.end_month,
                        run_period.end_day,
                        i32::from(
                            leap_year(run_period.end_year)
                                && self.cursor.header().allows_leap_years,
                        ),
                    )?;
                    let remaining_years =
                        checked(environment.number_simulation_years.checked_sub(2))?;
                    let remaining_years = if remaining_years < 0 {
                        0
                    } else {
                        remaining_years
                    }; // Native explicit std::max(int,0).
                    environment.total_days = checked(
                        366_i32
                            .checked_sub(environment.start_julian_day)
                            .and_then(|v| v.checked_add(environment.end_julian_day))
                            .and_then(|v| {
                                365_i32
                                    .checked_mul(remaining_years)
                                    .and_then(|w| v.checked_add(w))
                            }),
                    )?;
                    if self.cursor.header().allows_leap_years {
                        if environment.start_julian_day < 59 && leap_year(environment.start_year) {
                            environment.total_days =
                                checked(environment.total_days.checked_add(1))?;
                        }
                        let mut year = checked(environment.start_year.checked_add(1))?;
                        while year < environment.end_year {
                            if leap_year(year) {
                                environment.total_days =
                                    checked(environment.total_days.checked_add(1))?;
                            }
                            year = checked(year.checked_add(1))?;
                        }
                    }
                    environment.raw_simulation_days = environment.total_days;
                }
            }
            environment.use_daylight_saving = run_period.use_dst;
            environment.use_holidays = run_period.use_holidays;
            environment.title = if run_period.title.is_empty() {
                self.cursor.header().weather_location_title.clone()
            } else {
                run_period.title.clone()
            };
            if environment.kind_of_environment == 6 {
                environment.kind_of_environment_character = "ReadAllWeatherDataRunPeriod".into();
            } else {
                environment.kind_of_environment_character = "WeatherFileRunPeriod".into();
                environment.kind_of_environment = 3;
            }
            environment.day_of_week = run_period.day_of_week;
            environment.month_week_day = run_period.month_week_day;
            environment.set_week_days = false;
            environment.apply_weekend_rule = run_period.apply_weekend_rule;
            environment.use_rain = run_period.use_rain;
            environment.use_snow = run_period.use_snow;
            environment.first_hour_interpolation_uses_hour_one =
                run_period.first_hour_interpolation_uses_hour_one;
            self.state.weather.envrn = checked(self.state.weather.envrn.checked_add(1))?;
        }
        Ok(())
    }
}
