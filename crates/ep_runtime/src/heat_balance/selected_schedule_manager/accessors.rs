//! Genuine selected Constant Schedule pointer lookup and min/max methods.
use super::{SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as E;
impl SelectedScheduleManager {
    pub(crate) fn get_source_schedule(&mut self, name: &str) -> Option<SourceScheduleIndex> {
        let index = *self.schedule_map.get(name)?;
        let row = self.schedules.get_mut(index.0)?;
        if !row.is_used {
            row.is_used = true;
        }
        Some(index)
    }
    fn source_constant_min_max(&mut self, index: SourceScheduleIndex) -> Result<(f64, f64), E> {
        let row = self
            .schedules
            .get_mut(index.0)
            .ok_or(E::Unavailable("actual selected Schedule pointer"))?;
        if !row.is_min_max_set {
            row.min_value = row.current_value;
            row.max_value = row.current_value;
            row.is_min_max_set = true;
        }
        Ok((row.min_value, row.max_value))
    }
    pub(crate) fn source_min(&mut self, index: SourceScheduleIndex) -> Result<f64, E> {
        Ok(self.source_constant_min_max(index)?.0)
    }
    pub(crate) fn source_max(&mut self, index: SourceScheduleIndex) -> Result<f64, E> {
        Ok(self.source_constant_min_max(index)?.1)
    }
    pub(crate) fn source_check_min(
        &mut self,
        index: SourceScheduleIndex,
        minimum: f64,
    ) -> Result<bool, E> {
        Ok(f64::from(f32::EPSILON) >= minimum - self.source_min(index)?)
    }
    pub(crate) fn source_constant_day_min_max(
        &self,
        index: SourceScheduleIndex,
    ) -> Result<(f64, f64), E> {
        let row = self
            .schedules
            .get(index.0)
            .ok_or(E::Unavailable("actual Constant day-type schedule pointer"))?;
        // ScheduleConstant3534-3538 ignores day-group and uses currentVal;
        // it neither sets isMinMaxSet nor consumes any derived schedule cache.
        Ok((row.current_value, row.current_value))
    }
}
