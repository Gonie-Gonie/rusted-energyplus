//! Raw EPW record interpretation, before downstream weather normalization.

use super::lexical::{process_number, read_int, read_real};
use super::record_types::{RawEpwOutputs, RawEpwParserState, RawRecordFailure};

fn after_nth_comma(input: &str, count: usize) -> Option<usize> {
    input
        .bytes()
        .enumerate()
        .filter(|(_, byte)| *byte == b',')
        .nth(count - 1)
        .map(|(position, _)| position + 1)
}

struct OptionalNumbers<'a> {
    remaining: &'a str,
    reached_end: bool,
}

impl OptionalNumbers<'_> {
    fn next(&mut self) -> Result<f64, ()> {
        if self.reached_end {
            return Ok(999.0);
        }
        if let Some(comma) = self.remaining.find(',') {
            let value = if comma == 0 {
                999.0
            } else {
                process_number(&self.remaining[..comma])?
            };
            self.remaining = &self.remaining[comma + 1..];
            Ok(value)
        } else {
            self.reached_end = true;
            if self.remaining.is_empty() {
                Ok(999.0)
            } else {
                process_number(self.remaining)
            }
        }
    }
}

/// Interpret the public reference arguments of the original raw record method.
///
/// The method preserves source write order and final-list-status behavior. It
/// admits complete ASCII delimiter shapes and defined finite number conversions;
/// unsupported shapes return `OutsideBoundedDomain`, without claiming source
/// failure parity. Raw sentinels and record dates remain unnormalized.
pub fn interpret_weather_data_line(
    owner: &mut RawEpwParserState,
    line: &str,
    outputs: &mut RawEpwOutputs,
) -> Result<(), RawRecordFailure> {
    outputs.error_found = false;
    if !line.is_ascii() {
        return Err(RawRecordFailure::OutsideBoundedDomain(
            "ASCII record grammar",
        ));
    }
    let fifth = after_nth_comma(line, 5).ok_or(RawRecordFailure::OutsideBoundedDomain(
        "first five delimiters",
    ))?;
    let mut cursor = 0;
    let mut last_status = false;
    // A comma-fold evaluates every read and returns only the final status.
    for value in &mut outputs.dates {
        last_status = read_int(&line[..fifth - 1], &mut cursor, value);
    }
    if !last_status {
        return Err(RawRecordFailure::SourceFatalDateList);
    }
    let month = outputs.dates[1];
    let day = outputs.dates[2];
    if !(1..=12).contains(&month) {
        return Err(RawRecordFailure::SourceFatalDateAdmission);
    }
    let max_day = owner.end_day_of_month[(month - 1) as usize]
        .checked_add(i32::from(month == 2))
        .ok_or(RawRecordFailure::OutsideBoundedDomain(
            "bounded month-end addition",
        ))?;
    if day > max_day {
        return Err(RawRecordFailure::SourceFatalDateAdmission);
    }
    // No lower-day, year, hour or minute validation belongs to this method.
    let sixth = after_nth_comma(line, 6).ok_or(RawRecordFailure::OutsideBoundedDomain(
        "data-source delimiter",
    ))?;
    let numeric_line = &line[sixth..];
    let twenty_first = after_nth_comma(numeric_line, 21).ok_or(
        RawRecordFailure::OutsideBoundedDomain("mandatory numeric delimiters"),
    )?;
    let numeric_input = &numeric_line[..twenty_first - 1];
    cursor = 0;
    for value in &mut outputs.mandatory_reals {
        let _ = read_real(numeric_input, &mut cursor, value)
            .map_err(RawRecordFailure::OutsideBoundedDomain)?;
    }
    // This local is private in the original. Its initialized Rust storage is
    // never observed, and its value is consumed only after a successful read.
    let mut private_rfield21 = 0.0;
    if !read_real(numeric_input, &mut cursor, &mut private_rfield21)
        .map_err(RawRecordFailure::OutsideBoundedDomain)?
    {
        return Err(RawRecordFailure::SourceFatalMandatoryList);
    }
    let codes_and_tail = &numeric_line[twenty_first..];
    let Some(code_comma) = codes_and_tail.find(',') else {
        return Err(RawRecordFailure::OutsideBoundedDomain(
            "weather-code delimiter",
        ));
    };
    let codes = if code_comma == 0 {
        "999999999"
    } else {
        &codes_and_tail[..code_comma]
    };
    let mut tail = OptionalNumbers {
        remaining: &codes_and_tail[code_comma + 1..],
        reached_end: false,
    };
    for (field, target) in outputs.optional_reals.iter_mut().enumerate() {
        *target = tail
            .next()
            .map_err(|()| RawRecordFailure::SourceFatalOptionalNumber { field })?;
    }
    // Source nint groups the sign-directed half addition before integer cast.
    let rounded = private_rfield21 + if private_rfield21 >= 0.0 { 0.5 } else { -0.5 };
    let truncated = rounded.trunc();
    if !truncated.is_finite() || truncated < f64::from(i32::MIN) || truncated > f64::from(i32::MAX)
    {
        return Err(RawRecordFailure::OutsideBoundedDomain(
            "bounded defined nint cast",
        ));
    }
    outputs.observation_indicator = rounded as i32;
    if outputs.observation_indicator == 0 {
        let cleaned = codes.replace(['\'', '"'], " ");
        let cleaned = cleaned.trim_matches(' ');
        if cleaned.len() == 9 {
            for (target, byte) in outputs.weather_codes.iter_mut().zip(cleaned.bytes()) {
                *target = if byte.is_ascii_digit() {
                    i32::from(byte - b'0')
                } else {
                    9
                };
            }
        } else {
            owner.weather_code_missed_count =
                owner.weather_code_missed_count.checked_add(1).ok_or(
                    RawRecordFailure::OutsideBoundedDomain("bounded counter increment"),
                )?;
            outputs.weather_codes.fill(9);
        }
    } else {
        outputs.weather_codes.fill(9);
    }
    Ok(())
}
