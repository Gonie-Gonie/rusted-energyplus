//! Actual shared BranchNodeConnections module slice and two finite checks.
use super::{Error, extent};
use std::convert::Infallible;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceBranchNodeConnections {
    number_component_sets: i32,
    number_node_connection_errors: i32,
    number_node_connections: i32,
    component_sets: Option<Vec<Infallible>>,
    node_connections: Option<Vec<Infallible>>,
}
impl Default for SourceBranchNodeConnections {
    fn default() -> Self {
        Self {
            number_component_sets: 0,
            number_node_connection_errors: 0,
            number_node_connections: 0,
            component_sets: None,
            node_connections: None,
        }
    }
}
impl SourceBranchNodeConnections {
    /// Actual zero connection loops leave caller ErrorsFound latched unchanged.
    pub(crate) fn check_source_node_connections(
        &mut self,
        _errors_found: &mut bool,
    ) -> Result<(), Error> {
        let error_counter = 0;
        for _connection in 1..=self.number_node_connections {
            return Err(Error::Unavailable(
                "actual CheckNodeConnections919 NodeConnections.ConnectionType first record access",
            ));
        }
        // Native check10 array allocations are guarded by actual count>0.
        // All remaining count-governed checks also have zero iterations here.
        self.number_node_connection_errors += error_counter;
        Ok(())
    }
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
