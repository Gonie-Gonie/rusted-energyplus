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

/// One holiday declared by an EPW `HOLIDAYS/DAYLIGHT SAVINGS` header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EpwHoliday {
    /// Weather-file holiday name, normalized like EnergyPlus' uppercased header.
    pub name: String,
    /// Calendar rule selecting the holiday date.
    pub date: EpwCalendarDateRule,
}

/// Calendar policy carried by an EPW weather file header.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EpwCalendarMetadata {
    /// Whether the EPW `Leap Year Observed` field starts with `Y` after trimming.
    pub leap_year_observed: bool,
    /// Weather-file daylight-saving range, or `None` when the header uses `0`.
    pub daylight_saving_period: Option<EpwDaylightSavingPeriod>,
    /// Weather-file holidays in source header order.
    pub holidays: Vec<EpwHoliday>,
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
    let holiday_count_value = epw_field(&fields, line_number, 4, "number of holidays")?;
    let holiday_count = parse_holiday_count(holiday_count_value, line_number)?;
    let holiday_fields = fields.len().saturating_sub(5);
    let available_holiday_pairs = holiday_fields / 2;
    if holiday_count > available_holiday_pairs {
        return Err(EpwError::MissingField {
            line: line_number,
            field: if holiday_fields.is_multiple_of(2) {
                "holiday name"
            } else {
                "holiday date"
            },
        });
    }
    let mut holidays = Vec::with_capacity(holiday_count);
    for holiday_index in 0..holiday_count {
        let name_field_index = 5 + holiday_index * 2;
        let date_field_index = name_field_index + 1;
        let name = epw_field(&fields, line_number, name_field_index, "holiday name")?
            .trim()
            .to_ascii_uppercase();
        let date_value = epw_field(&fields, line_number, date_field_index, "holiday date")?;
        let date = parse_calendar_date_rule(date_value, line_number, "holiday date")?
            .ok_or_else(|| invalid_date_rule(line_number, "holiday date", date_value))?;
        holidays.push(EpwHoliday { name, date });
    }

    Ok(EpwCalendarMetadata {
        leap_year_observed,
        daylight_saving_period,
        holidays,
    })
}

fn parse_holiday_count(value: &str, line: usize) -> Result<usize, EpwError> {
    let parsed = value
        .trim()
        .parse::<f64>()
        .map_err(|_| EpwError::InvalidNumber {
            line,
            field: "number of holidays",
            value: value.to_string(),
        })?;
    if !parsed.is_finite() || parsed < 0.0 || parsed.fract() != 0.0 || parsed > usize::MAX as f64 {
        return Err(EpwError::InvalidNumber {
            line,
            field: "number of holidays",
            value: value.to_string(),
        });
    }
    Ok(parsed as usize)
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
