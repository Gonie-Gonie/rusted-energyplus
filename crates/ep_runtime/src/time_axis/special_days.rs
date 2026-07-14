use super::calendar_rules::{CalendarRuleResolutionError, resolve_calendar_date_rule};
use super::{
    DEFAULT_LEAP_RUN_PERIOD_YEAR, DEFAULT_RUN_PERIOD_YEAR, DayType, ResolvedRunPeriodCalendar,
    ResolvedWeatherEnvironmentCalendar, TimeAxisError, day_of_year, shift_day_of_week,
};
use ep_model::{CalendarDateRule, RunPeriod, RunPeriodSpecialDay};

/// Concrete start date used to project one input-file special day.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedSpecialDayDate {
    /// Month number, 1-12.
    pub month: u32,
    /// Day of month selected by the date rule.
    pub day_of_month: u32,
    /// Weather-effective ordinal used by EnergyPlus' special-day table.
    pub day_of_year: u32,
}

/// One resolved `RunPeriodControl:SpecialDays` definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedSpecialDay {
    /// Input object name.
    pub name: String,
    /// Concrete start after any applicable weekend shift.
    pub start: ResolvedSpecialDayDate,
    /// Inclusive number of days written from `start`.
    pub duration_days: u32,
    /// Effective EnergyPlus schedule day type.
    pub day_type: DayType,
    /// Days shifted forward by the weekend rule, zero when it did not apply.
    pub weekend_shift_days: u32,
}

/// Diagnostic and lookup state for input-file special days on one time axis.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SpecialDayAxisState {
    /// Number of typed `RunPeriodControl:SpecialDays` definitions supplied.
    pub input_file_special_days_declared: usize,
    /// Whether the active RunPeriod requests the weekend holiday rule.
    pub apply_weekend_rule: bool,
    /// Resolved input-file definitions in compiler order.
    pub resolved_days: Vec<ResolvedSpecialDay>,
    day_types_by_ordinal: Vec<Option<DayType>>,
}

pub(super) fn resolve_special_day_axis_state(
    run_period: &RunPeriod,
    calendar: &ResolvedRunPeriodCalendar,
    weather_calendar: Option<&ResolvedWeatherEnvironmentCalendar>,
    special_days: &[RunPeriodSpecialDay],
) -> Result<SpecialDayAxisState, TimeAxisError> {
    if !special_days.is_empty() && calendar.start_year != calendar.end_year {
        return Err(TimeAxisError::SpecialDayCrossYearUnsupported {
            run_period_name: run_period.name.0.clone(),
            start_year: calendar.start_year,
            end_year: calendar.end_year,
        });
    }
    let weather_effective_leap_year = weather_calendar
        .map(|calendar| calendar.start_year_is_weather_effective_leap_year)
        .unwrap_or(calendar.start_year_is_leap_year);
    let days_in_year = if weather_effective_leap_year {
        366
    } else {
        365
    };
    let mut day_types_by_ordinal = vec![None; days_in_year + 1];
    let mut resolved_days = Vec::with_capacity(special_days.len());

    for special_day in special_days {
        let resolved = resolve_calendar_date_rule(
            calendar,
            weather_effective_leap_year,
            special_day.start_date,
        )
        .map_err(|error| special_day_resolution_error(run_period, special_day, calendar, error))?;
        let weekend_shift_days = weekend_shift_days(
            run_period,
            calendar,
            special_day,
            resolved.day_of_year,
            weather_effective_leap_year,
        );
        let start_day_of_year = wrap_ordinal(
            resolved.day_of_year + weekend_shift_days,
            days_in_year as u32,
        );
        let (month, day_of_month) =
            month_day_from_weather_ordinal(start_day_of_year, weather_effective_leap_year);
        let day_type = special_day.special_day_type.into();
        for offset in 0..special_day.duration_days {
            let ordinal = wrap_ordinal(start_day_of_year + offset, days_in_year as u32);
            day_types_by_ordinal[ordinal as usize] = Some(day_type);
        }
        resolved_days.push(ResolvedSpecialDay {
            name: special_day.name.0.clone(),
            start: ResolvedSpecialDayDate {
                month,
                day_of_month,
                day_of_year: start_day_of_year,
            },
            duration_days: special_day.duration_days,
            day_type,
            weekend_shift_days,
        });
    }

    Ok(SpecialDayAxisState {
        input_file_special_days_declared: special_days.len(),
        apply_weekend_rule: run_period.apply_weekend_holiday_rule,
        resolved_days,
        day_types_by_ordinal,
    })
}

pub(super) fn special_day_type_for_ordinal(
    state: &SpecialDayAxisState,
    day_of_year: u32,
) -> Option<DayType> {
    state
        .day_types_by_ordinal
        .get(day_of_year as usize)
        .copied()
        .flatten()
}

fn special_day_resolution_error(
    run_period: &RunPeriod,
    special_day: &RunPeriodSpecialDay,
    calendar: &ResolvedRunPeriodCalendar,
    error: CalendarRuleResolutionError,
) -> TimeAxisError {
    match error {
        CalendarRuleResolutionError::InvalidDate {
            month,
            day_of_month,
        } => TimeAxisError::InvalidDate {
            run_period_name: run_period.name.0.clone(),
            field: "special-day start",
            year: calendar.start_year,
            month,
            day_of_month,
        },
        CalendarRuleResolutionError::NthWeekdayDoesNotExist {
            nth,
            weekday,
            month,
        } => TimeAxisError::SpecialDayDateRuleDoesNotExist {
            run_period_name: run_period.name.0.clone(),
            special_day_name: special_day.name.0.clone(),
            nth,
            weekday,
            month,
        },
    }
}

fn weekend_shift_days(
    run_period: &RunPeriod,
    calendar: &ResolvedRunPeriodCalendar,
    special_day: &RunPeriodSpecialDay,
    start_day_of_year: u32,
    weather_effective_leap_year: bool,
) -> u32 {
    if !run_period.apply_weekend_holiday_rule
        || special_day.duration_days != 1
        || !matches!(special_day.start_date, CalendarDateRule::MonthDay { .. })
    {
        return 0;
    }
    let start_ordinal = weather_day_of_year(
        calendar.start_month,
        calendar.start_day_of_month,
        weather_effective_leap_year,
    )
    .unwrap_or(1);
    match shift_day_of_week(
        calendar.start_day_of_week,
        i64::from(start_day_of_year) - i64::from(start_ordinal),
    ) {
        ep_model::DayOfWeek::Sunday => 1,
        ep_model::DayOfWeek::Saturday => 2,
        _ => 0,
    }
}

fn wrap_ordinal(day_of_year: u32, days_in_year: u32) -> u32 {
    (day_of_year.saturating_sub(1) % days_in_year) + 1
}

fn weather_day_of_year(
    month: u32,
    day_of_month: u32,
    weather_effective_leap_year: bool,
) -> Option<u32> {
    if month == 2 && day_of_month == 29 && !weather_effective_leap_year {
        return Some(60);
    }
    day_of_year(
        if weather_effective_leap_year {
            DEFAULT_LEAP_RUN_PERIOD_YEAR
        } else {
            DEFAULT_RUN_PERIOD_YEAR
        },
        month,
        day_of_month,
    )
}

fn month_day_from_weather_ordinal(
    day_of_year: u32,
    weather_effective_leap_year: bool,
) -> (u32, u32) {
    let year = if weather_effective_leap_year {
        DEFAULT_LEAP_RUN_PERIOD_YEAR
    } else {
        DEFAULT_RUN_PERIOD_YEAR
    };
    let mut remaining = day_of_year;
    for month in 1..=12 {
        let month_days = super::days_in_month(year, month);
        if remaining <= month_days {
            return (month, remaining);
        }
        remaining -= month_days;
    }
    (12, super::days_in_month(year, 12))
}
