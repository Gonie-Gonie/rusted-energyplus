//! Actual NodeInputManager::SetupNodeVarsForReporting over the sole NodeOwner.
use super::OrdinaryEquipment;
use crate::heat_balance::error_backend::SourceErrorTracking;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use std::io::Write;

/// Actual DataLoopNode.hh502–513 constructor record. The zero allocation has no rows.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceMoreNodeData {
    relative_humidity: f64,
    report_enthalpy: f64,
    volume_flow_rate_standard_density: f64,
    volume_flow_rate_current_density: f64,
    wet_bulb_temperature: f64,
    density: f64,
    air_dew_point_temperature: f64,
    specific_heat: f64,
}
impl OrdinaryEquipment {
    pub(crate) fn setup_source_node_variables_for_reporting(
        &mut self,
        tracking: &SourceErrorTracking,
        bnd: &mut dyn Write,
    ) -> Result<(), Error> {
        if !self.nodes.node_variables_setup {
            if !tracking.abort_processing {
                self.nodes.more_node_info = Some(vec![
                    SourceMoreNodeData::default();
                    self.nodes.number_of_unique_node_names
                ]);
                if self.nodes.number_of_unique_node_names > 0 {
                    // Actual full DataLoopNode::Node/NodeID are not the diagnostic
                    // NodeStateStore. Preserve allocation before the first lookup.
                    return Err(Error::Unavailable(
                        "actual SetupNodeVarsForReporting Node204/NodeID205",
                    ));
                }
            }
            self.nodes.node_variables_setup = true;
            bnd.write_all(b"! This file shows details about the branches, nodes, and other\n")?;
            bnd.write_all(b"! elements of the flow connections.\n")?;
            bnd.write_all(
                b"! This file is intended for use in \"debugging\" potential problems\n",
            )?;
            bnd.write_all(b"! that may also be detected by the program, but may be more easily\n")?;
            bnd.write_all(b"! identified by \"eye\".\n")?;
            bnd.write_all(b"! This file is also intended to support software which draws a\n")?;
            bnd.write_all(b"! schematic diagram of the HVAC system.\n")?;
            bnd.write_all(b"! ===============================================================\n")?;
            bnd.write_all(b"! #Nodes,<Number of Unique Nodes>\n")?;
            write!(bnd, " #Nodes,{}\n", self.nodes.number_of_unique_node_names)?;
            if self.nodes.number_of_unique_node_names > 0 {
                bnd.write_all(b"! <Node>,<NodeNumber>,<Node Name>,<Node Fluid Type>,<# Times Node Referenced After Definition>\n")?;
            }
            // Native Count0=0 is a caller local; the actual nonnegative count0
            // skips its first loop and therefore its suspicious-node branch.
            if self.nodes.number_of_unique_node_names > 0 {
                return Err(Error::Unavailable(
                    "actual SetupNodeVarsForReporting BND Node479/NodeID480",
                ));
            }
        }
        Ok(())
    }
}
