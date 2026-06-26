    #[test]
    fn execution_plan_includes_thermostat_and_ideal_loads_steps() {
        let mut typed = TypedModel::default();
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("Control Type"),
            schedule_type_limits: None,
            hourly_value: 4.0,
        });
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(1),
            name: NormalizedName::new("Heating Setpoint"),
            schedule_type_limits: None,
            hourly_value: 21.0,
        });
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(2),
            name: NormalizedName::new("Cooling Setpoint"),
            schedule_type_limits: None,
            hourly_value: 24.0,
        });
        typed.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
        });
        typed
            .thermostat_dual_setpoints
            .push(ThermostatDualSetpoint {
                id: ThermostatSetpointId(0),
                name: NormalizedName::new("Dual Setpoints"),
                heating_setpoint_schedule: ScheduleId(1),
                cooling_setpoint_schedule: ScheduleId(2),
            });
        typed.zone_thermostats.push(ZoneThermostat {
            id: ZoneThermostatId(0),
            name: NormalizedName::new("Zone Thermostat"),
            zone: ZoneId(0),
            control_type_schedule: ScheduleId(0),
            controls: vec![ZoneThermostatControl {
                object_type: ThermostatControlObjectType::DualSetpoint,
                dual_setpoint: ThermostatSetpointId(0),
            }],
            temperature_difference_between_cutout_and_setpoint_delta_c: 0.0,
        });
        typed.ideal_loads_air_systems.push(IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("Zone Ideal Loads"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("Zone Inlet"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: None,
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        });
        typed.zone_equipment_lists.push(ZoneEquipmentList {
            id: ZoneEquipmentListId(0),
            name: NormalizedName::new("Zone Equipment"),
            load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
            equipment: vec![ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(0),
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            }],
        });
        typed
            .zone_equipment_connections
            .push(ZoneEquipmentConnection {
                id: ZoneEquipmentConnectionId(0),
                zone: ZoneId(0),
                equipment_list: ZoneEquipmentListId(0),
                zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("Zone Inlet")),
                zone_air_exhaust_node_or_nodelist_name: None,
                zone_air_node_name: NormalizedName::new("Zone Air Node"),
                zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("Zone Return")),
                zone_return_air_node_1_flow_rate_fraction_schedule: None,
                zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
            });
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);

        assert_eq!(model.graph.zone_thermostats.len(), 1);
        assert_eq!(model.graph.zone_ideal_loads.len(), 1);
        assert_eq!(plan.stages.len(), 25);
        assert_eq!(plan.compatibility_stages.len(), 25);
        assert!(plan.source_order_stages_match());
        assert_eq!(
            plan.expected_source_order_stage_ids(),
            plan.actual_source_order_stage_ids()
        );
        assert!(
            plan.expected_source_order_stage_ids()
                .contains(&"sim-purchased-air")
        );
        assert!(
            plan.expected_source_order_stage_ids()
                .contains(&"get-purchased-air")
        );
        assert!(
            plan.expected_source_order_stage_ids()
                .contains(&"calc-purch-air-loads")
        );

        let manage_zone_air_updates =
            stage_with_kind(&plan.stages, ExecutionStageKind::ManageZoneAirUpdates);
        assert_eq!(manage_zone_air_updates.steps.len(), 2);
        assert_eq!(
            manage_zone_air_updates.steps[0],
            ExecutionStep::EvaluateZoneThermostat(ZoneThermostatId(0))
        );
        assert_eq!(
            manage_zone_air_updates.steps[1],
            ExecutionStep::SolveZone(ZoneId(0))
        );

        let zone_equipment =
            stage_with_kind(&plan.stages, ExecutionStageKind::ZoneEquipmentManager);
        assert_eq!(zone_equipment.name, "zone-equipment-manager");
        assert_eq!(zone_equipment.steps.len(), 2);
        assert_eq!(
            zone_equipment.steps[0],
            ExecutionStep::ManageZoneEquipment(ZoneId(0))
        );
        assert_eq!(
            zone_equipment.steps[1],
            ExecutionStep::SimZoneEquipment(ZoneEquipmentListId(0))
        );

        let purchased_air_sim = stage_with_kind(&plan.stages, ExecutionStageKind::SimPurchasedAir);
        assert_eq!(
            purchased_air_sim.steps[0],
            ExecutionStep::SimPurchasedAir(IdealLoadsAirSystemId(0))
        );

        let purchased_air_get = stage_with_kind(&plan.stages, ExecutionStageKind::GetPurchasedAir);
        assert_eq!(
            purchased_air_get.steps[0],
            ExecutionStep::GetIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );

        let purchased_air_init =
            stage_with_kind(&plan.stages, ExecutionStageKind::InitPurchasedAir);
        assert_eq!(
            purchased_air_init.steps[0],
            ExecutionStep::InitIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );

        let purchased_air_calc =
            stage_with_kind(&plan.stages, ExecutionStageKind::CalcPurchAirLoads);
        assert_eq!(
            purchased_air_calc.steps[0],
            ExecutionStep::EvaluateIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );

        let purchased_air_update =
            stage_with_kind(&plan.stages, ExecutionStageKind::UpdatePurchasedAir);
        assert_eq!(
            purchased_air_update.steps[0],
            ExecutionStep::UpdateIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );

        let purchased_air_report =
            stage_with_kind(&plan.stages, ExecutionStageKind::ReportPurchasedAir);
        assert_eq!(
            purchased_air_report.steps[0],
            ExecutionStep::ReportIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );
    }

    #[test]
    fn ideal_loads_node_state_projection_expands_nodelist_and_writes_series()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = ideal_loads_node_state_model();

        let projection = simulate_ideal_loads_node_state_projection(
            &model,
            NodeStateProjectionOptions::hourly_samples(4),
        )?;

        assert_eq!(projection.summary.samples, 4);
        assert_eq!(projection.summary.node_count, 3);
        assert_eq!(projection.summary.series_count, 9);
        assert_eq!(projection.summary.state_node_count, 3);
        assert_eq!(
            projection.summary.evidence_policy.source_map_path,
            NODE_STATE_SOURCE_MAP_PATH
        );
        assert_eq!(
            projection.summary.evidence_policy.excluded_variable,
            NODE_STATE_EXCLUDED_SETPOINT_VARIABLE
        );
        assert_eq!(
            node_temperature_setpoint_from_energyplus(NODE_TEMPERATURE_SETPOINT_SENTINEL_C),
            None
        );
        assert_eq!(node_temperature_setpoint_from_energyplus(21.0), Some(21.0));
        assert_eq!(projection.state.len(), 3);
        assert_eq!(
            projection
                .summary
                .nodes
                .iter()
                .map(|node| (node.node_name.as_str(), node.role))
                .collect::<Vec<_>>(),
            vec![
                ("ZONE ONE INLET", NodeStateRole::Supply),
                ("ZONE ONE AIR NODE", NodeStateRole::ZoneAir),
                ("ZONE ONE RETURN", NodeStateRole::ReturnAir),
            ]
        );

        let inlet_temperature = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Temperature")
            .ok_or_else(|| std::io::Error::other("missing inlet temperature series"))?;
        assert_eq!(inlet_temperature.values, vec![50.0; 4]);

        let inlet_humidity = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Humidity Ratio")
            .ok_or_else(|| std::io::Error::other("missing inlet humidity series"))?;
        assert_eq!(inlet_humidity.values, vec![0.0156; 4]);

        let inlet_mass_flow = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Mass Flow Rate")
            .ok_or_else(|| std::io::Error::other("missing inlet mass flow series"))?;
        assert!(
            inlet_mass_flow
                .values
                .iter()
                .all(|value| (*value - 0.3).abs() < 1.0e-12)
        );
        let inlet_state = projection
            .state
            .find_by_key("ZONE ONE INLET")
            .ok_or_else(|| std::io::Error::other("missing inlet node state"))?;
        assert!((inlet_state.mass_flow_rate_kg_per_s - 0.3).abs() < 1.0e-12);
        assert!((inlet_state.temperature_c - 50.0).abs() < 1.0e-12);
        assert_eq!(inlet_state.temperature_setpoint_c, None);

        let zone_air_temperature = projection
            .results
            .find_series("ZONE ONE AIR NODE", "System Node Temperature")
            .ok_or_else(|| std::io::Error::other("missing zone air temperature series"))?;
        assert_eq!(zone_air_temperature.values, vec![23.0; 4]);
        let zone_air_state = projection
            .state
            .find_by_key("ZONE ONE AIR NODE")
            .ok_or_else(|| std::io::Error::other("missing zone air node state"))?;
        assert!((zone_air_state.humidity_ratio - 0.008).abs() < 1.0e-12);

        let return_mass_flow = projection
            .results
            .find_series("ZONE ONE RETURN", "System Node Mass Flow Rate")
            .ok_or_else(|| std::io::Error::other("missing return mass flow series"))?;
        assert!(
            return_mass_flow
                .values
                .iter()
                .all(|value| (*value - 0.3).abs() < 1.0e-12)
        );

        Ok(())
    }

    #[test]
    fn ideal_loads_node_state_projection_resolves_supply_zone_and_return_node_ids()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = ideal_loads_node_state_model();

        let projection = simulate_ideal_loads_node_state_projection(
            &model,
            NodeStateProjectionOptions::hourly_samples(1),
        )?;

        assert_eq!(
            projection
                .summary
                .nodes
                .iter()
                .map(|node| (node.node_id, node.node_name.as_str(), node.role))
                .collect::<Vec<_>>(),
            vec![
                (NodeId(0), "ZONE ONE INLET", NodeStateRole::Supply),
                (NodeId(1), "ZONE ONE AIR NODE", NodeStateRole::ZoneAir),
                (NodeId(2), "ZONE ONE RETURN", NodeStateRole::ReturnAir),
            ]
        );
        assert_eq!(
            projection
                .state
                .find_by_key("zone one inlet")
                .unwrap()
                .node_id,
            NodeId(0)
        );
        assert_eq!(
            projection
                .state
                .find_by_key("Zone One Air Node")
                .unwrap()
                .node_id,
            NodeId(1)
        );
        assert_eq!(
            projection
                .state
                .find_by_id(NodeId(2))
                .unwrap()
                .node_name
                .as_str(),
            "ZONE ONE RETURN"
        );

        Ok(())
    }

    #[test]
    fn node_state_store_initializes_without_ideal_loads_result_structs() {
        let mut typed = TypedModel::default();
        let node_id = push_node(&mut typed, "Standalone Air Node");

        let state = NodeStateStore::from_typed_model(&typed, 21.5, 0.0085);

        assert!(typed.ideal_loads_air_systems.is_empty());
        assert_eq!(state.len(), 1);
        let node = state.find_by_key("standalone air node").unwrap();
        assert_eq!(node.node_id, node_id);
        assert_eq!(node.temperature_c, 21.5);
        assert_eq!(node.humidity_ratio, 0.0085);
        assert_eq!(node.mass_flow_rate_kg_per_s, 0.0);
    }

    fn ideal_loads_node_state_model() -> SimulationModel {
        let mut typed = TypedModel::default();
        typed.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
        });
        typed.nodes.push(Node {
            id: NodeId(0),
            name: NormalizedName::new("Zone One Inlet"),
        });
        typed.nodes.push(Node {
            id: NodeId(1),
            name: NormalizedName::new("Zone One Air Node"),
        });
        typed.nodes.push(Node {
            id: NodeId(2),
            name: NormalizedName::new("Zone One Return"),
        });
        typed.node_names.insert("Zone One Inlet", NodeId(0));
        typed.node_names.insert("Zone One Air Node", NodeId(1));
        typed.node_names.insert("Zone One Return", NodeId(2));
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: NormalizedName::new("Zone One Inlets"),
            nodes: vec![NodeId(0)],
        });
        typed
            .node_list_names
            .insert("Zone One Inlets", NodeListId(0));
        typed.ideal_loads_air_systems.push(IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("Zone One Ideal Loads"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("Zone One Inlets"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.25)),
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        });
        typed.zone_equipment_lists.push(ZoneEquipmentList {
            id: ZoneEquipmentListId(0),
            name: NormalizedName::new("Zone One Equipment"),
            load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
            equipment: vec![ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(0),
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            }],
        });
        typed
            .zone_equipment_connections
            .push(ZoneEquipmentConnection {
                id: ZoneEquipmentConnectionId(0),
                zone: ZoneId(0),
                equipment_list: ZoneEquipmentListId(0),
                zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("Zone One Inlets")),
                zone_air_exhaust_node_or_nodelist_name: None,
                zone_air_node_name: NormalizedName::new("Zone One Air Node"),
                zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("Zone One Return")),
                zone_return_air_node_1_flow_rate_fraction_schedule: None,
                zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
            });

        SimulationModel::from_typed(typed)
    }

    fn push_node(model: &mut TypedModel, name: &str) -> NodeId {
        let id = NodeId(model.nodes.len() as u32);
        model.nodes.push(Node {
            id,
            name: NormalizedName::new(name),
        });
        model.node_names.insert(name, id);
        id
    }

    #[test]
    fn parses_epw_records_after_header() -> Result<(), Box<dyn std::error::Error>> {
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
1999,1,1,1,0,Source,-3.0,-4.0,50,82000,0,0,300,10,20,30,0,0,0,0,180,2.5
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6,0,0,0,0,0,0,0,0,0,0,0,2.0,1.0
"#,
        )?;

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].dry_bulb_c, -3.0);
        assert_eq!(records[0].dew_point_c, -4.0);
        assert_eq!(records[0].relative_humidity_percent, 50.0);
        assert_eq!(records[0].atmospheric_pressure_pa, 82_000.0);
        assert_eq!(records[0].wind_direction_deg, 180.0);
        assert_eq!(records[0].wind_speed_m_per_s, 2.5);
        assert_eq!(records[0].liquid_precipitation_depth_mm, 0.0);
        assert_eq!(records[1].liquid_precipitation_depth_mm, 2.0);

        Ok(())
    }

    #[test]
    fn parses_epw_dry_bulb_values_after_header() -> Result<(), Box<dyn std::error::Error>> {
        let values = parse_epw_dry_bulb_series(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
1999,1,1,1,0,Source,-3.0,-4.0,50,82000,0,0,300,10,20,30,0,0,0,0,180,2.5
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6
"#,
        )?;

        assert_eq!(values, vec![-3.0, -2.0]);

        Ok(())
    }

    #[test]
    fn surface_area_handles_3d_rectangles() {
        let vertices = vec![
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 2.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 2.0,
                y_m: 0.0,
                z_m: 3.0,
            },
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 3.0,
            },
        ];

        assert_eq!(surface_area_m2(&vertices), 6.0);
    }

    #[test]
    fn zone_geometry_summary_reports_cube_metrics() {
        let summaries = zone_geometry_summaries(&cube_model());

        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].zone_name, "ZONE ONE");
        assert_eq!(summaries[0].surface_count, 6);
        assert_eq!(summaries[0].floor_area_m2, 1.0);
        assert_eq!(summaries[0].volume_m3, Some(1.0));
        assert_eq!(summaries[0].exterior_wall_area_m2, 4.0);
    }

    #[test]
    fn surface_geometry_summary_reports_cube_orientation() -> Result<(), Box<dyn std::error::Error>>
    {
        let summaries = surface_geometry_summaries(&cube_model());

        assert_eq!(summaries.len(), 6);
        let floor = summaries
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        assert_eq!(floor.zone_name, "ZONE ONE");
        assert_eq!(floor.surface_type, SurfaceType::Floor);
        assert_eq!(floor.area_m2, 1.0);
        assert!((floor.azimuth_deg - 270.0).abs() < 1.0e-9);
        assert!((floor.tilt_deg - 180.0).abs() < 1.0e-9);

        let roof = summaries
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        assert_eq!(roof.surface_type, SurfaceType::Roof);
        assert_eq!(roof.area_m2, 1.0);
        assert!((roof.azimuth_deg - 0.0).abs() < 1.0e-9);
        assert!((roof.tilt_deg - 0.0).abs() < 1.0e-9);

        let wall_azimuths = [
            ("WALL X0", 90.0),
            ("WALL X1", 270.0),
            ("WALL Y0", 0.0),
            ("WALL Y1", 180.0),
        ];
        for (surface_name, azimuth_deg) in wall_azimuths {
            let wall = summaries
                .iter()
                .find(|surface| surface.surface_name == surface_name)
                .ok_or_else(|| std::io::Error::other(format!("missing {surface_name} surface")))?;
            assert_eq!(wall.surface_type, SurfaceType::Wall);
            assert_eq!(wall.area_m2, 1.0);
            assert!((wall.azimuth_deg - azimuth_deg).abs() < 1.0e-9);
            assert!((wall.tilt_deg - 90.0).abs() < 1.0e-9);
        }

        Ok(())
    }

    #[test]
    fn single_system_timestep_syncs_adaptive_history() -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone = &mut state.zones[0];
        zone.mean_air_temperature_c = 20.1;
        zone.air_humidity_ratio = 0.004;
        zone.previous_mean_air_temperatures_c = [20.0, 19.0, 18.0];
        zone.previous_air_humidity_ratios = [0.003, 0.002, 0.001];
        zone.previous_system_mean_air_temperatures_c = [9.0, 8.0, 7.0];
        zone.previous_system_air_humidity_ratios = [0.009, 0.008, 0.007];
        zone.previous_system_timestep_count = 4;

        apply_energyplus_adaptive_system_timestep_zone_air_correction(
            &state.surfaces,
            &mut state.zones,
            900.0,
            None,
            20.0,
            false,
        );

        let zone = &state.zones[0];
        assert_eq!(zone.previous_system_timestep_count, 1);
        assert_eq!(
            zone.previous_system_mean_air_temperatures_c,
            [zone.mean_air_temperature_c, 20.0, 19.0]
        );
        assert_eq!(
            zone.previous_system_air_humidity_ratios,
            [zone.air_humidity_ratio, 0.003, 0.002]
        );
        assert_eq!(
            zone.zone_timestep_average_air_temperature_c,
            zone.mean_air_temperature_c
        );
        assert_eq!(
            zone.zone_timestep_average_air_humidity_ratio,
            zone.air_humidity_ratio
        );

        Ok(())
    }
