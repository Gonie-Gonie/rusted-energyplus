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
