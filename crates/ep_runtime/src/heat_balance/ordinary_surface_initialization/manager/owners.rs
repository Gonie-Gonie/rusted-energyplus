//! Temporary borrowed Source owners; no scientific state or output value copy.
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::ctf_public_initialization::OrderedCtfPublicInitialization;
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::facility_electric_service::FacilityElectricService;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_gain_modules::SourceGainModules;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    SelectedHeatBalanceInputState, shadowing::SourceShadowingInput,
    surface::SurfaceGeometryInputState, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_material_data::SelectedMaterialData;
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::SourceOutputRegistration;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::ReportInputs;
use std::io::Write;

pub(crate) struct SourceInitSurfaceOwners<'a, 'backend, H, D> {
    pub(crate) geometry: &'a mut SurfaceGeometryInputState,
    /// Sole moved DataSurface outdoor records and retained physical metadata.
    pub(crate) physical_surfaces: &'a mut [SourceSurfaceOutdoorState],
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) constructions: &'a mut SelectedHeatBalanceInputState,
    pub(crate) radiation: &'a mut OrdinarySolarRadiation,
    pub(crate) air: &'a mut SourceAirHeatBalance,
    pub(crate) history: &'a mut CtfThermalHistoryState,
    pub(crate) public_ctf: &'a OrderedCtfPublicInitialization,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) project: &'a SelectedProjectData,
    pub(crate) shadowing: &'a SourceShadowingInput,
    pub(crate) gains: &'a mut SelectedInternalHeatGains,
    pub(crate) gain_modules: &'a mut SourceGainModules,
    pub(crate) facility: Option<&'a mut FacilityElectricService>,
    pub(crate) schedules: &'a SelectedScheduleManager,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) output_host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) materials: &'a SelectedMaterialData,
    pub(crate) predefined_registry: &'a PredefinedRegistry,
    pub(crate) predefined_table: &'a mut PredefinedStringTable,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
