//! Native ScheduleManager3743-3858 annual and weekly methods on the same owner.
use super::{SelectedScheduleData, SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

impl SelectedScheduleManager {
    pub(crate) fn source_average_weekly_hours_full_load(
        &self,
        index: SourceScheduleIndex,
        start_day_of_week: i32,
        is_leap_year: bool,
        time_steps_in_hour: i32,
    ) -> Result<f64, Error> {
        let weeks_in_year = if is_leap_year {
            366.0 / 7.0
        } else {
            365.0 / 7.0
        };
        Ok(self.source_annual_hours_full_load(
            index,
            start_day_of_week,
            is_leap_year,
            time_steps_in_hour,
        )? / weeks_in_year)
    }

    // Preserve Original explicit signed-integer boundary predicates and branch order.
    #[allow(clippy::manual_range_contains)]
    pub(crate) fn source_annual_hours_full_load(
        &self,
        index: SourceScheduleIndex,
        start_day_of_week: i32,
        is_leap_year: bool,
        time_steps_in_hour: i32,
    ) -> Result<f64, Error> {
        let schedule = self.schedules.get(index.0).ok_or(Error::Unavailable(
            "actual selected Schedule getAnnualHoursFullLoad pointer",
        ))?;
        match &schedule.data {
            SelectedScheduleData::Constant { .. } => {
                if start_day_of_week < 1 || start_day_of_week > 7 {
                    return Ok(0.0);
                }
                let days_in_year: i32 = if is_leap_year { 366 } else { 365 };
                Ok(f64::from(days_in_year * 24) * schedule.current_value)
            }
            SelectedScheduleData::Detailed(detailed) => {
                let days_in_year: i32 = if is_leap_year { 366 } else { 365 };
                let mut day_type = start_day_of_week;
                let mut total_hours = 0.0;
                if day_type < 1 || day_type > 7 {
                    return Ok(total_hours);
                }
                for day in 1..=days_in_year {
                    let week_index = detailed.week_schedules[day as usize].ok_or(
                        Error::Unavailable("actual Detailed annual weekScheds[iDay] pointer"),
                    )?;
                    let week = self
                        .week_schedules
                        .get(week_index)
                        .ok_or(Error::Unavailable(
                            "actual Detailed annual WeekSchedule pointer",
                        ))?;
                    let day_index = week.day_schedules[day_type as usize].ok_or(
                        Error::Unavailable("actual Detailed annual dayScheds[DayT] pointer"),
                    )?;
                    let day_schedule = self.day_schedules.get(day_index).ok_or(
                        Error::Unavailable("actual Detailed annual DaySchedule pointer"),
                    )?;
                    total_hours += day_schedule.sum_timestep_values / f64::from(time_steps_in_hour);
                    day_type += 1;
                    if day_type > 7 {
                        day_type = 1;
                    }
                }
                Ok(total_hours)
            }
        }
    }

    // Preserve Original explicit signed-integer boundary predicates and branch order.
    #[allow(clippy::manual_range_contains)]
    pub(crate) fn source_annual_hours_greater_than_1_percent(
        &self,
        index: SourceScheduleIndex,
        start_day_of_week: i32,
        is_leap_year: bool,
        time_steps_in_hour: i32,
        time_step_zone: f64,
    ) -> Result<f64, Error> {
        let days_in_year: i32 = if is_leap_year { 366 } else { 365 };
        let schedule = self.schedules.get(index.0).ok_or(Error::Unavailable(
            "actual selected Schedule getAnnualHoursGreaterThan1Percent pointer",
        ))?;
        match &schedule.data {
            SelectedScheduleData::Constant { .. } => {
                if start_day_of_week < 1 || start_day_of_week > 7 {
                    return Ok(0.0);
                }
                Ok(if schedule.current_value > 0.0 {
                    24.0 * f64::from(days_in_year)
                } else {
                    0.0
                })
            }
            SelectedScheduleData::Detailed(detailed) => {
                let mut day_type = start_day_of_week;
                let mut total_hours = 0.0;
                if day_type < 1 || day_type > 7 {
                    return Ok(total_hours);
                }
                for day in 1..=days_in_year {
                    let week_index =
                        detailed.week_schedules[day as usize].ok_or(Error::Unavailable(
                            "actual Detailed nonzero annual weekScheds[iDay] pointer",
                        ))?;
                    let week = self
                        .week_schedules
                        .get(week_index)
                        .ok_or(Error::Unavailable(
                            "actual Detailed nonzero annual WeekSchedule pointer",
                        ))?;
                    let day_index =
                        week.day_schedules[day_type as usize].ok_or(Error::Unavailable(
                            "actual Detailed nonzero annual dayScheds[DayT] pointer",
                        ))?;
                    let day_schedule = self.day_schedules.get(day_index).ok_or(
                        Error::Unavailable("actual Detailed nonzero annual DaySchedule pointer"),
                    )?;
                    let slots =
                        24_i32
                            .checked_mul(time_steps_in_hour)
                            .ok_or(Error::Unavailable(
                                "actual Detailed annual signed timestep loop domain",
                            ))?;
                    for timestep in 0..slots {
                        let value = day_schedule.timestep_values.get(timestep as usize).ok_or(
                            Error::Unavailable("actual Detailed annual tsVals[i] operand"),
                        )?;
                        // The Native method name says 1%, but its literal
                        // predicate is >0.0 and adds the actual TimeStepZone.
                        if *value > 0.0 {
                            total_hours += time_step_zone;
                        }
                    }
                    day_type += 1;
                    if day_type > 7 {
                        day_type = 1;
                    }
                }
                Ok(total_hours)
            }
        }
    }
}
