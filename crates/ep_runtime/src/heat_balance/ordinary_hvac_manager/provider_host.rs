//! Concrete HVAC children reborrow the same run owners at every actual call.
//! Remaining reporting/facility children are mandatory and have no default.
use super::simulation::air_loops::SourceAirLoopOwners;
use super::simulation::output_registration::{SourceHvacOutputContext, SourceHvacOutputOwners};
use super::simulation::{SourceHvacSimulationChild, SourceHvacSimulationChildren};
use super::{Error, SourceHvacChild, SourceHvacChildren, SourceHvacManager, SourceHvacOwners};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::{
    SourceZoneSpaceHeatBalanceData,
    environment::SourceZoneFanHeatBalanceArrays,
    output_registration::SourceInitAirOutputHost,
    predictor_corrector::{
        SourceSystemMoistureDemand, SourceSystemSensibleDemand, SourceZoneAirRegistrationChildren,
        simple_air::SourceAirReportRegistrationContext,
    },
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost,
};
use crate::weather::day::WeatherGlobalState;
use std::io::Write;

pub(crate) trait SourceHvacRemainingChildren<H, D> {
    fn run(
        &mut self,
        child: SourceHvacChild,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
        eio: &mut dyn Write,
    ) -> Result<(), Error>;
    fn manage_electric_facility(
        &mut self,
        first: bool,
        simulate: &mut bool,
        update_meters_only: bool,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
        eio: &mut dyn Write,
    ) -> Result<(), Error>;
    /// The normal child borrows the actual owners.hvac.sim_electric_circuits.
    /// It is never copied out and restored after the call.
    fn manage_normal_electric_facility(
        &mut self,
        first: bool,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
        eio: &mut dyn Write,
    ) -> Result<(), Error>;
}
pub(crate) struct SourceHvacProviderHost<'a, H, D, R> {
    pub(crate) output: SourceHvacOutputContext<'a, H, D>,
    pub(crate) remaining: &'a mut R,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost, R>
    SourceHvacProviderHost<'_, H, D, R>
{
    fn init_air_output(&mut self) -> SourceInitAirOutputHost<'_, H, D> {
        SourceInitAirOutputHost {
            registration: self.output.registration,
            prescan: self.output.prescan,
            reports: self.output.reports,
            minimum_report_frequency: self.output.minimum_report_frequency,
            host: self.output.host,
            dictionary_host: self.output.dictionary_host,
        }
    }
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost, R: SourceHvacRemainingChildren<H, D>>
    SourceHvacChildren for SourceHvacProviderHost<'_, H, D, R>
{
    fn manage_electric_facility(
        &mut self,
        first: bool,
        simulate: &mut bool,
        update_meters_only: bool,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        self.remaining.manage_electric_facility(
            first,
            simulate,
            update_meters_only,
            manager,
            owners,
            &mut self.output,
            eio,
        )
    }
    fn run(
        &mut self,
        child: SourceHvacChild,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        use SourceHvacChild as Child;
        match child {
            Child::CalculateSimpleAirflow { system_step } => {
                manager.calculate_source_simple_airflow(system_step, owners)
            }
            Child::SetHeatToReturnAirFlag => owners.equipment.set_source_heat_to_return_air_flags(
                owners.hvac,
                manager,
                owners.globals,
                owners.zones,
            ),
            Child::SimulateHvac => manager.simulate_source_hvac(owners, self, eio),
            Child::ManageWaterInits => manager.ancillary.water.manage_source_water_inits(
                owners.globals,
                owners.clock,
                owners.hvac,
            ),
            Child::ManageWater => manager.ancillary.water.manage_source_water(owners.input),
            other => self
                .remaining
                .run(other, manager, owners, &mut self.output, eio),
        }
    }
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost, R: SourceHvacRemainingChildren<H, D>>
    SourceHvacSimulationChildren for SourceHvacProviderHost<'_, H, D, R>
{
    fn run_simulation(
        &mut self,
        child: SourceHvacSimulationChild,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        use SourceHvacSimulationChild as Child;
        match child {
            Child::RegisterSolverOutputs => self.output.register_source_solver_outputs(
                &SourceHvacOutputOwners {
                    manager,
                    control: owners.control,
                },
                owners.input,
                owners.clock.minutes_in_time_step,
                Some(crate::heat_balance::error_backend::SourceDiagnosticFlags {
                    kickoff: &owners.globals.kickoff_simulation,
                    developer: &owners.project.system.developer_flag,
                    warmup: &owners.clock.warmup_flag,
                    doing_sizing: &owners.globals.doing_sizing,
                }),
                owners.backend,
                eio,
            ),
            Child::RegisterPlantSolverOutputs => self.output.register_source_plant_solver_outputs(
                &SourceHvacOutputOwners {
                    manager,
                    control: owners.control,
                },
                owners.input,
                owners.clock.minutes_in_time_step,
                Some(crate::heat_balance::error_backend::SourceDiagnosticFlags {
                    kickoff: &owners.globals.kickoff_simulation,
                    developer: &owners.project.system.developer_flag,
                    warmup: &owners.clock.warmup_flag,
                    doing_sizing: &owners.globals.doing_sizing,
                }),
                owners.backend,
                eio,
            ),
            Child::ManageSystemAvailability => manager.manage_source_system_availability(owners),
            Child::ManageZoneEquipment {
                first_hvac_iteration,
            } => manager.manage_source_zone_equipment(
                first_hvac_iteration,
                owners,
                &mut self.output,
                eio,
            ),
            Child::ManageNonZoneEquipment {
                first_hvac_iteration,
            } => manager.manage_source_non_zone_equipment(first_hvac_iteration, owners),
            Child::ManageAirLoops {
                first_hvac_iteration,
            } => {
                let mut air_loop_owners = SourceAirLoopOwners {
                    input: owners.input,
                    backend: owners.backend,
                    globals: owners.globals,
                    clock: owners.clock,
                    hvac: owners.hvac,
                    project: owners.project,
                    equipment: owners.equipment,
                    sizing: owners.sizing,
                    distribution_simulated: &owners
                        .controller
                        .subsystems
                        .airflow_network
                        .distribution_simulated,
                };
                let mut output = SourceAirReportRegistrationContext {
                    registration: self.output.registration,
                    prescan: self.output.prescan,
                    reports: self.output.reports,
                    minimum_report_frequency: self.output.minimum_report_frequency,
                    minutes_in_time_step: owners.clock.minutes_in_time_step,
                    host: self.output.host,
                    eio,
                    dictionary_host: self.output.dictionary_host,
                };
                manager.simulation.air_loop.manage_source_air_loops(
                    first_hvac_iteration,
                    &mut air_loop_owners,
                    &mut output,
                )
            }
            Child::ManageElectricPowerService {
                first_hvac_iteration,
            } => self.remaining.manage_normal_electric_facility(
                first_hvac_iteration,
                manager,
                owners,
                &mut self.output,
                eio,
            ),
            Child::AirflowNetworkBalance {
                first_hvac_iteration: _,
                iteration: _,
            } => owners
                .controller
                .subsystems
                .airflow_network
                .manage_source_balance(owners.input, eio),
            Child::SetupPlantEmsActuators => Err(Error::Unavailable(
                "actual SetupPlantEMSActuators positive EMS child",
            )),
            Child::InitializeOneTimePlantSizing { loop_number: _ } => Err(Error::Unavailable(
                "actual positive InitOneTimePlantSizingInfo child",
            )),
        }
    }
}

impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost, R> SourceZoneAirRegistrationChildren
    for SourceHvacProviderHost<'_, H, D, R>
{
    fn register_zone_list(
        &mut self,
        number: i32,
        name: &str,
        reports: &super::super::ordinary_air_heat_balance::predictor_corrector::SourceZoneListGroupLoadReports,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        self.init_air_output().register_zone_list(
            number,
            name,
            reports,
            clock,
            reporting_diagnostic_flags,
            input,
            backend,
            eio,
        )
    }
    fn register_air(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        air: &SourceZoneSpaceHeatBalanceData,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        self.init_air_output().register_air(
            number,
            space,
            name,
            air,
            clock,
            reporting_diagnostic_flags,
            input,
            backend,
            eio,
        )
    }
    fn register_sensible_demand(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        staged: bool,
        attach_meters: bool,
        zone_multiplier: i32,
        list_multiplier: i32,
        demand: &SourceSystemSensibleDemand,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        self.init_air_output().register_sensible_demand(
            number,
            space,
            name,
            staged,
            attach_meters,
            zone_multiplier,
            list_multiplier,
            demand,
            clock,
            reporting_diagnostic_flags,
            input,
            backend,
            eio,
        )
    }
    fn register_moisture_demand(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        latent_sizing: bool,
        demand: &SourceSystemMoistureDemand,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        self.init_air_output().register_moisture_demand(
            number,
            space,
            name,
            latent_sizing,
            demand,
            clock,
            reporting_diagnostic_flags,
            input,
            backend,
            eio,
        )
    }
    fn register_thermostat(
        &mut self,
        number: i32,
        name: &str,
        fan: &SourceZoneFanHeatBalanceArrays,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        self.init_air_output().register_thermostat(
            number,
            name,
            fan,
            clock,
            reporting_diagnostic_flags,
            input,
            backend,
            eio,
        )
    }
}
