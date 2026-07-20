use crate::{
    IdealLoadsAirSystemId, NodeId, NodeListId, NormalizedName, ScheduleId,
    ZoneEquipmentConnectionId, ZoneEquipmentListId, ZoneId,
};

/// Typed air-side node discovered from node lists, local environments, and HVAC references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    /// Typed ID.
    pub id: NodeId,
    /// Node name.
    pub name: NormalizedName,
}

/// Typed `NodeList` input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeList {
    /// Typed ID.
    pub id: NodeListId,
    /// NodeList name.
    pub name: NormalizedName,
    /// Member nodes in declared order.
    pub nodes: Vec<NodeId>,
}

/// Zone equipment load distribution scheme.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadDistributionScheme {
    /// Sequential load distribution.
    SequentialLoad,
    /// Uniform load distribution.
    UniformLoad,
    /// Uniform part-load-ratio distribution.
    UniformPlr,
    /// Sequential uniform part-load-ratio distribution.
    SequentialUniformPlr,
}

/// Zone equipment object types supported by the first HVAC graph subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZoneEquipmentObjectType {
    /// `ZoneHVAC:IdealLoadsAirSystem`.
    IdealLoadsAirSystem,
}

/// One item in `ZoneHVAC:EquipmentList`.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneEquipmentListEntry {
    /// Equipment object type.
    pub object_type: ZoneEquipmentObjectType,
    /// Referenced IdealLoads air system.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Cooling sequence.
    pub cooling_sequence: u32,
    /// Heating or no-load sequence.
    pub heating_or_no_load_sequence: u32,
    /// Optional sequential cooling fraction schedule.
    pub sequential_cooling_fraction_schedule: Option<ScheduleId>,
    /// Optional sequential heating fraction schedule.
    pub sequential_heating_fraction_schedule: Option<ScheduleId>,
}

/// Zone equipment list.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneEquipmentList {
    /// Typed ID.
    pub id: ZoneEquipmentListId,
    /// Object name.
    pub name: NormalizedName,
    /// Load distribution scheme.
    pub load_distribution_scheme: LoadDistributionScheme,
    /// Ordered equipment entries.
    pub equipment: Vec<ZoneEquipmentListEntry>,
}

/// Zone HVAC equipment connections.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneEquipmentConnection {
    /// Typed ID.
    pub id: ZoneEquipmentConnectionId,
    /// Connected zone.
    pub zone: ZoneId,
    /// Conditioning equipment list.
    pub equipment_list: ZoneEquipmentListId,
    /// Zone air inlet node or node list name.
    pub zone_air_inlet_node_or_nodelist_name: Option<NormalizedName>,
    /// Zone air exhaust node or node list name.
    pub zone_air_exhaust_node_or_nodelist_name: Option<NormalizedName>,
    /// Zone air node name.
    pub zone_air_node_name: NormalizedName,
    /// Zone return air node or node list name.
    pub zone_return_air_node_or_nodelist_name: Option<NormalizedName>,
    /// Optional return-air fraction schedule.
    pub zone_return_air_node_1_flow_rate_fraction_schedule: Option<ScheduleId>,
    /// Optional return-air basis node or node list.
    pub zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: Option<NormalizedName>,
}
