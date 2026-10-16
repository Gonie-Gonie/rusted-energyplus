//! Actual GetNextEnvironment864-1174 on the live Environment and raw header.
use super::{Failure, fatal, ordinal};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::weather::day::{SourceEnvironmentData, WeatherEnvironmentState, WeatherGlobalState};
use crate::weather::raw::{RawEpwHeaderState, source_ordinal_day};
use std::io::Write;

/// Disjoint fields of the same WeatherManager; Environment remains its real row.
pub(super) struct SourceRunEnvironmentCalendarOwners<'a> {
    pub(super) use_daylight_saving: &'a mut bool,
    pub(super) use_special_days: &'a mut bool,
    pub(super) use_rain_values: &'a mut bool,
    pub(super) use_snow_values: &'a mut bool,
    pub(super) daylight_saving_is_active: &'a mut bool,
    pub(super) end_day_of_month_with_leap_day: &'a mut [i32; 12],
    pub(super) weekday_types: &'a mut [i32; 366],
    pub(super) special_day_types: &'a mut [i32; 366],
    pub(super) current_simulation_day_for_end_of_run_period: &'a mut i32,
    pub(super) start_dates_cycle_should_be_reset: &'a mut bool,
    pub(super) jan1_dates_should_be_reset: &'a mut bool,
}
fn between_dates(test: i32, start: i32, end: i32) -> bool {
    let mut between = false;
    if start <= end {
        if test >= start && test <= end {
            between = true;
        }
    } else if test <= end || test >= start {
        between = true;
    }
    between
}
pub(super) fn date(month: i32, day: i32) -> String {
    format!("{month:02}/{day:02}")
}
fn date_with_year(month: i32, day: i32, year: i32) -> String {
    format!("{month:02}/{day:02}/{year:04}")
}
pub(super) const DAY_NAMES: [&str; 13] = [
    "Unused",
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Holiday",
    "SummerDesignDay",
    "WinterDesignDay",
    "CustomDay1",
    "CustomDay2",
];
pub(super) const SKY_NAMES: [&str; 7] = [
    "Clark and Allen",
    "Schedule Value",
    "DryBulb Difference Schedule Value",
    "Dewpoint Difference Schedule Value",
    "Brunt",
    "Idso",
    "Berdahl and Martin",
];

pub(super) fn initialize_source_run_environment(
    row: &mut SourceEnvironmentData,
    environment: &mut WeatherEnvironmentState,
    calendar: &mut SourceRunEnvironmentCalendarOwners<'_>,
    header: &mut RawEpwHeaderState,
    clock: &WeatherGlobalState,
    globals: &SelectedHeatBalanceGlobals,
    control: &SelectedHeatBalanceInputState,
    reports: &SourceSolarReportFlags,
    eio: &mut dyn Write,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    let dst_actual_start_month = 0;
    let dst_actual_start_day = 0;
    let dst_actual_end_month = 0;
    let dst_actual_end_day = 0;
    let kind_of_run_period = row.kind_of_environment_character.clone();
    environment.run_period_environment = clock.kind_of_sim == 3;
    environment.current_year_is_leap_year = row.is_leap_year;
    header.leap_year_add = if environment.current_year_is_leap_year && header.allows_leap_years {
        1
    } else {
        0
    };
    if environment.current_year_is_leap_year {
        calendar.end_day_of_month_with_leap_day[1] = header.month_ends[1]
            .checked_add(header.leap_year_add)
            .ok_or(Error::Unavailable("actual February end-day arithmetic879"))?;
    }
    *calendar.use_daylight_saving = row.use_daylight_saving;
    *calendar.use_special_days = row.use_holidays;
    *calendar.use_rain_values = row.use_rain;
    *calendar.use_snow_values = row.use_snow;
    let mut missing_leap = false;
    if row.actual_weather && !header.allows_leap_years {
        let mut year = row.start_year;
        while year <= row.end_year {
            if is_leap_year(year) {
                backend.show_severe_error(&format!("GetNextEnvironment: Weatherfile does not support leap years but runperiod includes a leap year ({year})"))?;
                missing_leap = true;
            }
            year = year.checked_add(1).ok_or(Error::Unavailable(
                "actual GetNextEnvironment year increment889",
            ))?;
        }
    }
    let mut okay = false;
    if row.actual_weather {
        for period in &header.data_periods.values {
            let start = period.start_julian_day;
            let end = period.end_julian_day;
            if !period.has_year_data {
                backend.show_severe_error("GetNextEnvironment: Actual weather runperiod has been entered but weatherfile DATA PERIOD does not have year included in start/end date.")?;
                backend.show_continue_error(
                    "...to match the RunPeriod, the DATA PERIOD should be mm/dd/yyyy for both, or",
                )?;
                backend.show_continue_error("(...set \"Treat Weather as Actual\" to \"No\".)")?;
            }
            if !between_dates(row.start_date, start, end) {
                continue;
            }
            if !between_dates(row.end_date, start, end) {
                continue;
            }
            okay = true;
            break;
        }
    } else {
        for period in &header.data_periods.values {
            let start =
                source_ordinal_day(period.start_month, period.start_day, header.leap_year_add)?;
            let end = source_ordinal_day(period.end_month, period.end_day, header.leap_year_add)?;
            if start == 1 && (end == 366 || end == 365) {
                okay = true;
                break;
            }
            if !between_dates(row.start_julian_day, start, end) {
                continue;
            }
            if !between_dates(row.end_julian_day, start, end) {
                continue;
            }
            okay = true;
        }
    }
    if !okay {
        if !row.actual_weather {
            let start = date(row.start_month, row.start_day);
            let end = date(row.end_month, row.end_day);
            backend.show_severe_error(&format!("GetNextEnvironment: Runperiod [mm/dd] (Start={start},End={end}) requested not within Data Period(s) from Weather File"))?;
        } else {
            let start = date_with_year(row.start_month, row.start_day, row.start_year);
            let end = date_with_year(row.end_month, row.end_day, row.end_year);
            backend.show_severe_error(&format!("GetNextEnvironment: Runperiod [mm/dd/yyyy] (Start={start},End={end}) requested not within Data Period(s) from Weather File"))?;
        }
        let period = header
            .data_periods
            .values
            .first()
            .ok_or(Error::Unavailable(
                "actual first DataPeriod reference973 after reached severe",
            ))?;
        let mut start = date(period.start_month, period.start_day);
        let mut end = date(period.end_month, period.end_day);
        if period.start_year > 0 {
            start.push_str(&format!("/{}", period.start_year));
        } else {
            start.push_str("/<noyear>");
        }
        if period.end_year > 0 {
            end.push_str(&format!("/{}", period.end_year));
        } else {
            end.push_str("/<noyear>");
        }
        if header.number_data_periods == 1 {
            backend
                .show_continue_error(&format!("Weather Data Period (Start={start},End={end})"))?;
        } else {
            backend.show_continue_error(&format!(
                "Multiple Weather Data Periods 1st (Start={start},End={end})"
            ))?;
        }
        return Err(fatal(
            backend,
            "GetNextEnvironment: Program terminates due to preceding condition.",
        ));
    }
    if missing_leap {
        return Err(fatal(
            backend,
            "GetNextEnvironment: Program terminates due to preceding condition.",
        ));
    }
    let mut start = date(row.start_month, row.start_day);
    let mut end = date(row.end_month, row.end_day);
    if row.kind_of_environment == 3 {
        start.push_str(&format!("/{}", row.start_year));
        end.push_str(&format!("/{}", row.end_year));
    }
    environment.environment_start_end = format!("{start} - {end}");
    environment.start_year = row.start_year;
    environment.end_year = row.end_year;
    let mut weekday = if row.day_of_week == 0 {
        1
    } else {
        row.day_of_week
    };
    let month_weekdays = &row.month_week_day;
    if reports.do_weather_init_reporting {
        let day = DAY_NAMES
            .get(
                usize::try_from(weekday)
                    .map_err(|_| Error::Unavailable("actual report weekday index1026"))?,
            )
            .ok_or(Error::Unavailable("actual report weekday index1026"))?;
        let sky = SKY_NAMES
            .get(
                usize::try_from(row.sky_temperature_model)
                    .map_err(|_| Error::Unavailable("actual report sky enum1034"))?,
            )
            .ok_or(Error::Unavailable("actual report sky enum1034"))?;
        let yes_no = |value| if value { "Yes" } else { "No" };
        writeln!(
            eio,
            "Environment,{},{},{},{},{},{},Use RunPeriod Specified Day,{},{},{},{},{},{}",
            row.title,
            kind_of_run_period,
            start,
            end,
            day,
            row.total_days,
            yes_no(row.use_daylight_saving),
            yes_no(row.use_holidays),
            yes_no(row.apply_weekend_rule),
            yes_no(row.use_rain),
            yes_no(row.use_snow),
            sky
        )
        .map_err(Error::Io)?;
    }
    if !globals.doing_sizing
        && !globals.kickoff_simulation
        && clock.kind_of_sim == 3
        && clock.do_weath_sim
        && (control.adaptive_comfort_requested_ash55 || control.adaptive_comfort_requested_cen15251)
    {
        // Earlier actual adaptive initialization810/813 has already reached its
        // positive unavailable boundary on this same predicate before this point.
        return Err(Error::Unavailable("actual positive adaptive weather diagnostics1040").into());
    }
    environment.run_period_start_day_of_week = weekday;
    calendar.weekday_types.fill(0);
    let first_day = source_ordinal_day(row.start_month, row.start_day, header.leap_year_add)?;
    let last_day = source_ordinal_day(row.end_month, row.end_day, header.leap_year_add)?;
    *calendar.current_simulation_day_for_end_of_run_period = row.total_days;
    // Native's end-day is tested AFTER increment and is deliberately not filled
    // on a distinct start/end pair. Equal start/end genuinely fills all366 slots.
    if !(1..=366).contains(&last_day) {
        return Err(Error::Unavailable("actual nonterminating weekday end-day domain1099").into());
    }
    let mut day = first_day;
    loop {
        *calendar
            .weekday_types
            .get_mut(ordinal(day)?)
            .ok_or(Error::Unavailable(
                "actual WeekDayTypes one-based index1093",
            ))? = weekday;
        weekday = weekday % 7 + 1;
        day = day
            .checked_add(1)
            .ok_or(Error::Unavailable("actual weekday index increment1095"))?;
        if day > 366 {
            day = 1;
        }
        if day == last_day {
            break;
        }
    }
    *calendar.daylight_saving_is_active =
        (*calendar.use_daylight_saving && header.epw_daylight_saving) || header.idf_daylight_saving;
    row.set_week_days = false;
    if *calendar.daylight_saving_is_active {
        return Err(Error::Unavailable("actual positive SetDSTDateRanges1110").into());
    }
    // Actual SetSpecialDayDates1642-1651 prefix, including mandatory reset.
    let special_errors = false;
    calendar.special_day_types.fill(0);
    for number in 1..=header.number_special_days {
        let special = header
            .special_days
            .values
            .get_mut(ordinal(number)?)
            .ok_or(Error::Unavailable("actual SpecialDays reference1647"))?;
        if special.weather_file && !*calendar.use_special_days {
            continue;
        }
        return Err(
            Error::Unavailable("actual positive SetSpecialDayDates date resolution1651").into(),
        );
    }
    if special_errors {
        return Err(fatal(
            backend,
            "SetSpecialDayDates: Program terminates due to preceding condition(s).",
        ));
    }
    let _ = month_weekdays; // Borrowed real array for the genuine positive children.
    if row.start_month != 1 || row.start_day != 1 {
        *calendar.start_dates_cycle_should_be_reset = true;
        *calendar.jan1_dates_should_be_reset = true;
    }
    if row.start_month == 1 && row.start_day == 1 {
        *calendar.start_dates_cycle_should_be_reset = false;
        *calendar.jan1_dates_should_be_reset = true;
    }
    if row.actual_weather {
        *calendar.start_dates_cycle_should_be_reset = false;
        *calendar.jan1_dates_should_be_reset = true;
    }
    if !globals.kickoff_simulation {
        let mut source = String::new();
        if *calendar.use_daylight_saving {
            if header.epw_daylight_saving {
                source = "WeatherFile".into();
            }
        } else {
            source = "RunPeriod Object".into();
        }
        if header.idf_daylight_saving {
            source = "InputFile".into();
        }
        if *calendar.daylight_saving_is_active && reports.do_weather_init_reporting {
            let start = date(dst_actual_start_month, dst_actual_start_day);
            let end = date(dst_actual_end_month, dst_actual_end_day);
            writeln!(
                eio,
                "Environment:Daylight Saving,Yes,{source},{start},{end}"
            )
            .map_err(Error::Io)?;
        } else if clock.do_output_reporting {
            writeln!(eio, "Environment:Daylight Saving,No,{source}").map_err(Error::Io)?;
        }
        for number in 1..=header.number_special_days {
            let special = header
                .special_days
                .values
                .get_mut(ordinal(number)?)
                .ok_or(Error::Unavailable("actual special report row1151"))?;
            if special.weather_file
                && *calendar.use_special_days
                && reports.do_weather_init_reporting
            {
                return Err(Error::Unavailable(
                    "actual positive weather SpecialDay date report1154",
                )
                .into());
            }
            if !special.weather_file && reports.do_weather_init_reporting {
                return Err(
                    Error::Unavailable("actual positive input SpecialDay date report1164").into(),
                );
            }
        }
    }
    Ok(())
}
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
