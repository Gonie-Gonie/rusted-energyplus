//! Persistent genuine Source owners, separate from the legacy thermal factory.
//! The actual weather session and all context/report input/output owners remain
//! borrowed at each call; no global, clock or numerical snapshot is retained here.
mod body;
mod first_input;
mod next_environment;
mod step;
mod weather_exterior;
use super::ctf_thermal_history::CtfThermalHistoryState;
use super::inside_convection_dispatcher::source::SourceInsideConvection;
use super::ordinary_air_heat_balance::SourceAirHeatBalance;
use super::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays;
use super::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector;
use super::ordinary_daylighting::SourceDaylighting;
use super::ordinary_exterior_energy_use::SourceExteriorEnergyUse;
use super::ordinary_heat_balance_initialization::SourceHeatBalanceManager;
use super::ordinary_heat_balance_initialization::ctf::SourceCtfInitialization;
use super::ordinary_hvac_manager::SourceHvacManager;
use super::ordinary_surface_initialization::OrdinarySurfaceInitialization;
use super::ordinary_surface_management::SourceSurfaceManagement;
use super::ordinary_surface_source_averages::SourceFinalSurfaceHeatBalance;
use super::ordinary_tabular_reports::SourceTabularReports;
use super::ordinary_thermal_comfort::{SourceComfortPredefinedSummary, SourceThermalComfort};
use super::ordinary_window_optical_initialization::SourceWindowOpticalInitialization;
use super::radiation::ordinary::shading_report::SourceShadingStream;
use super::selected_gain_modules::SourceGainModules;
use super::selected_heat_balance_input::HeatBalanceInputError;
use super::warmup::report::{WarmupReportConfiguration, WarmupReportState};
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;

/// Actual post-input surface metadata and its sole moved outdoor owner.
/// Numerical history is still genuinely unallocated until the Source allocator.
pub(crate) struct SourceSessionSurfaces {
    pub(crate) convection: SourceInsideConvection,
    pub(crate) history: CtfThermalHistoryState,
    pub(crate) outdoor: Vec<SourceSurfaceOutdoorState>,
}

/// Opaque persistent ordinary Source session, retained across timestep calls.
/// Construction and invocation belong to the actual runtime caller; this type
/// does not initialize weather, run input getters, allocate thermal arrays or
/// claim that any environment is ready for simulation.
pub struct OrdinarySourceSession {
    pub(crate) heat_balance: SourceHeatBalanceManager,
    pub(crate) ctf: SourceCtfInitialization,
    pub(crate) surface: OrdinarySurfaceInitialization,
    pub(crate) air: SourceAirHeatBalance,
    pub(crate) controller: SourceZoneAirPredictorCorrector,
    pub(crate) fan: SourceZoneFanHeatBalanceArrays,
    pub(crate) hvac: SourceHvacManager,
    pub(crate) window: SourceWindowOpticalInitialization,
    pub(crate) surface_management: SourceSurfaceManagement,
    pub(crate) daylighting: SourceDaylighting,
    pub(crate) gain_modules: SourceGainModules,
    pub(crate) exterior: SourceExteriorEnergyUse,
    pub(crate) eco_roof: crate::weather::day::source_current_weather::SourceEcoRoofData,
    pub(crate) comfort: SourceThermalComfort,
    pub(crate) comfort_summary: SourceComfortPredefinedSummary,
    pub(crate) tabular: SourceTabularReports,
    pub(crate) final_surface: SourceFinalSurfaceHeatBalance,
    pub(crate) shading_stream: SourceShadingStream,
    pub(crate) surfaces: Option<SourceSessionSurfaces>,
}

impl OrdinarySourceSession {
    /// Actual native module constructor state only. Caller-supplied report
    /// names/configuration are metadata; the real Source report constructor
    /// leaves its seven arrays unallocated and supplies the native scalar flags.
    pub(crate) fn new(
        zone_names: Vec<String>,
        configuration: WarmupReportConfiguration,
    ) -> Result<Self, HeatBalanceInputError> {
        let report = WarmupReportState::new_source_unallocated(zone_names, configuration);
        Ok(Self {
            heat_balance: SourceHeatBalanceManager::new(report)?,
            ctf: SourceCtfInitialization::default(),
            surface: OrdinarySurfaceInitialization::default(),
            air: SourceAirHeatBalance::default(),
            controller: SourceZoneAirPredictorCorrector::default(),
            fan: SourceZoneFanHeatBalanceArrays::default(),
            hvac: SourceHvacManager::default(),
            window: SourceWindowOpticalInitialization::default(),
            surface_management: SourceSurfaceManagement::default(),
            daylighting: SourceDaylighting::default(),
            gain_modules: SourceGainModules::default(),
            exterior: SourceExteriorEnergyUse::default(),
            eco_roof: crate::weather::day::source_current_weather::SourceEcoRoofData::default(),
            comfort: SourceThermalComfort::default(),
            comfort_summary: SourceComfortPredefinedSummary::default(),
            tabular: SourceTabularReports::default(),
            final_surface: SourceFinalSurfaceHeatBalance::default(),
            shading_stream: SourceShadingStream::default(),
            surfaces: None,
        })
    }
}
