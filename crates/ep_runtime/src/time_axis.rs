//! Run-period environment, calendar, and time-axis helpers.

use crate::weather::EpwCalendarMetadata;
use ep_model::{
    DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId,
    TypedModel,
};
use std::fmt::{Display, Formatter};

mod weather_calendar;
pub use weather_calendar::resolve_weather_environment_calendar;

pub(crate) const DEFAULT_RUN_PERIOD_YEAR: u32 = 2017;
const DEFAULT_LEAP_RUN_PERIOD_YEAR: u32 = 2012;
const EARLIEST_GREGORIAN_RUN_PERIOD_YEAR: u32 = 1583;

/// Kind of simulation environment represented by a canonical time axis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnvironmentKind {
    /// A weather-file run period (`RunPeriodWeather` in EnergyPlus).
    WeatherRunPeriod,
}

/// Schedule day type selected for a simulation day.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DayType {
    /// Sunday schedule day.
    Sunday,
    /// Monday schedule day.
    Monday,
    /// Tuesday schedule day.
    Tuesday,
    /// Wednesday schedule day.
    Wednesday,
    /// Thursday schedule day.
    Thursday,
    /// Friday schedule day.
    Friday,
    /// Saturday schedule day.
    Saturday,
    /// Holiday schedule day.
    Holiday,
    /// Summer design-day schedule day.
    SummerDesignDay,
    /// Winter design-day schedule day.
    WinterDesignDay,
    /// First custom schedule day.
    CustomDay1,
    /// Second custom schedule day.
    CustomDay2,
}

impl From<DayOfWeek> for DayType {
    fn from(value: DayOfWeek) -> Self {
        match value {
            DayOfWeek::Monday => Self::Monday,
            DayOfWeek::Tuesday => Self::Tuesday,
            DayOfWeek::Wednesday => Self::Wednesday,
            DayOfWeek::Thursday => Self::Thursday,
            DayOfWeek::Friday => Self::Friday,
            DayOfWeek::Saturday => Self::Saturday,
            DayOfWeek::Sunday => Self::Sunday,
        }
    }
}

impl DayType {
    /// Returns the EnergyPlus timestamp label for this schedule day type.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sunday => "Sunday",
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Holiday => "Holiday",
            Self::SummerDesignDay => "SummerDesignDay",
            Self::WinterDesignDay => "WinterDesignDay",
            Self::CustomDay1 => "CustomDay1",
            Self::CustomDay2 => "CustomDay2",
        }
    }
}

/// Calendar dates resolved from one `RunPeriod` using EnergyPlus year rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedRunPeriodCalendar {
    /// Run-period name.
    pub run_period_name: String,
    /// Resolved start year.
    pub start_year: u32,
    /// Start month, 1-12.
    pub start_month: u32,
    /// Start day of month.
    pub start_day_of_month: u32,
    /// Gregorian weekday for the resolved start date.
    pub start_day_of_week: DayOfWeek,
    /// Whether the resolved start year is a Gregorian leap year.
    pub start_year_is_leap_year: bool,
    /// Resolved end year.
    pub end_year: u32,
    /// End month, 1-12.
    pub end_month: u32,
    /// End day of month.
    pub end_day_of_month: u32,
    /// Whether the resolved end year is a Gregorian leap year.
    pub end_year_is_leap_year: bool,
    /// Inclusive number of Gregorian input days before EPW policy is applied.
    pub total_days: usize,
}

impl ResolvedRunPeriodCalendar {
    fn start_date(&self) -> Date {
        Date {
            year: self.start_year,
            month: self.start_month,
            day_of_month: self.start_day_of_month,
        }
    }
}

/// Calendar state after applying the EPW leap-year policy to one run period.
///
/// `gregorian` remains the input-date interpretation. `total_days` follows
/// weather-effective endpoint ordinals: with EPW leap years disabled, a
/// February 29 endpoint aliases March 1 rather than reducing the duration.
/// Thus a February-29-only period has Gregorian/skipped/effective counts 1/1/1.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedWeatherEnvironmentCalendar {
    /// Gregorian run-period interpretation before EPW policy is applied.
    pub gregorian: ResolvedRunPeriodCalendar,
    /// Whether the EPW header says that leap years are observed.
    pub weather_file_allows_leap_years: bool,
    /// Whether the start year is leap-shaped for weather processing.
    pub start_year_is_weather_effective_leap_year: bool,
    /// Whether the end year is leap-shaped for weather processing.
    pub end_year_is_weather_effective_leap_year: bool,
    /// Raw February 29 dates skipped or endpoint-aliased; not a duration subtraction.
    pub leap_days_skipped: usize,
    /// Inclusive number of weather-effective simulation days.
    pub total_days: usize,
}

/// One zone-timestep state in EnergyPlus environment-loop order.
#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentTimePoint {
    /// Zero-based point index within this environment.
    pub sample_index: usize,
    /// One-based index within the materialized Rust environment list.
    ///
    /// This is not yet the complete EnergyPlus `Envrn` ordinal because design
    /// and sizing environments are outside this checkpoint.
    pub environment_index: usize,
    /// Environment kind.
    pub environment_kind: EnvironmentKind,
    /// One-based day number within the environment.
    pub day_of_sim: usize,
    /// Gregorian calendar year.
    pub year: u32,
    /// Whether `year` is a proleptic-Gregorian leap year.
    ///
    /// EPW-dependent leap shape is stored separately in
    /// `weather_effective_year_is_leap_year`.
    pub gregorian_year_is_leap_year: bool,
    /// Leap shape used by this axis; Gregorian-only projections use Gregorian shape.
    pub weather_effective_year_is_leap_year: bool,
    /// EnergyPlus `LeapYearAdd` value for weather day-of-year calculations.
    pub leap_year_add: u32,
    /// Month number, 1-12.
    pub month: u32,
    /// Day of month.
    pub day_of_month: u32,
    /// Gregorian ordinal day in the current year, 1-365/366.
    pub gregorian_day_of_year: u32,
    /// Weather-effective ordinal day, 1-365/366.
    pub day_of_year: u32,
    /// Leap-shaped schedule ordinal, where March 1 is always day 61.
    pub schedule_day_of_year: u32,
    /// Gregorian weekday for the calendar date.
    pub gregorian_day_of_week: DayOfWeek,
    /// Simulation-effective weekday after weather-calendar skips.
    pub day_of_week: DayOfWeek,
    /// Schedule day type before special-day overrides.
    pub day_type: DayType,
    /// Daylight-saving state. Weather-file DST rules are not applied yet.
    pub dst: bool,
    /// Effective special schedule day type, or `None` before overrides.
    pub special_day_type: Option<DayType>,
    /// EnergyPlus hour ending, 1-24.
    pub hour: u32,
    /// One-based zone timestep within the hour.
    pub zone_timestep: u32,
    /// Start minute of the zone-timestep interval, relative to the hour.
    pub start_minute: f64,
    /// End minute of the zone-timestep interval, relative to the hour.
    pub end_minute: f64,
    /// End of the zone timestep as a fractional hour from the start of the day.
    pub current_time_hours: f64,
    /// One-based zone-timestep count since the environment began.
    pub simulation_timestep: usize,
    /// Whether this is the first zone timestep of the environment.
    pub begin_environment: bool,
    /// Whether this is the final zone timestep of the environment.
    pub end_environment: bool,
    /// Whether this is the first zone timestep of the day.
    pub begin_day: bool,
    /// Whether this is the final zone timestep of the day.
    pub end_day: bool,
    /// Whether this is the first zone timestep of the hour.
    pub begin_hour: bool,
    /// Whether this is the final zone timestep of the hour.
    pub end_hour: bool,
}

/// Canonical zone-timestep axis for one simulation environment.
#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentTimeAxis {
    /// One-based index within the materialized Rust environment list.
    ///
    /// This is not yet the complete EnergyPlus `Envrn` ordinal because design
    /// and sizing environments are outside this checkpoint.
    pub environment_index: usize,
    /// Environment name.
    pub environment_name: String,
    /// Environment kind.
    pub environment_kind: EnvironmentKind,
    /// Calendar resolved from the environment's run period.
    pub calendar: ResolvedRunPeriodCalendar,
    /// EPW policy applied to this axis, or `None` for a Gregorian-only projection.
    pub weather_calendar: Option<ResolvedWeatherEnvironmentCalendar>,
    /// First-hour weather interpolation policy selected by the run period.
    pub first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    /// Zone timestep profile.
    pub zone_timestep: ZoneTimestepAxis,
    /// System timestep profile before adaptive shortening.
    pub system_timestep: SystemTimestepAxis,
    /// Zone-timestep points in SimulationManager loop order.
    pub points: Vec<EnvironmentTimePoint>,
}

impl EnvironmentTimeAxis {
    /// Returns the number of zone-timestep states.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.points.len()
    }
}

/// One hourly timestamp aligned to EnergyPlus run-period reporting.
#[derive(Clone, Debug, PartialEq)]
pub struct TimePoint {
    /// Zero-based sample index.
    pub sample_index: usize,
    /// One-based day number within the environment.
    pub day_of_sim: usize,
    /// Calendar year used for date arithmetic.
    pub year: u32,
    /// Whether `year` is a proleptic-Gregorian leap year.
    ///
    /// EPW-dependent leap shape is stored separately in
    /// `weather_effective_year_is_leap_year`.
    pub gregorian_year_is_leap_year: bool,
    /// Leap shape used by this axis; Gregorian-only projections use Gregorian shape.
    pub weather_effective_year_is_leap_year: bool,
    /// EnergyPlus `LeapYearAdd` value for weather day-of-year calculations.
    pub leap_year_add: u32,
    /// Month number, 1-12.
    pub month: u32,
    /// Day of month.
    pub day_of_month: u32,
    /// Gregorian ordinal day in the current year, 1-365/366.
    pub gregorian_day_of_year: u32,
    /// Weather-effective ordinal day, 1-365/366.
    pub day_of_year: u32,
    /// Leap-shaped schedule ordinal, where March 1 is always day 61.
    pub schedule_day_of_year: u32,
    /// Gregorian weekday for the calendar date.
    pub gregorian_day_of_week: DayOfWeek,
    /// Simulation-effective weekday after weather-calendar skips.
    pub day_of_week: DayOfWeek,
    /// Schedule day type before special-day overrides.
    pub day_type: DayType,
    /// Daylight-saving state. Weather-file DST rules are not applied yet.
    pub dst: bool,
    /// EnergyPlus-style hour ending, 1-24.
    pub hour: u32,
    /// Start minute of the hourly interval.
    pub start_minute: f64,
    /// End minute of the hourly interval.
    pub end_minute: f64,
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

/// Zone and system timestep settings shared by runtime time axes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeAxisTimestepProfile {
    /// Zone timestep profile derived from the model timestep object.
    pub zone_timestep: ZoneTimestepAxis,
    /// Nominal system timestep profile before adaptive shortening.
    pub system_timestep: SystemTimestepAxis,
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
    /// EPW policy applied to this axis, or `None` for a Gregorian-only projection.
    pub weather_calendar: Option<ResolvedWeatherEnvironmentCalendar>,
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

/// Error returned while resolving a run period or building a time axis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeAxisError {
    /// An end year was supplied without the required start year.
    EndYearWithoutStartYear {
        /// Run period name.
        run_period_name: String,
    },
    /// EnergyPlus does not accept run-period years before 1583.
    StartYearBeforeGregorianCalendar {
        /// Run period name.
        run_period_name: String,
        /// Invalid start year.
        year: u32,
    },
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
    /// Metadata-aware actual-weather traversal is not implemented yet.
    ActualWeatherUnsupported {
        /// Run period name.
        run_period_name: String,
    },
    /// Metadata-aware cross-year weather traversal is not implemented yet.
    WeatherMetadataCrossYearUnsupported {
        /// Run period name.
        run_period_name: String,
        /// Resolved Gregorian start year.
        start_year: u32,
        /// Resolved Gregorian end year.
        end_year: u32,
    },
}

impl Display for TimeAxisError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndYearWithoutStartYear { run_period_name } => write!(
                formatter,
                "run period {run_period_name} has an end year without a start year"
            ),
            Self::StartYearBeforeGregorianCalendar {
                run_period_name,
                year,
            } => write!(
                formatter,
                "run period {run_period_name} start year {year} is before 1583"
            ),
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
            Self::ActualWeatherUnsupported { run_period_name } => write!(
                formatter,
                "run period {run_period_name} treats weather as actual, but metadata-aware EPW record traversal is not implemented"
            ),
            Self::WeatherMetadataCrossYearUnsupported {
                run_period_name,
                start_year,
                end_year,
            } => write!(
                formatter,
                "run period {run_period_name} spans {start_year}-{end_year}, but metadata-aware cross-year weather traversal is not implemented"
            ),
        }
    }
}

impl std::error::Error for TimeAxisError {}

/// Resolves EnergyPlus run-period year, range, leap-day, and weekday rules.
pub fn resolve_run_period_calendar(
    run_period: &RunPeriod,
) -> Result<ResolvedRunPeriodCalendar, TimeAxisError> {
    if run_period.begin_year.is_none() && run_period.end_year.is_some() {
        return Err(TimeAxisError::EndYearWithoutStartYear {
            run_period_name: run_period.name.0.clone(),
        });
    }
    if let Some(start_year) = run_period.begin_year
        && start_year < EARLIEST_GREGORIAN_RUN_PERIOD_YEAR
    {
        return Err(TimeAxisError::StartYearBeforeGregorianCalendar {
            run_period_name: run_period.name.0.clone(),
            year: start_year,
        });
    }
    if let (Some(start_year), Some(end_year)) = (run_period.begin_year, run_period.end_year)
        && end_year < start_year
    {
        return Err(TimeAxisError::InvalidRange {
            run_period_name: run_period.name.0.clone(),
        });
    }

    let leap_day_start = run_period.begin_month == 2 && run_period.begin_day_of_month == 29;
    let start_year = if let Some(start_year) = run_period.begin_year {
        start_year
    } else if leap_day_start {
        run_period
            .day_of_week_for_start_day
            .map(|weekday| {
                find_energyplus_year_for_weekday(
                    run_period.begin_month,
                    run_period.begin_day_of_month,
                    weekday,
                    true,
                )
            })
            .unwrap_or(DEFAULT_LEAP_RUN_PERIOD_YEAR)
    } else {
        run_period
            .day_of_week_for_start_day
            .map(|weekday| {
                find_energyplus_year_for_weekday(
                    run_period.begin_month,
                    run_period.begin_day_of_month,
                    weekday,
                    false,
                )
            })
            .unwrap_or(DEFAULT_RUN_PERIOD_YEAR)
    };
    let start = Date {
        year: start_year,
        month: run_period.begin_month,
        day_of_month: run_period.begin_day_of_month,
    };
    let start_ordinal = validated_date_ordinal(run_period, "begin", start)?;

    let leap_day_end = run_period.end_month == 2 && run_period.end_day_of_month == 29;
    let end_year = if let Some(end_year) = run_period.end_year {
        end_year
    } else if leap_day_end {
        if is_leap_year(start_year) && run_period.begin_month < 3 {
            start_year
        } else {
            next_leap_year(start_year).ok_or_else(|| TimeAxisError::InvalidRange {
                run_period_name: run_period.name.0.clone(),
            })?
        }
    } else {
        let same_year_end = Date {
            year: start_year,
            month: run_period.end_month,
            day_of_month: run_period.end_day_of_month,
        };
        let same_year_end_ordinal = validated_date_ordinal(run_period, "end", same_year_end)?;
        if same_year_end_ordinal < start_ordinal {
            start_year
                .checked_add(1)
                .ok_or_else(|| TimeAxisError::InvalidRange {
                    run_period_name: run_period.name.0.clone(),
                })?
        } else {
            start_year
        }
    };
    let end = Date {
        year: end_year,
        month: run_period.end_month,
        day_of_month: run_period.end_day_of_month,
    };
    let end_ordinal = validated_date_ordinal(run_period, "end", end)?;
    if end_ordinal < start_ordinal {
        return Err(TimeAxisError::InvalidRange {
            run_period_name: run_period.name.0.clone(),
        });
    }
    let total_days = usize::try_from(end_ordinal - start_ordinal + 1).map_err(|_| {
        TimeAxisError::InvalidRange {
            run_period_name: run_period.name.0.clone(),
        }
    })?;

    Ok(ResolvedRunPeriodCalendar {
        run_period_name: run_period.name.0.clone(),
        start_year,
        start_month: start.month,
        start_day_of_month: start.day_of_month,
        start_day_of_week: day_of_week(start_ordinal),
        start_year_is_leap_year: is_leap_year(start_year),
        end_year,
        end_month: end.month,
        end_day_of_month: end.day_of_month,
        end_year_is_leap_year: is_leap_year(end_year),
        total_days,
    })
}

/// Builds canonical zone-timestep axes for every model `RunPeriod`.
///
/// If no run period is present, one deterministic one-day weather environment
/// is returned so early runtime paths still have explicit calendar state.
pub fn build_environment_time_axes(
    model: &TypedModel,
) -> Result<Vec<EnvironmentTimeAxis>, TimeAxisError> {
    let zone_timesteps_per_hour = model.timestep.number_of_timesteps_per_hour.max(1);
    if model.run_periods.is_empty() {
        return build_environment_time_axis_for_run_period_with_zone_timesteps(
            &default_run_period(),
            1,
            zone_timesteps_per_hour,
        )
        .map(|axis| vec![axis]);
    }

    model
        .run_periods
        .iter()
        .enumerate()
        .map(|(index, run_period)| {
            build_environment_time_axis_for_run_period_with_zone_timesteps(
                run_period,
                index + 1,
                zone_timesteps_per_hour,
            )
        })
        .collect()
}

/// Builds canonical zone-timestep axes after applying EPW calendar metadata.
pub fn build_environment_time_axes_with_weather_metadata(
    model: &TypedModel,
    metadata: &EpwCalendarMetadata,
) -> Result<Vec<EnvironmentTimeAxis>, TimeAxisError> {
    let zone_timesteps_per_hour = model.timestep.number_of_timesteps_per_hour.max(1);
    if model.run_periods.is_empty() {
        return build_environment_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
            &default_run_period(),
            metadata,
            1,
            zone_timesteps_per_hour,
        )
        .map(|axis| vec![axis]);
    }

    model
        .run_periods
        .iter()
        .enumerate()
        .map(|(index, run_period)| {
            build_environment_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
                run_period,
                metadata,
                index + 1,
                zone_timesteps_per_hour,
            )
        })
        .collect()
}

/// Builds a canonical axis for one run period with one zone timestep per hour.
pub fn build_environment_time_axis_for_run_period(
    run_period: &RunPeriod,
) -> Result<EnvironmentTimeAxis, TimeAxisError> {
    build_environment_time_axis_for_run_period_with_zone_timesteps(run_period, 1, 1)
}

/// Builds a canonical axis for one run period and explicit environment state.
pub fn build_environment_time_axis_for_run_period_with_zone_timesteps(
    run_period: &RunPeriod,
    environment_index: usize,
    zone_timesteps_per_hour: u32,
) -> Result<EnvironmentTimeAxis, TimeAxisError> {
    build_environment_time_axis_for_run_period_internal(
        run_period,
        None,
        environment_index,
        zone_timesteps_per_hour,
    )
}

/// Builds a canonical environment axis after applying EPW calendar metadata.
pub fn build_environment_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
    run_period: &RunPeriod,
    metadata: &EpwCalendarMetadata,
    environment_index: usize,
    zone_timesteps_per_hour: u32,
) -> Result<EnvironmentTimeAxis, TimeAxisError> {
    build_environment_time_axis_for_run_period_internal(
        run_period,
        Some(metadata),
        environment_index,
        zone_timesteps_per_hour,
    )
}

fn build_environment_time_axis_for_run_period_internal(
    run_period: &RunPeriod,
    metadata: Option<&EpwCalendarMetadata>,
    environment_index: usize,
    zone_timesteps_per_hour: u32,
) -> Result<EnvironmentTimeAxis, TimeAxisError> {
    let weather_calendar = metadata
        .map(|metadata| resolve_weather_environment_calendar(run_period, metadata))
        .transpose()?;
    let calendar = weather_calendar.as_ref().map_or_else(
        || resolve_run_period_calendar(run_period),
        |weather_calendar| Ok(weather_calendar.gregorian.clone()),
    )?;
    let timestep_profile = time_axis_timestep_profile_for_zone_timesteps(zone_timesteps_per_hour);
    let zone_timesteps_per_hour = timestep_profile.zone_timestep.timesteps_per_hour;
    let interval_minutes = 60.0 / f64::from(zone_timesteps_per_hour);
    let days = resolved_calendar_days(run_period, &calendar, weather_calendar.as_ref())?;
    let total_points = days
        .len()
        .saturating_mul(24)
        .saturating_mul(zone_timesteps_per_hour as usize);
    let mut points = Vec::with_capacity(total_points);

    for day in days {
        for hour in 1..=24 {
            for zone_timestep in 1..=zone_timesteps_per_hour {
                let start_minute = f64::from(zone_timestep - 1) * interval_minutes;
                let end_minute = f64::from(zone_timestep) * interval_minutes;
                let sample_index = points.len();
                points.push(EnvironmentTimePoint {
                    sample_index,
                    environment_index,
                    environment_kind: EnvironmentKind::WeatherRunPeriod,
                    day_of_sim: day.day_of_sim,
                    year: day.date.year,
                    gregorian_year_is_leap_year: day.gregorian_year_is_leap_year,
                    weather_effective_year_is_leap_year: day.weather_effective_year_is_leap_year,
                    leap_year_add: day.leap_year_add,
                    month: day.date.month,
                    day_of_month: day.date.day_of_month,
                    gregorian_day_of_year: day.gregorian_day_of_year,
                    day_of_year: day.day_of_year,
                    schedule_day_of_year: day.schedule_day_of_year,
                    gregorian_day_of_week: day.gregorian_day_of_week,
                    day_of_week: day.day_of_week,
                    day_type: day.day_of_week.into(),
                    dst: false,
                    special_day_type: None,
                    hour,
                    zone_timestep,
                    start_minute,
                    end_minute,
                    current_time_hours: f64::from(hour - 1)
                        + f64::from(zone_timestep) / f64::from(zone_timesteps_per_hour),
                    simulation_timestep: sample_index + 1,
                    begin_environment: sample_index == 0,
                    end_environment: sample_index + 1 == total_points,
                    begin_day: hour == 1 && zone_timestep == 1,
                    end_day: hour == 24 && zone_timestep == zone_timesteps_per_hour,
                    begin_hour: zone_timestep == 1,
                    end_hour: zone_timestep == zone_timesteps_per_hour,
                });
            }
        }
    }

    Ok(EnvironmentTimeAxis {
        environment_index,
        environment_name: run_period.name.0.clone(),
        environment_kind: EnvironmentKind::WeatherRunPeriod,
        calendar,
        weather_calendar,
        first_hour_interpolation_starting_values: run_period
            .first_hour_interpolation_starting_values,
        zone_timestep: timestep_profile.zone_timestep,
        system_timestep: timestep_profile.system_timestep,
        points,
    })
}

/// Builds the first hourly time axis from the model `RunPeriod` list.
///
/// This compatibility view is an hourly projection of the first canonical
/// environment axis. Its number and order of output samples remain unchanged.
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

/// Builds the first hourly time axis after applying EPW calendar metadata.
pub fn build_hourly_time_axis_with_weather_metadata(
    model: &TypedModel,
    metadata: &EpwCalendarMetadata,
) -> Result<TimeAxis, TimeAxisError> {
    let fallback;
    let run_period = if let Some(run_period) = model.run_periods.first() {
        run_period
    } else {
        fallback = default_run_period();
        &fallback
    };

    build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
        run_period,
        metadata,
        model.timestep.number_of_timesteps_per_hour.max(1),
    )
}

/// Builds the timestep-only portion of a runtime time axis.
///
/// This does not validate or allocate run-period calendar points, so consumers
/// that only need timestep metadata do not become coupled to calendar ranges.
#[must_use]
pub fn time_axis_timestep_profile(model: &TypedModel) -> TimeAxisTimestepProfile {
    time_axis_timestep_profile_for_zone_timesteps(
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
    build_hourly_time_axis_for_run_period_internal(run_period, None, zone_timesteps_per_hour)
}

/// Builds an hourly time axis after applying EPW calendar metadata.
pub fn build_hourly_time_axis_for_run_period_with_weather_metadata(
    run_period: &RunPeriod,
    metadata: &EpwCalendarMetadata,
) -> Result<TimeAxis, TimeAxisError> {
    build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
        run_period, metadata, 1,
    )
}

/// Builds an hourly time axis with EPW metadata and an explicit zone timestep count.
pub fn build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
    run_period: &RunPeriod,
    metadata: &EpwCalendarMetadata,
    zone_timesteps_per_hour: u32,
) -> Result<TimeAxis, TimeAxisError> {
    build_hourly_time_axis_for_run_period_internal(
        run_period,
        Some(metadata),
        zone_timesteps_per_hour,
    )
}

fn build_hourly_time_axis_for_run_period_internal(
    run_period: &RunPeriod,
    metadata: Option<&EpwCalendarMetadata>,
    zone_timesteps_per_hour: u32,
) -> Result<TimeAxis, TimeAxisError> {
    let weather_calendar = metadata
        .map(|metadata| resolve_weather_environment_calendar(run_period, metadata))
        .transpose()?;
    let calendar = weather_calendar.as_ref().map_or_else(
        || resolve_run_period_calendar(run_period),
        |weather_calendar| Ok(weather_calendar.gregorian.clone()),
    )?;
    let timestep_profile = time_axis_timestep_profile_for_zone_timesteps(zone_timesteps_per_hour);
    let days = resolved_calendar_days(run_period, &calendar, weather_calendar.as_ref())?;
    let mut points = Vec::with_capacity(days.len() * 24);

    for day in days {
        for hour in 1..=24 {
            points.push(TimePoint {
                sample_index: points.len(),
                day_of_sim: day.day_of_sim,
                year: day.date.year,
                gregorian_year_is_leap_year: day.gregorian_year_is_leap_year,
                weather_effective_year_is_leap_year: day.weather_effective_year_is_leap_year,
                leap_year_add: day.leap_year_add,
                month: day.date.month,
                day_of_month: day.date.day_of_month,
                gregorian_day_of_year: day.gregorian_day_of_year,
                day_of_year: day.day_of_year,
                schedule_day_of_year: day.schedule_day_of_year,
                gregorian_day_of_week: day.gregorian_day_of_week,
                day_of_week: day.day_of_week,
                day_type: day.day_of_week.into(),
                dst: false,
                hour,
                start_minute: 0.0,
                end_minute: 60.0,
            });
        }
    }

    Ok(TimeAxis {
        run_period_name: run_period.name.0.clone(),
        weather_calendar,
        first_hour_interpolation_starting_values: run_period
            .first_hour_interpolation_starting_values,
        zone_timestep: timestep_profile.zone_timestep,
        system_timestep: timestep_profile.system_timestep,
        sample_partitions: TimeAxisSamplePartitions {
            warmup_reported_samples: 0,
            run_period_reported_samples: points.len(),
            design_day_reported_samples: 0,
        },
        points,
    })
}

fn time_axis_timestep_profile_for_zone_timesteps(
    zone_timesteps_per_hour: u32,
) -> TimeAxisTimestepProfile {
    let zone_timesteps_per_hour = zone_timesteps_per_hour.max(1);
    let zone_timestep_seconds = 3600.0 / f64::from(zone_timesteps_per_hour);
    TimeAxisTimestepProfile {
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
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Date {
    pub(crate) year: u32,
    pub(crate) month: u32,
    pub(crate) day_of_month: u32,
}

#[derive(Clone, Copy)]
struct ResolvedCalendarDay {
    day_of_sim: usize,
    date: Date,
    gregorian_year_is_leap_year: bool,
    weather_effective_year_is_leap_year: bool,
    leap_year_add: u32,
    gregorian_day_of_year: u32,
    day_of_year: u32,
    schedule_day_of_year: u32,
    gregorian_day_of_week: DayOfWeek,
    day_of_week: DayOfWeek,
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
        use_weather_file_holidays_and_special_days: true,
        use_weather_file_daylight_saving_period: true,
        apply_weekend_holiday_rule: true,
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: false,
    }
}

fn validated_date_ordinal(
    run_period: &RunPeriod,
    field: &'static str,
    date: Date,
) -> Result<i64, TimeAxisError> {
    date_ordinal(date).ok_or_else(|| invalid_date_error(run_period, field, date))
}

fn resolved_calendar_days(
    run_period: &RunPeriod,
    calendar: &ResolvedRunPeriodCalendar,
    weather_calendar: Option<&ResolvedWeatherEnvironmentCalendar>,
) -> Result<Vec<ResolvedCalendarDay>, TimeAxisError> {
    let weather_file_allows_leap_years = weather_calendar
        .map(|calendar| calendar.weather_file_allows_leap_years)
        .unwrap_or(true);
    let expected_days = weather_calendar
        .map(|calendar| calendar.total_days)
        .unwrap_or(calendar.total_days);
    let mut days = Vec::with_capacity(expected_days);
    let mut date = calendar.start_date();
    if !weather_file_allows_leap_years && date.month == 2 && date.day_of_month == 29 {
        date = next_day(date);
    }
    for day_index in 0..expected_days {
        let date_ordinal = validated_date_ordinal(run_period, "current", date)?;
        let Some(gregorian_day_of_year) = day_of_year(date.year, date.month, date.day_of_month)
        else {
            return Err(invalid_date_error(run_period, "current", date));
        };
        let gregorian_year_is_leap_year = is_leap_year(date.year);
        let weather_effective_year_is_leap_year =
            gregorian_year_is_leap_year && weather_file_allows_leap_years;
        let leap_year_add = u32::from(weather_effective_year_is_leap_year);
        let weather_shape_year = if weather_effective_year_is_leap_year {
            DEFAULT_LEAP_RUN_PERIOD_YEAR
        } else {
            DEFAULT_RUN_PERIOD_YEAR
        };
        let Some(weather_day_of_year) =
            day_of_year(weather_shape_year, date.month, date.day_of_month)
        else {
            return Err(invalid_date_error(run_period, "current", date));
        };
        let Some(schedule_day_of_year) =
            leap_shaped_schedule_day_of_year(date.month, date.day_of_month)
        else {
            return Err(invalid_date_error(run_period, "current", date));
        };
        let gregorian_day_of_week = day_of_week(date_ordinal);
        let simulation_day_of_week = if weather_calendar.is_some() {
            advance_day_of_week(calendar.start_day_of_week, days.len())
        } else {
            gregorian_day_of_week
        };
        days.push(ResolvedCalendarDay {
            day_of_sim: days.len() + 1,
            date,
            gregorian_year_is_leap_year,
            weather_effective_year_is_leap_year,
            leap_year_add,
            gregorian_day_of_year,
            day_of_year: weather_day_of_year,
            schedule_day_of_year,
            gregorian_day_of_week,
            day_of_week: simulation_day_of_week,
        });
        if day_index + 1 < expected_days {
            date = next_day(date);
            if !weather_file_allows_leap_years && date.month == 2 && date.day_of_month == 29 {
                date = next_day(date);
            }
        }
    }
    debug_assert_eq!(days.len(), expected_days);
    Ok(days)
}

fn invalid_date_error(run_period: &RunPeriod, field: &'static str, date: Date) -> TimeAxisError {
    TimeAxisError::InvalidDate {
        run_period_name: run_period.name.0.clone(),
        field,
        year: date.year,
        month: date.month,
        day_of_month: date.day_of_month,
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

fn day_of_week(date_ordinal: i64) -> DayOfWeek {
    match date_ordinal.rem_euclid(7) {
        0 => DayOfWeek::Monday,
        1 => DayOfWeek::Tuesday,
        2 => DayOfWeek::Wednesday,
        3 => DayOfWeek::Thursday,
        4 => DayOfWeek::Friday,
        5 => DayOfWeek::Saturday,
        _ => DayOfWeek::Sunday,
    }
}

fn energyplus_weekday_number(day_of_week: DayOfWeek) -> i32 {
    match day_of_week {
        DayOfWeek::Sunday => 1,
        DayOfWeek::Monday => 2,
        DayOfWeek::Tuesday => 3,
        DayOfWeek::Wednesday => 4,
        DayOfWeek::Thursday => 5,
        DayOfWeek::Friday => 6,
        DayOfWeek::Saturday => 7,
    }
}

fn advance_day_of_week(start: DayOfWeek, elapsed_simulation_days: usize) -> DayOfWeek {
    let number = ((energyplus_weekday_number(start) - 1
        + i32::try_from(elapsed_simulation_days % 7).unwrap_or(0))
        % 7)
        + 1;
    match number {
        1 => DayOfWeek::Sunday,
        2 => DayOfWeek::Monday,
        3 => DayOfWeek::Tuesday,
        4 => DayOfWeek::Wednesday,
        5 => DayOfWeek::Thursday,
        6 => DayOfWeek::Friday,
        _ => DayOfWeek::Saturday,
    }
}

fn find_energyplus_year_for_weekday(
    month: u32,
    day_of_month: u32,
    weekday: DayOfWeek,
    leap_year: bool,
) -> u32 {
    const DEFAULT_YEARS: [u32; 13] = [
        2013, 2014, 2015, 2010, 2011, 2017, 2007, 2013, 2014, 2015, 2010, 2011, 2017,
    ];
    const DEFAULT_LEAP_YEARS: [u32; 13] = [
        2008, 1992, 2004, 2016, 2000, 2012, 1996, 2008, 1992, 2004, 2016, 2000, 2012,
    ];
    let ordinal = if leap_year {
        day_of_year(DEFAULT_LEAP_RUN_PERIOD_YEAR, month, day_of_month)
    } else {
        day_of_year(DEFAULT_RUN_PERIOD_YEAR, month, day_of_month)
    }
    .unwrap_or(1);
    let index = energyplus_weekday_number(weekday) - (ordinal % 7) as i32 + 5;
    if leap_year {
        DEFAULT_LEAP_YEARS[index as usize]
    } else {
        DEFAULT_YEARS[index as usize]
    }
}

fn next_leap_year(start_year: u32) -> Option<u32> {
    (1..10)
        .filter_map(|offset| start_year.checked_add(offset))
        .find(|year| is_leap_year(*year))
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

fn leap_shaped_schedule_day_of_year(month: u32, day_of_month: u32) -> Option<u32> {
    day_of_year(DEFAULT_LEAP_RUN_PERIOD_YEAR, month, day_of_month)
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
