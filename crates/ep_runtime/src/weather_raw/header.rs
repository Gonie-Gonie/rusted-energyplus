//! Literal selected ProcessEPWHeader stores and OpenEPlusWeatherFile ordering.
use super::dates::{DateScratch, finish_period, number_with_error, process_date_string};
use super::header_state::{
    DateType, HeaderCallerPreparation, HeaderContextConsumption, RawEpwHeaderError,
    RawEpwHeaderState, defined, source_count, to_source_int,
};
use super::input::{RawEpwInput, erase_field};

/// Original EPW header enum order, separate from the whole-file token gate.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderKind {
    /// LOCATION.
    Location = 0,
    /// DESIGN CONDITIONS.
    DesignConditions = 1,
    /// TYPICAL/EXTREME PERIODS.
    TypicalExtremePeriods = 2,
    /// GROUND TEMPERATURES.
    GroundTemperatures = 3,
    /// HOLIDAYS/DAYLIGHT SAVING (the source token also matches a trailing S).
    HolidaysDst = 4,
    /// COMMENTS 1.
    Comments1 = 5,
    /// COMMENTS 2.
    Comments2 = 6,
    /// DATA PERIODS.
    DataPeriods = 7,
}

const HEADERS: [(HeaderKind, &str); 8] = [
    (HeaderKind::Location, "LOCATION"),
    (HeaderKind::DesignConditions, "DESIGN CONDITIONS"),
    (HeaderKind::TypicalExtremePeriods, "TYPICAL/EXTREME PERIODS"),
    (HeaderKind::GroundTemperatures, "GROUND TEMPERATURES"),
    (HeaderKind::HolidaysDst, "HOLIDAYS/DAYLIGHT SAVING"),
    (HeaderKind::Comments1, "COMMENTS 1"),
    (HeaderKind::Comments2, "COMMENTS 2"),
    (HeaderKind::DataPeriods, "DATA PERIODS"),
];

/// Process one explicitly selected header, destructively consuming line/stream.
///
/// Typical/Ground consumption is retained in `context`; their physical computed
/// fields and the original global error manager's text are outside this owner.
pub fn process_epw_header(
    kind: HeaderKind,
    line: &mut String,
    errors: &mut bool,
    state: &mut RawEpwHeaderState,
    input: &mut RawEpwInput,
    caller: HeaderCallerPreparation,
    context: &mut HeaderContextConsumption,
) -> Result<(), RawEpwHeaderError> {
    if let Some(at) = line.find(',') {
        erase_field(line, at);
    } else if !matches!(kind, HeaderKind::Comments1 | HeaderKind::Comments2) {
        return Err(RawEpwHeaderError::SourceFatal("header-no-comma"));
    }
    match kind {
        HeaderKind::Location => location(line, state, input)?,
        HeaderKind::HolidaysDst => holidays(line, errors, state, input, caller, context)?,
        HeaderKind::DataPeriods => periods(line, errors, state, input)?,
        HeaderKind::TypicalExtremePeriods => context.consume_typical(line, input, errors)?,
        HeaderKind::GroundTemperatures => context.consume_ground(line)?,
        HeaderKind::Comments1 | HeaderKind::Comments2 | HeaderKind::DesignConditions => {}
    }
    Ok(())
}

fn location(
    line: &mut String,
    state: &mut RawEpwHeaderState,
    input: &mut RawEpwInput,
) -> Result<(), RawEpwHeaderError> {
    for field in 1..=9 {
        let end = input.field_end(line)?;
        let token = &line[..end];
        match field {
            1 => state.header_title = token.trim_matches(' ').to_owned(),
            2..=4 => {
                state.header_title = format!(
                    "{} {}",
                    state.header_title.trim_matches(' '),
                    token.trim_matches(' ')
                )
            }
            5 => state
                .header_title
                .push_str(&format!(" WMO#={}", token.trim_matches(' '))),
            6..=9 => {
                let (number, error) = number_with_error(token);
                if !error {
                    match field {
                        6 => state.latitude = number,
                        7 => state.longitude = number,
                        8 => state.time_zone = number,
                        9 => state.elevation = number,
                        _ => {}
                    }
                }
                // Invalid LOCATION numerics retain values and do not set caller errors.
            }
            _ => {}
        }
        erase_field(line, end);
    }
    state.weather_location_title = state.header_title.trim_matches(' ').to_owned();
    Ok(())
}

fn assign_start(state: &mut RawEpwHeaderState, date: DateScratch) -> Result<(), RawEpwHeaderError> {
    state.epw_dst.start_date_type = date.date_type;
    state.epw_dst.start_month = defined(date.month, "unwritten DST start month")?;
    state.epw_dst.start_day = defined(date.day, "unwritten DST start day")?;
    state.epw_dst.start_weekday = defined(date.weekday, "unwritten active DST start weekday")?;
    Ok(())
}

fn holidays(
    line: &mut String,
    errors: &mut bool,
    state: &mut RawEpwHeaderState,
    input: &mut RawEpwInput,
    caller: HeaderCallerPreparation,
    context: &mut HeaderContextConsumption,
) -> Result<(), RawEpwHeaderError> {
    line.make_ascii_uppercase();
    let mut arguments = 4;
    let mut current = 0usize;
    let mut field = 1;
    while field <= arguments {
        let end = input.field_end(line)?;
        let token = &line[..end];
        match field {
            1 => state.allows_leap_years = line.as_bytes().first() == Some(&b'Y'),
            2 => {
                // Source local errflag1 is never observed or subsequently read.
                let mut local_error = false;
                let date = process_date_string(token, false, &mut local_error)?;
                if date.date_type == DateType::Invalid
                    || (date.month == Some(0) && date.day == Some(0))
                {
                    state.epw_daylight_saving = false;
                } else {
                    state.epw_daylight_saving = true;
                    assign_start(state, date)?;
                }
            }
            3 => {
                let date = process_date_string(token, false, errors)?;
                if state.epw_daylight_saving {
                    if date.date_type != DateType::Invalid {
                        state.epw_dst.end_date_type = date.date_type;
                        state.epw_dst.end_month = defined(date.month, "unwritten DST end month")?;
                        state.epw_dst.end_day = defined(date.day, "unwritten DST end day")?;
                        state.epw_dst.end_weekday =
                            defined(date.weekday, "unwritten active DST end weekday")?;
                    } else {
                        state.epw_daylight_saving = false;
                    }
                    state.dst = state.epw_dst.clone();
                }
            }
            4 => {
                let count = to_source_int(number_with_error(token).0)?;
                state.number_special_days = count
                    .checked_add(caller.run_period_control_special_days)
                    .ok_or(RawEpwHeaderError::OutsideBoundedDomain(
                        "original special-day count arithmetic",
                    ))?;
                state
                    .special_days
                    .allocate(source_count(state.number_special_days)?);
                arguments = count
                    .checked_mul(2)
                    .and_then(|value| value.checked_add(4))
                    .ok_or(RawEpwHeaderError::OutsideBoundedDomain(
                        "original holiday field-count arithmetic",
                    ))?;
            }
            _ if field % 2 != 0 => {
                current += 1;
                if current > source_count(state.number_special_days)? {
                    *errors = true;
                } else if let Some(day) = state.special_days.values.get_mut(current - 1) {
                    day.name = token.to_owned();
                } else {
                    return Err(RawEpwHeaderError::OutsideBoundedDomain(
                        "original holiday storage index",
                    ));
                }
            }
            _ => {
                if current <= source_count(state.number_special_days)? {
                    let date = process_date_string(token, false, errors)?;
                    let day = state
                        .special_days
                        .values
                        .get_mut(current.saturating_sub(1))
                        .ok_or(RawEpwHeaderError::OutsideBoundedDomain(
                            "original holiday storage index",
                        ))?;
                    if date.date_type != DateType::Invalid {
                        day.date_type = date.date_type;
                        day.month = defined(date.month, "unwritten holiday month")?;
                        day.day = defined(date.day, "unwritten holiday day")?;
                        if date.date_type == DateType::MonthDay {
                            day.weekday = 0;
                            day.compressed_date = day.month * 32 + day.day;
                        } else {
                            day.weekday = defined(date.weekday, "unwritten holiday weekday")?;
                            day.compressed_date = 0;
                        }
                        day.duration = 1;
                        day.day_type = 1;
                        day.weather_file = true;
                    } else {
                        *errors = true;
                    }
                }
            }
        }
        erase_field(line, end);
        field += 1;
    }
    if context
        .typical_declared_count
        .is_some_and(|count| count > 0)
    {
        context
            .unimplemented_computed_state
            .push("HolidaysDST:TypicalExtremePeriod ordinal/date totals");
    }
    Ok(())
}

const WEEKDAY_NAMES: [&str; 13] = [
    "UNUSED",
    "SUNDAY",
    "MONDAY",
    "TUESDAY",
    "WEDNESDAY",
    "THURSDAY",
    "FRIDAY",
    "SATURDAY",
    "HOLIDAY",
    "SUMMERDESIGNDAY",
    "WINTERDESIGNDAY",
    "CUSTOMDAY1",
    "CUSTOMDAY2",
];

fn periods(
    line: &mut String,
    errors: &mut bool,
    state: &mut RawEpwHeaderState,
    input: &mut RawEpwInput,
) -> Result<(), RawEpwHeaderError> {
    line.make_ascii_uppercase();
    let mut arguments = 2;
    let mut current = 0usize;
    let mut field = 1;
    while field <= arguments {
        let end = input.field_end(line)?;
        let token = &line[..end];
        if field == 1 {
            state.number_data_periods = to_source_int(number_with_error(token).0)?;
            state
                .data_periods
                .allocate(source_count(state.number_data_periods)?);
            arguments = state
                .number_data_periods
                .checked_mul(4)
                .and_then(|value| value.checked_add(2))
                .ok_or(RawEpwHeaderError::OutsideBoundedDomain(
                    "original period field-count arithmetic",
                ))?;
            if state.number_data_periods > 0 {
                for period in &mut state.data_periods.values {
                    period.number_days = 0;
                }
            }
        } else if field == 2 {
            state.intervals_per_hour = to_source_int(number_with_error(token).0)?;
        } else {
            let part = (field - 3) % 4;
            if part == 0 {
                current += 1;
            }
            if current > source_count(state.number_data_periods)? {
                *errors = true;
            } else {
                let period = state
                    .data_periods
                    .values
                    .get_mut(current.saturating_sub(1))
                    .ok_or(RawEpwHeaderError::OutsideBoundedDomain(
                        "original period storage index",
                    ))?;
                match part {
                    0 => period.name = token.to_owned(),
                    1 => {
                        period.day_of_week = token.to_owned();
                        period.weekday = WEEKDAY_NAMES
                            .iter()
                            .position(|name| *name == token)
                            .map_or(-1, |index| index as i32);
                        if !(1..=7).contains(&period.weekday) {
                            *errors = true;
                        }
                    }
                    2 | 3 => {
                        let date = process_date_string(token, true, errors)?;
                        if date.date_type == DateType::MonthDay {
                            if part == 2 {
                                period.start_month =
                                    defined(date.month, "unwritten period start month")?;
                                period.start_day = defined(date.day, "unwritten period start day")?;
                                period.start_year =
                                    defined(date.year, "unwritten period start year")?;
                                if period.start_year != 0 {
                                    period.has_year_data = true;
                                }
                            } else {
                                period.end_month =
                                    defined(date.month, "unwritten period end month")?;
                                period.end_day = defined(date.day, "unwritten period end day")?;
                                period.end_year = defined(date.year, "unwritten period end year")?;
                                if period.end_year == 0 && period.has_year_data {
                                    period.end_year = period.start_year;
                                }
                            }
                        } else {
                            *errors = true;
                        }
                        if part == 3 {
                            finish_period(period, *errors, &state.month_ends, state.leap_year_add)?;
                        }
                    }
                    _ => {}
                }
            }
        }
        erase_field(line, end);
        field += 1;
    }
    Ok(())
}

/// Open supplied bytes and process exactly eight ordered, case-sensitive headers.
///
/// The first non-ASCII-space position must equal the expected header's first
/// occurrence. A mismatched line is skipped; this operation never searches ahead.
pub fn open_epw_header(
    input: &mut RawEpwInput,
    state: &mut RawEpwHeaderState,
    errors: &mut bool,
    caller: HeaderCallerPreparation,
    context: &mut HeaderContextConsumption,
) -> Result<(), RawEpwHeaderError> {
    input.reopen();
    for (kind, expected) in HEADERS {
        let read = input.read_line()?;
        if read.eof {
            return Err(RawEpwHeaderError::SourceFatal("unexpected-header-EOF"));
        }
        if read.data.as_bytes().last() == Some(&0) {
            return Err(RawEpwHeaderError::SourceFatal("binary-Unicode-header-end"));
        }
        let first = read.data.as_bytes().iter().position(|byte| *byte != b' ');
        if first != read.data.find(expected) {
            continue;
        }
        let mut line = read.data;
        process_epw_header(kind, &mut line, errors, state, input, caller, context)?;
    }
    Ok(())
}
