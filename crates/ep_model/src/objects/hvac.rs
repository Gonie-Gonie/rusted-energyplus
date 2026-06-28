use crate::{
    BranchListId, ComponentId, ConnectorListId, LoopId, NodeId, NormalizedName, ScheduleId,
};

/// Typed `AirLoopHVAC` shell for the first air-loop graph subset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AirLoopHvac {
    /// Typed loop ID.
    pub id: LoopId,
    /// Air loop name.
    pub name: NormalizedName,
    /// Optional availability manager list name as declared by EnergyPlus input.
    pub availability_manager_list_name: Option<NormalizedName>,
    /// Optional branch list for the air loop.
    pub branch_list: Option<BranchListId>,
    /// Optional connector list for the air loop.
    pub connector_list: Option<ConnectorListId>,
    /// Supply-side inlet node.
    pub supply_side_inlet_node: Option<NodeId>,
    /// Demand-side outlet node.
    pub demand_side_outlet_node: Option<NodeId>,
    /// Demand-side inlet node or node-list names.
    pub demand_side_inlet_node_names: Vec<NormalizedName>,
    /// Supply-side outlet node or node-list names.
    pub supply_side_outlet_node_names: Vec<NormalizedName>,
}

/// Fan component kinds tracked by the HVAC component registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FanComponentKind {
    /// `Fan:ConstantVolume`.
    ConstantVolume,
    /// `Fan:OnOff`.
    OnOff,
    /// `Fan:VariableVolume`.
    VariableVolume,
    /// `Fan:SystemModel`.
    SystemModel,
}

impl FanComponentKind {
    /// EnergyPlus object type for this fan kind.
    #[must_use]
    pub const fn object_type(self) -> &'static str {
        match self {
            Self::ConstantVolume => "Fan:ConstantVolume",
            Self::OnOff => "Fan:OnOff",
            Self::VariableVolume => "Fan:VariableVolume",
            Self::SystemModel => "Fan:SystemModel",
        }
    }
}

/// Fan component identity and node endpoints.
#[derive(Clone, Debug, PartialEq)]
pub struct FanComponent {
    /// Typed component ID within the fan subset.
    pub id: ComponentId,
    /// Fan kind.
    pub kind: FanComponentKind,
    /// Fan name.
    pub name: NormalizedName,
    /// Optional availability schedule.
    pub availability_schedule: Option<ScheduleId>,
    /// Fan inlet node.
    pub inlet_node: NodeId,
    /// Fan outlet node.
    pub outlet_node: NodeId,
    /// Optional maximum flow rate in m3/s.
    pub maximum_flow_rate_m3_per_s: Option<f64>,
    /// Optional pressure rise in Pa.
    pub pressure_rise_pa: Option<f64>,
}

/// Coil component kinds tracked by the HVAC component registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoilComponentKind {
    /// `Coil:Heating:Electric`.
    HeatingElectric,
    /// `Coil:Heating:Fuel`.
    HeatingFuel,
    /// `Coil:Heating:Water`.
    HeatingWater,
    /// `Coil:Cooling:Water`.
    CoolingWater,
    /// `Coil:Cooling:DX:SingleSpeed`.
    CoolingDxSingleSpeed,
}

impl CoilComponentKind {
    /// EnergyPlus object type for this coil kind.
    #[must_use]
    pub const fn object_type(self) -> &'static str {
        match self {
            Self::HeatingElectric => "Coil:Heating:Electric",
            Self::HeatingFuel => "Coil:Heating:Fuel",
            Self::HeatingWater => "Coil:Heating:Water",
            Self::CoolingWater => "Coil:Cooling:Water",
            Self::CoolingDxSingleSpeed => "Coil:Cooling:DX:SingleSpeed",
        }
    }
}

/// Coil component identity and optional node endpoints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoilComponent {
    /// Typed component ID within the coil subset.
    pub id: ComponentId,
    /// Coil kind.
    pub kind: CoilComponentKind,
    /// Coil name.
    pub name: NormalizedName,
    /// Optional inlet node.
    pub inlet_node: Option<NodeId>,
    /// Optional outlet node.
    pub outlet_node: Option<NodeId>,
    /// Optional availability schedule.
    pub availability_schedule: Option<ScheduleId>,
}

/// Setpoint manager source-map entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HvacSourceMap {
    /// EnergyPlus source file.
    pub source_file: &'static str,
    /// EnergyPlus source routine.
    pub source_routine: &'static str,
    /// Boundary policy for the current Rust subset.
    pub policy: &'static str,
}

/// Source map for setpoint manager dispatch.
pub const SETPOINT_MANAGER_SOURCE_MAP: HvacSourceMap = HvacSourceMap {
    source_file: "src/EnergyPlus/SetPointManager.cc",
    source_routine: "ManageSetPoints",
    policy: "source-map-only until AirLoopHVAC component simulation is promoted",
};

/// Source map for availability manager dispatch.
pub const AVAILABILITY_MANAGER_SOURCE_MAP: HvacSourceMap = HvacSourceMap {
    source_file: "src/EnergyPlus/SystemAvailabilityManager.cc",
    source_routine: "ManageSystemAvailability",
    policy: "source-map-only until AirLoopHVAC component simulation is promoted",
};

/// Setpoint manager identity tracked for source-map coverage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetpointManagerComponent {
    /// Typed component ID within the setpoint-manager subset.
    pub id: ComponentId,
    /// EnergyPlus object type.
    pub object_type: NormalizedName,
    /// Manager name.
    pub name: NormalizedName,
    /// Controlled node when present in the input object.
    pub setpoint_node: Option<NodeId>,
}

/// Availability manager identity tracked for source-map coverage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AvailabilityManagerComponent {
    /// Typed component ID within the availability-manager subset.
    pub id: ComponentId,
    /// EnergyPlus object type.
    pub object_type: NormalizedName,
    /// Manager name.
    pub name: NormalizedName,
    /// Optional schedule reference.
    pub schedule: Option<ScheduleId>,
}
