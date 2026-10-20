//! Native ResetWeekDaysByMonth1330–1526 and zero/skip SetSpecialDayDates prefix.
use super::{Error, Failure, ordinal};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::weather::day::SourceEnvironmentData;
use crate::weather::raw::RawEpwHeaderState;

pub(super) fn initialize_day_of_week(declared: i32, current: &mut i32) -> Result<(), Error> {
    if declared != 0 {
        if declared <= 7 {
            *current = declared
                .checked_sub(1)
                .ok_or(Error::Unavailable("actual weekday signed subtraction3244"))?;
        } else {
            *current = declared;
        }
    }
    Ok(())
}
// Actual WM1530-1732 children on the retained WeatherManager/header owners.
pub(in crate::weather::day) fn set_dst_dates(
    month_weekdays: &[i32; 12],
    month_ends_with_leap: &[i32; 12],
    dst_index: &mut [i32; 366],
    header: &RawEpwHeaderState,
    backend: &mut SourceErrorBackend<'_>,
    flags: &SourceDiagnosticFlags<'_>,
) -> Result<(i32, i32, i32, i32), Failure> {
    use crate::weather::raw::DateType;
    let mut errors = false;
    let d = &header.dst;
    let mut start = None;
    let mut end = None;
    for is_start in [true, false] {
        let (kind, month, day, weekday) = if is_start {
            (
                d.start_date_type,
                d.start_month,
                d.start_day,
                d.start_weekday,
            )
        } else {
            (d.end_date_type, d.end_month, d.end_day, d.end_weekday)
        };
        let date;
        if kind == DateType::MonthDay {
            // Do not read the EPW's genuinely unwritten weekday in this branch.
            date = Some((month, day));
        } else {
            let mut value = sub(
                weekday.ok_or(Error::Unavailable("actual DST weekday definedness1564"))?,
                month_value(month_weekdays, month)?,
            )?;
            value = add(value, 1)?;
            let limit = month_value(month_ends_with_leap, month)?;
            if kind == DateType::NthDayInMonth {
                while value <= 0 {
                    value = add(value, 7)?;
                }
                value = add(value, mul(7, sub(day, 1)?)?)?;
                // Pinned Original1591 is a right shift for the END date. It is
                // intentionally not the START date's greater-than predicate.
                let insufficient = if is_start {
                    value > limit
                } else {
                    value
                        .checked_shr(
                            u32::try_from(limit)
                                .map_err(|_| Error::Unavailable("actual DST end shift1591"))?,
                        )
                        .ok_or(Error::Unavailable("actual DST end shift1591"))?
                        != 0
                };
                if insufficient {
                    let side = if is_start { "Start" } else { "End" };
                    backend.show_source_severe_error(&format!("SetDSTDateRanges: Determining DST: DST {side} Date, Nth Day of Month, not enough Nths"), flags)?;
                    errors = true;
                    date = if is_start { None } else { Some((0, 0)) };
                } else {
                    date = Some((month, value));
                }
            } else {
                while add(value, 7)? <= limit {
                    value = add(value, 7)?;
                }
                date = Some((month, value));
            }
        }
        if is_start {
            start = date;
        } else {
            end = date;
        }
    }
    if errors {
        return Err(calendar_fatal(
            backend,
            flags,
            "SetDSTDateRanges: Program terminates due to preceding condition(s).",
        ));
    }
    let (sm, sd) = start.ok_or(Error::Unavailable("actual DST start defined output1613"))?;
    let (em, ed) = end.ok_or(Error::Unavailable("actual DST end defined output1616"))?;
    // The caller's optional outputs are these genuine local values; reset/fill
    // follows them in Native. No preview calendar is consulted.
    dst_index.fill(0);
    let first = crate::weather::raw::source_ordinal_day(sm, sd, header.leap_year_add)?;
    let last = crate::weather::raw::source_ordinal_day(em, ed, header.leap_year_add)?;
    if last >= first {
        fill_dst(dst_index, first, last)?;
    } else {
        fill_dst(dst_index, first, 366)?;
        fill_dst(dst_index, 1, last)?;
    }
    Ok((sm, sd, em, ed))
}
fn calendar_fatal(
    backend: &mut SourceErrorBackend<'_>,
    flags: &SourceDiagnosticFlags<'_>,
    text: &str,
) -> Failure {
    match backend.show_source_fatal_error(text, flags) {
        Ok(never) => match never {},
        Err(error) => error.into(),
    }
}
fn month_value(values: &[i32; 12], month: i32) -> Result<i32, Error> {
    values
        .get(ordinal(month)?)
        .copied()
        .ok_or(Error::Unavailable("actual calendar month subscript"))
}
fn mul(a: i32, b: i32) -> Result<i32, Error> {
    a.checked_mul(b)
        .ok_or(Error::Unavailable("actual calendar signed multiplication"))
}
fn fill_dst(values: &mut [i32; 366], first: i32, last: i32) -> Result<(), Error> {
    for day in first..=last {
        *values
            .get_mut(ordinal(day)?)
            .ok_or(Error::Unavailable("actual DSTIndex range1628"))? = 1;
    }
    Ok(())
}
pub(crate) fn inverse_ordinal(
    number: i32,
    month: &mut i32,
    day: &mut i32,
    leap: i32,
) -> Result<(), Error> {
    const END: [i32; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    if !(0..=366).contains(&number) {
        return Ok(());
    }
    let mut month_index = 1_usize;
    let (current, leap_current) = loop {
        // If no interval matched, Native exits with WMonth13 and the last
        // iteration's LeapAddCur (the supplied leap), including Number0/366.
        if month_index > 12 {
            break (month_index, leap);
        }
        let leap_previous = if month_index <= 2 { 0 } else { leap };
        let leap_current = if month_index == 1 { 0 } else { leap };
        if number > add(END[month_index - 1], leap_previous)?
            && number <= add(END[month_index], leap_current)?
        {
            break (month_index, leap_current);
        }
        month_index += 1;
    };
    *month = current as i32;
    // Original General777 uses LeapAddCur even for February. Preserve it.
    *day = sub(number, add(END[current - 1], leap_current)?)?;
    Ok(())
}
// Independent actual Source arrays and same live Environment flag, not a
// synthesized calendar owner or a copied preparation-time schedule.
#[allow(clippy::too_many_arguments)]
pub(in crate::weather::day) fn set_special_days(
    month_weekdays: &[i32; 12],
    month_ends_with_leap: &[i32; 12],
    weekday_types: &[i32; 366],
    special_day_types: &mut [i32; 366],
    use_special_days: &bool,
    apply_weekend_rule: &bool,
    header: &mut RawEpwHeaderState,
    backend: &mut SourceErrorBackend<'_>,
    flags: &SourceDiagnosticFlags<'_>,
) -> Result<(), Failure> {
    use crate::weather::raw::{DateType, source_ordinal_day};
    let mut errors_found = false;
    special_day_types.fill(0);
    let leap = header.leap_year_add;
    for number in 1..=header.number_special_days {
        let special = header
            .special_days
            .values
            .get_mut(ordinal(number)?)
            .ok_or(Error::Unavailable("actual SpecialDays reference1647"))?;
        if special.weather_file && !*use_special_days {
            continue;
        }
        let mut jday;
        if special.date_type as i32 <= DateType::MonthDay as i32 {
            jday = source_ordinal_day(special.month, special.day, leap)?;
            if special.duration == 1 && *apply_weekend_rule {
                let weekday = *weekday_types
                    .get(ordinal(jday)?)
                    .ok_or(Error::Unavailable("actual WeekDayTypes1654"))?;
                let increments = if weekday == 1 {
                    1
                } else if weekday == 7 {
                    2
                } else {
                    0
                };
                for _ in 0..increments {
                    jday = add(jday, 1)?;
                    if jday == 366 && leap == 0 {
                        jday = 1;
                    }
                }
            }
            inverse_ordinal(
                jday,
                &mut special.actual_start_month,
                &mut special.actual_start_day,
                leap,
            )?;
        } else if special.date_type == DateType::NthDayInMonth {
            let start_weekday = month_value(month_weekdays, special.month)?;
            let mut value = add(sub(special.weekday, start_weekday)?, 1)?;
            if special.weekday < start_weekday {
                value = add(value, 7)?;
            }
            value = add(value, mul(7, sub(special.day, 1)?)?)?;
            if value > month_value(month_ends_with_leap, special.month)? {
                backend.show_source_severe_error(&format!("SetSpecialDayDates: Special Day Date, Nth Day of Month, not enough Nths, for SpecialDay={}", special.name), flags)?;
                errors_found = true;
                continue;
            }
            special.actual_start_month = special.month;
            special.actual_start_day = value;
            jday = source_ordinal_day(special.month, value, leap)?;
        } else {
            let mut value = add(
                sub(special.weekday, month_value(month_weekdays, special.month)?)?,
                1,
            )?;
            while add(value, 7)? <= month_value(month_ends_with_leap, special.month)? {
                value = add(value, 7)?;
            }
            special.actual_start_month = special.month;
            special.actual_start_day = value;
            jday = source_ordinal_day(special.month, value, leap)?;
        }
        if *special_day_types
            .get(ordinal(jday)?)
            .ok_or(Error::Unavailable("actual special overlap1700"))?
            != 0
        {
            backend.show_source_warning_error(&format!("SetSpecialDayDates: Special Day definition ({}) is overwriting previously entered special day period", special.name), flags)?;
            if *use_special_days {
                backend.show_continue_error(
                    "...This could be caused by definitions on the Weather File.",
                )?;
            }
            backend.show_continue_error(
                "...This could be caused by duplicate definitions in the Input File.",
            )?;
        }
        let mut last = sub(jday, 1)?;
        for _ in 0..special.duration {
            last = add(last, 1)?;
            if last == 366 && leap == 0 {
                last = 1;
            }
            if last == 367 {
                last = 1;
            }
            *special_day_types
                .get_mut(ordinal(last)?)
                .ok_or(Error::Unavailable("actual SpecialDayTypes duration1722"))? =
                special.day_type;
        }
    }
    if errors_found {
        return Err(calendar_fatal(
            backend,
            flags,
            "SetSpecialDayDates: Program terminates due to preceding condition(s).",
        ));
    }
    Ok(())
}
fn add(first: i32, second: i32) -> Result<i32, Error> {
    first.checked_add(second).ok_or(Error::Unavailable(
        "actual ResetWeekDaysByMonth signed addition",
    ))
}
fn sub(first: i32, second: i32) -> Result<i32, Error> {
    first.checked_sub(second).ok_or(Error::Unavailable(
        "actual ResetWeekDaysByMonth signed subtraction",
    ))
}
fn previous_fill(
    weekdays: &mut [i32; 12],
    ends: &[i32; 12],
    start: i32,
    start_day: i32,
    leap: i32,
) -> Result<(), Error> {
    if weekdays.contains(&0) {
        let mut current = weekdays[ordinal(start)?];
        for _ in 1..=sub(start_day, 1)? {
            current = sub(current, 1)?;
            if current == 0 {
                current = 7;
            }
        }
        for month in (1..start).rev() {
            if month == 1 {
                current = sub(current, ends[0])?;
                while current <= 0 {
                    current = add(current, 7)?;
                }
                weekdays[ordinal(month)?] = current;
            } else if month == 2 {
                current = add(sub(current, ends[1])?, leap)?;
                while current <= 0 {
                    current = add(current, 7)?;
                }
                weekdays[ordinal(month)?] = current;
            } else if (3..=12).contains(&month) {
                current = sub(current, ends[ordinal(month)?])?;
                while current <= 0 {
                    current = add(current, 7)?;
                }
                weekdays[ordinal(month)?] = current;
            }
        }
    }
    Ok(())
}
fn next_fill(
    weekdays: &mut [i32; 12],
    ends: &[i32; 12],
    start: i32,
    leap: i32,
    current: &mut i32,
) -> Result<(), Error> {
    for month in add(start, 1)?..=12 {
        if month == 2 {
            *current = add(*current, ends[0])?;
            while *current > 7 {
                *current = sub(*current, 7)?;
            }
            weekdays[ordinal(month)?] = *current;
        } else if month == 3 {
            *current = add(*current, add(ends[ordinal(sub(month, 1)?)?], leap)?)?;
            while *current > 7 {
                *current = sub(*current, 7)?;
            }
            weekdays[ordinal(month)?] = *current;
        } else {
            *current = add(*current, ends[ordinal(sub(month, 1)?)?])?;
            while *current > 7 {
                *current = sub(*current, 7)?;
            }
            weekdays[ordinal(month)?] = *current;
        }
    }
    Ok(())
}
pub(super) fn reset_weekdays(
    row: &mut SourceEnvironmentData,
    header: &RawEpwHeaderState,
    mid_sim: bool,
) -> Result<(), Failure> {
    let weekdays = &mut row.month_week_day;
    let start = row.start_month;
    let start_day = row.start_day;
    let end = row.end_month;
    let end_day = row.end_day;
    let rollover = row.roll_day_type_on_repeat;
    let ends = &header.month_ends;
    let leap = header.leap_year_add;
    let copy = *weekdays;
    let mut current;
    if !mid_sim {
        if rollover {
            if start == 1 {
                current = sub(add(add(weekdays[11], ends[11])?, start_day)?, 1)?;
            } else {
                current = add(weekdays[ordinal(end)?], end_day)?;
            }
        } else {
            current = weekdays[ordinal(start)?];
        }
        while current > 7 {
            current = sub(current, 7)?;
        }
        weekdays.fill(0);
        weekdays[ordinal(start)?] = current;
        next_fill(weekdays, ends, start, leap, &mut current)?;
        previous_fill(weekdays, ends, start, start_day, leap)?;
    } else {
        if rollover {
            if start == 1 {
                current = sub(add(add(weekdays[11], ends[11])?, start_day)?, 1)?;
            } else {
                current = add(weekdays[ordinal(end)?], end_day)?;
            }
        } else {
            current = weekdays[ordinal(start)?];
        }
        while current > 7 {
            current = sub(current, 7)?;
        }
        weekdays.fill(0);
        if start != 1 {
            current = add(copy[11], ends[11])?;
            while current > 7 {
                current = sub(current, 7)?;
            }
            weekdays[0] = current;
            current = add(current, ends[0])?;
            while current > 7 {
                current = sub(current, 7)?;
            }
            weekdays[1] = current;
            current = add(current, add(ends[1], leap)?)?;
            while current > 7 {
                current = sub(current, 7)?;
            }
            weekdays[2] = current;
            for month in 4..=12 {
                current = add(current, ends[ordinal(sub(month, 1)?)?])?;
                while current > 7 {
                    current = sub(current, 7)?;
                }
                weekdays[ordinal(month)?] = current;
            }
        } else {
            weekdays.fill(0);
            weekdays[ordinal(start)?] = current;
            next_fill(weekdays, ends, start, leap, &mut current)?;
            previous_fill(weekdays, ends, start, start_day, leap)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
