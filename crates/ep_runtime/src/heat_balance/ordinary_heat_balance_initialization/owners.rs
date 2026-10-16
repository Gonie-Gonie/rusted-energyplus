use super::*;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::ordinary_daylighting::SourceDaylighting;
use crate::heat_balance::ordinary_window_optical_initialization::SourceWindowOpticalInitialization;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_gain_modules::SourceGainModules;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_material_data::SelectedMaterialData;
use crate::heat_balance::selected_simulation_startup::SelectedConstructionBounds;
use ep_model::ReportInputs;

/// Disjoint live owners. The CTF and Solar children construct temporary hosts
/// inside their calls, so no IP/control/Zone borrow is retained across calls.
pub(crate) struct SourceInitHeatBalanceOwners<'a, 'b> {
    pub(crate) surface: &'a mut OrdinarySurfaceInitialization,
    pub(crate) geometry: &'a mut SurfaceGeometryInputState,
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) control: &'a mut SelectedHeatBalanceInputState,
    pub(crate) air: &'a mut SourceAirHeatBalance,
    pub(crate) radiation: &'a mut OrdinarySolarRadiation,
    pub(crate) fan_zone: &'a mut SourceZoneFanHeatBalanceArrays,
    pub(crate) gains: &'a mut SelectedInternalHeatGains,
    pub(crate) gain_modules: &'a mut SourceGainModules,
    pub(crate) daylighting: &'a mut SourceDaylighting,
    pub(crate) window: &'a mut SourceWindowOpticalInitialization,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) weather_owner: &'a WeatherOwnerState,
    pub(crate) shadowing: &'a SourceShadowingInput,
    pub(crate) materials: &'a SelectedMaterialData,
    pub(crate) bounds: &'a SelectedConstructionBounds,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) backend: &'a mut SourceErrorBackend<'b>,
}
