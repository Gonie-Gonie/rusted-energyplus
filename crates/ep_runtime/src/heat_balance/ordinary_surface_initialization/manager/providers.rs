//! Required real child calls at their actual InitSurface Source positions.
use super::*;

/// Implementations must invoke the real accepted Source children on these live
/// owners. This trait has no defaults or success placeholder implementation.
pub(crate) trait SourceInitSurfaceProviders<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>
{
    fn set_surface_outbulb_temperature(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn check_surface_outbulb_temperature(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn set_surface_wind_speed(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn set_surface_wind_direction(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn apply_linked_outdoor_nodes(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn apply_outdoor_ems_overrides(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    /// Complete real2231-2449 order: Zone reconstruction/thermostat, MRT,
    /// Space reconstruction, surface bulk and sole history initialization.
    /// Bypass legacy CtfSurfaceLifecycle::begin_step and its copied temperatures.
    fn initialize_thermal_and_flux_histories(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn evaluate_outside_movable_insulation(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn evaluate_inside_movable_insulation(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn compute_diffuse_solar_interzone_exchange(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    /// Real Dayltg::initDaylighting includes the same gain-owned space power
    /// reduction reset at5639-5642; no reset is appended outside that child.
    fn initialize_daylighting(
        &mut self,
        first_time: bool,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn control_window_gap_airflow(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn manage_daylighting(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    /// Source473 passes the actual SurfTempInTmp slice. The CTF-only driver
    /// has a distinct provider method for the actual SurfTempIn argument.
    fn initialize_inside_convection_temporary(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn initialize_finite_difference(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    /// Source537-579 positive internal-source continuation, if actually reached.
    fn initialize_internal_source_ctf_constants(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
    fn gather_component_surface_absorption_factors(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
}
