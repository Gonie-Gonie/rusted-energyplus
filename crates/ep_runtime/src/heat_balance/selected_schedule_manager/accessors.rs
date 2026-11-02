//! Genuine native Schedule pointer use propagation and selected min/max.
use super::{
    SelectedScheduleData, SelectedScheduleManager, SourceDayScheduleIndex, SourceScheduleIndex,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as E;
use crate::weather::day::WeatherGlobalState;
impl SelectedScheduleManager {
    /// Native GetDaySchedule2640–2662 consumes the already processed real map.
    pub(crate) fn get_source_day_schedule(
        &mut self,
        name: &str,
    ) -> Result<Option<SourceDayScheduleIndex>, E> {
        let Some(&number) = self.day_schedule_map.get(name) else {
            return Ok(None);
        };
        let day = self.day_schedules.get_mut(number).ok_or(E::Unavailable(
            "actual GetDaySchedule map/index association",
        ))?;
        day.is_used = true;
        Ok(Some(SourceDayScheduleIndex(number)))
    }

    /// Native DaySchedule::getDayVals returns the sole retained tsVals owner.
    pub(crate) fn source_day_values(&self, index: SourceDayScheduleIndex) -> Result<&[f64], E> {
        self.day_schedules
            .get(index.0)
            .map(|day| day.timestep_values.as_slice())
            .ok_or(E::Unavailable("actual DaySchedule getDayVals pointer"))
    }

    /// Native ScheduleBase3376 inclusive bound, using its actual cached reducer.
    pub(crate) fn source_day_check_min(
        &mut self,
        index: SourceDayScheduleIndex,
        minimum: f64,
        clock: &WeatherGlobalState,
    ) -> Result<bool, E> {
        let (actual_minimum, _) = self.source_day_min_max_with_clock(index.0, clock)?;
        Ok(f64::from(f32::EPSILON) >= minimum - actual_minimum)
    }

    /// Native ScheduleBase3410–3413 evaluates both inclusive checks in order.
    pub(crate) fn source_day_check_min_max(
        &mut self,
        index: SourceDayScheduleIndex,
        minimum: f64,
        maximum: f64,
        clock: &WeatherGlobalState,
    ) -> Result<bool, E> {
        let (actual_minimum, actual_maximum) =
            self.source_day_min_max_with_clock(index.0, clock)?;
        let min_ok = f64::from(f32::EPSILON) >= minimum - actual_minimum;
        let max_ok = actual_maximum - maximum <= f64::from(f32::EPSILON);
        Ok(min_ok && max_ok)
    }
    pub(crate) fn get_source_schedule(
        &mut self,
        name: &str,
    ) -> Result<Option<SourceScheduleIndex>, E> {
        let Some(&index) = self.schedule_map.get(name) else {
            return Ok(None);
        };
        let row = self
            .schedules
            .get_mut(index.0)
            .ok_or(E::Unavailable("actual Schedule map/index association"))?;
        if !row.is_used {
            row.is_used = true;
            if matches!(row.data, SelectedScheduleData::Detailed(_)) {
                // Native2560-2603 propagates used through each actual pointer.
                for day in 1..=366 {
                    let SelectedScheduleData::Detailed(detail) = &self.schedules[index.0].data
                    else {
                        return Err(E::Unavailable("actual GetSchedule Detailed dynamic type"));
                    };
                    let Some(week) = detail.week_schedules[day] else {
                        continue;
                    };
                    let week_row = self
                        .week_schedules
                        .get_mut(week)
                        .ok_or(E::Unavailable("actual GetSchedule week pointer"))?;
                    if week_row.is_used {
                        continue;
                    }
                    week_row.is_used = true;
                    for day_type in 1..13 {
                        let day = self.week_schedules[week].day_schedules[day_type]
                            .ok_or(E::Unavailable("actual GetSchedule null day dereference"))?;
                        self.day_schedules
                            .get_mut(day)
                            .ok_or(E::Unavailable("actual GetSchedule day pointer"))?
                            .is_used = true;
                    }
                }
            }
        }
        Ok(Some(index))
    }
    fn source_selected_min_max(
        &mut self,
        index: SourceScheduleIndex,
        clock: Option<&WeatherGlobalState>,
    ) -> Result<(f64, f64), E> {
        if let Some(clock) = clock {
            self.ensure_source_min_max_with_clock(index, clock)?;
        } else {
            self.ensure_source_min_max(index)?;
        }
        let row = self
            .schedules
            .get(index.0)
            .ok_or(E::Unavailable("actual selected Schedule pointer"))?;
        Ok((row.min_value, row.max_value))
    }
    pub(crate) fn source_min(&mut self, index: SourceScheduleIndex) -> Result<f64, E> {
        Ok(self.source_selected_min_max(index, None)?.0)
    }
    pub(crate) fn source_max(&mut self, index: SourceScheduleIndex) -> Result<f64, E> {
        Ok(self.source_selected_min_max(index, None)?.1)
    }
    pub(crate) fn source_check_min(
        &mut self,
        index: SourceScheduleIndex,
        minimum: f64,
    ) -> Result<bool, E> {
        Ok(f64::from(f32::EPSILON) >= minimum - self.source_min(index)?)
    }

    pub(crate) fn source_min_with_clock(
        &mut self,
        index: SourceScheduleIndex,
        clock: &WeatherGlobalState,
    ) -> Result<f64, E> {
        Ok(self.source_selected_min_max(index, Some(clock))?.0)
    }
    pub(crate) fn source_max_with_clock(
        &mut self,
        index: SourceScheduleIndex,
        clock: &WeatherGlobalState,
    ) -> Result<f64, E> {
        Ok(self.source_selected_min_max(index, Some(clock))?.1)
    }
    pub(crate) fn source_check_min_with_clock(
        &mut self,
        index: SourceScheduleIndex,
        minimum: f64,
        clock: &WeatherGlobalState,
    ) -> Result<bool, E> {
        Ok(f64::from(f32::EPSILON) >= minimum - self.source_min_with_clock(index, clock)?)
    }

    /// Actual ScheduleBase::checkMinMaxVals for the caller's two inclusive bounds.
    pub(crate) fn source_check_min_max_inclusive(
        &mut self,
        index: SourceScheduleIndex,
        minimum: f64,
        maximum: f64,
        clock: &WeatherGlobalState,
    ) -> Result<bool, E> {
        let (actual_minimum, actual_maximum) = self.source_selected_min_max(index, Some(clock))?;
        let min_ok = f64::from(f32::EPSILON) >= minimum - actual_minimum;
        let max_ok = actual_maximum - maximum <= f64::from(f32::EPSILON);
        Ok(min_ok && max_ok)
    }
    /// Original Constant::hasVal3428 and Detailed::hasVal3433-3470.
    /// The caller supplies the actual DataGlobal timestep owner; no min/max
    /// cache, typed schedule expansion or currentVal substitutes this traversal.
    pub(crate) fn source_has_value(
        &self,
        index: SourceScheduleIndex,
        value: f64,
        clock: &WeatherGlobalState,
    ) -> Result<bool, E> {
        let row = self
            .schedules
            .get(index.0)
            .ok_or(E::Unavailable("actual hasVal Schedule pointer"))?;
        match &row.data {
            SelectedScheduleData::Constant { .. } => Ok(value == row.current_value),
            SelectedScheduleData::Detailed(detailed) => {
                let mut week_checked = vec![false; self.week_schedules.len()];
                let mut day_checked = vec![false; self.day_schedules.len()];
                // The fixed retained array has its genuine unused slot0 and
                // all366 original week pointers. Preserve original visit order.
                for &week_index in detailed.week_schedules.iter().skip(1) {
                    let week_index =
                        week_index.ok_or(E::Unavailable("actual hasVal null week pointer"))?;
                    let week = self
                        .week_schedules
                        .get(week_index)
                        .ok_or(E::Unavailable("actual hasVal WeekSchedule pointer"))?;
                    if *week_checked
                        .get(week.number)
                        .ok_or(E::Unavailable("actual hasVal weekSchedChecked Num"))?
                    {
                        continue;
                    }
                    for &day_index in week.day_schedules.iter().skip(1) {
                        let day_index =
                            day_index.ok_or(E::Unavailable("actual hasVal null day pointer"))?;
                        let day = self
                            .day_schedules
                            .get(day_index)
                            .ok_or(E::Unavailable("actual hasVal DaySchedule pointer"))?;
                        if *day_checked
                            .get(day.number)
                            .ok_or(E::Unavailable("actual hasVal daySchedChecked Num"))?
                        {
                            continue;
                        }
                        // Original reads TimeStepsInHour only for an unvisited
                        // Detailed day. Constant and visited branches do not.
                        let slots = 24_i32
                            .checked_mul(clock.time_steps_in_hour)
                            .ok_or(E::Unavailable("actual hasVal signed timestep loop extent"))?;
                        for at in 0..slots {
                            let at = usize::try_from(at)
                                .map_err(|_| E::Unavailable("actual hasVal timestep index"))?;
                            if *day
                                .timestep_values
                                .get(at)
                                .ok_or(E::Unavailable("actual hasVal day tsVals index"))?
                                == value
                            {
                                return Ok(true);
                            }
                        }
                        day_checked[day.number] = true;
                    }
                    week_checked[week.number] = true;
                }
                Ok(false)
            }
        }
    }
}
