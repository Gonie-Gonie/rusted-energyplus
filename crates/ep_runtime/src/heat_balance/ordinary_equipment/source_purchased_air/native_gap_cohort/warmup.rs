//! Test-only narrow original RecKeep/Check caller. No expected state is restored.
use super::*;
use hb::ordinary_heat_balance_initialization::SourceInitHeatBalanceOwners;
use hb::ordinary_heat_balance_initialization::provider_host::SourceOrdinaryInitHeatBalanceHost;
use hb::ordinary_heat_balance_initialization::warmup::{
    SourceWarmupCheckOwners, SourceWarmupRecordOwners,
};
use hb::ordinary_source_session::OrdinarySourceSession;
use hb::selected_heat_balance_input::{HeatBalanceInputOwners, shadowing::SourceShadowingInput};
use hb::warmup::report::WarmupReportConfiguration;

struct WarmupFactory {
    input: SelectedInputProcessor,
    shortcut: SourceInputProcessorShortCut,
    globals: SelectedHeatBalanceGlobals,
    weather: crate::weather::day::WeatherDayState,
    project: SelectedProjectData,
    startup: SelectedSimulationStartup,
    control: SelectedHeatBalanceInputState,
    spaces: SelectedHeatBalanceSpaceControls,
    zones: SelectedZoneInputOwners,
    model: ep_model::TypedModel,
    reports: hb::ctf_initialization_context::CtfInitializationReports,
    session: OrdinarySourceSession,
    geometry: hb::selected_heat_balance_input::surface::SurfaceGeometryInputState,
    history: hb::ctf_thermal_history::CtfThermalHistoryState,
    radiation: hb::radiation::ordinary::OrdinarySolarRadiation,
    shadowing: SourceShadowingInput,
    report: ep_model::ReportInputs,
    prescan: SourceOutputPreScan,
    files: Files,
    eio: Vec<u8>,
    eso: Vec<u8>,
    rf: Vec<String>,
    error_state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    input_file: std::path::PathBuf,
}

fn diagnostics(
    definition: &Value,
) -> Result<Vec<hb::selected_project_data::ProjectDiagnosticsObject>> {
    raw_family(definition, "Output:Diagnostics")?
        .into_iter()
        .map(|(name, row)| {
            let keys = array(member(row, "diagnostics")?)?
                .iter()
                .map(|item| {
                    object(item)?
                        .get("key")
                        .map(|v| text(v).map(str::to_owned))
                        .transpose()
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(hb::selected_project_data::ProjectDiagnosticsObject {
                object_name: name.into(),
                keys,
            })
        })
        .collect()
}

impl WarmupFactory {
    fn new(definitions: &Value, definition: &Value, phase: &Value) -> Result<Box<Self>> {
        if object(phase)?.len() != 5 {
            return Err(invalid("exact initial warmup caller phase fields").into());
        }
        let mut input = make_input(definitions, definition)?;
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input)?;
        let (report, raw_prescan) = report_inputs(definition)?;
        let mut prescan = SourceOutputPreScan::default();
        let mut globals = SelectedHeatBalanceGlobals::default();
        let mut weather = crate::weather::day::WeatherDayState::default();
        let mut project = SelectedProjectData::default();
        let mut startup = SelectedSimulationStartup::default();
        let mut control = SelectedHeatBalanceInputState::default();
        let mut spaces = SelectedHeatBalanceSpaceControls::default();
        let mut reports = hb::ctf_initialization_context::CtfInitializationReports::default();
        let mut state = error_state();
        let header = format!("Program Version,{}\n", state.version_string);
        let mut eio = header.as_bytes().to_vec();
        let eso = header.into_bytes();
        let mut error = Vec::new();
        let mut console = Vec::new();
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
            project.get_project_data(
                &SelectedProjectDataInput {
                    source_is_epjson: false,
                    objects: project_objects(
                        definition,
                        &[
                            "Version",
                            "HeatBalanceAlgorithm",
                            "Timestep",
                            "SimulationControl",
                        ],
                    )?,
                    diagnostics: diagnostics(definition)?,
                },
                &mut ProjectDataOwners {
                    input: &mut input,
                    shortcut: &mut shortcut,
                    globals: &mut globals,
                    weather: &mut weather,
                    hvac: &mut startup.hvac_globals,
                    heat_balance_current_module_object: &mut reports
                        .heat_balance_current_module_object,
                },
                &mut backend,
                &mut eio,
            )?;
            let raw = SelectedHeatBalanceInput {
                source_is_epjson: false,
                surface: Default::default(),
                objects: project_objects(
                    definition,
                    &[
                        "Building",
                        "SurfaceConvectionAlgorithm:Inside",
                        "SurfaceConvectionAlgorithm:Outside",
                        "HeatBalanceAlgorithm",
                        "ZoneAirHeatBalanceAlgorithm",
                        "ZoneAirContaminantBalance",
                        "ZoneCapacitanceMultiplier:ResearchSpecial",
                        "ZoneAirMassFlowConservation",
                    ],
                )?,
            };
            let mut errors_found = false;
            control.get_project_control_data(
                &raw,
                &mut HeatBalanceInputOwners {
                    input: &mut input,
                    shortcut: &mut shortcut,
                    project: &mut project,
                    space_controls: &mut spaces,
                    environment: &mut weather.environment,
                    current_module_object: &mut reports.heat_balance_current_module_object,
                },
                &mut backend,
                &mut eio,
                &mut errors_found,
            )?;
            if errors_found {
                return Err(invalid("real warmup ProjectControl accumulated input error").into());
            }
        }
        let model = identity_model(definition, &control, weather.global.time_steps_in_hour)?;
        let mut zones = SelectedZoneInputOwners::default();
        let mut files = Files::default();
        let mut zone_backend = SourceErrorBackend {
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
        zones.get_zone_data(
            &SelectedHeatBalanceInput {
                source_is_epjson: false,
                surface: Default::default(),
                objects: project_objects(definition, &["Zone"])?,
            },
            &control,
            &model,
            &mut ZoneGetterOwners {
                input: &mut input,
                shortcut: &mut shortcut,
                globals: &mut globals,
                output: &mut reports.output_registration,
                prescan: &prescan,
                report_input: &report,
                minimum_report_frequency: &project.system.minimum_report_frequency,
                developer: &project.system.developer_flag,
                clock: Some(&weather.global),
                backend: &mut zone_backend,
                minutes_in_time_step: weather.global.minutes_in_time_step,
                host: &mut files,
                eio: &mut eio,
            },
            &mut reports.heat_balance_input_errors_found,
        )?;
        let names = zones
            .zones
            .as_ref()
            .ok_or_else(|| invalid("real getter Zone allocation"))?
            .iter()
            .map(|z| z.name.clone())
            .collect();
        let config = WarmupReportConfiguration {
            doing_sizing: boolean(member(phase, "doing_sizing")?)?,
            do_pure_load_calc: boolean(member(phase, "pure_load_calculation")?)?,
            run_period_environment: boolean(member(phase, "run_period_environment")?)?,
            environment_name: text(member(phase, "environment_name")?)?.into(),
            detailed: project.system.report_detailed_warmup_convergence,
        };
        let session = OrdinarySourceSession::new(names, config)?;
        let inventory = object(member(definition, "raw")?)?
            .iter()
            .map(|(f, v)| Ok((f.as_str(), object(v)?.len())))
            .collect::<Result<Vec<_>>>()?;
        let radiation_input =
            hb::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory(
                inventory, false,
            )?;
        let mut out = Box::new(Self {
            input,
            shortcut,
            globals,
            weather,
            project,
            startup,
            control,
            spaces,
            zones,
            model,
            reports,
            session,
            geometry: Default::default(),
            history: hb::ctf_thermal_history::CtfThermalHistoryState::new_source_unallocated(
                Vec::new(),
            ),
            radiation: hb::radiation::ordinary::OrdinarySolarRadiation::for_surface_input(
                radiation_input,
            ),
            shadowing: SourceShadowingInput::new(),
            report,
            prescan,
            files,
            eio,
            eso,
            rf: Vec::new(),
            error_state: state,
            error,
            console,
            input_file: std::path::PathBuf::from(text(member(definition, "input_file")?)?),
        });
        // A real production demand allocator; no scalar/rolling report defaults are staged here.
        out.session
            .controller
            .demand
            .allocate_source_demands(&out.globals, &out.spaces)?;
        out.allocate()?;
        out.weather.global.warmup_flag = boolean(member(phase, "warmup")?)?;
        out.globals.doing_sizing = boolean(member(phase, "doing_sizing")?)?;
        out.globals.do_pure_load_calc = boolean(member(phase, "pure_load_calculation")?)?;
        out.weather.environment.run_period_environment =
            boolean(member(phase, "run_period_environment")?)?;
        out.weather.environment.environment_name = text(member(phase, "environment_name")?)?.into();
        out.synchronize_diagnostic_inputs();
        Ok(out)
    }
    fn synchronize_diagnostic_inputs(&mut self) {
        // Same live phase inputs are represented by the existing backend view.
        self.error_state.warmup_flag = self.weather.global.warmup_flag;
        self.error_state.doing_sizing = self.globals.doing_sizing;
    }
    fn allocate(&mut self) -> Result {
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
        let mut dictionary = SourceOutputDictionaryStreams {
            eso: Some(&mut self.eso),
            sqlite_present: self.reports.source_sqlite.present(),
            results_framework_output_variables: &mut self.rf,
        };
        let mut providers = SourceOrdinaryInitHeatBalanceHost {
            model: &self.model,
            ctf: &mut self.session.ctf,
            registry: &self.reports.registry,
            table: &mut self.reports.table,
            registration: &mut self.reports.output_registration,
            prescan: &self.prescan,
            minimum_report_frequency: &self.project.system.minimum_report_frequency,
            developer: &self.project.system.developer_flag,
            output_host: &mut self.files,
            dictionary_host: &mut dictionary,
            eio: &mut self.eio,
            shading_stream: &mut self.session.shading_stream,
            shading_path: &self.input_file,
            report_flags: &mut self.reports.source_report_flags,
        };
        self.session
            .heat_balance
            .allocate_native_warmup_cohort_arrays(
                &mut SourceInitHeatBalanceOwners {
                    surface: &mut self.session.surface,
                    geometry: &mut self.geometry,
                    zones: &mut self.zones,
                    control: &mut self.control,
                    air: &mut self.session.air,
                    radiation: &mut self.radiation,
                    fan_zone: &mut self.session.fan,
                    gains: &mut self.reports.selected_internal_heat_gains,
                    gain_modules: &mut self.session.gain_modules,
                    daylighting: &mut self.session.daylighting,
                    window: &mut self.session.window,
                    globals: &self.globals,
                    clock: &self.weather.global,
                    environment: &mut self.weather.environment,
                    weather_owner: &self.weather.weather,
                    shadowing: &self.shadowing,
                    materials: &self.reports.selected_material_data,
                    bounds: &self.startup.construction_bounds,
                    input: &mut self.input,
                    shortcut: &mut self.shortcut,
                    report_inputs: &self.report,
                    scan: &mut self.reports.scan,
                    backend: &mut backend,
                },
                &mut providers,
            )?;
        Ok(())
    }
    fn prepare_call(&mut self, step: &Value) -> Result {
        let clock = member(step, "clock")?;
        if object(clock)?.len() != 3 {
            return Err(invalid("exact named warmup clock fields").into());
        }
        let day = int(member(clock, "day_of_sim")?)?;
        let hour = int(member(clock, "hour")?)?;
        let timestep = int(member(clock, "timestep")?)?;
        if day <= 0
            || !(1..=24).contains(&hour)
            || !(1..=self.weather.global.time_steps_in_hour).contains(&timestep)
        {
            return Err(invalid("actual named warmup clock domain").into());
        }
        self.weather.global.day_of_sim = day;
        self.weather.global.hour_of_day = hour;
        self.weather.global.time_step = timestep;
        let operation = text(member(step, "operation")?)?;
        if operation == "record" {
            let rows = array(member(step, "zone_inputs")?)?;
            if rows.len() != usize::try_from(self.globals.num_of_zones)? {
                return Err(invalid("actual warmup Zone input extent").into());
            }
            for (at, row) in rows.iter().enumerate() {
                if object(row)?.len() != 4 || index(int(member(row, "ordinal")?)?)? != at {
                    return Err(invalid("actual warmup Zone input fields/ordinal").into());
                }
                self.session
                    .air
                    .zones
                    .as_mut()
                    .and_then(|z| z.get_mut(at))
                    .ok_or_else(|| invalid("real allocated warmup Air row"))?
                    .air
                    .ztav = real(member(row, "ZTAV")?)?;
                let demand = self
                    .session
                    .controller
                    .demand
                    .zones
                    .as_mut()
                    .and_then(|z| z.get_mut(at))
                    .ok_or_else(|| invalid("real allocated warmup demand row"))?;
                demand.air_sys_heat_rate = real(member(row, "airSysHeatRate")?)?;
                demand.air_sys_cool_rate = real(member(row, "airSysCoolRate")?)?;
            }
        }
        if !matches!(operation, "record" | "check") {
            return Err(invalid("actual full warmup operation").into());
        }
        self.synchronize_diagnostic_inputs();
        Ok(())
    }
    fn invoke(&mut self, step: &Value) -> Result {
        let operation = text(member(step, "operation")?)?;
        match operation {
            "record" => self.session.heat_balance.record_source_heat_balance(
                &mut SourceWarmupRecordOwners {
                    air: &self.session.air,
                    demand: &self.session.controller.demand,
                    geometry: &self.geometry,
                    history: &self.history,
                    radiation: &mut self.radiation,
                    zones: &self.zones,
                    control: &self.control,
                    globals: &self.globals,
                    clock: &self.weather.global,
                    project: &self.project,
                    eio: &mut self.eio,
                },
            )?,
            "check" => {
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
                self.session.heat_balance.check_source_warmup_convergence(
                    &mut SourceWarmupCheckOwners {
                        zones: &self.zones,
                        control: &self.control,
                        project: &self.project,
                        globals: &self.globals,
                        clock: &mut self.weather.global,
                        environment: &self.weather.environment,
                        backend: &mut backend,
                    },
                )?;
            }
            _ => return Err(invalid("actual RecKeep or Check operation").into()),
        }
        self.synchronize_diagnostic_inputs();
        Ok(())
    }
}

// Passive same-owner observations. No Native output is restored.
fn report_array_shape<T>(values: &[T], allocated: bool) -> Value {
    json!({"allocated":allocated,"size":values.len(),"lower":1,"upper":values.len()})
}
fn observed_real_array(values: &Option<Vec<f64>>) -> Value {
    let mut out = array1(values);
    out["items"] = Value::Array(
        values
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .copied()
            .map(number)
            .collect(),
    );
    out
}
fn observed_report_array(values: &[f64], allocated: bool) -> Value {
    let mut out = report_array_shape(values, allocated);
    out["items"] = Value::Array(values.iter().copied().map(number).collect());
    out
}
fn observed_report_matrix(
    values: &[Vec<f64>],
    allocated: bool,
    zones: usize,
    points: usize,
) -> Result<Value> {
    if values.len() != points || values.iter().any(|row| row.len() != zones) {
        return Err(invalid("same real warmup report matrix shape").into());
    }
    Ok(
        json!({"allocated":allocated,"size":zones.checked_mul(points).ok_or_else(||invalid("warmup matrix size"))?,
        "lower1":1,"upper1":zones,"lower2":1,"upper2":points,
        "items_by_point":values.iter().map(|row| Value::Array(row.iter().copied().map(number).collect())).collect::<Vec<_>>()}),
    )
}
fn observed_manager(factory: &WarmupFactory) -> Result<Value> {
    let m = &factory.session.heat_balance;
    let report = &m.report;
    let available = report
        .source_allocations
        .ok_or_else(|| invalid("actual Source warmup array availability"))?;
    let mut arrays = Map::new();
    macro_rules! real_array {
        ($native:literal, $field:ident) => {
            arrays.insert($native.into(), observed_real_array(&m.$field));
        };
    }
    real_array!("MaxCoolLoadPrevDay", max_cool_load_prev_day);
    real_array!("MaxCoolLoadZone", max_cool_load_zone);
    real_array!("MaxHeatLoadPrevDay", max_heat_load_prev_day);
    real_array!("MaxHeatLoadZone", max_heat_load_zone);
    real_array!("MaxTempPrevDay", max_temp_prev_day);
    real_array!("MaxTempZone", max_temp_zone);
    real_array!("MinTempPrevDay", min_temp_prev_day);
    real_array!("MinTempZone", min_temp_zone);
    arrays.insert(
        "WarmupTempDiff".into(),
        observed_report_array(&report.temperature_difference, available[0]),
    );
    arrays.insert(
        "WarmupLoadDiff".into(),
        observed_report_array(&report.load_difference, available[1]),
    );
    real_array!("TempZoneSecPrevDay", temp_zone_sec_prev_day);
    real_array!("LoadZoneSecPrevDay", load_zone_sec_prev_day);
    real_array!("TempZonePrevDay", temp_zone_prev_day);
    real_array!("LoadZonePrevDay", load_zone_prev_day);
    real_array!("TempZone", temp_zone);
    real_array!("LoadZone", load_zone);
    let mut convergence = array1(&m.convergence);
    convergence["items"] = Value::Array(m.convergence.as_deref().unwrap_or(&[]).iter().map(|row|json!({
        "address":pointer(row),"PassFlag":{"allocated":true,"size":row.pass_flags.len(),"lower":1,"upper":row.pass_flags.len(),"items":row.pass_flags},
        "TestMaxTempValue":number(row.test_values[0]),"TestMinTempValue":number(row.test_values[1]),
        "TestMaxHeatLoadValue":number(row.test_values[2]),"TestMaxCoolLoadValue":number(row.test_values[3])})).collect());
    let mut temp_scratch = report_array_shape(&report.temperature_stddev, available[5]);
    let mut load_scratch = report_array_shape(&report.load_stddev, available[6]);
    // Native allocated primitive scratch is indeterminate until another routine writes it.
    // This unit never invokes that writer; Source's None entries are not numeric answers.
    if report.temperature_stddev.iter().any(Option::is_some)
        || report.load_stddev.iter().any(Option::is_some)
    {
        return Err(invalid("warmup scratch has an unexpected actual writer").into());
    }
    temp_scratch["items"] = Value::Null;
    temp_scratch["values_written"] = json!(false);
    load_scratch["items"] = Value::Null;
    load_scratch["values_written"] = json!(false);
    let zones = usize::try_from(factory.globals.num_of_zones)?;
    Ok(json!({"owner_address":pointer(m),"arrays":arrays,
        "TempZoneRpt":observed_report_matrix(&report.temperature,available[2],zones,report.points_per_day)?,
        "LoadZoneRpt":observed_report_matrix(&report.load,available[3],zones,report.points_per_day)?,
        "MaxLoadZoneRpt":observed_report_matrix(&report.maximum_load,available[4],zones,report.points_per_day)?,
        "TempZoneRptStdDev":temp_scratch,"LoadZoneRptStdDev":load_scratch,"WarmupConvergenceValues":convergence,
        "CountWarmupDayPoints":report.count,"ManageHeatBalanceGetInputFlag":m.manage_heat_balance_get_input_flag,
        "ChangeSet":m.change_set,"FirstWarmupWrite":report.first_detailed_write,
        "WarmupConvergenceWarning":report.warmup_convergence_warning,"SizingWarmupConvergenceWarning":report.sizing_warmup_convergence_warning,
        "ReportWarmupConvergenceFirstWarmupWrite":report.first_summary_write,
        "CurrentModuleObject":factory.reports.heat_balance_current_module_object}))
}
fn observed_state(factory: &WarmupFactory) -> Result<Value> {
    let zones = factory
        .zones
        .zones
        .as_deref()
        .ok_or_else(|| invalid("real warmup Zone owner"))?;
    let air = factory
        .session
        .air
        .zones
        .as_deref()
        .ok_or_else(|| invalid("real warmup Air owner"))?;
    let demand = factory
        .session
        .controller
        .demand
        .zones
        .as_deref()
        .ok_or_else(|| invalid("real warmup demand owner"))?;
    let count = usize::try_from(factory.globals.num_of_zones)?;
    if zones.len() != count || air.len() != count || demand.len() != count {
        return Err(invalid("genuine warmup physical row extents").into());
    }
    let rows = zones.iter().zip(air).zip(demand).enumerate().map(|(at,((zone,air),demand))|json!({
        "ordinal":at+1,"zone_address":pointer(zone),"air_address":pointer(&air.air),"demand_address":pointer(demand),
        "name":zone.name,"ZTAV":number(air.air.ztav),"airSysHeatRate":number(demand.air_sys_heat_rate),"airSysCoolRate":number(demand.air_sys_cool_rate)})).collect::<Vec<_>>();
    let clock = &factory.weather.global;
    let project = &factory.control;
    let error = &factory.error_state;
    let tracking = &error.tracking;
    let recurring = &factory.reports.source_recurring_errors;
    Ok(
        json!({"state_address":pointer(factory),"manager":observed_manager(factory)?,"zones":rows,
        "clock":{"NumOfZones":factory.globals.num_of_zones,"TimeStepsInHour":clock.time_steps_in_hour,"TimeStep":clock.time_step,
            "HourOfDay":clock.hour_of_day,"DayOfSim":clock.day_of_sim,"WarmupFlag":clock.warmup_flag,
            "DoingSizing":factory.globals.doing_sizing,"DoPureLoadCalc":factory.globals.do_pure_load_calc,
            "KickOffSimulation":error.kickoff_simulation,"DoingInputProcessing":error.doing_input_processing},
        "project":{"LoadsConvergTol":number(project.loads_convergence_tolerance),"TempConvergTol":number(project.temperature_convergence_tolerance),
            "MaxNumberOfWarmupDays":project.max_number_of_warmup_days,"MinNumberOfWarmupDays":factory.project.min_number_of_warmup_days,
            "ReportDetailedWarmupConvergence":factory.project.system.report_detailed_warmup_convergence},
        "environment":{"EnvironmentName":factory.weather.environment.environment_name,"RunPeriodEnvironment":factory.weather.environment.run_period_environment},
        "allocation_context":{"numSpaces":factory.globals.num_spaces,"NumOfSolarEnclosures":factory.radiation.num_solar_enclosures(),
            "CO2Simulation":project.carbon_dioxide_simulation,"GenericContamSimulation":project.generic_contaminant_simulation,
            "TotThermalReportPers":factory.weather.weather.tot_thermal_report_periods,"TotCO2ReportPers":factory.weather.weather.tot_co2_report_periods,
            "TotVisualReportPers":factory.weather.weather.tot_visual_report_periods},
        "surface_child_domain":{"AnyMovableInsulation":factory.geometry.any_movable_insulation,
            "AllHTWindowSurfaceList":factory.geometry.surface_lists.all_ht_window,"TotSurfaces":factory.geometry.total_surfaces},
        "eio":std::str::from_utf8(&factory.eio)?,
        "errors":{"text":std::str::from_utf8(&factory.error)?,"TotalWarningErrors":tracking.total_warning_errors,
            "TotalSevereErrors":tracking.total_severe_errors,"TotalWarningErrorsDuringWarmup":tracking.total_warning_errors_during_warmup,
            "TotalSevereErrorsDuringWarmup":tracking.total_severe_errors_during_warmup,"TotalWarningErrorsDuringSizing":tracking.total_warning_errors_during_sizing,
            "TotalSevereErrorsDuringSizing":tracking.total_severe_errors_during_sizing,"NumRecurringErrors":recurring.number,
            "RecurringErrors":array1(&recurring.rows),"MatchCounts":{"allocated":true,"size":tracking.match_counts.len(),"lower":1,"upper":tracking.match_counts.len(),"items":tracking.match_counts}},
        "error_context":{"LastSevereError":tracking.last_severe_error,"AbortProcessing":tracking.abort_processing,
            "outputErrorHeader":error.output_error_header,"VerStringVar":error.version_string,"IDDVerString":error.idd_version_string,
            "sqlite_present":factory.reports.source_sqlite.present(),
            // All actual backend views in this factory pass the genuine optional callback as None.
            "error_callback_present":false}}),
    )
}

#[derive(Default)]
struct WarmupIdentities {
    pairs: BTreeMap<(String, String), String>,
    raw_bits: usize,
}
impl WarmupIdentities {
    fn compare(&mut self, source: &Value, native: &Value, path: &str) -> Result {
        match (source, native) {
            (Value::Object(a), Value::Object(b)) => {
                assert_eq!(
                    a.keys().collect::<Vec<_>>(),
                    b.keys().collect::<Vec<_>>(),
                    "{path} complete admitted keyset"
                );
                if a.len() == 1 && a.contains_key("value_bits") {
                    self.raw_bits += 1;
                }
                for (key, actual) in a {
                    let expected = b.get(key).ok_or_else(|| invalid("observed warmup key"))?;
                    if matches!(
                        key.as_str(),
                        "address"
                            | "owner_address"
                            | "state_address"
                            | "zone_address"
                            | "air_address"
                            | "demand_address"
                    ) {
                        let actual = text(actual)?;
                        let expected = text(expected)?;
                        assert!(
                            actual.starts_with("0x") && expected.starts_with("0x"),
                            "{path} real pointer transport"
                        );
                        let kind = key.clone();
                        let identity = (kind.clone(), expected.to_owned());
                        if let Some(previous) = self.pairs.get(&identity) {
                            assert_eq!(previous, actual, "{path} persistent same owner");
                        } else {
                            assert!(
                                !self
                                    .pairs
                                    .iter()
                                    .any(|((old_kind, old_native), old_source)| old_kind == &kind
                                        && old_native != expected
                                        && old_source == actual),
                                "{path} distinct real rows"
                            );
                            self.pairs.insert(identity, actual.to_owned());
                        }
                    } else {
                        self.compare(actual, expected, &format!("{path}.{key}"))?;
                    }
                }
            }
            (Value::Array(a), Value::Array(b)) => {
                assert_eq!(a.len(), b.len(), "{path} complete array extent");
                for (at, (actual, expected)) in a.iter().zip(b).enumerate() {
                    self.compare(actual, expected, &format!("{path}[{at}]"))?;
                }
            }
            _ => assert_eq!(source, native, "{path} exact observed value"),
        }
        Ok(())
    }
    fn check(&mut self, factory: &WarmupFactory, native: &Value, path: &str) -> Result {
        let source = observed_state(factory)?;
        let mut admitted = native.clone();
        let manager = admitted
            .get_mut("manager")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| invalid("actual Native manager observation"))?;
        if !manager.remove("DoReport").is_some_and(|v| v.is_boolean()) {
            return Err(invalid("actual unused Native DoReport shape").into());
        }
        let surface = admitted
            .get_mut("surface_child_domain")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| invalid("actual Native surface observation"))?;
        let insulation = surface
            .remove("intMovInsulSurfNums")
            .ok_or_else(|| invalid("actual Native unused insulation list"))?;
        assert!(
            array(&insulation)?.is_empty(),
            "unit genuinely has no movable-insulation rows"
        );
        // No Source cells represent these two unconsumed legacy fields. All other
        // observed values, including entire diagnostics and all consumed arrays, compare.
        self.compare(&source, &admitted, path)
    }
}

pub(super) fn run(fixture: &Value) -> Result<usize> {
    let ingress = member(fixture, "source_inputs")?;
    assert_eq!(
        member(ingress, "schema")?,
        "src06-warmup-Source-primary-ingress.v1"
    );
    let definitions = member(ingress, "definitions")?;
    let factories = array(member(ingress, "factories")?)?;
    let named = member(member(fixture, "named_inputs")?, "warmup")?;
    let expected = member(
        member(member(fixture, "native_output")?, "groups")?,
        "warmup",
    )?;
    assert_eq!(
        member(named, "schema")?,
        "src06-warmup-RecKeep-Check-input.v1"
    );
    assert_eq!(
        member(expected, "schema")?,
        "src06-warmup-RecKeep-Check-output.v1"
    );
    let cases = array(member(named, "cases")?)?;
    let outputs = array(member(expected, "cases")?)?;
    assert_eq!(cases.len(), 6);
    assert_eq!(outputs.len(), 6);
    assert_eq!(factories.len(), 6);
    let mut calls = 0;
    let mut bits = 0;
    for (case, output) in cases.iter().zip(outputs) {
        let name = member(case, "name")?;
        assert_eq!(name, member(output, "name")?);
        assert_eq!(member(case, "origin")?, member(output, "origin")?);
        assert_eq!(
            member(case, "initial_phase")?,
            member(output, "initial_phase")?
        );
        let definition = factories
            .iter()
            .find(|v| v.get("name") == Some(name))
            .ok_or_else(|| invalid("exact real warmup factory ingress"))?;
        assert_eq!(
            member(definition, "input_file")?,
            member(member(case, "factory")?, "input_file")?
        );
        assert_eq!(member(definition, "steps")?, member(case, "steps")?);
        let mut factory =
            WarmupFactory::new(definitions, definition, member(case, "initial_phase")?)?;
        let mut identities = WarmupIdentities::default();
        identities.check(
            &factory,
            member(output, "after_real_factory_project_and_allocation")?,
            &format!("{name}.allocated"),
        )?;
        let steps = array(member(case, "steps")?)?;
        let actuals = array(member(output, "steps")?)?;
        assert_eq!(steps.len(), actuals.len());
        for (at, (step, actual)) in steps.iter().zip(actuals).enumerate() {
            assert_eq!(count(member(actual, "ordinal")?)?, at + 1);
            assert_eq!(member(step, "name")?, member(actual, "name")?);
            assert_eq!(member(step, "operation")?, member(actual, "operation")?);
            assert_eq!(step, member(actual, "named_inputs")?);
            // Caller clock and current primary values precede the genuine method entry.
            factory.prepare_call(step)?;
            identities.check(
                &factory,
                member(actual, "entry")?,
                &format!("{name}.step{at}.entry"),
            )?;
            factory.invoke(step)?;
            identities.check(
                &factory,
                member(actual, "exit")?,
                &format!("{name}.step{at}.exit"),
            )?;
            calls += 1;
        }
        bits += identities.raw_bits;
    }
    assert_eq!(calls, 69);
    println!(
        "Native warmup cohort: {calls} full persistent RecKeep/Check calls; {bits} strict finite fields; no prior cohort rerun"
    );
    Ok(calls)
}

impl OrdinaryEquipment {
    pub(crate) fn run_native_gap_warmup_cohort(fixture: &Value) -> Result<usize> {
        run(fixture)
    }
}
