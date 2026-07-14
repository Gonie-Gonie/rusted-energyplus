//! EPW calendar-policy header parsing.

use super::{EPW_HEADER_LINE_COUNT, EpwError, epw_field};
use ep_model::DayOfWeek;

const EPW_HOLIDAYS_DAYLIGHT_SAVINGS_HEADER: &str = "HOLIDAYS/DAYLIGHT SAVINGS";
const EPW_HOLIDAYS_DAYLIGHT_SAVING_HEADER_PREFIX: &str = "HOLIDAYS/DAYLIGHT SAVING";

/// One calendar date rule accepted by the EPW holidays/daylight-saving header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpwCalendarDateRule {
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

/// Daylight-saving period declared by an EPW header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EpwDaylightSavingPeriod {
    /// First day on which daylight saving is active.
    pub start: EpwCalendarDateRule,
    /// Final day on which daylight saving is active.
    pub end: EpwCalendarDateRule,
}

/// Calendar policy carried by an EPW weather file header.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EpwCalendarMetadata {
    /// Whether the EPW `Leap Year Observed` field starts with `Y` after trimming.
    pub leap_year_observed: bool,
    /// Weather-file daylight-saving range, or `None` when the header uses `0`.
    pub daylight_saving_period: Option<EpwDaylightSavingPeriod>,
}

pub(super) fn parse_epw_calendar_metadata(contents: &str) -> Result<EpwCalendarMetadata, EpwError> {
    let (line_index, line) = contents
        .lines()
        .take(EPW_HEADER_LINE_COUNT)
        .enumerate()
        .find(|(_line_index, line)| {
            line.split(',')
                .next()
                .is_some_and(holidays_daylight_saving_header_matches)
        })
        .ok_or(EpwError::MissingHeader {
            header: EPW_HOLIDAYS_DAYLIGHT_SAVINGS_HEADER,
        })?;
    let line_number = line_index + 1;
    let fields = line.split(',').collect::<Vec<_>>();
    let value = epw_field(&fields, line_number, 1, "leap year observed")?;
    let leap_year_observed = value
        .trim()
        .chars()
        .next()
        .ok_or_else(|| EpwError::InvalidValue {
            line: line_number,
            field: "leap year observed",
            value: value.to_string(),
        })?
        .eq_ignore_ascii_case(&'y');
    let daylight_saving_start_value =
        epw_field(&fields, line_number, 2, "daylight saving start date")?;
    let daylight_saving_start = parse_calendar_date_rule(
        daylight_saving_start_value,
        line_number,
        "daylight saving start date",
    )?;
    let daylight_saving_end_value = epw_field(&fields, line_number, 3, "daylight saving end date")?;
    let daylight_saving_end = parse_calendar_date_rule(
        daylight_saving_end_value,
        line_number,
        "daylight saving end date",
    )?;
    let daylight_saving_period = match (daylight_saving_start, daylight_saving_end) {
        (None, None) => None,
        (Some(start), Some(end)) => Some(EpwDaylightSavingPeriod { start, end }),
        (None, Some(_)) => {
            return Err(invalid_date_rule(
                line_number,
                "daylight saving start date",
                daylight_saving_start_value,
            ));
        }
        (Some(_), None) => {
            return Err(invalid_date_rule(
                line_number,
                "daylight saving end date",
                daylight_saving_end_value,
            ));
        }
    };

    Ok(EpwCalendarMetadata {
        leap_year_observed,
        daylight_saving_period,
    })
}

fn parse_calendar_date_rule(
    value: &str,
    line: usize,
    field: &'static str,
) -> Result<Option<EpwCalendarDateRule>, EpwError> {
    let trimmed = value.trim();
    if trimmed.parse::<f64>().is_ok_and(|number| number == 0.0) {
        return Ok(None);
    }

    let normalized = trimmed.to_ascii_uppercase().replace(['/', ':', '-'], " ");
    let tokens = normalized
        .split_whitespace()
        .filter(|token| !matches!(*token, "IN" | "OF"))
        .collect::<Vec<_>>();

    let rule = parse_month_day_rule(&tokens)
        .or_else(|| parse_weekday_in_month_rule(&tokens))
        .ok_or_else(|| invalid_date_rule(line, field, value))?;
    Ok(Some(rule))
}

fn parse_month_day_rule(tokens: &[&str]) -> Option<EpwCalendarDateRule> {
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
    valid_month_day(month, day_of_month).then_some(EpwCalendarDateRule::MonthDay {
        month,
        day_of_month,
    })
}

fn parse_weekday_in_month_rule(tokens: &[&str]) -> Option<EpwCalendarDateRule> {
    if tokens.len() != 3 {
        return None;
    }

    let (weekday, month) = parse_weekday_and_month(tokens[1], tokens[2])?;
    if tokens[0].starts_with("LAST") {
        return Some(EpwCalendarDateRule::LastWeekdayInMonth { weekday, month });
    }
    let nth = parse_ordinal_number(tokens[0])?;
    (1..=5)
        .contains(&nth)
        .then_some(EpwCalendarDateRule::NthWeekdayInMonth {
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

fn invalid_date_rule(line: usize, field: &'static str, value: &str) -> EpwError {
    EpwError::InvalidValue {
        line,
        field,
        value: value.to_string(),
    }
}

fn holidays_daylight_saving_header_matches(header: &str) -> bool {
    header
        .trim()
        .get(..EPW_HOLIDAYS_DAYLIGHT_SAVING_HEADER_PREFIX.len())
        .is_some_and(|prefix| {
            prefix.eq_ignore_ascii_case(EPW_HOLIDAYS_DAYLIGHT_SAVING_HEADER_PREFIX)
        })
}
