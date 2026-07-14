//! Source-order EPW positioning and today/tomorrow day-buffer materialization.

use super::{EpwDataPeriod, EpwRecord, EpwWeatherFile};
use crate::time_axis::TimeAxis;
use std::fmt::{Display, Formatter};

const HOURS_PER_DAY: usize = 24;

/// One `UpdateWeatherData` commit followed by the optional next-day prefetch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EpwWeatherDayBufferTransition {
    /// One-based simulation day committed from tomorrow into today.
    pub day_of_sim: usize,
    /// Source record index for today's hour 1.
    pub today_source_record_start: usize,
    /// Source record index left in the tomorrow buffer after the commit.
    ///
    /// On the last simulation day this remains equal to today's start because
    /// EnergyPlus does not prefetch beyond the environment boundary.
    pub tomorrow_source_record_start: usize,
    /// Whether the transition read a new source day into tomorrow.
    pub prefetched_next_day: bool,
}

/// Weather records selected for one already-resolved weather environment.
#[derive(Clone, Debug, PartialEq)]
pub struct EpwEnvironmentWeather {
    /// Zero-based matching `DATA PERIODS` entry.
    pub data_period_index: usize,
    /// Literal RunPeriod start-date record found before leap-day filtering.
    pub source_start_record_index: usize,
    /// First effective day initially read into EnergyPlus' tomorrow buffer.
    pub initial_tomorrow_source_record_start: usize,
    /// Source indices corresponding one-to-one with `hourly_records`.
    pub selected_source_record_indices: Vec<usize>,
    /// Raw February 29 day starts consumed but not materialized.
    pub skipped_february_29_source_record_starts: Vec<usize>,
    /// Eager source-order representation of daily today/tomorrow transitions.
    pub day_buffer_transitions: Vec<EpwWeatherDayBufferTransition>,
    hourly_records: Vec<EpwRecord>,
}

impl EpwEnvironmentWeather {
    /// Returns the dense weather-effective hourly stream in time-axis order.
    #[must_use]
    pub fn hourly_records(&self) -> &[EpwRecord] {
        &self.hourly_records
    }
}

/// Error returned while positioning or traversing EPW records for an environment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EpwEnvironmentWeatherError {
    /// The supplied axis was not built with EPW calendar metadata.
    WeatherCalendarMissing {
        /// RunPeriod name.
        run_period_name: String,
    },
    /// The resolved weather environment contains no effective simulation day.
    EmptyEnvironment {
        /// RunPeriod name.
        run_period_name: String,
    },
    /// The hourly compatibility path does not yet support subhourly EPW rows.
    RecordsPerHourUnsupported {
        /// Declared EPW records per hour.
        records_per_hour: u32,
    },
    /// No one data period covers both RunPeriod endpoints.
    RunPeriodOutsideDataPeriods {
        /// RunPeriod name.
        run_period_name: String,
        /// Requested start month.
        start_month: u32,
        /// Requested start day of month.
        start_day: u32,
        /// Requested end month.
        end_month: u32,
        /// Requested end day of month.
        end_day: u32,
    },
    /// The literal RunPeriod start date was not present in the EPW rows.
    StartRecordNotFound {
        /// RunPeriod name.
        run_period_name: String,
        /// Literal start month.
        month: u32,
        /// Literal start day of month.
        day: u32,
    },
    /// The resolved hourly axis did not contain exactly 24 samples per day.
    HourlyAxisShapeMismatch {
        /// RunPeriod name.
        run_period_name: String,
        /// Sample count implied by the weather calendar.
        expected_samples: usize,
        /// Actual number of hourly axis points.
        actual_samples: usize,
    },
    /// A source day ended before all 24 hourly rows were available.
    IncompleteSourceDay {
        /// Zero-based source record at which the day begins.
        source_record_start: usize,
        /// Source records available from that position through EOF.
        available_records: usize,
    },
    /// A source day contained a date change before hour 24.
    SourceDateChangedWithinDay {
        /// Zero-based source record containing the unexpected date.
        source_record_index: usize,
        /// Month carried by the day's first record.
        expected_month: u32,
        /// Day carried by the day's first record.
        expected_day: u32,
        /// Unexpected record month.
        actual_month: u32,
        /// Unexpected record day of month.
        actual_day: u32,
    },
    /// A source day did not contain hour-ending rows 1 through 24 in order.
    SourceHourOutOfOrder {
        /// Zero-based source record containing the unexpected hour.
        source_record_index: usize,
        /// Required hour-ending value.
        expected_hour: u32,
        /// Observed hour-ending value.
        actual_hour: u32,
    },
    /// The next source day did not match the next weather-effective axis day.
    SourceDayDoesNotMatchAxis {
        /// One-based simulation day.
        day_of_sim: usize,
        /// Zero-based source record at which the day begins.
        source_record_start: usize,
        /// Axis month.
        expected_month: u32,
        /// Axis day of month.
        expected_day: u32,
        /// Source month.
        actual_month: u32,
        /// Source day of month.
        actual_day: u32,
    },
    /// Every reachable source day was filtered without finding an effective day.
    NoUsableSourceDay {
        /// One-based simulation day being materialized.
        day_of_sim: usize,
        /// Number of source-day positions inspected before detecting the cycle.
        scanned_source_days: usize,
    },
    /// EOF could not wrap because the file is not one full-cycle data period.
    SourceEndedBeforeEnvironment {
        /// Effective environment days already consumed.
        completed_days: usize,
        /// Effective environment days required by the axis.
        required_days: usize,
    },
}

impl Display for EpwEnvironmentWeatherError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WeatherCalendarMissing { run_period_name } => write!(
                formatter,
                "run period {run_period_name} has no metadata-aware weather calendar"
            ),
            Self::EmptyEnvironment { run_period_name } => write!(
                formatter,
                "run period {run_period_name} has no weather-effective simulation day"
            ),
            Self::RecordsPerHourUnsupported { records_per_hour } => write!(
                formatter,
                "EPW records-per-hour value {records_per_hour} is not supported by the hourly weather traversal"
            ),
            Self::RunPeriodOutsideDataPeriods {
                run_period_name,
                start_month,
                start_day,
                end_month,
                end_day,
            } => write!(
                formatter,
                "run period {run_period_name} ({start_month}/{start_day} through {end_month}/{end_day}) is outside the EPW DATA PERIODS ranges"
            ),
            Self::StartRecordNotFound {
                run_period_name,
                month,
                day,
            } => write!(
                formatter,
                "run period {run_period_name} start date {month}/{day} was not found in the EPW records"
            ),
            Self::HourlyAxisShapeMismatch {
                run_period_name,
                expected_samples,
                actual_samples,
            } => write!(
                formatter,
                "run period {run_period_name} expected {expected_samples} hourly samples but its axis contains {actual_samples}"
            ),
            Self::IncompleteSourceDay {
                source_record_start,
                available_records,
            } => write!(
                formatter,
                "EPW source day at record {source_record_start} has only {available_records} available hourly records"
            ),
            Self::SourceDateChangedWithinDay {
                source_record_index,
                expected_month,
                expected_day,
                actual_month,
                actual_day,
            } => write!(
                formatter,
                "EPW record {source_record_index} changed date within a source day: expected {expected_month}/{expected_day}, found {actual_month}/{actual_day}"
            ),
            Self::SourceHourOutOfOrder {
                source_record_index,
                expected_hour,
                actual_hour,
            } => write!(
                formatter,
                "EPW record {source_record_index} has hour {actual_hour}, expected {expected_hour}"
            ),
            Self::SourceDayDoesNotMatchAxis {
                day_of_sim,
                source_record_start,
                expected_month,
                expected_day,
                actual_month,
                actual_day,
            } => write!(
                formatter,
                "simulation day {day_of_sim} expects {expected_month}/{expected_day}, but EPW record {source_record_start} begins {actual_month}/{actual_day}"
            ),
            Self::NoUsableSourceDay {
                day_of_sim,
                scanned_source_days,
            } => write!(
                formatter,
                "simulation day {day_of_sim} found no usable EPW source day after scanning {scanned_source_days} source-day positions"
            ),
            Self::SourceEndedBeforeEnvironment {
                completed_days,
                required_days,
            } => write!(
                formatter,
                "EPW ended after {completed_days} environment days; {required_days} days were required and the DATA PERIODS header does not permit a full-cycle rewind"
            ),
        }
    }
}

impl std::error::Error for EpwEnvironmentWeatherError {}

/// Selects EPW rows in EnergyPlus `ReadEPlusWeatherForDay` source order.
///
/// The start search uses the literal Gregorian RunPeriod boundary retained by
/// the metadata-aware axis. Leap-day filtering happens only after that search,
/// which preserves EnergyPlus' fatal behavior when a February 29 start row is
/// absent under a no-leap EPW policy.
pub fn select_epw_environment_weather(
    weather_file: &EpwWeatherFile,
    time_axis: &TimeAxis,
) -> Result<EpwEnvironmentWeather, EpwEnvironmentWeatherError> {
    let weather_calendar = time_axis.weather_calendar.as_ref().ok_or_else(|| {
        EpwEnvironmentWeatherError::WeatherCalendarMissing {
            run_period_name: time_axis.run_period_name.clone(),
        }
    })?;
    if weather_file.data_periods.records_per_hour != 1 {
        return Err(EpwEnvironmentWeatherError::RecordsPerHourUnsupported {
            records_per_hour: weather_file.data_periods.records_per_hour,
        });
    }

    let gregorian = &weather_calendar.gregorian;
    let data_period_index = weather_file
        .data_periods
        .periods
        .iter()
        .position(|period| data_period_covers_calendar(period, weather_calendar))
        .ok_or_else(|| EpwEnvironmentWeatherError::RunPeriodOutsideDataPeriods {
            run_period_name: time_axis.run_period_name.clone(),
            start_month: gregorian.start_month,
            start_day: gregorian.start_day_of_month,
            end_month: gregorian.end_month,
            end_day: gregorian.end_day_of_month,
        })?;
    let selected_period = &weather_file.data_periods.periods[data_period_index];
    let allow_eof_rewind = weather_file.data_periods.periods.len() == 1
        && data_period_is_full_cycle(
            selected_period,
            weather_calendar.start_year_is_weather_effective_leap_year,
        );

    let source_start_record_index = weather_file
        .records
        .iter()
        .position(|record| {
            record.month == gregorian.start_month
                && record.day == gregorian.start_day_of_month
                && record.hour == 1
        })
        .ok_or_else(|| EpwEnvironmentWeatherError::StartRecordNotFound {
            run_period_name: time_axis.run_period_name.clone(),
            month: gregorian.start_month,
            day: gregorian.start_day_of_month,
        })?;

    let expected_samples = weather_calendar.total_days.saturating_mul(HOURS_PER_DAY);
    if weather_calendar.total_days == 0 {
        return Err(EpwEnvironmentWeatherError::EmptyEnvironment {
            run_period_name: time_axis.run_period_name.clone(),
        });
    }
    if time_axis.points.len() != expected_samples {
        return Err(EpwEnvironmentWeatherError::HourlyAxisShapeMismatch {
            run_period_name: time_axis.run_period_name.clone(),
            expected_samples,
            actual_samples: time_axis.points.len(),
        });
    }

    let mut cursor = source_start_record_index;
    let mut selected_source_record_indices = Vec::with_capacity(expected_samples);
    let mut hourly_records = Vec::with_capacity(expected_samples);
    let mut selected_day_starts = Vec::with_capacity(weather_calendar.total_days);
    let mut skipped_february_29_source_record_starts = Vec::new();
    let weather_effective_leap_year = weather_calendar.start_year_is_weather_effective_leap_year;
    let maximum_source_days_to_scan = weather_file.records.len().div_ceil(HOURS_PER_DAY);

    for day_index in 0..weather_calendar.total_days {
        let mut scanned_source_days = 0;
        loop {
            if scanned_source_days >= maximum_source_days_to_scan {
                return Err(EpwEnvironmentWeatherError::NoUsableSourceDay {
                    day_of_sim: day_index + 1,
                    scanned_source_days,
                });
            }
            validate_source_day(&weather_file.records, cursor)?;
            scanned_source_days += 1;
            let first_record = weather_file.records[cursor];
            if first_record.month == 2 && first_record.day == 29 && !weather_effective_leap_year {
                skipped_february_29_source_record_starts.push(cursor);
                cursor = advance_source_day(
                    cursor,
                    weather_file.records.len(),
                    allow_eof_rewind,
                    day_index,
                    weather_calendar.total_days,
                )?;
                continue;
            }
            break;
        }

        let expected = &time_axis.points[day_index * HOURS_PER_DAY];
        let first_record = weather_file.records[cursor];
        if first_record.month != expected.month || first_record.day != expected.day_of_month {
            return Err(EpwEnvironmentWeatherError::SourceDayDoesNotMatchAxis {
                day_of_sim: day_index + 1,
                source_record_start: cursor,
                expected_month: expected.month,
                expected_day: expected.day_of_month,
                actual_month: first_record.month,
                actual_day: first_record.day,
            });
        }

        selected_day_starts.push(cursor);
        for source_record_index in cursor..cursor + HOURS_PER_DAY {
            selected_source_record_indices.push(source_record_index);
            hourly_records.push(weather_file.records[source_record_index]);
        }
        if day_index + 1 < weather_calendar.total_days {
            cursor = advance_source_day(
                cursor,
                weather_file.records.len(),
                allow_eof_rewind,
                day_index + 1,
                weather_calendar.total_days,
            )?;
        }
    }

    let initial_tomorrow_source_record_start =
        selected_day_starts.first().copied().ok_or_else(|| {
            EpwEnvironmentWeatherError::EmptyEnvironment {
                run_period_name: time_axis.run_period_name.clone(),
            }
        })?;
    let day_buffer_transitions = selected_day_starts
        .iter()
        .enumerate()
        .map(|(day_index, today_source_record_start)| {
            let next = selected_day_starts.get(day_index + 1).copied();
            EpwWeatherDayBufferTransition {
                day_of_sim: day_index + 1,
                today_source_record_start: *today_source_record_start,
                tomorrow_source_record_start: next.unwrap_or(*today_source_record_start),
                prefetched_next_day: next.is_some(),
            }
        })
        .collect();

    Ok(EpwEnvironmentWeather {
        data_period_index,
        source_start_record_index,
        initial_tomorrow_source_record_start,
        selected_source_record_indices,
        skipped_february_29_source_record_starts,
        day_buffer_transitions,
        hourly_records,
    })
}

fn validate_source_day(
    records: &[EpwRecord],
    source_record_start: usize,
) -> Result<(), EpwEnvironmentWeatherError> {
    let available_records = records.len().saturating_sub(source_record_start);
    if available_records < HOURS_PER_DAY {
        return Err(EpwEnvironmentWeatherError::IncompleteSourceDay {
            source_record_start,
            available_records,
        });
    }
    let first = records[source_record_start];
    for offset in 0..HOURS_PER_DAY {
        let source_record_index = source_record_start + offset;
        let record = records[source_record_index];
        if record.month != first.month || record.day != first.day {
            return Err(EpwEnvironmentWeatherError::SourceDateChangedWithinDay {
                source_record_index,
                expected_month: first.month,
                expected_day: first.day,
                actual_month: record.month,
                actual_day: record.day,
            });
        }
        let expected_hour = u32::try_from(offset + 1).unwrap_or(24);
        if record.hour != expected_hour {
            return Err(EpwEnvironmentWeatherError::SourceHourOutOfOrder {
                source_record_index,
                expected_hour,
                actual_hour: record.hour,
            });
        }
    }
    Ok(())
}

fn advance_source_day(
    source_record_start: usize,
    record_count: usize,
    allow_eof_rewind: bool,
    completed_days: usize,
    required_days: usize,
) -> Result<usize, EpwEnvironmentWeatherError> {
    let next = source_record_start + HOURS_PER_DAY;
    if next < record_count {
        Ok(next)
    } else if next == record_count && allow_eof_rewind {
        Ok(0)
    } else {
        Err(EpwEnvironmentWeatherError::SourceEndedBeforeEnvironment {
            completed_days,
            required_days,
        })
    }
}

fn data_period_covers_calendar(
    period: &EpwDataPeriod,
    calendar: &crate::time_axis::ResolvedWeatherEnvironmentCalendar,
) -> bool {
    let leap_year = calendar.start_year_is_weather_effective_leap_year;
    let Some(period_start) =
        weather_ordinal(period.start_date.month, period.start_date.day, leap_year)
    else {
        return false;
    };
    let Some(period_end) = weather_ordinal(period.end_date.month, period.end_date.day, leap_year)
    else {
        return false;
    };
    let Some(run_start) = weather_ordinal(
        calendar.gregorian.start_month,
        calendar.gregorian.start_day_of_month,
        leap_year,
    ) else {
        return false;
    };
    let Some(run_end) = weather_ordinal(
        calendar.gregorian.end_month,
        calendar.gregorian.end_day_of_month,
        leap_year,
    ) else {
        return false;
    };
    ordinal_is_between(run_start, period_start, period_end)
        && ordinal_is_between(run_end, period_start, period_end)
}

fn data_period_is_full_cycle(period: &EpwDataPeriod, leap_year: bool) -> bool {
    let Some(start) = weather_ordinal(period.start_date.month, period.start_date.day, leap_year)
    else {
        return false;
    };
    let Some(end) = weather_ordinal(period.end_date.month, period.end_date.day, leap_year) else {
        return false;
    };
    let days_in_year = if leap_year { 366 } else { 365 };
    let span = if start <= end {
        end - start + 1
    } else {
        days_in_year - start + 1 + end
    };
    span >= days_in_year
}

fn ordinal_is_between(value: u32, start: u32, end: u32) -> bool {
    if start <= end {
        (start..=end).contains(&value)
    } else {
        value >= start || value <= end
    }
}

fn weather_ordinal(month: u32, day: u32, leap_year: bool) -> Option<u32> {
    if month == 2 && day == 29 && !leap_year {
        return Some(60);
    }
    if !(1..=12).contains(&month) || day == 0 {
        return None;
    }
    let month_lengths = [
        31,
        if leap_year { 29 } else { 28 },
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
    let month_index = usize::try_from(month - 1).ok()?;
    if day > month_lengths[month_index] {
        return None;
    }
    Some(month_lengths[..month_index].iter().sum::<u32>() + day)
}

#[cfg(test)]
#[path = "weather_environment_tests.rs"]
mod tests;
