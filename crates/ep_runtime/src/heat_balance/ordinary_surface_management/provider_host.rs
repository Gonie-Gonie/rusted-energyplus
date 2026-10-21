//! Real ManageSurface children over one set of retained numerical/input owners.
//! The mandatory Air child receives these same live owners at its actual call.
use super::{SourceManageSurfaceChildren, SourceSurfaceManagement};
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::{
    SelectedInternalHeatGains, SourceZoneGenericGainAllocation,
};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::facility_electric_service::FacilityElectricService;
use crate::heat_balance::inside_convection_dispatcher::source::SourceInsideConvection;
use crate::heat_balance::ordinary_air_heat_balance::{
    SourceAirHeatBalance, environment::SourceZoneFanHeatBalanceArrays,
    predictor_corrector::SourceZoneAirPredictorCorrector,
};
use crate::heat_balance::ordinary_daylighting::SourceDaylighting;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::ordinary_heat_balance_initialization::{
    SourceHeatBalanceManager, ctf::SourceCtfInitialization,
};
use crate::heat_balance::ordinary_surface_initialization::{
    OrdinarySurfaceInitialization, SourceInitSurfaceOwners, SourceThermalHistoryOwners,
    outside_driver::SourceOutsideCtfOwners, provider_host::SourceOrdinaryInitSurfaceHost,
    report::SourceSurfaceReportOwners,
};
use crate::heat_balance::ordinary_surface_source_averages::SourceFinalSurfaceHeatBalance;
use crate::heat_balance::ordinary_tabular_reports::SourceTabularReports;
use crate::heat_balance::ordinary_thermal_comfort::{
    SourceComfortPredefinedSummary, SourceThermalComfort, SourceThermalComfortOwners,
};
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_gain_modules::SourceGainModules;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, shadowing::SourceShadowingInput,
    surface::SurfaceGeometryInputState, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_material_data::SelectedMaterialData;
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_simulation_startup::SelectedSimulationStartup;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
};
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState, WeatherOwnerState};
use ep_model::ReportInputs;
use std::io::Write;

/// Per-call disjoint references; no constructor, copied numeric state or readiness flag.
pub(crate) struct SourceManageSurfaceOwners<'a, 'backend, H, D> {
    pub(crate) geometry: &'a mut SurfaceGeometryInputState,
    pub(crate) physical_surfaces: &'a mut [SourceSurfaceOutdoorState],
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) control: &'a mut SelectedHeatBalanceInputState,
    pub(crate) radiation: &'a mut OrdinarySolarRadiation,
    pub(crate) air: &'a mut SourceAirHeatBalance,
    pub(crate) history: &'a mut CtfThermalHistoryState,
    pub(crate) ctf: &'a mut SourceCtfInitialization,
    pub(crate) globals: &'a mut SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a mut WeatherGlobalState,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) weather: &'a WeatherOwnerState,
    pub(crate) project: &'a mut SelectedProjectData,
    pub(crate) shadowing: &'a SourceShadowingInput,
    pub(crate) gains: &'a mut SelectedInternalHeatGains,
    pub(crate) gain_modules: &'a mut SourceGainModules,
    pub(crate) facility: Option<&'a mut FacilityElectricService>,
    pub(crate) exterior:
        &'a crate::heat_balance::ordinary_exterior_energy_use::SourceExteriorEnergyUse,
    pub(crate) schedules: &'a mut SelectedScheduleManager,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) source_is_epjson: &'a bool,
    pub(crate) faults: &'a crate::heat_balance::source_initialization::SourceFaultsManager,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) output_control:
        &'a crate::heat_balance::ordinary_output_control::SourceOutputControl,
    pub(crate) tabular_raw: &'a crate::heat_balance::ordinary_tabular_reports::SelectedTabularInput,
    pub(crate) sizing_raw: &'a crate::heat_balance::selected_sizing::SelectedSizingInput,
    pub(crate) results:
        &'a mut crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions,
    pub(crate) standard_output_record_count: &'a mut i64,
    pub(crate) standard_meter_record_count: &'a mut i64,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) output_host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) debug: &'a mut dyn Write,
    pub(crate) materials: &'a SelectedMaterialData,
    pub(crate) predefined_registry: &'a mut PredefinedRegistry,
    pub(crate) predefined_table: &'a mut PredefinedStringTable,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) recurring_errors:
        &'a mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
    pub(crate) fan: &'a mut SourceZoneFanHeatBalanceArrays,
    pub(crate) equipment: &'a mut OrdinaryEquipment,
    pub(crate) sizing: &'a mut crate::heat_balance::selected_sizing::SelectedSizingManager,
    pub(crate) convection: &'a mut SourceInsideConvection,
    pub(crate) daylighting: &'a mut SourceDaylighting,
    pub(crate) startup: &'a mut SelectedSimulationStartup,
    pub(crate) space_controls: &'a SelectedHeatBalanceSpaceControls,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
    pub(crate) comfort: &'a mut SourceThermalComfort,
    pub(crate) comfort_summary: &'a mut SourceComfortPredefinedSummary,
    pub(crate) heat_balance: &'a mut SourceHeatBalanceManager,
    pub(crate) tabular: &'a mut SourceTabularReports,
    pub(crate) final_surface: &'a mut SourceFinalSurfaceHeatBalance,
    pub(crate) plugin_data: &'a mut crate::heat_balance::ordinary_ems_plugin::SourcePluginData,
    pub(crate) report_flags:
        &'a mut crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags,
    pub(crate) economic_tariff:
        &'a mut crate::heat_balance::ordinary_utility_bills::SourceEconomicTariff,
    pub(crate) cost_estimate_monetary:
        &'a mut crate::heat_balance::ordinary_utility_bills::SourceCostEstimateMonetary,
}

/// No successful default: an actual ManageAir/HVAC provider must be supplied.
/// Surface arrays remain a single owner, reborrowed only during this child call.
pub(crate) trait SourceManageSurfaceAirChild<H, D> {
    fn manage_air_heat_balance(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceManageSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error>;
}
pub(crate) struct SourceOrdinaryManageSurfaceHost<'a, 'owners, 'backend, H, D, A> {
    pub(crate) surface: &'a mut OrdinarySurfaceInitialization,
    pub(crate) controller: &'a mut SourceZoneAirPredictorCorrector,
    pub(crate) owners: &'a mut SourceManageSurfaceOwners<'owners, 'backend, H, D>,
    pub(crate) air_child: &'a mut A,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost, A: SourceManageSurfaceAirChild<H, D>>
    SourceManageSurfaceChildren for SourceOrdinaryManageSurfaceHost<'_, '_, '_, H, D, A>
{
    fn control(&self) -> &SelectedHeatBalanceInputState {
        self.owners.control
    }
    fn globals(&self) -> &SelectedHeatBalanceGlobals {
        self.owners.globals
    }
    fn tabular(&self) -> &SourceTabularReports {
        self.owners.tabular
    }
    fn display_string(&mut self, message: &str) -> Result<(), Error> {
        self.owners
            .backend
            .display_string(message)
            .map_err(Error::from)
    }
    fn initialize_surface_heat_balance(&mut self) -> Result<(), Error> {
        let cadence_scope = crate::heat_balance::source_surface_cadence::enter(
            crate::heat_balance::source_surface_cadence::Procedure::Initialize,
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.globals.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
        );
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            let o = &mut self.owners;
            let mut providers = SourceOrdinaryInitSurfaceHost {
                fan_zone: o.fan,
                daylighting: o.daylighting,
                convection: o.convection,
                equipment: o.equipment,
                psychrometrics: o.psychrometrics,
                recurring: o.recurring_errors,
                hvac: &o.startup.hvac_globals,
            };
            self.surface
                .initialize_source_surface_heat_balance(
                    &mut SourceInitSurfaceOwners {
                        geometry: o.geometry,
                        physical_surfaces: o.physical_surfaces,
                        zones: o.zones,
                        constructions: o.control,
                        radiation: o.radiation,
                        air: o.air,
                        room_air: &self.controller.subsystems.room_air,
                        history: o.history,
                        public_ctf: &o.ctf.public,
                        globals: o.globals,
                        clock: o.clock,
                        environment: o.environment,
                        project: o.project,
                        shadowing: o.shadowing,
                        gains: o.gains,
                        gain_modules: o.gain_modules,
                        facility: o.facility.as_deref_mut(),
                        schedules: o.schedules,
                        input: o.input,
                        shortcut: o.shortcut,
                        scan: o.scan,
                        report_inputs: o.report_inputs,
                        registration: o.registration,
                        prescan: o.prescan,
                        minimum_report_frequency: o.minimum_report_frequency,
                        output_host: o.output_host,
                        dictionary_host: o.dictionary_host,
                        eio: o.eio,
                        debug: o.debug,
                        materials: o.materials,
                        predefined_registry: o.predefined_registry,
                        predefined_table: o.predefined_table,
                        backend: o.backend,
                    },
                    SourceZoneGenericGainAllocation::Unavailable,
                    &mut providers,
                )
                .map_err(Error::from)
        })();
        cadence_scope.finish(
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.globals.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
            cadence_result.is_ok(),
        );
        if cadence_result.is_ok() {
            crate::heat_balance::source_surface_cadence::record(
                "actual_InitSurfaceHeatBalance_return",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self.surface,
                    self.owners.clock,
                    self.owners.globals,
                    self.owners.environment,
                    self.owners.globals.kickoff_simulation,
                    self.surface.inside_surface_iterations,
                ),
            );
        }
        cadence_result
    }
    fn calculate_outside_surface_heat_balance(&mut self) -> Result<(), Error> {
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            let o = &mut self.owners;
            self.surface
                .calculate_source_outside_surface_balance(&mut SourceOutsideCtfOwners {
                    geometry: o.geometry,
                    zones: o.zones,
                    air: o.air,
                    control: o.control,
                    public: &o.ctf.public,
                    history: o.history,
                    radiation: o.radiation,
                    physical: o.physical_surfaces,
                    globals: o.globals,
                    clock: o.clock,
                    environment: o.environment,
                    convection: o.convection,
                    input: o.input,
                    reports: o.report_inputs,
                    scan: o.scan,
                    carroll_method: o.project.carroll_method,
                    backend: o.backend,
                })
        })();
        if cadence_result.is_ok() {
            crate::heat_balance::source_surface_cadence::record(
                "actual_outside_surface_return",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self.surface,
                    self.owners.clock,
                    self.owners.globals,
                    self.owners.environment,
                    self.owners.globals.kickoff_simulation,
                    self.surface.inside_surface_iterations,
                ),
            );
        }
        cadence_result
    }
    // Keep the explicit same-owner reborrow at this scientific callback boundary.
    #[allow(clippy::needless_borrow)]
    fn calculate_inside_surface_heat_balance(&mut self) -> Result<(), Error> {
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            self.surface.calculate_source_inside_surface_heat_balance(
                &mut self.owners,
                &self.controller.subsystems.airflow_network.control_type,
            )
        })();
        if cadence_result.is_ok() {
            crate::heat_balance::source_surface_cadence::record(
                "actual_inside_surface_return",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self.surface,
                    self.owners.clock,
                    self.owners.globals,
                    self.owners.environment,
                    self.owners.globals.kickoff_simulation,
                    self.surface.inside_surface_iterations,
                ),
            );
        }
        cadence_result
    }
    // Keep the explicit same-owner reborrow at this scientific callback boundary.
    #[allow(clippy::needless_borrow)]
    fn manage_air_heat_balance(&mut self) -> Result<(), Error> {
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            self.air_child
                .manage_air_heat_balance(self.surface, self.controller, &mut self.owners)
        })();
        if cadence_result.is_ok() {
            crate::heat_balance::source_surface_cadence::record(
                "actual_air_heat_balance_return",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self.surface,
                    self.owners.clock,
                    self.owners.globals,
                    self.owners.environment,
                    self.owners.globals.kickoff_simulation,
                    self.surface.inside_surface_iterations,
                ),
            );
        }
        cadence_result
    }
    fn update_final_surface_heat_balance(&mut self) -> Result<(), Error> {
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            self.owners
                .final_surface
                .update_final_surface_heat_balance()
        })();
        if cadence_result.is_ok() {
            crate::heat_balance::source_surface_cadence::record(
                "actual_final_surface_update_return",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self.surface,
                    self.owners.clock,
                    self.owners.globals,
                    self.owners.environment,
                    self.owners.globals.kickoff_simulation,
                    self.surface.inside_surface_iterations,
                ),
            );
        }
        cadence_result
    }
    fn update_thermal_histories(&mut self) -> Result<(), Error> {
        let cadence_scope = crate::heat_balance::source_surface_cadence::enter(
            crate::heat_balance::source_surface_cadence::Procedure::UpdateHistories,
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.globals.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
        );
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            let o = &mut self.owners;
            self.surface
                .update_source_thermal_histories(&mut SourceThermalHistoryOwners {
                    geometry: o.geometry,
                    zones: o.zones,
                    control: o.control,
                    ctf: o.ctf,
                    globals: o.globals,
                    clock: o.clock,
                    history: o.history,
                    physical_surfaces: o.physical_surfaces,
                })
                .map(|_| ())
        })();
        cadence_scope.finish(
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.globals.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
            cadence_result.is_ok(),
        );
        if cadence_result.is_ok() {
            crate::heat_balance::source_surface_cadence::record(
                "actual_UpdateThermalHistories_return",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self.surface,
                    self.owners.clock,
                    self.owners.globals,
                    self.owners.environment,
                    self.owners.globals.kickoff_simulation,
                    self.surface.inside_surface_iterations,
                ),
            );
        }
        cadence_result
    }
    fn update_condfd_moisture_balance(&mut self) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive ManageSurface CondFD surface moisture traversal",
        ))
    }
    fn manage_thermal_comfort(&mut self, initialize_only: bool) -> Result<(), Error> {
        let o = &mut self.owners;
        o.comfort.manage_source_thermal_comfort(
            initialize_only,
            &mut SourceThermalComfortOwners {
                gains: o.gains,
                schedules: o.schedules,
                developer: &o.project.system.developer_flag,
                zones: o.zones,
                control: o.control,
                globals: o.globals,
                clock: o.clock,
                environment: o.environment,
                hvac: &o.startup.hvac_globals,
                air: o.air,
                controller: self.controller,
                fan: o.fan,
                radiation: o.radiation,
                geometry: o.geometry,
                history: o.history,
                psychrometrics: o.psychrometrics,
                recurring_errors: o.recurring_errors,
                input: o.input,
                source_is_epjson: o.source_is_epjson,
                shortcut: o.shortcut,
                registration: o.registration,
                prescan: o.prescan,
                report_inputs: o.report_inputs,
                minimum_report_frequency: o.minimum_report_frequency,
                host: o.output_host,
                dictionary_host: o.dictionary_host,
                eio: o.eio,
                backend: o.backend,
                registry: o.predefined_registry,
                table: o.predefined_table,
                predefined_summary: o.comfort_summary,
            },
        )
    }
    fn report_surface_heat_balance(&mut self) -> Result<(), Error> {
        let cadence_scope = crate::heat_balance::source_surface_cadence::enter(
            crate::heat_balance::source_surface_cadence::Procedure::Report,
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.globals.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
        );
        // Keep the once-invoked observer closure: local Result propagation must not bypass the after-call exit observation.
        #[allow(clippy::redundant_closure_call)]
        let cadence_result = (|| {
            let o = &mut self.owners;
            self.surface
                .report_source_surface_heat_balance(&mut SourceSurfaceReportOwners {
                    geometry: o.geometry,
                    zones: o.zones,
                    radiation: o.radiation,
                    history: o.history,
                    physical: o.physical_surfaces,
                    globals: o.globals,
                    clock: o.clock,
                    environment: o.environment,
                    predefined: o.predefined_registry,
                    display_heat_emissions_summary: &o.tabular.display_heat_emissions_summary,
                })
        })();
        cadence_scope.finish(
            crate::heat_balance::source_surface_cadence::Frame::from_owners(
                self.surface,
                self.owners.clock,
                self.owners.globals,
                self.owners.environment,
                self.owners.globals.kickoff_simulation,
                self.surface.inside_surface_iterations,
            ),
            cadence_result.is_ok(),
        );
        if cadence_result.is_ok() {
            crate::heat_balance::source_surface_cadence::record(
                "actual_ReportSurfaceHeatBalance_return",
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self.surface,
                    self.owners.clock,
                    self.owners.globals,
                    self.owners.environment,
                    self.owners.globals.kickoff_simulation,
                    self.surface.inside_surface_iterations,
                ),
            );
        }
        cadence_result
    }
    fn gather_component_loads_surface(&mut self) -> Result<(), Error> {
        Err(Error::Unavailable(
            "actual positive ManageSurface sizing GatherComponentLoadsSurface child",
        ))
    }
    fn calculate_thermal_resilience(
        &mut self,
        state: &mut SourceSurfaceManagement,
    ) -> Result<(), Error> {
        let o = &mut self.owners;
        state.calculate_source_thermal_resilience(&mut super::SourceResilienceOwners {
            zones: o.zones,
            globals: o.globals,
            control: o.control,
            tabular: o.tabular,
            air: o.air,
            psychrometrics: o.psychrometrics,
            environment: o.environment,
            clock: o.clock,
            input: o.input,
            registration: o.registration,
            prescan: o.prescan,
            report_inputs: o.report_inputs,
            minimum_report_frequency: o.minimum_report_frequency,
            host: o.output_host,
            dictionary_host: o.dictionary_host,
            eio: o.eio,
            backend: o.backend,
        })
    }
    fn report_thermal_resilience(
        &mut self,
        state: &mut SourceSurfaceManagement,
    ) -> Result<(), Error> {
        state.report_source_thermal_resilience(&mut self.resilience_report_owners())
    }
    fn report_co2_resilience(&mut self, state: &mut SourceSurfaceManagement) -> Result<(), Error> {
        state.report_source_co2_resilience(&mut self.resilience_report_owners())
    }
    fn report_visual_resilience(
        &mut self,
        state: &mut SourceSurfaceManagement,
    ) -> Result<(), Error> {
        state.report_source_visual_resilience(&mut self.resilience_report_owners())
    }
}
impl<'backend, H, D, A> SourceOrdinaryManageSurfaceHost<'_, '_, 'backend, H, D, A> {
    fn resilience_report_owners(&mut self) -> super::SourceResilienceReportOwners<'_, 'backend> {
        let o = &mut self.owners;
        super::SourceResilienceReportOwners {
            globals: o.globals,
            zones: o.zones,
            air: o.air,
            fan: o.fan,
            clock: o.clock,
            weather: o.weather,
            environment: o.environment,
            control: o.control,
            gains: o.gains,
            schedules: o.schedules,
            comfort: o.comfort,
            heat_balance: o.heat_balance,
            daylighting: o.daylighting,
            tabular: o.tabular,
            backend: o.backend,
        }
    }
}
