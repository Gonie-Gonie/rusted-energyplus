//! Native ResetWeekDaysByMonth1330–1526 and zero/skip SetSpecialDayDates prefix.
use super::{Error, Failure, ordinal};
use crate::heat_balance::error_backend::SourceErrorBackend;
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
pub(super) fn set_dst_dates(
    _month_weekdays: &[i32; 12],
    _dst_index: &mut [i32; 366],
    _header: &mut RawEpwHeaderState,
) -> Result<(), Failure> {
    // Actual child starts by reading the retained DST StartDateType. No numeric
    // seed or false activity substitute is admitted at this positive boundary.
    Err(
        Error::Unavailable("actual positive SetDSTDateRanges retained date resolution1530–1620")
            .into(),
    )
}
pub(super) fn set_special_days(
    month_weekdays: &[i32; 12],
    special_day_types: &mut [i32; 366],
    use_special_days: &bool,
    header: &mut RawEpwHeaderState,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    let errors_found = false;
    special_day_types.fill(0);
    for number in 1..=header.number_special_days {
        let special = header
            .special_days
            .values
            .get_mut(ordinal(number)?)
            .ok_or(Error::Unavailable("actual SpecialDays reference1647"))?;
        if special.weather_file && !*use_special_days {
            continue;
        }
        return Err(
            Error::Unavailable("actual positive SetSpecialDayDates date resolution1651").into(),
        );
    }
    if errors_found {
        return Err(super::fatal(
            backend,
            "SetSpecialDayDates: Program terminates due to preceding condition(s).",
        ));
    }
    let _ = month_weekdays;
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
