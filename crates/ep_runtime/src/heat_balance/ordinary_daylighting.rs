//! Genuine ordinary Dayltg zero-input producers on sole live Source owners.
//! Positive daylight/control/map/device bodies stop at their first unported operation.
mod devices;
mod input;
mod state;
mod timestep;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::{OrdinarySolarRadiation, SolarRadiationEnclosure};
use crate::heat_balance::selected_gain_modules::SourceGainModules;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use state::SourceEnclosureDaylight;
use std::convert::Infallible;

/// Selected actual DaylightingData members used on this route. Array members
/// remain constructor-unallocated when Controls count is zero; no getter ran
/// for RefPoints/maps/light wells merely because such raw families are present.
// Actual retained constructor fields; positive reporting consumers unported.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceDaylighting {
    pub(crate) calc_coefficients_first_time: bool,
    pub(crate) get_parameters_input_flag: bool,
    pub(crate) map_results_to_report: bool,
    pub(crate) dfs_report_sizing_days: bool,
    pub(crate) dfs_report_all_shadow_calculation_days: bool,
    pub(crate) max_num_ref_pt_in_any_daylight_control: i32,
    pub(crate) max_num_ref_pt_in_any_enclosure: i32,
    pub(crate) max_enclosure_sub_surfaces: i32,
    pub(crate) total_windows_with_daylighting: i32,
    enclosure_daylight: Option<Vec<SourceEnclosureDaylight>>,
    daylight_controls: Option<Vec<Infallible>>,
    daylight_reference_points: Option<Vec<Infallible>>,
    illuminance_maps: Option<Vec<Infallible>>,
}
impl Default for SourceDaylighting {
    fn default() -> Self {
        Self {
            calc_coefficients_first_time: true,
            get_parameters_input_flag: true,
            map_results_to_report: false,
            dfs_report_sizing_days: false,
            dfs_report_all_shadow_calculation_days: false,
            max_num_ref_pt_in_any_daylight_control: 0,
            max_num_ref_pt_in_any_enclosure: 0,
            max_enclosure_sub_surfaces: 0,
            total_windows_with_daylighting: 0,
            enclosure_daylight: None,
            daylight_controls: None,
            daylight_reference_points: None,
            illuminance_maps: None,
        }
    }
}
impl SourceDaylighting {
    /// Actual DaylightingData.DaylRefPt EPVector size, including constructor-empty.
    pub(crate) fn source_daylight_reference_point_count(&self) -> usize {
        self.daylight_reference_points.as_ref().map_or(0, Vec::len)
    }
    /// Actual DaylightingData.daylightControl EPVector size, not an IP recount.
    pub(crate) fn source_daylight_control_count(&self) -> usize {
        self.daylight_controls.as_ref().map_or(0, Vec::len)
    }
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("actual Source Dayltg positive ordinal"))
}
fn solar(
    radiation: &mut OrdinarySolarRadiation,
    number: i32,
) -> Result<&mut SolarRadiationEnclosure, Error> {
    radiation
        .source_solar_enclosures_mut()
        .and_then(|rows| index(number).ok().and_then(|i| rows.get_mut(i)))
        .ok_or(Error::Unavailable(
            "same actual Source Dayltg Solar enclosure",
        ))
}
fn empty<T>(rows: &Option<Vec<T>>) -> bool {
    rows.as_ref().is_none_or(Vec::is_empty)
}
fn surface_row(
    geometry: &SurfaceGeometryInputState,
    number: i32,
) -> Result<&crate::heat_balance::selected_heat_balance_input::surface::SurfaceTmpInput, Error> {
    geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| index(number).ok().and_then(|i| rows.get(i)))
        .map(|row| &row.row)
        .ok_or(Error::Unavailable(
            "actual same Source Dayltg final Surface",
        ))
}
