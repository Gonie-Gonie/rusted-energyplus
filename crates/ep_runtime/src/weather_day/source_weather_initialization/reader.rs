//! Real raw-day sink: physical parsing mutates one cursor/header and one WeatherDayState.
use super::{Error, Failure, ordinal};
use crate::heat_balance::error_backend::SourceErrorBackend;
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
            super::calendar::set_dst_dates(&row.month_week_day, &mut weather.dst_index, header)?;
        }
        super::calendar::set_special_days(
            &row.month_week_day,
            &mut weather.special_day_types,
            &weather.use_special_days,
            header,
            self.backend,
        )
    }
}
impl WeatherSession {
    pub(crate) fn read_source_weather_for_day(
        &mut self,
        day: i32,
        environ: i32,
        backspace: bool,
        globals: &SelectedHeatBalanceGlobals,
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
