//! Actual PlantManager4596-4622 endrun read on the same retained DataPlant owner.
use super::{Error, SourceHvacManager};

impl SourceHvacManager {
    pub(crate) fn check_source_ongoing_plant_warnings(&self) -> Result<(), Error> {
        // The Native signed 1..TotNumLoops loop is empty for zero or negatives.
        // No count query, constructor reset or PlantLoop replacement occurs here.
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable(
                "actual CheckOngoingPlantWarnings first PlantLoop Demand side4601",
            ));
        }
        Ok(())
    }
}

impl SourceHvacManager {
    /// Actual DumpAirLoopStatistics1998-2024. The live SysVars flag is the
    /// first predicate; a zero NumPrimaryAirSys is not a substitute guard.
    pub(crate) fn dump_source_air_loop_statistics(
        &self,
        system: &crate::heat_balance::selected_project_data::ProjectSystemVariables,
    ) -> Result<(), Error> {
        if !system.track_air_loop_env_flag {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual DumpAirLoopStatistics local StatisticsFilePath.open2015",
        ))
    }
}
