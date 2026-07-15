use super::calendar_rules::{CalendarRuleResolutionError, resolve_calendar_date_rule};
use super::{ResolvedRunPeriodCalendar, ResolvedWeatherEnvironmentCalendar, TimeAxisError};
use crate::weather::{EpwCalendarDateRule, EpwCalendarMetadata, EpwDaylightSavingPeriod};
use ep_model::{RunPeriod, RunPeriodDaylightSavingTime};

/// One concrete weather-effective daylight-saving boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedDaylightSavingDate {
    /// Month number, 1-12.
    pub month: u32,
    /// Day of month selected by the EPW rule.
    pub day_of_month: u32,
    /// Weather-effective ordinal used by EnergyPlus' `DSTIndex`.
    pub day_of_year: u32,
}

/// Concrete inclusive daylight-saving range for the resolved weather year.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedDaylightSavingPeriod {
    /// First active daylight-saving day.
    pub start: ResolvedDaylightSavingDate,
    /// Final active daylight-saving day.
    pub end: ResolvedDaylightSavingDate,
    /// Whether the active range crosses the end of the weather year.
    pub wraps_year: bool,
}

/// Source of the daylight-saving period selected for one axis.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DaylightSavingPeriodSource {
    /// No daylight-saving period is active.
    #[default]
    None,
    /// The EPW HOLIDAYS/DAYLIGHT SAVINGS header supplies the active period.
    WeatherFile,
    /// RunPeriodControl:DaylightSavingTime supplies the active period.
    InputFile,
}

impl DaylightSavingPeriodSource {
    /// Stable diagnostic label used by CLI summaries.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::WeatherFile => "weather-file",
            Self::InputFile => "input-file",
        }
    }
}

/// Diagnostic state describing how weather-file and input-file daylight-saving
/// declarations affect an axis.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DaylightSavingAxisState {
    /// Whether the EPW header declared a daylight-saving period.
    pub weather_file_period_declared: bool,
    /// Whether the RunPeriod enables the weather-file daylight-saving period.
    pub run_period_uses_weather_file_period: bool,
    /// Whether an input-file RunPeriodControl:DaylightSavingTime was declared.
    pub input_file_period_declared: bool,
    /// Whether an effective period is active for this axis.
    pub active: bool,
    /// Source selected after applying EnergyPlus GetDSTData precedence.
    pub effective_source: DaylightSavingPeriodSource,
    /// Concrete start/end dates resolved with EnergyPlus weekday rules.
    pub resolved_period: Option<ResolvedDaylightSavingPeriod>,
}

pub(super) fn resolve_daylight_saving_axis_state(
    run_period: &RunPeriod,
    calendar: &ResolvedRunPeriodCalendar,
    weather_calendar: Option<&ResolvedWeatherEnvironmentCalendar>,
    metadata: Option<&EpwCalendarMetadata>,
    input_file_daylight_saving_time: Option<&RunPeriodDaylightSavingTime>,
) -> Result<DaylightSavingAxisState, TimeAxisError> {
    let weather_file_period = metadata.and_then(|metadata| metadata.daylight_saving_period);
    let weather_file_period_declared = weather_file_period.is_some();
    let run_period_uses_weather_file_period = run_period.use_weather_file_daylight_saving_period;
    let input_file_period = input_file_daylight_saving_time.map(|period| EpwDaylightSavingPeriod {
        start: period.start_date,
        end: period.end_date,
    });
    let input_file_period_declared = input_file_period.is_some();

    // EnergyPlus first reads the EPW declaration, then GetDSTData replaces
    // it with the IDF object. The IDF object is active independently of the
    // RunPeriod weather-file use flag.
    let (effective_source, effective_period) = if let Some(period) = input_file_period {
        (DaylightSavingPeriodSource::InputFile, Some(period))
    } else if run_period_uses_weather_file_period {
        weather_file_period.map_or((DaylightSavingPeriodSource::None, None), |period| {
            (DaylightSavingPeriodSource::WeatherFile, Some(period))
        })
    } else {
        (DaylightSavingPeriodSource::None, None)
    };
    let resolved_period = effective_period
        .map(|period| {
            resolve_daylight_saving_period(run_period, calendar, weather_calendar, period)
        })
        .transpose()?;
    let active = resolved_period.is_some();

    Ok(DaylightSavingAxisState {
        weather_file_period_declared,
        run_period_uses_weather_file_period,
        input_file_period_declared,
        active,
        effective_source,
        resolved_period,
    })
}

fn resolve_daylight_saving_period(
    run_period: &RunPeriod,
    calendar: &ResolvedRunPeriodCalendar,
    weather_calendar: Option<&ResolvedWeatherEnvironmentCalendar>,
    period: EpwDaylightSavingPeriod,
) -> Result<ResolvedDaylightSavingPeriod, TimeAxisError> {
    let weather_effective_leap_year = weather_calendar
        .map(|calendar| calendar.start_year_is_weather_effective_leap_year)
        .unwrap_or(calendar.start_year_is_leap_year);
    let start = resolve_daylight_saving_date_rule(
        run_period,
        calendar,
        weather_effective_leap_year,
        period.start,
        "start",
    )?;
    let end = resolve_daylight_saving_date_rule(
        run_period,
        calendar,
        weather_effective_leap_year,
        period.end,
        "end",
    )?;
    Ok(ResolvedDaylightSavingPeriod {
        start,
        end,
        wraps_year: end.day_of_year < start.day_of_year,
    })
}

fn resolve_daylight_saving_date_rule(
    run_period: &RunPeriod,
    calendar: &ResolvedRunPeriodCalendar,
    weather_effective_leap_year: bool,
    rule: EpwCalendarDateRule,
    boundary: &'static str,
) -> Result<ResolvedDaylightSavingDate, TimeAxisError> {
    let resolved = resolve_calendar_date_rule(calendar, weather_effective_leap_year, rule)
        .map_err(|error| match error {
            CalendarRuleResolutionError::InvalidDate {
                month,
                day_of_month,
            } => TimeAxisError::InvalidDate {
                run_period_name: run_period.name.0.clone(),
                field: if boundary == "start" {
                    "daylight-saving start"
                } else {
                    "daylight-saving end"
                },
                year: calendar.start_year,
                month,
                day_of_month,
            },
            CalendarRuleResolutionError::NthWeekdayDoesNotExist {
                nth,
                weekday,
                month,
            } => TimeAxisError::DaylightSavingDateRuleDoesNotExist {
                run_period_name: run_period.name.0.clone(),
                boundary,
                nth,
                weekday,
                month,
            },
        })?;
    Ok(ResolvedDaylightSavingDate {
        month: resolved.month,
        day_of_month: resolved.day_of_month,
        day_of_year: resolved.day_of_year,
    })
}

pub(super) fn daylight_saving_is_active(
    daylight_saving: &DaylightSavingAxisState,
    day_of_year: u32,
) -> bool {
    let Some(period) = daylight_saving.resolved_period else {
        return false;
    };
    if period.wraps_year {
        day_of_year >= period.start.day_of_year || day_of_year <= period.end.day_of_year
    } else {
        (period.start.day_of_year..=period.end.day_of_year).contains(&day_of_year)
    }
}
