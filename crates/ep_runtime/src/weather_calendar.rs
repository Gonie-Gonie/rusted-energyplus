//! EPW calendar-policy header parsing.

use super::{EPW_HEADER_LINE_COUNT, EpwError, epw_field};
pub use ep_model::CalendarDateRule as EpwCalendarDateRule;
use ep_model::parse_calendar_date_rule as parse_shared_calendar_date_rule;

const EPW_HOLIDAYS_DAYLIGHT_SAVINGS_HEADER: &str = "HOLIDAYS/DAYLIGHT SAVINGS";
const EPW_HOLIDAYS_DAYLIGHT_SAVING_HEADER_PREFIX: &str = "HOLIDAYS/DAYLIGHT SAVING";

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

    let rule = parse_shared_calendar_date_rule(trimmed)
        .ok_or_else(|| invalid_date_rule(line, field, value))?;
    Ok(Some(rule))
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
