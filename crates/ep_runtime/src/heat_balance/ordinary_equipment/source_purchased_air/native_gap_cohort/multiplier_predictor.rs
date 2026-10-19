//! One genuine multiplier2 getter lifetime; full Predict/Correct/history calls.
//! Native expected snapshots are assertions only and never initialize a cell.
use super::*;
use hb::ordinary_air_heat_balance::output_registration::SourceInitAirOutputHost;
use hb::ordinary_air_heat_balance::predictor_corrector::{SourceAirUpdate, SourceZoneAirOwners};
use hb::ordinary_heat_balance_initialization::provider_host::SourceOrdinaryInitHeatBalanceHost;
use hb::ordinary_heat_balance_initialization::{
    SourceHeatBalanceManager, SourceInitHeatBalanceOwners,
};
use hb::selected_heat_balance_input::{HeatBalanceInputOwners, shadowing::SourceShadowingInput};
use hb::warmup::report::{WarmupReportConfiguration, WarmupReportState};

fn real_values(v: &Option<Vec<f64>>, bounded: bool) -> Value {
    let mut out = if bounded { array1(v) } else { shape(v) };
    out["items"] = Value::Array(
        v.as_deref()
            .unwrap_or(&[])
            .iter()
            .copied()
            .map(number)
            .collect(),
    );
    out
}
fn state(f: &Factory) -> Result<Value> {
    let mut out = super::air_history::multiplier_state(f)?;
    let z = index(f.zone)?;
    let fan = &f.fan.thermostat;
    out["fan"] = json!({"LoadCorrectionFactor":real_values(&fan.load_correction_factor,false),
        "TempTstatAir":real_values(&f.fan.temp_tstat_air,true)});
    let sp = fan
        .zone_tstat_setpoints
        .as_ref()
        .and_then(|r| r.get(z))
        .ok_or_else(|| invalid("real thermostat row"))?;
    out["thermostat"] = json!({"setpt":number(sp.setpt),"setptLo":number(sp.setpt_lo),"setptHi":number(sp.setpt_hi),
        "setptAdapComfortCool":number(sp.setpt_adap_comfort_cool),"address":pointer(sp)});
    let demand = &f.controller.demand;
    let flag = |rows: &Option<Vec<bool>>| -> Result<bool> {
        Ok(*rows
            .as_ref()
            .and_then(|r| r.get(z))
            .ok_or_else(|| invalid("real allocated demand flag"))?)
    };
    out["deadband"] = json!({"DeadBandOrSetback":flag(&demand.deadband_or_setback)?,
        "CurDeadBandOrSetback":flag(&demand.current_deadband_or_setback)?,"Setback":flag(&demand.setback)?});
    let zone = f
        .zones
        .zones
        .as_ref()
        .and_then(|r| r.get(z))
        .ok_or_else(|| invalid("same real Zone"))?;
    out["predictor_flags"] = json!({"GetZoneAirStatsInputFlag":f.controller.controls.get_zone_air_stats_input,
        "InitZoneAirSetPointsOneTimeFlag":f.controller.flags.initialize_setpoints_once,"MyEnvrnFlag":f.controller.flags.my_environment,
        "NumStageCtrZone":f.controller.flags.number_stage_zones,"NumOnOffCtrZone":f.controller.flags.number_on_off_zones,
        "humidityControlZoneIndex":zone.humidity_control_zone_index,"NumOfSysTimeSteps":f.startup.hvac_globals.number_system_time_steps,
        "NumOfSysTimeStepsLastZoneTimeStep":f.startup.hvac_globals.number_system_time_steps_last_zone,
        "ShortenTimeStepSysRoomAir":f.startup.hvac_globals.shorten_time_step_sys_room_air});
    let sensible = demand
        .zones
        .as_ref()
        .and_then(|r| r.get(z))
        .ok_or_else(|| invalid("same sensible demand row"))?;
    let moisture = demand
        .zone_moisture
        .as_ref()
        .and_then(|r| r.get(z))
        .ok_or_else(|| invalid("same moisture demand row"))?;
    let mut ds = Map::new();
    let mut dm = Map::new();
    ds.insert("address".into(), pointer(sensible));
    dm.insert("address".into(), pointer(moisture));
    ds.insert(
        "NumZoneEquipment".into(),
        json!(sensible.base.num_zone_equipment),
    );
    ds.insert("StageNum".into(), json!(sensible.base.stage_num));
    dm.insert(
        "NumZoneEquipment".into(),
        json!(moisture.base.num_zone_equipment),
    );
    dm.insert("StageNum".into(), json!(moisture.base.stage_num));
    ds.insert(
        "RemainingOutputRequired".into(),
        number(sensible.base.remaining_output_required),
    );
    ds.insert(
        "UnadjRemainingOutputRequired".into(),
        number(sensible.base.unadjusted_remaining_output_required),
    );
    ds.insert(
        "TotalOutputRequired".into(),
        number(sensible.base.total_output_required),
    );
    ds.insert(
        "SupplyAirAdjustFactor".into(),
        number(sensible.base.supply_air_adjust_factor),
    );
    ds.insert(
        "OutputRequiredToHeatingSP".into(),
        number(sensible.output_required_to_heating_sp),
    );
    ds.insert(
        "OutputRequiredToCoolingSP".into(),
        number(sensible.output_required_to_cooling_sp),
    );
    ds.insert(
        "RemainingOutputReqToHeatSP".into(),
        number(sensible.remaining_output_to_heat_sp),
    );
    ds.insert(
        "RemainingOutputReqToCoolSP".into(),
        number(sensible.remaining_output_to_cool_sp),
    );
    ds.insert(
        "UnadjRemainingOutputReqToHeatSP".into(),
        number(sensible.unadjusted_remaining_output_to_heat_sp),
    );
    ds.insert(
        "UnadjRemainingOutputReqToCoolSP".into(),
        number(sensible.unadjusted_remaining_output_to_cool_sp),
    );
    ds.insert("predictedRate".into(), number(sensible.predicted_rate));
    ds.insert(
        "predictedHSPRate".into(),
        number(sensible.predicted_heating_sp_rate),
    );
    ds.insert(
        "predictedCSPRate".into(),
        number(sensible.predicted_cooling_sp_rate),
    );
    ds.insert("airSysHeatRate".into(), number(sensible.air_sys_heat_rate));
    ds.insert("airSysCoolRate".into(), number(sensible.air_sys_cool_rate));
    ds.insert(
        "airSysHeatEnergy".into(),
        number(sensible.air_sys_heat_energy),
    );
    ds.insert(
        "airSysCoolEnergy".into(),
        number(sensible.air_sys_cool_energy),
    );
    ds.insert(
        "SequencedOutputRequired".into(),
        real_values(&sensible.sequenced_output_required, false),
    );
    ds.insert(
        "SequencedOutputRequiredToHeatingSP".into(),
        real_values(&sensible.sequenced_output_to_heating_sp, false),
    );
    ds.insert(
        "SequencedOutputRequiredToCoolingSP".into(),
        real_values(&sensible.sequenced_output_to_cooling_sp, false),
    );
    dm.insert(
        "RemainingOutputRequired".into(),
        number(moisture.base.remaining_output_required),
    );
    dm.insert(
        "UnadjRemainingOutputRequired".into(),
        number(moisture.base.unadjusted_remaining_output_required),
    );
    dm.insert(
        "TotalOutputRequired".into(),
        number(moisture.base.total_output_required),
    );
    dm.insert(
        "SupplyAirAdjustFactor".into(),
        number(moisture.base.supply_air_adjust_factor),
    );
    dm.insert(
        "OutputRequiredToHumidifyingSP".into(),
        number(moisture.output_required_to_humidifying_sp),
    );
    dm.insert(
        "OutputRequiredToDehumidifyingSP".into(),
        number(moisture.output_required_to_dehumidifying_sp),
    );
    dm.insert(
        "RemainingOutputReqToHumidSP".into(),
        number(moisture.remaining_output_to_humid_sp),
    );
    dm.insert(
        "RemainingOutputReqToDehumidSP".into(),
        number(moisture.remaining_output_to_dehumid_sp),
    );
    dm.insert(
        "UnadjRemainingOutputReqToHumidSP".into(),
        number(moisture.unadjusted_remaining_output_to_humid_sp),
    );
    dm.insert(
        "UnadjRemainingOutputReqToDehumidSP".into(),
        number(moisture.unadjusted_remaining_output_to_dehumid_sp),
    );
    dm.insert("predictedRate".into(), number(moisture.predicted_rate));
    dm.insert(
        "predictedHumSPRate".into(),
        number(moisture.predicted_humid_sp_rate),
    );
    dm.insert(
        "predictedDehumSPRate".into(),
        number(moisture.predicted_dehumid_sp_rate),
    );
    dm.insert("airSysHeatRate".into(), number(moisture.air_sys_heat_rate));
    dm.insert("airSysCoolRate".into(), number(moisture.air_sys_cool_rate));
    dm.insert(
        "airSysHeatEnergy".into(),
        number(moisture.air_sys_heat_energy),
    );
    dm.insert(
        "airSysCoolEnergy".into(),
        number(moisture.air_sys_cool_energy),
    );
    dm.insert(
        "airSysSensibleHeatRatio".into(),
        number(moisture.air_sys_sensible_heat_ratio),
    );
    dm.insert(
        "vaporPressureDifference".into(),
        number(moisture.vapor_pressure_difference),
    );
    dm.insert(
        "SequencedOutputRequired".into(),
        real_values(&moisture.sequenced_output_required, false),
    );
    dm.insert(
        "SequencedOutputRequiredToHumidSP".into(),
        real_values(&moisture.sequenced_output_to_humid_sp, false),
    );
    dm.insert(
        "SequencedOutputRequiredToDehumidSP".into(),
        real_values(&moisture.sequenced_output_to_dehumid_sp, false),
    );
    out["sensible"] = Value::Object(ds);
    out["moisture"] = Value::Object(dm);
    Ok(out)
}

fn primary_inputs(f: &mut Factory, v: &Value) -> Result {
    let values = object(v)?;
    for key in values.keys() {
        if !matches!(
            key.as_str(),
            "begin_environment"
                | "std_rho"
                | "pressure"
                | "system_hours"
                | "system_seconds"
                | "hour"
                | "timestep"
                | "day_of_year"
                | "day_of_week"
                | "dst"
                | "warmup"
                | "outdoor_humidity"
                | "MAT"
                | "NonAirSystemResponse"
                | "SysDepZoneLoadsLagged"
                | "latentGain"
                | "inlet_temperature"
                | "inlet_humidity"
                | "inlet_mass_flow"
                | "zone_mass_flow"
                | "ZTAV"
                | "airHumRatAvg"
                | "num_sys_steps"
                | "last_num_sys_steps"
        ) {
            return Err(invalid("only named multiplier caller inputs, no histories/coefficient/property/multiplier/exit answers").into());
        }
    }
    f.phase_inputs(v)?;
    if let Some(v) = values.get("outdoor_humidity") {
        f.weather.environment.current_weather.out_hum_rat = real(v)?;
    }
    let air = &mut f
        .air
        .zones
        .as_mut()
        .and_then(|r| r.get_mut(index(f.zone).ok()?))
        .ok_or_else(|| invalid("actual persistent AirHB row"))?
        .air;
    for (name, slot) in [
        ("MAT", &mut air.mat),
        ("NonAirSystemResponse", &mut air.non_air_system_response),
        ("SysDepZoneLoadsLagged", &mut air.sys_dep_zone_loads_lagged),
        ("latentGain", &mut air.latent_gain),
        ("ZTAV", &mut air.ztav),
        ("airHumRatAvg", &mut air.air_hum_rat_avg),
    ] {
        if let Some(v) = values.get(name) {
            *slot = real(v)?;
        }
    }
    if let Some(v) = values.get("num_sys_steps") {
        f.startup.hvac_globals.number_system_time_steps = int(v)?;
    }
    if let Some(v) = values.get("last_num_sys_steps") {
        f.startup.hvac_globals.number_system_time_steps_last_zone = int(v)?;
    }
    let config = f
        .equipment
        .zone_config
        .as_ref()
        .and_then(|r| r.get(index(f.zone).ok()?))
        .ok_or_else(|| invalid("same getter Config"))?;
    let inlets = config
        .inlet_node_numbers
        .as_ref()
        .ok_or_else(|| invalid("same inlet array"))?;
    if config.inlet_nodes != 1 || inlets.len() != 1 || config.zone_node == 0 {
        return Err(invalid("genuine single inlet and physical zone node").into());
    }
    let inlet = inlets[0];
    let zone = config.zone_node;
    {
        let row = f
            .equipment
            .nodes
            .physical
            .as_mut()
            .and_then(|r| r.get_mut(inlet.checked_sub(1)?))
            .ok_or_else(|| invalid("same physical inlet"))?;
        if let Some(v) = values.get("inlet_temperature") {
            row.temperature = real(v)?;
        }
        if let Some(v) = values.get("inlet_humidity") {
            row.humidity_ratio = real(v)?;
        }
        if let Some(v) = values.get("inlet_mass_flow") {
            row.mass_flow_rate = real(v)?;
        }
    }
    if let Some(v) = values.get("zone_mass_flow") {
        f.equipment
            .nodes
            .physical
            .as_mut()
            .and_then(|r| r.get_mut(zone.checked_sub(1)?))
            .ok_or_else(|| invalid("same physical ZoneNode"))?
            .mass_flow_rate = real(v)?;
    }
    Ok(())
}

// These extra Native module owners are constructed once, then borrowed with the
// existing P1 factory's actual Zone/Air/Node/gain owners by the real allocator.
struct AllocationOwners {
    _manager: SourceHeatBalanceManager,
    _ctf: hb::ordinary_heat_balance_initialization::ctf::SourceCtfInitialization,
    _daylighting: hb::ordinary_daylighting::SourceDaylighting,
    _shadowing: SourceShadowingInput,
    _shading: hb::radiation::ordinary::shading_report::SourceShadingStream,
}
fn allocate(
    f: &mut Factory,
    definition: &Value,
    path: &std::path::Path,
) -> Result<AllocationOwners> {
    let names = f
        .zones
        .zones
        .as_ref()
        .ok_or_else(|| invalid("real getter zones"))?
        .iter()
        .map(|z| z.name.clone())
        .collect();
    let mut manager = SourceHeatBalanceManager::new(WarmupReportState::new_source_unallocated(
        names,
        WarmupReportConfiguration {
            doing_sizing: f.globals.doing_sizing,
            do_pure_load_calc: f.globals.do_pure_load_calc,
            run_period_environment: f.weather.environment.run_period_environment,
            environment_name: f.weather.environment.environment_name.clone(),
            detailed: f.project.system.report_detailed_warmup_convergence,
        },
    ))?;
    let model = identity_model(definition, &f.control, f.weather.global.time_steps_in_hour)?;
    let mut ctf = hb::ordinary_heat_balance_initialization::ctf::SourceCtfInitialization::default();
    let mut daylighting = hb::ordinary_daylighting::SourceDaylighting::default();
    let shadowing = SourceShadowingInput::new();
    let mut shading = hb::radiation::ordinary::shading_report::SourceShadingStream::default();
    let mut backend = SourceErrorBackend {
        state: &mut f.error_state,
        error_stream: Some(&mut f.error),
        console: &mut f.console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let mut dictionary = SourceOutputDictionaryStreams {
        eso: Some(&mut f.eso),
        sqlite_present: f.ancillary.source_sqlite.present(),
        results_framework_output_variables: &mut f.rf,
    };
    let mut providers = SourceOrdinaryInitHeatBalanceHost {
        model: &model,
        ctf: &mut ctf,
        registry: &f.registry,
        table: &mut f.ancillary.table,
        registration: &mut f.registration,
        prescan: &f.prescan,
        minimum_report_frequency: &f.project.system.minimum_report_frequency,
        output_host: &mut f.files,
        dictionary_host: &mut dictionary,
        eio: &mut f.eio,
        shading_stream: &mut shading,
        shading_path: path,
        report_flags: &mut f.ancillary.source_report_flags,
    };
    manager.allocate_native_warmup_cohort_arrays(
        &mut SourceInitHeatBalanceOwners {
            surface: &mut f.surface,
            geometry: &mut f.ancillary.surface_geometry_input,
            zones: &mut f.zones,
            control: &mut f.control,
            air: &mut f.air,
            radiation: &mut f.radiation,
            fan_zone: &mut f.fan,
            gains: &mut f.ancillary.selected_internal_heat_gains,
            gain_modules: &mut f.gain_modules,
            daylighting: &mut daylighting,
            window: &mut f.ancillary.window_optical_initialization,
            globals: &f.globals,
            clock: &f.weather.global,
            environment: &mut f.weather.environment,
            weather_owner: &f.weather.weather,
            shadowing: &shadowing,
            materials: &f.ancillary.selected_material_data,
            bounds: &f.startup.construction_bounds,
            input: &mut f.input,
            shortcut: &mut f.shortcut,
            report_inputs: &f.report,
            scan: &mut f.ancillary.scan,
            backend: &mut backend,
        },
        &mut providers,
    )?;
    Ok(AllocationOwners {
        _manager: manager,
        _ctf: ctf,
        _daylighting: daylighting,
        _shadowing: shadowing,
        _shading: shading,
    })
}

enum FullCall {
    Setpoints {
        prior: f64,
    },
    Predict {
        shorten: bool,
        zone_history: bool,
        prior: f64,
    },
    Correct {
        zone_history: bool,
    },
}
fn call(f: &mut Factory, operation: FullCall) -> Result<Option<f64>> {
    let mut backend = SourceErrorBackend {
        state: &mut f.error_state,
        error_stream: Some(&mut f.error),
        console: &mut f.console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let source_is_epjson = false;
    let mut owners = SourceZoneAirOwners {
        zones: &mut f.zones,
        equipment: &mut f.equipment,
        geometry: &f.ancillary.surface_geometry_input,
        surface_arrays: &f.surface.arrays,
        fan: &mut f.fan,
        gains: &f.ancillary.selected_internal_heat_gains,
        control: &mut f.control,
        space_controls: &f.space_controls,
        globals: &f.globals,
        clock: &f.weather.global,
        environment: &f.weather.environment,
        hvac: &mut f.startup.hvac_globals,
        input: &mut f.input,
        shortcut: &mut f.shortcut,
        source_is_epjson: &source_is_epjson,
        faults: &f.ancillary.source_faults_manager,
        schedules: &mut f.schedules,
        availability: &f.hvac_manager.ancillary.availability,
        psychrometrics: &mut f.psych,
        backend: &mut backend,
    };
    match operation {
        FullCall::Setpoints { prior } => {
            let mut dictionary = SourceOutputDictionaryStreams {
                eso: Some(&mut f.eso),
                sqlite_present: f.ancillary.source_sqlite.present(),
                results_framework_output_variables: &mut f.rf,
            };
            let mut outputs = SourceInitAirOutputHost {
                registration: &mut f.registration,
                prescan: &f.prescan,
                reports: &f.report,
                minimum_report_frequency: &f.project.system.minimum_report_frequency,
                host: &mut f.files,
                dictionary_host: &mut dictionary,
            };
            let mut unused_change = 0.0;
            f.controller.manage_source_zone_air_updates(
                SourceAirUpdate::GetZoneSetPoints,
                &mut unused_change,
                false,
                true,
                prior,
                &mut f.air,
                &mut owners,
                &mut outputs,
                &mut f.eio,
            )?;
            Ok(None)
        }
        FullCall::Predict {
            shorten,
            zone_history,
            prior,
        } => {
            f.air
                .zones
                .as_mut()
                .and_then(|r| r.get_mut(index(f.zone).ok()?))
                .ok_or_else(|| invalid("real full Predict row"))?
                .air
                .predict_native_multiplier_cohort(
                    f.zone,
                    shorten,
                    zone_history,
                    prior,
                    &mut f.controller,
                    &mut owners,
                )?;
            Ok(None)
        }
        FullCall::Correct { zone_history } => Ok(Some(
            f.air
                .zones
                .as_mut()
                .and_then(|r| r.get_mut(index(f.zone).ok()?))
                .ok_or_else(|| invalid("real full Correct row"))?
                .air
                .correct_native_multiplier_cohort(
                    f.zone,
                    zone_history,
                    &mut f.controller,
                    &mut owners,
                )?,
        )),
    }
}

fn bootstrap(
    definitions: &Value,
    definition: &Value,
    case: &Value,
) -> Result<(Box<Factory>, AllocationOwners)> {
    let mut f = Box::new(Factory::new(definitions, definition)?);
    let z = f
        .zones
        .zones
        .as_ref()
        .and_then(|r| r.get(index(f.zone).ok()?))
        .ok_or_else(|| invalid("real raw multiplier getter row"))?;
    if f.globals.num_of_zones != 1
        || f.globals.num_spaces != 1
        || z.multiplier != 2
        || z.list_multiplier != 1
    {
        return Err(
            invalid("raw Zone N6 getter-produced multiplier2, no unit-field assignment").into(),
        );
    }
    {
        let raw = hb::selected_heat_balance_input::SelectedHeatBalanceInput {
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
        let mut errors = false;
        let mut backend = SourceErrorBackend {
            state: &mut f.error_state,
            error_stream: Some(&mut f.error),
            console: &mut f.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        f.control.get_project_control_data(
            &raw,
            &mut HeatBalanceInputOwners {
                input: &mut f.input,
                shortcut: &mut f.shortcut,
                project: &mut f.project,
                space_controls: &mut f.space_controls,
                environment: &mut f.weather.environment,
                current_module_object: &mut f.ancillary.heat_balance_current_module_object,
            },
            &mut backend,
            &mut f.eio,
            &mut errors,
        )?;
        if errors {
            return Err(invalid("real ProjectControl getter accumulated error").into());
        }
    }
    let path = std::path::Path::new(text(member(member(case, "factory")?, "input_file")?)?);
    let allocation = allocate(&mut f, definition, path)?;
    f.controller
        .subsystems
        .room_air
        .get_source_model_parameters(
            &f.zones,
            &f.globals,
            &f.space_controls,
            &f.input,
            &mut f.shortcut,
            &mut f.eio,
        )?;
    primary_inputs(&mut f, member(case, "initial_caller_inputs")?)?;
    call(
        &mut f,
        FullCall::Setpoints {
            prior: real(member(case, "initial_prior_hours")?)?,
        },
    )?;
    if f.space_controls.current
        || f.controller.subsystems.room_air.any_non_mixing
        || f.control.zone_air_solution
            != hb::selected_heat_balance_input::SelectedZoneAirSolution::ThirdOrder
        || f.controller.flags.number_stage_zones != 0
        || f.controller.flags.number_on_off_zones != 0
    {
        return Err(invalid("genuine mixed ThirdOrder zone-only scope").into());
    }
    if !f
        .zones
        .spaces
        .as_ref()
        .ok_or_else(|| invalid("real default space"))?
        .iter()
        .all(|s| s.ht_surface_first == 0 && s.ht_surface_last == -1)
    {
        return Err(invalid("genuine geometry-not-loaded unit scope").into());
    }
    Ok((f, allocation))
}

pub(super) fn run(fixture: &Value) -> Result<usize> {
    let input = member(fixture, "source_inputs")?;
    assert_eq!(
        member(input, "schema")?,
        "src06-multiplier2-primary-Source-getter-input.v1"
    );
    let named = member(member(fixture, "named_inputs")?, "multiplier_predictor")?;
    let output = member(
        member(member(fixture, "native_output")?, "groups")?,
        "multiplier_predictor",
    )?;
    assert_eq!(
        member(named, "schema")?,
        "src06-multiplier2-full-predictor-corrector-input.v1"
    );
    assert_eq!(
        member(output, "schema")?,
        "src06-multiplier2-full-predictor-corrector-output.v1"
    );
    let cases = array(member(named, "cases")?)?;
    let exits = array(member(output, "cases")?)?;
    assert_eq!(cases.len(), 1);
    assert_eq!(exits.len(), 1);
    let mut calls = 0;
    let mut raw_bits = 0;
    for (case, expected) in cases.iter().zip(exits) {
        assert_eq!(member(case, "name")?, member(expected, "name")?);
        assert_eq!(member(case, "origin")?, member(expected, "origin")?);
        let (mut f, _allocation) = bootstrap(
            member(input, "definitions")?,
            member(input, "factory")?,
            case,
        )?;
        let mut ids = super::air_history::MultiplierObservations::default();
        ids.check(
            &state(&f)?,
            member(expected, "after_real_getter_and_initializer_chain")?,
            "multiplier.after_init",
        )?;
        let steps = array(member(case, "steps")?)?;
        let expected_steps = array(member(expected, "steps")?)?;
        assert_eq!(steps.len(), 21);
        assert_eq!(expected_steps.len(), 21);
        for (at, (step, expected)) in steps.iter().zip(expected_steps).enumerate() {
            assert_eq!(member(step, "name")?, member(expected, "name")?);
            assert_eq!(member(step, "operation")?, member(expected, "operation")?);
            assert_eq!(member(step, "inputs")?, member(expected, "named_inputs")?);
            primary_inputs(&mut f, member(step, "inputs")?)?;
            ids.check(
                &state(&f)?,
                member(expected, "entry")?,
                &format!("multiplier.step{}.entry", at + 1),
            )?;
            match text(member(step, "operation")?)? {
                "predict" => {
                    let a = member(step, "arguments")?;
                    assert_eq!(a, member(expected, "arguments")?);
                    call(
                        &mut f,
                        FullCall::Predict {
                            shorten: boolean(member(a, "shorten_system")?)?,
                            zone_history: boolean(member(a, "use_zone_history")?)?,
                            prior: real(member(a, "prior_hours")?)?,
                        },
                    )?;
                }
                "correct" => {
                    let a = member(step, "arguments")?;
                    assert_eq!(a, member(expected, "arguments")?);
                    let returned = call(
                        &mut f,
                        FullCall::Correct {
                            zone_history: boolean(member(a, "use_zone_history")?)?,
                        },
                    )?
                    .ok_or_else(|| invalid("full corrector returned value"))?;
                    ids.check(
                        &number(returned),
                        member(expected, "returned_temp_change")?,
                        &format!("multiplier.step{}.return", at + 1),
                    )?;
                }
                op @ ("push_zone" | "push_system" | "revert_zone") => {
                    super::air_history::multiplier_history_call(&mut f, op)?
                }
                _ => return Err(invalid("declared genuine full multiplier operation").into()),
            }
            ids.check(
                &state(&f)?,
                member(expected, "exit")?,
                &format!("multiplier.step{}.exit", at + 1),
            )?;
            calls += 1;
        }
        raw_bits += ids.raw_bits();
    }
    assert_eq!(calls, 21);
    assert_eq!(raw_bits, 17893);
    println!(
        "Multiplier2 genuine persistent full calls={calls}; strict raw-bit state/return observations={raw_bits}; raw getter multiplier2; no history/expected restore; private cache unobserved"
    );
    Ok(calls)
}
impl OrdinaryEquipment {
    pub(crate) fn run_native_gap_multiplier_predictor_cohort(fixture: &Value) -> Result<usize> {
        run(fixture)
    }
}
