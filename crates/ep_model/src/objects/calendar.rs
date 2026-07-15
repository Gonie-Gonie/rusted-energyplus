//! Calendar-control records shared by model intake and weather metadata.

use crate::{DayOfWeek, NormalizedName, RunPeriodSpecialDayId};

/// One EnergyPlus month/day or weekday-in-month date rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarDateRule {
    /// A fixed month and day, such as `3/10` or `March 10`.
    MonthDay {
        /// Month number, 1-12.
        month: u32,
        /// Day of month.
        day_of_month: u32,
    },
    /// The nth occurrence of a weekday in a month, such as `2nd Sunday in March`.
    NthWeekdayInMonth {
        /// One-based weekday occurrence, 1-5.
        nth: u32,
        /// Weekday selected by the rule.
        weekday: DayOfWeek,
        /// Month number, 1-12.
        month: u32,
    },
    /// The final occurrence of a weekday in a month, such as `Last Sunday in November`.
    LastWeekdayInMonth {
        /// Weekday selected by the rule.
        weekday: DayOfWeek,
        /// Month number, 1-12.
        month: u32,
    },
}

/// The unique input-file daylight-saving period applied to weather run periods.
///
/// EnergyPlus `RunPeriodControl:DaylightSavingTime` takes precedence over a
/// daylight-saving period declared by the weather file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunPeriodDaylightSavingTime {
    /// Calendar rule used to resolve the first daylight-saving day.
    pub start_date: CalendarDateRule,
    /// Calendar rule used to resolve the final daylight-saving day.
    pub end_date: CalendarDateRule,
}

/// Schedule day type selected by `RunPeriodControl:SpecialDays`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpecialDayType {
    /// Holiday schedule day.
    #[default]
    Holiday,
    /// Summer design-day schedule day.
    SummerDesignDay,
    /// Winter design-day schedule day.
    WinterDesignDay,
    /// First user-defined special schedule day.
    CustomDay1,
    /// Second user-defined special schedule day.
    CustomDay2,
}

/// One typed `RunPeriodControl:SpecialDays` object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunPeriodSpecialDay {
    /// Typed ID.
    pub id: RunPeriodSpecialDayId,
    /// Object name.
    pub name: NormalizedName,
    /// Calendar rule used to resolve the first special day.
    pub start_date: CalendarDateRule,
    /// Number of consecutive special days, 1-366.
    pub duration_days: u32,
    /// Schedule day type assigned to the resolved range.
    pub special_day_type: SpecialDayType,
}

/// Parses the month/day, Nth-weekday, and last-weekday forms used by
/// EnergyPlus `DetermineDateTokens`.
///
/// Nonzero single-number Julian dates remain outside this parser.
#[must_use]
pub fn parse_calendar_date_rule(value: &str) -> Option<CalendarDateRule> {
    let normalized = value
        .trim()
        .to_ascii_uppercase()
        .replace(['/', ':', '-'], " ");
    let tokens = normalized
        .split_whitespace()
        .filter(|token| !matches!(*token, "IN" | "OF"))
        .collect::<Vec<_>>();
    parse_month_day_rule(&tokens).or_else(|| parse_weekday_in_month_rule(&tokens))
}

fn parse_month_day_rule(tokens: &[&str]) -> Option<CalendarDateRule> {
    if tokens.len() != 2 {
        return None;
    }

    let first_number = parse_ordinal_number(tokens[0]);
    let second_number = parse_ordinal_number(tokens[1]);
    let first_month = parse_month_name(tokens[0]);
    let second_month = parse_month_name(tokens[1]);
    let (month, day_of_month) = match (first_number, second_number, first_month, second_month) {
        (Some(month), Some(day), None, None) => (month, day),
        (None, Some(day), Some(month), None) => (month, day),
        (Some(day), None, None, Some(month)) => (month, day),
        _ => return None,
    };
    valid_month_day(month, day_of_month).then_some(CalendarDateRule::MonthDay {
        month,
        day_of_month,
    })
}

fn parse_weekday_in_month_rule(tokens: &[&str]) -> Option<CalendarDateRule> {
    if tokens.len() != 3 {
        return None;
    }

    let (weekday, month) = parse_weekday_and_month(tokens[1], tokens[2])?;
    if tokens[0] == "LAST" {
        return Some(CalendarDateRule::LastWeekdayInMonth { weekday, month });
    }
    let nth = parse_ordinal_number(tokens[0])?;
    (1..=5)
        .contains(&nth)
        .then_some(CalendarDateRule::NthWeekdayInMonth {
            nth,
            weekday,
            month,
        })
}

fn parse_weekday_and_month(first: &str, second: &str) -> Option<(DayOfWeek, u32)> {
    match (
        parse_weekday(first),
        parse_month_name(first),
        parse_weekday(second),
        parse_month_name(second),
    ) {
        (Some(weekday), None, None, Some(month)) | (None, Some(month), Some(weekday), None) => {
            Some((weekday, month))
        }
        _ => None,
    }
}

fn parse_ordinal_number(token: &str) -> Option<u32> {
    let numeric_end = token
        .char_indices()
        .find_map(|(index, character)| (!character.is_ascii_digit()).then_some(index))
        .unwrap_or(token.len());
    if numeric_end == 0 {
        return None;
    }
    let suffix = &token[numeric_end..];
    if !suffix.is_empty() && !matches!(suffix, "ST" | "ND" | "RD" | "TH") {
        return None;
    }
    token[..numeric_end].parse().ok()
}

fn parse_month_name(token: &str) -> Option<u32> {
    let prefix = token.get(..3)?;
    [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ]
    .iter()
    .position(|month| prefix == *month)
    .and_then(|index| u32::try_from(index + 1).ok())
}

fn parse_weekday(token: &str) -> Option<DayOfWeek> {
    match token.get(..3)? {
        "SUN" => Some(DayOfWeek::Sunday),
        "MON" => Some(DayOfWeek::Monday),
        "TUE" => Some(DayOfWeek::Tuesday),
        "WED" => Some(DayOfWeek::Wednesday),
        "THU" => Some(DayOfWeek::Thursday),
        "FRI" => Some(DayOfWeek::Friday),
        "SAT" => Some(DayOfWeek::Saturday),
        _ => None,
    }
}

fn valid_month_day(month: u32, day_of_month: u32) -> bool {
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 29,
        _ => return false,
    };
    (1..=days_in_month).contains(&day_of_month)
}
