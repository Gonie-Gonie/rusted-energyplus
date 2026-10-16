//! Genuine GetNextEnvironment739-1215 continuation, preserving the real caller flag.
//! DesignDay/adaptive/DST/positive special-day children remain explicit boundaries.
use super::source_initialization::SourceWeatherInitializationOwners;
use super::{SourceWeatherInitializationError as Failure, WeatherSession};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
mod design_day;
mod progress;
mod run_period;
pub(crate) use progress::SourceProgressCallbacks;
use run_period::{SourceRunEnvironmentCalendarOwners, initialize_source_run_environment};

fn ordinal(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Error::Unavailable(
            "actual positive Source environment/calendar ordinal",
        ))
}
fn fatal(backend: &mut SourceErrorBackend<'_>, text: &str) -> Failure {
    match backend.show_fatal_error(text) {
        Ok(never) => match never {},
        Err(error) => Error::Fatal(error).into(),
    }
}

impl WeatherSession {
    /// Exactly739-771; call only inside the original GetBranchInputOneTimeFlag block.
    pub(crate) fn finish_source_environment_input(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
    ) -> Result<(), Failure> {
        if owners.system.dd_only && self.state.environment.tot_des_days == 0 {
            self.errors_found = true;
            owners.backend.show_severe_error("GetNextEnvironment: Requested Design Days only (DataSystemVariables::DDOnly) but no Design Days specified, program will terminate.")?;
        }
        if *owners.reverse_design_days && self.state.environment.tot_des_days == 1 {
            self.errors_found = true;
            owners.backend.show_severe_error("GetNextEnvironment: Requested Reverse Design Days (DataSystemVariables::ReverseDD) but only 1 Design Day specified, program will terminate.")?;
        }
        if self.errors_found {
            self.cursor.close();
            return Err(fatal(
                owners.backend,
                "GetNextEnvironment: Errors found in Weather Data Input. Program terminates.",
            ));
        }
        self.state.environment.current_overall_sim_day = 0;
        self.state.environment.total_overall_sim_days = 0;
        self.state.environment.max_number_sim_years = 1;
        for number in 1..=self.state.weather.num_of_envrn {
            let row = self
                .state
                .weather
                .source_environments
                .as_ref()
                .and_then(|rows| rows.get(ordinal(number).ok()?))
                .ok_or(Error::Unavailable("actual Environment TotalDays row766"))?;
            self.state.environment.total_overall_sim_days = self
                .state
                .environment
                .total_overall_sim_days
                .checked_add(row.total_days)
                .ok_or(Error::Unavailable(
                    "actual TotalOverallSimDays signed addition766",
                ))?;
            if row.kind_of_environment == 3 {
                self.state.environment.max_number_sim_years =
                    if self.state.environment.max_number_sim_years < row.number_simulation_years {
                        row.number_simulation_years
                    } else {
                        self.state.environment.max_number_sim_years
                    };
            }
        }
        progress::display_source_simulation_days(
            owners.globals,
            owners.system,
            self.state.environment.current_overall_sim_day,
            self.state.environment.total_overall_sim_days,
        )?;
        Ok(())
    }

    /// Exactly774-1215 each call. `available` is the actual caller reference.
    /// The legacy Session.available is neither read nor written by this route.
    pub(crate) fn advance_source_environment(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        available: &mut bool,
    ) -> Result<bool, Failure> {
        self.cursor.close();
        let weather = &mut self.state.weather;
        weather.envrn = weather
            .envrn
            .checked_add(1)
            .ok_or(Error::Unavailable("actual Envrn signed increment775"))?;
        weather.dates_should_be_reset = false;
        if weather.envrn > weather.num_of_envrn {
            *available = false;
            weather.envrn = 0;
            self.state.environment.cur_envir_num = 0;
        } else {
            let row = weather
                .source_environments
                .as_mut()
                .and_then(|rows| rows.get_mut(ordinal(weather.envrn).ok()?))
                .ok_or(Error::Unavailable(
                    "actual current Environment reference782",
                ))?;
            let clock = &mut self.state.global;
            let environment = &mut self.state.environment;
            clock.kind_of_sim = row.kind_of_environment;
            environment.day_of_year = row.start_julian_day;
            environment.day_of_month = row.start_day;
            clock.calendar_year = row.start_year;
            clock.calendar_year_chr = clock.calendar_year.to_string();
            environment.month = row.start_month;
            clock.num_of_day_in_envrn = row.total_days;
            if !owners.globals.doing_sizing
                && !owners.globals.kickoff_simulation
                && (owners.control.adaptive_comfort_requested_ash55
                    || owners.control.adaptive_comfort_requested_cen15251)
            {
                if clock.kind_of_sim == 1 {
                    if clock.do_des_day_sim {
                        owners.backend.show_warning_error("GetNextEnvironment: Adaptive Comfort being reported during design day.")?;
                        return Err(Error::Unavailable(
                            "actual adaptive DesignDay MaxDryBulb reference796",
                        )
                        .into());
                    }
                } else if clock.do_weath_sim || clock.do_des_day_sim {
                    if owners.control.adaptive_comfort_requested_ash55 {
                        return Err(Error::Unavailable(
                            "actual CalcThermalComfortAdaptiveASH55 initialization810",
                        )
                        .into());
                    }
                    if owners.control.adaptive_comfort_requested_cen15251 {
                        return Err(Error::Unavailable(
                            "actual CalcThermalComfortAdaptiveCEN15251 initialization813",
                        )
                        .into());
                    }
                }
            }
            if weather.envrn > environment.tot_des_days && weather.weather_file_exists {
                self.cursor.open_source_epw_file(
                    owners.input_weather_file_path,
                    false,
                    &mut self.errors_found,
                    owners.input,
                    owners.files,
                    owners.backend,
                )?;
            }
            *available = true;
            if clock.kind_of_sim == 3 && !weather.weather_file_exists && clock.do_weath_sim {
                if !owners.globals.doing_sizing && !owners.globals.kickoff_simulation {
                    owners.backend.show_severe_error(
                        "Weather Simulation requested, but no weather file attached.",
                    )?;
                    self.errors_found = true;
                }
                if !owners.globals.doing_hvac_sizing_simulations {
                    weather.envrn = 0;
                }
                *available = false;
            } else if clock.kind_of_sim == 3 && !weather.weather_file_exists && !clock.do_weath_sim
            {
                *available = false;
                if !owners.globals.doing_hvac_sizing_simulations {
                    weather.envrn = 0;
                }
            } else if clock.kind_of_sim == 3 && owners.globals.doing_sizing {
                *available = false;
                weather.envrn = 0;
            }
            if !self.errors_found && *available && weather.envrn > 0 {
                environment.environment_name = row.title.clone();
                environment.cur_envir_num = weather.envrn;
                environment.run_period_start_day_of_week = 0;
                if (clock.do_des_day_sim && clock.kind_of_sim != 3)
                    || (clock.kind_of_sim == 3 && clock.do_weath_sim)
                {
                    if weather.print_environment_headers
                        && owners.report_flags.do_weather_init_reporting
                    {
                        writeln!(owners.eio,"! <Environment>,Environment Name,Environment Type, Start Date, End Date, Start DayOfWeek, Duration {{#days}}, Source:Start DayOfWeek,  Use Daylight Saving, Use Holidays, Apply Weekend Holiday Rule,  Use Rain Values, Use Snow Values, Sky Temperature Model\n! <Environment:Special Days>, Special Day Name, Special Day Type, Source, Start Date, Duration {{#days}}\n! <Environment:Daylight Saving>, Daylight Saving Indicator, Source, Start Date, End Date\n! <Environment:WarmupDays>, NumberofWarmupDays").map_err(Error::Io)?;
                        weather.print_environment_headers = false;
                    }
                    if clock.kind_of_sim == 3 || clock.kind_of_sim == 2 {
                        initialize_source_run_environment(
                            row,
                            environment,
                            &mut SourceRunEnvironmentCalendarOwners {
                                use_daylight_saving: &mut weather.use_daylight_saving,
                                use_special_days: &mut weather.use_special_days,
                                use_rain_values: &mut weather.use_rain_values,
                                use_snow_values: &mut weather.use_snow_values,
                                daylight_saving_is_active: &mut weather.daylight_saving_is_active,
                                end_day_of_month_with_leap_day: &mut weather
                                    .end_day_of_month_with_leap_day,
                                weekday_types: &mut weather.weekday_types,
                                special_day_types: &mut weather.special_day_types,
                                current_simulation_day_for_end_of_run_period: &mut weather
                                    .cur_sim_day_for_end_of_run_period,
                                start_dates_cycle_should_be_reset: &mut weather
                                    .start_dates_cycle_should_be_reset,
                                jan1_dates_should_be_reset: &mut weather.jan1_dates_should_be_reset,
                            },
                            self.cursor.source_header_mut(),
                            clock,
                            owners.globals,
                            owners.control,
                            owners.report_flags,
                            owners.eio,
                            owners.backend,
                        )?;
                    } else if clock.kind_of_sim == 1 || clock.kind_of_sim == 4 {
                        let design_day = weather
                            .source_design_day_input
                            .design_day_input
                            .as_ref()
                            .and_then(|rows| rows.get(ordinal(row.design_day_number).ok()?))
                            .ok_or(Error::Unavailable("actual DesDayInput reference1178"))?;
                        design_day::initialize_source_design_day_environment(
                            row,
                            environment,
                            design_day,
                            owners.report_flags,
                            owners.eio,
                        )?;
                    }
                }
            }
        }
        if self.errors_found && !owners.globals.doing_sizing && !owners.globals.kickoff_simulation {
            owners.backend.show_severe_error(
                "GetNextEnvironment: Errors found in getting a new environment",
            )?;
            *available = false;
        } else if self.errors_found {
            *available = false;
        }
        Ok(*available && !self.errors_found)
    }
}

impl super::WeatherDayState {
    /// Same DisplaySimDaysProgress child used by GetNextEnvironment and primary.
    /// This forwards actual retained operands and creates no progress owner.
    pub(crate) fn display_source_primary_simulation_days(
        &self,
        globals: &crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals,
        system: &crate::heat_balance::selected_project_data::ProjectSystemVariables,
    ) -> Result<(), Error> {
        progress::display_source_simulation_days(
            globals,
            system,
            self.environment.current_overall_sim_day,
            self.environment.total_overall_sim_days,
        )
    }
}
