//! ScheduleManager3862-3935 on the same retained schedule/day/week owners.
use super::{SelectedScheduleData, SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState, WeatherOwnerState};

impl SelectedScheduleManager {
    /// Genuine Constant/Detailed getValAndCountOnDay; no cached report values.
    pub(crate) fn source_value_and_count_on_day(
        &self,
        index: SourceScheduleIndex,
        is_summer: bool,
        day_of_week: i32,
        hour_of_day: i32,
        environment: &WeatherEnvironmentState,
        weather: &WeatherOwnerState,
        clock: &WeatherGlobalState,
    ) -> Result<(f64, i32, String), Error> {
        let schedule = self.schedules.get(index.0).ok_or(Error::Unavailable(
            "actual getValAndCountOnDay Schedule pointer",
        ))?;
        let month = if is_summer {
            if environment.latitude > 0.0 { 7 } else { 1 }
        } else if environment.latitude > 0.0 {
            1
        } else {
            7
        };
        let month_name = if month == 1 { "January" } else { "July" }.to_owned();
        match &schedule.data {
            SelectedScheduleData::Constant { .. } => {
                let days_in_year = if environment.current_year_is_leap_year {
                    366
                } else {
                    365
                };
                Ok((schedule.current_value, days_in_year, month_name))
            }
            SelectedScheduleData::Detailed(detailed) => {
                // Literal General::nthDayOfWeekOfMonth889-903, including its
                // native modulo expression; no weekday normalization is added.
                let first_day = crate::weather::raw::source_ordinal_day(
                    month,
                    1,
                    i32::from(environment.current_year_is_leap_year),
                )
                .map_err(|_| Error::Unavailable("actual nthDayOfWeekOfMonth OrdinalDay"))?;
                let first_weekday = environment
                    .run_period_start_day_of_week
                    .checked_add(first_day)
                    .and_then(|v| v.checked_sub(1))
                    .ok_or(Error::Unavailable(
                        "actual nthDayOfWeekOfMonth signed arithmetic",
                    ))?
                    % 7;
                let offset = if day_of_week >= first_weekday {
                    day_of_week.checked_sub(first_weekday)
                } else {
                    day_of_week
                        .checked_add(7)
                        .and_then(|v| v.checked_sub(first_weekday))
                }
                .ok_or(Error::Unavailable("actual nthDayOfWeekOfMonth day offset"))?;
                // The actual caller requests nthTime=1, so its final 7*(1-1)
                // term is exactly integer zero, not a year/calendar projection.
                let selected_date = first_day.checked_add(offset).ok_or(Error::Unavailable(
                    "actual nthDayOfWeekOfMonth selected date",
                ))?;
                let days_in_year = if environment.current_year_is_leap_year {
                    366
                } else {
                    365
                };
                let dst_slot = selected_date
                    .checked_sub(1)
                    .and_then(|v| usize::try_from(v).ok())
                    .ok_or(Error::Unavailable(
                        "actual DSTIndex one-based selected date",
                    ))?;
                let hour_select = hour_of_day
                    .checked_add(
                        *weather
                            .dst_index
                            .get(dst_slot)
                            .ok_or(Error::Unavailable("actual getValAndCountOnDay DSTIndex"))?,
                    )
                    .ok_or(Error::Unavailable(
                        "actual getValAndCountOnDay hour+Solar DST",
                    ))?;
                let selected_week =
                    detailed
                        .week_schedules
                        .get(usize::try_from(selected_date).map_err(|_| {
                            Error::Unavailable("actual selected Julian schedule index")
                        })?)
                        .copied()
                        .flatten()
                        .ok_or(Error::Unavailable("actual selected weekScheds pointer"))?;
                let week = self
                    .week_schedules
                    .get(selected_week)
                    .ok_or(Error::Unavailable("actual selected WeekSchedule pointer"))?;
                let day_type = usize::try_from(day_of_week)
                    .map_err(|_| Error::Unavailable("actual selected day type index"))?;
                let selected_day = week
                    .day_schedules
                    .get(day_type)
                    .copied()
                    .flatten()
                    .ok_or(Error::Unavailable("actual selected dayScheds pointer"))?;
                let day = self
                    .day_schedules
                    .get(selected_day)
                    .ok_or(Error::Unavailable("actual selected DaySchedule pointer"))?;
                let timestep = hour_select
                    .checked_sub(1)
                    .and_then(|v| v.checked_mul(clock.time_steps_in_hour))
                    .and_then(|v| usize::try_from(v).ok())
                    .ok_or(Error::Unavailable(
                        "actual getValAndCountOnDay timestep expression",
                    ))?;
                let value = *day
                    .timestep_values
                    .get(timestep)
                    .ok_or(Error::Unavailable("actual selected tsVals firstTimeStep"))?;
                let mut count_of_same = 0;
                for julian in 1..=days_in_year {
                    let week_index = detailed.week_schedules[julian].ok_or(Error::Unavailable(
                        "actual getValAndCountOnDay annual weekScheds pointer",
                    ))?;
                    if week_index == selected_week {
                        count_of_same += 1;
                        continue;
                    }
                    let week = self
                        .week_schedules
                        .get(week_index)
                        .ok_or(Error::Unavailable(
                            "actual getValAndCountOnDay annual WeekSchedule pointer",
                        ))?;
                    let day_index = week.day_schedules.get(day_type).copied().flatten().ok_or(
                        Error::Unavailable("actual getValAndCountOnDay annual dayScheds pointer"),
                    )?;
                    if day_index == selected_day {
                        count_of_same += 1;
                        continue;
                    }
                    let day = self.day_schedules.get(day_index).ok_or(Error::Unavailable(
                        "actual getValAndCountOnDay annual DaySchedule pointer",
                    ))?;
                    if *day.timestep_values.get(timestep).ok_or(Error::Unavailable(
                        "actual getValAndCountOnDay annual tsVals",
                    ))? == value
                    {
                        count_of_same += 1;
                    }
                }
                Ok((value, count_of_same, month_name))
            }
        }
    }
}
