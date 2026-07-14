use super::{
    DEFAULT_RUN_PERIOD_YEAR, Date, ResolvedRunPeriodCalendar, ResolvedWeatherEnvironmentCalendar,
    TimeAxisError, day_of_year, invalid_date_error, next_day, resolve_run_period_calendar,
};
use crate::weather::EpwCalendarMetadata;
use ep_model::RunPeriod;

/// Applies EPW leap-year policy to a Gregorian run-period calendar.
///
/// This resolves calendar-axis shape only. Actual-weather and cross-year EPW
/// record traversal remain explicit errors until DATA PERIOD selection is ported.
pub fn resolve_weather_environment_calendar(
    run_period: &RunPeriod,
    metadata: &EpwCalendarMetadata,
) -> Result<ResolvedWeatherEnvironmentCalendar, TimeAxisError> {
    let gregorian = resolve_run_period_calendar(run_period)?;
    if run_period.treat_weather_as_actual {
        return Err(TimeAxisError::ActualWeatherUnsupported {
            run_period_name: run_period.name.0.clone(),
        });
    }
    if gregorian.start_year != gregorian.end_year {
        return Err(TimeAxisError::WeatherMetadataCrossYearUnsupported {
            run_period_name: run_period.name.0.clone(),
            start_year: gregorian.start_year,
            end_year: gregorian.end_year,
        });
    }

    let leap_days_skipped = if metadata.leap_year_observed {
        0
    } else {
        count_february_29_days(&gregorian)
    };
    let total_days = if metadata.leap_year_observed {
        gregorian.total_days
    } else {
        let start = weather_effective_ordinal_day(
            run_period,
            "begin",
            gregorian.start_year,
            gregorian.start_month,
            gregorian.start_day_of_month,
        )?;
        let end = weather_effective_ordinal_day(
            run_period,
            "end",
            gregorian.end_year,
            gregorian.end_month,
            gregorian.end_day_of_month,
        )?;
        let duration = end
            .checked_sub(start)
            .and_then(|days| days.checked_add(1))
            .ok_or_else(|| invalid_range_error(run_period))?;
        usize::try_from(duration).map_err(|_| invalid_range_error(run_period))?
    };

    Ok(ResolvedWeatherEnvironmentCalendar {
        start_year_is_weather_effective_leap_year: gregorian.start_year_is_leap_year
            && metadata.leap_year_observed,
        end_year_is_weather_effective_leap_year: gregorian.end_year_is_leap_year
            && metadata.leap_year_observed,
        weather_file_allows_leap_years: metadata.leap_year_observed,
        total_days,
        leap_days_skipped,
        gregorian,
    })
}

fn count_february_29_days(calendar: &ResolvedRunPeriodCalendar) -> usize {
    let mut count = 0;
    let mut date = calendar.start_date();
    for day_index in 0..calendar.total_days {
        if date.month == 2 && date.day_of_month == 29 {
            count += 1;
        }
        if day_index + 1 < calendar.total_days {
            date = next_day(date);
        }
    }
    count
}

fn weather_effective_ordinal_day(
    run_period: &RunPeriod,
    field: &'static str,
    year: u32,
    month: u32,
    day_of_month: u32,
) -> Result<u32, TimeAxisError> {
    if month == 2 && day_of_month == 29 {
        Ok(60)
    } else {
        day_of_year(DEFAULT_RUN_PERIOD_YEAR, month, day_of_month).ok_or_else(|| {
            invalid_date_error(
                run_period,
                field,
                Date {
                    year,
                    month,
                    day_of_month,
                },
            )
        })
    }
}

fn invalid_range_error(run_period: &RunPeriod) -> TimeAxisError {
    TimeAxisError::InvalidRange {
        run_period_name: run_period.name.0.clone(),
    }
}
