//! Source date-token classifications and conditional period-calendar stores.
use super::header_state::{
    DataPeriod, DateType, RawEpwHeaderError, strip_ascii_spaces, to_source_int,
};
use super::lexical::process_number;

/// None identifies an original local that ProcessDateString did not write.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct DateScratch {
    pub date_type: DateType,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub weekday: Option<i32>,
    pub year: Option<i32>,
}

pub(super) fn number_with_error(text: &str) -> (f64, bool) {
    match process_number(text) {
        Ok(value) => (value, false),
        Err(()) => (0.0, true),
    }
}

fn inverse_ordinal(number: i32) -> (i32, i32) {
    const END: [i32; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    let mut month = 1;
    while month <= 12 {
        if number > END[month - 1] && number <= END[month] {
            break;
        }
        month += 1;
    }
    // InvOrdinalDay reaches month 13 for 366 with its explicit leap argument 0.
    (month as i32, number - END[month - 1])
}

fn lookup_prefix(text: &str, items: &[&str]) -> i32 {
    // FindItemInList uses exact equality. Header callers uppercase before this.
    items
        .iter()
        .position(|item| text.get(..3) == Some(*item))
        .map_or(0, |index| index as i32 + 1)
}

fn validate_month_day(day: i32, month: i32, internal: &mut bool) {
    const ENDS: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    *internal = !(1..=12).contains(&month);
    if !*internal {
        *internal = day < 1 || day > ENDS[month as usize - 1];
    }
}

pub(super) fn process_date_string(
    text: &str,
    want_year: bool,
    errors: &mut bool,
) -> Result<DateScratch, RawEpwHeaderError> {
    let (value, numeric_error) = number_with_error(text);
    let first = to_source_int(value)?;
    let mut out = DateScratch::default();
    if !numeric_error {
        if first == 0 {
            out.month = Some(0);
            out.day = Some(0);
            out.date_type = DateType::MonthDay;
        } else if !(0..=366).contains(&first) {
            *errors = true;
        } else {
            let (month, day) = inverse_ordinal(first);
            out.month = Some(month);
            out.day = Some(day);
            out.date_type = DateType::LastDayInMonth;
        }
        return Ok(out);
    }
    let mut current = text.replace(['/', ':', '-'], " ");
    let mut weekday_form = false;
    for token in ["ST ", "ND ", "RD ", "TH ", "OF ", "IN "] {
        while let Some(at) = current.find(token) {
            current.replace_range(at..at + 2, "  ");
            weekday_form = true;
        }
    }
    strip_ascii_spaces(&mut current);
    let mut fields = Vec::new();
    while fields.len() < 3 && !current.is_empty() {
        let at = current.find(' ').map_or(current.len(), |position| position);
        fields.push(current[..at].to_owned());
        current.drain(..at);
        strip_ascii_spaces(&mut current);
    }
    let mut tokens = DateTokens::default();
    if !current.is_empty() {
        *errors = true;
    } else {
        match fields.as_slice() {
            [first, second] => tokens.two_fields(first, second, errors)?,
            [first, second, third] if weekday_form => {
                tokens.weekday_fields(first, second, third)?
            }
            [first, second, third] => tokens.year_fields(first, second, third)?,
            _ => *errors = true,
        }
    }
    if tokens.internal_error {
        tokens.date_type = DateType::Invalid;
        *errors = true;
    }
    out.date_type = tokens.date_type;
    if want_year {
        out.year = Some(tokens.year);
    }
    match out.date_type {
        DateType::MonthDay => {
            out.day = Some(tokens.day);
            out.month = Some(tokens.month);
        }
        DateType::NthDayInMonth | DateType::LastDayInMonth => {
            out.day = Some(tokens.day);
            out.month = Some(tokens.month);
            out.weekday = Some(tokens.weekday);
        }
        DateType::Invalid => {}
    }
    Ok(out)
}

#[derive(Default)]
struct DateTokens {
    day: i32,
    month: i32,
    weekday: i32,
    year: i32,
    date_type: DateType,
    internal_error: bool,
}
const MONTHS: [&str; 12] = [
    "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
];
const WEEKDAYS: [&str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

impl DateTokens {
    fn two_fields(
        &mut self,
        first: &str,
        second: &str,
        errors: &mut bool,
    ) -> Result<(), RawEpwHeaderError> {
        let (a, bad_a) = number_with_error(first);
        let (b, bad_b) = number_with_error(second);
        let a = to_source_int(a)?;
        let b = to_source_int(b)?;
        if bad_a {
            self.internal_error = bad_b;
            if !bad_b {
                self.day = b;
            }
            self.month = lookup_prefix(first, &MONTHS);
        } else if !bad_b {
            self.month = a;
            self.day = b;
        } else {
            self.day = a;
            self.month = lookup_prefix(second, &MONTHS);
        }
        // ValidateMonthDay overwrites its local error argument, not caller ErrorsFound.
        validate_month_day(self.day, self.month, &mut self.internal_error);
        if self.internal_error {
            *errors = true;
        } else {
            self.date_type = DateType::MonthDay;
        }
        Ok(())
    }

    fn weekday_fields(
        &mut self,
        first: &str,
        second: &str,
        third: &str,
    ) -> Result<(), RawEpwHeaderError> {
        let (a, error) = number_with_error(first);
        if !error {
            self.day = to_source_int(a)?;
            self.date_type = DateType::NthDayInMonth;
            self.internal_error = !(0..=5).contains(&self.day);
        } else if first == "LA" {
            self.date_type = DateType::LastDayInMonth;
        } else {
            // Source reports severe here without writing caller ErrorsFound.
            return Ok(());
        }
        self.weekday = lookup_prefix(second, &WEEKDAYS);
        if self.weekday == 0 {
            self.month = lookup_prefix(second, &MONTHS);
            self.weekday = lookup_prefix(third, &WEEKDAYS);
            self.internal_error |= self.month == 0 || self.weekday == 0;
        } else {
            self.month = lookup_prefix(third, &MONTHS);
            self.internal_error |= self.month == 0;
        }
        Ok(())
    }

    fn year_fields(
        &mut self,
        first: &str,
        second: &str,
        third: &str,
    ) -> Result<(), RawEpwHeaderError> {
        let a = to_source_int(number_with_error(first).0)?;
        let b = to_source_int(number_with_error(second).0)?;
        let c = to_source_int(number_with_error(third).0)?;
        self.date_type = DateType::MonthDay;
        if a > 100 {
            self.year = a;
            self.month = b;
            self.day = c;
        } else if c > 100 {
            self.year = c;
            self.month = a;
            self.day = b;
        }
        // The original three-field route does not call ValidateMonthDay.
        Ok(())
    }
}

fn ordinal_day(month: i32, day: i32, leap: i32) -> i64 {
    const END: [i64; 12] = [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    match month {
        1 => i64::from(day),
        2 => i64::from(day) + END[0],
        3..=12 => i64::from(day) + END[month as usize - 2] + i64::from(leap),
        _ => 0,
    }
}

fn julian_date(year: i32, month: i32, day: i32) -> i64 {
    let (year, month, day) = (i64::from(year), i64::from(month), i64::from(day));
    let l = (month - 14) / 12;
    day - 32075 + 1461 * (year + 4800 + l) / 4 + 367 * (month - 2 - l * 12) / 12
        - 3 * ((year + 4900 + l) / 100) / 4
}

fn checked_int(value: i64) -> Result<i32, RawEpwHeaderError> {
    i32::try_from(value).map_err(|_| {
        RawEpwHeaderError::OutsideBoundedDomain("original signed date arithmetic range")
    })
}

pub(super) fn finish_period(
    period: &mut DataPeriod,
    errors: bool,
    ends: &[i32; 12],
    leap: i32,
) -> Result<(), RawEpwHeaderError> {
    let no_year = period.start_year == 0 || period.end_year == 0;
    let (start, end) = if no_year {
        (
            ordinal_day(period.start_month, period.start_day, leap),
            ordinal_day(period.end_month, period.end_day, leap),
        )
    } else {
        (
            julian_date(period.start_year, period.start_month, period.start_day),
            julian_date(period.end_year, period.end_month, period.end_day),
        )
    };
    period.start_julian_day = checked_int(start)?;
    period.end_julian_day = checked_int(end)?;
    period.number_days = checked_int(if no_year && start > end {
        (365 - start + 1) + (end - 1 + 1)
    } else {
        end - start + 1
    })?;
    period.month_weekdays = [0; 12];
    if !errors {
        setup_weekdays(
            period.start_month,
            period.start_day,
            period.weekday,
            ends,
            leap,
            &mut period.month_weekdays,
        )?;
    }
    Ok(())
}

fn setup_weekdays(
    start_month: i32,
    start_day: i32,
    start_weekday: i32,
    ends: &[i32; 12],
    leap: i32,
    out: &mut [i32; 12],
) -> Result<(), RawEpwHeaderError> {
    if !(1..=12).contains(&start_month) {
        return Err(RawEpwHeaderError::OutsideBoundedDomain(
            "invalid original one-based weekday array index",
        ));
    }
    let month_start = || {
        let mut weekday = i64::from(start_weekday);
        for _ in 1..start_day {
            weekday -= 1;
            if weekday == 0 {
                weekday = 7;
            }
        }
        weekday
    };
    let mut weekday = month_start();
    out[start_month as usize - 1] = checked_int(weekday)?;
    for month in start_month + 1..=12 {
        weekday +=
            i64::from(ends[month as usize - 2]) + if month == 3 { i64::from(leap) } else { 0 };
        while weekday > 7 {
            weekday -= 7;
        }
        out[month as usize - 1] = checked_int(weekday)?;
    }
    if out.contains(&0) {
        weekday = month_start();
        for month in (1..start_month).rev() {
            weekday -= i64::from(ends[month as usize - 1]);
            if month == 2 {
                weekday += i64::from(leap);
            }
            while weekday <= 0 {
                weekday += 7;
            }
            out[month as usize - 1] = checked_int(weekday)?;
        }
    }
    Ok(())
}
