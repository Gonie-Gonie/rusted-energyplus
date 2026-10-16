//! Genuine primary Weather children on the same persistent WeatherManager owner.
use super::WeatherDayState;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;

/// Actual WeatherManager.hh427--434 row; no row is made by the zero getter.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceUnderwaterBoundary {
    pub(crate) name: String,
    pub(crate) distance_from_leading_edge: f64,
    pub(crate) oscm_index: i32,
    pub(crate) water_temperature_schedule: Option<SourceScheduleIndex>,
    pub(crate) velocity_schedule: Option<SourceScheduleIndex>,
}

impl WeatherDayState {
    /// Literal zero-object CheckIfAnyUnderwaterBoundaries164--222 prefix/return.
    /// Positive input stops before getObjectItem175 and before emplace_back187.
    pub(crate) fn check_source_underwater_boundaries(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<bool, Error> {
        shortcut.current_module_object = "SurfaceProperty:Underwater".into();
        let number = i32::try_from(input.get_num_objects_found(&shortcut.current_module_object))
            .map_err(|_| Error::Unavailable("actual underwater Num outside Native int173"))?;
        if number > 0 {
            return Err(Error::Unavailable(
                "actual positive CheckIfAnyUnderwaterBoundaries getObjectItem175",
            ));
        }
        // Native local errorsFound=false cannot change on the empty getter loop.
        // Do not reset, allocate or infer this module's retained boundary vector.
        Ok(number > 0)
    }

    /// Exact empty native vector traversal253, independent of any input count.
    pub(crate) fn update_source_underwater_boundaries(&mut self) -> Result<(), Error> {
        if !self.weather.underwater_boundaries.is_empty() {
            return Err(Error::Unavailable(
                "actual UnderwaterBoundary waterTempSched getCurrentVal254",
            ));
        }
        Ok(())
    }

    /// Actual Simulation357 child: real shared shortcut write and count query.
    /// Zero leaves all three actual Environment schedule pointers untouched.
    pub(crate) fn read_source_variable_location_orientation(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "Site:VariableLocation".into();
        let number = i32::try_from(input.get_num_objects_found(&shortcut.current_module_object))
            .map_err(|_| {
                Error::Unavailable("actual variable-location count outside Native int275")
            })?;
        if number == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual positive ReadVariableLocationOrientation getObjectItem278",
        ))
    }
}
