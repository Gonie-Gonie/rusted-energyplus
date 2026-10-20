//! Real raw-day sink: physical parsing mutates one cursor/header and one WeatherDayState.
use super::{Error, Failure, ordinal};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::{WeatherDayError, WeatherDayState, WeatherSession, WeatherVars};
use crate::weather::raw::{
    RawDayFailure, RawDayReadMode, RawDayReadSelection, RawEpwCursorOwner, RawEpwHeaderState,
    RawEpwLineRead, RawEpwOutputs, RawWeatherDay, RawWeatherSlot, SourceRawDayReadCallbacks,
    SourceRawDayReadClock, SourceRawDayReadFailure,
};

struct SourceDayReadSink<'a, 'backend> {
    state: &'a mut WeatherDayState,
    environ: usize,
    globals: &'a SelectedHeatBalanceGlobals,
    developer_flag: &'a bool,
    psy: &'a mut SourcePsychrometricAccess,
    backend: &'a mut SourceErrorBackend<'backend>,
}
impl SourceRawDayReadClock for SourceDayReadSink<'_, '_> {
    fn current_year_is_leap_year(&self) -> Result<bool, RawDayFailure> {
        Ok(self.state.environment.current_year_is_leap_year)
    }
    fn current_day_of_week(&self) -> i32 {
        self.state.weather.cur_day_of_week
    }
    fn set_current_day_of_week(&mut self, value: i32) {
        self.state.weather.cur_day_of_week = value;
    }
    fn set_read_time(&mut self, value: f64) {
        self.state.weather.read_e_plus_weather_cur_time = value;
    }
    fn set_last_hour_set(&mut self, value: bool) {
        self.state.weather.last_hour_set = value;
    }
    fn interpret_record(
        &mut self,
        cursor: &mut RawEpwCursorOwner,
        read: &RawEpwLineRead,
    ) -> Result<(), RawDayFailure> {
        cursor.interpret_with_live_source_counter(read, &mut self.state.missed_counts.weath_codes)
    }
}
impl SourceRawDayReadCallbacks for SourceDayReadSink<'_, '_> {
    type Error = Failure;
    fn prepare_hour(&mut self, args: &mut RawEpwOutputs) -> Result<(), Failure> {
        super::hourly::prepare_source_hour(self.state, args)?;
        Ok(())
    }
    fn prepare_skipped_leap_day(&mut self, header: &mut RawEpwHeaderState) -> Result<(), Failure> {
        header.month_ends[1] = 28;
        self.state.weather.end_day_of_month_with_leap_day[1] = 28;
        Ok(())
    }
    fn report_skipped_leap_day(&mut self, header: &RawEpwHeaderState) -> Result<(), Failure> {
        let flags = SourceDiagnosticFlags {
            kickoff: &self.globals.kickoff_simulation,
            developer: self.developer_flag,
            warmup: &self.state.global.warmup_flag,
            doing_sizing: &self.globals.doing_sizing,
        };
        self.backend.show_source_warning_error(
            "ReadEPlusWeatherForDay: Feb29 data encountered but will not be processed.",
            &flags,
        )?;
        if !header.allows_leap_years {
            self.backend.show_continue_error(
                r#"...WeatherFile does not allow Leap Years. HOLIDAYS/DAYLIGHT SAVINGS header must indicate "Yes"."#,
            )?;
        }
        Ok(())
    }
    fn first_day_selected(
        &mut self,
        _slot: &RawWeatherSlot,
        header: &mut RawEpwHeaderState,
        args: &mut RawEpwOutputs,
    ) -> Result<(), Failure> {
        let mut errors = false;
        macro_rules! range_check {
            ($condition:expr,$value:expr,$name:literal,$range:literal,$precision:expr) => {
                if $condition {
                    self.backend.show_severe_error(&format!(
                        "ReadEPlusWeatherForDay: {}",
                        header.weather_location_title
                    ))?;
                    let value = crate::heat_balance::source_round_format::round($value, $precision)
                        .map_err(|_| {
                            Error::Format("actual initial weather R formatter".to_owned())
                        })?;
                    self.backend.show_continue_error(&format!(
                        concat!($name, " ({}) is out of range ", $range),
                        value
                    ))?;
                    errors = true;
                }
            };
        }
        range_check!(
            args.mandatory_reals[0] < 99.9
                && (args.mandatory_reals[0] < -90.0 || args.mandatory_reals[0] > 70.0),
            args.mandatory_reals[0],
            "DryBulb Temperature",
            "[-90.0, 70.0]",
            2
        );
        range_check!(
            args.mandatory_reals[1] < 99.9
                && (args.mandatory_reals[1] < -90.0 || args.mandatory_reals[1] > 70.0),
            args.mandatory_reals[1],
            "DewPoint Temperature",
            "[-90.0, 70.0]",
            2
        );
        range_check!(
            args.mandatory_reals[2] < 999.0
                && (args.mandatory_reals[2] < 0.0 || args.mandatory_reals[2] > 110.0),
            args.mandatory_reals[2],
            "Relative Humidity",
            "[0.0, 100.0]",
            2
        );
        range_check!(
            args.mandatory_reals[3] < 999999.0
                && (args.mandatory_reals[3] <= 31000.0 || args.mandatory_reals[3] > 120000.0),
            args.mandatory_reals[3],
            "Atmospheric Pressure",
            "[31000, 120000]",
            0
        );
        range_check!(
            args.mandatory_reals[8] < 0.0,
            args.mandatory_reals[8],
            "Direct Radiation",
            "[0.0, -]",
            2
        );
        range_check!(
            args.mandatory_reals[9] < 0.0,
            args.mandatory_reals[9],
            "Diffuse Radiation",
            "[0.0, -]",
            2
        );
        range_check!(
            args.mandatory_reals[14] < 999.0
                && (args.mandatory_reals[14] < 0.0 || args.mandatory_reals[14] > 360.0),
            args.mandatory_reals[14],
            "Wind Direction",
            "[0.0, 360.0]",
            2
        );
        range_check!(
            args.mandatory_reals[15] < 999.0
                && (args.mandatory_reals[15] < 0.0 || args.mandatory_reals[15] > 40.0),
            args.mandatory_reals[15],
            "Wind Speed",
            "[0.0, 40.0]",
            2
        );
        if errors {
            self.backend
                .show_severe_error("Out of Range errors found with initial day of WeatherFile")?;
        }
        // Native obtains the global envCurr reference before the cycle branch,
        // even when KickOff/DoingSizing selects its final weekday-only arm.
        self.state
            .weather
            .source_environments
            .as_ref()
            .and_then(|v| v.get(ordinal(self.state.weather.envrn).ok()?))
            .ok_or(Error::Unavailable("actual global envCurr reference2567"))?;
        let weather = &mut self.state.weather;
        let row = weather
            .source_environments
            .as_mut()
            .and_then(|v| v.get_mut(self.environ))
            .ok_or(Error::Unavailable(
                "actual selected Environment reference2573",
            ))?;
        // The true global Envrn row may differ from the passed Environ. Children
        // resolve its array by the actual index below, never clone either row.
        if !self.globals.kickoff_simulation
            && !self.globals.doing_sizing
            && row.kind_of_environment == 3
        {
            row.current_cycle = row.current_cycle.checked_add(1).ok_or(Error::Unavailable(
                "actual CurrentCycle signed increment2576",
            ))?;
            if !row.roll_day_type_on_repeat {
                super::calendar::initialize_day_of_week(
                    row.day_of_week,
                    &mut weather.cur_day_of_week,
                )?;
                self.reset_selected_dates(header)?;
            } else if row.current_cycle == 1 {
                super::calendar::initialize_day_of_week(
                    row.day_of_week,
                    &mut weather.cur_day_of_week,
                )?;
                row.set_week_days = true;
                self.reset_selected_dates(header)?;
            } else {
                weather.cur_day_of_week = self.state.environment.day_of_week_tomorrow;
            }
        } else {
            super::calendar::initialize_day_of_week(row.day_of_week, &mut weather.cur_day_of_week)?;
        }
        Ok(())
    }
    fn clear_source_interval(&mut self, hour: usize, interval: usize) -> Result<(), Failure> {
        *self
            .state
            .tomorrow_values
            .hour_mut(hour)
            .map_err(WeatherDayError::from)?
            .get_mut(interval - 1)
            .ok_or(Error::Unavailable("actual Tomorrow interval clear2608"))? =
            WeatherVars::default();
        Ok(())
    }
    fn parsed_hour(
        &mut self,
        hour: usize,
        _slot: &RawWeatherSlot,
        header: &mut RawEpwHeaderState,
        args: &mut RawEpwOutputs,
    ) -> Result<(), Failure> {
        super::hourly::process_source_hour(
            self.state,
            header,
            self.environ,
            hour,
            args,
            self.psy,
            self.backend,
        )
    }
    fn complete_day(
        &mut self,
        _day: &RawWeatherDay,
        header: &mut RawEpwHeaderState,
    ) -> Result<(), Failure> {
        super::interpolation::interpolate_source_day(self.state, header, self.environ, self.psy)
    }
}
impl SourceDayReadSink<'_, '_> {
    fn reset_selected_dates(&mut self, header: &mut RawEpwHeaderState) -> Result<(), Failure> {
        let flags = SourceDiagnosticFlags {
            kickoff: &self.globals.kickoff_simulation,
            developer: self.developer_flag,
            warmup: &self.state.global.warmup_flag,
            doing_sizing: &self.globals.doing_sizing,
        };
        let current = ordinal(self.state.weather.envrn)?;
        // Exact month array is small Native borrowed input; the callee needs its
        // same live storage, split from the remaining WeatherManager fields.
        let weather = &mut self.state.weather;
        let row = weather
            .source_environments
            .as_ref()
            .and_then(|v| v.get(current))
            .ok_or(Error::Unavailable("actual global envCurr reference2567"))?;
        if weather.daylight_saving_is_active {
            super::calendar::set_dst_dates(
                &row.month_week_day,
                &weather.end_day_of_month_with_leap_day,
                &mut weather.dst_index,
                header,
                self.backend,
                &flags,
            )?;
        }
        super::calendar::set_special_days(
            &row.month_week_day,
            &weather.end_day_of_month_with_leap_day,
            &weather.weekday_types,
            &mut weather.special_day_types,
            &weather.use_special_days,
            &row.apply_weekend_rule,
            header,
            self.backend,
            &flags,
        )
    }
}
impl WeatherSession {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn read_source_weather_for_day(
        &mut self,
        day: i32,
        environ: i32,
        backspace: bool,
        globals: &SelectedHeatBalanceGlobals,
        developer_flag: &bool,
        psy: &mut SourcePsychrometricAccess,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Failure> {
        let index = ordinal(environ)?;
        let row = self
            .state
            .weather
            .source_environments
            .as_ref()
            .and_then(|v| v.get(index))
            .ok_or(Error::Unavailable(
                "actual ReadEPlusWeatherForDay thisEnviron2347",
            ))?;
        let selection = RawDayReadSelection {
            start_month: row.start_month,
            start_day: row.start_day,
            start_year: row.start_year,
            match_year: row.match_year,
            days_in_year: self.state.weather.number_days_in_year,
        };
        let mut sink = SourceDayReadSink {
            state: &mut self.state,
            environ: index,
            globals,
            developer_flag,
            psy,
            backend,
        };
        self.cursor
            .read_day_with_live_source_state(
                selection,
                if day == 1 {
                    RawDayReadMode::FirstDay
                } else {
                    RawDayReadMode::NextDay
                },
                backspace,
                &mut sink,
            )
            .map_err(|error| match error {
                SourceRawDayReadFailure::Native(error) => Failure::Raw(error),
                SourceRawDayReadFailure::Callback(error) => error,
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod skip_tests {
    use super::*;
    use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
    use crate::weather::day::{WeatherDayValues, WeatherVars};
    use crate::weather::raw::{HeaderCallerPreparation, RawDaySourceFatal};

    // Explicit unit boundary construction, not a production activation/reset.
    fn errors() -> SourceErrorState {
        SourceErrorState {
            output_error_header: false,
            version_string: String::new(),
            idd_version_string: String::new(),
            doing_input_processing: false,
            print_console_output: false,
            kickoff_simulation: false,
            developer_flag: false,
            warmup_flag: false,
            doing_sizing: false,
            tracking: SourceErrorTracking {
                ask_for_plant_check_on_abort: false,
                exit_during_simulations: false,
                match_counts: [0; 20],
                abort_processing: false,
                ask_for_connections_report: false,
                ask_for_surfaces_report: false,
                total_multiplied_windows: 0,
                total_coincident_vertices: 0,
                total_degenerate_surfaces: 0,
                total_receiving_non_convex_surfaces: 0,
                total_casting_non_convex_surfaces: 0,
                total_severe_errors: 0,
                total_severe_errors_during_warmup: 0,
                total_severe_errors_during_sizing: 0,
                last_severe_error: String::new(),
                total_warning_errors: 0,
                total_warning_errors_during_warmup: 0,
                total_warning_errors_during_sizing: 0,
            },
        }
    }

    #[test]
    fn skipped_hours_keep_real_prefix_effects_caches_slots_and_live_warning_phase() {
        for allows in [true, false] {
            let mut text = format!(
                "LOCATION,Example,CO,USA,Test,000000,39.74,-105.18,-7,1600\nDESIGN CONDITIONS,0\nTYPICAL/EXTREME PERIODS,0\nGROUND TEMPERATURES,0\nHOLIDAYS/DAYLIGHT SAVINGS,{},0,0,0\nCOMMENTS 1,\nCOMMENTS 2,\nDATA PERIODS,1,1,Data,Sunday,1/1,12/31\n",
                if allows { "Yes" } else { "No" }
            );
            for hour in 1..=24 {
                text.push_str(&format!("2016,2,29,{hour},60,A,9999,9999,-1,-1,-1,-1,0,-1,9999,-1,-1,-1,-1,-1,361,-1,-1,-1,-1,-1,0,9,-1,-1,-1,-1,-1,-1\n"));
            }
            text.push_str("2016,3,1,2,60,A,20,10,50,101325,0,0,300,0,0,0,0,0,0,0,180,2.5,0,0,0,0,9,999999999,0,0,0,0,0,0\n");
            let mut cursor = RawEpwCursorOwner::open_from_bytes(
                text.into_bytes(),
                RawEpwHeaderState::default(),
                HeaderCallerPreparation::default(),
            )
            .unwrap();
            cursor.source_header_mut().month_ends[1] = 29;
            let mut state = WeatherDayState::default();
            state.environment.current_year_is_leap_year = false;
            state.environment.display_weather_missing_data_warnings = true;
            state.global.warmup_flag = true;
            state.weather.end_day_of_month_with_leap_day[1] = 29;
            state.weather.cur_day_of_week = 5;
            state.weather.read_e_plus_weather_cur_time = 3.25;
            state.weather.last_hour_set = true;
            state.last_hour.out_dry_bulb_temp = 42.0;
            state.missing_values.base.out_dry_bulb_temp = 41.0;
            state.tomorrow_values = WeatherDayValues::allocated(4).unwrap();
            state.tomorrow_values.slots_mut().fill(WeatherVars {
                out_dry_bulb_temp: 77.0,
                ..Default::default()
            });
            let globals = SelectedHeatBalanceGlobals::default();
            let developer = false;
            let mut psy = SourcePsychrometricAccess;
            let mut error_state = errors();
            let mut error_text = Vec::new();
            let mut console = Vec::new();
            let result = {
                let mut backend = SourceErrorBackend {
                    state: &mut error_state,
                    error_stream: Some(&mut error_text),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                };
                let mut sink = SourceDayReadSink {
                    state: &mut state,
                    environ: 0,
                    globals: &globals,
                    developer_flag: &developer,
                    psy: &mut psy,
                    backend: &mut backend,
                };
                cursor.read_day_with_live_source_state(
                    RawDayReadSelection {
                        start_month: 2,
                        start_day: 28,
                        start_year: 2013,
                        match_year: false,
                        days_in_year: 365,
                    },
                    RawDayReadMode::NextDay,
                    false,
                    &mut sink,
                )
            };
            assert!(matches!(
                result,
                Err(SourceRawDayReadFailure::Native(RawDayFailure::SourceFatal(
                    RawDaySourceFatal::UnexpectedHour {
                        expected: 1,
                        actual: 2
                    }
                )))
            ));
            assert_eq!(cursor.interpret_count(), 25);
            assert_eq!(state.missed_counts.weath_codes, 24);
            assert_eq!(state.missed_counts.beam_solar_rad, 24);
            assert_eq!(state.out_of_range_counts.dif_solar_rad, 24);
            assert_eq!(state.missed_counts.out_dry_bulb_temp, 0);
            assert_eq!(state.missing_values.base.out_dry_bulb_temp, 41.0);
            assert_eq!(state.last_hour.out_dry_bulb_temp, 42.0);
            assert!(state.weather.last_hour_set);
            assert_eq!(state.weather.cur_day_of_week, 5);
            assert_eq!(state.weather.read_e_plus_weather_cur_time, 3.25);
            assert_eq!(cursor.header().month_ends[1], 28);
            assert_eq!(state.weather.end_day_of_month_with_leap_day[1], 28);
            assert_eq!(cursor.header().allows_leap_years, allows);
            assert!(!state.environment.current_year_is_leap_year);
            for hour in 1..=24 {
                let slots = state.tomorrow_values.hour(hour).unwrap();
                assert_eq!(slots[0], WeatherVars::default());
                assert!(slots[1..].iter().all(|slot| slot.out_dry_bulb_temp == 77.0));
            }
            assert_eq!(error_state.tracking.total_warning_errors, 1);
            assert_eq!(error_state.tracking.total_warning_errors_during_warmup, 1);
            assert_eq!(error_state.tracking.total_warning_errors_during_sizing, 0);
            let warning = "   ** Warning ** ReadEPlusWeatherForDay: Feb29 data encountered but will not be processed.\n";
            let expected = if allows {
                warning.to_owned()
            } else {
                format!(
                    "{warning}   **   ~~~   ** ...WeatherFile does not allow Leap Years. HOLIDAYS/DAYLIGHT SAVINGS header must indicate \"Yes\".\n"
                )
            };
            assert_eq!(String::from_utf8(error_text).unwrap(), expected);
        }
    }
}
