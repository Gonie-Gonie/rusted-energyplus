//! Actual shared BranchNodeConnections module slice and two finite checks.
mod check_connections;
mod component_data;
mod registration;
mod reporting;
use super::{Error, extent};
use registration::SourceNodeConnection;
pub(crate) use registration::{
    SourceCompFluidStream, SourceNodeConnectionType, SourceNodeObjectType,
};
use std::convert::Infallible;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceBranchNodeConnections {
    number_component_sets: i32,
    number_node_connection_errors: i32,
    number_node_connections: i32,
    /// Actual DataBranchNodeConnections.hh119 constructor0, same module.
    max_number_node_connections: i32,
    component_sets: Option<Vec<Infallible>>,
    node_connections: Option<Vec<SourceNodeConnection>>,
    /// Actual DataBranchNodeConnections.hh120 constructor0; reporting never resets it.
    number_actual_parents: i32,
    parent_node_list: Option<Vec<Infallible>>,
    non_connected_nodes: Option<Vec<bool>>,
}
// Keep the explicit Native constructor/default literals and ordered member initialization.
#[allow(clippy::derivable_impls)]
impl Default for SourceBranchNodeConnections {
    fn default() -> Self {
        Self {
            number_component_sets: 0,
            number_node_connection_errors: 0,
            number_node_connections: 0,
            max_number_node_connections: 0,
            component_sets: None,
            node_connections: None,
            number_actual_parents: 0,
            parent_node_list: None,
            non_connected_nodes: None,
        }
    }
}
impl SourceBranchNodeConnections {
    /// Actual self-pair skip precedes first component record read at2452.
    pub(crate) fn test_source_component_sets(&self, _errors_found: &mut bool) -> Result<(), Error> {
        let already_noted = vec![false; extent(self.number_component_sets)?];
        for count in 1..=self.number_component_sets {
            for other in 1..=self.number_component_sets {
                if count == other {
                    continue;
                }
                return Err(Error::Unavailable(
                    "actual TestCompSetInletOutletNodes2452 CompSets.ComponentObjectType first record access",
                ));
            }
        }
        drop(already_noted);
        Ok(())
    }
}
