use super::{
    DEFAULT_LEAP_RUN_PERIOD_YEAR, DEFAULT_RUN_PERIOD_YEAR, Date, ResolvedRunPeriodCalendar,
    ResolvedWeatherEnvironmentCalendar, TimeAxisError, day_of_year, days_in_month,
    energyplus_weekday_number, invalid_date_error, shift_day_of_week,
};
use crate::weather::{EpwCalendarDateRule, EpwCalendarMetadata, EpwDaylightSavingPeriod};
use ep_model::{DayOfWeek, RunPeriod};

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

/// Diagnostic state describing how EPW daylight-saving input affects an axis.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DaylightSavingAxisState {
    /// Whether the EPW header declared a daylight-saving period.
    pub weather_file_period_declared: bool,
    /// Whether the RunPeriod enables the weather-file daylight-saving period.
    pub run_period_uses_weather_file_period: bool,
    /// Whether the declared period is active for this axis.
    pub active: bool,
    /// Concrete start/end dates resolved with EnergyPlus weekday rules.
    pub resolved_period: Option<ResolvedDaylightSavingPeriod>,
}

pub(super) fn resolve_daylight_saving_axis_state(
    run_period: &RunPeriod,
    calendar: &ResolvedRunPeriodCalendar,
    weather_calendar: Option<&ResolvedWeatherEnvironmentCalendar>,
    metadata: Option<&EpwCalendarMetadata>,
) -> Result<DaylightSavingAxisState, TimeAxisError> {
    let daylight_saving_period = metadata.and_then(|metadata| metadata.daylight_saving_period);
    let weather_file_period_declared = daylight_saving_period.is_some();
    let run_period_uses_weather_file_period = run_period.use_weather_file_daylight_saving_period;
    let active = weather_file_period_declared && run_period_uses_weather_file_period;
    let resolved_period = if active {
        daylight_saving_period
            .map(|period| {
                resolve_daylight_saving_period(run_period, calendar, weather_calendar, period)
            })
            .transpose()?
    } else {
        None
    };

    Ok(DaylightSavingAxisState {
        weather_file_period_declared,
        run_period_uses_weather_file_period,
        active,
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
    let (month, day_of_month) = match rule {
        EpwCalendarDateRule::MonthDay {
            month,
            day_of_month,
        } => (month, day_of_month),
        EpwCalendarDateRule::NthWeekdayInMonth {
            nth,
            weekday,
            month,
        } => {
            let first_weekday =
                run_period_month_weekday_for_month_day(run_period, calendar, month, 1, boundary)?;
            let first_occurrence = 1
                + (energyplus_weekday_number(weekday) - energyplus_weekday_number(first_weekday))
                    .rem_euclid(7) as u32;
            let day_of_month = first_occurrence + 7 * nth.saturating_sub(1);
            let weather_shape_year = if weather_effective_leap_year {
                DEFAULT_LEAP_RUN_PERIOD_YEAR
            } else {
                DEFAULT_RUN_PERIOD_YEAR
            };
            if nth == 0 || day_of_month > days_in_month(weather_shape_year, month) {
                return Err(TimeAxisError::DaylightSavingDateRuleDoesNotExist {
                    run_period_name: run_period.name.0.clone(),
                    boundary,
                    nth,
                    weekday,
                    month,
                });
            }
            (month, day_of_month)
        }
        EpwCalendarDateRule::LastWeekdayInMonth { weekday, month } => {
            let first_weekday =
                run_period_month_weekday_for_month_day(run_period, calendar, month, 1, boundary)?;
            let mut day_of_month = 1
                + (energyplus_weekday_number(weekday) - energyplus_weekday_number(first_weekday))
                    .rem_euclid(7) as u32;
            let weather_shape_year = if weather_effective_leap_year {
                DEFAULT_LEAP_RUN_PERIOD_YEAR
            } else {
                DEFAULT_RUN_PERIOD_YEAR
            };
            let last_day_of_month = days_in_month(weather_shape_year, month);
            while day_of_month + 7 <= last_day_of_month {
                day_of_month += 7;
            }
            (month, day_of_month)
        }
    };
    let day_of_year =
        weather_effective_day_of_year(month, day_of_month, weather_effective_leap_year)
            .ok_or_else(|| {
                invalid_date_error(
                    run_period,
                    if boundary == "start" {
                        "daylight-saving start"
                    } else {
                        "daylight-saving end"
                    },
                    Date {
                        year: calendar.start_year,
                        month,
                        day_of_month,
                    },
                )
            })?;
    Ok(ResolvedDaylightSavingDate {
        month,
        day_of_month,
        day_of_year,
    })
}

fn run_period_month_weekday_for_month_day(
    run_period: &RunPeriod,
    calendar: &ResolvedRunPeriodCalendar,
    month: u32,
    day_of_month: u32,
    boundary: &'static str,
) -> Result<DayOfWeek, TimeAxisError> {
    // EnergyPlus seeds Environment::MonWeekDay while RunPeriod input is read,
    // before the environment-specific LeapYearAdd is applied. Preserve that
    // non-leap weekday projection for Nth/Last rule resolution; the resolved
    // date is converted to a weather-effective ordinal separately below.
    let start_day_of_year =
        weather_effective_day_of_year(calendar.start_month, calendar.start_day_of_month, false)
            .ok_or_else(|| invalid_date_error(run_period, "begin", calendar.start_date()))?;
    let target_day_of_year =
        weather_effective_day_of_year(month, day_of_month, false).ok_or_else(|| {
            invalid_date_error(
                run_period,
                if boundary == "start" {
                    "daylight-saving start"
                } else {
                    "daylight-saving end"
                },
                Date {
                    year: calendar.start_year,
                    month,
                    day_of_month,
                },
            )
        })?;
    Ok(shift_day_of_week(
        calendar.start_day_of_week,
        i64::from(target_day_of_year) - i64::from(start_day_of_year),
    ))
}

fn weather_effective_day_of_year(
    month: u32,
    day_of_month: u32,
    weather_effective_leap_year: bool,
) -> Option<u32> {
    if month == 2 && day_of_month == 29 && !weather_effective_leap_year {
        return Some(60);
    }
    let weather_shape_year = if weather_effective_leap_year {
        DEFAULT_LEAP_RUN_PERIOD_YEAR
    } else {
        DEFAULT_RUN_PERIOD_YEAR
    };
    day_of_year(weather_shape_year, month, day_of_month)
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
