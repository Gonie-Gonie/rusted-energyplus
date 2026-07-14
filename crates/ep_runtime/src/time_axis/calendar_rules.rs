use super::{
    DEFAULT_LEAP_RUN_PERIOD_YEAR, DEFAULT_RUN_PERIOD_YEAR, ResolvedRunPeriodCalendar, day_of_year,
    days_in_month, energyplus_weekday_number, shift_day_of_week,
};
use ep_model::{CalendarDateRule, DayOfWeek};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedCalendarRuleDate {
    pub(super) month: u32,
    pub(super) day_of_month: u32,
    pub(super) day_of_year: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CalendarRuleResolutionError {
    InvalidDate {
        month: u32,
        day_of_month: u32,
    },
    NthWeekdayDoesNotExist {
        nth: u32,
        weekday: DayOfWeek,
        month: u32,
    },
}

pub(super) fn resolve_calendar_date_rule(
    calendar: &ResolvedRunPeriodCalendar,
    weather_effective_leap_year: bool,
    rule: CalendarDateRule,
) -> Result<ResolvedCalendarRuleDate, CalendarRuleResolutionError> {
    let (month, day_of_month) = match rule {
        CalendarDateRule::MonthDay {
            month,
            day_of_month,
        } => (month, day_of_month),
        CalendarDateRule::NthWeekdayInMonth {
            nth,
            weekday,
            month,
        } => {
            let first_weekday = run_period_month_weekday_for_month_day(calendar, month, 1)?;
            let first_occurrence = 1
                + (energyplus_weekday_number(weekday) - energyplus_weekday_number(first_weekday))
                    .rem_euclid(7) as u32;
            let day_of_month = first_occurrence + 7 * nth.saturating_sub(1);
            let weather_shape_year = weather_shape_year(weather_effective_leap_year);
            if nth == 0 || day_of_month > days_in_month(weather_shape_year, month) {
                return Err(CalendarRuleResolutionError::NthWeekdayDoesNotExist {
                    nth,
                    weekday,
                    month,
                });
            }
            (month, day_of_month)
        }
        CalendarDateRule::LastWeekdayInMonth { weekday, month } => {
            let first_weekday = run_period_month_weekday_for_month_day(calendar, month, 1)?;
            let mut day_of_month = 1
                + (energyplus_weekday_number(weekday) - energyplus_weekday_number(first_weekday))
                    .rem_euclid(7) as u32;
            let last_day_of_month =
                days_in_month(weather_shape_year(weather_effective_leap_year), month);
            while day_of_month + 7 <= last_day_of_month {
                day_of_month += 7;
            }
            (month, day_of_month)
        }
    };
    let day_of_year =
        weather_effective_day_of_year(month, day_of_month, weather_effective_leap_year).ok_or(
            CalendarRuleResolutionError::InvalidDate {
                month,
                day_of_month,
            },
        )?;
    Ok(ResolvedCalendarRuleDate {
        month,
        day_of_month,
        day_of_year,
    })
}

fn run_period_month_weekday_for_month_day(
    calendar: &ResolvedRunPeriodCalendar,
    month: u32,
    day_of_month: u32,
) -> Result<DayOfWeek, CalendarRuleResolutionError> {
    // EnergyPlus seeds MonWeekDay before environment-specific LeapYearAdd is
    // applied. Nth/Last rules therefore use this non-leap weekday projection.
    let start_day_of_year =
        weather_effective_day_of_year(calendar.start_month, calendar.start_day_of_month, false)
            .ok_or(CalendarRuleResolutionError::InvalidDate {
                month: calendar.start_month,
                day_of_month: calendar.start_day_of_month,
            })?;
    let target_day_of_year = weather_effective_day_of_year(month, day_of_month, false).ok_or(
        CalendarRuleResolutionError::InvalidDate {
            month,
            day_of_month,
        },
    )?;
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
    day_of_year(
        weather_shape_year(weather_effective_leap_year),
        month,
        day_of_month,
    )
}

const fn weather_shape_year(weather_effective_leap_year: bool) -> u32 {
    if weather_effective_leap_year {
        DEFAULT_LEAP_RUN_PERIOD_YEAR
    } else {
        DEFAULT_RUN_PERIOD_YEAR
    }
}
