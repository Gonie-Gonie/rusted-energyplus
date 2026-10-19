//! Actual ProcessEPWHeader caller for the retained physical Source stream.
use super::dates::{number_with_error, setup_weekdays};
use super::header::{HeaderKind, location};
use super::header_state::{defined, source_count, to_source_int};
use super::input::erase_field;
use super::source_typical::{
    SourceDateScratch, process_source_date_string, process_source_typical_extreme_periods,
    source_ordinal_day,
};
use super::{
    DateType, HeaderContextConsumption, RawDayFailure, RawEpwHeaderError, RawEpwHeaderState,
    RawEpwInput,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::SourceWeatherInitializationError as Failure;

fn raw(error: RawEpwHeaderError) -> Failure {
    RawDayFailure::Header(error).into()
}
fn fatal(backend: &mut SourceErrorBackend<'_>, text: &str) -> Failure {
    match backend.show_fatal_error(text) {
        Ok(never) => match never {},
        Err(error) => Error::Fatal(error).into(),
    }
}

// These are the real independently borrowed Native operands; do not hide them in a copied owner DTO.
#[allow(clippy::too_many_arguments)]
pub(super) fn process_source_epw_header(
    kind: HeaderKind,
    line: &mut String,
    errors: &mut bool,
    state: &mut RawEpwHeaderState,
    input: &mut RawEpwInput,
    processor: &mut SelectedInputProcessor,
    context: &mut HeaderContextConsumption,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    if let Some(at) = line.find(',') {
        erase_field(line, at);
    } else if !matches!(kind, HeaderKind::Comments1 | HeaderKind::Comments2) {
        backend.show_severe_error("Invalid Header line in in.epw -- no commas")?;
        backend.show_continue_error(&format!("Line={line}"))?;
        return Err(fatal(backend, "Previous conditions cause termination."));
    }
    match kind {
        HeaderKind::Location => location(line, state, input).map_err(raw),
        HeaderKind::TypicalExtremePeriods => {
            process_source_typical_extreme_periods(line, errors, state, input, backend)
        }
        HeaderKind::GroundTemperatures => context.consume_ground(line, state).map_err(raw),
        HeaderKind::HolidaysDst => holidays(line, errors, state, input, processor, backend),
        HeaderKind::DataPeriods => periods(line, errors, state, input, backend),
        HeaderKind::Comments1 | HeaderKind::Comments2 | HeaderKind::DesignConditions => Ok(()),
    }
}

fn holidays(
    line: &mut String,
    errors: &mut bool,
    state: &mut RawEpwHeaderState,
    input: &mut RawEpwInput,
    processor: &mut SelectedInputProcessor,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    line.make_ascii_uppercase();
    let mut arguments = 4_i32;
    let mut current = 0_usize;
    let mut field = 1_i32;
    while field <= arguments {
        let end = input.field_end(line).map_err(raw)?;
        let token = &line[..end];
        match field {
            1 => state.allows_leap_years = line.as_bytes().first() == Some(&b'Y'),
            2 => {
                let mut local_error = false; // Native errflag1 is not subsequently read.
                let mut date = SourceDateScratch::default();
                process_source_date_string(token, &mut date, false, &mut local_error, backend)?;
                if date.date_type != DateType::Invalid {
                    let month = defined(date.month, "actual DST start PMonth").map_err(raw)?;
                    let day = defined(date.day, "actual DST start PDay").map_err(raw)?;
                    if month == 0 && day == 0 {
                        state.epw_daylight_saving = false;
                    } else {
                        state.epw_daylight_saving = true;
                        state.epw_dst.start_date_type = date.date_type;
                        state.epw_dst.start_month = month;
                        state.epw_dst.start_day = day;
                        state.epw_dst.start_weekday =
                            defined(date.weekday, "actual DST start PWeekDay").map_err(raw)?;
                    }
                } else {
                    backend.show_continue_error(&format!("ProcessEPWHeader: Invalid Daylight Saving Period Start Date Field(WeatherFile)={token}"))?;
                    backend.show_continue_error("...invalid header=HOLIDAYS/DAYLIGHT SAVING")?;
                    backend.show_continue_error("...Setting Weather File DST to false.")?;
                    state.epw_daylight_saving = false;
                }
            }
            3 => {
                let mut date = SourceDateScratch::default();
                process_source_date_string(token, &mut date, false, errors, backend)?;
                if state.epw_daylight_saving {
                    if date.date_type != DateType::Invalid {
                        state.epw_dst.end_date_type = date.date_type;
                        state.epw_dst.end_month =
                            defined(date.month, "actual DST end PMonth").map_err(raw)?;
                        state.epw_dst.end_day =
                            defined(date.day, "actual DST end PDay").map_err(raw)?;
                        state.epw_dst.end_weekday =
                            defined(date.weekday, "actual DST end PWeekDay").map_err(raw)?;
                    } else {
                        backend.show_warning_error(&format!("ProcessEPWHeader: Invalid Daylight Saving Period End Date Field(WeatherFile)={token}"))?;
                        backend.show_continue_error("...Setting Weather File DST to false.")?;
                        state.epw_daylight_saving = false;
                    }
                    state.dst = state.epw_dst.clone();
                }
            }
            4 => {
                let epw_count = to_source_int(number_with_error(token).0).map_err(raw)?;
                let idf_count = processor.get_num_objects_found("RunPeriodControl:SpecialDays");
                let idf_count = i32::try_from(idf_count)
                    .map_err(|_| Error::Unavailable("Native getNumObjectsFound i32 result7954"))?;
                state.number_special_days = epw_count
                    .checked_add(idf_count)
                    .ok_or(Error::Unavailable("Native NumSpecialDays addition7954"))?;
                state
                    .special_days
                    .allocate(source_count(state.number_special_days).map_err(raw)?);
                arguments = epw_count
                    .checked_mul(2)
                    .and_then(|v| v.checked_add(4))
                    .ok_or(Error::Unavailable("Native NumHdArgs arithmetic7956"))?;
            }
            _ if field % 2 != 0 => {
                current = current
                    .checked_add(1)
                    .ok_or(Error::Unavailable("Native CurCount increment7960"))?;
                if current > source_count(state.number_special_days).map_err(raw)? {
                    backend.show_severe_error("Too many SpecialDays")?;
                    *errors = true;
                } else {
                    state
                        .special_days
                        .values
                        .get_mut(current - 1)
                        .ok_or(Error::Unavailable("actual SpecialDays row7965"))?
                        .name = token.to_owned();
                }
            }
            _ => {
                if current <= source_count(state.number_special_days).map_err(raw)? {
                    let day = state
                        .special_days
                        .values
                        .get_mut(current.checked_sub(1).ok_or(Error::Unavailable(
                            "actual SpecialDays one-based ordinal7970",
                        ))?)
                        .ok_or(Error::Unavailable("actual SpecialDays row7970"))?;
                    let mut date = SourceDateScratch::default();
                    process_source_date_string(token, &mut date, false, errors, backend)?;
                    if date.date_type == DateType::MonthDay {
                        day.date_type = date.date_type;
                        day.month = defined(date.month, "actual SpecialDay PMonth").map_err(raw)?;
                        day.day = defined(date.day, "actual SpecialDay PDay").map_err(raw)?;
                        day.weekday = 0;
                        day.compressed_date = day
                            .month
                            .checked_mul(32)
                            .and_then(|v| v.checked_add(day.day))
                            .ok_or(Error::Unavailable(
                                "Native SpecialDay CompDate arithmetic7979",
                            ))?;
                        day.duration = 1;
                        day.day_type = 1;
                        day.weather_file = true;
                    } else if date.date_type != DateType::Invalid {
                        day.date_type = date.date_type;
                        day.month = defined(date.month, "actual SpecialDay PMonth").map_err(raw)?;
                        day.day = defined(date.day, "actual SpecialDay PDay").map_err(raw)?;
                        day.weekday =
                            defined(date.weekday, "actual SpecialDay PWeekDay").map_err(raw)?;
                        day.compressed_date = 0;
                        day.duration = 1;
                        day.day_type = 1;
                        day.weather_file = true;
                    } else {
                        backend.show_severe_error(&format!(
                            "Invalid SpecialDay Date Field(WeatherFile)={token}"
                        ))?;
                        *errors = true;
                    }
                }
            }
        }
        erase_field(line, end);
        field = field
            .checked_add(1)
            .ok_or(Error::Unavailable("Native header i increment"))?;
    }
    for ordinal in 0..source_count(state.number_typical_extreme_sets).map_err(raw)? {
        let period = state
            .typical_extreme_periods
            .values
            .get_mut(ordinal)
            .ok_or(Error::Unavailable("actual TypicalExtremePeriods row8005"))?;
        period.start_julian_day =
            source_ordinal_day(period.start_month, period.start_day, state.leap_year_add)?;
        period.end_julian_day =
            source_ordinal_day(period.end_month, period.end_day, state.leap_year_add)?;
        period.total_days = if period.start_julian_day <= period.end_julian_day {
            period
                .end_julian_day
                .checked_sub(period.start_julian_day)
                .and_then(|v| v.checked_add(1))
        } else {
            source_ordinal_day(12, 31, state.leap_year_add)?
                .checked_sub(period.start_julian_day)
                .and_then(|v| v.checked_add(1))
                .and_then(|v| v.checked_add(period.end_julian_day))
        }
        .ok_or(Error::Unavailable(
            "Native TypicalExtreme TotalDays arithmetic8010",
        ))?;
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
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    line.make_ascii_uppercase();
    let mut arguments = 2_i32;
    let mut current = 0_usize;
    let mut field = 1_i32;
    while field <= arguments {
        let end = input.field_end(line).map_err(raw)?;
        let token = &line[..end];
        if field == 1 {
            state.number_data_periods = to_source_int(number_with_error(token).0).map_err(raw)?;
            state
                .data_periods
                .allocate(source_count(state.number_data_periods).map_err(raw)?);
            arguments = state
                .number_data_periods
                .checked_mul(4)
                .and_then(|v| v.checked_add(2))
                .ok_or(Error::Unavailable("Native DATA PERIODS NumHdArgs8054"))?;
            if state.number_data_periods > 0 {
                for period in &mut state.data_periods.values {
                    period.number_days = 0;
                }
            }
        } else if field == 2 {
            state.intervals_per_hour = to_source_int(number_with_error(token).0).map_err(raw)?;
        } else {
            let part = (field - 3) % 4;
            if part == 0 {
                current = current
                    .checked_add(1)
                    .ok_or(Error::Unavailable("Native DataPeriods CurCount8072"))?;
                if current > source_count(state.number_data_periods).map_err(raw)? {
                    backend.show_severe_error("Too many data periods")?;
                    *errors = true;
                } else {
                    state
                        .data_periods
                        .values
                        .get_mut(current - 1)
                        .ok_or(Error::Unavailable("actual DataPeriod Name row8077"))?
                        .name = token.to_owned();
                }
            } else if part == 1 {
                if current <= source_count(state.number_data_periods).map_err(raw)? {
                    let period = state
                        .data_periods
                        .values
                        .get_mut(
                            current
                                .checked_sub(1)
                                .ok_or(Error::Unavailable("actual DataPeriod ordinal8084"))?,
                        )
                        .ok_or(Error::Unavailable("actual DataPeriod row8084"))?;
                    period.day_of_week = token.to_owned();
                    period.weekday = WEEKDAY_NAMES
                        .iter()
                        .position(|name| *name == period.day_of_week)
                        .map_or(-1, |v| v as i32);
                    if period.weekday < 1 || period.weekday > 7 {
                        backend.show_severe_error(&format!("Weather File -- Invalid Start Day of Week for Data Period #{current}, Invalid day={}",period.day_of_week))?;
                        *errors = true;
                    }
                }
            } else {
                // Native obtains the end-date reference before its count guard.
                // The start-date reference is obtained only inside the guard.
                let allowed = current <= source_count(state.number_data_periods).map_err(raw)?;
                if part == 3 || allowed {
                    let period = state
                        .data_periods
                        .values
                        .get_mut(current.checked_sub(1).ok_or(Error::Unavailable(
                            "actual DataPeriod one-based date ordinal",
                        ))?)
                        .ok_or(Error::Unavailable(
                            "actual DataPeriod end-date reference8121",
                        ))?;
                    if allowed {
                        let mut date = SourceDateScratch::default();
                        process_source_date_string(token, &mut date, true, errors, backend)?;
                        if date.date_type == DateType::MonthDay {
                            if part == 2 {
                                period.start_month =
                                    defined(date.month, "actual DataPeriod start PMonth")
                                        .map_err(raw)?;
                                period.start_day =
                                    defined(date.day, "actual DataPeriod start PDay")
                                        .map_err(raw)?;
                                period.start_year =
                                    defined(date.year, "actual DataPeriod start PYear")
                                        .map_err(raw)?;
                                if period.start_year != 0 {
                                    period.has_year_data = true;
                                }
                            } else {
                                period.end_month =
                                    defined(date.month, "actual DataPeriod end PMonth")
                                        .map_err(raw)?;
                                period.end_day =
                                    defined(date.day, "actual DataPeriod end PDay").map_err(raw)?;
                                period.end_year = defined(date.year, "actual DataPeriod end PYear")
                                    .map_err(raw)?;
                                if period.end_year == 0 && period.has_year_data {
                                    backend.show_warning_error("Data Period (WeatherFile) - Start Date contains year. End Date does not.")?;
                                    backend.show_continue_error(
                                        "...Assuming same year as Start Date for this data.",
                                    )?;
                                    period.end_year = period.start_year;
                                }
                            }
                        } else {
                            let text = if part == 2 {
                                "Data Periods must be of the form <DayOfYear> or <Month Day> (WeatherFile), found="
                            } else {
                                "Data Periods must be of the form <DayOfYear> or <Month Day>, (WeatherFile) found="
                            };
                            backend.show_severe_error(&format!("{text}{token}"))?;
                            *errors = true;
                        }
                    }
                    if part == 3 {
                        finish_source_period(
                            period,
                            *errors,
                            &state.month_ends,
                            state.leap_year_add,
                        )?;
                    }
                }
            }
        }
        erase_field(line, end);
        field = field
            .checked_add(1)
            .ok_or(Error::Unavailable("Native DataPeriods i increment"))?;
    }
    Ok(())
}

fn finish_source_period(
    period: &mut super::DataPeriod,
    errors: bool,
    ends: &[i32; 12],
    leap: i32,
) -> Result<(), Failure> {
    if period.start_year == 0 || period.end_year == 0 {
        period.start_julian_day = source_ordinal_day(period.start_month, period.start_day, leap)?;
        period.end_julian_day = source_ordinal_day(period.end_month, period.end_day, leap)?;
        period.number_days = if period.start_julian_day <= period.end_julian_day {
            period
                .end_julian_day
                .checked_sub(period.start_julian_day)
                .and_then(|v| v.checked_add(1))
        } else {
            365_i32
                .checked_sub(period.start_julian_day)
                .and_then(|v| v.checked_add(1))
                .and_then(|v| {
                    period
                        .end_julian_day
                        .checked_sub(1)
                        .and_then(|w| w.checked_add(1))
                        .and_then(|w| v.checked_add(w))
                })
        }
        .ok_or(Error::Unavailable(
            "Native DataPeriod NumDays arithmetic8143",
        ))?;
    } else {
        period.start_julian_day =
            source_julian_date(period.start_year, period.start_month, period.start_day)?;
        period.end_julian_day =
            source_julian_date(period.end_year, period.end_month, period.end_day)?;
        period.number_days = period
            .end_julian_day
            .checked_sub(period.start_julian_day)
            .and_then(|v| v.checked_add(1))
            .ok_or(Error::Unavailable(
                "Native actual-year DataPeriod NumDays8149",
            ))?;
    }
    period.month_weekdays = [0; 12];
    if !errors {
        setup_weekdays(
            period.start_month,
            period.start_day,
            period.weekday,
            ends,
            leap,
            &mut period.month_weekdays,
        )
        .map_err(raw)?;
    }
    Ok(())
}

pub(crate) fn source_julian_date(year: i32, month: i32, day: i32) -> Result<i32, Failure> {
    // Native8533–8557 signed-int expression; stop before undefined overflow.
    let fail = || Error::Unavailable("Native computeJulianDate signed-int domain8557");
    let l = month.checked_sub(14).ok_or_else(fail)? / 12;
    let a = day.checked_sub(32075).ok_or_else(fail)?;
    let b = year
        .checked_add(4800)
        .and_then(|v| v.checked_add(l))
        .and_then(|v| v.checked_mul(1461))
        .ok_or_else(fail)?
        / 4;
    let c = month
        .checked_sub(2)
        .and_then(|v| l.checked_mul(12).and_then(|w| v.checked_sub(w)))
        .and_then(|v| v.checked_mul(367))
        .ok_or_else(fail)?
        / 12;
    let d = year
        .checked_add(4900)
        .and_then(|v| v.checked_add(l))
        .map(|v| v / 100)
        .and_then(|v| v.checked_mul(3))
        .ok_or_else(fail)?
        / 4;
    Ok(a.checked_add(b)
        .and_then(|v| v.checked_add(c))
        .and_then(|v| v.checked_sub(d))
        .ok_or_else(fail)?)
}
