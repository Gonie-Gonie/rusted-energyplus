//! Literal Schedule3790-3806 and selected Constant annual3740-3753 calculations.
use super::{SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
impl SelectedScheduleManager {
    pub(crate) fn source_average_weekly_hours_full_load(
        &self,
        index: SourceScheduleIndex,
        start_day_of_week: i32,
        is_leap_year: bool,
    ) -> Result<f64, Error> {
        let weeks_in_year = if is_leap_year {
            366.0 / 7.0
        } else {
            365.0 / 7.0
        };
        let schedule = self.schedules.get(index.0).ok_or(Error::Unavailable(
            "actual selected Constant getAnnualHoursFullLoad pointer3741",
        ))?;
        let annual = if start_day_of_week < 1 || start_day_of_week > 7 {
            0.0
        } else {
            let days_in_year = if is_leap_year { 366 } else { 365 };
            f64::from(days_in_year * 24) * schedule.current_value
        };
        Ok(annual / weeks_in_year)
    }
}

impl SelectedScheduleManager {
    pub(crate) fn source_annual_hours_full_load(
        &self,
        index: SourceScheduleIndex,
        start_day_of_week: i32,
        is_leap_year: bool,
    ) -> Result<f64, Error> {
        let schedule = self.schedules.get(index.0).ok_or(Error::Unavailable(
            "actual selected Constant getAnnualHoursFullLoad pointer",
        ))?;
        if start_day_of_week < 1 || start_day_of_week > 7 {
            return Ok(0.0);
        }
        let days_in_year = if is_leap_year { 366 } else { 365 };
        Ok(f64::from(days_in_year * 24) * schedule.current_value)
    }
    pub(crate) fn source_annual_hours_greater_than_1_percent(
        &self,
        index: SourceScheduleIndex,
        start_day_of_week: i32,
        is_leap_year: bool,
    ) -> Result<f64, Error> {
        let days_in_year = if is_leap_year { 366 } else { 365 };
        let schedule = self.schedules.get(index.0).ok_or(Error::Unavailable(
            "actual selected Constant getAnnualHoursGreaterThan1Percent pointer",
        ))?;
        if start_day_of_week < 1 || start_day_of_week > 7 {
            return Ok(0.0);
        }
        Ok(if schedule.current_value > 0.0 {
            24.0 * f64::from(days_in_year)
        } else {
            0.0
        })
    }
}
