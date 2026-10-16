//! Mandatory real children; all mutable simulation flags remain on the borrowed HVAC owner.
use super::super::{Error, SourceHvacChildren, SourceHvacManager, SourceHvacOwners};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceHvacSimulationChild {
    RegisterSolverOutputs,
    SetupPlantEmsActuators,
    RegisterPlantSolverOutputs,
    InitializeOneTimePlantSizing {
        loop_number: i32,
    },
    ManageSystemAvailability,
    ManageZoneEquipment {
        first_hvac_iteration: bool,
    },
    ManageNonZoneEquipment {
        first_hvac_iteration: bool,
    },
    ManageAirLoops {
        first_hvac_iteration: bool,
    },
    /// Same actual facility object and same SimElecCircuitsFlag by-reference, UpdateMetersOnly=false.
    ManageElectricPowerService {
        first_hvac_iteration: bool,
    },
    AirflowNetworkBalance {
        first_hvac_iteration: bool,
        iteration: Option<i32>,
    },
}
pub(crate) trait SourceHvacSimulationChildren: SourceHvacChildren {
    fn run_simulation(
        &mut self,
        child: SourceHvacSimulationChild,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error>;
}
