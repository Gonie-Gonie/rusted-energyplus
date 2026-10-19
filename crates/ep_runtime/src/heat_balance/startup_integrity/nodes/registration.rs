//! Native RegisterNodeConnection651-813 on the sole existing shared owner.
use super::{Error, SourceBranchNodeConnections, extent};

/// Actual ConnectionObjectType ordinals in the finite Zone equipment closure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceNodeObjectType {
    Invalid = -1,
    ZoneHvacEquipmentConnections = 261,
    ZoneHvacIdealLoadsAirSystem = 268,
    #[allow(
        dead_code,
        reason = "DataLoopNode.hh connection/object enumerations retain exact original alternatives and count sentinels; finite Zone equipment does not construct every connection kind."
    )]
    SpaceHvacEquipmentConnections = 282,
    #[allow(
        dead_code,
        reason = "DataLoopNode.hh connection/object enumerations retain exact original alternatives and count sentinels; finite Zone equipment does not construct every connection kind."
    )]
    Num = 286,
}
impl SourceNodeObjectType {
    pub(crate) fn source_name(self) -> Result<&'static str, Error> {
        match self {
            Self::ZoneHvacEquipmentConnections => Ok("ZoneHVAC:EquipmentConnections"),
            Self::ZoneHvacIdealLoadsAirSystem => Ok("ZoneHVAC:IdealLoadsAirSystem"),
            Self::SpaceHvacEquipmentConnections => Ok("SpaceHVAC:EquipmentConnections"),
            _ => Err(Error::Unavailable(
                "actual finite Node ConnectionObjectType name domain",
            )),
        }
    }
}

/// Whole Native DataLoopNode ConnectionType enumeration, including sentinels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceNodeConnectionType {
    Invalid = -1,
    #[allow(
        dead_code,
        reason = "DataLoopNode.hh connection/object enumerations retain exact original alternatives and count sentinels; finite Zone equipment does not construct every connection kind."
    )]
    Blank = 0,
    Inlet = 1,
    Outlet = 2,
    #[allow(
        dead_code,
        reason = "DataLoopNode.hh connection/object enumerations retain exact original alternatives and count sentinels; finite Zone equipment does not construct every connection kind."
    )]
    Internal = 3,
    ZoneNode = 4,
    Sensor = 5,
    Actuator = 6,
    OutsideAir = 7,
    #[allow(
        dead_code,
        reason = "DataLoopNode.hh connection/object enumerations retain exact original alternatives and count sentinels; finite Zone equipment does not construct every connection kind."
    )]
    ReliefAir = 8,
    ZoneInlet = 9,
    ZoneReturn = 10,
    ZoneExhaust = 11,
    SetPoint = 12,
    #[allow(
        dead_code,
        reason = "DataLoopNode.hh connection/object enumerations retain exact original alternatives and count sentinels; finite Zone equipment does not construct every connection kind."
    )]
    Electric = 13,
    OutsideAirReference = 14,
    InducedAir = 15,
    #[allow(
        dead_code,
        reason = "DataLoopNode.hh connection/object enumerations retain exact original alternatives and count sentinels; finite Zone equipment does not construct every connection kind."
    )]
    Num = 16,
}

/// Whole named Native CompFluidStream enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceCompFluidStream {
    Invalid = -1,
    Primary = 1,
    Secondary = 2,
    Tertiary = 3,
    Quaternary = 4,
    Num = 5,
}
impl SourceCompFluidStream {
    pub(crate) fn source_increment(self, increment: usize) -> Result<Self, Error> {
        let increment = i32::try_from(increment)
            .map_err(|_| Error::Unavailable("actual Node fluid-stream signed increment domain"))?;
        let value = (self as i32)
            .checked_add(increment)
            .ok_or(Error::Unavailable(
                "actual Node fluid-stream signed addition domain",
            ))?;
        match value {
            -1 => Ok(Self::Invalid),
            1 => Ok(Self::Primary),
            2 => Ok(Self::Secondary),
            3 => Ok(Self::Tertiary),
            4 => Ok(Self::Quaternary),
            5 => Ok(Self::Num),
            _ => Err(Error::Unavailable(
                "actual Node fluid-stream unnamed enum value",
            )),
        }
    }
}

/// Whole Native NodeConnectionDef constructor and field order.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceNodeConnection {
    pub(super) node_number: usize,
    pub(super) node_name: String,
    pub(super) object_type: SourceNodeObjectType,
    pub(super) object_name: String,
    pub(super) connection_type: SourceNodeConnectionType,
    pub(super) fluid_stream: SourceCompFluidStream,
    pub(super) object_is_parent: bool,
}
impl Default for SourceNodeConnection {
    fn default() -> Self {
        Self {
            node_number: 0,
            node_name: String::new(),
            object_type: SourceNodeObjectType::Invalid,
            object_name: String::new(),
            connection_type: SourceNodeConnectionType::Invalid,
            fluid_stream: SourceCompFluidStream::Invalid,
            object_is_parent: false,
        }
    }
}

impl SourceBranchNodeConnections {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn register_source_node_connection(
        &mut self,
        node_number: usize,
        node_name: &str,
        object_type: SourceNodeObjectType,
        object_name: &str,
        connection_type: SourceNodeConnectionType,
        fluid_stream: SourceCompFluidStream,
        is_parent: bool,
        _errors_found: &mut bool,
        _input_field_name: &str,
    ) -> Result<(), Error> {
        i32::try_from(node_number).map_err(|_| {
            Error::Unavailable("actual RegisterNodeConnection NodeNumber signed domain")
        })?;
        if matches!(
            object_type,
            SourceNodeObjectType::Invalid | SourceNodeObjectType::Num
        ) {
            return Err(Error::Unavailable(
                "actual RegisterNodeConnection680 invalid-object severe diagnostic",
            ));
        }
        let _object_type_name = object_type.source_name()?;
        if matches!(
            connection_type,
            SourceNodeConnectionType::Invalid | SourceNodeConnectionType::Num
        ) {
            return Err(Error::Unavailable(
                "actual RegisterNodeConnection689 invalid-connection severe diagnostic",
            ));
        }
        let mut make_new = true;
        for index in 0..extent(self.number_node_connections)? {
            let row = self
                .node_connections
                .as_ref()
                .and_then(|rows| rows.get(index))
                .ok_or(Error::Unavailable(
                    "actual RegisterNodeConnection existing record allocation",
                ))?;
            if row.node_number != node_number {
                continue;
            }
            if row.object_type != object_type {
                continue;
            }
            if !row.object_name.eq_ignore_ascii_case(object_name) {
                continue;
            }
            if row.connection_type != connection_type {
                continue;
            }
            if row.fluid_stream != fluid_stream {
                continue;
            }
            if row.object_is_parent != is_parent {
                return Err(Error::Unavailable(
                    "actual RegisterNodeConnection713 parent-conflict severe diagnostic",
                ));
            }
            make_new = false;
        }
        if make_new {
            self.number_node_connections =
                self.number_node_connections
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "actual RegisterNodeConnection count signed addition",
                    ))?;
            if self.number_node_connections > 1
                && self.number_node_connections > self.max_number_node_connections
            {
                self.max_number_node_connections = self
                    .max_number_node_connections
                    .checked_add(1000)
                    .ok_or(Error::Unavailable(
                        "actual RegisterNodeConnection allocation signed addition",
                    ))?;
                self.node_connections
                    .as_mut()
                    .ok_or(Error::Unavailable(
                        "actual RegisterNodeConnection existing array before resize",
                    ))?
                    .resize(
                        extent(self.max_number_node_connections)?,
                        SourceNodeConnection::default(),
                    );
            } else if self.number_node_connections == 1 {
                self.node_connections = Some(vec![SourceNodeConnection::default(); 1000]);
                self.max_number_node_connections = 1000;
            }
            let row = self
                .node_connections
                .as_mut()
                .and_then(|rows| {
                    rows.get_mut(extent(self.number_node_connections).ok()?.checked_sub(1)?)
                })
                .ok_or(Error::Unavailable(
                    "actual RegisterNodeConnection new record allocation",
                ))?;
            row.node_number = node_number;
            row.node_name = node_name.into();
            row.object_type = object_type;
            row.object_name = object_name.into();
            row.connection_type = connection_type;
            row.fluid_stream = fluid_stream;
            row.object_is_parent = is_parent;
        }
        // All represented object-type names are genuine non-AirTerminal
        // families, so Native's prefix guard is false and caller error latches
        // are unchanged. AirTerminal-specific records are not projected here.
        Ok(())
    }
}
