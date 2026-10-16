//! Actual HVACManager::ResetNodeData2151--2197 on the sole retained node owner.
use super::OrdinaryEquipment;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

impl OrdinaryEquipment {
    /// Native2161 returns before touching Node, DefaultNodeValues or MoreNodeInfo.
    /// The represented count is the same native ctor-zero counter, independent
    /// of diagnostic NodeStateStore length and any input/equipment family count.
    pub(crate) fn reset_source_node_data(&mut self) -> Result<(), Error> {
        if self.nodes.number_of_nodes == 0 {
            return Ok(());
        }
        // The diagnostic physical projection does not own the full native Node
        // or DefaultNodeValues records. Never seed, copy or partially reset it.
        Err(Error::Unavailable(
            "actual positive ResetNodeData Node traversal2165/DefaultNodeValues2166",
        ))
    }
}
