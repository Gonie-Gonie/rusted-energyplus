//! Native day/week/detailed min/max and actual getHrTsVal2490-2541.
use super::{SelectedScheduleData, SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as E;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
fn unavailable(text: &'static str) -> E { E::Unavailable(text) }
impl SelectedScheduleManager {
    pub(super) fn source_day_min_max(&mut self, number: usize) -> Result<(f64, f64), E> {
        let row = self.day_schedules.get(number).ok_or_else(|| unavailable("actual native DaySchedule pointer"))?;
        if !row.is_min_max_set { return Err(unavailable("actual uncached DaySchedule reduction needs live TimeStepsInHour")); }
        Ok((row.min_value, row.max_value))
    }
    fn set_source_day_min_max(&mut self, number: usize, steps: usize) -> Result<(), E> {
        let row = self.day_schedules.get_mut(number).ok_or_else(|| unavailable("actual native DaySchedule min/max pointer"))?;
        if row.is_min_max_set { return Ok(()); }
        let first = *row.timestep_values.first().ok_or_else(|| unavailable("actual DaySchedule tsVals0"))?;
        row.min_value = first; row.max_value = first;
        let slots = 24usize.checked_mul(steps).ok_or_else(|| unavailable("actual DaySchedule loop extent"))?;
        for index in 0..slots {
            let value = *row.timestep_values.get(index).ok_or_else(|| unavailable("actual DaySchedule tsVals index"))?;
            if value < row.min_value { row.min_value = value; }
            else if value > row.max_value { row.max_value = value; }
        }
        row.is_min_max_set = true;
        Ok(())
    }
    fn set_source_week_min_max(&mut self, number: usize, steps: usize) -> Result<(), E> {
        let week = self.week_schedules.get(number).ok_or_else(|| unavailable("actual WeekSchedule min/max pointer"))?;
        if week.is_min_max_set { return Ok(()); }
        let Some(first) = week.day_schedules[1] else { return Ok(()); };
        self.set_source_day_min_max(first, steps)?;
        let (minimum, maximum) = self.source_day_min_max(first)?;
        self.week_schedules[number].min_value = minimum;
        self.week_schedules[number].max_value = maximum;
        let mut previous = first;
        for day in 2..13 {
            let index = self.week_schedules[number].day_schedules[day]
                .ok_or_else(|| unavailable("actual WeekSchedule null day dereference"))?;
            if index == previous { continue; }
            self.set_source_day_min_max(index, steps)?;
            let (minimum, maximum) = self.source_day_min_max(index)?;
            if minimum < self.week_schedules[number].min_value { self.week_schedules[number].min_value = minimum; }
            if maximum > self.week_schedules[number].max_value { self.week_schedules[number].max_value = maximum; }
            previous = index;
        }
        self.week_schedules[number].is_min_max_set = true;
        Ok(())
    }
    /// Actual input validation supplies the same live global timestep operand.
    pub(super) fn ensure_source_min_max_with_steps(&mut self, index: SourceScheduleIndex, steps: usize) -> Result<(), E> {
        let row = self.schedules.get(index.0).ok_or_else(|| unavailable("actual Schedule min/max pointer"))?;
        if row.is_min_max_set { return Ok(()); }
        match &row.data {
            SelectedScheduleData::Constant { .. } => {
                let row = &mut self.schedules[index.0];
                row.min_value = row.current_value; row.max_value = row.current_value; row.is_min_max_set = true;
                return Ok(());
            }
            SelectedScheduleData::Detailed(_) => {}
        }
        let first = match &self.schedules[index.0].data {
            SelectedScheduleData::Detailed(detail) => detail.week_schedules[1],
            SelectedScheduleData::Constant { .. } => return Err(unavailable("actual Detailed dynamic type")),
        };
        let Some(first) = first else { return Ok(()); };
        self.set_source_week_min_max(first, steps)?;
        self.schedules[index.0].min_value = self.week_schedules[first].min_value;
        self.schedules[index.0].max_value = self.week_schedules[first].max_value;
        let mut previous = first;
        for day in 2..=366 {
            let week = match &self.schedules[index.0].data {
                SelectedScheduleData::Detailed(detail) => detail.week_schedules[day],
                SelectedScheduleData::Constant { .. } => return Err(unavailable("actual Detailed dynamic type")),
            };
            if day == 366 && week.is_none() { continue; }
            let week = week.ok_or_else(|| unavailable("actual Detailed null week dereference"))?;
            if week == previous { continue; }
            self.set_source_week_min_max(week, steps)?;
            let row = &self.week_schedules[week];
            if row.min_value < self.schedules[index.0].min_value { self.schedules[index.0].min_value = row.min_value; }
            if row.max_value > self.schedules[index.0].max_value { self.schedules[index.0].max_value = row.max_value; }
            previous = week;
        }
        self.schedules[index.0].is_min_max_set = true;
        Ok(())
    }
    /// Cached getters do not fabricate the unpassed Native global clock.
    pub(super) fn ensure_source_min_max(&mut self, index: SourceScheduleIndex) -> Result<(), E> {
        let row = self.schedules.get(index.0).ok_or_else(|| unavailable("actual Schedule min/max pointer"))?;
        if row.is_min_max_set { return Ok(()); }
        if matches!(row.data, SelectedScheduleData::Detailed(_)) {
            return Err(unavailable("actual uncached Detailed min/max needs live TimeStepsInHour"));
        }
        // Constant setMinMaxVals ignores the global timestep entirely.
        self.ensure_source_min_max_with_steps(index, 0)
    }
    pub(super) fn source_hr_ts_value(
        &self, index: SourceScheduleIndex, hr: i32, ts: i32,
        clock: &WeatherGlobalState, environment: &WeatherEnvironmentState,
    ) -> Result<f64, E> {
        let row = self.schedules.get(index.0).ok_or_else(|| unavailable("actual Schedule getHrTsVal pointer"))?;
        let detail = match &row.data {
            SelectedScheduleData::Constant { timestep_values } => return timestep_values.first().copied().ok_or_else(|| unavailable("actual Constant tsVals.front2546")),
            SelectedScheduleData::Detailed(detail) => detail,
        };
        if row.ems_actuated_on { return Ok(row.ems_value); }
        if hr > 24 { return Err(unavailable("actual Detailed getHrTsVal Hour>24 fatal branch")); }
        let shift = environment.dst_indicator.checked_mul(i32::from(detail.use_daylight_saving))
            .ok_or_else(|| unavailable("actual Detailed DST integer multiplication"))?;
        let mut hour = hr.checked_add(shift).ok_or_else(|| unavailable("actual Detailed DST hour addition"))?;
        let mut day = environment.day_of_year_schedule;
        let mut weekday = environment.day_of_week;
        let mut holiday = environment.holiday_index;
        if hour > 24 {
            day = day.checked_add(1).ok_or_else(|| unavailable("actual Detailed next schedule day"))?;
            hour -= 24;
            weekday = environment.day_of_week_tomorrow;
            holiday = environment.holiday_index_tomorrow;
        }
        if day == 367 { day = 1; }
        let day = usize::try_from(day).ok().filter(|day| (1..=366).contains(day))
            .ok_or_else(|| unavailable("actual Detailed day-of-year index"))?;
        let week = detail.week_schedules[day].and_then(|number| self.week_schedules.get(number))
            .ok_or_else(|| unavailable("actual Detailed live week pointer"))?;
        let day_type = usize::try_from(if holiday > 0 { holiday } else { weekday }).ok().filter(|n| (1..13).contains(n))
            .ok_or_else(|| unavailable("actual Detailed live day-type index"))?;
        let day = week.day_schedules[day_type].and_then(|number| self.day_schedules.get(number))
            .ok_or_else(|| unavailable("actual Detailed live day pointer"))?;
        let ts = if ts <= 0 { clock.time_steps_in_hour } else { ts };
        if !(1..=24).contains(&hour) || ts < 1 || ts > clock.time_steps_in_hour {
            return Err(unavailable("actual Detailed hour/timestep storage domain"));
        }
        let offset = (hour - 1).checked_mul(clock.time_steps_in_hour).and_then(|x| x.checked_add(ts - 1))
            .and_then(|x| usize::try_from(x).ok()).ok_or_else(|| unavailable("actual Detailed tsVals subscript"))?;
        day.timestep_values.get(offset).copied().ok_or_else(|| unavailable("actual Detailed tsVals storage"))
    }
}
