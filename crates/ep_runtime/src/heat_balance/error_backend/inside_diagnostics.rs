//! Actual timestamp and recurring warning helpers over their existing owners.
use super::{MESSAGE_SEARCH, SourceErrorBackend, SourceErrorBackendFailure, effect, increment};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_initialization::SourceRecurringErrorState;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

fn defined_int(value: f64) -> Result<i32, Error> {
    if !value.is_finite() || value.trunc() < i32::MIN as f64 || value.trunc() > i32::MAX as f64 {
        return Err(Error::Unavailable(
            "CreateSysTimeIntervalString defined integer conversion",
        ));
    }
    Ok(value as i32)
}

/// General.cc831-887. Each arithmetic operation and time branch is retained.
fn system_interval(
    clock: &WeatherGlobalState,
    hvac: &SelectedHvacGlobals,
) -> Result<String, Error> {
    let elapsed = hvac.system_time_elapsed;
    let system_step = hvac.time_step_sys;
    let (start, end) = if elapsed == 0.0 {
        let end = clock.current_time;
        (end - clock.time_step_zone, end)
    } else if (clock.time_step_zone - elapsed).abs() <= 0.0001 {
        let end = clock.current_time;
        (end - system_step, end)
    } else {
        let start = clock.current_time - clock.time_step_zone + elapsed;
        (start, start + system_step)
    };
    let mut start_hour = defined_int(start)?;
    // ObjexxFCL Fmath.hh566-591: retain the actual add/sign/cast order.
    let fractional_minutes = (start - start_hour as f64) * 60.0;
    let sign = if fractional_minutes >= 0.0 { 1.0 } else { -1.0 };
    let mut start_minute = defined_int(fractional_minutes + (sign * 0.5))?;
    if start_minute == 60 {
        start_hour = start_hour
            .checked_add(1)
            .ok_or(Error::Unavailable("timestamp hour increment"))?;
        start_minute = 0;
    }
    let start_text = format!("{start_hour:02}:{start_minute:02}");
    let end_hour = defined_int(end)?;
    let minutes = (end - end_hour as f64) * 60.0;
    if !minutes.is_finite() {
        return Err(Error::Unavailable("timestamp finite F-format minutes"));
    }
    let mut end_text = format!("{end_hour:02}:{minutes:2.0}");
    if end_text.as_bytes().get(3) == Some(&b' ') {
        end_text.replace_range(3..4, "0");
    }
    Ok(start_text + " - " + &end_text)
}

impl SourceErrorBackend<'_> {
    fn show_error_message_without_optional_outputs(
        &mut self,
        message: &str,
    ) -> Result<(), SourceErrorBackendFailure> {
        Self::write_error_message(
            self.state,
            self.error_stream.as_deref_mut(),
            self.console,
            None,
            None,
            message,
        )
    }
    /// The real warning operation with current phase references, not legacy
    /// callback-state phase copies. Other active callers retain their API.
    pub(crate) fn show_inside_warning_error(
        &mut self,
        message: &str,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        Self::count_warning_error(
            &mut self.state.tracking,
            message,
            &clock.warmup_flag,
            &globals.doing_sizing,
            &globals.kickoff_simulation,
        )?;
        self.write_warning_error(message)?;
        if self.error_callback.is_some() {
            return Err(Error::Unavailable(
                "actual inside ShowWarningError live-owner errorCallback1014",
            ));
        }
        Ok(())
    }

    /// UtilityRoutines892-952 with actual environment, clock and HVAC cells.
    pub(crate) fn show_source_continue_error_time_stamp(
        &mut self,
        message: &str,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
        environment: &WeatherEnvironmentState,
        hvac: &SelectedHvacGlobals,
    ) -> Result<(), Error> {
        let header = match (clock.warmup_flag, globals.doing_sizing) {
            (true, false) => " During Warmup, Environment=",
            (true, true) => " During Warmup & Sizing, Environment=",
            (false, false) => " Environment=",
            (false, true) => " During Sizing, Environment=",
        };
        if message.len() < 50 {
            let text = format!(
                "{message}{header}{}, at Simulation time={} {}",
                environment.environment_name,
                environment.current_month_day,
                system_interval(clock, hvac)?
            );
            self.show_error_message(&format!(" **   ~~~   ** {text}"))?;
            if let Some(sqlite) = self.sqlite.as_deref_mut() {
                effect(
                    "SQLite update timestamp continue",
                    sqlite.update_error_record(&text),
                )?;
            }
            if self.error_callback.is_some() {
                return Err(Error::Unavailable(
                    "actual timestamp live-owner errorCallback928",
                ));
            }
        } else {
            let postfix = format!(
                "{header}{}, at Simulation time={} {}",
                environment.environment_name,
                environment.current_month_day,
                system_interval(clock, hvac)?
            );
            // Native's first long-message write has no optional output args.
            self.show_error_message_without_optional_outputs(&format!(" **   ~~~   ** {message}"))?;
            self.show_error_message(&format!(" **   ~~~   ** {postfix}"))?;
            if let Some(sqlite) = self.sqlite.as_deref_mut() {
                effect(
                    "SQLite update timestamp continue",
                    sqlite.update_error_record(message),
                )?;
            }
            if self.error_callback.is_some() {
                return Err(Error::Unavailable(
                    "actual long timestamp live-owner errorCallback946",
                ));
            }
        }
        Ok(())
    }

    /// UtilityRoutines1146-1195 first overload. SameString matches the actual
    /// ASCII diagnostic messages in this caller; no raw/derived counter seeds.
    pub(crate) fn show_source_recurring_warning_at_end(
        &mut self,
        message: &str,
        message_index: &mut i32,
        state: &mut SourceRecurringErrorState,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
        maximum: Option<f64>,
        minimum: Option<f64>,
        sum: Option<f64>,
        maximum_units: &str,
        minimum_units: &str,
        sum_units: &str,
    ) -> Result<(), Error> {
        for (needle, count) in MESSAGE_SEARCH
            .iter()
            .zip(&mut self.state.tracking.match_counts)
        {
            if message.contains(needle) {
                increment(count, "MatchCounts")?;
                break;
            }
        }
        let full_message = " ** Warning ** ".to_owned() + message;
        let mut found = false;
        for ordinal in 1..=state.number {
            let row = state
                .rows
                .as_deref()
                .and_then(|v| v.get((ordinal - 1) as usize))
                .ok_or(Error::Unavailable(
                    "actual allocated RecurringErrors search row",
                ))?;
            if row.message.eq_ignore_ascii_case(&full_message) {
                *message_index = ordinal;
                found = true;
                break;
            }
        }
        if !found {
            *message_index = 0;
        }
        increment(
            &mut self.state.tracking.total_warning_errors,
            "TotalWarningErrors",
        )?;
        self.store_source_recurring_error_message(
            &full_message,
            message_index,
            state,
            clock,
            globals,
            maximum,
            minimum,
            sum,
            maximum_units,
            minimum_units,
            sum_units,
        )
    }

    /// UtilityRoutines1293-1379: true Array1D redimension preserves old rows;
    /// each new class row has its actual declared constructor defaults.
    fn store_source_recurring_error_message(
        &mut self,
        message: &str,
        message_index: &mut i32,
        state: &mut SourceRecurringErrorState,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
        maximum: Option<f64>,
        minimum: Option<f64>,
        sum: Option<f64>,
        maximum_units: &str,
        minimum_units: &str,
        sum_units: &str,
    ) -> Result<(), Error> {
        if *message_index == 0 {
            increment(&mut state.number, "NumRecurringErrors")?;
            let size = usize::try_from(state.number)
                .map_err(|_| Error::Unavailable("RecurringErrors extent"))?;
            let rows = state.rows.get_or_insert_with(Vec::new);
            if size > rows.len() {
                rows.try_reserve_exact(size - rows.len()).map_err(|_| {
                    Error::Unavailable("actual RecurringErrors redimension allocation")
                })?;
            }
            rows.resize_with(size, Default::default);
            *message_index = state.number;
            let row = rows
                .last_mut()
                .ok_or(Error::Unavailable("new RecurringErrors class row"))?;
            row.message = message.to_owned();
            row.count = 1;
            if clock.warmup_flag {
                row.warmup_count = 1;
            }
            if globals.doing_sizing {
                row.sizing_count = 1;
            }
            if let Some(value) = maximum {
                row.max_value = value;
                row.report_max = true;
                if !maximum_units.is_empty() {
                    row.max_units = maximum_units.to_owned();
                }
            }
            if let Some(value) = minimum {
                row.min_value = value;
                row.report_min = true;
                if !minimum_units.is_empty() {
                    row.min_units = minimum_units.to_owned();
                }
            }
            if let Some(value) = sum {
                row.sum_value = value;
                row.report_sum = true;
                if !sum_units.is_empty() {
                    row.sum_units = sum_units.to_owned();
                }
            }
        } else if *message_index > 0 {
            let at = usize::try_from(*message_index - 1)
                .map_err(|_| Error::Unavailable("recurring index"))?;
            let row = state
                .rows
                .as_mut()
                .and_then(|v| v.get_mut(at))
                .ok_or(Error::Unavailable(
                    "same allocated RecurringErrors store row",
                ))?;
            increment(&mut row.count, "RecurringErrors.Count")?;
            if clock.warmup_flag {
                increment(&mut row.warmup_count, "RecurringErrors.WarmupCount")?;
            }
            if globals.doing_sizing {
                increment(&mut row.sizing_count, "RecurringErrors.SizingCount")?;
            }
            if let Some(value) = maximum {
                row.max_value = if value < row.max_value {
                    row.max_value
                } else {
                    value
                };
                row.report_max = true;
            }
            if let Some(value) = minimum {
                row.min_value = if row.min_value < value {
                    row.min_value
                } else {
                    value
                };
                row.report_min = true;
            }
            if let Some(value) = sum {
                row.sum_value += value;
                row.report_sum = true;
            }
        }
        Ok(())
    }
}
