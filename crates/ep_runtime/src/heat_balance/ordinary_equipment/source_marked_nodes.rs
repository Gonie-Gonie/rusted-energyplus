//! Actual CheckMarkedNodes1230-1257 on the existing sole NodeOwner.
use super::*;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceMarkedNode {
    is_marked: bool,
    object_type: i32,
    object_name: String,
    field_name: String,
}
impl Default for SourceMarkedNode {
    fn default() -> Self {
        Self {
            is_marked: false,
            object_type: -1,
            object_name: String::new(),
            field_name: String::new(),
        }
    }
}
impl OrdinaryEquipment {
    /// The native zero-node loop returns without touching ErrorsFound or arrays.
    pub(crate) fn check_source_marked_nodes(&self, _errors_found: &mut bool) -> Result<(), Error> {
        for ordinal in 1..=self.nodes.number_of_nodes {
            let marked = self
                .nodes
                .marked_nodes
                .as_ref()
                .and_then(|rows| rows.get(ordinal - 1))
                .ok_or(Error::Unavailable(
                    "actual CheckMarkedNodes1243 MarkedNode.IsMarked",
                ))?;
            if marked.is_marked {
                let references = self
                    .nodes
                    .node_references
                    .as_ref()
                    .and_then(|rows| rows.get(ordinal - 1))
                    .ok_or(Error::Unavailable(
                        "actual CheckMarkedNodes1244 NodeInputMgr.NodeRef",
                    ))?;
                if *references == 0 {
                    return Err(Error::Unavailable(
                        "actual CheckMarkedNodes1245 ConnectionObjectTypeNames before severe/continue/ErrorsFoundtrue",
                    ));
                }
            }
        }
        Ok(())
    }
}
