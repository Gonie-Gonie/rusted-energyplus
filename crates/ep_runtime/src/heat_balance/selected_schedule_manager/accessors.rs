//! Genuine native Schedule pointer use propagation and selected min/max.
use super::{SelectedScheduleData, SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as E;
impl SelectedScheduleManager {
    pub(crate) fn get_source_schedule(&mut self, name: &str) -> Result<Option<SourceScheduleIndex>, E> {
        let Some(&index) = self.schedule_map.get(name) else { return Ok(None); };
        let row = self.schedules.get_mut(index.0).ok_or(E::Unavailable("actual Schedule map/index association"))?;
        if !row.is_used {
            row.is_used = true;
            if matches!(row.data, SelectedScheduleData::Detailed(_)) {
                // Native2560-2603 propagates used through each actual pointer.
                for day in 1..=366 {
                    let SelectedScheduleData::Detailed(detail) = &self.schedules[index.0].data else {
                        return Err(E::Unavailable("actual GetSchedule Detailed dynamic type"));
                    };
                    let Some(week) = detail.week_schedules[day] else { continue; };
                    let week_row = self.week_schedules.get_mut(week).ok_or(E::Unavailable("actual GetSchedule week pointer"))?;
                    if week_row.is_used { continue; }
                    week_row.is_used = true;
                    for day_type in 1..13 {
                        let day = self.week_schedules[week].day_schedules[day_type]
                            .ok_or(E::Unavailable("actual GetSchedule null day dereference"))?;
                        self.day_schedules.get_mut(day).ok_or(E::Unavailable("actual GetSchedule day pointer"))?.is_used = true;
                    }
                }
            }
        }
        Ok(Some(index))
    }
    fn source_selected_min_max(&mut self, index: SourceScheduleIndex) -> Result<(f64, f64), E> {
        self.ensure_source_min_max(index)?;
        let row = self.schedules.get(index.0).ok_or(E::Unavailable("actual selected Schedule pointer"))?;
        Ok((row.min_value, row.max_value))
    }
    pub(crate) fn source_min(&mut self, index: SourceScheduleIndex) -> Result<f64, E> {
        Ok(self.source_selected_min_max(index)?.0)
    }
    pub(crate) fn source_max(&mut self, index: SourceScheduleIndex) -> Result<f64, E> {
        Ok(self.source_selected_min_max(index)?.1)
    }
    pub(crate) fn source_check_min(&mut self, index: SourceScheduleIndex, minimum: f64) -> Result<bool, E> {
        Ok(f64::from(f32::EPSILON) >= minimum - self.source_min(index)?)
    }

    /// Actual ScheduleBase::checkMinMaxVals for the caller's two inclusive bounds.
    pub(crate) fn source_check_min_max_inclusive(
        &mut self, index: SourceScheduleIndex, minimum: f64, maximum: f64,
    ) -> Result<bool, E> {
        let (actual_minimum, actual_maximum) = self.source_selected_min_max(index)?;
        let min_ok = f64::from(f32::EPSILON) >= minimum - actual_minimum;
        let max_ok = actual_maximum - maximum <= f64::from(f32::EPSILON);
        Ok(min_ok && max_ok)
    }
    /// Exact Constant::hasVal. Detailed traversal is an explicit separate bound.
    pub(crate) fn source_has_value(&self, index: SourceScheduleIndex, value: f64) -> Result<bool, E> {
        let row = self.schedules.get(index.0).ok_or(E::Unavailable("actual hasVal Schedule pointer"))?;
        match &row.data {
            SelectedScheduleData::Constant { .. } => Ok(value == row.current_value),
            SelectedScheduleData::Detailed(_) => Err(E::Unavailable("actual ScheduleDetailed::hasVal3433")),
        }
    }
}
