//! Concrete ordinary InitSurface providers over the sole retained Source owners.
//! Eight admitted-route child bodies are attached; positive unsupported children
//! stop at their actual call boundary. No default-success callback is supplied.
use super::OrdinarySurfaceInitialization;
use super::manager::{SourceInitSurfaceOwners, SourceInitSurfaceProviders};
use crate::heat_balance::convection::surface_environment::source_rows;
use crate::heat_balance::inside_convection_dispatcher::{
    InsideSurfaceTemperatureOwner, source::SourceInsideConvection,
};
use crate::heat_balance::ordinary_air_heat_balance::environment::{
    SourceZoneFanHeatBalanceArrays, initialize_source_air_surface_environment,
};
use crate::heat_balance::ordinary_daylighting::SourceDaylighting;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost,
};

/// Temporary mutable borrows only. The enclosing runtime/context remains the
/// sole owner of fan-zone arrays, Dayltg, convection state and equipment/Nodes.
/// The same convection object is borrowed by the later inside iteration host.
pub(crate) struct SourceOrdinaryInitSurfaceHost<'a> {
    pub(crate) fan_zone: &'a mut SourceZoneFanHeatBalanceArrays,
    pub(crate) daylighting: &'a mut SourceDaylighting,
    pub(crate) convection: &'a mut SourceInsideConvection,
    pub(crate) equipment: &'a mut OrdinaryEquipment,
    pub(crate) psychrometrics: &'a mut crate::psychrometrics::SourcePsychrometricAccess,
    pub(crate) recurring:
        &'a mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
    pub(crate) hvac: &'a crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost> SourceInitSurfaceProviders<H, D>
    for SourceOrdinaryInitSurfaceHost<'_>
{
    fn set_surface_outbulb_temperature(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        source_rows::set_source_surface_out_bulb_temperatures(
            owners.geometry,
            owners.physical_surfaces,
            owners.environment,
        )
    }
    fn check_surface_outbulb_temperature(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        source_rows::check_source_surface_out_bulb_temperatures(
            owners.geometry,
            owners.physical_surfaces,
            owners.backend,
        )
    }
    fn set_surface_wind_speed(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        source_rows::set_source_surface_wind_speeds(
            owners.geometry,
            owners.physical_surfaces,
            owners.environment,
        )
    }
    fn set_surface_wind_direction(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        source_rows::set_source_surface_wind_directions(
            owners.geometry,
            owners.physical_surfaces,
            owners.environment,
        )
    }
    fn apply_linked_outdoor_nodes(
        &mut self,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        // Parent316 is true only outside current raw local-environment admission.
        Err(Error::Unavailable(
            "actual positive linked outdoor-air Surface/Node branch317-324",
        ))
    }
    fn apply_outdoor_ems_overrides(
        &mut self,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive EMS surface override arrays328-346",
        ))
    }
    fn initialize_thermal_and_flux_histories(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        let cadence_scope = crate::heat_balance::source_surface_cadence::enter(
            crate::heat_balance::source_surface_cadence::Procedure::InitializeHistories,
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                surface,
                owners.clock,
                owners.globals,
                owners.environment,
                owners.globals.kickoff_simulation,
                surface.inside_surface_iterations,
            ),
        );
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            initialize_source_air_surface_environment(
                surface,
                owners.air,
                self.fan_zone,
                owners.radiation,
                owners.geometry,
                owners.zones,
                owners.constructions,
                owners.public_ctf,
                owners.history,
                owners.physical_surfaces,
                owners.globals,
                owners.clock,
                owners.environment,
            )
        })();
        cadence_scope.finish(
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                surface,
                owners.clock,
                owners.globals,
                owners.environment,
                owners.globals.kickoff_simulation,
                surface.inside_surface_iterations,
            ),
            cadence_result.is_ok(),
        );
        if cadence_result.is_ok() {
            crate::heat_balance::source_surface_cadence::record(
                "actual_InitThermalAndFluxHistories_return",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    surface,
                    owners.clock,
                    owners.globals,
                    owners.environment,
                    owners.globals.kickoff_simulation,
                    surface.inside_surface_iterations,
                ),
            );
        }
        cadence_result
    }
    fn evaluate_outside_movable_insulation(
        &mut self,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive EvalOutsideMovableInsulation child388",
        ))
    }
    fn evaluate_inside_movable_insulation(
        &mut self,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive EvalInsideMovableInsulation child389",
        ))
    }
    fn compute_diffuse_solar_interzone_exchange(
        &mut self,
        _surface: &mut OrdinarySurfaceInitialization,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive ComputeDifSolExcZonesWIZWindows child439",
        ))
    }
    fn initialize_daylighting(
        &mut self,
        first_time: bool,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        self.daylighting.initialize_daylighting(
            first_time,
            owners.geometry,
            owners.zones,
            owners.radiation,
            owners.gains,
            owners.gain_modules,
            owners.globals,
            owners.clock,
            owners.environment,
            owners.backend,
        )
    }
    fn control_window_gap_airflow(
        &mut self,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive WindowGapAirflowControl child448",
        ))
    }
    fn manage_daylighting(
        &mut self,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        self.daylighting
            .manage_daylighting(owners.radiation, owners.clock, owners.environment)
    }
    fn initialize_inside_convection_temporary(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        self.convection.initialize(
            InsideSurfaceTemperatureOwner::SurfTempInTmp,
            owners.geometry,
            &mut surface.arrays,
            owners.zones,
            owners.air,
            owners.constructions,
            owners.globals,
            owners.clock,
            self.equipment,
            owners.input,
            owners.report_inputs,
            owners.scan,
            crate::heat_balance::inside_convection_dispatcher::source::SourceInsideConvectionPropertyOwners {
                environment: owners.environment, hvac: self.hvac,
                psychrometrics: self.psychrometrics, recurring: self.recurring,
                developer: &owners.project.system.developer_flag,
            },
            owners.backend,
        )
    }
    fn initialize_finite_difference(
        &mut self,
        _surface: &mut OrdinarySurfaceInitialization,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive InitHeatBalFiniteDiff child486",
        ))
    }
    fn initialize_internal_source_ctf_constants(
        &mut self,
        _surface: &mut OrdinarySurfaceInitialization,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive internal-source CTF constants537-579",
        ))
    }
    fn gather_component_surface_absorption_factors(
        &mut self,
        _surface: &mut OrdinarySurfaceInitialization,
        _owners: &mut SourceInitSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive GatherComponentLoadsSurfAbsFact child614",
        ))
    }
}
