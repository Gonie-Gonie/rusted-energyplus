//! Run-period time-axis and calendar helpers.

use ep_model::{
    FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId, TypedModel,
};
use std::fmt::{Display, Formatter};

pub(crate) const DEFAULT_RUN_PERIOD_YEAR: u32 = 2013;

/// One hourly timestamp aligned to EnergyPlus run-period reporting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePoint {
    /// Zero-based sample index.
    pub sample_index: usize,
    /// Calendar year used for date arithmetic.
    pub year: u32,
    /// Month number, 1-12.
    pub month: u32,
    /// Day of month.
    pub day_of_month: u32,
    /// EnergyPlus-style hour ending, 1-24.
    pub hour: u32,
}

/// Zone timestep settings attached to a shared run-period time axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneTimestepAxis {
    /// Number of zone timesteps in each weather/output hour.
    pub timesteps_per_hour: u32,
    /// Nominal zone timestep duration in seconds.
    pub timestep_seconds: f64,
}

/// System timestep settings attached to a shared run-period time axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SystemTimestepAxis {
    /// Nominal system timestep duration before adaptive shortening.
    pub nominal_timestep_seconds: f64,
    /// Explicit placeholder for later variable system timestep support.
    pub variable_system_timestep_support: &'static str,
    /// Whether `ShortenTimeStepSys` state is represented in runtime state.
    pub shorten_timestep_sys_state: bool,
    /// Whether `UseZoneTimeStepHistory` state is represented in runtime state.
    pub use_zone_timestep_history_state: bool,
}

/// Reported sample partitioning for one shared time axis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeAxisSamplePartitions {
    /// Whether warmup samples are part of the reported output sample axis.
    pub warmup_reported_samples: usize,
    /// Number of reported run-period hourly samples.
    pub run_period_reported_samples: usize,
    /// Number of design-day reported samples in this axis.
    pub design_day_reported_samples: usize,
}

/// Hourly time axis for one run period.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeAxis {
    /// Run period name.
    pub run_period_name: String,
    /// First-hour weather interpolation policy selected by the run period.
    pub first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    /// Zone timestep profile used by weather, schedules, output, and report sampling.
    pub zone_timestep: ZoneTimestepAxis,
    /// System timestep profile used by adaptive zone-air correction state.
    pub system_timestep: SystemTimestepAxis,
    /// Warmup/run-period/design-day sample partitioning.
    pub sample_partitions: TimeAxisSamplePartitions,
    /// Hourly samples in output order.
    pub points: Vec<TimePoint>,
}

impl TimeAxis {
    /// Returns the number of hourly samples.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.points.len()
    }
}

/// Error returned while building a run-period time axis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeAxisError {
    /// A run-period date was invalid.
    InvalidDate {
        /// Run period name.
        run_period_name: String,
        /// Field group, such as begin or end.
        field: &'static str,
        /// Calendar year.
        year: u32,
        /// Month number.
        month: u32,
        /// Day of month.
        day_of_month: u32,
    },
    /// The end date came before the begin date.
    InvalidRange {
        /// Run period name.
        run_period_name: String,
    },
}

impl Display for TimeAxisError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDate {
                run_period_name,
                field,
                year,
                month,
                day_of_month,
            } => write!(
                formatter,
                "run period {run_period_name} has invalid {field} date {year:04}-{month:02}-{day_of_month:02}"
            ),
            Self::InvalidRange { run_period_name } => {
                write!(
                    formatter,
                    "run period {run_period_name} ends before it begins"
                )
            }
        }
    }
}

impl std::error::Error for TimeAxisError {}

/// Builds the first hourly time axis from the model `RunPeriod` list.
///
/// If no `RunPeriod` is present, a one-day default axis is returned so early
/// diagnostic runtime paths remain explicit and deterministic.
pub fn build_hourly_time_axis(model: &TypedModel) -> Result<TimeAxis, TimeAxisError> {
    let fallback;
    let run_period = if let Some(run_period) = model.run_periods.first() {
        run_period
    } else {
        fallback = default_run_period();
        &fallback
    };

    build_hourly_time_axis_for_run_period_with_zone_timesteps(
        run_period,
        model.timestep.number_of_timesteps_per_hour.max(1),
    )
}

pub(crate) fn run_period_first_hour_interpolation_starting_values(
    model: &TypedModel,
) -> FirstHourInterpolationStartingValues {
    model
        .run_periods
        .first()
        .map(|run_period| run_period.first_hour_interpolation_starting_values)
        .unwrap_or_default()
}

/// Builds an hourly time axis for one run period.
pub fn build_hourly_time_axis_for_run_period(
    run_period: &RunPeriod,
) -> Result<TimeAxis, TimeAxisError> {
    build_hourly_time_axis_for_run_period_with_zone_timesteps(run_period, 1)
}

/// Builds an hourly time axis for one run period and an explicit zone timestep count.
pub fn build_hourly_time_axis_for_run_period_with_zone_timesteps(
    run_period: &RunPeriod,
    zone_timesteps_per_hour: u32,
) -> Result<TimeAxis, TimeAxisError> {
    let begin_year = run_period
        .begin_year
        .or(run_period.end_year)
        .unwrap_or(DEFAULT_RUN_PERIOD_YEAR);
    let end_year = run_period
        .end_year
        .or(run_period.begin_year)
        .unwrap_or(begin_year);
    let begin = Date {
        year: begin_year,
        month: run_period.begin_month,
        day_of_month: run_period.begin_day_of_month,
    };
    let end = Date {
        year: end_year,
        month: run_period.end_month,
        day_of_month: run_period.end_day_of_month,
    };

    let begin_ordinal = date_ordinal(begin).ok_or_else(|| TimeAxisError::InvalidDate {
        run_period_name: run_period.name.0.clone(),
        field: "begin",
        year: begin.year,
        month: begin.month,
        day_of_month: begin.day_of_month,
    })?;
    let end_ordinal = date_ordinal(end).ok_or_else(|| TimeAxisError::InvalidDate {
        run_period_name: run_period.name.0.clone(),
        field: "end",
        year: end.year,
        month: end.month,
        day_of_month: end.day_of_month,
    })?;
    if end_ordinal < begin_ordinal {
        return Err(TimeAxisError::InvalidRange {
            run_period_name: run_period.name.0.clone(),
        });
    }

    let mut points = Vec::new();
    let mut date = begin;
    let mut ordinal = begin_ordinal;
    while ordinal <= end_ordinal {
        for hour in 1..=24 {
            points.push(TimePoint {
                sample_index: points.len(),
                year: date.year,
                month: date.month,
                day_of_month: date.day_of_month,
                hour,
            });
        }
        if ordinal == end_ordinal {
            break;
        }
        date = next_day(date);
        ordinal += 1;
    }

    let zone_timesteps_per_hour = zone_timesteps_per_hour.max(1);
    let zone_timestep_seconds = 3600.0 / f64::from(zone_timesteps_per_hour);
    Ok(TimeAxis {
        run_period_name: run_period.name.0.clone(),
        first_hour_interpolation_starting_values: run_period
            .first_hour_interpolation_starting_values,
        zone_timestep: ZoneTimestepAxis {
            timesteps_per_hour: zone_timesteps_per_hour,
            timestep_seconds: zone_timestep_seconds,
        },
        system_timestep: SystemTimestepAxis {
            nominal_timestep_seconds: zone_timestep_seconds,
            variable_system_timestep_support: "placeholder-state-backed",
            shorten_timestep_sys_state: true,
            use_zone_timestep_history_state: true,
        },
        sample_partitions: TimeAxisSamplePartitions {
            warmup_reported_samples: 0,
            run_period_reported_samples: points.len(),
            design_day_reported_samples: 0,
        },
        points,
    })
}

#[derive(Clone, Copy)]
pub(crate) struct Date {
    pub(crate) year: u32,
    pub(crate) month: u32,
    pub(crate) day_of_month: u32,
}

fn default_run_period() -> RunPeriod {
    RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new("Default Run Period"),
        begin_month: 1,
        begin_day_of_month: 1,
        begin_year: Some(DEFAULT_RUN_PERIOD_YEAR),
        end_month: 1,
        end_day_of_month: 1,
        end_year: Some(DEFAULT_RUN_PERIOD_YEAR),
        day_of_week_for_start_day: None,
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
    }
}

fn date_ordinal(date: Date) -> Option<i64> {
    let day_of_year = day_of_year(date.year, date.month, date.day_of_month)?;
    Some(days_before_year(date.year) + i64::from(day_of_year - 1))
}

fn days_before_year(year: u32) -> i64 {
    let previous = i64::from(year.saturating_sub(1));
    365 * previous + previous / 4 - previous / 100 + previous / 400
}

pub(crate) fn day_of_year(year: u32, month: u32, day_of_month: u32) -> Option<u32> {
    if !(1..=12).contains(&month) {
        return None;
    }
    let month_days = days_in_month(year, month);
    if day_of_month == 0 || day_of_month > month_days {
        return None;
    }
    let before_month = (1..month)
        .map(|value| days_in_month(year, value))
        .sum::<u32>();
    Some(before_month + day_of_month)
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

pub(crate) fn next_day(date: Date) -> Date {
    let month_days = days_in_month(date.year, date.month);
    if date.day_of_month < month_days {
        return Date {
            day_of_month: date.day_of_month + 1,
            ..date
        };
    }
    if date.month < 12 {
        return Date {
            month: date.month + 1,
            day_of_month: 1,
            ..date
        };
    }
    Date {
        year: date.year + 1,
        month: 1,
        day_of_month: 1,
    }
}
