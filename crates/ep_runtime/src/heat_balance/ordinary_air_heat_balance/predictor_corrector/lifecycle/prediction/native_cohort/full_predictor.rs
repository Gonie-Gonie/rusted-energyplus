//! Test-only full parent replay. Each retained engine call is independent.
use super::*;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;

include!("../fixtures/native_full_predictor_b.rs");

fn restore_external_inputs(
    zones: &mut SelectedZoneInputOwners,
    geometry: &mut SurfaceGeometryInputState,
    arrays: &mut SourceSurfaceHeatBalanceArrays,
    gains: &mut SelectedInternalHeatGains,
    spaces: &Value,
    first: bool,
) -> Result {
    if first {
        return restore_corrector_surfaces(zones, geometry, arrays, gains, spaces);
    }
    // The streaming admission proved this exact graph is fixed across all 1168
    // parent calls. Retain allocations and restore only outside-parent inputs.
    for (at, raw) in array(spaces)?.iter().enumerate() {
        assert_eq!(integer(member(raw, "index")?)?, i32::try_from(at + 1)?);
        let v = member(raw, "value")?;
        let space = zones
            .spaces
            .as_mut()
            .and_then(|r| r.get_mut(at))
            .ok_or_else(|| invalid("same actual Space allocation"))?;
        space.volume = real(member(v, "Volume")?)?;
        space.fraction_zone_volume = real(member(v, "fracZoneVolume")?)?;
        space.system_zone_node_number = integer(member(v, "SystemZoneNodeNumber")?)?;
        assert_eq!(
            space.ht_surface_first,
            integer(member(v, "HTSurfaceFirst")?)?
        );
        assert_eq!(space.ht_surface_last, integer(member(v, "HTSurfaceLast")?)?);
        for raw in array(member(v, "surfaces")?)? {
            let at = usize::try_from(integer(member(raw, "index")?)? - 1)?;
            let v = member(raw, "value")?;
            let surface = geometry
                .final_surfaces
                .as_ref()
                .and_then(|r| r.get(at))
                .ok_or_else(|| invalid("same actual opaque Surface"))?;
            assert_real(
                surface.row.area,
                member(v, "Area")?,
                "parent input",
                "unchanged Surface.Area",
            )?;
            for (field, target) in [
                ("SurfHConvInt", &mut arrays.surf_h_conv_int),
                ("SurfTempInTmp", &mut arrays.surf_temp_in_tmp),
                ("SurfTempEffBulkAir", &mut arrays.surf_temp_eff_bulk_air),
            ] {
                *target
                    .as_mut()
                    .and_then(|r| r.get_mut(at))
                    .ok_or_else(|| invalid("same actual surface parallel array"))? =
                    real(member(member(v, field)?, "value")?)?;
            }
        }
        let raw_gain = member(member(v, "gains")?, "value")?;
        let owner = gains
            .allocations
            .space_devices
            .as_mut()
            .and_then(|r| r.get_mut(at))
            .ok_or_else(|| invalid("same actual gain allocation"))?;
        assert_eq!(
            owner.number_of_devices,
            count(member(raw_gain, "numberOfDevices")?)?
        );
        assert_eq!(
            owner.max_number_of_devices,
            count(member(raw_gain, "maxNumberOfDevices")?)?
        );
        for (number, raw) in array(member(member(raw_gain, "device")?, "values")?)?
            .iter()
            .enumerate()
        {
            let device = owner
                .devices
                .as_mut()
                .and_then(|r| r.get_mut(number))
                .ok_or_else(|| invalid("same actual active gain"))?;
            assert_eq!(device.component_name, text(member(raw, "CompObjectName")?)?);
            assert_eq!(device.component_type, integer(member(raw, "CompType")?)?);
            assert_eq!(
                device.return_air_node,
                integer(member(raw, "ReturnAirNodeNum")?)?
            );
            device.cached_rates[0] = real(member(raw, "ConvectGainRate")?)?;
            device.cached_rates[1] = real(member(raw, "ReturnAirConvGainRate")?)?;
        }
    }
    Ok(())
}

fn check_external_inputs(
    owners: &SourceZoneAirOwners<'_, '_>,
    spaces: &Value,
    context: &str,
) -> Result {
    for (at, raw) in array(spaces)?.iter().enumerate() {
        let v = member(raw, "value")?;
        let space = owners
            .zones
            .spaces
            .as_ref()
            .and_then(|r| r.get(at))
            .ok_or_else(|| invalid("same Space after parent"))?;
        assert_real(
            space.volume,
            member(v, "Volume")?,
            context,
            "Space.Volume readonly",
        )?;
        assert_real(
            space.fraction_zone_volume,
            member(v, "fracZoneVolume")?,
            context,
            "Space fraction readonly",
        )?;
        for raw in array(member(v, "surfaces")?)? {
            let at = usize::try_from(integer(member(raw, "index")?)? - 1)?;
            let v = member(raw, "value")?;
            for (field, target) in [
                ("SurfHConvInt", &owners.surface_arrays.surf_h_conv_int),
                ("SurfTempInTmp", &owners.surface_arrays.surf_temp_in_tmp),
                (
                    "SurfTempEffBulkAir",
                    &owners.surface_arrays.surf_temp_eff_bulk_air,
                ),
            ] {
                assert_real(
                    *target
                        .as_ref()
                        .and_then(|r| r.get(at))
                        .ok_or_else(|| invalid("surface array after parent"))?,
                    member(member(v, field)?, "value")?,
                    context,
                    field,
                )?;
            }
        }
        let raw_gain = member(member(v, "gains")?, "value")?;
        let owner = owners
            .gains
            .allocations
            .space_devices
            .as_ref()
            .and_then(|r| r.get(at))
            .ok_or_else(|| invalid("gain owner after parent"))?;
        for (number, raw) in array(member(member(raw_gain, "device")?, "values")?)?
            .iter()
            .enumerate()
        {
            let device = owner
                .devices
                .as_ref()
                .and_then(|r| r.get(number))
                .ok_or_else(|| invalid("active gain after parent"))?;
            assert_real(
                device.cached_rates[0],
                member(raw, "ConvectGainRate")?,
                context,
                "saved convective gain readonly",
            )?;
            assert_real(
                device.cached_rates[1],
                member(raw, "ReturnAirConvGainRate")?,
                context,
                "saved return gain readonly",
            )?;
        }
    }
    Ok(())
}

pub(super) fn run() -> Result<usize> {
    let fixture: Value = serde_json::from_str(NATIVE_FULL_PREDICTOR_JSON)?;
    let cases = array(member(&fixture, "cases")?)?;
    assert_eq!(cases.len(), 1168);
    let mut input = input(&fixture)?;
    // Real absent-family getter on retained genuine B raw input. This does not
    // execute the entire thermostat getter or claim its lifecycle as tested.
    assert_eq!(input.get_num_objects_found("ZoneControl:Humidistat"), 0);
    assert_eq!(input.get_num_objects_found("RoomAirModelType"), 0);
    let inventory = object(member(&fixture, "raw")?)?
        .iter()
        .map(|(family, objects)| Ok((family.as_str(), object(objects)?.len())))
        .collect::<Result<Vec<_>>>()?;
    let equipment_input = OrdinaryEquipmentInput::from_source_family_inventory(inventory, 4)?;
    let model = identity_model();
    let mut equipment = OrdinaryEquipment::new_for_source(&model, equipment_input)?;
    let mut globals = SelectedHeatBalanceGlobals::default();
    globals.num_of_zones = 1;
    globals.num_spaces = 1;
    let mut zones = SelectedZoneInputOwners::default();
    zones.zones = Some(vec![SelectedZoneData {
        name: "ZONE ONE".into(),
        typed_zone_id: Some(ep_model::ZoneId(0)),
        space_indexes: vec![1],
        num_spaces: 1,
        ..Default::default()
    }]);
    let mut schedules = SelectedScheduleManager::default();
    schedules.init_constant_state(4)?;
    let mut sizing = SelectedSizingManager::default();
    let mut branch = SourceBranchNodeConnections::default();
    let mut gains = SelectedInternalHeatGains::default();
    equipment.get_zone_equipment_source(
        &mut input,
        equipment_input,
        false,
        false,
        &globals,
        &mut gains,
        SourceEquipmentInputOwners {
            zones: zones
                .zones
                .as_deref_mut()
                .ok_or_else(|| invalid("Zone allocation"))?,
            schedules: &mut schedules,
            sizing: &mut sizing,
            branch: Some(&mut branch),
            source_is_epjson: false,
        },
    )?;
    let mut controller = SourceZoneAirPredictorCorrector::default();
    let space_controls = SelectedHeatBalanceSpaceControls::default();
    controller
        .demand
        .allocate_source_demands(&globals, &space_controls)?;
    let mut air_owner = SourceAirHeatBalance::default();
    air_owner.allocate_source_arrays(&globals)?;
    let mut fan = crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays::default();
    let mut geometry = SurfaceGeometryInputState::default();
    let mut surface_arrays = SourceSurfaceHeatBalanceArrays::default();
    let mut control = SelectedHeatBalanceInputState::default();
    let faults = crate::heat_balance::source_initialization::SourceFaultsManager::default();
    let availability = crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability::default();
    let mut shortcut = SourceInputProcessorShortCut::default();
    let mut psychrometrics = SourcePsychrometricAccess::default();
    let mut state = error_state();
    let mut console = Vec::new();
    let mut backend = SourceErrorBackend {
        state: &mut state,
        error_stream: None,
        console: &mut console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    controller.subsystems.room_air.manage_source_air_model(
        1,
        &mut input,
        &mut shortcut,
        &mut backend,
    )?;
    let source_is_epjson = false;
    let mut recurring_errors =
        crate::heat_balance::source_initialization::SourceRecurringErrorState::default();
    let mut stable_addresses = None;
    let mut shortened = 0;
    // No property-state reset between calls. Native cache state was unobserved;
    // this fresh process-main selector checks normal numeric results only.
    for (at, case) in cases.iter().enumerate() {
        assert_eq!(count(member(case, "invocation")?)?, at + 1);
        let context = format!("genuine full predictor parent{}", at + 1);
        let entry = snapshot(&fixture, member(case, "entry")?)?;
        let expected = snapshot(&fixture, member(case, "exit")?)?;
        let aliases = member(member(&entry, "actual_shapes")?, "aliases")?;
        assert!(boolean(member(aliases, "is_zone_row")?)?);
        assert!(member(aliases, "is_space_row")?.is_null());
        for (key, rows) in [
            ("zoneHeatBalance", air_owner.zones.as_ref().map(Vec::len)),
            ("spaceHeatBalance", air_owner.spaces.as_ref().map(Vec::len)),
        ] {
            let shape = member(aliases, key)?;
            assert!(boolean(member(shape, "allocated")?)?);
            assert_eq!(rows, Some(count(member(shape, "size")?)?));
        }
        let spaces = array(member(member(&fixture, "state_tables")?, "sums_inputs")?)?
            .get(count(member(case, "sums_inputs")?)?)
            .ok_or_else(|| invalid("real nested Sums inputs"))?;
        restore_external_inputs(
            &mut zones,
            &mut geometry,
            &mut surface_arrays,
            &mut gains,
            spaces,
            at == 0,
        )?;
        let flags = member(&entry, "flags")?;
        assert!(!boolean(member(flags, "doSpaceHeatBalance")?)?);
        assert!(!boolean(member(flags, "DoLatentSizing")?)?);
        assert!(!boolean(member(flags, "FlagHybridModel_PC")?)?);
        assert!(!boolean(member(flags, "anyNonMixingRoomAirModel")?)?);
        assert_eq!(integer(member(flags, "NumStageCtrZone")?)?, 0);
        assert_eq!(integer(member(flags, "AFN_control_type")?)?, 0);
        assert!(!boolean(member(flags, "multizone_always_simulated")?)?);
        assert!(!boolean(member(flags, "distribution_simulated")?)?);
        assert!(!boolean(member(flags, "AirflowNetworkFanActivated")?)?);
        assert!(!globals.hybrid_model_people_count);
        assert!(!control.do_latent_sizing);
        assert!(
            !controller
                .subsystems
                .airflow_network
                .multizone_always_simulated
        );
        assert!(!controller.subsystems.airflow_network.distribution_simulated);
        assert_eq!(controller.subsystems.airflow_network.control_type, 0);
        assert!(!controller.subsystems.airflow_network.fan_activated);
        let c = member(&entry, "clock")?;
        assert!(!boolean(member(c, "DoingSizing")?)?);
        let mut clock = WeatherGlobalState::default();
        clock.begin_sim_flag = boolean(member(c, "BeginSimFlag")?)?;
        let mut environment = WeatherEnvironmentState::default();
        environment.current_weather.out_baro_press = real(member(c, "OutBaroPress")?)?;
        environment.current_weather.out_hum_rat = real(member(c, "OutHumRat")?)?;
        let mut hvac = SelectedHvacGlobals::default();
        hvac.time_step_sys_seconds = real(member(c, "TimeStepSysSec")?)?;
        // Deliberate test-only output canary, NOT a captured input/Native bit.
        hvac.shorten_time_step_sys_room_air = true;
        let air = &mut air_owner
            .zones
            .as_mut()
            .and_then(|r| r.first_mut())
            .ok_or_else(|| invalid("sole real allocated zoneHeatBalance(1)"))?
            .air;
        let mut owners = SourceZoneAirOwners {
            zones: &mut zones,
            equipment: &mut equipment,
            geometry: &geometry,
            surface_arrays: &surface_arrays,
            fan: &mut fan,
            gains: &gains,
            control: &mut control,
            space_controls: &space_controls,
            globals: &globals,
            clock: &clock,
            environment: &environment,
            hvac: &mut hvac,
            input: &mut input,
            shortcut: &mut shortcut,
            source_is_epjson: &source_is_epjson,
            faults: &faults,
            schedules: &mut schedules,
            availability: &availability,
            psychrometrics: &mut psychrometrics,
            backend: &mut backend,
            recurring_errors: &mut recurring_errors,
        };
        seed_state(air, &mut controller, &mut owners, &entry)?;
        seed_corrector_air(air, member(&entry, "air")?)?;
        // These three operation outputs are intentionally NOT supplied from a
        // child checkpoint or expected exit. The real parent must generate all.
        air.air_power_cap = 0.0;
        air.temp_dep_load = 0.0;
        air.temp_ind_load = 0.0;
        let z = member(&entry, "zone")?;
        let zone = owners
            .zones
            .zones
            .as_mut()
            .and_then(|r| r.first_mut())
            .ok_or_else(|| invalid("same Zone"))?;
        assert_eq!(
            zone.humidity_control_zone_index, 0,
            "real constructor/no input writer"
        );
        seed_reals!(zone,z,{"Volume"=>volume,"ZoneVolCapMultpSens"=>sensible_capacity_multiplier,
            "ZoneVolCapMultpMoist"=>moisture_capacity_multiplier,"OutDryBulbTemp"=>dry_bulb});
        zone.no_heat_to_return_air = boolean(member(z, "NoHeatToReturnAir")?)?;
        zone.is_return_plenum = boolean(member(z, "IsReturnPlenum")?)?;
        zone.is_supply_plenum = boolean(member(z, "IsSupplyPlenum")?)?;
        zone.space_indexes = array(member(z, "spaceIndexes")?)?
            .iter()
            .map(integer)
            .collect::<Result<Vec<_>>>()?;
        let f = member(&entry, "fan")?;
        owners.fan.sum_conv_ht_rad_sys = Some(vec![real(member(f, "SumConvHTRadSys")?)?]);
        owners.fan.sum_conv_pool = Some(vec![real(member(f, "SumConvPool")?)?]);
        let node_number = integer(member(&entry, "node_ordinal")?)?;
        owners.equipment.restore_native_cohort_node(
            node_number,
            text(member(&entry, "node_name")?)?,
            captured_node_values(member(&entry, "node")?)?,
        )?;
        let inlet_rows = array(member(
            member(member(&entry, "config")?, "InletNode")?,
            "values",
        )?)?;
        let cfg = owners.equipment.source_zone_equipment_config(1)?;
        assert_eq!(inlet_rows.len(), *cfg.number_inlet_nodes());
        for raw in inlet_rows {
            assert_eq!(
                cfg.inlet_node_number(count(member(raw, "slot")?)?).copied(),
                Some(usize::try_from(integer(member(raw, "NodeNum")?)?)?)
            );
        }
        for raw in inlet_rows {
            let p = member(raw, "physical")?;
            owners.equipment.restore_native_cohort_node(
                integer(member(p, "index")?)?,
                text(member(p, "NodeID")?)?,
                captured_node_values(member(p, "value")?)?,
            )?;
        }
        let moisture = controller
            .demand
            .zone_moisture
            .as_mut()
            .and_then(|r| r.first_mut())
            .ok_or_else(|| invalid("real allocated moisture demand row"))?;
        // Three explicit output canaries, not reconstructed Native observations.
        moisture.base.total_output_required = 17.0;
        moisture.output_required_to_dehumidifying_sp = -19.0;
        moisture.output_required_to_humidifying_sp = 23.0;
        let addresses = [
            std::ptr::from_ref(&*air) as usize,
            {
                let n = owners.equipment.source_zone_air_node_mut(node_number)?;
                std::ptr::from_ref(&*n.temperature) as usize
            },
            std::ptr::from_ref(
                controller
                    .demand
                    .zones
                    .as_ref()
                    .and_then(|r| r.first())
                    .ok_or_else(|| invalid("same sensible row"))?,
            ) as usize,
            std::ptr::from_ref(
                controller
                    .demand
                    .zone_moisture
                    .as_ref()
                    .and_then(|r| r.first())
                    .ok_or_else(|| invalid("same moisture row"))?,
            ) as usize,
        ];
        if let Some(first) = stable_addresses {
            assert_eq!(
                addresses, first,
                "same real owner graph across independent cases"
            );
        } else {
            stable_addresses = Some(addresses);
        }
        let args = member(case, "arguments")?;
        let shorten = bool_integer(member(args, "shortenTimeStepSys")?)?;
        if shorten {
            shortened += 1;
        }
        air.predict_source_system_load(
            1,
            shorten,
            bool_integer(member(args, "useZoneTimeStepHistory")?)?,
            real(member(args, "priorTimeStep")?)?,
            &mut controller,
            &mut owners,
        )?;
        check_state(air, &controller, &mut owners, &expected, &context)?;
        check_corrector_air(air, member(&expected, "air")?, &context)?;
        check_external_inputs(&owners, spaces, &context)?;
        let z_after = owners
            .zones
            .zones
            .as_ref()
            .and_then(|r| r.first())
            .ok_or_else(|| invalid("same Zone after parent"))?;
        assert_real(
            z_after.volume,
            member(z, "Volume")?,
            &context,
            "Zone.Volume readonly",
        )?;
        assert_real(
            z_after.sensible_capacity_multiplier,
            member(z, "ZoneVolCapMultpSens")?,
            &context,
            "Zone cap readonly",
        )?;
        assert_real(
            z_after.dry_bulb,
            member(z, "OutDryBulbTemp")?,
            &context,
            "Zone outdoor temperature readonly",
        )?;
        assert_eq!(z_after.humidity_control_zone_index, 0);
        assert!(
            !owners.hvac.shorten_time_step_sys_room_air,
            "separate Native3212 literal postcondition"
        );
        let moisture = controller
            .demand
            .zone_moisture
            .as_ref()
            .and_then(|r| r.first())
            .ok_or_else(|| invalid("same moisture exit row"))?;
        assert_eq!(
            moisture.base.total_output_required.to_bits(),
            0.0_f64.to_bits(),
            "separate Native3810 literal"
        );
        assert_eq!(
            moisture.output_required_to_dehumidifying_sp.to_bits(),
            0.0_f64.to_bits(),
            "separate Native3811 literal"
        );
        assert_eq!(
            moisture.output_required_to_humidifying_sp.to_bits(),
            0.0_f64.to_bits(),
            "separate Native3812 literal"
        );
        assert_eq!(std::ptr::from_ref(&*air) as usize, addresses[0]);
        let node = owners.equipment.source_zone_air_node_mut(node_number)?;
        assert_eq!(
            std::ptr::from_ref(&*node.temperature) as usize,
            addresses[1]
        );
        assert_real(
            *node.mass_flow_rate,
            member(member(&expected, "node")?, "MassFlowRate")?,
            &context,
            "ZoneNode.Mdot readonly",
        )?;
        for raw in inlet_rows {
            let p = member(raw, "physical")?;
            let node = owners
                .equipment
                .source_zone_air_node_mut(integer(member(p, "index")?)?)?;
            let v = member(p, "value")?;
            for (key, actual) in [
                ("Temp", *node.temperature),
                ("MassFlowRate", *node.mass_flow_rate),
                ("HumRat", *node.humidity_ratio),
                ("Enthalpy", *node.enthalpy),
                ("TempSetPoint", *node.temperature_setpoint),
            ] {
                assert_real(actual, member(v, key)?, &context, "readonly actual inlet")?;
            }
        }
    }
    assert_eq!(shortened, 77);
    assert!(
        console.is_empty(),
        "normal full parent should not emit diagnostic errors"
    );
    Ok(cases.len())
}
