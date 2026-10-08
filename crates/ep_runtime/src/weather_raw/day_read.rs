//! Source-order raw day extraction, separate from the processed weather carrier.

use super::cursor::RawEpwCursorOwner;
use super::{RawEpwHeaderError, RawEpwOutputs, RawEpwStreamState, RawRecordFailure};
use std::fmt::{Display, Formatter};

/// Actual supplied-byte range and attempted getline that produced a raw slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawReadProvenance {
    /// Actual byte cursor before the successful extraction.
    pub start_byte: usize,
    /// Actual byte cursor after extraction, including consumed LF.
    pub end_byte: usize,
    /// Cumulative reads on this stream, including header reads and rewinds.
    pub line_read_attempt: usize,
}

/// One parsed raw row. No physical processing is represented by this value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawWeatherSlot {
    /// Original parser arguments before physical processing.
    pub raw: RawEpwOutputs,
    /// Supplied-byte observation, without an inferred native record index.
    pub provenance: RawReadProvenance,
}

/// A successfully read day: exactly 24 actual rows in source consumption order.
#[derive(Clone, Debug, PartialEq)]
pub struct RawWeatherDay {
    /// Twenty-four successfully parsed hourly rows, in their read order.
    pub hours: Vec<RawWeatherSlot>,
    /// Actual stream after the optional one-record backspace.
    pub final_stream: RawEpwStreamState,
}

/// Caller-owned clock state mutated by search and the processed-day callback.
/// Default zeros initialize this Rust storage; callers may provide live values.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct RawDayReadState {
    /// CurDayOfWeek, including holiday/design-day values greater than seven.
    pub current_day_of_week: i32,
    /// ReadEPlusWeatherCurTime; first-day search writes one over file intervals.
    pub read_time: f64,
    /// LastHourSet; search clears it and upstream interpolation may set it.
    pub last_hour_set: bool,
}

/// RunPeriod selection controls; civil schedule identity is kept by the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawDayReadSelection {
    /// Declared environment start month.
    pub start_month: i32,
    /// Declared environment start day of month.
    pub start_day: i32,
    /// Declared year, retained separately while the non-actual search ignores it.
    pub start_year: i32,
    /// Actual-year matching is outside this bounded non-actual reader.
    pub match_year: bool,
    /// Original NumDaysInYear, used with declared DataPeriod.NumDays at EOF.
    pub days_in_year: i32,
}

/// DayToRead==1 searches the current cursor; later days consume it directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawDayReadMode {
    /// Source DayToRead equals one, including explicit replay calls.
    FirstDay,
    /// Source DayToRead is later than one.
    NextDay,
}

/// Source termination routes owned by the raw cursor/day extraction boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawDaySourceFatal {
    /// Failed read while searching, including the defined second-EOF route.
    SearchRead,
    /// EOF cannot wrap because the declared period is shorter than a year.
    PartialPeriodEof,
    /// Failed middle-of-day extraction outside the accepted EOF route.
    UnexpectedRead,
    /// Parsed WHour differs from the current source loop hour.
    UnexpectedHour {
        /// Current one-based source loop hour.
        expected: i32,
        /// Actual raw WHour written by the parser.
        actual: i32,
    },
}

/// Failure preserves cursor, counters, raw argument writes and prior callbacks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RawDayFailure {
    /// Fatal route belonging to this source day reader.
    SourceFatal(RawDaySourceFatal),
    /// Actual header/byte-stream failure.
    Header(RawEpwHeaderError),
    /// Actual raw-record failure, preserving partial reference writes.
    Record(RawRecordFailure),
    /// Unsupported or undefined original input, distinct from source termination.
    OutsideBoundedDomain(&'static str),
    /// Actual failure returned by a separately owned processed-weather callback.
    Upstream(String),
}

impl RawDayFailure {
    /// Whether the observed route is an original source FatalError route.
    pub fn is_source_fatal(&self) -> bool {
        match self {
            Self::SourceFatal(_) | Self::Header(RawEpwHeaderError::SourceFatal(_)) => true,
            Self::Record(error) => !matches!(error, RawRecordFailure::OutsideBoundedDomain(_)),
            _ => false,
        }
    }
}

impl Display for RawDayFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceFatal(error) => write!(formatter, "EPW day source termination: {error:?}"),
            Self::Header(error) => write!(formatter, "EPW day header operation: {error:?}"),
            Self::Record(error) => Display::fmt(error, formatter),
            Self::OutsideBoundedDomain(reason) => {
                write!(formatter, "EPW day outside bounded domain: {reason}")
            }
            Self::Upstream(error) => formatter.write_str(error),
        }
    }
}

impl std::error::Error for RawDayFailure {}
impl From<RawEpwHeaderError> for RawDayFailure {
    fn from(error: RawEpwHeaderError) -> Self {
        Self::Header(error)
    }
}

/// In-call boundaries for the actual processed Tomorrow owner.
///
/// `clear_source_interval` must clear only timestep one for hourly EPW, not an
/// entire day/hour. `parsed_hour` owns processing and first-hour calendar writes.
/// `complete_day` owns interpolation and previous-hour caches. No callback is
/// issued for skipped decoy rows; no callback after the failing source operation
/// runs. Defaults permit a raw-only consumer without fabricating processed data.
pub trait RawDayReadCallbacks {
    /// Source calendar preparation after a match/backspace, before day reads.
    fn first_day_selected(
        &mut self,
        _state: &mut RawDayReadState,
        _slot: &RawWeatherSlot,
    ) -> Result<(), String> {
        Ok(())
    }
    /// Source reset of the current Tomorrow interval before attempting getline.
    fn clear_source_interval(&mut self, _hour: usize, _interval: usize) -> Result<(), String> {
        Ok(())
    }
    /// Process an actually parsed/admitted raw hour and retain live clock writes.
    fn parsed_hour(
        &mut self,
        _state: &mut RawDayReadState,
        _hour: usize,
        _slot: &RawWeatherSlot,
    ) -> Result<(), String> {
        Ok(())
    }
    /// Upstream completion after 24 rows and any requested one-record backspace.
    fn complete_day(
        &mut self,
        _state: &mut RawDayReadState,
        _day: &RawWeatherDay,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl RawEpwCursorOwner {
    /// Execute the bounded hourly, single-period, non-actual-weather day reader.
    /// There is no eager parse, selected-day cache, or reconstructed counter delta.
    pub fn read_day(
        &mut self,
        selection: RawDayReadSelection,
        state: &mut RawDayReadState,
        mode: RawDayReadMode,
        backspace_after_read: bool,
        callbacks: &mut impl RawDayReadCallbacks,
    ) -> Result<RawWeatherDay, RawDayFailure> {
        self.admit_day_read(selection)?;
        if mode == RawDayReadMode::FirstDay {
            let selected = self.search_first_day(selection, state)?;
            callbacks
                .first_day_selected(state, &selected)
                .map_err(RawDayFailure::Upstream)?;
        }
        let mut hours = Vec::with_capacity(24);
        for hour in 1..=24 {
            callbacks
                .clear_source_interval(hour, 1)
                .map_err(RawDayFailure::Upstream)?;
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
                self.interpret(&read)?;
            } else if read.eof && self.header.number_data_periods == 1 {
                if self.header.data_periods.values[0].number_days >= selection.days_in_year {
                    self.rewind_and_skip_header()?;
                    read.update(self.input.read_line()?);
                    // Source calls Interpret before testing this replacement read.
                    self.interpret(&read)?;
                } else {
                    return Err(RawDayFailure::SourceFatal(
                        RawDaySourceFatal::PartialPeriodEof,
                    ));
                }
            } else {
                return Err(RawDayFailure::SourceFatal(
                    RawDaySourceFatal::UnexpectedRead,
                ));
            }
            if self.arguments.dates[3] != hour as i32 {
                return Err(RawDayFailure::SourceFatal(
                    RawDaySourceFatal::UnexpectedHour {
                        expected: hour as i32,
                        actual: self.arguments.dates[3],
                    },
                ));
            }
            let slot = self.slot(&read);
            // Feb29 retry/physical processing is outside this frozen nonleap
            // domain. Do not turn that native skip branch into a source fatal.
            if hour == 1 && slot.raw.dates[1..3] == [2, 29] {
                return Err(RawDayFailure::OutsideBoundedDomain(
                    "February 29 daily retry",
                ));
            }
            callbacks
                .parsed_hour(state, hour, &slot)
                .map_err(RawDayFailure::Upstream)?;
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
            .complete_day(state, &day)
            .map_err(RawDayFailure::Upstream)?;
        Ok(day)
    }
}
