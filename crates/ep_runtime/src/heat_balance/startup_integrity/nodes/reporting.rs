//! Literal SimulationManager2066-2204 on the sole BranchNodeConnections owner.
use super::*;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use std::io::Write;

impl SourceBranchNodeConnections {
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
        let number_non_parents = 0;
        for _connection in 1..=self.number_node_connections {
            return Err(Error::Unavailable(
                "actual ReportNodeConnections2088 NodeConnections.ObjectIsParent",
            ));
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
            "! <{} Node Connection>,<Node Name>,<Node ObjectType>,<Node ObjectName>,<Node ConnectionType>,<Node FluidStream>",
            "Parent"
        )?;
        // Both parent and non-parent record loops use the same actual zero count.
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
            "! <{} Node Connection>,<Node Name>,<Node ObjectType>,<Node ObjectName>,<Node ConnectionType>,<Node FluidStream>",
            "Non-Parent"
        )?;
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
                return Err(Error::Unavailable(
                    "actual ReportNodeConnections2199 NodeID before NonConnected Node record",
                ));
            }
        }
        self.non_connected_nodes = None;
        Ok(())
    }
}
