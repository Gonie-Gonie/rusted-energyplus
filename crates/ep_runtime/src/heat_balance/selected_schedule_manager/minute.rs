//! Native minute expansion and retained DaySchedule population.
//! The actual Compact producer supplies its retained interpolation enum.
use super::{SelectedDaySchedule, SelectedScheduleError};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceScheduleInterpolation {
    No,
    Average,
    Linear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SourceScheduleMinuteDiagnostic {
    Warning,
    Severe,
    Continue,
}

fn unavailable(message: &'static str) -> SelectedScheduleError {
    SelectedScheduleError::Unavailable(message)
}

fn decode_hhmm(
    field: &str,
    full_field: &str,
    day_name: &str,
    mode: SourceScheduleInterpolation,
    minutes_per_step: i32,
    errors_found: &mut bool,
    diagnostic: &mut impl FnMut(
        SourceScheduleMinuteDiagnostic,
        &str,
    ) -> Result<(), SelectedScheduleError>,
) -> Result<(i32, i32), SelectedScheduleError> {
    use SourceScheduleMinuteDiagnostic::{Continue, Severe, Warning};
    let field = field.trim_matches(' ');
    let Some((hour, minute)) = field.split_once(':') else {
        diagnostic(
            Severe,
            &format!(
                "ProcessScheduleInput: DecodeHHMMField, Invalid \"until\" field submitted (no : separator in hh:mm)={}",
                full_field.trim_matches(' ')
            ),
        )?;
        diagnostic(Continue, &format!("Occurred in Day Schedule={day_name}"))?;
        *errors_found = true;
        // Native returns without assigning either output. Its caller's next
        // read is not represented by invented HH/MM constructor values.
        return Err(unavailable(
            "actual ProcessIntervalFields read after DecodeHHMMField missing-separator return",
        ));
    };
    let number = |text: &str| -> Result<f64, SelectedScheduleError> {
        // Native Util::ProcessNumber returns +0 for empty/space-only text;
        // fast_float default from_chars rejects a leading plus. Rust parse
        // admits it, so exclude that unsupported grammar before parsing.
        let text = text.trim_matches(' ');
        if text.is_empty() {
            return Ok(0.0);
        }
        if !text.is_ascii()
            || text.starts_with('+')
            || text.bytes().any(|b| b.is_ascii_whitespace())
        {
            return Err(unavailable(
                "actual DecodeHHMMField Util::ProcessNumber noncanonical numeric grammar",
            ));
        }
        text.parse::<f64>()
            .ok()
            .filter(|x| x.is_finite())
            .ok_or_else(|| {
                unavailable(
                    "actual DecodeHHMMField Util::ProcessNumber unsupported numeric/error branch",
                )
            })
    };
    let mut non_integral = false;
    let mut decode = |text: &str,
                      label: &str,
                      empty_zero: bool|
     -> Result<i32, SelectedScheduleError> {
        let value = if empty_zero && text.is_empty() {
            0.0
        } else {
            number(text)?
        };
        if value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
            return Err(unavailable("actual DecodeHHMMField int conversion domain"));
        }
        let integer = value as i32;
        if f64::from(integer) != value || value < 0.0 {
            if f64::from(integer) != value && value >= 0.0 {
                diagnostic(
                    Warning,
                    &format!(
                        "ProcessScheduleInput: DecodeHHMMField, Invalid \"until\" field submitted (non-integer numeric in {label})={}",
                        full_field.trim_matches(' ')
                    ),
                )?;
                diagnostic(
                    Continue,
                    &format!("Other errors may result. Occurred in Day Schedule={day_name}"),
                )?;
                non_integral = true;
            } else {
                diagnostic(
                    Severe,
                    &format!(
                        "ProcessScheduleInput: DecodeHHMMField, Invalid \"until\" field submitted (invalid numeric in {label})={}",
                        full_field.trim_matches(' ')
                    ),
                )?;
                diagnostic(
                    Continue,
                    &format!(
                        "Field values must be integer and represent hours:minutes. Occurred in Day Schedule={day_name}"
                    ),
                )?;
                *errors_found = true;
                return Err(unavailable(
                    "actual ProcessIntervalFields output read after invalid DecodeHHMMField return",
                ));
            }
        }
        Ok(integer)
    };
    let hour = decode(hour, "HH", true)?;
    let minute = decode(minute, "MM", false)?;
    if non_integral {
        // Pinned Native formats default std::string hHour/mMinute with :2.2F.
        // Preserve the actual first unsupported formatting operation.
        return Err(unavailable(
            "actual DecodeHHMMField nonIntegral string floating-format diagnostic",
        ));
    }
    if mode == SourceScheduleInterpolation::No && minute != 0 && minute % minutes_per_step != 0 {
        diagnostic(
            Warning,
            &format!(
                "ProcessScheduleInput: DecodeHHMMField, Invalid \"until\" field value is not a multiple of the minutes for each timestep: {}",
                full_field.trim_matches(' ')
            ),
        )?;
        diagnostic(
            Continue,
            &format!("Other errors may result. Occurred in Day Schedule={day_name}"),
        )?;
    }
    Ok((hour, minute))
}

fn index(hour: i32, minute: i32) -> Result<usize, SelectedScheduleError> {
    let at = hour
        .checked_mul(60)
        .and_then(|h| h.checked_add(minute))
        .ok_or_else(|| unavailable("actual ProcessIntervalFields minute index conversion"))?;
    usize::try_from(at)
        .ok()
        .filter(|at| *at < 1440)
        .ok_or_else(|| unavailable("actual ProcessIntervalFields native minute array bounds"))
}

/// Native2733-2980. Generic diagnostic callback also supports the current
/// production Compact owner's deliberate boundary at unsupported diagnostics.
// Keep Original constructor/local initialization and later assignment order, including unconsumed suffix writes.
#[allow(unused_assignments)]
pub(super) fn process_interval_fields_with(
    untils: &[String],
    values: &[f64],
    num_untils: usize,
    num_values: usize,
    minute_values: &mut [f64; 1440],
    set_minutes: &mut [bool; 1440],
    errors_found: &mut bool,
    day_name: &str,
    context: &str,
    mode: SourceScheduleInterpolation,
    minutes_per_step: i32,
    diagnostic: &mut impl FnMut(
        SourceScheduleMinuteDiagnostic,
        &str,
    ) -> Result<(), SelectedScheduleError>,
) -> Result<(), SelectedScheduleError> {
    use SourceScheduleMinuteDiagnostic::{Continue, Severe, Warning};
    minute_values.fill(0.0);
    set_minutes.fill(false);
    if num_untils != num_values {
        diagnostic(
            Severe,
            &format!(
                "ProcessScheduleInput: ProcessIntervalFields, number of Time fields does not match number of value fields, {context}={day_name}"
            ),
        )?;
        *errors_found = true;
        return Ok(());
    }
    if minutes_per_step <= 0 || num_untils > untils.len() || num_values > values.len() {
        return Err(unavailable(
            "actual ProcessIntervalFields borrowed input/clock extent",
        ));
    }
    let (mut begin_hour, mut begin_minute) = (0_i32, 0_i32);
    let (mut end_hour, mut end_minute) = (-1_i32, -1_i32);
    let (mut start_value, mut end_value) = (0.0, 0.0);
    'untils: for count in 0..num_untils {
        let until = &untils[count];
        let field = match until.find("UNTIL") {
            Some(0) => until
                .get(
                    if until.as_bytes().get(5) == Some(&b':') {
                        6
                    } else {
                        5
                    }..,
                )
                .ok_or_else(|| unavailable("actual ProcessIntervalFields UNTIL byte substring"))?,
            None => until.as_str(),
            Some(_) => {
                diagnostic(
                    Severe,
                    &format!(
                        "ProcessScheduleInput: ProcessIntervalFields, Invalid \"Until\" field encountered={until}"
                    ),
                )?;
                diagnostic(Continue, &format!("Occurred in Day Schedule={day_name}"))?;
                *errors_found = true;
                continue;
            }
        };
        let (hour, mut minute) = decode_hhmm(
            field,
            until,
            day_name,
            mode,
            minutes_per_step,
            errors_found,
            diagnostic,
        )?;
        if !(0..=24).contains(&hour) || !(0..=60).contains(&minute) {
            diagnostic(
                Severe,
                &format!(
                    "ProcessScheduleInput: ProcessIntervalFields, Invalid \"Until\" field encountered={until}"
                ),
            )?;
            diagnostic(Continue, &format!("Occurred in Day Schedule={day_name}"))?;
            *errors_found = true;
            continue;
        }
        if hour == 24 && minute > 0 && minute < 60 {
            diagnostic(
                Warning,
                &format!(
                    "ProcessScheduleInput: ProcessIntervalFields, Invalid \"Until\" field encountered={until}"
                ),
            )?;
            diagnostic(Continue, &format!("Occurred in Day Schedule={day_name}"))?;
            diagnostic(Continue, "Terminating the field at 24:00")?;
            minute = 0;
        }
        if minute == 0 {
            end_hour = hour - 1;
            end_minute = 59;
        } else if minute < 60 {
            end_hour = hour;
            end_minute = minute - 1;
        }
        // MM==60 deliberately retains the Native endHr/endMin locals.
        let (mut increment, mut current_value) = (0.0, 0.0);
        if mode == SourceScheduleInterpolation::Linear {
            let mut total_minutes = (end_hour - begin_hour) * 60 + (end_minute - begin_minute) + 1;
            if total_minutes == 0 {
                total_minutes = 1;
            }
            if count == 0 {
                start_value = values[count];
                end_value = values[count];
            } else {
                start_value = end_value;
                end_value = values[count];
            }
            increment = (end_value - start_value) / f64::from(total_minutes);
            current_value = start_value + increment;
        }
        if begin_hour > end_hour {
            if begin_hour == end_hour + 1 && begin_minute == 0 && end_minute == 59 {
                diagnostic(
                    Warning,
                    &format!(
                        "ProcessScheduleInput: ProcessIntervalFields, Processing time fields, zero time interval detected, {context}={day_name}"
                    ),
                )?;
            } else {
                diagnostic(
                    Severe,
                    &format!(
                        "ProcessScheduleInput: ProcessIntervalFields, Processing time fields, overlapping times detected, {context}={day_name}"
                    ),
                )?;
                *errors_found = true;
            }
        } else if begin_hour == end_hour {
            for minute in begin_minute..=end_minute {
                if set_minutes[index(begin_hour, minute)?] {
                    diagnostic(
                        Severe,
                        &format!(
                            "ProcessScheduleInput: ProcessIntervalFields, Processing time fields, overlapping times detected, {context}={day_name}"
                        ),
                    )?;
                    *errors_found = true;
                    break 'untils;
                }
            }
            for minute in begin_minute..=end_minute {
                let at = index(begin_hour, minute)?;
                if mode == SourceScheduleInterpolation::Linear {
                    minute_values[at] = current_value;
                    current_value += increment;
                } else {
                    minute_values[at] = values[count];
                }
                set_minutes[at] = true;
            }
            begin_minute = end_minute + 1;
            if begin_minute >= 60 {
                begin_hour += 1;
                begin_minute = 0;
            }
        } else {
            let last_start_minute = if mode == SourceScheduleInterpolation::Linear {
                59
            } else {
                60
            };
            for (hour, first, last) in
                std::iter::once((begin_hour, begin_minute, last_start_minute))
                    .chain((begin_hour + 1..end_hour).map(|hour| (hour, 0, 59)))
                    .chain(std::iter::once((end_hour, 0, end_minute)))
            {
                for minute in first..=last {
                    let at = index(hour, minute)?;
                    if mode == SourceScheduleInterpolation::Linear {
                        minute_values[at] = current_value;
                        current_value += increment;
                    } else {
                        minute_values[at] = values[count];
                    }
                    set_minutes[at] = true;
                }
            }
            begin_hour = end_hour;
            begin_minute = end_minute + 1;
            if begin_minute >= 60 {
                begin_hour += 1;
                begin_minute = 0;
            }
        }
    }
    for set in set_minutes.iter() {
        if !set {
            diagnostic(
                Severe,
                &format!(
                    "ProcessScheduleInput: ProcessIntervalFields, Processing time fields, incomplete day detected, {context}={day_name}"
                ),
            )?;
            *errors_found = true;
        }
    }
    Ok(())
}

/// Actual diagnostic/counter owner for direct minute-helper consumers.
#[allow(
    dead_code,
    reason = "Keep the genuine ScheduleManager minute/interpolation interface for paired No/Average/Linear units; production admission remains separately restricted to its selected input mode."
)]
pub(crate) fn process_source_interval_fields(
    untils: &[String],
    values: &[f64],
    num_untils: usize,
    num_values: usize,
    minute_values: &mut [f64; 1440],
    set_minutes: &mut [bool; 1440],
    errors_found: &mut bool,
    day_name: &str,
    context: &str,
    mode: SourceScheduleInterpolation,
    minutes_per_step: i32,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), SelectedScheduleError> {
    process_interval_fields_with(
        untils,
        values,
        num_untils,
        num_values,
        minute_values,
        set_minutes,
        errors_found,
        day_name,
        context,
        mode,
        minutes_per_step,
        &mut |kind, message| {
            let result = match kind {
                SourceScheduleMinuteDiagnostic::Warning => backend.show_warning_error(message),
                SourceScheduleMinuteDiagnostic::Severe => backend.show_severe_error(message),
                SourceScheduleMinuteDiagnostic::Continue => backend.show_continue_error(message),
            };
            result.map_err(|e| SelectedScheduleError::from(HeatBalanceInputError::from(e)))
        },
    )
}

/// Native211-249. Average accumulates every minute in original order; No and
/// Linear sample the ending minute. The real retained sum is never reset here.
pub(super) fn populate_source_day_from_minutes(
    day: &mut SelectedDaySchedule,
    minute_values: &[f64; 1440],
    steps: usize,
) -> Result<(), SelectedScheduleError> {
    if steps == 0 || steps > 60 || 60 % steps != 0 || day.timestep_values.len() != 24 * steps {
        return Err(unavailable(
            "actual DaySchedule populate timestep/storage extent",
        ));
    }
    let minutes = 60 / steps;
    if day.interpolation == 1 {
        for hour in 0..24 {
            let mut begin = 0;
            let mut end = minutes - 1;
            for timestep in 0..steps {
                let mut accum = 0.0;
                for minute in begin..=end {
                    accum += minute_values[hour * 60 + minute];
                }
                let at = hour * steps + timestep;
                day.timestep_values[at] = accum / minutes as f64;
                day.sum_timestep_values += day.timestep_values[at];
                begin = end + 1;
                end += minutes;
            }
        }
    } else {
        for hour in 0..24 {
            let mut end = minutes - 1;
            for timestep in 0..steps {
                let at = hour * steps + timestep;
                day.timestep_values[at] = minute_values[hour * 60 + end];
                day.sum_timestep_values += day.timestep_values[at];
                end += minutes;
            }
        }
    }
    Ok(())
}
