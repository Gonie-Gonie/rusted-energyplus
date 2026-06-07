//! Aggregate typed model and graph structures.

use crate::{
    BoilerHotWater, BranchId, BranchListId, Building, ChillerElectricEir, ComponentId, ConnectorId,
    ConnectorListId, Construction, ConstructionId, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    InternalGainId, LoopId, Material, MaterialId, NameMap, Node, NodeId, NodeList, NodeListId,
    NormalizedName, OtherEquipment, PlantBranch, PlantBranchList, PlantConnector,
    PlantConnectorKind, PlantConnectorList, PlantLoop, PumpConstantSpeed, RunPeriod, RunPeriodId,
    ScheduleCompact, ScheduleConstant, ScheduleId, ScheduleTypeLimitId, ScheduleTypeLimits,
    SiteLocation, Surface, SurfaceId, ThermostatDualSetpoint, ThermostatSetpointId, TimestepConfig,
    Version, Zone, ZoneEquipmentConnection, ZoneEquipmentList, ZoneEquipmentListId, ZoneId,
    ZoneThermostat, ZoneThermostatId,
};

/// Minimal typed model for early compiler stages.
#[derive(Clone, Debug, PartialEq)]
pub struct TypedModel {
    /// Model version.
    pub version: Version,
    /// Building settings.
    pub building: Option<Building>,
    /// Zone timestep config.
    pub timestep: TimestepConfig,
    /// Run periods.
    pub run_periods: Vec<RunPeriod>,
    /// Run period names.
    pub run_period_names: NameMap<RunPeriodId>,
    /// Site location.
    pub site: Option<SiteLocation>,
    /// Materials.
    pub materials: Vec<Material>,
    /// Material names.
    pub material_names: NameMap<MaterialId>,
    /// Constructions.
    pub constructions: Vec<Construction>,
    /// Construction names.
    pub construction_names: NameMap<ConstructionId>,
    /// Schedule type limits.
    pub schedule_type_limits: Vec<ScheduleTypeLimits>,
    /// Schedule type limit names.
    pub schedule_type_limit_names: NameMap<ScheduleTypeLimitId>,
    /// Constant schedules.
    pub schedules: Vec<ScheduleConstant>,
    /// Compact schedules.
    pub compact_schedules: Vec<ScheduleCompact>,
    /// Schedule names.
    pub schedule_names: NameMap<ScheduleId>,
    /// Zone internal gains from OtherEquipment objects.
    pub other_equipment: Vec<OtherEquipment>,
    /// OtherEquipment names.
    pub other_equipment_names: NameMap<InternalGainId>,
    /// Dual setpoint thermostat objects.
    pub thermostat_dual_setpoints: Vec<ThermostatDualSetpoint>,
    /// Dual setpoint names.
    pub thermostat_dual_setpoint_names: NameMap<ThermostatSetpointId>,
    /// Zone thermostat controls.
    pub zone_thermostats: Vec<ZoneThermostat>,
    /// Zone thermostat names.
    pub zone_thermostat_names: NameMap<ZoneThermostatId>,
    /// IdealLoads air systems.
    pub ideal_loads_air_systems: Vec<IdealLoadsAirSystem>,
    /// IdealLoads air system names.
    pub ideal_loads_air_system_names: NameMap<IdealLoadsAirSystemId>,
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
            timestep: TimestepConfig::default(),
            run_periods: Vec::new(),
            run_period_names: NameMap::default(),
            site: None,
            materials: Vec::new(),
            material_names: NameMap::default(),
            constructions: Vec::new(),
            construction_names: NameMap::default(),
            schedule_type_limits: Vec::new(),
            schedule_type_limit_names: NameMap::default(),
            schedules: Vec::new(),
            compact_schedules: Vec::new(),
            schedule_names: NameMap::default(),
            other_equipment: Vec::new(),
            other_equipment_names: NameMap::default(),
            thermostat_dual_setpoints: Vec::new(),
            thermostat_dual_setpoint_names: NameMap::default(),
            zone_thermostats: Vec::new(),
            zone_thermostat_names: NameMap::default(),
            ideal_loads_air_systems: Vec::new(),
            ideal_loads_air_system_names: NameMap::default(),
            zone_equipment_lists: Vec::new(),
            zone_equipment_list_names: NameMap::default(),
            zone_equipment_connections: Vec::new(),
            nodes: Vec::new(),
            node_names: NameMap::default(),
            node_lists: Vec::new(),
            node_list_names: NameMap::default(),
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
    /// Number of typed object instances in the current subset.
    #[must_use]
    pub fn object_count(&self) -> usize {
        usize::from(self.building.is_some())
            + usize::from(self.site.is_some())
            + 1
            + self.run_periods.len()
            + self.materials.len()
            + self.constructions.len()
            + self.schedule_type_limits.len()
            + self.schedules.len()
            + self.compact_schedules.len()
            + self.other_equipment.len()
            + self.thermostat_dual_setpoints.len()
            + self.zone_thermostats.len()
            + self.ideal_loads_air_systems.len()
            + self.zone_equipment_lists.len()
            + self.zone_equipment_connections.len()
            + self.node_lists.len()
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
                .map(|construction| ConstructionMaterialEdge {
                    construction: construction.id,
                    material: construction.outside_layer,
                    layer_index: 0,
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
        }
    }
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
        AutoOrNumber, Construction, ConstructionId, Material, MaterialId, MaterialKind, ModelGraph,
        NameMap, NormalizedName, OutsideBoundaryCondition, SunExposure, Surface, SurfaceId,
        SurfaceType, TypedModel, Version, WindExposure, ZoneId,
    };

    #[test]
    fn default_model_uses_oracle_version() {
        let model = TypedModel::default();

        assert_eq!(model.version, Version::oracle_26_1_0());
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
            kind: MaterialKind::Mass,
            conductivity_w_per_m_k: Some(2.0),
            density_kg_per_m3: Some(2_000.0),
            specific_heat_j_per_kg_k: Some(800.0),
            thickness_m: Some(0.1),
            thermal_resistance_m2_k_per_w: None,
        };

        assert_eq!(material.thermal_resistance(), Some(0.05));
        assert_eq!(material.heat_capacity_per_area(), Some(160_000.0));
    }

    #[test]
    fn model_graph_links_surfaces_and_constructions() {
        let mut model = TypedModel::default();
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
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
        assert_eq!(graph.construction_materials[0].material, MaterialId(0));
    }
}
