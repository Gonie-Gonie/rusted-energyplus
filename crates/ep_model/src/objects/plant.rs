use crate::{
    AutosizeOrNumber, BranchId, BranchListId, ComponentId, ConnectorId, ConnectorListId, LoopId,
    NodeId, NormalizedName,
};

/// One central plant loop shell.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantLoop {
    /// Typed ID.
    pub id: LoopId,
    /// Plant loop name.
    pub name: NormalizedName,
    /// Fluid type as declared by EnergyPlus input.
    pub fluid_type: NormalizedName,
    /// Plant side inlet node.
    pub plant_side_inlet_node: NodeId,
    /// Plant side outlet node.
    pub plant_side_outlet_node: NodeId,
    /// Plant side branch list.
    pub plant_side_branch_list: BranchListId,
    /// Optional plant side connector list.
    pub plant_side_connector_list: Option<ConnectorListId>,
    /// Demand side inlet node.
    pub demand_side_inlet_node: NodeId,
    /// Demand side outlet node.
    pub demand_side_outlet_node: NodeId,
    /// Demand side branch list.
    pub demand_side_branch_list: BranchListId,
    /// Optional demand side connector list.
    pub demand_side_connector_list: Option<ConnectorListId>,
    /// Load distribution scheme as declared by EnergyPlus input.
    pub load_distribution_scheme: Option<NormalizedName>,
}

/// Component reference inside one plant branch.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantBranchComponent {
    /// Component object type.
    pub object_type: NormalizedName,
    /// Component object name.
    pub name: NormalizedName,
    /// Component inlet node.
    pub inlet_node: NodeId,
    /// Component outlet node.
    pub outlet_node: NodeId,
}

/// Plant branch with ordered components.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantBranch {
    /// Typed ID.
    pub id: BranchId,
    /// Branch name.
    pub name: NormalizedName,
    /// Ordered branch components.
    pub components: Vec<PlantBranchComponent>,
}

/// Ordered branch list.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantBranchList {
    /// Typed ID.
    pub id: BranchListId,
    /// Branch list name.
    pub name: NormalizedName,
    /// Branches in EnergyPlus flow order.
    pub branches: Vec<BranchId>,
}

/// Connector type supported by the plant skeleton.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantConnectorKind {
    /// `Connector:Splitter`.
    Splitter,
    /// `Connector:Mixer`.
    Mixer,
}

/// Plant connector with resolved branch references.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantConnector {
    /// Typed ID.
    pub id: ConnectorId,
    /// Connector name.
    pub name: NormalizedName,
    /// Connector kind.
    pub kind: PlantConnectorKind,
    /// Inlet branches for the connector.
    pub inlet_branches: Vec<BranchId>,
    /// Outlet branches for the connector.
    pub outlet_branches: Vec<BranchId>,
}

/// Connector reference inside a connector list.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantConnectorListEntry {
    /// Connector kind.
    pub kind: PlantConnectorKind,
    /// Connector ID.
    pub connector: ConnectorId,
}

/// Ordered plant connector list.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantConnectorList {
    /// Typed ID.
    pub id: ConnectorListId,
    /// Connector list name.
    pub name: NormalizedName,
    /// Connector entries in EnergyPlus order.
    pub connectors: Vec<PlantConnectorListEntry>,
}

/// Generic typed `BranchList` alias used by AirLoop and Plant graph skeletons.
pub type BranchList = PlantBranchList;

/// Generic typed `ConnectorList` alias used by AirLoop and Plant graph skeletons.
pub type ConnectorList = PlantConnectorList;

/// Typed `Pump:ConstantSpeed` identity and node endpoints.
#[derive(Clone, Debug, PartialEq)]
pub struct PumpConstantSpeed {
    /// Typed ID within the constant-speed pump subset.
    pub id: ComponentId,
    /// Pump name.
    pub name: NormalizedName,
    /// Inlet node.
    pub inlet_node: NodeId,
    /// Outlet node.
    pub outlet_node: NodeId,
    /// Optional design flow rate in m3/s.
    pub design_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
    /// Optional design pump head in Pa.
    pub design_pump_head_pa: Option<f64>,
    /// Pump control type string.
    pub pump_control_type: Option<NormalizedName>,
}

/// Typed `Boiler:HotWater` identity and node endpoints.
#[derive(Clone, Debug, PartialEq)]
pub struct BoilerHotWater {
    /// Typed ID within the hot-water boiler subset.
    pub id: ComponentId,
    /// Boiler name.
    pub name: NormalizedName,
    /// Fuel type string.
    pub fuel_type: Option<NormalizedName>,
    /// Inlet node.
    pub inlet_node: NodeId,
    /// Outlet node.
    pub outlet_node: NodeId,
    /// Optional nominal capacity in W.
    pub nominal_capacity_w: Option<AutosizeOrNumber>,
    /// Optional design water flow rate in m3/s.
    pub design_water_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
}

/// Typed `Chiller:Electric:EIR` identity and node endpoints.
#[derive(Clone, Debug, PartialEq)]
pub struct ChillerElectricEir {
    /// Typed ID within the electric EIR chiller subset.
    pub id: ComponentId,
    /// Chiller name.
    pub name: NormalizedName,
    /// Chilled water inlet node.
    pub chilled_water_inlet_node: NodeId,
    /// Chilled water outlet node.
    pub chilled_water_outlet_node: NodeId,
    /// Condenser inlet node, when declared.
    pub condenser_inlet_node: Option<NodeId>,
    /// Condenser outlet node, when declared.
    pub condenser_outlet_node: Option<NodeId>,
    /// Optional reference capacity in W.
    pub reference_capacity_w: Option<AutosizeOrNumber>,
    /// Optional reference COP.
    pub reference_cop: Option<f64>,
}
