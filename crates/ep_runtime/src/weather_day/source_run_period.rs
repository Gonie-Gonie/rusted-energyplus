//! Actual retained WeatherManager RunPeriodData and GetRunPeriodData scalar helpers.
use super::{SourceWeatherInitializationError as Failure, WeatherEnvironmentState};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

/// Native WeatherManager.hh275–305; allocation belongs to actual GetRunPeriodData.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceRunPeriodData {
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub title: String,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub period_type: String,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub total_days: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub start_month: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub start_day: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub start_julian_date: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub start_year: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub end_month: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub end_day: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub end_julian_date: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub end_year: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub day_of_week: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub start_weekday: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub use_dst: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub use_holidays: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub apply_weekend_rule: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub use_rain: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub use_snow: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub month_week_day: [i32; 12],
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub number_simulation_years: i32,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub is_leap_year: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub roll_day_type_on_repeat: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub treat_years_as_consecutive: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub actual_weather: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub first_hour_interpolation_uses_hour_one: bool,
}
impl Default for SourceRunPeriodData {
    fn default() -> Self {
        Self {
            title: String::new(),
            period_type: String::new(),
            total_days: 365,
            start_month: 1,
            start_day: 1,
            start_julian_date: 2457755,
            start_year: 2017,
            end_month: 12,
            end_day: 31,
            end_julian_date: 2458119,
            end_year: 2017,
            day_of_week: 1,
            start_weekday: 1,
            use_dst: false,
            use_holidays: false,
            apply_weekend_rule: false,
            use_rain: true,
            use_snow: true,
            month_week_day: [1, 4, 4, 7, 2, 5, 7, 3, 6, 1, 4, 6],
            number_simulation_years: 1,
            is_leap_year: false,
            roll_day_type_on_repeat: true,
            treat_years_as_consecutive: true,
            actual_weather: false,
            first_hour_interpolation_uses_hour_one: false,
        }
    }
}

pub(super) fn checked(value: Option<i32>) -> Result<i32, Failure> {
    Ok(value.ok_or(Error::Unavailable(
        "actual WeatherManager defined signed date arithmetic",
    ))?)
}
pub(super) fn leap_year(year: i32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}
fn day_of_year(month: i32, day: i32, leap: bool) -> Result<i32, Failure> {
    const NORMAL: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const LEAP: [i32; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
    let ordinal = usize::try_from(checked(month.checked_sub(1))?)
        .map_err(|_| Error::Unavailable("actual calculateDayOfYear month index8631"))?;
    let offset = if leap {
        LEAP.get(ordinal)
    } else {
        NORMAL.get(ordinal)
    }
    .ok_or(Error::Unavailable(
        "actual calculateDayOfYear month index8631",
    ))?;
    checked(offset.checked_add(day))
}
fn year_for_weekday(month: i32, day: i32, weekday: i32, leap: bool) -> Result<i32, Failure> {
    const NORMAL: [i32; 13] = [
        2013, 2014, 2015, 2010, 2011, 2017, 2007, 2013, 2014, 2015, 2010, 2011, 2017,
    ];
    const LEAP: [i32; 13] = [
        2008, 1992, 2004, 2016, 2000, 2012, 1996, 2008, 1992, 2004, 2016, 2000, 2012,
    ];
    let remainder = day_of_year(month, day, leap)? % 7;
    let ordinal = usize::try_from(checked(
        weekday
            .checked_sub(remainder)
            .and_then(|v| v.checked_add(5)),
    )?)
    .map_err(|_| Error::Unavailable("actual findYearForWeekday lookup index"))?;
    Ok(*(if leap {
        LEAP.get(ordinal)
    } else {
        NORMAL.get(ordinal)
    })
    .ok_or(Error::Unavailable("actual findYearForWeekday lookup index"))?)
}
pub(super) fn valid_month_day(month: i32, day: i32) -> bool {
    // Native does not test day>=1 here; InputProcessor validation is separate.
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11 => day <= 30,
        2 => day <= 28,
        _ => false,
    }
}
fn calculate_weekday(
    environment: &mut WeatherEnvironmentState,
    year: i32,
    month: i32,
    day: i32,
) -> Result<i32, Failure> {
    let mut adjusted_year = year;
    let mut adjusted_month = month;
    if adjusted_month < 3 {
        adjusted_month = checked(adjusted_month.checked_add(12))?;
        adjusted_year = checked(adjusted_year.checked_sub(1))?;
    }
    let a = checked(
        adjusted_month
            .checked_add(1)
            .and_then(|v| v.checked_mul(13)),
    )? / 5;
    let b = checked((adjusted_year / 100).checked_mul(6))?;
    environment.day_of_week = checked(
        day.checked_add(a)
            .and_then(|v| v.checked_add(adjusted_year))
            .and_then(|v| v.checked_add(adjusted_year / 4))
            .and_then(|v| v.checked_add(b))
            .and_then(|v| v.checked_add(adjusted_year / 400)),
    )? % 7;
    if environment.day_of_week == 0 {
        environment.day_of_week = 7;
    }
    Ok(environment.day_of_week)
}

use super::source_initialization::SourceWeatherInitializationOwners;
use super::{SourceEnvironmentData, WeatherSession};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_input_processor::SourceInputProcessorShortCut;
use crate::weather::raw::{source_julian_date, source_ordinal_day};

const DAY_TYPES: [&str; 13] = [
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
fn alpha(s: &SourceInputProcessorShortCut, n: usize) -> Result<&str, Failure> {
    Ok(s.alpha_args.get(n - 1).ok_or(Error::Unavailable(
        "actual RunPeriod shared alpha allocation",
    ))?)
}
fn blank(s: &SourceInputProcessorShortCut, n: usize) -> Result<bool, Failure> {
    Ok(*s.alpha_field_blanks.get(n - 1).ok_or(Error::Unavailable(
        "actual RunPeriod shared alpha blank allocation",
    ))?)
}
fn field_name(s: &SourceInputProcessorShortCut, n: usize) -> Result<&str, Failure> {
    Ok(s.alpha_field_names.get(n - 1).ok_or(Error::Unavailable(
        "actual RunPeriod shared alpha-name allocation",
    ))?)
}
fn numeric_int(s: &SourceInputProcessorShortCut, n: usize) -> Result<i32, Failure> {
    let value = *s.numeric_args.get(n - 1).ok_or(Error::Unavailable(
        "actual RunPeriod shared numeric allocation",
    ))?;
    let value = value.trunc();
    if !value.is_finite() || value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
        return Err(
            Error::Unavailable("actual RunPeriod native float-to-int defined domain").into(),
        );
    }
    Ok(value as i32)
}
fn selected_bool(
    s: &SourceInputProcessorShortCut,
    n: usize,
    title: &str,
    default: bool,
    backend: &mut SourceErrorBackend<'_>,
    errors: &mut bool,
) -> Result<Option<bool>, Failure> {
    if blank(s, n)? {
        return Ok(Some(default));
    }
    let value = alpha(s, n)?.to_ascii_uppercase();
    match value.as_str() {
        "YES" => Ok(Some(true)),
        "NO" => Ok(Some(false)),
        _ => {
            backend.show_severe_error(&format!(
                "GetRunPeriodData: {} = {title}",
                s.current_module_object
            ))?;
            backend.show_continue_error(&format!(
                "{} = {}, invalid boolean (\"Yes\"/\"No\").",
                field_name(s, n)?,
                alpha(s, n)?
            ))?;
            *errors = true;
            Ok(None)
        }
    }
}
fn checked_weekday_name(weekday: i32) -> Result<&'static str, Failure> {
    let n = usize::try_from(weekday)
        .map_err(|_| Error::Unavailable("actual Sched dayTypeNamesUC index"))?;
    Ok(DAY_TYPES
        .get(n)
        .copied()
        .ok_or(Error::Unavailable("actual Sched dayTypeNamesUC index"))?)
}
fn corrected_weekday(
    row: &mut SourceRunPeriodData,
    s: &SourceInputProcessorShortCut,
    environment: &mut WeatherEnvironmentState,
    input_weekday: bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Failure> {
    let weekday = calculate_weekday(environment, row.start_year, row.start_month, row.start_day)?;
    if input_weekday {
        if weekday != row.start_weekday {
            backend.show_warning_error(&format!("{}: object={}, start weekday ({}) does not match the start year ({}), corrected to {}.",s.current_module_object,row.title,alpha(s,2)?,row.start_year,checked_weekday_name(weekday)?))?;
            row.start_weekday = weekday;
        }
    } else {
        row.start_weekday = weekday;
    }
    Ok(())
}

impl WeatherSession {
    // Keep Original explicit branch assignments and request order.
    #[allow(clippy::needless_bool_assign)]
    pub(super) fn get_source_run_period_data(
        &mut self,
        mut n_run_periods: i32,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        errors: &mut bool,
    ) -> Result<(), Failure> {
        let n = usize::try_from(n_run_periods)
            .map_err(|_| Error::Unavailable("actual RunPeriodInput allocation5031"))?;
        self.state.weather.source_run_period_input = Some(vec![SourceRunPeriodData::default(); n]);
        owners.shortcut.current_module_object = "RunPeriod".into();
        let mut count = 0_i32;
        for ordinal in 0..n {
            owners.raw.get_shared_source_object_item(
                "RunPeriod",
                ordinal,
                owners.input,
                owners.shortcut,
            )?;
            let new_name = alpha(owners.shortcut, 1)?.to_ascii_uppercase();
            let rows = self
                .state
                .weather
                .source_run_period_input
                .as_mut()
                .ok_or(Error::Unavailable("actual RunPeriodInput array5031"))?;
            if rows.iter().any(|row| row.title == new_name) {
                owners.backend.show_severe_error(&format!(
                    "GetRunPeriodData: {} = {new_name}, duplicate name.",
                    owners.shortcut.current_module_object
                ))?;
                *errors = true;
            }
            count = checked(count.checked_add(1))?;
            let row = rows
                .get_mut(ordinal)
                .ok_or(Error::Unavailable("actual RunPeriodInput row5066"))?;
            row.title = alpha(owners.shortcut, 1)?.to_owned();
            row.start_month = numeric_int(owners.shortcut, 1)?;
            row.start_day = numeric_int(owners.shortcut, 2)?;
            row.start_year = numeric_int(owners.shortcut, 3)?;
            row.end_month = numeric_int(owners.shortcut, 4)?;
            row.end_day = numeric_int(owners.shortcut, 5)?;
            row.end_year = numeric_int(owners.shortcut, 6)?;
            row.treat_years_as_consecutive = true;
            if owners.system.full_annual_run && ordinal == 0 {
                row.start_month = 1;
                row.start_day = 1;
                row.end_month = 12;
                row.end_day = 31;
            }
            if row.start_year == 0 {
                if row.end_year != 0 {
                    owners.backend.show_severe_error(&format!(
                        "{}: object={}, end year cannot be specified if the start year is not.",
                        owners.shortcut.current_module_object, row.title
                    ))?;
                    *errors = true;
                }
            } else if row.start_year < 1583 {
                owners.backend.show_severe_error(&format!(
                    "{}: object={}, start year ({}) is too early, please choose a date after 1582.",
                    owners.shortcut.current_module_object, row.title, row.start_year
                ))?;
                *errors = true;
            }
            if row.end_year != 0 && row.start_year > row.end_year {
                owners.backend.show_severe_error(&format!(
                    "{}: object={}, start year ({}) is after the end year ({}).",
                    owners.shortcut.current_module_object, row.title, row.start_year, row.end_year
                ))?;
                *errors = true;
            }
            let mut input_weekday = false;
            if !blank(owners.shortcut, 2)? {
                let day_name = alpha(owners.shortcut, 2)?;
                let day_type = DAY_TYPES
                    .iter()
                    .position(|v| *v == day_name)
                    .map_or(-1, |v| v as i32);
                if day_type < 1 {
                    owners.backend.show_warning_error(&format!("{}: object={}{} invalid (Day of Week) [{}] for Start is not valid, Sunday will be used.",owners.shortcut.current_module_object,row.title,field_name(owners.shortcut,2)?,alpha(owners.shortcut,2)?))?;
                    row.start_weekday = 1;
                } else {
                    row.start_weekday = day_type;
                    input_weekday = true;
                }
            } else {
                row.start_weekday = 1;
            }
            if row.start_month == 2 && row.start_day == 29 {
                if row.start_year == 0 {
                    if input_weekday {
                        row.start_year = year_for_weekday(
                            row.start_month,
                            row.start_day,
                            row.start_weekday,
                            true,
                        )?;
                    } else {
                        row.start_year = 2012;
                        row.start_weekday = calculate_weekday(
                            &mut self.state.environment,
                            row.start_year,
                            row.start_month,
                            row.start_day,
                        )?;
                    }
                } else if !leap_year(row.start_year) {
                    owners.backend.show_severe_error(&format!("{}: object={}, start year ({}) is not a leap year but the requested start date is 2/29.",owners.shortcut.current_module_object,row.title,row.start_year))?;
                    *errors = true;
                } else {
                    corrected_weekday(
                        row,
                        owners.shortcut,
                        &mut self.state.environment,
                        input_weekday,
                        owners.backend,
                    )?;
                }
            } else if !valid_month_day(row.start_month, row.start_day) {
                owners.backend.show_severe_error(&format!(
                    "{}: object={}, Invalid input start month/day ({}/{})",
                    owners.shortcut.current_module_object,
                    row.title,
                    row.start_month,
                    row.start_day
                ))?;
                *errors = true;
            } else if row.start_year == 0 {
                if input_weekday {
                    row.start_year =
                        year_for_weekday(row.start_month, row.start_day, row.start_weekday, false)?;
                } else {
                    row.start_year = 2017;
                    row.start_weekday = calculate_weekday(
                        &mut self.state.environment,
                        row.start_year,
                        row.start_month,
                        row.start_day,
                    )?;
                }
            } else {
                corrected_weekday(
                    row,
                    owners.shortcut,
                    &mut self.state.environment,
                    input_weekday,
                    owners.backend,
                )?;
            }
            row.start_julian_date =
                source_julian_date(row.start_year, row.start_month, row.start_day)?;
            if row.end_month == 2 && row.end_day == 29 {
                if row.end_year == 0 {
                    if leap_year(row.start_year) && row.start_month < 3 {
                        row.end_year = row.start_year;
                    } else {
                        let mut year = checked(row.start_year.checked_add(1))?;
                        let limit = checked(row.start_year.checked_add(10))?;
                        while year < limit {
                            if leap_year(year) {
                                row.end_year = year;
                                break;
                            }
                            year = checked(year.checked_add(1))?;
                        }
                    }
                } else if !leap_year(row.end_year) {
                    // Native message deliberately prints startYear in this end-year diagnostic.
                    owners.backend.show_severe_error(&format!("{}: object={}, end year ({}) is not a leap year but the requested end date is 2/29.",owners.shortcut.current_module_object,row.title,row.start_year))?;
                    *errors = true;
                } else {
                    row.end_julian_date =
                        source_julian_date(row.end_year, row.end_month, row.end_day)?;
                    if row.start_julian_date > row.end_julian_date {
                        owners.backend.show_severe_error(&format!("{}: object={}, start Julian date ({}) is after the end Julian date ({}).",owners.shortcut.current_module_object,row.title,row.start_julian_date,row.end_julian_date))?;
                        *errors = true;
                    }
                }
            } else if !valid_month_day(row.end_month, row.end_day) {
                // Native prints the start date in its end-date diagnostic.
                owners.backend.show_severe_error(&format!(
                    "{}: object={}, Invalid input end month/day ({}/{})",
                    owners.shortcut.current_module_object,
                    row.title,
                    row.start_month,
                    row.start_day
                ))?;
                *errors = true;
            } else if row.end_year == 0 {
                row.end_year = row.start_year;
                row.end_julian_date = source_julian_date(row.end_year, row.end_month, row.end_day)?;
                if row.start_julian_date > row.end_julian_date {
                    row.end_julian_date = 0;
                    row.end_year = checked(row.end_year.checked_add(1))?;
                }
            } else {
                row.end_julian_date = source_julian_date(row.end_year, row.end_month, row.end_day)?;
                if row.start_julian_date > row.end_julian_date {
                    owners.backend.show_severe_error(&format!(
                        "{}: object={}, start Julian date ({}) is after the end Julian date ({}).",
                        owners.shortcut.current_module_object,
                        row.title,
                        row.start_julian_date,
                        row.end_julian_date
                    ))?;
                    *errors = true;
                }
            }
            if row.end_julian_date == 0 {
                row.end_julian_date = source_julian_date(row.end_year, row.end_month, row.end_day)?;
            }
            row.number_simulation_years = checked(
                row.end_year
                    .checked_sub(row.start_year)
                    .and_then(|v| v.checked_add(1)),
            )?;
            if let Some(v) =
                selected_bool(owners.shortcut, 3, &new_name, true, owners.backend, errors)?
            {
                row.use_holidays = v;
            }
            if let Some(v) =
                selected_bool(owners.shortcut, 4, &new_name, true, owners.backend, errors)?
            {
                row.use_dst = v;
            }
            if let Some(v) =
                selected_bool(owners.shortcut, 5, &new_name, true, owners.backend, errors)?
            {
                row.apply_weekend_rule = v;
            }
            if let Some(v) =
                selected_bool(owners.shortcut, 6, &new_name, true, owners.backend, errors)?
            {
                row.use_rain = v;
            }
            if let Some(v) =
                selected_bool(owners.shortcut, 7, &new_name, true, owners.backend, errors)?
            {
                row.use_snow = v;
            }
            if let Some(v) =
                selected_bool(owners.shortcut, 8, &new_name, false, owners.backend, errors)?
            {
                row.actual_weather = v;
            }
            // Native's Hour24 operand is alpha8, not alpha9; preserve the literal bug.
            if blank(owners.shortcut, 9)?
                || alpha(owners.shortcut, 8)?.eq_ignore_ascii_case("Hour24")
            {
                row.first_hour_interpolation_uses_hour_one = false;
            } else if alpha(owners.shortcut, 9)?.eq_ignore_ascii_case("Hour1") {
                row.first_hour_interpolation_uses_hour_one = true;
            } else {
                row.first_hour_interpolation_uses_hour_one = false;
            }
            row.day_of_week = row.start_weekday;
            row.is_leap_year = leap_year(row.start_year);
            row.month_week_day = [0; 12];
            if row.day_of_week != 0 && !*errors {
                self.cursor.setup_source_weekdays(
                    row.start_month,
                    row.start_day,
                    row.day_of_week,
                    &mut row.month_week_day,
                )?;
            }
        }
        if n_run_periods == 0 && owners.system.full_annual_run {
            owners.backend.show_warning_error("No Run Periods input but Full Annual Simulation selected.  Adding Run Period to 1/1 through 12/31.")?;
            self.state.weather.num_of_envrn =
                checked(self.state.weather.num_of_envrn.checked_add(1))?;
            let n = usize::try_from(self.state.weather.num_of_envrn)
                .map_err(|_| Error::Unavailable("actual Environment redimension5390"))?;
            let environments = self
                .state
                .weather
                .source_environments
                .as_mut()
                .ok_or(Error::Unavailable("actual Environment array5390"))?;
            environments.resize(n, SourceEnvironmentData::default());
            let index = n
                .checked_sub(1)
                .ok_or(Error::Unavailable("actual Environment ordinal5391"))?;
            environments
                .get_mut(index)
                .ok_or(Error::Unavailable("actual Environment row5391"))?
                .kind_of_environment = 3;
            n_run_periods = 1;
            self.state.global.weather_simulation_requested = true;
            self.state.weather.source_run_period_input =
                Some(vec![SourceRunPeriodData::default(); n_run_periods as usize]);
            let row = self
                .state
                .weather
                .source_run_period_input
                .as_mut()
                .and_then(|v| v.first_mut())
                .ok_or(Error::Unavailable("actual implicit RunPeriodInput row5395"))?;
            row.start_julian_date = source_ordinal_day(
                row.start_month,
                row.start_day,
                self.cursor.header().leap_year_add,
            )?;
            row.end_julian_date = source_ordinal_day(
                row.end_month,
                row.end_day,
                self.cursor.header().leap_year_add,
            )?;
            row.month_week_day = [0; 12];
            if row.day_of_week != 0 && !*errors {
                self.cursor.setup_source_weekdays(
                    row.start_month,
                    row.start_day,
                    row.day_of_week,
                    &mut row.month_week_day,
                )?;
            }
        } else {
            n_run_periods = 1;
        }
        let _ = (count, n_run_periods); // Actual Native local stores, not external counts.
        Ok(())
    }
}
