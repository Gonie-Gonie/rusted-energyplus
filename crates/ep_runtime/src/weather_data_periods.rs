//! EPW `DATA PERIODS` header parsing.

use super::{EpwDataPeriod, EpwDataPeriodDate, EpwDataPeriods, EpwError, epw_field, parse_epw_u32};
use ep_model::DayOfWeek;

const EPW_DATA_PERIODS_HEADER: &str = "DATA PERIODS";

pub(super) fn parse_epw_data_periods(contents: &str) -> Result<(EpwDataPeriods, usize), EpwError> {
    let lines = contents.lines().collect::<Vec<_>>();
    let (line_index, line) = lines
        .iter()
        .enumerate()
        .find(|(_line_index, line)| {
            line.split(',')
                .next()
                .is_some_and(|header| header.trim().eq_ignore_ascii_case(EPW_DATA_PERIODS_HEADER))
        })
        .ok_or(EpwError::MissingHeader {
            header: EPW_DATA_PERIODS_HEADER,
        })?;
    let line_number = line_index + 1;
    let mut last_header_line_index = line_index;
    let mut fields = line_fields(line);
    while fields.len() < 3 {
        last_header_line_index += 1;
        let continuation = lines
            .get(last_header_line_index)
            .ok_or(EpwError::MissingField {
                line: line_number,
                field: "records per hour",
            })?;
        fields.extend(line_fields(continuation));
    }
    let period_count = parse_epw_u32(&fields, line_number, 1, "number of data periods")?;
    if period_count == 0 {
        return Err(EpwError::InvalidValue {
            line: line_number,
            field: "number of data periods",
            value: "0".to_string(),
        });
    }
    let records_per_hour = parse_epw_u32(&fields, line_number, 2, "records per hour")?;
    if records_per_hour == 0 {
        return Err(EpwError::InvalidValue {
            line: line_number,
            field: "records per hour",
            value: "0".to_string(),
        });
    }

    let required_field_count = 3 + period_count as usize * 4;
    while fields.len() < required_field_count {
        last_header_line_index += 1;
        let continuation = lines
            .get(last_header_line_index)
            .ok_or(EpwError::MissingField {
                line: line_number,
                field: "data period end date",
            })?;
        fields.extend(line_fields(continuation));
    }

    let mut periods = Vec::with_capacity(period_count as usize);
    for period_index in 0..period_count as usize {
        let field_offset = 3 + period_index * 4;
        let name = epw_field(&fields, line_number, field_offset, "data period name")?
            .trim()
            .to_string();
        let start_day_of_week_text = epw_field(
            &fields,
            line_number,
            field_offset + 1,
            "data period start day of week",
        )?;
        let start_day_of_week =
            parse_day_of_week(start_day_of_week_text).ok_or_else(|| EpwError::InvalidValue {
                line: line_number,
                field: "data period start day of week",
                value: start_day_of_week_text.to_string(),
            })?;
        let start_text = epw_field(
            &fields,
            line_number,
            field_offset + 2,
            "data period start date",
        )?;
        let end_text = epw_field(
            &fields,
            line_number,
            field_offset + 3,
            "data period end date",
        )?;
        let start_date = parse_data_period_date(start_text, line_number, "data period start date")?;
        let mut end_date = parse_data_period_date(end_text, line_number, "data period end date")?;
        if start_date.year.is_some() && end_date.year.is_none() {
            end_date.year = start_date.year;
        }
        periods.push(EpwDataPeriod {
            name,
            start_day_of_week,
            start_date,
            end_date,
        });
    }

    Ok((
        EpwDataPeriods {
            records_per_hour,
            periods,
        },
        last_header_line_index + 1,
    ))
}

fn line_fields(line: &str) -> Vec<&str> {
    if line.trim().is_empty() {
        return Vec::new();
    }
    let mut fields = line.split(',').collect::<Vec<_>>();
    if line.trim_end().ends_with(',') && fields.last().is_some_and(|field| field.is_empty()) {
        fields.pop();
    }
    fields
}

fn parse_day_of_week(value: &str) -> Option<DayOfWeek> {
    let value = value.trim();
    if value.eq_ignore_ascii_case("Monday") {
        Some(DayOfWeek::Monday)
    } else if value.eq_ignore_ascii_case("Tuesday") {
        Some(DayOfWeek::Tuesday)
    } else if value.eq_ignore_ascii_case("Wednesday") {
        Some(DayOfWeek::Wednesday)
    } else if value.eq_ignore_ascii_case("Thursday") {
        Some(DayOfWeek::Thursday)
    } else if value.eq_ignore_ascii_case("Friday") {
        Some(DayOfWeek::Friday)
    } else if value.eq_ignore_ascii_case("Saturday") {
        Some(DayOfWeek::Saturday)
    } else if value.eq_ignore_ascii_case("Sunday") {
        Some(DayOfWeek::Sunday)
    } else {
        None
    }
}

fn parse_data_period_date(
    value: &str,
    line: usize,
    field: &'static str,
) -> Result<EpwDataPeriodDate, EpwError> {
    let parts = value
        .split(|character: char| character.is_whitespace() || matches!(character, '/' | ':' | '-'))
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if !(2..=3).contains(&parts.len()) {
        return Err(invalid_date_value(line, field, value));
    }
    let (year, month, day) = if parts.len() == 2 {
        match (parts[0].parse::<u32>().ok(), parts[1].parse::<u32>().ok()) {
            (Some(month), Some(day)) => (None, month, day),
            (Some(day), None) => (
                None,
                parse_month_name(parts[1]).ok_or_else(|| invalid_date_value(line, field, value))?,
                day,
            ),
            (None, Some(day)) => (
                None,
                parse_month_name(parts[0]).ok_or_else(|| invalid_date_value(line, field, value))?,
                day,
            ),
            (None, None) => return Err(invalid_date_value(line, field, value)),
        }
    } else {
        let first = parse_date_component(parts[0], line, field, value)?;
        let second = parse_date_component(parts[1], line, field, value)?;
        let third = parse_date_component(parts[2], line, field, value)?;
        if first > 100 {
            (Some(first), second, third)
        } else if third > 100 {
            (Some(third), first, second)
        } else {
            return Err(invalid_date_value(line, field, value));
        }
    };
    if !valid_date(year, month, day) {
        return Err(invalid_date_value(line, field, value));
    }
    Ok(EpwDataPeriodDate { year, month, day })
}

fn parse_month_name(value: &str) -> Option<u32> {
    const MONTHS: [&str; 12] = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    let prefix = value.get(..3)?;
    MONTHS
        .iter()
        .position(|month| prefix.eq_ignore_ascii_case(month))
        .and_then(|index| u32::try_from(index + 1).ok())
}

fn parse_date_component(
    component: &str,
    line: usize,
    field: &'static str,
    whole_value: &str,
) -> Result<u32, EpwError> {
    component
        .parse::<u32>()
        .map_err(|_error| invalid_date_value(line, field, whole_value))
}

fn invalid_date_value(line: usize, field: &'static str, value: &str) -> EpwError {
    EpwError::InvalidValue {
        line,
        field,
        value: value.to_string(),
    }
}

fn valid_date(year: Option<u32>, month: u32, day: u32) -> bool {
    if !(1..=12).contains(&month) || day == 0 {
        return false;
    }
    let leap_year = year.is_none_or(is_leap_year);
    day <= match month {
        2 if leap_year => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}
