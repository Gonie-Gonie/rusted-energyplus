//! One actual cursor/day loop shared by legacy and live Source state borrowers.
//! Source callbacks own the single state borrow; no clock snapshot/copyback.
use super::{
    RawDayFailure, RawDayReadCallbacks, RawDayReadMode, RawDayReadSelection, RawDayReadState,
    RawDaySourceFatal, RawEpwCursorOwner, RawEpwHeaderError, RawEpwHeaderState, RawEpwLineRead,
    RawEpwOutputs, RawWeatherDay, RawWeatherSlot,
};

/// Access each genuine field at its actual Source read/write position.
/// A Source sink borrows its WeatherDayState once and implements these methods
/// directly against WeatherOwner CurDayOfWeek/ReadEPlusWeatherCurTime/LastHourSet.
pub(crate) trait SourceRawDayReadClock {
    /// Actual CurrentYearIsLeapYear; a raw-only adapter has no calendar owner.
    fn current_year_is_leap_year(&self) -> Result<bool, RawDayFailure>;
    fn current_day_of_week(&self) -> i32;
    fn set_current_day_of_week(&mut self, value: i32);
    fn set_read_time(&mut self, value: f64);
    fn set_last_hour_set(&mut self, value: bool);
    /// Source implementation passes its actual missed_counts.weath_codes;
    /// the legacy adapter delegates to its existing internal parser owner.
    fn interpret_record(
        &mut self,
        cursor: &mut RawEpwCursorOwner,
        read: &RawEpwLineRead,
    ) -> Result<(), RawDayFailure>;
}
/// All actual processing children are mandatory on the live Source entry.
/// Unlike the retained raw-only legacy trait, no successful defaults exist.
pub(crate) trait SourceRawDayReadCallbacks: SourceRawDayReadClock {
    type Error;
    fn first_day_selected(
        &mut self,
        slot: &RawWeatherSlot,
        header: &mut RawEpwHeaderState,
        arguments: &mut RawEpwOutputs,
    ) -> Result<(), Self::Error>;
    fn clear_source_interval(&mut self, hour: usize, interval: usize) -> Result<(), Self::Error>;
    /// `arguments` is the actual parser reference owner. Source hourly physical
    /// preprocessing must mutate it directly; detached slot is metadata only.
    fn parsed_hour(
        &mut self,
        hour: usize,
        slot: &RawWeatherSlot,
        header: &mut RawEpwHeaderState,
        arguments: &mut RawEpwOutputs,
    ) -> Result<(), Self::Error>;
    fn complete_day(
        &mut self,
        day: &RawWeatherDay,
        header: &mut RawEpwHeaderState,
    ) -> Result<(), Self::Error>;
}
/// Preserve genuine raw failures and the actual processing child's typed cause.
#[derive(Debug)]
pub(crate) enum SourceRawDayReadFailure<E> {
    Native(RawDayFailure),
    Callback(E),
}
impl<E> From<RawDayFailure> for SourceRawDayReadFailure<E> {
    fn from(error: RawDayFailure) -> Self {
        Self::Native(error)
    }
}
impl<E> From<RawEpwHeaderError> for SourceRawDayReadFailure<E> {
    fn from(error: RawEpwHeaderError) -> Self {
        Self::Native(RawDayFailure::Header(error))
    }
}
pub(super) struct LegacyDayReadCallbacks<'a, C> {
    pub(super) state: &'a mut RawDayReadState,
    pub(super) callbacks: &'a mut C,
}
impl<C> SourceRawDayReadClock for LegacyDayReadCallbacks<'_, C> {
    fn current_year_is_leap_year(&self) -> Result<bool, RawDayFailure> {
        // This retained raw-only entry never owned CurrentYearIsLeapYear.
        // Keep its Feb29 unavailable result rather than infer a calendar flag.
        Err(RawDayFailure::OutsideBoundedDomain(
            "February 29 daily retry",
        ))
    }
    fn current_day_of_week(&self) -> i32 {
        self.state.current_day_of_week
    }
    fn set_current_day_of_week(&mut self, value: i32) {
        self.state.current_day_of_week = value;
    }
    fn set_read_time(&mut self, value: f64) {
        self.state.read_time = value;
    }
    fn set_last_hour_set(&mut self, value: bool) {
        self.state.last_hour_set = value;
    }
    fn interpret_record(
        &mut self,
        cursor: &mut RawEpwCursorOwner,
        read: &RawEpwLineRead,
    ) -> Result<(), RawDayFailure> {
        cursor.interpret(read)
    }
}
impl<C: RawDayReadCallbacks> SourceRawDayReadCallbacks for LegacyDayReadCallbacks<'_, C> {
    type Error = String;
    fn first_day_selected(
        &mut self,
        slot: &RawWeatherSlot,
        _header: &mut RawEpwHeaderState,
        _arguments: &mut RawEpwOutputs,
    ) -> Result<(), String> {
        self.callbacks.first_day_selected(self.state, slot)
    }
    fn clear_source_interval(&mut self, hour: usize, interval: usize) -> Result<(), String> {
        self.callbacks.clear_source_interval(hour, interval)
    }
    fn parsed_hour(
        &mut self,
        hour: usize,
        slot: &RawWeatherSlot,
        _header: &mut RawEpwHeaderState,
        _arguments: &mut RawEpwOutputs,
    ) -> Result<(), String> {
        self.callbacks.parsed_hour(self.state, hour, slot)
    }
    fn complete_day(
        &mut self,
        day: &RawWeatherDay,
        _header: &mut RawEpwHeaderState,
    ) -> Result<(), String> {
        self.callbacks.complete_day(self.state, day)
    }
}
impl RawEpwCursorOwner {
    /// Same bounded raw loop, with actual state access and mandatory real children.
    pub(crate) fn read_day_with_live_source_state<C: SourceRawDayReadCallbacks>(
        &mut self,
        selection: RawDayReadSelection,
        mode: RawDayReadMode,
        backspace_after_read: bool,
        callbacks: &mut C,
    ) -> Result<RawWeatherDay, SourceRawDayReadFailure<C::Error>> {
        self.admit_day_read(selection)?;
        if mode == RawDayReadMode::FirstDay {
            let selected = self.search_first_day(selection, callbacks)?;
            callbacks
                .first_day_selected(&selected, &mut self.header, &mut self.arguments)
                .map_err(SourceRawDayReadFailure::Callback)?;
        }
        let mut hours = Vec::with_capacity(24);
        for hour in 1..=24 {
            callbacks
                .clear_source_interval(hour, 1)
                .map_err(SourceRawDayReadFailure::Callback)?;
            let mut read = self.input.read_line()?;
            if !read.read_good {
                read.data.clear();
            }
            if read.data.is_empty() {
                if hour == 1 {
                    read.eof = true;
                }
                read.read_good = false;
            }
            if read.read_good {
                callbacks.interpret_record(self, &read)?;
            } else if read.eof && self.header.number_data_periods == 1 {
                if self.header.data_periods.values[0].number_days >= selection.days_in_year {
                    self.rewind_and_skip_header()?;
                    read.update(self.input.read_line()?);
                    // Source calls Interpret before testing this replacement read.
                    callbacks.interpret_record(self, &read)?;
                } else {
                    return Err(
                        RawDayFailure::SourceFatal(RawDaySourceFatal::PartialPeriodEof).into(),
                    );
                }
            } else {
                return Err(RawDayFailure::SourceFatal(RawDaySourceFatal::UnexpectedRead).into());
            }
            if self.arguments.dates[3] != hour as i32 {
                return Err(
                    RawDayFailure::SourceFatal(RawDaySourceFatal::UnexpectedHour {
                        expected: hour as i32,
                        actual: self.arguments.dates[3],
                    })
                    .into(),
                );
            }
            let slot = self.slot(&read);
            // Original WeatherManager.cc2826: only a disallowed Feb29 takes
            // the unported skip/retry branch. Read the live calendar owner;
            // header leap permission and CurrentYearIsLeapYear are distinct.
            if hour == 1
                && slot.raw.dates[1..3] == [2, 29]
                && (!callbacks.current_year_is_leap_year()? || !self.header.allows_leap_years)
            {
                return Err(RawDayFailure::OutsideBoundedDomain("February 29 daily retry").into());
            }
            callbacks
                .parsed_hour(hour, &slot, &mut self.header, &mut self.arguments)
                .map_err(SourceRawDayReadFailure::Callback)?;
            hours.push(slot);
        }
        if backspace_after_read {
            self.backspace_record()?;
        }
        let day = RawWeatherDay {
            hours,
            final_stream: self.stream_state(),
        };
        callbacks
            .complete_day(&day, &mut self.header)
            .map_err(SourceRawDayReadFailure::Callback)?;
        Ok(day)
    }
}
