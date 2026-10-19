//! Literal SimulationManager2066-2204 on the sole BranchNodeConnections owner.
use super::*;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use std::io::Write;

/// Literal Native DataLoopNode.hh112-127 names, including its exact spelling.
fn connection_type_name(kind: SourceNodeConnectionType) -> Result<&'static str, Error> {
    use SourceNodeConnectionType::*;
    match kind {
        Blank => Ok("blank"),
        Inlet => Ok("Inlet"),
        Outlet => Ok("Outlet"),
        Internal => Ok("Internal"),
        ZoneNode => Ok("ZoneNode"),
        Sensor => Ok("Sensor"),
        Actuator => Ok("Actuator"),
        OutsideAir => Ok("OutdoorAir"),
        ReliefAir => Ok("ReliefAir"),
        ZoneInlet => Ok("ZoneInlet"),
        ZoneReturn => Ok("ZoneReturn"),
        ZoneExhaust => Ok("ZoneExhaust"),
        SetPoint => Ok("Setpoint"),
        Electric => Ok("Electric"),
        OutsideAirReference => Ok("OutsideAirReference"),
        InducedAir => Ok("InducedAir"),
        Invalid | Num => Err(Error::Unavailable(
            "actual ReportNodeConnections ConnectionTypeNames index domain",
        )),
    }
}

impl SourceBranchNodeConnections {
    pub(crate) fn source_number_node_connections(&self) -> &i32 {
        &self.number_node_connections
    }
    pub(crate) fn source_max_number_node_connections(&self) -> &i32 {
        &self.max_number_node_connections
    }
    pub(crate) fn source_number_component_sets(&self) -> &i32 {
        &self.number_component_sets
    }
    pub(crate) fn source_number_connection_errors(&self) -> &i32 {
        &self.number_node_connection_errors
    }

    /// Parent-list allocation persists; NonConnectedNodes deallocates only at suffix.
    pub(crate) fn report_source_node_connections(
        &mut self,
        equipment: &OrdinaryEquipment,
        bnd: &mut dyn Write,
    ) -> Result<(), Error> {
        let nodes = *equipment.source_number_of_nodes();
        i32::try_from(nodes).map_err(|_| {
            Error::Unavailable("actual ReportNodeConnections2084 NumOfNodes signed domain")
        })?;
        self.non_connected_nodes = Some(vec![true; nodes]);
        let mut number_non_parents: i32 = 0;
        for number in 1..=self.number_node_connections {
            let row = self
                .node_connections
                .as_ref()
                .and_then(|rows| {
                    extent(number)
                        .ok()
                        .and_then(|n| n.checked_sub(1))
                        .and_then(|i| rows.get(i))
                })
                .ok_or(Error::Unavailable(
                    "actual ReportNodeConnections2088 live NodeConnections",
                ))?;
            if row.object_is_parent {
                continue;
            }
            number_non_parents = number_non_parents.checked_add(1).ok_or(Error::Unavailable(
                "actual ReportNodeConnections2091 NumNonParents signed addition",
            ))?;
        }
        let number_parents = self.number_node_connections - number_non_parents;
        let parent_count = extent(number_parents)?;
        if parent_count > 0 {
            return Err(Error::Unavailable(
                "actual ReportNodeConnections2094 positive ParentNodeList row constructor",
            ));
        }
        self.parent_node_list = Some(Vec::new());
        writeln!(
            bnd,
            "! ==============================================================="
        )?;
        writeln!(
            bnd,
            "! <#{0} Node Connections>,<Number of {0} Node Connections>",
            "Parent"
        )?;
        writeln!(bnd, " #Parent Node Connections,{number_parents}")?;
        writeln!(
            bnd,
            "! <Parent Node Connection>,<Node Name>,<Node ObjectType>,<Node ObjectName>,<Node ConnectionType>,<Node FluidStream>"
        )?;
        for number in 1..=self.number_node_connections {
            let row = self
                .node_connections
                .as_ref()
                .and_then(|rows| {
                    extent(number)
                        .ok()
                        .and_then(|n| n.checked_sub(1))
                        .and_then(|i| rows.get(i))
                })
                .ok_or(Error::Unavailable(
                    "actual ReportNodeConnections2103 live NodeConnections",
                ))?;
            if !row.object_is_parent {
                continue;
            }
            return Err(Error::Unavailable(
                "actual ReportNodeConnections2106 positive parent NonConnectedNodes write",
            ));
        }
        writeln!(
            bnd,
            "! ==============================================================="
        )?;
        writeln!(
            bnd,
            "! <#{0} Node Connections>,<Number of {0} Node Connections>",
            "Non-Parent"
        )?;
        writeln!(bnd, " #Non-Parent Node Connections,{number_non_parents}")?;
        writeln!(
            bnd,
            "! <Non-Parent Node Connection>,<Node Name>,<Node ObjectType>,<Node ObjectName>,<Node ConnectionType>,<Node FluidStream>"
        )?;
        for number in 1..=self.number_node_connections {
            let row = self
                .node_connections
                .as_ref()
                .and_then(|rows| {
                    extent(number)
                        .ok()
                        .and_then(|n| n.checked_sub(1))
                        .and_then(|i| rows.get(i))
                })
                .ok_or(Error::Unavailable(
                    "actual ReportNodeConnections2169 live NodeConnections",
                ))?;
            if row.object_is_parent {
                continue;
            }
            *self
                .non_connected_nodes
                .as_mut()
                .and_then(|rows| row.node_number.checked_sub(1).and_then(|i| rows.get_mut(i)))
                .ok_or(Error::Unavailable(
                    "actual ReportNodeConnections2172 NonConnectedNodes(NodeNumber)",
                ))? = false;
            let object_type = row.object_type.source_name()?.to_ascii_uppercase();
            let connection_type = connection_type_name(row.connection_type)?;
            writeln!(
                bnd,
                " Non-Parent Node Connection,{},{},{},{},{}",
                row.node_name,
                object_type,
                row.object_name,
                connection_type,
                row.fluid_stream as i32
            )?;
        }
        let mut number_non_connected: i32 = 0;
        for number in 1..=*equipment.source_number_of_nodes() {
            let unconnected = self
                .non_connected_nodes
                .as_ref()
                .and_then(|rows| rows.get(number - 1))
                .ok_or(Error::Unavailable(
                    "actual ReportNodeConnections2184 NonConnectedNodes",
                ))?;
            if *unconnected {
                number_non_connected =
                    number_non_connected
                        .checked_add(1)
                        .ok_or(Error::Unavailable(
                            "actual ReportNodeConnections2185 NumNonConnected increment domain",
                        ))?;
            }
        }
        if number_non_connected > 0 {
            writeln!(
                bnd,
                "! ==============================================================="
            )?;
            write!(
                bnd,
                "! <#NonConnected Nodes>,<Number of NonConnected Nodes>\n #NonConnected Nodes,{}\n",
                number_non_connected
            )?;
            writeln!(
                bnd,
                "! <NonConnected Node>,<NonConnected Node Number>,<NonConnected Node Name>"
            )?;
            for number in 1..=*equipment.source_number_of_nodes() {
                if !*self
                    .non_connected_nodes
                    .as_ref()
                    .and_then(|rows| rows.get(number - 1))
                    .ok_or(Error::Unavailable(
                        "actual ReportNodeConnections2196 NonConnectedNodes",
                    ))?
                {
                    continue;
                }
                writeln!(
                    bnd,
                    " NonConnected Node,{},{}",
                    number,
                    equipment.source_node_id(number)?
                )?;
            }
        }
        self.non_connected_nodes = None;
        Ok(())
    }
}
