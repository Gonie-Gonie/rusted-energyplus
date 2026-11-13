//! Authored Source-only Scheduled regressions. Not Native numerical comparisons or RawModel ordering evidence.
use super::*;
// Candidate child of native_gap_cohort. No executed-proof claim.
// F must call the actual manage_source_air_loops body; never supply a successful no-op.
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::SourceZoneSpaceHeatBalanceData;
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::{
    SourceSystemMoistureDemand, SourceSystemSensibleDemand,
};
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::{
    SourceZoneAirRegistrationChildren, SourceZoneListGroupLoadReports,
};
use crate::heat_balance::ordinary_hvac_manager::simulation::{
    SourceHvacSimulationChild, SourceHvacSimulationChildren,
};
use crate::heat_balance::ordinary_hvac_manager::{
    SourceHvacChild, SourceHvacChildren, SourceHvacManager, SourceHvacOwners,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::WeatherGlobalState;
const PREFIX_STOP: &str =
    "Scheduled unit prefix: actual air-loop returned; stop at next zone child";
struct ScheduledReadyPrefix<F> {
    actual_air_loop: F,
    air_calls: usize,
    stopped: bool,
}
#[allow(unused_variables)]
impl<F> SourceZoneAirRegistrationChildren for ScheduledReadyPrefix<F> {
    fn register_zone_list(
        &mut self,
        number: i32,
        name: &str,
        reports: &SourceZoneListGroupLoadReports,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> std::result::Result<(), Error> {
        Err(Error::Unavailable(
            "Scheduled unit prefix: unexpected registration child",
        ))
    }
    fn register_zone_group(
        &mut self,
        number: i32,
        name: &str,
        reports: &SourceZoneListGroupLoadReports,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> std::result::Result<(), Error> {
        Err(Error::Unavailable(
            "Scheduled unit prefix: unexpected registration child",
        ))
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
    ) -> std::result::Result<(), Error> {
        Err(Error::Unavailable(
            "Scheduled unit prefix: unexpected registration child",
        ))
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
    ) -> std::result::Result<(), Error> {
        Err(Error::Unavailable(
            "Scheduled unit prefix: unexpected registration child",
        ))
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
    ) -> std::result::Result<(), Error> {
        Err(Error::Unavailable(
            "Scheduled unit prefix: unexpected registration child",
        ))
    }
    fn register_thermostat(
        &mut self,
        number: i32,
        name: &str,
        fan: &crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> std::result::Result<(), Error> {
        Err(Error::Unavailable(
            "Scheduled unit prefix: unexpected registration child",
        ))
    }
}
#[allow(unused_variables)]
impl<F> SourceHvacChildren for ScheduledReadyPrefix<F> {
    fn manage_electric_facility(
        &mut self,
        first_hvac_iteration: bool,
        simulate_electrical_circuits: &mut bool,
        update_meters_only: bool,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn std::io::Write,
    ) -> std::result::Result<(), Error> {
        Err(Error::Unavailable(
            "Scheduled unit prefix: unexpected facility child",
        ))
    }
    fn run(
        &mut self,
        child: SourceHvacChild,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn std::io::Write,
    ) -> std::result::Result<(), Error> {
        Err(Error::Unavailable(
            "Scheduled unit prefix: unexpected HVAC child",
        ))
    }
}
impl<F> SourceHvacSimulationChildren for ScheduledReadyPrefix<F>
where
    F: FnMut(
        bool,
        &mut SourceHvacManager,
        &mut SourceHvacOwners<'_, '_>,
        &mut dyn std::io::Write,
    ) -> std::result::Result<(), Error>,
{
    fn run_simulation(
        &mut self,
        child: SourceHvacSimulationChild,
        manager: &mut SourceHvacManager,
        owners: &mut SourceHvacOwners<'_, '_>,
        eio: &mut dyn std::io::Write,
    ) -> std::result::Result<(), Error> {
        match child {
            SourceHvacSimulationChild::ManageAirLoops {
                first_hvac_iteration,
            } if self.air_calls == 0 => {
                (self.actual_air_loop)(first_hvac_iteration, manager, owners, eio)?;
                self.air_calls += 1;
                Ok(())
            }
            SourceHvacSimulationChild::ManageZoneEquipment { .. } if self.air_calls == 1 => {
                self.stopped = true;
                Err(Error::Unavailable(PREFIX_STOP))
            }
            _ => Err(Error::Unavailable(
                "Scheduled unit prefix: unexpected simulation child",
            )),
        }
    }
}
// Caller: set only authored clock.begin_envrn_flag=false. Assert readiness starts false.
// Call manager.simulate_source_selected_equipment(true,false,&mut owners,&mut prefix,eio).
// Require Err(Error::Unavailable(PREFIX_STOP)), air_calls==1, stopped=true,
// and manager.simulation.air_loop.inputs_filled==true. Do not assign that field.

impl Factory {
    /// Unit clock operands feed the actual schedule producer before any manager read.
    /// Original phase_inputs ends before caller_inputs; only the latter updates schedules.
    /// This Source-only fixture intentionally does not invoke unrelated load/node caller_inputs.
    fn scheduled_phase_inputs(&mut self, operands: &Value) -> Result {
        self.phase_inputs(operands)?;
        self.schedules
            .update_source_schedule_values(&self.weather.global, &self.weather.environment)?;
        Ok(())
    }
    // Insert as additional methods inside Factory impl, not replacement of existing methods.
    // Input is explicitly authored epJSON-shaped unit operands, NOT a RawModel producer result.
    // The caller must strip captured IDF transport metadata before supplying definition;
    // preserve old Factory::new/new_with_console_output and its captured fixtures untouched.
    // This copied setup uses real Zone/Schedule/Equipment/Node getters; no physical values are seeded.
    fn new_authored_epjson_operands(
        definitions: &Value,
        definition: &Value,
        print_console_output: bool,
    ) -> Result<Self> {
        let input = make_input(definitions, definition)?;
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input)?;
        let control = SelectedHeatBalanceInputState::default();
        let (report, raw_prescan) = report_inputs(definition)?;
        let inventory = object(member(definition, "raw")?)?
            .iter()
            .map(|(f, v)| Ok((f.as_str(), object(v)?.len())))
            .collect::<Result<Vec<_>>>()?;
        let radiation_input =
            hb::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory(
                inventory.clone(),
                false,
            )?;
        let mut input = input;
        let mut prescan = SourceOutputPreScan::default();
        let mut state = error_state();
        // DataGlobals.hh177 real Native constructor; EH selects true before any getter.
        // Existing P1 factories explicitly retain their original false caller setting.
        state.print_console_output = print_console_output;
        let mut error = Vec::new();
        let mut console = Vec::new();
        let mut project = SelectedProjectData::default();
        let mut weather = crate::weather::day::WeatherDayState::default();
        let mut globals = SelectedHeatBalanceGlobals::default();
        let mut startup = SelectedSimulationStartup::default();
        // Genuine init_state -> OpenOutputFiles precedes GetProjectData.
        // The cold version string is actual owner input, not expected output.
        let output_header = format!("Program Version,{}\n", state.version_string);
        let mut eio = output_header.as_bytes().to_vec();
        let eso = output_header.into_bytes();
        let mut fluid = hb::selected_fluid_properties::SelectedFluidProperties::default();
        fluid.init_constant_state()?;
        let mut schedules = SelectedScheduleManager::default();
        schedules.init_constant_state(0)?;
        let mut scan = hb::report_scan::ReportScanState::default();
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            hb::source_output_prescan::post_input_processing(
                &input,
                &mut shortcut,
                &raw_prescan,
                &mut prescan,
                false,
                &mut backend,
            )?;
            let raw = SelectedProjectDataInput {
                source_is_epjson: true,
                objects: project_objects(
                    definition,
                    &[
                        "Version",
                        "HeatBalanceAlgorithm",
                        "Timestep",
                        "SimulationControl",
                    ],
                )?,
                diagnostics: Vec::new(),
            };
            let mut current_module = String::new();
            project.get_project_data(
                &raw,
                &mut ProjectDataOwners {
                    input: &mut input,
                    shortcut: &mut shortcut,
                    globals: &mut globals,
                    weather: &mut weather,
                    hvac: &mut startup.hvac_globals,
                    heat_balance_current_module_object: &mut current_module,
                },
                &mut backend,
                &mut eio,
            )?;
            startup.check_if_any_ems(
                &mut input,
                &mut shortcut,
                &mut globals,
                &report,
                &mut scan,
                false,
                false,
                0,
                &mut backend,
            )?;
        }
        fluid.get_fluid_properties_data(&mut input)?;
        let steps = usize::try_from(weather.global.time_steps_in_hour)?;
        let mut files = Files::default();
        schedules.process_schedule_input(
            &mut input,
            &schedule_inputs(definition)?,
            steps,
            &mut files,
        )?;
        let model = identity_model(definition, &control, weather.global.time_steps_in_hour)?;
        let mut registration = SourceOutputRegistration::default();
        let mut zones = SelectedZoneInputOwners::default();
        let raw = SelectedHeatBalanceInput {
            source_is_epjson: true,
            surface: Default::default(),
            objects: project_objects(definition, &["Zone"])?,
        };
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            let mut zone_errors_found = false;
            zones.get_zone_data(
                &raw,
                &control,
                &model,
                &mut ZoneGetterOwners {
                    input: &mut input,
                    shortcut: &mut shortcut,
                    globals: &mut globals,
                    output: &mut registration,
                    prescan: &prescan,
                    report_input: &report,
                    minimum_report_frequency: &project.system.minimum_report_frequency,
                    developer: &project.system.developer_flag,
                    clock: Some(&weather.global),
                    backend: &mut backend,
                    minutes_in_time_step: weather.global.minutes_in_time_step,
                    host: &mut files,
                    eio: &mut eio,
                },
                &mut zone_errors_found,
            )?;
        }
        let admitted = OrdinaryEquipmentInput::from_source_family_inventory(inventory, steps)?;
        let mut equipment = OrdinaryEquipment::new_for_source(&model, admitted)?;
        let mut ancillary = hb::ctf_initialization_context::CtfInitializationReports::default();
        let mut manager = SourceHvacManager::default();
        let mut sizing = hb::selected_sizing::SelectedSizingManager::default();
        equipment.get_zone_equipment_source(
            &mut input,
            admitted,
            false,
            false,
            &globals,
            &mut ancillary.selected_internal_heat_gains,
            hb::ordinary_equipment::SourceEquipmentInputOwners {
                zones: zones
                    .zones
                    .as_deref_mut()
                    .ok_or_else(|| invalid("getter Zone rows"))?,
                schedules: &mut schedules,
                sizing: &mut sizing,
                branch: Some(&mut manager.ancillary.branch_node_connections),
                source_is_epjson: true,
            },
        )?;
        let mut selected = None;
        for (z, c) in equipment
            .zone_config
            .as_deref()
            .ok_or_else(|| invalid("actual Config allocation"))?
            .iter()
            .enumerate()
        {
            if !c.is_controlled {
                continue;
            }
            let l = c.equipment_list_index;
            let row = equipment
                .equipment_lists
                .as_deref()
                .and_then(|r| l.checked_sub(1).and_then(|i| r.get(i)))
                .ok_or_else(|| invalid("actual List index"))?;
            for p in 0..row.number_of_equipment_types {
                if row.equipment_types.as_deref().and_then(|v| v.get(p)) == Some(&12) {
                    if selected.is_some() {
                        return Err(invalid("unique actual PurchasedAir component").into());
                    }
                    selected = Some((
                        i32::try_from(z + 1)?,
                        i32::try_from(l)?,
                        i32::try_from(p + 1)?,
                    ));
                }
            }
        }
        let (zone, list, pointer) = selected.ok_or_else(|| invalid("real PurchasedAir binding"))?;
        let space_controls = SelectedHeatBalanceSpaceControls::default();
        let mut controller = SourceZoneAirPredictorCorrector::default();
        controller
            .demand
            .allocate_source_demands(&globals, &space_controls)?;
        let n = usize::try_from(globals.num_of_zones)?;
        let mut air = SourceAirHeatBalance::default();
        air.zones = Some(vec![SourceZoneHeatBalanceData::default(); n]);
        let mut fan = SourceZoneFanHeatBalanceArrays::default();
        fan.thermostat.zone_tstat_setpoints = Some((0..n).map(|_| Default::default()).collect());
        fan.thermostat.temp_control_type = Some(vec![0; n]);
        let mut out = Self {
            equipment,
            input,
            shortcut,
            globals,
            zones,
            schedules,
            sizing,
            startup,
            hvac_manager: manager,
            air,
            controller,
            fan,
            weather,
            project,
            control,
            space_controls,
            report,
            prescan,
            registration,
            registry: Default::default(),
            files,
            eio,
            eso,
            rf: Vec::new(),
            error_state: state,
            error,
            console,
            psych: Default::default(),
            ancillary,
            surface: Default::default(),
            radiation: hb::radiation::ordinary::OrdinarySolarRadiation::for_surface_input(
                radiation_input,
            ),
            history: hb::ctf_thermal_history::CtfThermalHistoryState::new_source_unallocated(
                Vec::new(),
            ),
            gain_modules: Default::default(),
            contaminants: Default::default(),
            tabular: Default::default(),
            comfort: Default::default(),
            comfort_summary: Default::default(),
            final_surface: Default::default(),
            exterior: Default::default(),
            zone,
            list,
            pointer,
            sensible: 0.0,
            moisture: 0.0,
        };
        out.initialize_authored_epjson_prefix(true, true)?;
        out.registry
            .set_predefined_tables(out.globals.do_zone_sizing, out.globals.do_system_sizing)?;
        out.sizing.current_zone_equipment_number = zone;
        Ok(out)
    }
    fn initialize_authored_epjson_prefix(
        &mut self,
        first: bool,
        availability_input: bool,
    ) -> Result {
        let mut backend = SourceErrorBackend {
            state: &mut self.error_state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let source_is_epjson = true;
        let mut owners = SourceHvacOwners {
            runtime_language: &self.startup.runtime_language,
            air: &mut self.air,
            controller: &mut self.controller,
            zones: &mut self.zones,
            sizing: &mut self.sizing,
            equipment: &mut self.equipment,
            geometry: &self.ancillary.surface_geometry_input,
            surface_arrays: &mut self.surface.arrays,
            fan: &mut self.fan,
            gains: &mut self.ancillary.selected_internal_heat_gains,
            control: &mut self.control,
            space_controls: &self.space_controls,
            globals: &mut self.globals,
            clock: &mut self.weather.global,
            environment: &mut self.weather.environment,
            hvac: &mut self.startup.hvac_globals,
            input: &mut self.input,
            shortcut: &mut self.shortcut,
            source_is_epjson: &source_is_epjson,
            faults: &self.ancillary.source_faults_manager,
            psychrometrics: &mut self.psych,
            backend: &mut backend,
            recurring_errors: &mut self.ancillary.source_recurring_errors,
            plant_loops: &mut self.startup.plant_loops,
            project: &mut self.project,
            contaminants: &mut self.contaminants,
            gain_modules: &mut self.gain_modules,
            facility: None,
            schedules: &mut self.schedules,
            tabular: &mut self.tabular,
            radiation: &mut self.radiation,
            history: &mut self.history,
            physical_surfaces: &mut [],
            comfort: &mut self.comfort,
            comfort_summary: &mut self.comfort_summary,
            predefined_registry: &mut self.registry,
            predefined_table: &mut self.ancillary.table,
            weather: &self.weather.weather,
            inside_surface_iterations: &self.surface.inside_surface_iterations,
            surface_heat_emission: &self.surface.sum_surface_heat_emission,
            surface_averages: &self.final_surface,
            exterior: &self.exterior,
        };
        if availability_input {
            self.hvac_manager
                .manage_source_system_availability(&mut owners)?;
        }
        self.hvac_manager
            .initialize_source_zone_equipment_cohort_bridge(first, &mut owners)?;
        Ok(())
    }
    fn scheduled_step(&mut self, ready_prefix: bool) -> Result {
        let mut backend = SourceErrorBackend {
            state: &mut self.error_state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let source_is_epjson = true;
        let mut owners = SourceHvacOwners {
            runtime_language: &self.startup.runtime_language,
            air: &mut self.air,
            controller: &mut self.controller,
            zones: &mut self.zones,
            sizing: &mut self.sizing,
            equipment: &mut self.equipment,
            geometry: &self.ancillary.surface_geometry_input,
            surface_arrays: &mut self.surface.arrays,
            fan: &mut self.fan,
            gains: &mut self.ancillary.selected_internal_heat_gains,
            control: &mut self.control,
            space_controls: &self.space_controls,
            globals: &mut self.globals,
            clock: &mut self.weather.global,
            environment: &mut self.weather.environment,
            hvac: &mut self.startup.hvac_globals,
            input: &mut self.input,
            shortcut: &mut self.shortcut,
            source_is_epjson: &source_is_epjson,
            faults: &self.ancillary.source_faults_manager,
            psychrometrics: &mut self.psych,
            backend: &mut backend,
            recurring_errors: &mut self.ancillary.source_recurring_errors,
            plant_loops: &mut self.startup.plant_loops,
            project: &mut self.project,
            contaminants: &mut self.contaminants,
            gain_modules: &mut self.gain_modules,
            facility: None,
            schedules: &mut self.schedules,
            tabular: &mut self.tabular,
            radiation: &mut self.radiation,
            history: &mut self.history,
            physical_surfaces: &mut [],
            comfort: &mut self.comfort,
            comfort_summary: &mut self.comfort_summary,
            predefined_registry: &mut self.registry,
            predefined_table: &mut self.ancillary.table,
            weather: &self.weather.weather,
            inside_surface_iterations: &self.surface.inside_surface_iterations,
            surface_heat_emission: &self.surface.sum_surface_heat_emission,
            surface_averages: &self.final_surface,
            exterior: &self.exterior,
        };

        if !ready_prefix {
            self.hvac_manager
                .manage_source_scheduled_owner_test(&mut owners)?;
            return Ok(());
        }
        assert!(!self.hvac_manager.simulation.air_loop.inputs_filled);
        owners.clock.begin_envrn_flag = false; // explicit unit clock operand
        let mut dictionary = SourceOutputDictionaryStreams {
            eso: Some(&mut self.eso),
            sqlite_present: false,
            results_framework_output_variables: &mut self.rf,
        };
        let mut prefix = ScheduledReadyPrefix {
            air_calls: 0,
            stopped: false,
            actual_air_loop: |first,
                              manager: &mut SourceHvacManager,
                              o: &mut SourceHvacOwners<'_, '_>,
                              eio: &mut dyn std::io::Write| {
                use hb::ordinary_air_heat_balance::predictor_corrector::simple_air::SourceAirReportRegistrationContext;
                use hb::ordinary_hvac_manager::simulation::air_loops::SourceAirLoopOwners;
                let mut output = SourceAirReportRegistrationContext {
                    registration: &mut self.registration,
                    prescan: &self.prescan,
                    reports: &self.report,
                    minimum_report_frequency: &o.project.system.minimum_report_frequency.clone(),
                    minutes_in_time_step: o.clock.minutes_in_time_step,
                    host: &mut self.files,
                    eio,
                    dictionary_host: &mut dictionary,
                };
                let mut ao = SourceAirLoopOwners {
                    input: o.input,
                    backend: o.backend,
                    globals: o.globals,
                    clock: o.clock,
                    hvac: o.hvac,
                    project: o.project,
                    equipment: o.equipment,
                    sizing: o.sizing,
                    distribution_simulated: &o
                        .controller
                        .subsystems
                        .airflow_network
                        .distribution_simulated,
                };
                manager
                    .simulation
                    .air_loop
                    .manage_source_air_loops(first, &mut ao, &mut output)
            },
        };
        let result = self.hvac_manager.simulate_source_selected_equipment(
            true,
            false,
            &mut owners,
            &mut prefix,
            &mut self.eio,
        );
        assert!(matches!(result, Err(Error::Unavailable(PREFIX_STOP))));
        assert_eq!(prefix.air_calls, 1);
        assert!(prefix.stopped);
        assert!(self.hvac_manager.simulation.air_loop.inputs_filled);
        Ok(())
    }
    fn scheduled_setpoint_fields(&self, number: Option<usize>) -> Result<[u64; 9]> {
        let row = match number {
            Some(number) => self
                .equipment
                .nodes
                .physical
                .as_ref()
                .and_then(|rows| number.checked_sub(1).and_then(|at| rows.get(at)))
                .ok_or_else(|| invalid("actual Scheduled Node canaries"))?,
            None => &self.equipment.nodes.default_node_values,
        };
        Ok([
            row.temperature_setpoint.to_bits(),
            row.temperature_setpoint_high.to_bits(),
            row.temperature_setpoint_low.to_bits(),
            row.humidity_ratio_setpoint.to_bits(),
            row.humidity_ratio_max.to_bits(),
            row.humidity_ratio_min.to_bits(),
            row.mass_flow_rate_setpoint.to_bits(),
            row.mass_flow_rate_max.to_bits(),
            row.mass_flow_rate_min.to_bits(),
        ])
    }
    fn scheduled_value(&self, number: usize, control: usize, expected: f64) -> Result {
        use hb::source_output_registration::{
            SourceNodeOutputField as Field, SourceRealOutputTarget,
        };
        let row = self
            .equipment
            .nodes
            .physical
            .as_ref()
            .and_then(|v| v.get(number - 1))
            .ok_or_else(|| invalid("actual Scheduled Node"))?;

        let fields = [
            &row.temperature_setpoint,
            &row.temperature_setpoint_high,
            &row.temperature_setpoint_low,
            &row.humidity_ratio_setpoint,
            &row.humidity_ratio_max,
            &row.humidity_ratio_min,
            &row.mass_flow_rate_setpoint,
            &row.mass_flow_rate_max,
            &row.mass_flow_rate_min,
        ];
        let targets = [
            Field::TemperatureSetpoint,
            Field::TemperatureSetpointHigh,
            Field::TemperatureSetpointLow,
            Field::HumidityRatioSetpoint,
            Field::HumidityRatioMax,
            Field::HumidityRatioMin,
            Field::MassFlowRateSetpoint,
            Field::MassFlowRateMax,
            Field::MassFlowRateMin,
        ];
        let selected = *fields
            .get(control)
            .ok_or_else(|| invalid("actual Scheduled selector"))?;
        assert_eq!(selected.to_bits(), expected.to_bits());
        let value = self.equipment.real_output(SourceRealOutputTarget::Node {
            field: *targets
                .get(control)
                .ok_or_else(|| invalid("actual Scheduled output field"))?,
            index: i32::try_from(number)?,
        })?;
        assert!(std::ptr::eq(value, selected));

        Ok(())
    }
    fn scheduled_selected_step(&mut self, number: usize, control: usize, expected: f64) -> Result {
        let before = self.scheduled_setpoint_fields(Some(number))?;
        self.scheduled_step(false)?;
        self.scheduled_value(number, control, expected)?;
        let after = self.scheduled_setpoint_fields(Some(number))?;
        for (field, (actual, prior)) in after.iter().zip(before.iter()).enumerate() {
            if field != control {
                assert_eq!(actual, prior, "unselected Node field {field}");
            }
        }
        Ok(())
    }
}
fn strip_idf(v: &mut Value) {
    match v {
        Value::Object(m) => {
            m.retain(|k, _| !k.starts_with("idf_"));
            for item in m.values_mut() {
                strip_idf(item);
            }
        }
        Value::Array(a) => {
            for item in a {
                strip_idf(item);
            }
        }
        _ => {}
    }
}
pub(super) fn run() -> Result<usize> {
    let ingress: Value = serde_json::from_str(input::SOURCE_PURCHASED_AIR_INPUT_JSON)?;
    let mut definitions = member(&ingress, "definitions")?.clone();
    // Explicit unit schema operands: Native IDD Scheduled has name + 3 alpha fields, no numerics.
    definitions
        .as_array_mut()
        .ok_or_else(|| invalid("definitions"))?
        .push(json!(["SetpointManager:Scheduled", 4, 0, 0, 0, null]));
    let template = array(member(&ingress, "factories")?)?
        .first()
        .ok_or_else(|| invalid("actual fixture template"))?;
    let mut cases = 0;
    for (label, target, conflict) in [
        ("direct-node", "ZONE ONE INLET", false),
        ("actual-one-member-NodeList", "ZONE ONE INLETS", false),
        ("conflict-live-warmup-last-writer", "ZONE ONE INLET", true),
    ] {
        let mut variants = 0;
        let mut total_manager_calls = 0;
        for (control, control_name) in [
            "Temperature",
            "MaximumTemperature",
            "MinimumTemperature",
            "HumidityRatio",
            "MaximumHumidityRatio",
            "MinimumHumidityRatio",
            "MassFlowRate",
            "MaximumMassFlowRate",
            "MinimumMassFlowRate",
        ]
        .iter()
        .enumerate()
        {
            let mut definition = template.clone();
            strip_idf(&mut definition);
            for family in [
                "Output:Variable",
                "Output:VariableDictionary",
                "Output:Table:SummaryReports",
            ] {
                definition["raw"]
                    .as_object_mut()
                    .ok_or_else(|| invalid("raw input"))?
                    .remove(family);
                definition["object_order"]
                    .as_object_mut()
                    .ok_or_else(|| invalid("unit operand order"))?
                    .remove(family);
            }
            // Clone INPUT ONLY. No Native expected payload is read or injected.
            definition["raw"]["Schedule:Compact"]["Scheduled Unit Swing"] = json!({"schedule_type_limits_name":"Temperature","data":[{"field":"Through: 12/31"},{"field":"For: AllDays"},{"field":"Until: 03:00"},{"field":18.0},{"field":"Until: 24:00"},{"field":27.0}]});
            definition["object_order"]["Schedule:Compact"]
                .as_array_mut()
                .ok_or_else(|| invalid("compact input order"))?
                .push(json!("Scheduled Unit Swing"));
            definition["raw"]["SetpointManager:Scheduled"] = json!({"SP A":{"control_variable":control_name,"schedule_name":"Scheduled Unit Swing","setpoint_node_or_nodelist_name":target}});
            definition["object_order"]["SetpointManager:Scheduled"] = json!(["SP A"]);
            if conflict {
                definition["raw"]["SetpointManager:Scheduled"]["SP B"] = json!({"control_variable":control_name,"schedule_name":"Cooling Setpoint","setpoint_node_or_nodelist_name":target});
                definition["object_order"]["SetpointManager:Scheduled"]
                    .as_array_mut()
                    .ok_or_else(|| invalid("manager input order"))?
                    .push(json!("SP B"));
            }
            let mut manager_calls = 0;
            let mut f = Factory::new_authored_epjson_operands(&definitions, &definition, false)?;
            let names_before = f.equipment.nodes.node_ids.clone();

            let actual_number = f
                .equipment
                .nodes
                .node_ids
                .as_ref()
                .and_then(|rows| rows.iter().position(|name| name == "ZONE ONE INLET"))
                .ok_or_else(|| invalid("actual preexisting inlet identity"))?;
            f.scheduled_phase_inputs(&json!({"hour":2,"timestep":1,"day_of_year":1,"day_of_week":3,"dst":0,"begin_environment":true}))?;
            f.scheduled_selected_step(actual_number, control, if conflict { 24.0 } else { 18.0 })?;
            manager_calls += 1;
            let sp = &f.hvac_manager.simulation.setpoints;
            assert!(!sp.get_input_flag);
            assert!(sp.init_one_time_flag);
            assert!(sp.init_one_time_flag2);
            assert!(!sp.manager_on);
            let rows = sp
                .managers
                .as_ref()
                .ok_or_else(|| invalid("real Scheduled rows"))?;
            assert_eq!(rows.len(), if conflict { 2 } else { 1 });
            assert_eq!(rows[0].control_nodes.len(), 1);
            let number = rows[0].control_nodes[0];
            assert_eq!(number, actual_number);
            assert_eq!(rows[0].control_variable, i32::try_from(control)?);
            assert_eq!(f.equipment.source_node_id(number)?, "ZONE ONE INLET");
            assert_eq!(rows[0].setpoint.to_bits(), 0.0f64.to_bits());
            assert_eq!(f.equipment.nodes.node_ids, names_before);
            f.scheduled_value(number, control, if conflict { 24.0 } else { 18.0 })?;
            f.scheduled_selected_step(number, control, if conflict { 24.0 } else { 18.0 })?;
            manager_calls += 1; // actual repeat while readiness is still false
            assert!(f.hvac_manager.simulation.setpoints.init_one_time_flag2);
            f.scheduled_step(true)?; // real AirLoop call -> actual caller readiness writer -> intentional stop
            let row = &f
                .equipment
                .nodes
                .physical
                .as_ref()
                .ok_or_else(|| invalid("physical nodes"))?[number - 1];
            let unaffected = (
                row.temperature.to_bits(),
                row.humidity_ratio.to_bits(),
                row.mass_flow_rate.to_bits(),
            );
            f.weather.global.warmup_flag = true;
            f.globals.kickoff_simulation = false;
            f.globals.doing_sizing = false;
            f.error_state.warmup_flag = false;
            f.error_state.doing_sizing = true;
            f.error_state.kickoff_simulation = true;
            let warnings = f.error_state.tracking.total_warning_errors;
            let warmup = f.error_state.tracking.total_warning_errors_during_warmup;
            let sizing = f.error_state.tracking.total_warning_errors_during_sizing;
            f.scheduled_selected_step(number, control, if conflict { 24.0 } else { 18.0 })?;
            manager_calls += 1;
            assert!(!f.hvac_manager.simulation.setpoints.init_one_time_flag);
            assert!(!f.hvac_manager.simulation.setpoints.init_one_time_flag2);
            assert_eq!(
                f.error_state.tracking.total_warning_errors - warnings,
                i32::from(conflict)
            );
            assert_eq!(
                f.error_state.tracking.total_warning_errors_during_warmup - warmup,
                i32::from(conflict)
            );
            assert_eq!(
                f.error_state.tracking.total_warning_errors_during_sizing - sizing,
                0
            );
            if conflict {
                assert!(
                    String::from_utf8_lossy(&f.error)
                        .contains("conflicting node name = ZONE ONE INLET")
                );
            }
            f.scheduled_phase_inputs(&json!({"hour":4,"begin_environment":false}))?;
            f.scheduled_selected_step(number, control, if conflict { 24.0 } else { 27.0 })?;
            manager_calls += 1;
            assert!(f.hvac_manager.simulation.setpoints.manager_on);
            assert!(f.hvac_manager.simulation.setpoints.environment_flag);
            assert_eq!(
                f.hvac_manager
                    .simulation
                    .setpoints
                    .managers
                    .as_ref()
                    .ok_or_else(|| invalid("managers"))?[0]
                    .setpoint
                    .to_bits(),
                27.0f64.to_bits()
            );
            f.scheduled_value(number, control, if conflict { 24.0 } else { 27.0 })?;
            let row = &f
                .equipment
                .nodes
                .physical
                .as_ref()
                .ok_or_else(|| invalid("physical nodes"))?[number - 1];
            assert_eq!(
                (
                    row.temperature.to_bits(),
                    row.humidity_ratio.to_bits(),
                    row.mass_flow_rate.to_bits()
                ),
                unaffected
            );

            let before_reset = f.scheduled_setpoint_fields(Some(number))?;
            f.equipment.reset_source_hvac_node_setpoints()?;

            let reset_fields = f.scheduled_setpoint_fields(Some(number))?;
            for (field, (actual, prior)) in reset_fields.iter().zip(before_reset.iter()).enumerate()
            {
                let expected = if matches!(field, 0 | 3..=6) {
                    (-999.0f64).to_bits()
                } else {
                    *prior
                };
                assert_eq!(
                    *actual, expected,
                    "Native first-global-reset subset {field}"
                );
            }
            f.scheduled_selected_step(number, control, if conflict { 24.0 } else { 27.0 })?;
            manager_calls += 1;
            let defaults = f.scheduled_setpoint_fields(None)?;
            f.equipment.reset_source_node_data()?;

            assert_eq!(f.scheduled_setpoint_fields(Some(number))?, defaults);
            f.scheduled_phase_inputs(&json!({"hour":2,"begin_environment":true}))?;
            f.scheduled_selected_step(number, control, if conflict { 24.0 } else { 18.0 })?;
            manager_calls += 1;
            assert!(!f.hvac_manager.simulation.setpoints.manager_on);
            assert!(!f.hvac_manager.simulation.setpoints.environment_flag);
            f.scheduled_value(number, control, if conflict { 24.0 } else { 18.0 })?;
            f.scheduled_phase_inputs(&json!({"begin_environment":false}))?;
            f.scheduled_selected_step(number, control, if conflict { 24.0 } else { 18.0 })?;
            manager_calls += 1;
            assert!(f.hvac_manager.simulation.setpoints.environment_flag);
            f.scheduled_phase_inputs(&json!({"hour":4,"begin_environment":true}))?;
            f.scheduled_selected_step(number, control, if conflict { 24.0 } else { 27.0 })?;
            manager_calls += 1;
            f.scheduled_value(number, control, if conflict { 24.0 } else { 27.0 })?;
            assert_eq!(f.equipment.nodes.node_ids, names_before);
            assert_eq!(manager_calls, 8);

            total_manager_calls += manager_calls;
            variants += 1;
        }
        assert_eq!(variants, 9);
        assert_eq!(total_manager_calls, 72);
        cases += 1;
        println!(
            "Source Scheduled all-selector owner case: {label}; selectors={variants}; actual Manage calls={total_manager_calls}; actual readiness finite-prefix calls={variants}; full-HVAC success not claimed"
        );
    }
    assert_eq!(cases, 3);
    Ok(cases)
}

// Explicit authored epJSON raw operands have no IDF physical-field metadata.
fn project_objects(
    factory: &Value,
    families: &[&str],
) -> Result<BTreeMap<String, Vec<ProjectInputObject>>> {
    let mut out = BTreeMap::new();
    for &family in families {
        let mut rows = Vec::new();
        for (name, raw) in raw_family(factory, family)? {
            let fields = object(raw)?;
            let idf_max_fields = fields.get("idf_max_fields").map(count).transpose()?;
            let fields = fields
                .iter()
                .filter(|(k, _)| {
                    !matches!(
                        k.as_str(),
                        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
                    )
                })
                .map(|(k, v)| {
                    Ok((
                        k.clone(),
                        match v {
                            Value::String(s) => ProjectInputValue::String(s.clone()),
                            Value::Number(n) => {
                                let n = n.as_f64().ok_or_else(|| invalid("JSON binary64"))?;
                                if !n.is_finite() {
                                    return Err(invalid("finite raw number").into());
                                }
                                ProjectInputValue::Number(n)
                            }
                            _ => return Err(invalid("scalar getter input").into()),
                        },
                    ))
                })
                .collect::<Result<_>>()?;
            rows.push(ProjectInputObject {
                object_name: name.into(),
                fields,
                idf_max_fields,
            });
        }
        out.insert(family.into(), rows);
    }
    Ok(out)
}
