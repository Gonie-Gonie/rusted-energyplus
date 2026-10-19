//! Actual Source solar array producers and opaque timestep consumers.
//! Held until the genuine InitHeatBalance/Solar child host is assembled.
mod allocation;
pub(crate) mod array;
mod distribution;
mod environment;
mod general_shadow;
mod heat_gain;
mod initialization;
mod opaque_exterior;
mod period;
mod sky;
pub(crate) mod state;
use super::OrdinarySolarRadiation;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, SelectedSolarDistribution,
};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
pub(crate) use state::SourceSolarTimestepState;

fn ordinal(value: i32) -> Result<usize, Error> {
    usize::try_from(value)
        .ok()
        .filter(|&v| v > 0)
        .ok_or(Error::Unavailable("positive actual Source Solar ordinal"))
}
fn geometry_properties(
    geometry: &SurfaceGeometryInputState,
    number: usize,
) -> Result<crate::geometry::GetVerticesProperties, Error> {
    geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(number - 1))
        .and_then(|row| row.row.vertex_progress.properties)
        .ok_or(Error::Unavailable(
            "same actual final Source Surface geometry",
        ))
}
// Borrow the sole mutable Native SurfaceData row; geometric properties are not its area state.
fn source_surface(
    geometry: &SurfaceGeometryInputState,
    number: usize,
) -> Result<&crate::heat_balance::selected_heat_balance_input::surface::SurfaceTmpInput, Error> {
    geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(number - 1))
        .map(|surface| &surface.row)
        .ok_or(Error::Unavailable(
            "same actual final Source Surface area owner",
        ))
}
fn surface_count(geometry: &SurfaceGeometryInputState) -> Result<usize, Error> {
    usize::try_from(geometry.total_surfaces)
        .map_err(|_| Error::Unavailable("actual Source TotSurfaces"))
}
fn get(values: &Option<Vec<f64>>, number: usize) -> Result<f64, Error> {
    values
        .as_ref()
        .and_then(|rows| rows.get(number - 1))
        .copied()
        .ok_or(Error::Unavailable(
            "same actual allocated HBSM Solar scalar",
        ))
}
fn set(values: &mut Option<Vec<f64>>, number: usize, value: f64) -> Result<(), Error> {
    *values
        .as_mut()
        .and_then(|rows| rows.get_mut(number - 1))
        .ok_or(Error::Unavailable(
            "same actual allocated HBSM Solar scalar",
        ))? = value;
    Ok(())
}
fn add(values: &mut Option<Vec<f64>>, number: usize, value: f64) -> Result<(), Error> {
    let previous = get(values, number)?;
    set(values, number, previous + value)
}

/// Actual child operations whose other module owners must be supplied by the
/// production caller. There is deliberately no default/no-op implementation.
/// The same mutable Solar/Surface owners are passed through each child.
pub(crate) trait SourceSolarChildren {
    /// The real Source shd file/null-stream host, opened before getter flags.
    fn open_shading_stream(
        &mut self,
        output_shd: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;
    /// Actual5621-5727 report and diagnostic effects on the physical stream.
    fn write_shadow_combinations(
        &mut self,
        solar: &mut OrdinarySolarRadiation,
        geometry: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;
    fn reset_shading_stream(&mut self);
    fn display_shadowing_update(
        &mut self,
        globals: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;
    fn set_display_performance_simulation_flag(&mut self) -> Result<(), Error>;
    fn register_solar_module_outputs(
        &mut self,
        solar: &OrdinarySolarRadiation,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;
    fn register_sky_diffuse_outputs(
        &mut self,
        solar: &OrdinarySolarRadiation,
        geometry: &SurfaceGeometryInputState,
        detailed_shading: bool,
    ) -> Result<(), Error>;
    fn calc_daylight_coefficients(
        &mut self,
        solar: &mut OrdinarySolarRadiation,
        geometry: &mut SurfaceGeometryInputState,
        globals: &WeatherGlobalState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;
}
