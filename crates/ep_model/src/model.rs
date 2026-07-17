//! Aggregate typed model and graph structures.

use crate::{
    AirLoopHvac, AvailabilityManagerComponent, BoilerHotWater, BranchId, BranchListId, Building,
    ChillerElectricEir, CoilComponent, ComponentId, ConnectorId, ConnectorListId, Construction,
    ConstructionId, DayScheduleId, DesignSpecificationOutdoorAir, DesignSpecificationOutdoorAirId,
    ExternalInterfaceFmuExportSchedule, ExternalInterfaceFmuImportSchedule,
    ExternalInterfaceSchedule, FanComponent, GlazingSpectralData, GlazingSpectralDataId,
    GlobalGeometryRules, IdealLoadsAirSystem, IdealLoadsAirSystemId, InternalGainId, LoopId,
    Material, MaterialHeatAndMoistureTransferRedistribution,
    MaterialHeatAndMoistureTransferSettings, MaterialHeatAndMoistureTransferSorptionIsotherm,
    MaterialHeatAndMoistureTransferSuction, MaterialId, MaterialMoisturePenetrationDepthSettings,
    MaterialPhaseChange, MaterialPhaseChangeHysteresis, MaterialVariableAbsorptance,
    MaterialVariableAbsorptanceId, MaterialVariableThermalConductivity, NameMap, Node, NodeId,
    NodeList, NodeListId, NormalizedName, OtherEquipment, People, PlantBranch, PlantBranchList,
    PlantConnector, PlantConnectorKind, PlantConnectorList, PlantLoop, PumpConstantSpeed,
    RunPeriod, RunPeriodDaylightSavingTime, RunPeriodId, RunPeriodSpecialDay,
    RunPeriodSpecialDayId, ScheduleCompact, ScheduleConstant, ScheduleDayHourly,
    ScheduleDayInterval, ScheduleDayList, ScheduleFile, ScheduleFileShading, ScheduleId,
    ScheduleTypeLimitId, ScheduleTypeLimits, ScheduleWeekCompact, ScheduleWeekDaily, ScheduleYear,
    SetpointManagerComponent, SiteLocation, Surface, SurfaceConvectionAlgorithms, SurfaceId,
    ThermostatDualSetpoint, ThermostatSetpointId, TimestepConfig, Version, WeekScheduleId,
    WindowGlazingThermochromicGroupMaterial, WindowGlazingThermochromicState, Zone,
    ZoneEquipmentConnection, ZoneEquipmentList, ZoneEquipmentListId, ZoneEquipmentObjectType,
    ZoneHumidistat, ZoneHumidistatId, ZoneId, ZoneThermostat, ZoneThermostatId,
};

/// Minimal typed model for early compiler stages.
#[derive(Clone, Debug, PartialEq)]
pub struct TypedModel {
    /// Model version.
    pub version: Version,
    /// Building settings.
    pub building: Option<Building>,
    /// Global surface geometry input rules, when declared.
    pub global_geometry_rules: Option<GlobalGeometryRules>,
    /// Zone timestep config.
    pub timestep: TimestepConfig,
    /// Global surface convection algorithm settings.
    pub surface_convection_algorithms: SurfaceConvectionAlgorithms,
    /// Run periods.
    pub run_periods: Vec<RunPeriod>,
    /// Run period names.
    pub run_period_names: NameMap<RunPeriodId>,
    /// Input-file holidays and other special schedule days.
    pub run_period_special_days: Vec<RunPeriodSpecialDay>,
    /// Run-period special-day names.
    pub run_period_special_day_names: NameMap<RunPeriodSpecialDayId>,
    /// Unique input-file daylight-saving period, when declared.
    pub run_period_daylight_saving_time: Option<RunPeriodDaylightSavingTime>,
    /// Site location.
    pub site: Option<SiteLocation>,
    /// Standalone glazing spectral datasets.
    pub glazing_spectral_data: Vec<GlazingSpectralData>,
    /// Glazing spectral dataset names in their separate namespace.
    pub glazing_spectral_data_names: NameMap<GlazingSpectralDataId>,
    /// Materials.
    pub materials: Vec<Material>,
    /// Material names.
    pub material_names: NameMap<MaterialId>,
    /// Variable thermal/solar absorptance overlays.
    pub material_variable_absorptances: Vec<MaterialVariableAbsorptance>,
    /// Variable-absorptance overlay names in their separate namespace.
    pub material_variable_absorptance_names: NameMap<MaterialVariableAbsorptanceId>,
    /// Hysteretic phase-change attachments keyed by their referenced material names.
    pub material_phase_change_hystereses: Vec<MaterialPhaseChangeHysteresis>,
    /// CondFD temperature-enthalpy attachments keyed by referenced material names.
    pub material_phase_changes: Vec<MaterialPhaseChange>,
    /// CondFD temperature-conductivity attachments keyed by referenced material names.
    pub material_variable_thermal_conductivities: Vec<MaterialVariableThermalConductivity>,
    /// EMPD settings attachments keyed by referenced material names.
    pub material_moisture_penetration_depth_settings: Vec<MaterialMoisturePenetrationDepthSettings>,
    /// HAMT settings attachments keyed by referenced material names.
    pub material_heat_and_moisture_transfer_settings: Vec<MaterialHeatAndMoistureTransferSettings>,
    /// HAMT sorption-isotherm attachments keyed by referenced material names.
    pub material_heat_and_moisture_transfer_sorption_isotherms:
        Vec<MaterialHeatAndMoistureTransferSorptionIsotherm>,
    /// HAMT suction attachments keyed by referenced material names.
    pub material_heat_and_moisture_transfer_suctions: Vec<MaterialHeatAndMoistureTransferSuction>,
    /// HAMT redistribution attachments keyed by referenced material names.
    pub material_heat_and_moisture_transfer_redistributions:
        Vec<MaterialHeatAndMoistureTransferRedistribution>,
    /// Ordered thermochromic glazing states referenced by range descriptors on materials.
    pub window_glazing_thermochromic_state_arena: Vec<WindowGlazingThermochromicState>,
    /// Constructions.
    pub constructions: Vec<Construction>,
    /// Construction names.
    pub construction_names: NameMap<ConstructionId>,
    /// Schedule type limits.
    pub schedule_type_limits: Vec<ScheduleTypeLimits>,
    /// Schedule type limit names.
    pub schedule_type_limit_names: NameMap<ScheduleTypeLimitId>,
    /// Hourly day schedules.
    pub day_schedules: Vec<ScheduleDayHourly>,
    /// Interval day schedules, assigned IDs after all hourly day schedules.
    pub day_interval_schedules: Vec<ScheduleDayInterval>,
    /// List day schedules, assigned IDs after all hourly and interval day schedules.
    pub day_list_schedules: Vec<ScheduleDayList>,
    /// Day schedule names.
    pub day_schedule_names: NameMap<DayScheduleId>,
    /// Daily week schedules.
    pub week_schedules: Vec<ScheduleWeekDaily>,
    /// Compact week schedules, assigned IDs after all daily week schedules.
    pub week_compact_schedules: Vec<ScheduleWeekCompact>,
    /// Week schedule names.
    pub week_schedule_names: NameMap<WeekScheduleId>,
    /// Unique bulk surface-shading schedule file, when declared.
    pub file_shading_schedule: Option<ScheduleFileShading>,
    /// Constant schedules.
    pub schedules: Vec<ScheduleConstant>,
    /// Compact schedules.
    pub compact_schedules: Vec<ScheduleCompact>,
    /// File-backed schedules loaded into immutable source values.
    pub file_schedules: Vec<ScheduleFile>,
    /// Annual schedules composed from day and week schedule references.
    pub year_schedules: Vec<ScheduleYear>,
    /// Inactive external-interface schedules held at their initial values.
    pub external_interface_schedules: Vec<ExternalInterfaceSchedule>,
    /// Inactive FMU-import schedules held at their initial values.
    pub external_interface_fmu_import_schedules: Vec<ExternalInterfaceFmuImportSchedule>,
    /// Inactive FMU-export schedules held at their initial values.
    pub external_interface_fmu_export_schedules: Vec<ExternalInterfaceFmuExportSchedule>,
    /// Schedule names.
    pub schedule_names: NameMap<ScheduleId>,
    /// Zone internal gains from OtherEquipment objects.
    pub other_equipment: Vec<OtherEquipment>,
    /// OtherEquipment names.
    pub other_equipment_names: NameMap<InternalGainId>,
    /// Zone occupants from People objects.
    pub people: Vec<People>,
    /// People object names.
    pub people_names: NameMap<InternalGainId>,
    /// Dual setpoint thermostat objects.
    pub thermostat_dual_setpoints: Vec<ThermostatDualSetpoint>,
    /// Dual setpoint names.
    pub thermostat_dual_setpoint_names: NameMap<ThermostatSetpointId>,
    /// Zone thermostat controls.
    pub zone_thermostats: Vec<ZoneThermostat>,
    /// Zone thermostat names.
    pub zone_thermostat_names: NameMap<ZoneThermostatId>,
    /// Zone humidistat controls.
    pub zone_humidistats: Vec<ZoneHumidistat>,
    /// Zone humidistat names.
    pub zone_humidistat_names: NameMap<ZoneHumidistatId>,
    /// IdealLoads air systems.
    pub ideal_loads_air_systems: Vec<IdealLoadsAirSystem>,
    /// IdealLoads air system names.
    pub ideal_loads_air_system_names: NameMap<IdealLoadsAirSystemId>,
    /// DesignSpecification:OutdoorAir objects.
    pub design_specification_outdoor_air: Vec<DesignSpecificationOutdoorAir>,
    /// DesignSpecification:OutdoorAir names.
    pub design_specification_outdoor_air_names: NameMap<DesignSpecificationOutdoorAirId>,
    /// Zone equipment lists.
    pub zone_equipment_lists: Vec<ZoneEquipmentList>,
    /// Zone equipment list names.
    pub zone_equipment_list_names: NameMap<ZoneEquipmentListId>,
    /// Zone equipment connections.
    pub zone_equipment_connections: Vec<ZoneEquipmentConnection>,
    /// Discovered air-side nodes.
    pub nodes: Vec<Node>,
    /// Node names.
    pub node_names: NameMap<NodeId>,
    /// Node lists.
    pub node_lists: Vec<NodeList>,
    /// NodeList names.
    pub node_list_names: NameMap<NodeListId>,
    /// Air loops.
    pub air_loops: Vec<AirLoopHvac>,
    /// Air loop names.
    pub air_loop_names: NameMap<LoopId>,
    /// Fan components.
    pub fans: Vec<FanComponent>,
    /// Fan component names.
    pub fan_names: NameMap<ComponentId>,
    /// Coil components.
    pub coils: Vec<CoilComponent>,
    /// Coil component names.
    pub coil_names: NameMap<ComponentId>,
    /// Setpoint managers tracked by source map.
    pub setpoint_managers: Vec<SetpointManagerComponent>,
    /// Setpoint manager names.
    pub setpoint_manager_names: NameMap<ComponentId>,
    /// Availability managers tracked by source map.
    pub availability_managers: Vec<AvailabilityManagerComponent>,
    /// Availability manager names.
    pub availability_manager_names: NameMap<ComponentId>,
    /// Plant loops.
    pub plant_loops: Vec<PlantLoop>,
    /// Plant loop names.
    pub plant_loop_names: NameMap<LoopId>,
    /// Plant branches.
    pub plant_branches: Vec<PlantBranch>,
    /// Plant branch names.
    pub plant_branch_names: NameMap<BranchId>,
    /// Plant branch lists.
    pub plant_branch_lists: Vec<PlantBranchList>,
    /// Plant branch list names.
    pub plant_branch_list_names: NameMap<BranchListId>,
    /// Plant connectors.
    pub plant_connectors: Vec<PlantConnector>,
    /// Plant connector names.
    pub plant_connector_names: NameMap<ConnectorId>,
    /// Plant connector lists.
    pub plant_connector_lists: Vec<PlantConnectorList>,
    /// Plant connector list names.
    pub plant_connector_list_names: NameMap<ConnectorListId>,
    /// Constant-speed pumps.
    pub pumps_constant_speed: Vec<PumpConstantSpeed>,
    /// Constant-speed pump names.
    pub pump_constant_speed_names: NameMap<ComponentId>,
    /// Hot-water boilers.
    pub boilers_hot_water: Vec<BoilerHotWater>,
    /// Hot-water boiler names.
    pub boiler_hot_water_names: NameMap<ComponentId>,
    /// Electric EIR chillers.
    pub chillers_electric_eir: Vec<ChillerElectricEir>,
    /// Electric EIR chiller names.
    pub chiller_electric_eir_names: NameMap<ComponentId>,
    /// Zones.
    pub zones: Vec<Zone>,
    /// Zone names.
    pub zone_names: NameMap<ZoneId>,
    /// Building surfaces.
    pub surfaces: Vec<Surface>,
    /// Surface names.
    pub surface_names: NameMap<SurfaceId>,
}

impl Default for TypedModel {
    fn default() -> Self {
        Self {
            version: Version::oracle_26_1_0(),
            building: None,
            global_geometry_rules: None,
            timestep: TimestepConfig::default(),
            surface_convection_algorithms: SurfaceConvectionAlgorithms::default(),
            run_periods: Vec::new(),
            run_period_names: NameMap::default(),
            run_period_special_days: Vec::new(),
            run_period_special_day_names: NameMap::default(),
            run_period_daylight_saving_time: None,
            site: None,
            glazing_spectral_data: Vec::new(),
            glazing_spectral_data_names: NameMap::default(),
            materials: Vec::new(),
            material_names: NameMap::default(),
            material_variable_absorptances: Vec::new(),
            material_variable_absorptance_names: NameMap::default(),
            material_phase_change_hystereses: Vec::new(),
            material_phase_changes: Vec::new(),
            material_variable_thermal_conductivities: Vec::new(),
            material_moisture_penetration_depth_settings: Vec::new(),
            material_heat_and_moisture_transfer_settings: Vec::new(),
            material_heat_and_moisture_transfer_sorption_isotherms: Vec::new(),
            material_heat_and_moisture_transfer_suctions: Vec::new(),
            material_heat_and_moisture_transfer_redistributions: Vec::new(),
            window_glazing_thermochromic_state_arena: Vec::new(),
            constructions: Vec::new(),
            construction_names: NameMap::default(),
            schedule_type_limits: Vec::new(),
            schedule_type_limit_names: NameMap::default(),
            day_schedules: Vec::new(),
            day_interval_schedules: Vec::new(),
            day_list_schedules: Vec::new(),
            day_schedule_names: NameMap::default(),
            week_schedules: Vec::new(),
            week_compact_schedules: Vec::new(),
            week_schedule_names: NameMap::default(),
            file_shading_schedule: None,
            schedules: Vec::new(),
            compact_schedules: Vec::new(),
            file_schedules: Vec::new(),
            year_schedules: Vec::new(),
            external_interface_schedules: Vec::new(),
            external_interface_fmu_import_schedules: Vec::new(),
            external_interface_fmu_export_schedules: Vec::new(),
            schedule_names: NameMap::default(),
            other_equipment: Vec::new(),
            other_equipment_names: NameMap::default(),
            people: Vec::new(),
            people_names: NameMap::default(),
            thermostat_dual_setpoints: Vec::new(),
            thermostat_dual_setpoint_names: NameMap::default(),
            zone_thermostats: Vec::new(),
            zone_thermostat_names: NameMap::default(),
            zone_humidistats: Vec::new(),
            zone_humidistat_names: NameMap::default(),
            ideal_loads_air_systems: Vec::new(),
            ideal_loads_air_system_names: NameMap::default(),
            design_specification_outdoor_air: Vec::new(),
            design_specification_outdoor_air_names: NameMap::default(),
            zone_equipment_lists: Vec::new(),
            zone_equipment_list_names: NameMap::default(),
            zone_equipment_connections: Vec::new(),
            nodes: Vec::new(),
            node_names: NameMap::default(),
            node_lists: Vec::new(),
            node_list_names: NameMap::default(),
            air_loops: Vec::new(),
            air_loop_names: NameMap::default(),
            fans: Vec::new(),
            fan_names: NameMap::default(),
            coils: Vec::new(),
            coil_names: NameMap::default(),
            setpoint_managers: Vec::new(),
            setpoint_manager_names: NameMap::default(),
            availability_managers: Vec::new(),
            availability_manager_names: NameMap::default(),
            plant_loops: Vec::new(),
            plant_loop_names: NameMap::default(),
            plant_branches: Vec::new(),
            plant_branch_names: NameMap::default(),
            plant_branch_lists: Vec::new(),
            plant_branch_list_names: NameMap::default(),
            plant_connectors: Vec::new(),
            plant_connector_names: NameMap::default(),
            plant_connector_lists: Vec::new(),
            plant_connector_list_names: NameMap::default(),
            pumps_constant_speed: Vec::new(),
            pump_constant_speed_names: NameMap::default(),
            boilers_hot_water: Vec::new(),
            boiler_hot_water_names: NameMap::default(),
            chillers_electric_eir: Vec::new(),
            chiller_electric_eir_names: NameMap::default(),
            zones: Vec::new(),
            zone_names: NameMap::default(),
            surfaces: Vec::new(),
            surface_names: NameMap::default(),
        }
    }
}

impl TypedModel {
    /// Returns the ordered state slice described by a thermochromic glazing group.
    ///
    /// Returns `None` when the descriptor overflows or lies outside the state arena.
    #[must_use]
    pub fn window_glazing_thermochromic_states(
        &self,
        group: WindowGlazingThermochromicGroupMaterial,
    ) -> Option<&[WindowGlazingThermochromicState]> {
        let first_state = usize::try_from(group.first_state).ok()?;
        let state_count = usize::try_from(group.state_count).ok()?;
        let end = first_state.checked_add(state_count)?;
        self.window_glazing_thermochromic_state_arena
            .get(first_state..end)
    }

    /// Number of typed object instances in the current subset.
    #[must_use]
    pub fn object_count(&self) -> usize {
        usize::from(self.building.is_some())
            + usize::from(self.global_geometry_rules.is_some())
            + usize::from(self.site.is_some())
            + 1
            + usize::from(self.surface_convection_algorithms.inside.is_some())
            + usize::from(self.surface_convection_algorithms.outside.is_some())
            + self.run_periods.len()
            + self.run_period_special_days.len()
            + usize::from(self.run_period_daylight_saving_time.is_some())
            + self.glazing_spectral_data.len()
            + self.materials.len()
            + self.material_variable_absorptances.len()
            + self.material_phase_change_hystereses.len()
            + self.material_phase_changes.len()
            + self.material_variable_thermal_conductivities.len()
            + self.material_moisture_penetration_depth_settings.len()
            + self.material_heat_and_moisture_transfer_settings.len()
            + self
                .material_heat_and_moisture_transfer_sorption_isotherms
                .len()
            + self.material_heat_and_moisture_transfer_suctions.len()
            + self
                .material_heat_and_moisture_transfer_redistributions
                .len()
            + self.constructions.len()
            + self.schedule_type_limits.len()
            + self.day_schedules.len()
            + self.day_interval_schedules.len()
            + self.day_list_schedules.len()
            + self.week_schedules.len()
            + self.week_compact_schedules.len()
            + usize::from(self.file_shading_schedule.is_some())
            + self.schedules.len()
            + self.compact_schedules.len()
            + self.file_schedules.len()
            + self.year_schedules.len()
            + self.external_interface_schedules.len()
            + self.external_interface_fmu_import_schedules.len()
            + self.external_interface_fmu_export_schedules.len()
            + self.other_equipment.len()
            + self.people.len()
            + self.thermostat_dual_setpoints.len()
            + self.zone_thermostats.len()
            + self.zone_humidistats.len()
            + self.ideal_loads_air_systems.len()
            + self.design_specification_outdoor_air.len()
            + self.zone_equipment_lists.len()
            + self.zone_equipment_connections.len()
            + self.node_lists.len()
            + self.air_loops.len()
            + self.fans.len()
            + self.coils.len()
            + self.setpoint_managers.len()
            + self.availability_managers.len()
            + self.plant_loops.len()
            + self.plant_branches.len()
            + self.plant_branch_lists.len()
            + self.plant_connectors.len()
            + self.plant_connector_lists.len()
            + self.pumps_constant_speed.len()
            + self.boilers_hot_water.len()
            + self.chillers_electric_eir.len()
            + self.zones.len()
            + self.surfaces.len()
    }
}

/// Runtime-ready immutable model plus graph relations.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationModel {
    /// Typed model payload.
    pub typed: TypedModel,
    /// Static model graph.
    pub graph: ModelGraph,
}

impl SimulationModel {
    /// Builds a runtime-ready model from an already reference-resolved typed model.
    #[must_use]
    pub fn from_typed(typed: TypedModel) -> Self {
        let graph = ModelGraph::from_typed(&typed);
        Self { typed, graph }
    }
}

/// Static model graph used for validation and execution planning.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelGraph {
    /// Zone to surface edges.
    pub zone_surfaces: Vec<ZoneSurfaceEdge>,
    /// Construction to material edges.
    pub construction_materials: Vec<ConstructionMaterialEdge>,
    /// Zone to thermostat edges.
    pub zone_thermostats: Vec<ZoneThermostatEdge>,
    /// Thermostat to dual setpoint edges.
    pub thermostat_setpoints: Vec<ThermostatSetpointEdge>,
    /// Zone to IdealLoads equipment edges through equipment connections/lists.
    pub zone_ideal_loads: Vec<ZoneIdealLoadsEdge>,
    /// NodeList membership edges.
    pub node_list_members: Vec<NodeListMemberEdge>,
    /// IdealLoads supply-node edges.
    pub ideal_loads_supply_nodes: Vec<IdealLoadsSupplyNodeEdge>,
    /// IdealLoads outdoor-air design specification edges.
    pub ideal_loads_outdoor_air_specs: Vec<IdealLoadsOutdoorAirSpecEdge>,
    /// Zone air-node edges.
    pub zone_air_nodes: Vec<ZoneAirNodeEdge>,
    /// Plant loop to branch-list edges.
    pub plant_loop_branch_lists: Vec<PlantLoopBranchListEdge>,
    /// Branch-list membership edges.
    pub plant_branch_list_members: Vec<PlantBranchListMemberEdge>,
    /// Connector-list membership edges.
    pub plant_connector_list_members: Vec<PlantConnectorListMemberEdge>,
    /// Branch to component edges.
    pub plant_branch_components: Vec<PlantBranchComponentEdge>,
    /// Plant-loop graph skeleton.
    pub plant_loop_graph: PlantLoopGraph,
    /// Air-loop graph skeleton.
    pub air_loop_graph: AirLoopGraph,
    /// HVAC/plant/zone component registry.
    pub component_registry: ComponentRegistry,
    /// Node graph with component ownership and node diagnostics.
    pub node_graph: NodeGraph,
}

impl ModelGraph {
    /// Builds static graph edges from the typed subset.
    #[must_use]
    pub fn from_typed(model: &TypedModel) -> Self {
        Self {
            zone_surfaces: model
                .surfaces
                .iter()
                .map(|surface| ZoneSurfaceEdge {
                    zone: surface.zone,
                    surface: surface.id,
                })
                .collect(),
            construction_materials: model
                .constructions
                .iter()
                .flat_map(|construction| {
                    construction
                        .layers
                        .iter()
                        .enumerate()
                        .map(|(index, material)| ConstructionMaterialEdge {
                            construction: construction.id,
                            material: *material,
                            layer_index: index as u32,
                        })
                })
                .collect(),
            zone_thermostats: model
                .zone_thermostats
                .iter()
                .map(|thermostat| ZoneThermostatEdge {
                    zone: thermostat.zone,
                    thermostat: thermostat.id,
                })
                .collect(),
            thermostat_setpoints: model
                .zone_thermostats
                .iter()
                .flat_map(|thermostat| {
                    thermostat
                        .controls
                        .iter()
                        .map(move |control| ThermostatSetpointEdge {
                            thermostat: thermostat.id,
                            setpoint: control.dual_setpoint,
                        })
                })
                .collect(),
            zone_ideal_loads: sorted_zone_ideal_loads(model),
            node_list_members: model
                .node_lists
                .iter()
                .flat_map(|node_list| {
                    node_list
                        .nodes
                        .iter()
                        .enumerate()
                        .map(move |(index, node)| NodeListMemberEdge {
                            node_list: node_list.id,
                            node: *node,
                            index: index as u32,
                        })
                })
                .collect(),
            ideal_loads_supply_nodes: model
                .ideal_loads_air_systems
                .iter()
                .flat_map(|system| {
                    resolve_node_or_list(model, &system.zone_supply_air_node_name)
                        .into_iter()
                        .map(move |node| IdealLoadsSupplyNodeEdge {
                            ideal_loads_air_system: system.id,
                            node,
                        })
                })
                .collect(),
            ideal_loads_outdoor_air_specs: model
                .ideal_loads_air_systems
                .iter()
                .filter_map(|system| {
                    let spec_name = system
                        .design_specification_outdoor_air_object_name
                        .as_ref()?;
                    model
                        .design_specification_outdoor_air_names
                        .resolve(&spec_name.0)
                        .map(
                            |design_specification_outdoor_air| IdealLoadsOutdoorAirSpecEdge {
                                ideal_loads_air_system: system.id,
                                design_specification_outdoor_air,
                            },
                        )
                })
                .collect(),
            zone_air_nodes: model
                .zone_equipment_connections
                .iter()
                .filter_map(|connection| {
                    model
                        .node_names
                        .resolve(&connection.zone_air_node_name.0)
                        .map(|node| ZoneAirNodeEdge {
                            zone: connection.zone,
                            node,
                        })
                })
                .collect(),
            plant_loop_branch_lists: plant_loop_branch_lists(model),
            plant_branch_list_members: model
                .plant_branch_lists
                .iter()
                .flat_map(|list| {
                    list.branches
                        .iter()
                        .enumerate()
                        .map(move |(index, branch)| PlantBranchListMemberEdge {
                            branch_list: list.id,
                            branch: *branch,
                            index: index as u32,
                        })
                })
                .collect(),
            plant_connector_list_members: model
                .plant_connector_lists
                .iter()
                .flat_map(|list| {
                    list.connectors
                        .iter()
                        .enumerate()
                        .map(move |(index, entry)| PlantConnectorListMemberEdge {
                            connector_list: list.id,
                            connector: entry.connector,
                            kind: entry.kind,
                            index: index as u32,
                        })
                })
                .collect(),
            plant_branch_components: model
                .plant_branches
                .iter()
                .flat_map(|branch| {
                    branch
                        .components
                        .iter()
                        .enumerate()
                        .map(move |(index, component)| PlantBranchComponentEdge {
                            branch: branch.id,
                            component_type: component.object_type.clone(),
                            component_name: component.name.clone(),
                            inlet_node: component.inlet_node,
                            outlet_node: component.outlet_node,
                            index: index as u32,
                        })
                })
                .collect(),
            plant_loop_graph: PlantLoopGraph::from_typed(model),
            air_loop_graph: AirLoopGraph::from_typed(model),
            component_registry: ComponentRegistry::from_typed(model),
            node_graph: NodeGraph::from_typed(model),
        }
    }
}

/// Static plant-loop graph skeleton used before full PlantLoop simulation parity.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PlantLoopGraph {
    /// Supply and demand half loops.
    pub half_loops: Vec<PlantHalfLoop>,
    /// Plant-loop scoped branch-list membership edges.
    pub branch_list_members: Vec<PlantLoopBranchListMemberEdge>,
    /// Plant-loop scoped connector-list membership edges.
    pub connector_list_members: Vec<PlantLoopConnectorListMemberEdge>,
    /// Plant-loop scoped component registry entries.
    pub component_registry: Vec<PlantComponentRegistryEntry>,
    /// Topology diagnostics kept diagnostic-only.
    pub diagnostics: Vec<PlantLoopGraphDiagnostic>,
}

impl PlantLoopGraph {
    /// Builds a diagnostic plant-loop graph from typed PlantLoop, BranchList,
    /// ConnectorList, Branch, and component data.
    #[must_use]
    pub fn from_typed(model: &TypedModel) -> Self {
        let half_loops = model
            .plant_loops
            .iter()
            .flat_map(|plant_loop| {
                [
                    PlantHalfLoop {
                        plant_loop: plant_loop.id,
                        side: PlantLoopSide::Plant,
                        inlet_node: plant_loop.plant_side_inlet_node,
                        outlet_node: plant_loop.plant_side_outlet_node,
                        branch_list: plant_loop.plant_side_branch_list,
                        connector_list: plant_loop.plant_side_connector_list,
                    },
                    PlantHalfLoop {
                        plant_loop: plant_loop.id,
                        side: PlantLoopSide::Demand,
                        inlet_node: plant_loop.demand_side_inlet_node,
                        outlet_node: plant_loop.demand_side_outlet_node,
                        branch_list: plant_loop.demand_side_branch_list,
                        connector_list: plant_loop.demand_side_connector_list,
                    },
                ]
            })
            .collect::<Vec<_>>();

        let branch_list_members = half_loops
            .iter()
            .flat_map(|half_loop| {
                model
                    .plant_branch_lists
                    .iter()
                    .find(|list| list.id == half_loop.branch_list)
                    .into_iter()
                    .flat_map(move |list| {
                        list.branches
                            .iter()
                            .enumerate()
                            .map(move |(index, branch)| PlantLoopBranchListMemberEdge {
                                plant_loop: half_loop.plant_loop,
                                side: half_loop.side,
                                branch_list: half_loop.branch_list,
                                branch: *branch,
                                index: index as u32,
                            })
                    })
            })
            .collect::<Vec<_>>();

        let connector_list_members = half_loops
            .iter()
            .flat_map(|half_loop| {
                half_loop
                    .connector_list
                    .into_iter()
                    .flat_map(move |connector_list| {
                        model
                            .plant_connector_lists
                            .iter()
                            .find(move |list| list.id == connector_list)
                            .into_iter()
                            .flat_map(move |list| {
                                list.connectors
                                    .iter()
                                    .enumerate()
                                    .map(move |(index, entry)| PlantLoopConnectorListMemberEdge {
                                        plant_loop: half_loop.plant_loop,
                                        side: half_loop.side,
                                        connector_list,
                                        connector: entry.connector,
                                        kind: entry.kind,
                                        index: index as u32,
                                    })
                            })
                    })
            })
            .collect::<Vec<_>>();

        let component_registry = branch_list_members
            .iter()
            .flat_map(|member| {
                model
                    .plant_branches
                    .iter()
                    .find(|branch| branch.id == member.branch)
                    .into_iter()
                    .flat_map(move |branch| {
                        branch.components.iter().enumerate().map(
                            move |(component_index, component)| PlantComponentRegistryEntry {
                                plant_loop: member.plant_loop,
                                side: member.side,
                                branch: branch.id,
                                component_type: component.object_type.clone(),
                                component_name: component.name.clone(),
                                inlet_node: component.inlet_node,
                                outlet_node: component.outlet_node,
                                branch_index: member.index,
                                component_index: component_index as u32,
                            },
                        )
                    })
            })
            .collect();

        let diagnostics = unsupported_plant_topology_diagnostics(model, &half_loops);

        Self {
            half_loops,
            branch_list_members,
            connector_list_members,
            component_registry,
            diagnostics,
        }
    }
}

/// One supply or demand half loop in a PlantLoop graph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantHalfLoop {
    /// Plant loop ID.
    pub plant_loop: LoopId,
    /// Supply/plant side or demand side.
    pub side: PlantLoopSide,
    /// Half-loop inlet node.
    pub inlet_node: NodeId,
    /// Half-loop outlet node.
    pub outlet_node: NodeId,
    /// Branch list for this half loop.
    pub branch_list: BranchListId,
    /// Optional connector list for this half loop.
    pub connector_list: Option<ConnectorListId>,
}

/// Plant-loop scoped branch-list membership relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantLoopBranchListMemberEdge {
    /// Plant loop ID.
    pub plant_loop: LoopId,
    /// Supply/plant side or demand side.
    pub side: PlantLoopSide,
    /// Branch list ID.
    pub branch_list: BranchListId,
    /// Branch ID.
    pub branch: BranchId,
    /// Zero-based branch order within the branch list.
    pub index: u32,
}

/// Plant-loop scoped connector-list membership relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantLoopConnectorListMemberEdge {
    /// Plant loop ID.
    pub plant_loop: LoopId,
    /// Supply/plant side or demand side.
    pub side: PlantLoopSide,
    /// Connector list ID.
    pub connector_list: ConnectorListId,
    /// Connector ID.
    pub connector: ConnectorId,
    /// Connector kind.
    pub kind: PlantConnectorKind,
    /// Zero-based connector order within the connector list.
    pub index: u32,
}

/// Plant component indexed by loop, side, branch, and component order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlantComponentRegistryEntry {
    /// Plant loop ID.
    pub plant_loop: LoopId,
    /// Supply/plant side or demand side.
    pub side: PlantLoopSide,
    /// Owning branch.
    pub branch: BranchId,
    /// Component object type.
    pub component_type: NormalizedName,
    /// Component name.
    pub component_name: NormalizedName,
    /// Component inlet node.
    pub inlet_node: NodeId,
    /// Component outlet node.
    pub outlet_node: NodeId,
    /// Zero-based branch order within the half loop.
    pub branch_index: u32,
    /// Zero-based component order within the branch.
    pub component_index: u32,
}

/// Diagnostic severity for plant-loop graph skeleton checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantLoopGraphDiagnosticSeverity {
    /// Informational diagnostic.
    Info,
    /// Warning that keeps the case diagnostic-only.
    Warning,
    /// Error that blocks future promotion.
    Error,
}

/// Plant-loop graph diagnostic code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantLoopGraphDiagnosticCode {
    /// Topology is beyond the current diagnostic skeleton.
    UnsupportedTopology,
}

/// Diagnostic emitted by the plant-loop graph skeleton.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlantLoopGraphDiagnostic {
    /// Plant loop ID if the diagnostic is loop-scoped.
    pub plant_loop: Option<LoopId>,
    /// Half-loop side if the diagnostic is side-scoped.
    pub side: Option<PlantLoopSide>,
    /// Diagnostic severity.
    pub severity: PlantLoopGraphDiagnosticSeverity,
    /// Stable diagnostic code.
    pub code: PlantLoopGraphDiagnosticCode,
    /// Human-readable diagnostic message.
    pub message: String,
}

/// Static air-loop graph skeleton used before full AirLoopHVAC simulation parity.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AirLoopGraph {
    /// AirLoopHVAC to BranchList edges.
    pub branch_lists: Vec<AirLoopBranchListEdge>,
    /// AirLoopHVAC BranchList membership edges.
    pub branch_list_members: Vec<AirLoopBranchListMemberEdge>,
    /// AirLoopHVAC to ConnectorList edges.
    pub connector_lists: Vec<AirLoopConnectorListEdge>,
    /// AirLoopHVAC ConnectorList membership edges.
    pub connector_list_members: Vec<AirLoopConnectorListMemberEdge>,
    /// Component execution order compiled from AirLoopHVAC branch graph order.
    pub execution_order: Vec<AirLoopExecutionStep>,
}

impl AirLoopGraph {
    /// Builds an air-loop graph from typed AirLoopHVAC, BranchList, ConnectorList, and Branch data.
    #[must_use]
    pub fn from_typed(model: &TypedModel) -> Self {
        let branch_lists = model
            .air_loops
            .iter()
            .filter_map(|air_loop| {
                air_loop
                    .branch_list
                    .map(|branch_list| AirLoopBranchListEdge {
                        air_loop: air_loop.id,
                        branch_list,
                    })
            })
            .collect::<Vec<_>>();

        let branch_list_members = branch_lists
            .iter()
            .flat_map(|edge| {
                model
                    .plant_branch_lists
                    .iter()
                    .find(|list| list.id == edge.branch_list)
                    .into_iter()
                    .flat_map(move |list| {
                        list.branches
                            .iter()
                            .enumerate()
                            .map(move |(index, branch)| AirLoopBranchListMemberEdge {
                                air_loop: edge.air_loop,
                                branch_list: edge.branch_list,
                                branch: *branch,
                                index: index as u32,
                            })
                    })
            })
            .collect::<Vec<_>>();

        let connector_lists = model
            .air_loops
            .iter()
            .filter_map(|air_loop| {
                air_loop
                    .connector_list
                    .map(|connector_list| AirLoopConnectorListEdge {
                        air_loop: air_loop.id,
                        connector_list,
                    })
            })
            .collect::<Vec<_>>();

        let connector_list_members = connector_lists
            .iter()
            .flat_map(|edge| {
                model
                    .plant_connector_lists
                    .iter()
                    .find(|list| list.id == edge.connector_list)
                    .into_iter()
                    .flat_map(move |list| {
                        list.connectors
                            .iter()
                            .enumerate()
                            .map(move |(index, entry)| AirLoopConnectorListMemberEdge {
                                air_loop: edge.air_loop,
                                connector_list: edge.connector_list,
                                connector: entry.connector,
                                kind: entry.kind,
                                index: index as u32,
                            })
                    })
            })
            .collect::<Vec<_>>();

        let execution_order = branch_list_members
            .iter()
            .flat_map(|edge| {
                model
                    .plant_branches
                    .iter()
                    .find(|branch| branch.id == edge.branch)
                    .into_iter()
                    .flat_map(move |branch| {
                        branch
                            .components
                            .iter()
                            .enumerate()
                            .map(move |(index, component)| AirLoopExecutionStep {
                                air_loop: edge.air_loop,
                                branch: branch.id,
                                component_type: component.object_type.clone(),
                                component_name: component.name.clone(),
                                inlet_node: component.inlet_node,
                                outlet_node: component.outlet_node,
                                sequence_index: index as u32,
                            })
                    })
            })
            .collect();

        Self {
            branch_lists,
            branch_list_members,
            connector_lists,
            connector_list_members,
            execution_order,
        }
    }
}

/// AirLoopHVAC to BranchList edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AirLoopBranchListEdge {
    /// Air loop.
    pub air_loop: LoopId,
    /// Branch list used by the air loop.
    pub branch_list: BranchListId,
}

/// AirLoopHVAC BranchList member edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AirLoopBranchListMemberEdge {
    /// Air loop.
    pub air_loop: LoopId,
    /// Branch list.
    pub branch_list: BranchListId,
    /// Branch.
    pub branch: BranchId,
    /// Zero-based branch order.
    pub index: u32,
}

/// AirLoopHVAC to ConnectorList edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AirLoopConnectorListEdge {
    /// Air loop.
    pub air_loop: LoopId,
    /// Connector list used by the air loop.
    pub connector_list: ConnectorListId,
}

/// AirLoopHVAC ConnectorList member edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AirLoopConnectorListMemberEdge {
    /// Air loop.
    pub air_loop: LoopId,
    /// Connector list.
    pub connector_list: ConnectorListId,
    /// Connector.
    pub connector: ConnectorId,
    /// Connector kind.
    pub kind: PlantConnectorKind,
    /// Zero-based connector order.
    pub index: u32,
}

/// Component execution step compiled from an air-loop branch graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AirLoopExecutionStep {
    /// Air loop.
    pub air_loop: LoopId,
    /// Owning branch.
    pub branch: BranchId,
    /// Component object type.
    pub component_type: NormalizedName,
    /// Component name.
    pub component_name: NormalizedName,
    /// Component inlet node.
    pub inlet_node: NodeId,
    /// Component outlet node.
    pub outlet_node: NodeId,
    /// Zero-based order within the owning branch.
    pub sequence_index: u32,
}

/// Component category used by the static registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentRegistryCategory {
    /// Fan component.
    Fan,
    /// Coil component.
    Coil,
    /// Zone equipment component.
    ZoneEquipment,
    /// Plant component declared on a branch.
    Plant,
    /// Setpoint manager.
    SetpointManager,
    /// Availability manager.
    AvailabilityManager,
}

/// One component indexed for graph validation and later dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentRegistryEntry {
    /// Component category.
    pub category: ComponentRegistryCategory,
    /// EnergyPlus object type.
    pub object_type: NormalizedName,
    /// Object name.
    pub name: NormalizedName,
    /// Optional typed component ID within its family.
    pub component_id: Option<ComponentId>,
    /// Optional inlet node.
    pub inlet_node: Option<NodeId>,
    /// Optional outlet node.
    pub outlet_node: Option<NodeId>,
}

/// Static component registry for HVAC, zone equipment, and plant skeletons.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ComponentRegistry {
    /// Registered components.
    pub entries: Vec<ComponentRegistryEntry>,
}

impl ComponentRegistry {
    /// Builds a component registry from the typed model.
    #[must_use]
    pub fn from_typed(model: &TypedModel) -> Self {
        let mut entries = Vec::new();

        entries.extend(model.fans.iter().map(|fan| ComponentRegistryEntry {
            category: ComponentRegistryCategory::Fan,
            object_type: NormalizedName::new(fan.kind.object_type()),
            name: fan.name.clone(),
            component_id: Some(fan.id),
            inlet_node: Some(fan.inlet_node),
            outlet_node: Some(fan.outlet_node),
        }));

        entries.extend(model.coils.iter().map(|coil| ComponentRegistryEntry {
            category: ComponentRegistryCategory::Coil,
            object_type: NormalizedName::new(coil.kind.object_type()),
            name: coil.name.clone(),
            component_id: Some(coil.id),
            inlet_node: coil.inlet_node,
            outlet_node: coil.outlet_node,
        }));

        entries.extend(model.zone_equipment_lists.iter().flat_map(|list| {
            list.equipment.iter().map(|entry| ComponentRegistryEntry {
                category: ComponentRegistryCategory::ZoneEquipment,
                object_type: NormalizedName::new(zone_equipment_object_type_label(
                    entry.object_type,
                )),
                name: model
                    .ideal_loads_air_systems
                    .iter()
                    .find(|system| system.id == entry.ideal_loads_air_system)
                    .map(|system| system.name.clone())
                    .unwrap_or_else(|| NormalizedName::new("<unresolved>")),
                component_id: None,
                inlet_node: None,
                outlet_node: None,
            })
        }));

        entries.extend(model.plant_branches.iter().flat_map(|branch| {
            branch
                .components
                .iter()
                .map(|component| ComponentRegistryEntry {
                    category: ComponentRegistryCategory::Plant,
                    object_type: component.object_type.clone(),
                    name: component.name.clone(),
                    component_id: None,
                    inlet_node: Some(component.inlet_node),
                    outlet_node: Some(component.outlet_node),
                })
        }));

        entries.extend(
            model
                .setpoint_managers
                .iter()
                .map(|manager| ComponentRegistryEntry {
                    category: ComponentRegistryCategory::SetpointManager,
                    object_type: manager.object_type.clone(),
                    name: manager.name.clone(),
                    component_id: Some(manager.id),
                    inlet_node: None,
                    outlet_node: manager.setpoint_node,
                }),
        );

        entries.extend(
            model
                .availability_managers
                .iter()
                .map(|manager| ComponentRegistryEntry {
                    category: ComponentRegistryCategory::AvailabilityManager,
                    object_type: manager.object_type.clone(),
                    name: manager.name.clone(),
                    component_id: Some(manager.id),
                    inlet_node: None,
                    outlet_node: None,
                }),
        );

        Self { entries }
    }
}

fn zone_equipment_object_type_label(object_type: ZoneEquipmentObjectType) -> &'static str {
    match object_type {
        ZoneEquipmentObjectType::IdealLoadsAirSystem => "ZoneHVAC:IdealLoadsAirSystem",
    }
}

/// Static node graph used for HVAC/plant ownership and diagnostics.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NodeGraph {
    /// Component inlet/outlet ownership edges.
    pub component_node_ownership: Vec<ComponentNodeOwnershipEdge>,
    /// Node diagnostics kept separate from conformance diagnostics.
    pub diagnostics: Vec<NodeGraphDiagnostic>,
}

impl NodeGraph {
    /// Builds a node graph from typed nodes and component inlet/outlet ownership.
    #[must_use]
    pub fn from_typed(model: &TypedModel) -> Self {
        let mut component_node_ownership = model
            .plant_branches
            .iter()
            .flat_map(|branch| {
                branch
                    .components
                    .iter()
                    .enumerate()
                    .map(move |(index, component)| ComponentNodeOwnershipEdge {
                        component_type: component.object_type.clone(),
                        component_name: component.name.clone(),
                        inlet_node: component.inlet_node,
                        outlet_node: component.outlet_node,
                        owner_branch: Some(branch.id),
                        index: index as u32,
                    })
            })
            .collect::<Vec<_>>();
        component_node_ownership.extend(model.fans.iter().enumerate().map(|(index, fan)| {
            ComponentNodeOwnershipEdge {
                component_type: NormalizedName::new(fan.kind.object_type()),
                component_name: fan.name.clone(),
                inlet_node: fan.inlet_node,
                outlet_node: fan.outlet_node,
                owner_branch: None,
                index: index as u32,
            }
        }));
        component_node_ownership.extend(model.coils.iter().enumerate().filter_map(
            |(index, coil)| {
                let inlet_node = coil.inlet_node?;
                let outlet_node = coil.outlet_node?;
                Some(ComponentNodeOwnershipEdge {
                    component_type: NormalizedName::new(coil.kind.object_type()),
                    component_name: coil.name.clone(),
                    inlet_node,
                    outlet_node,
                    owner_branch: None,
                    index: index as u32,
                })
            },
        ));
        let diagnostics = node_graph_diagnostics(model, &component_node_ownership);
        Self {
            component_node_ownership,
            diagnostics,
        }
    }
}

/// Component inlet/outlet node ownership edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentNodeOwnershipEdge {
    /// Component object type.
    pub component_type: NormalizedName,
    /// Component object name.
    pub component_name: NormalizedName,
    /// Component inlet node.
    pub inlet_node: NodeId,
    /// Component outlet node.
    pub outlet_node: NodeId,
    /// Owning plant branch when the component is declared on a branch.
    pub owner_branch: Option<BranchId>,
    /// Zero-based component index inside the owner.
    pub index: u32,
}

/// Severity for static node graph diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeGraphDiagnosticSeverity {
    /// Diagnostic-only information.
    Info,
    /// Model graph issue that should block node graph promotion.
    Error,
}

impl NodeGraphDiagnosticSeverity {
    /// Stable severity label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Error => "error",
        }
    }
}

/// Static node graph diagnostic code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeGraphDiagnosticCode {
    /// Two typed node rows share one normalized node name.
    DuplicateNode,
    /// A typed node is not referenced by node lists, zone equipment, loops, or components.
    DanglingNode,
    /// A component has invalid or self-looping inlet/outlet nodes.
    DisconnectedComponent,
}

impl NodeGraphDiagnosticCode {
    /// Stable diagnostic code label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::DuplicateNode => "DuplicateNode",
            Self::DanglingNode => "DanglingNode",
            Self::DisconnectedComponent => "DisconnectedComponent",
        }
    }
}

/// Static node graph diagnostic, separate from conformance gate diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeGraphDiagnostic {
    /// Diagnostic severity.
    pub severity: NodeGraphDiagnosticSeverity,
    /// Diagnostic code.
    pub code: NodeGraphDiagnosticCode,
    /// Node involved when known.
    pub node: Option<NodeId>,
    /// Component involved when known.
    pub component_name: Option<NormalizedName>,
    /// Human-readable diagnostic message.
    pub message: String,
}

fn node_graph_diagnostics(
    model: &TypedModel,
    component_edges: &[ComponentNodeOwnershipEdge],
) -> Vec<NodeGraphDiagnostic> {
    let mut diagnostics = Vec::new();
    for (left_index, left) in model.nodes.iter().enumerate() {
        for right in model.nodes.iter().skip(left_index + 1) {
            if left.name == right.name {
                diagnostics.push(NodeGraphDiagnostic {
                    severity: NodeGraphDiagnosticSeverity::Error,
                    code: NodeGraphDiagnosticCode::DuplicateNode,
                    node: Some(right.id),
                    component_name: None,
                    message: format!(
                        "node {} duplicates normalized node name {}",
                        right.id.0, right.name.0
                    ),
                });
            }
        }
    }

    let referenced_nodes = referenced_node_ids(model, component_edges);
    for node in &model.nodes {
        if !referenced_nodes.contains(&node.id) {
            diagnostics.push(NodeGraphDiagnostic {
                severity: NodeGraphDiagnosticSeverity::Info,
                code: NodeGraphDiagnosticCode::DanglingNode,
                node: Some(node.id),
                component_name: None,
                message: format!(
                    "node {} is not referenced by the typed node graph",
                    node.name.0
                ),
            });
        }
    }

    for edge in component_edges {
        let inlet_exists = model.nodes.iter().any(|node| node.id == edge.inlet_node);
        let outlet_exists = model.nodes.iter().any(|node| node.id == edge.outlet_node);
        if edge.inlet_node == edge.outlet_node || !inlet_exists || !outlet_exists {
            diagnostics.push(NodeGraphDiagnostic {
                severity: NodeGraphDiagnosticSeverity::Error,
                code: NodeGraphDiagnosticCode::DisconnectedComponent,
                node: None,
                component_name: Some(edge.component_name.clone()),
                message: format!(
                    "component {} has invalid inlet/outlet node ownership",
                    edge.component_name.0
                ),
            });
        }
    }

    diagnostics
}

fn referenced_node_ids(
    model: &TypedModel,
    component_edges: &[ComponentNodeOwnershipEdge],
) -> Vec<NodeId> {
    let mut nodes = Vec::new();
    for list in &model.node_lists {
        for node in &list.nodes {
            push_unique_node(&mut nodes, *node);
        }
    }
    for system in &model.ideal_loads_air_systems {
        for node in resolve_node_or_list(model, &system.zone_supply_air_node_name) {
            push_unique_node(&mut nodes, node);
        }
    }
    for connection in &model.zone_equipment_connections {
        if let Some(node) = model.node_names.resolve(&connection.zone_air_node_name.0) {
            push_unique_node(&mut nodes, node);
        }
        if let Some(name) = &connection.zone_air_inlet_node_or_nodelist_name {
            for node in resolve_node_or_list(model, name) {
                push_unique_node(&mut nodes, node);
            }
        }
        if let Some(name) = &connection.zone_return_air_node_or_nodelist_name {
            for node in resolve_node_or_list(model, name) {
                push_unique_node(&mut nodes, node);
            }
        }
    }
    for plant_loop in &model.plant_loops {
        push_unique_node(&mut nodes, plant_loop.plant_side_inlet_node);
        push_unique_node(&mut nodes, plant_loop.plant_side_outlet_node);
        push_unique_node(&mut nodes, plant_loop.demand_side_inlet_node);
        push_unique_node(&mut nodes, plant_loop.demand_side_outlet_node);
    }
    for air_loop in &model.air_loops {
        if let Some(node) = air_loop.supply_side_inlet_node {
            push_unique_node(&mut nodes, node);
        }
        if let Some(node) = air_loop.demand_side_outlet_node {
            push_unique_node(&mut nodes, node);
        }
        for name in air_loop
            .demand_side_inlet_node_names
            .iter()
            .chain(air_loop.supply_side_outlet_node_names.iter())
        {
            for node in resolve_node_or_list(model, name) {
                push_unique_node(&mut nodes, node);
            }
        }
    }
    for edge in component_edges {
        push_unique_node(&mut nodes, edge.inlet_node);
        push_unique_node(&mut nodes, edge.outlet_node);
    }
    nodes
}

fn push_unique_node(nodes: &mut Vec<NodeId>, node: NodeId) {
    if !nodes.contains(&node) {
        nodes.push(node);
    }
}

fn unsupported_plant_topology_diagnostics(
    model: &TypedModel,
    half_loops: &[PlantHalfLoop],
) -> Vec<PlantLoopGraphDiagnostic> {
    let mut diagnostics = Vec::new();
    for half_loop in half_loops {
        if !model
            .plant_branch_lists
            .iter()
            .any(|list| list.id == half_loop.branch_list)
        {
            diagnostics.push(PlantLoopGraphDiagnostic {
                plant_loop: Some(half_loop.plant_loop),
                side: Some(half_loop.side),
                severity: PlantLoopGraphDiagnosticSeverity::Error,
                code: PlantLoopGraphDiagnosticCode::UnsupportedTopology,
                message: "plant half-loop references an unresolved branch list".to_string(),
            });
        }

        if let Some(connector_list) = half_loop.connector_list
            && !model
                .plant_connector_lists
                .iter()
                .any(|list| list.id == connector_list)
        {
            diagnostics.push(PlantLoopGraphDiagnostic {
                plant_loop: Some(half_loop.plant_loop),
                side: Some(half_loop.side),
                severity: PlantLoopGraphDiagnosticSeverity::Error,
                code: PlantLoopGraphDiagnosticCode::UnsupportedTopology,
                message: "plant half-loop references an unresolved connector list".to_string(),
            });
        }
    }
    diagnostics
}

fn plant_loop_branch_lists(model: &TypedModel) -> Vec<PlantLoopBranchListEdge> {
    model
        .plant_loops
        .iter()
        .flat_map(|plant_loop| {
            [
                PlantLoopBranchListEdge {
                    plant_loop: plant_loop.id,
                    side: PlantLoopSide::Plant,
                    branch_list: plant_loop.plant_side_branch_list,
                },
                PlantLoopBranchListEdge {
                    plant_loop: plant_loop.id,
                    side: PlantLoopSide::Demand,
                    branch_list: plant_loop.demand_side_branch_list,
                },
            ]
        })
        .collect()
}

fn sorted_zone_ideal_loads(model: &TypedModel) -> Vec<ZoneIdealLoadsEdge> {
    let mut edges: Vec<_> = model
        .zone_equipment_connections
        .iter()
        .flat_map(|connection| {
            model
                .zone_equipment_lists
                .iter()
                .find(move |list| list.id == connection.equipment_list)
                .into_iter()
                .flat_map(move |list| {
                    list.equipment.iter().map(move |entry| ZoneIdealLoadsEdge {
                        zone: connection.zone,
                        equipment_list: list.id,
                        ideal_loads_air_system: entry.ideal_loads_air_system,
                        cooling_sequence: entry.cooling_sequence,
                        heating_or_no_load_sequence: entry.heating_or_no_load_sequence,
                    })
                })
        })
        .collect();
    edges.sort_by_key(|edge| {
        (
            edge.zone,
            edge.heating_or_no_load_sequence,
            edge.cooling_sequence,
            edge.ideal_loads_air_system,
        )
    });
    edges
}

fn resolve_node_or_list(model: &TypedModel, name: &NormalizedName) -> Vec<NodeId> {
    if let Some(node) = model.node_names.resolve(&name.0) {
        return vec![node];
    }
    if let Some(node_list) = model.node_list_names.resolve(&name.0)
        && let Some(list) = model.node_lists.iter().find(|list| list.id == node_list)
    {
        return list.nodes.clone();
    }
    Vec::new()
}

/// NodeList membership relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeListMemberEdge {
    /// NodeList ID.
    pub node_list: NodeListId,
    /// Member node ID.
    pub node: NodeId,
    /// Zero-based member index.
    pub index: u32,
}

/// IdealLoads supply-node relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdealLoadsSupplyNodeEdge {
    /// IdealLoads system ID.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Resolved supply node ID.
    pub node: NodeId,
}

/// IdealLoads outdoor-air design specification relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdealLoadsOutdoorAirSpecEdge {
    /// IdealLoads system ID.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Resolved DesignSpecification:OutdoorAir ID.
    pub design_specification_outdoor_air: DesignSpecificationOutdoorAirId,
}

/// Zone air-node relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneAirNodeEdge {
    /// Zone ID.
    pub zone: ZoneId,
    /// Node ID.
    pub node: NodeId,
}

/// Zone/surface relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneSurfaceEdge {
    /// Zone ID.
    pub zone: ZoneId,
    /// Surface ID.
    pub surface: SurfaceId,
}

/// Construction/material relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstructionMaterialEdge {
    /// Construction ID.
    pub construction: ConstructionId,
    /// Material ID.
    pub material: MaterialId,
    /// Zero-based layer index.
    pub layer_index: u32,
}

/// Zone/thermostat relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneThermostatEdge {
    /// Zone ID.
    pub zone: ZoneId,
    /// Thermostat ID.
    pub thermostat: ZoneThermostatId,
}

/// Thermostat/setpoint relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThermostatSetpointEdge {
    /// Thermostat ID.
    pub thermostat: ZoneThermostatId,
    /// Dual setpoint ID.
    pub setpoint: ThermostatSetpointId,
}

/// Zone/IdealLoads relation through equipment connections and lists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneIdealLoadsEdge {
    /// Zone ID.
    pub zone: ZoneId,
    /// Equipment list ID.
    pub equipment_list: ZoneEquipmentListId,
    /// IdealLoads system ID.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Cooling sequence.
    pub cooling_sequence: u32,
    /// Heating or no-load sequence.
    pub heating_or_no_load_sequence: u32,
}

/// Side of a plant loop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantLoopSide {
    /// Supply/plant side.
    Plant,
    /// Demand side.
    Demand,
}

/// Plant loop to branch-list relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantLoopBranchListEdge {
    /// Plant loop ID.
    pub plant_loop: LoopId,
    /// Loop side.
    pub side: PlantLoopSide,
    /// Branch list ID.
    pub branch_list: BranchListId,
}

/// Branch-list membership relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantBranchListMemberEdge {
    /// Branch list ID.
    pub branch_list: BranchListId,
    /// Branch ID.
    pub branch: BranchId,
    /// Zero-based member index.
    pub index: u32,
}

/// Connector-list membership relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantConnectorListMemberEdge {
    /// Connector list ID.
    pub connector_list: ConnectorListId,
    /// Connector ID.
    pub connector: ConnectorId,
    /// Connector kind.
    pub kind: PlantConnectorKind,
    /// Zero-based member index.
    pub index: u32,
}

/// Branch to component relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlantBranchComponentEdge {
    /// Branch ID.
    pub branch: BranchId,
    /// Component object type.
    pub component_type: NormalizedName,
    /// Component name.
    pub component_name: NormalizedName,
    /// Component inlet node.
    pub inlet_node: NodeId,
    /// Component outlet node.
    pub outlet_node: NodeId,
    /// Zero-based component index.
    pub index: u32,
}

#[cfg(test)]
mod tests {
    use crate::{
        AirLoopHvac, AutoOrNumber, BranchId, BranchListId, ComponentId, ComponentRegistryCategory,
        ConnectorId, ConnectorListId, Construction, ConstructionId, ConstructionKind, FanComponent,
        FanComponentKind, InsideSurfaceConvectionAlgorithm, LoopId, Material, MaterialDefinition,
        MaterialFamily, MaterialId, MaterialKind, MaterialSurfaceRoughness, ModelGraph, NameMap,
        NoMassMaterial, Node, NodeGraphDiagnosticCode, NodeId, NormalizedName,
        OpaqueSurfaceProperties, OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm,
        PlantBranch, PlantBranchComponent, PlantBranchList, PlantConnectorKind, PlantConnectorList,
        PlantConnectorListEntry, PlantLoop, PlantLoopSide, RegularMaterial, SunExposure, Surface,
        SurfaceId, SurfaceType, TypedModel, Version, WindExposure,
        WindowGlazingSpectralAverageMaterial, ZoneId,
    };

    #[test]
    fn default_model_uses_oracle_version() {
        let model = TypedModel::default();

        assert_eq!(model.version, Version::oracle_26_1_0());
    }

    #[test]
    fn object_count_includes_explicit_surface_convection_algorithms() {
        let mut model = TypedModel::default();
        assert_eq!(model.object_count(), 1);

        model.surface_convection_algorithms.inside = Some(InsideSurfaceConvectionAlgorithm::Tarp);
        model.surface_convection_algorithms.outside = Some(OutsideSurfaceConvectionAlgorithm::Doe2);

        assert_eq!(model.object_count(), 3);
    }

    #[test]
    fn object_count_includes_explicit_global_geometry_rules() {
        let mut model = TypedModel::default();
        assert_eq!(model.object_count(), 1);

        model.global_geometry_rules = Some(crate::GlobalGeometryRules::default());

        assert_eq!(model.object_count(), 2);
    }

    #[test]
    fn ids_are_copyable_values() {
        let first = ZoneId(7);
        let second = first;

        assert_eq!(first, second);
    }

    #[test]
    fn name_map_resolves_trimmed_case_insensitive_names() {
        let mut names = NameMap::default();
        assert_eq!(names.insert("Zone One", ZoneId(0)), None);

        assert_eq!(names.resolve(" zone one "), Some(ZoneId(0)));
        assert_eq!(names.len(), 1);
    }

    #[test]
    fn material_derives_resistance_and_capacity() {
        let material = Material {
            id: MaterialId(0),
            name: NormalizedName::new("Concrete"),
            definition: MaterialDefinition::Regular(RegularMaterial {
                roughness: MaterialSurfaceRoughness::MediumRough,
                thickness_m: 0.1,
                conductivity_w_per_m_k: 2.0,
                density_kg_per_m3: 2_000.0,
                specific_heat_j_per_kg_k: 800.0,
                surface: OpaqueSurfaceProperties {
                    thermal_absorptance: 0.9,
                    solar_absorptance: 0.75,
                    visible_absorptance: 0.75,
                },
            }),
        };

        assert_eq!(material.kind(), MaterialKind::Mass);
        assert_eq!(material.thickness_m(), Some(0.1));
        assert_eq!(material.no_mass_thermal_resistance_m2_k_per_w(), None);
        assert_eq!(material.thermal_resistance(), Some(0.05));
        assert_eq!(material.heat_capacity_per_area(), Some(160_000.0));
        assert_eq!(material.thermal_absorptance(), Some(0.9));
        assert_eq!(
            MaterialSurfaceRoughness::from_energyplus_name("mediumrough"),
            Some(MaterialSurfaceRoughness::MediumRough)
        );
    }

    #[test]
    fn no_mass_material_projects_resistance_without_mass_fields() {
        let material = Material {
            id: MaterialId(0),
            name: NormalizedName::new("R1"),
            definition: MaterialDefinition::NoMass(NoMassMaterial {
                roughness: MaterialSurfaceRoughness::Rough,
                thermal_resistance_m2_k_per_w: 1.0,
                surface: OpaqueSurfaceProperties::default(),
            }),
        };

        assert_eq!(material.kind(), MaterialKind::NoMass);
        assert_eq!(material.roughness(), Some(MaterialSurfaceRoughness::Rough));
        assert_eq!(material.thickness_m(), None);
        assert_eq!(material.no_mass_thermal_resistance_m2_k_per_w(), Some(1.0));
        assert_eq!(material.thermal_resistance(), Some(1.0));
        assert_eq!(material.heat_capacity_per_area(), None);
        assert_eq!(material.solar_absorptance(), Some(0.7));
        assert_eq!(material.visible_absorptance(), Some(0.7));
    }

    #[test]
    fn window_glazing_stays_outside_opaque_material_projections() {
        let material = Material {
            id: MaterialId(0),
            name: NormalizedName::new("Clear Glass"),
            definition: MaterialDefinition::WindowGlazingSpectralAverage(
                WindowGlazingSpectralAverageMaterial {
                    thickness_m: 0.006,
                    solar_transmittance_at_normal_incidence: 0.775,
                    front_side_solar_reflectance_at_normal_incidence: 0.071,
                    back_side_solar_reflectance_at_normal_incidence: 0.071,
                    visible_transmittance_at_normal_incidence: 0.881,
                    front_side_visible_reflectance_at_normal_incidence: 0.08,
                    back_side_visible_reflectance_at_normal_incidence: 0.08,
                    infrared_transmittance_at_normal_incidence: 0.0,
                    front_side_infrared_hemispherical_emissivity: 0.84,
                    back_side_infrared_hemispherical_emissivity: 0.84,
                    conductivity_w_per_m_k: 0.9,
                    dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
                    solar_diffusing: false,
                    youngs_modulus_pa: 72.0e9,
                    poissons_ratio: 0.22,
                },
            ),
        };

        assert_eq!(material.kind(), MaterialKind::WindowGlazing);
        assert_eq!(material.family(), MaterialFamily::Fenestration);
        assert!(material.as_opaque().is_none());
        assert_eq!(
            material
                .as_window_glazing_spectral_average()
                .map(|glazing| glazing.thickness_m),
            Some(0.006)
        );
        assert_eq!(material.roughness(), None);
        assert_eq!(material.thickness_m(), None);
        assert_eq!(material.conductivity_w_per_m_k(), None);
        assert_eq!(material.density_kg_per_m3(), None);
        assert_eq!(material.specific_heat_j_per_kg_k(), None);
        assert_eq!(material.no_mass_thermal_resistance_m2_k_per_w(), None);
        assert_eq!(material.is_resistance_only(), None);
        assert_eq!(material.surface_properties(), None);
        assert_eq!(material.thermal_resistance(), None);
        assert_eq!(material.heat_capacity_per_area(), None);
    }

    #[test]
    fn model_graph_links_surfaces_and_constructions() {
        let mut model = TypedModel::default();
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            kind: ConstructionKind::Opaque,
            outside_layer: MaterialId(0),
            layers: vec![MaterialId(0), MaterialId(1)],
        });
        model.surfaces.push(Surface {
            id: SurfaceId(0),
            name: NormalizedName::new("Surface"),
            surface_type: SurfaceType::Wall,
            construction: ConstructionId(0),
            zone: ZoneId(0),
            outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
            outside_boundary_condition_object: None,
            sun_exposure: SunExposure::SunExposed,
            wind_exposure: WindExposure::WindExposed,
            view_factor_to_ground: AutoOrNumber::AutoCalculate,
            vertices: Vec::new(),
        });

        let graph = ModelGraph::from_typed(&model);

        assert_eq!(graph.zone_surfaces[0].zone, ZoneId(0));
        assert_eq!(graph.zone_surfaces[0].surface, SurfaceId(0));
        assert_eq!(graph.construction_materials.len(), 2);
        assert_eq!(graph.construction_materials[0].material, MaterialId(0));
        assert_eq!(graph.construction_materials[0].layer_index, 0);
        assert_eq!(graph.construction_materials[1].material, MaterialId(1));
        assert_eq!(graph.construction_materials[1].layer_index, 1);
    }

    #[test]
    fn node_graph_records_component_node_ownership_and_diagnostics() {
        let mut model = TypedModel::default();
        model.nodes.push(Node {
            id: NodeId(0),
            name: NormalizedName::new("Component Inlet"),
        });
        model.nodes.push(Node {
            id: NodeId(1),
            name: NormalizedName::new("Component Outlet"),
        });
        model.nodes.push(Node {
            id: NodeId(2),
            name: NormalizedName::new("Dangling Node"),
        });
        model.plant_branches.push(PlantBranch {
            id: BranchId(0),
            name: NormalizedName::new("Branch"),
            components: vec![
                PlantBranchComponent {
                    object_type: NormalizedName::new("Pump:ConstantSpeed"),
                    name: NormalizedName::new("Pump"),
                    inlet_node: NodeId(0),
                    outlet_node: NodeId(1),
                },
                PlantBranchComponent {
                    object_type: NormalizedName::new("Boiler:HotWater"),
                    name: NormalizedName::new("Self Loop"),
                    inlet_node: NodeId(1),
                    outlet_node: NodeId(1),
                },
            ],
        });

        let graph = ModelGraph::from_typed(&model);

        assert_eq!(graph.node_graph.component_node_ownership.len(), 2);
        assert_eq!(
            graph.node_graph.component_node_ownership[0].inlet_node,
            NodeId(0)
        );
        assert_eq!(
            graph.node_graph.component_node_ownership[0].outlet_node,
            NodeId(1)
        );
        assert!(
            graph
                .node_graph
                .diagnostics
                .iter()
                .any(
                    |diagnostic| diagnostic.code == NodeGraphDiagnosticCode::DanglingNode
                        && diagnostic.node == Some(NodeId(2))
                )
        );
        assert!(
            graph
                .node_graph
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code
                    == NodeGraphDiagnosticCode::DisconnectedComponent)
        );
    }

    #[test]
    fn air_loop_graph_compiles_branch_order_and_component_registry() {
        let mut model = TypedModel::default();
        model.nodes.push(Node {
            id: NodeId(0),
            name: NormalizedName::new("Fan Inlet"),
        });
        model.nodes.push(Node {
            id: NodeId(1),
            name: NormalizedName::new("Fan Outlet"),
        });
        model.fans.push(FanComponent {
            id: ComponentId(0),
            kind: FanComponentKind::ConstantVolume,
            name: NormalizedName::new("Supply Fan"),
            availability_schedule: None,
            inlet_node: NodeId(0),
            outlet_node: NodeId(1),
            maximum_flow_rate_m3_per_s: Some(0.5),
            pressure_rise_pa: Some(500.0),
        });
        model.plant_branches.push(PlantBranch {
            id: BranchId(0),
            name: NormalizedName::new("Supply Branch"),
            components: vec![PlantBranchComponent {
                object_type: NormalizedName::new("Fan:ConstantVolume"),
                name: NormalizedName::new("Supply Fan"),
                inlet_node: NodeId(0),
                outlet_node: NodeId(1),
            }],
        });
        model.plant_branch_lists.push(PlantBranchList {
            id: BranchListId(0),
            name: NormalizedName::new("Air Loop Branches"),
            branches: vec![BranchId(0)],
        });
        model.air_loops.push(AirLoopHvac {
            id: LoopId(0),
            name: NormalizedName::new("Air Loop"),
            availability_manager_list_name: None,
            branch_list: Some(BranchListId(0)),
            connector_list: None,
            supply_side_inlet_node: Some(NodeId(0)),
            demand_side_outlet_node: Some(NodeId(1)),
            demand_side_inlet_node_names: Vec::new(),
            supply_side_outlet_node_names: Vec::new(),
        });

        let graph = ModelGraph::from_typed(&model);

        assert_eq!(graph.air_loop_graph.branch_lists.len(), 1);
        assert_eq!(graph.air_loop_graph.branch_list_members.len(), 1);
        assert_eq!(graph.air_loop_graph.execution_order.len(), 1);
        assert_eq!(
            graph.air_loop_graph.execution_order[0].component_type,
            NormalizedName::new("Fan:ConstantVolume")
        );
        assert!(
            graph
                .component_registry
                .entries
                .iter()
                .any(|entry| entry.category == ComponentRegistryCategory::Fan
                    && entry.name == NormalizedName::new("Supply Fan"))
        );
        assert!(
            graph
                .node_graph
                .component_node_ownership
                .iter()
                .any(
                    |edge| edge.component_name == NormalizedName::new("Supply Fan")
                        && edge.inlet_node == NodeId(0)
                        && edge.outlet_node == NodeId(1)
                )
        );
    }

    #[test]
    fn plant_loop_graph_compiles_half_loops_and_component_registry() {
        let mut model = TypedModel::default();
        for (id, name) in [
            (0, "HW Supply Inlet"),
            (1, "HW Pump Outlet"),
            (2, "HW Supply Outlet"),
            (3, "HW Demand Inlet"),
            (4, "HW Demand Outlet"),
        ] {
            model.nodes.push(Node {
                id: NodeId(id),
                name: NormalizedName::new(name),
            });
        }
        model.plant_branches.extend([
            PlantBranch {
                id: BranchId(0),
                name: NormalizedName::new("HW Supply Inlet Branch"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("Pump:ConstantSpeed"),
                    name: NormalizedName::new("HW Pump"),
                    inlet_node: NodeId(0),
                    outlet_node: NodeId(1),
                }],
            },
            PlantBranch {
                id: BranchId(1),
                name: NormalizedName::new("HW Boiler Branch"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("Boiler:HotWater"),
                    name: NormalizedName::new("HW Boiler"),
                    inlet_node: NodeId(1),
                    outlet_node: NodeId(2),
                }],
            },
            PlantBranch {
                id: BranchId(2),
                name: NormalizedName::new("HW Demand Branch"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("Pipe:Adiabatic"),
                    name: NormalizedName::new("HW Demand Pipe"),
                    inlet_node: NodeId(3),
                    outlet_node: NodeId(4),
                }],
            },
        ]);
        model.plant_branch_lists.extend([
            PlantBranchList {
                id: BranchListId(0),
                name: NormalizedName::new("HW Supply Branches"),
                branches: vec![BranchId(0), BranchId(1)],
            },
            PlantBranchList {
                id: BranchListId(1),
                name: NormalizedName::new("HW Demand Branches"),
                branches: vec![BranchId(2)],
            },
        ]);
        model.plant_connector_lists.push(PlantConnectorList {
            id: ConnectorListId(0),
            name: NormalizedName::new("HW Supply Connectors"),
            connectors: vec![
                PlantConnectorListEntry {
                    kind: PlantConnectorKind::Splitter,
                    connector: ConnectorId(0),
                },
                PlantConnectorListEntry {
                    kind: PlantConnectorKind::Mixer,
                    connector: ConnectorId(1),
                },
            ],
        });
        model.plant_loops.push(PlantLoop {
            id: LoopId(0),
            name: NormalizedName::new("Hot Water Loop"),
            fluid_type: NormalizedName::new("Water"),
            plant_side_inlet_node: NodeId(0),
            plant_side_outlet_node: NodeId(2),
            plant_side_branch_list: BranchListId(0),
            plant_side_connector_list: Some(ConnectorListId(0)),
            demand_side_inlet_node: NodeId(3),
            demand_side_outlet_node: NodeId(4),
            demand_side_branch_list: BranchListId(1),
            demand_side_connector_list: None,
            load_distribution_scheme: Some(NormalizedName::new("SequentialLoad")),
        });

        let graph = ModelGraph::from_typed(&model);

        assert_eq!(graph.plant_loop_graph.half_loops.len(), 2);
        assert!(
            graph
                .plant_loop_graph
                .half_loops
                .iter()
                .any(|half_loop| half_loop.side == PlantLoopSide::Plant
                    && half_loop.inlet_node == NodeId(0)
                    && half_loop.outlet_node == NodeId(2))
        );
        assert!(
            graph
                .plant_loop_graph
                .half_loops
                .iter()
                .any(|half_loop| half_loop.side == PlantLoopSide::Demand
                    && half_loop.inlet_node == NodeId(3)
                    && half_loop.outlet_node == NodeId(4))
        );
        assert_eq!(graph.plant_loop_graph.branch_list_members.len(), 3);
        assert_eq!(graph.plant_loop_graph.connector_list_members.len(), 2);
        assert_eq!(graph.plant_loop_graph.component_registry.len(), 3);
        assert!(
            graph
                .plant_loop_graph
                .component_registry
                .iter()
                .any(|entry| entry.side == PlantLoopSide::Plant
                    && entry.component_name == NormalizedName::new("HW Boiler")
                    && entry.inlet_node == NodeId(1)
                    && entry.outlet_node == NodeId(2))
        );
        assert!(graph.plant_loop_graph.diagnostics.is_empty());
    }
}
