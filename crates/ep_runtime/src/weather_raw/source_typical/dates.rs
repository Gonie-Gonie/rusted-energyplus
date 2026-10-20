//! Genuine General ProcessDateString395-454/Determine456-668/Validate670-703.
//! Native optional/unwritten out arguments remain explicit masks, never zero probes.
use super::super::header_state::{strip_ascii_spaces, to_source_int};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::weather::day::SourceWeatherInitializationError as Error;
use crate::weather::raw::{DateType, RawDayFailure, RawEpwHeaderError};
#[derive(Clone, Debug, Default)]
pub(crate) struct SourceDateScratch {
    pub(crate) date_type: DateType,
    pub(crate) month: Option<i32>,
    pub(crate) day: Option<i32>,
    pub(crate) weekday: Option<i32>,
    pub(crate) year: Option<i32>,
}
fn raw(e: RawEpwHeaderError) -> Error {
    RawDayFailure::Header(e).into()
}
fn number(text: &str) -> Result<(i32, bool), Error> {
    match ep_model::process_source_number_outcome(text) {
        ep_model::SourceNumberOutcome::Finite(value) => {
            Ok((to_source_int(value).map_err(raw)?, false))
        }
        ep_model::SourceNumberOutcome::ErrorZero => Ok((0, true)),
        ep_model::SourceNumberOutcome::NonFinite => {
            Err(raw(RawEpwHeaderError::OutsideBoundedDomain(
                "undefined original non-finite real-to-int conversion",
            )))
        }
    }
}
fn checked(value: Option<i32>) -> Result<i32, Error> {
    value.ok_or_else(|| {
        raw(RawEpwHeaderError::OutsideBoundedDomain(
            "actual General defined signed date arithmetic",
        ))
    })
}
pub(crate) fn source_ordinal_day(month: i32, day: i32, leap: i32) -> Result<i32, Error> {
    const END: [i32; 12] = [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    if month == 1 {
        return Ok(day);
    }
    if month == 2 {
        return checked(day.checked_add(END[0]));
    }
    if (3..=12).contains(&month) {
        return checked(
            day.checked_add(END[month as usize - 2])
                .and_then(|v| v.checked_add(leap)),
        );
    }
    Ok(0)
}
fn inverse_ordinal(number: i32, out: &mut SourceDateScratch) {
    const END: [i32; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    // This genuine caller supplies LeapYr=0 and Number1..366 only.
    let mut month = 1;
    while month <= 12 {
        if number > END[month - 1] && number <= END[month] {
            break;
        }
        month += 1;
    }
    out.month = Some(month as i32);
    out.day = Some(number - END[month - 1]);
}
/// Existing weather callers retain the same genuine backend diagnostic effects.
pub(crate) fn process_source_date_string(
    text: &str,
    out: &mut SourceDateScratch,
    want_year: bool,
    errors: &mut bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    process_source_date_string_with_diagnostic(text, out, want_year, errors, &mut |message| {
        backend.show_severe_error(message).map_err(Error::from)
    })
}
/// Same General child, with the reached diagnostic effect supplied by its actual
/// caller. A caller lacking that owner must stop before any diagnostic effect.
// Preserve Original explicit signed-integer boundary predicates and branch order.
#[allow(clippy::manual_range_contains)]
pub(crate) fn process_source_date_string_with_diagnostic(
    text: &str,
    out: &mut SourceDateScratch,
    want_year: bool,
    errors: &mut bool,
    diagnostic: &mut impl FnMut(&str) -> Result<(), Error>,
) -> Result<(), Error> {
    let (first, numeric_error) = number(text)?;
    out.date_type = DateType::Invalid;
    if !numeric_error {
        if first == 0 {
            out.month = Some(0);
            out.day = Some(0);
            out.date_type = DateType::MonthDay;
        } else if first < 0 || first > 366 {
            diagnostic(&format!("Invalid Julian date Entered={text}"))?;
            *errors = true;
        } else {
            inverse_ordinal(first, out);
            out.date_type = DateType::LastDayInMonth;
        }
    } else {
        let tokens =
            determine_date_tokens(text, want_year, &mut out.date_type, errors, diagnostic)?;
        // Only the nonnumeric route writes PYear, after the child returns.
        if want_year {
            out.year = Some(tokens.year);
        }
        if out.date_type == DateType::MonthDay {
            out.day = Some(tokens.day);
            out.month = Some(tokens.month);
        } else if out.date_type == DateType::NthDayInMonth
            || out.date_type == DateType::LastDayInMonth
        {
            out.day = Some(tokens.day);
            out.month = Some(tokens.month);
            out.weekday = Some(tokens.weekday);
        }
    }
    Ok(())
}
#[derive(Default)]
struct Tokens {
    number: i32,
    day: i32,
    month: i32,
    weekday: i32,
    year: i32,
}
fn lookup_prefix(text: &str, items: &[&str]) -> i32 {
    // Native substr(0,3) permits shorter tokens and FindItem uses exact equality.
    let prefix = text.get(..3).unwrap_or(text);
    items
        .iter()
        .position(|item| prefix == *item)
        .map_or(0, |n| n as i32 + 1)
}
// Preserve Original explicit signed-integer boundary predicates and branch order.
#[allow(clippy::manual_range_contains)]
fn validate_month_day(
    text: &str,
    day: i32,
    month: i32,
    errors: &mut bool,
    diagnostic: &mut impl FnMut(&str) -> Result<(), Error>,
) -> Result<(), Error> {
    const END: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut internal_error = month < 1 || month > 12;
    if !internal_error && (day < 1 || day > END[month as usize - 1]) {
        internal_error = true;
    }
    if internal_error {
        diagnostic(&format!("Invalid Month Day date format={text}"))?;
        *errors = true;
    } else {
        *errors = false;
    }
    Ok(())
}
fn determine_date_tokens(
    text: &str,
    want_year: bool,
    date_type: &mut DateType,
    errors: &mut bool,
    diagnostic: &mut impl FnMut(&str) -> Result<(), Error>,
) -> Result<Tokens, Error> {
    const MONTHS: [&str; 12] = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    const WEEKDAYS: [&str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let mut current = text.to_owned();
    let mut fields: [String; 3] = std::array::from_fn(|_| String::new());
    let mut internal_error = false;
    let mut weekday_in_month = false;
    let mut tokens = Tokens::default();
    *date_type = DateType::Invalid;
    for character in ["/", ":", "-"] {
        while let Some(at) = current.find(character) {
            current.replace_range(at..at + 1, " ");
        }
    }
    for word in ["ST ", "ND ", "RD ", "TH ", "OF ", "IN "] {
        while let Some(at) = current.find(word) {
            current.replace_range(at..at + 2, "  ");
            weekday_in_month = true;
        }
    }
    strip_ascii_spaces(&mut current);
    if current.is_empty() {
        diagnostic(&format!("Invalid date field={text}"))?;
        *errors = true;
    } else {
        let mut count = 0;
        while count < 3 {
            if current.is_empty() {
                break;
            }
            let at = current.find(' ').unwrap_or(current.len());
            count += 1;
            fields[count - 1] = current[..at].to_owned();
            current.drain(..at);
            strip_ascii_spaces(&mut current);
        }
        if !current.is_empty() {
            diagnostic(&format!("Invalid date field={text}"))?;
            *errors = true;
        } else if count == 2 {
            internal_error = false;
            let (first, first_error) = number(&fields[0])?;
            if first_error {
                let (second, second_error) = number(&fields[1])?;
                if second_error {
                    diagnostic(&format!("Invalid date field={text}"))?;
                    internal_error = true;
                } else {
                    tokens.day = second;
                }
                tokens.month = lookup_prefix(&fields[0], &MONTHS);
                validate_month_day(
                    text,
                    tokens.day,
                    tokens.month,
                    &mut internal_error,
                    diagnostic,
                )?;
                if !internal_error {
                    *date_type = DateType::MonthDay;
                } else {
                    *errors = true;
                }
            } else {
                let (second, second_error) = number(&fields[1])?;
                if !second_error {
                    tokens.month = first;
                    tokens.day = second;
                    validate_month_day(
                        text,
                        tokens.day,
                        tokens.month,
                        &mut internal_error,
                        diagnostic,
                    )?;
                    if !internal_error {
                        *date_type = DateType::MonthDay;
                    } else {
                        *errors = true;
                    }
                } else {
                    tokens.day = first;
                    tokens.month = lookup_prefix(&fields[1], &MONTHS);
                    validate_month_day(
                        text,
                        tokens.day,
                        tokens.month,
                        &mut internal_error,
                        diagnostic,
                    )?;
                    if !internal_error {
                        *date_type = DateType::MonthDay;
                        tokens.number = 2;
                    } else {
                        *errors = true;
                    }
                }
            }
        } else if count == 3 {
            if weekday_in_month {
                let (first, first_error) = number(&fields[0])?;
                if !first_error {
                    tokens.day = first;
                    tokens.weekday = lookup_prefix(&fields[1], &WEEKDAYS);
                    if tokens.weekday == 0 {
                        tokens.month = lookup_prefix(&fields[1], &MONTHS);
                        tokens.weekday = lookup_prefix(&fields[2], &WEEKDAYS);
                        if tokens.month == 0 || tokens.weekday == 0 {
                            internal_error = true;
                        }
                    } else {
                        tokens.month = lookup_prefix(&fields[2], &MONTHS);
                        if tokens.month == 0 {
                            internal_error = true;
                        }
                    }
                    *date_type = DateType::NthDayInMonth;
                    tokens.number = 3;
                    if tokens.day < 0 || tokens.day > 5 {
                        internal_error = true;
                    }
                } else if fields[0] == "LA" {
                    *date_type = DateType::LastDayInMonth;
                    tokens.number = 3;
                    tokens.weekday = lookup_prefix(&fields[1], &WEEKDAYS);
                    if tokens.weekday == 0 {
                        tokens.month = lookup_prefix(&fields[1], &MONTHS);
                        tokens.weekday = lookup_prefix(&fields[2], &WEEKDAYS);
                        if tokens.month == 0 || tokens.weekday == 0 {
                            internal_error = true;
                        }
                    } else {
                        tokens.month = lookup_prefix(&fields[2], &MONTHS);
                        if tokens.month == 0 {
                            internal_error = true;
                        }
                    }
                } else {
                    // Native deliberately leaves caller ErrorsFound unchanged here.
                    diagnostic(&format!("First date field not numeric, field={text}"))?;
                }
            } else {
                let (first, _) = number(&fields[0])?;
                let (second, _) = number(&fields[1])?;
                let (third, _) = number(&fields[2])?;
                *date_type = DateType::MonthDay;
                if first > 100 {
                    if want_year {
                        tokens.year = first;
                    }
                    tokens.month = second;
                    tokens.day = third;
                } else if third > 100 {
                    if want_year {
                        tokens.year = third;
                    }
                    tokens.month = first;
                    tokens.day = second;
                }
            }
        } else {
            diagnostic(&format!("Invalid date field={text}"))?;
            *errors = true;
        }
    }
    if internal_error {
        *date_type = DateType::Invalid;
        *errors = true;
    }
    Ok(tokens)
}
