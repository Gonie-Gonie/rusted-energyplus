//! Actual unconditional/conditional Source module boundaries; no default bodies.
use super::{Error, SourceHvacManager, SourceHvacOwners};
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirRegistrationChildren;
pub(crate) trait SourceHvacChildren: SourceZoneAirRegistrationChildren {
    fn manage_electric_facility(
        &mut self,
        first_hvac_iteration: bool,
        simulate_electrical_circuits: &mut bool,
        update_meters_only: bool,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error>;
    fn run(
        &mut self,
        child: SourceHvacChild,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error>;
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SourceHvacChild {
    ContaminantGetZoneSetPoints { prior_time_step: f64 },
    ContaminantPredictStep { prior_time_step: f64 },
    ContaminantCorrectStep { prior_time_step: f64 },
    ContaminantPushSystemHistories { prior_time_step: f64 },
    ContaminantPushZoneHistories { prior_time_step: f64 },
    CalculateSimpleAirflow { system_step: Option<i32> },
    SetHeatToReturnAirFlag,
    SimulateHvac,
    ManageWaterInits,
    ManageDemand,
    DetectOscillatingZoneTemperature,
    UpdateZoneListAndGroupLoads,
    UpdateIceFractions,
    ManageWater,
    UpdatePlantNodeThermalHistory,
    CalculateHeatEmissionReport,
    WriteWarmupEnvironmentStamp,
    CalculateMoreNodeInfo,
    CalculatePollution,
    InitializeEnergyReports,
    ReportSystemEnergyUse,
    ReportAirHeatBalance,
    GatherComponentLoadsHvac,
    ReportVentilationLoads,
    UpdateDataAndReportSystem,
    UpdateTabularReportsSystem,
    UpdateZoneSizingDuringDay,
    UpdateFacilitySizingDuringDay,
    CheckConcurrentEirPlantLoopHeatPumpOperation,
    UpdateDemandManagers,
}
