//! Genuine ProcessEPWHeader Typical/Extreme Periods7584-7824 on the sole raw header.
mod dates;
use super::dates::number_with_error;
use super::header_state::{source_count, strip_ascii_spaces, to_source_int};
use super::{DateType, RawDayFailure, RawEpwHeaderError, RawEpwHeaderState, RawEpwInput};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::weather::day::SourceWeatherInitializationError as Error;
pub(crate) use dates::{
    SourceDateScratch, process_source_date_string, process_source_date_string_with_diagnostic,
    source_ordinal_day,
};
fn raw(e: RawEpwHeaderError) -> Error {
    RawDayFailure::Header(e).into()
}
fn unavailable(reason: &'static str) -> Error {
    raw(RawEpwHeaderError::OutsideBoundedDomain(reason))
}
fn erase(line: &mut String, position: Option<usize>) {
    // Native npos+1 wraps to0, so a missing comma does not erase any bytes.
    if let Some(at) = position {
        line.drain(..(at + 1).min(line.len()));
    }
}
fn prefix(text: &str, needle: &str) -> bool {
    text.get(..needle.len())
        .is_some_and(|part| part.eq_ignore_ascii_case(needle))
}
fn generic_short_title(title: &str) -> Option<&'static str> {
    [
        ("SUMMER", "Summer"),
        ("WINTER", "Winter"),
        ("TROPICAL HOT", "TropicalHot"),
        ("TROPICAL COLD", "TropicalCold"),
        ("AUTUMN", "Autumn"),
        ("NO DRY", "NoDrySeason"),
        ("NO WET", "NoWetSeason"),
        ("WET ", "WetSeason"),
        ("DRY ", "DrySeason"),
        ("SPRING", "Spring"),
    ]
    .into_iter()
    .find_map(|(p, v)| prefix(title, p).then_some(v))
}
pub(crate) fn process_source_typical_extreme_periods(
    line: &mut String,
    errors: &mut bool,
    state: &mut RawEpwHeaderState,
    input: &mut RawEpwInput,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    strip_ascii_spaces(line);
    let mut position = line.find(',');
    if position.is_none() {
        if line.is_empty() {
            while position.is_none() && line.is_empty() {
                let read = input.read_line().map_err(raw)?;
                *line = read.data;
                strip_ascii_spaces(line);
                position = line.find(',');
                if read.eof && line.is_empty() {
                    return Err(unavailable(
                        "actual Typical continuation has no terminating input7590",
                    ));
                }
            }
        } else {
            position = Some(line.len());
        }
    }
    // Native IOStatus is deliberately ignored at this caller.
    state.number_typical_extreme_sets =
        to_source_int(number_with_error(&line[..position.unwrap_or(line.len())]).0).map_err(raw)?;
    erase(line, position);
    state
        .typical_extreme_periods
        .allocate(source_count(state.number_typical_extreme_sets).map_err(raw)?);
    let mut tropical_extreme_count = 0;
    let mut number = 1;
    while number <= state.number_typical_extreme_sets {
        let at = usize::try_from(number - 1)
            .map_err(|_| unavailable("actual Typical native row ordinal"))?;
        strip_ascii_spaces(line);
        position = line.find(',');
        if let Some(end) = position {
            state.typical_extreme_periods.values[at].title = line[..end].to_owned();
            erase(line, position);
        } else {
            backend.show_warning_error(&format!(
                "ProcessEPWHeader: Invalid Typical/Extreme Periods Header(WeatherFile)={line}"
            ))?;
            backend.show_continue_error(&format!(
                "...on processing Typical/Extreme period #{number}"
            ))?;
            state.number_typical_extreme_sets = number - 1;
            break;
        }
        position = line.find(',');
        if let Some(end) = position {
            state.typical_extreme_periods.values[at].te_type = line[..end].to_owned();
            erase(line, position);
            let row = &mut state.typical_extreme_periods.values[at];
            if row.te_type.eq_ignore_ascii_case("EXTREME") {
                if prefix(&row.title, "NO DRY SEASON - WEEK NEAR ANNUAL MAX") {
                    row.short_title = "NoDrySeasonMax".into();
                } else if prefix(&row.title, "NO DRY SEASON - WEEK NEAR ANNUAL MIN") {
                    row.short_title = "NoDrySeasonMin".into();
                } else if prefix(&row.title, "NO WET SEASON - WEEK NEAR ANNUAL MAX") {
                    row.short_title = "NoWetSeasonMax".into();
                } else if prefix(&row.title, "NO WET SEASON - WEEK NEAR ANNUAL MIN") {
                    row.short_title = "NoWetSeasonMin".into();
                } else if prefix(&row.title, "NO DRY") {
                    if tropical_extreme_count == 0 {
                        row.title = "No Dry Season - Week Near Annual Max".into();
                        row.short_title = "NoDrySeasonMax".into();
                        tropical_extreme_count += 1;
                    } else if tropical_extreme_count == 1 {
                        row.title = "No Dry Season - Week Near Annual Min".into();
                        row.short_title = "NoDrySeasonMin".into();
                        tropical_extreme_count += 1;
                    }
                } else if let Some(short) = generic_short_title(&row.title) {
                    row.short_title = short.into();
                }
            } else if let Some(short) = generic_short_title(&row.title) {
                row.short_title = short.into();
            }
        } else {
            backend.show_warning_error(&format!(
                "ProcessEPWHeader: Invalid Typical/Extreme Periods Header(WeatherFile)={} {line}",
                state.typical_extreme_periods.values[at].title
            ))?;
            backend.show_continue_error(&format!(
                "...on processing Typical/Extreme period #{number}"
            ))?;
            state.number_typical_extreme_sets = number - 1;
            break;
        }
        let mut date = SourceDateScratch::default();
        position = line.find(',');
        if let Some(end) = position {
            let upper = line[..end].to_ascii_uppercase();
            process_source_date_string(&upper, &mut date, false, errors, backend)?;
            if date.date_type != DateType::Invalid {
                let month = date
                    .month
                    .ok_or_else(|| unavailable("actual Typical PMonth unwritten7704"))?;
                let day = date
                    .day
                    .ok_or_else(|| unavailable("actual Typical PDay unwritten7704"))?;
                if month != 0 && day != 0 {
                    state.typical_extreme_periods.values[at].start_month = month;
                    state.typical_extreme_periods.values[at].start_day = day;
                }
            } else {
                backend.show_severe_error(&format!("ProcessEPWHeader: Invalid Typical/Extreme Periods Start Date Field(WeatherFile)={}",&line[..end]))?;
                backend.show_continue_error(&format!(
                    "...on processing Typical/Extreme period #{number}"
                ))?;
                *errors = true;
            }
            erase(line, position);
        }
        position = line.find(',');
        if let Some(end) = position {
            let upper = line[..end].to_ascii_uppercase();
            process_source_date_string(&upper, &mut date, false, errors, backend)?;
            if date.date_type != DateType::Invalid {
                let month = date
                    .month
                    .ok_or_else(|| unavailable("actual Typical PMonth unwritten7723"))?;
                let day = date
                    .day
                    .ok_or_else(|| unavailable("actual Typical PDay unwritten7723"))?;
                if month != 0 && day != 0 {
                    state.typical_extreme_periods.values[at].end_month = month;
                    state.typical_extreme_periods.values[at].end_day = day;
                }
            } else {
                backend.show_severe_error(&format!("ProcessEPWHeader: Invalid Typical/Extreme Periods End Date Field(WeatherFile)={}",&line[..end]))?;
                backend.show_continue_error(&format!(
                    "...on processing Typical/Extreme period #{number}"
                ))?;
                *errors = true;
            }
            erase(line, position);
        } else {
            let upper = line.to_ascii_uppercase();
            process_source_date_string(&upper, &mut date, false, errors, backend)?;
            if date.date_type != DateType::Invalid {
                let month = date
                    .month
                    .ok_or_else(|| unavailable("actual Typical PMonth unwritten7739"))?;
                let day = date
                    .day
                    .ok_or_else(|| unavailable("actual Typical PDay unwritten7739"))?;
                if month != 0 && day != 0 {
                    state.typical_extreme_periods.values[at].end_month = month;
                    state.typical_extreme_periods.values[at].end_day = day;
                }
            } else {
                backend.show_severe_error(&format!("ProcessEPWHeader: Invalid Typical/Extreme Periods End Date Field(WeatherFile)={line}"))?;
                *errors = true;
            }
        }
        number = number
            .checked_add(1)
            .ok_or_else(|| unavailable("actual Typical first-loop signed counter domain"))?;
    }
    let mut number = 1;
    while number <= state.number_typical_extreme_sets {
        let at = usize::try_from(number - 1)
            .map_err(|_| unavailable("actual Typical second-loop row ordinal"))?;
        let row = &mut state.typical_extreme_periods.values[at];
        let title = row.short_title.to_ascii_uppercase();
        if title == "SUMMER" {
            if row.te_type.eq_ignore_ascii_case("EXTREME") {
                row.match_value = "SummerExtreme".into();
                row.match_value1 = "TropicalHot".into();
                row.match_value2 = "NoDrySeasonMax".into();
            } else {
                row.match_value = "SummerTypical".into();
            }
        } else if title == "WINTER" {
            if row.te_type.eq_ignore_ascii_case("EXTREME") {
                row.match_value = "WinterExtreme".into();
                row.match_value1 = "TropicalCold".into();
                row.match_value2 = "NoDrySeasonMin".into();
            } else {
                row.match_value = "WinterTypical".into();
            }
        } else if title == "AUTUMN" {
            row.match_value = "AutumnTypical".into();
        } else if title == "SPRING" {
            row.match_value = "SpringTypical".into();
        } else if title == "WETSEASON" {
            row.match_value = "WetSeason".into();
        } else if title == "DRYSEASON" {
            row.match_value = "DrySeason".into();
        } else if title == "NOWETSEASON" {
            row.match_value = "NoWetSeason".into();
        } else if title == "NODRYSEASON" {
            row.match_value = "NoDrySeason".into();
        } else if title == "NODRYSEASONMAX" || title == "NOWETSEASONMAX" {
            row.match_value = row.short_title.clone();
            row.match_value1 = "TropicalHot".into();
            row.match_value2 = "SummerExtreme".into();
        } else if title == "NODRYSEASONMIN" || title == "NOWETSEASONMIN" {
            row.match_value = row.short_title.clone();
            row.match_value1 = "TropicalCold".into();
            row.match_value2 = "WinterExtreme".into();
        } else if title == "TROPICALHOT" {
            row.match_value = "TropicalHot".into();
            row.match_value1 = "SummerExtreme".into();
            row.match_value2 = "NoDrySeasonMax".into();
        } else if title == "TROPICALCOLD" {
            row.match_value = "TropicalCold".into();
            row.match_value1 = "WinterExtreme".into();
            row.match_value2 = "NoDrySeasonMin".into();
        } else {
            row.match_value = "Invalid - no match".into();
        }
        row.start_julian_day = source_ordinal_day(row.start_month, row.start_day, 0)?;
        row.end_julian_day = source_ordinal_day(row.end_month, row.end_day, 0)?;
        if row.start_julian_day <= row.end_julian_day {
            row.total_days = row
                .end_julian_day
                .checked_sub(row.start_julian_day)
                .and_then(|v| v.checked_add(1))
                .ok_or_else(|| unavailable("actual Typical TotalDays7818 int domain"))?;
        } else {
            row.total_days = source_ordinal_day(12, 31, state.leap_year_add)?
                .checked_sub(row.start_julian_day)
                .and_then(|v| v.checked_add(1))
                .and_then(|v| v.checked_add(row.end_julian_day))
                .ok_or_else(|| unavailable("actual Typical TotalDays7821 int domain"))?;
        }
        number = number
            .checked_add(1)
            .ok_or_else(|| unavailable("actual Typical second-loop signed counter domain"))?;
    }
    Ok(())
}
