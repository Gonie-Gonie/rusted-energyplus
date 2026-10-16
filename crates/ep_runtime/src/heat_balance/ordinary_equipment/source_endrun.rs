//! Actual EMSManager1574-1612 endrun check on the sole retained DataLoopNode count.
use super::OrdinaryEquipment;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

impl OrdinaryEquipment {
    pub(crate) fn check_source_setpoint_nodes_at_end(&self) -> Result<(), Error> {
        // Native1585 local only; this is not a retained readiness/error flag.
        let _fatal_error_flag = false;
        if self.nodes.number_of_nodes > 0 {
            return Err(Error::Unavailable(
                "actual checkSetpointNodesAtEnd first NodeSetpointCheck1588",
            ));
        }
        // With the actual empty traversal no writer changes the local false,
        // so the actual FatalErrorFlag test1607 does not invoke ShowFatalError.
        Ok(())
    }
}
