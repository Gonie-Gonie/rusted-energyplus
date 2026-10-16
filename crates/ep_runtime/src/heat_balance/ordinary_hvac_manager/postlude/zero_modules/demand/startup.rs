//! Actual InitDemandManagers1573-1589, independent of per-step ManageDemand.
use super::{Error, SourceDemandManager};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;

impl SourceDemandManager {
    /// Reads the same actual input owner at the genuine startup302 call.
    /// Unlike ManageDemand, Native InitDemandManagers has no DoingSizing guard.
    /// Each private getter preserves its real reached writes and typed boundary;
    /// GetInput clears only after both actual children return successfully.
    pub(crate) fn initialize_source_demand_managers(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), Error> {
        if self.get_input {
            self.get_source_manager_input(input)?;
            self.get_source_list_input(input)?;
            self.get_input = false;
        }
        Ok(())
    }
}
